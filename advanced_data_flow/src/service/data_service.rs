//! 业务编排: 并发导入 + 写竞争演示.

use std::sync::Arc;
use tokio::task::JoinSet;

use crate::error::FlowError;
use crate::pipeline::async_readers::{self, FileFormat};
use crate::pipeline::async_writers;
use crate::pipeline::db;

/// 连接 PostgreSQL, 返回连接池.
pub async fn connect(db_url: &str) -> Result<sqlx::PgPool, FlowError> {
    println!("[连接] PostgreSQL → {}", db_url);
    db::create_pool(db_url).await
}

/// 查看表内容(带列名打印).
pub async fn show_table(pool: &sqlx::PgPool, table_name: &str) -> Result<(), FlowError> {
    let data = db::read_table(pool, table_name).await?;

    if data.is_empty() {
        println!("\n表 '{}' 为空", table_name);
        return Ok(());
    }

    // 打印列名
    let header = data.columns.join(" | ");
    println!("\n表 '{}':", table_name);
    println!("{}", header);
    println!("{}", "-".repeat(header.len()));

    // 打印数据行
    for row in &data.rows {
        let vals: Vec<String> = data
            .columns
            .iter()
            .map(|c| row.get(c).cloned().unwrap_or_default())
            .collect();
        println!("{}", vals.join(" | "));
    }

    println!("(共 {} 行)\n", data.len());
    Ok(())
}

/// 异步导入: 读取文件 → 写入 PostgreSQL.
pub async fn import_file(
    pool: &sqlx::PgPool,
    file_path: &str,
    format: &FileFormat,
    table_name: &str,
) -> Result<(), FlowError> {
    println!("[导入] {:?} → 表 '{}'", format, table_name);

    let data = async_readers::read_file(file_path, format).await?;
    let count = db::insert_dataset(pool, table_name, &data).await?;

    println!("[完成] 写入 {} 行", count);
    Ok(())
}

/// 异步导出: PostgreSQL → 文件.
pub async fn export_table(
    pool: &sqlx::PgPool,
    table_name: &str,
    output_path: &str,
    format: &FileFormat,
) -> Result<(), FlowError> {
    println!("[导出] 表 '{}' → {:?}", table_name, format);

    let data = db::read_table(pool, table_name).await?;
    async_writers::write_file(&data, output_path, format).await?;

    println!("[完成] 导出 {} 行 → {}", data.len(), output_path);
    Ok(())
}

/// 列出表.
pub async fn list_tables(pool: &sqlx::PgPool) -> Result<(), FlowError> {
    let tables = db::list_tables(pool).await?;
    println!("\n数据库中的表:");
    for t in &tables {
        println!("  {}", t);
    }
    if tables.is_empty() {
        println!("  (空)");
    }
    Ok(())
}

// ===== 并发导入演示 =====

/// 同时读取多个文件, 并发写入数据库.
pub async fn concurrent_import(
    pool: &sqlx::PgPool,
    files: &[(String, FileFormat, String)], // (路径, 格式, 表名)
) -> Result<(), FlowError> {
    println!("\n=== 并发导入 {} 个文件 ===", files.len());

    let pool = Arc::new(pool.clone());
    let mut joinset = JoinSet::new();

    for (path, format, table) in files.iter() {
        let pool = pool.clone();
        let path = path.clone();
        let format = format.clone();
        let table = table.clone();

        joinset.spawn(async move {
            let data = async_readers::read_file(&path, &format).await?;
            db::insert_dataset(&pool, &table, &data).await
        });
    }

    // 收集结果
    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(Ok(count)) => println!("  任务完成: {} 行", count),
            Ok(Err(e)) => eprintln!("  任务失败: {}", e),
            Err(e) => eprintln!("  JoinError: {}", e),
        }
    }

    Ok(())
}

// ===== 写竞争演示 =====

/// 模拟多个并发任务同时修改同一行, 对比三种策略.
pub async fn demo_write_contention(pool: &sqlx::PgPool) -> Result<(), FlowError> {
    db::init_schema(pool).await?;

    println!("\n=== 写竞争策略演示 ===\n");

    // --- 策略1: 乐观锁 ---
    println!("--- 策略1: 乐观锁(version) ---");
    println!("  场景: 10 个并发任务同时更新同一行");
    println!("  预期: 只有第一个成功, 其余因版本号不匹配失败");

    let pool = Arc::new(pool.clone());
    let mut joinset = JoinSet::new();

    for i in 0..10 {
        let pool = pool.clone();
        joinset.spawn(async move {
            let success = db::optimistic_update(&pool, 1, &format!("任务{}", i)).await?;
            Ok::<_, FlowError>((i, success))
        });
    }

    let mut success_count = 0;
    let mut fail_count = 0;
    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(Ok((i, true))) => {
                println!("  [OK]   任务 {} 写入成功", i);
                success_count += 1;
            }
            Ok(Ok((i, false))) => {
                println!("  [冲突] 任务 {} 版本不匹配(被抢先)", i);
                fail_count += 1;
            }
            _ => {}
        }
    }
    println!("  结果: {} 成功, {} 冲突\n", success_count, fail_count);

    // --- 策略2: 悲观锁 ---
    println!("--- 策略2: 悲观锁(SELECT FOR UPDATE) ---");
    println!("  场景: 5 个并发任务, 依次排队等待锁");

    let pool = pool.clone();
    let mut joinset = JoinSet::new();

    for i in 0..5 {
        let pool = pool.clone();
        joinset.spawn(async move {
            db::pessimistic_update(&pool, 1, &format!("悲观任务{}", i)).await?;
            Ok::<_, FlowError>(i)
        });
    }

    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(Ok(i)) => println!("  [OK] 悲观任务 {} 完成", i),
            _ => {}
        }
    }
    println!("  结果: 全部排队执行(顺序不保证)\n");

    // --- 策略3: UPSERT ---
    println!("--- 策略3: ON CONFLICT (upsert) ---");
    println!("  场景: 用唯一键写入, 有则更新无则插入");

    db::upsert_item(&pool, "并发测试", "值1").await?;
    println!("  第一次 upsert: 插入新行");
    db::upsert_item(&pool, "并发测试", "值2").await?;
    println!("  第二次 upsert: 更新已有行\n");

    println!("写竞争策略总结:");
    println!("  乐观锁:    读多写少, 冲突少时性能最好");
    println!("  悲观锁:    写冲突频繁, 需要严格顺序");
    println!("  ON CONFLICT: PostgreSQL 原生, 简洁高效");
    println!("  实际选择: 根据业务场景和数据特征决定");

    Ok(())
}
