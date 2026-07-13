//! PostgreSQL 异步操作 + 写竞争处理.
//!
//! 展示三种写竞争处理策略:
//!   1. 乐观锁(version 列) — 适合读多写少
//!   2. 悲观锁(SELECT FOR UPDATE) — 适合写冲突频繁
//!   3. 事务 + ON CONFLICT — PostgreSQL 原生 upsert

use crate::error::FlowError;
use crate::models::record::{DataSet, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// 创建连接池.
pub async fn create_pool(db_url: &str) -> Result<PgPool, FlowError> {
    let pool = PgPoolOptions::new()
        .max_connections(10) // 连接池上限
        .connect(db_url)
        .await?;
    Ok(pool)
}

/// 建表 + 插入种子数据(确保测试环境就绪).
pub async fn init_schema(pool: &PgPool) -> Result<(), FlowError> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            value   TEXT NOT NULL,
            version INTEGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 插入一条种子数据(忽略重复)
    sqlx::query(
        r#"
        INSERT INTO items (name, value, version)
        VALUES ('种子数据', '初始值', 1)
        ON CONFLICT DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ===== 策略1: 乐观锁(version) =====

/// 乐观更新: 读出版本号 → 修改 → 写回时检查版本号是否一致.
/// 如果版本号变了, 说明被其他事务修改过, 返回冲突错误.
pub async fn optimistic_update(
    pool: &PgPool,
    id: i32,
    new_value: &str,
) -> Result<bool, FlowError> {
    // 1. 读出当前值和版本号
    let row: (String, i32) = sqlx::query_as(
        "SELECT value, version FROM items WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let (_old_value, old_version) = row;

    // 2. 更新时检查版本号(原子操作)
    let result = sqlx::query(
        "UPDATE items SET value = $1, version = version + 1 WHERE id = $2 AND version = $3",
    )
    .bind(new_value)
    .bind(id)
    .bind(old_version)
    .execute(pool)
    .await?;

    // rows_affected = 0 表示版本号不匹配(被其他事务抢先修改)
    Ok(result.rows_affected() > 0)
}

// ===== 策略2: 悲观锁(SELECT FOR UPDATE) =====

/// 悲观更新: 读取时锁定行, 其他事务必须等待.
/// 适合写冲突频繁的场景.
pub async fn pessimistic_update(
    pool: &PgPool,
    id: i32,
    new_value: &str,
) -> Result<(), FlowError> {
    // 开启事务
    let mut tx = pool.begin().await?;

    // SELECT FOR UPDATE: 锁定该行直到事务结束
    let _row: (String, i32) = sqlx::query_as(
        "SELECT value, version FROM items WHERE id = $1 FOR UPDATE",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    // 更新
    sqlx::query(
        "UPDATE items SET value = $1, version = version + 1 WHERE id = $2",
    )
    .bind(new_value)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

// ===== 策略3: ON CONFLICT (upsert) =====

/// 使用 PostgreSQL 原生的 ON CONFLICT 做 upsert.
/// 适合"有则更新, 无则插入"场景.
pub async fn upsert_item(
    pool: &PgPool,
    name: &str,
    value: &str,
) -> Result<(), FlowError> {
    sqlx::query(
        r#"
        INSERT INTO items (name, value, version)
        VALUES ($1, $2, 1)
        ON CONFLICT (name) DO UPDATE
        SET value = $2, version = items.version + 1
        "#,
    )
    .bind(name)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

// ===== 通用表读写(动态 SQL) =====

/// 把 DataSet 插入到指定表.
pub async fn insert_dataset(
    pool: &PgPool,
    table_name: &str,
    data: &DataSet,
) -> Result<usize, FlowError> {
    if data.is_empty() {
        return Ok(0);
    }

    // 动态建表
    let col_defs: Vec<String> = data
        .columns
        .iter()
        .map(|c| format!("\"{}\" TEXT", c))
        .collect();

    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
        table_name,
        col_defs.join(", ")
    );
    sqlx::query(&create_sql).execute(pool).await?;

    // 批量插入
    let col_list = data
        .columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ");

    let placeholders: Vec<String> = (1..=data.columns.len())
        .map(|i| format!("${}", i))
        .collect();

    let insert_sql = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table_name,
        col_list,
        placeholders.join(", ")
    );

    let mut count = 0;
    for row in &data.rows {
        // 用原始 SQL 构建参数(略繁琐, 实际项目建议用 sqlx::query_as + 结构体)
        let mut query = sqlx::query(&insert_sql);
        for col in &data.columns {
            let val = row.get(col).cloned().unwrap_or_default();
            query = query.bind(val);
        }
        query.execute(pool).await?;
        count += 1;
    }

    Ok(count)
}

/// 从表读出全部数据.
pub async fn read_table(pool: &PgPool, table_name: &str) -> Result<DataSet, FlowError> {
    use sqlx::Row as _;
    // 获取列名
    let col_rows: Vec<(String,)> = sqlx::query_as(
        "SELECT column_name FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position",
    )
    .bind(table_name)
    .fetch_all(pool)
    .await?;

    let columns: Vec<String> = col_rows.into_iter().map(|(c,)| c).collect();

    let sql = format!(
        "SELECT * FROM \"{}\"",
        table_name
    );

    let rows = sqlx::query(&sql)
        .fetch_all(pool)
        .await?;

    // 转换 rows → DataSet
    let mut data_rows = Vec::new();
    for pg_row in &rows {
        let mut row = Row::new();
        for (i, col) in columns.iter().enumerate() {
            // pg_row 的列可以按索引访问
            let val: Result<String, _> = pg_row.try_get(i);
            row.insert(col.clone(), val.unwrap_or_default());
        }
        data_rows.push(row);
    }

    Ok(DataSet::from_parts(columns, data_rows))
}

/// 列出所有用户表.
pub async fn list_tables(pool: &PgPool) -> Result<Vec<String>, FlowError> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT tablename FROM pg_tables WHERE schemaname = 'public' ORDER BY tablename",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(t,)| t).collect())
}
