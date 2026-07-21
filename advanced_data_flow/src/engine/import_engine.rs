//! 导入引擎 — 异步执行: 读取多数据源 → 列映射 → 写入数据库。
//!
//! 完全独立于 GUI, 通过 mpsc 通道与界面通信。
//!
//! 数据流: 多个文件(CSV/JSON/Excel/TXT) → 列映射 → 合并写入 PostgreSQL

use std::sync::mpsc::Sender;

use crate::engine::EngineEvent;
use crate::error::FlowError;
use sqlx::PgPool;

/// 导入参数
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub source_paths: Vec<String>,
    pub target_db: String,
    pub target_table: String,
    pub column_mapping: Vec<(String, String)>, // source_col → target_col
}

/// 异步执行导入 (在后台 tokio 线程中调用)。
pub async fn run_import(
    pool: &PgPool,
    config: ImportConfig,
    event_tx: &Sender<EngineEvent>,
) -> Result<(), FlowError> {
    let total_sources = config.source_paths.len();
    let _ = event_tx.send(EngineEvent::Status(format!(
        "正在导入 {} 个数据源到 {}.{} ...",
        total_sources, config.target_db, config.target_table
    )));
    let _ = event_tx.send(EngineEvent::ImportProgress(0.0));

    // ── 1. 确保目标表存在 (动态建表) ──
    let target_cols: Vec<String> = config
        .column_mapping
        .iter()
        .map(|(_, target)| target.clone())
        .collect();

    create_table_if_needed(pool, &config.target_table, &target_cols).await?;
    let _ = event_tx.send(EngineEvent::ImportProgress(0.05));

    // ── 2. 逐个读取数据源并写入 ──
    let mut total_rows = 0usize;

    for (idx, path) in config.source_paths.iter().enumerate() {
        let _ = event_tx.send(EngineEvent::Status(format!(
            "正在处理 ({}/{}) {} ...",
            idx + 1,
            total_sources,
            path
        )));

        // 读取数据源
        let (src_cols, src_rows) = read_source_data(path).await?;

        // 应用列映射: source_cols → target_cols
        let mapped_rows = apply_column_mapping(&src_cols, &src_rows, &config.column_mapping);

        // 批量写入数据库
        let count = insert_rows(pool, &config.target_table, &target_cols, &mapped_rows).await?;
        total_rows += count;

        let progress = (idx + 1) as f32 / total_sources as f32;
        let _ = event_tx.send(EngineEvent::ImportProgress(progress));
    }

    // ── 完成 ──
    let _ = event_tx.send(EngineEvent::ImportProgress(1.0));
    let _ = event_tx.send(EngineEvent::ImportComplete(total_sources));
    let _ = event_tx.send(EngineEvent::Status(format!(
        "导入完成: {} 个数据源, 共 {} 行 → 表 '{}'",
        total_sources, total_rows, config.target_table
    )));

    Ok(())
}

// ======================================================================
//  读取数据源文件
// ======================================================================

/// 从文件读取数据, 返回 (列名列表, 行数据)。
async fn read_source_data(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    let lower = path.to_lowercase();

    if lower.ends_with(".csv") {
        read_csv(path)
    } else if lower.ends_with(".json") {
        read_json(path)
    } else if lower.ends_with(".xlsx") || lower.ends_with(".xls") {
        read_excel(path)
    } else {
        read_txt(path)
    }
}

fn read_csv(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers: Vec<String> = reader
        .headers()?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        rows.push(row);
    }

    Ok((headers, rows))
}

fn read_json(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    let content = std::fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&content)?;

    let arr = value
        .as_array()
        .ok_or_else(|| FlowError::Other("JSON 顶层不是数组".to_string()))?;

    if arr.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    // 收集所有列名(并集)
    let mut all_cols: Vec<String> = Vec::new();
    for item in arr {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if !all_cols.contains(key) {
                    all_cols.push(key.clone());
                }
            }
        }
    }

    // 转换每行为 Vec<String>, 按 all_cols 顺序
    let mut rows = Vec::new();
    for item in arr {
        if let Some(obj) = item.as_object() {
            let row: Vec<String> = all_cols
                .iter()
                .map(|col| {
                    obj.get(col)
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        })
                        .unwrap_or_default()
                })
                .collect();
            rows.push(row);
        }
    }

    Ok((all_cols, rows))
}

fn read_excel(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    use calamine::{open_workbook, Reader, Xlsx};

    let mut workbook: Xlsx<_> =
        open_workbook(path).map_err(|e| FlowError::Other(format!("Excel 读取失败: {}", e)))?;

    let range = workbook
        .worksheet_range_at(0)
        .ok_or_else(|| FlowError::Other("Excel 工作表为空".to_string()))?
        .map_err(|e| FlowError::Other(format!("Excel 读取失败: {}", e)))?;

    let mut rows_iter = range.rows();

    // 第一行作为表头
    let headers: Vec<String> = rows_iter
        .next()
        .map(|row| row.iter().map(|cell| cell.to_string()).collect())
        .unwrap_or_default();

    // 后续行为数据
    let rows: Vec<Vec<String>> = rows_iter
        .map(|row| row.iter().map(|cell| cell.to_string()).collect())
        .collect();

    Ok((headers, rows))
}

fn read_txt(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    let headers = vec!["line_number".to_string(), "content".to_string()];
    let rows: Vec<Vec<String>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| vec![(i + 1).to_string(), line.to_string()])
        .collect();

    Ok((headers, rows))
}

// ======================================================================
//  列映射 + 写入
// ======================================================================

/// 将源数据按列映射转换为目标列顺序的行数据。
fn apply_column_mapping(
    src_cols: &[String],
    src_rows: &[Vec<String>],
    mapping: &[(String, String)], // (source_col, target_col)
) -> Vec<Vec<String>> {
    // 构建: 目标列在 src_cols 中的索引位置
    let target_order: Vec<Option<usize>> = mapping
        .iter()
        .map(|(src, _target)| src_cols.iter().position(|c| c == src))
        .collect();

    src_rows
        .iter()
        .map(|row| {
            target_order
                .iter()
                .map(|idx| match idx {
                    Some(i) => row.get(*i).cloned().unwrap_or_default(),
                    None => String::new(),
                })
                .collect()
        })
        .collect()
}

/// 建表 (如果不存在)。
async fn create_table_if_needed(
    pool: &PgPool,
    table_name: &str,
    columns: &[String],
) -> Result<(), FlowError> {
    let col_defs: Vec<String> = columns
        .iter()
        .map(|c| format!("\"{}\" TEXT", c))
        .collect();

    if col_defs.is_empty() {
        return Ok(());
    }

    let sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
        table_name,
        col_defs.join(", ")
    );

    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}

/// 批量插入行数据。
async fn insert_rows(
    pool: &PgPool,
    table_name: &str,
    columns: &[String],
    rows: &[Vec<String>],
) -> Result<usize, FlowError> {
    if rows.is_empty() || columns.is_empty() {
        return Ok(0);
    }

    let col_list = columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ");

    // 参数占位符: $1, $2, ...
    let placeholders: Vec<String> = (1..=columns.len())
        .map(|i| format!("${}", i))
        .collect();

    let insert_sql = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table_name,
        col_list,
        placeholders.join(", ")
    );

    let mut count = 0;
    for row in rows {
        let mut query = sqlx::query(&insert_sql);
        for cell in row {
            query = query.bind(cell);
        }
        query.execute(pool).await?;
        count += 1;
    }

    Ok(count)
}
