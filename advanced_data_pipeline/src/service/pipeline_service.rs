//! 业务编排: 把 readers / writers / db 组合成具体流程.

use std::path::Path;

use crate::error::PipelineError;
use crate::logger;
use crate::pipeline::db;
use crate::pipeline::readers::{self, FileFormat};
use crate::pipeline::writers;

/// 默认数据库文件.
const DEFAULT_DB: &str = "pipeline.db";

/// 导入: 文件 → 数据库表.
pub fn import_file(
    db_path: Option<&str>,
    file_path: &Path,
    format: &FileFormat,
    table_name: &str,
) -> Result<(), PipelineError> {
    let db_path = db_path.unwrap_or(DEFAULT_DB);
    let conn = db::open_db(db_path)?;

    // 1. 读取文件
    let data = readers::read_file(file_path, format)?;
    let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();

    logger::log_operation(
        "IMPORT",
        &format!("{:?} → 表 '{}' ({} 行)", format, table_name, data.len()),
    );

    // 2. 写入数据库
    let count = db::insert_dataset(&conn, table_name, &data)?;

    logger::log_operation(
        "IMPORT_OK",
        &format!(
            "文件 '{}' → 表 '{}', 成功写入 {} 行",
            file_name, table_name, count
        ),
    );

    println!();
    println!("✓ 导入完成: {} → 表 '{}' ({} 行)", file_name, table_name, count);
    Ok(())
}

/// 导出: 数据库表 → 文件.
pub fn export_table(
    db_path: Option<&str>,
    table_name: &str,
    output_path: &Path,
    format: &FileFormat,
) -> Result<(), PipelineError> {
    let db_path = db_path.unwrap_or(DEFAULT_DB);
    let conn = db::open_db(db_path)?;

    // 1. 从数据库读取
    let data = db::read_table(&conn, table_name)?;

    logger::log_operation(
        "EXPORT",
        &format!(
            "表 '{}' → {:?} ({} 行)",
            table_name,
            format,
            data.len()
        ),
    );

    // 2. 写入文件
    writers::write_file(&data, output_path, format)?;

    let out_name = output_path.file_name().unwrap_or_default().to_string_lossy();
    println!();
    println!("✓ 导出完成: 表 '{}' → {} ({} 行)", table_name, out_name, data.len());
    Ok(())
}

/// 列出所有表.
pub fn show_tables(db_path: Option<&str>) -> Result<(), PipelineError> {
    let db_path = db_path.unwrap_or(DEFAULT_DB);
    let conn = db::open_db(db_path)?;

    let tables = db::list_tables(&conn)?;

    println!("\n数据库 '{}' 中的表:", db_path);
    if tables.is_empty() {
        println!("  (空)");
    } else {
        for table in &tables {
            let count = db::count_rows(&conn, table)?;
            println!("  {}  ({} 行)", table, count);
        }
    }

    logger::log_operation("LIST", &format!("列出 {} 个表", tables.len()));
    Ok(())
}

/// 查看表内容.
pub fn show_table(db_path: Option<&str>, table_name: &str) -> Result<(), PipelineError> {
    let db_path = db_path.unwrap_or(DEFAULT_DB);
    let conn = db::open_db(db_path)?;

    let data = db::read_table(&conn, table_name)?;

    println!("\n表 '{}' ({} 行, {} 列):", table_name, data.len(), data.columns.len());

    // 打印表头
    let header: Vec<&str> = data.columns.iter().map(|c| c.as_str()).collect();
    println!("  {}", header.join(" | "));
    println!("  {}", header.iter().map(|_| "---").collect::<Vec<_>>().join("-|-"));

    // 打印数据行
    for row in &data.rows {
        let values: Vec<&str> = data
            .columns
            .iter()
            .map(|col| row.get(col).map(|s| s.as_str()).unwrap_or(""))
            .collect();
        println!("  {}", values.join(" | "));
    }

    logger::log_operation(
        "SHOW",
        &format!("查看表 '{}' ({} 行)", table_name, data.len()),
    );
    Ok(())
}

/// 显示操作日志.
pub fn show_log() {
    println!("\n操作日志:");
    for line in logger::read_logs() {
        println!("  {}", line);
    }
}
