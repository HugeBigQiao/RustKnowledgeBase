//! 导出引擎 — 异步执行: 查库 → 列变换 → 写入 Excel。
//!
//! 完全独立于 GUI, 通过 mpsc 通道与界面通信。
//!
//! 数据流: PostgreSQL 查询 → 列运算(加减乘除) → 排序 → Excel 写入

use std::sync::mpsc::Sender;

use sqlx::Row;
use rust_xlsxwriter::Workbook;

use crate::engine::EngineEvent;
use crate::error::FlowError;
use sqlx::PgPool;

/// 导出参数
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub db_name: String,
    pub table_name: String,
    pub columns: Vec<String>,
    pub operations: Vec<ColumnOp>,
    pub sort_column: Option<String>,
    pub sort_ascending: bool,
    pub row_limit: Option<usize>,
    pub output_path: String,
}

/// 列运算定义
#[derive(Debug, Clone)]
pub enum ColumnOp {
    /// col_a [+-*/] col_b → new_col_name
    Arithmetic {
        col_a: String,
        op: ArithmeticOp,
        col_b: String,
        result_col: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithmeticOp {
    pub fn symbol(&self) -> &str {
        match self {
            ArithmeticOp::Add => "+",
            ArithmeticOp::Sub => "-",
            ArithmeticOp::Mul => "*",
            ArithmeticOp::Div => "/",
        }
    }
}

// ======================================================================
//  主流程: 异步导出
// ======================================================================

/// 异步执行导出 (在后台 tokio 线程中调用)。
pub async fn run_export(
    pool: &PgPool,
    config: ExportConfig,
    event_tx: &Sender<EngineEvent>,
) -> Result<(), FlowError> {
    let _ = event_tx.send(EngineEvent::Status(format!(
        "正在从 {}.{} 导出 {} 列...",
        config.db_name,
        config.table_name,
        config.columns.len()
    )));
    let _ = event_tx.send(EngineEvent::ExportProgress(0.05));

    // ── 1. 查询数据 ──
    let limit = config.row_limit.unwrap_or(10000);
    let (mut columns, mut rows) =
        fetch_preview(pool, &config.table_name, &config.columns, limit).await?;
    let _ = event_tx.send(EngineEvent::ExportProgress(0.35));

    // ── 2. 应用列运算 ──
    if !config.operations.is_empty() {
        let _ = event_tx.send(EngineEvent::Status("正在计算列运算...".to_string()));
        apply_operations(&mut columns, &mut rows, &config.operations);
    }
    let _ = event_tx.send(EngineEvent::ExportProgress(0.55));

    // ── 3. 排序 ──
    if let Some(ref sort_col) = config.sort_column {
        let _ = event_tx.send(EngineEvent::Status(format!(
            "正在按 '{}' 排序...",
            sort_col
        )));
        apply_sort(&columns, &mut rows, sort_col, config.sort_ascending);
    }
    let _ = event_tx.send(EngineEvent::ExportProgress(0.70));

    // ── 4. 写入 Excel ──
    let _ = event_tx.send(EngineEvent::Status(format!(
        "正在写入 Excel: {} ...",
        config.output_path
    )));
    write_to_excel(&columns, &rows, &config.output_path)?;
    let _ = event_tx.send(EngineEvent::ExportProgress(1.0));

    // ── 完成 ──
    let _ = event_tx.send(EngineEvent::ExportComplete(config.output_path.clone()));
    let _ = event_tx.send(EngineEvent::Status(format!(
        "导出完成: {} 行, {} 列 → {}",
        rows.len(),
        columns.len(),
        config.output_path
    )));

    Ok(())
}

// ======================================================================
//  列运算
// ======================================================================

/// 在数据上应用列运算, 将结果作为新列追加。
fn apply_operations(
    columns: &mut Vec<String>,
    rows: &mut [Vec<String>],
    ops: &[ColumnOp],
) {
    for op in ops {
        match op {
            ColumnOp::Arithmetic {
                col_a,
                op,
                col_b,
                result_col,
            } => {
                let idx_a = columns.iter().position(|c| c == col_a);
                let idx_b = columns.iter().position(|c| c == col_b);

                if let (Some(ia), Some(ib)) = (idx_a, idx_b) {
                    // 为每一行计算
                    for row in rows.iter_mut() {
                        let val_a: f64 = row.get(ia).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        let val_b: f64 = row.get(ib).and_then(|s| s.parse().ok()).unwrap_or(0.0);

                        let result = match op {
                            ArithmeticOp::Add => val_a + val_b,
                            ArithmeticOp::Sub => val_a - val_b,
                            ArithmeticOp::Mul => val_a * val_b,
                            ArithmeticOp::Div => {
                                if val_b == 0.0 {
                                    f64::NAN
                                } else {
                                    val_a / val_b
                                }
                            }
                        };

                        row.push(format!("{:.4}", result));
                    }

                    // 新列追加到列名列表
                    columns.push(result_col.clone());
                }
            }
        }
    }
}

// ======================================================================
//  排序
// ======================================================================

/// 按指定列排序 (稳定排序, 保留原始顺序用于相等元素)。
fn apply_sort(
    columns: &[String],
    rows: &mut Vec<Vec<String>>,
    sort_col: &str,
    ascending: bool,
) {
    if let Some(col_idx) = columns.iter().position(|c| c == sort_col) {
        rows.sort_by(|a, b| {
            let va = a.get(col_idx).map(|s| s.as_str()).unwrap_or("");
            let vb = b.get(col_idx).map(|s| s.as_str()).unwrap_or("");

            // 尝试数字排序, 失败则字符串排序
            let cmp = if let (Ok(na), Ok(nb)) = (va.parse::<f64>(), vb.parse::<f64>()) {
                na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                va.cmp(vb)
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }
}

// ======================================================================
//  Excel 写入
// ======================================================================

/// 将列+行数据写入 Excel 文件。
fn write_to_excel(
    columns: &[String],
    rows: &[Vec<String>],
    output_path: &str,
) -> Result<(), FlowError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 写表头 (加粗, 蓝钢色背景)
    let header_format = rust_xlsxwriter::Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0x4682B4))
        .set_font_color(rust_xlsxwriter::Color::White);

    for (col_idx, col_name) in columns.iter().enumerate() {
        worksheet.write_with_format(
            0,
            col_idx as u16,
            col_name.as_str(),
            &header_format,
        )?;
    }

    // 写数据行
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            // 尝试写数字
            if let Ok(num) = cell.parse::<f64>() {
                worksheet.write((row_idx + 1) as u32, col_idx as u16, num)?;
            } else {
                worksheet.write((row_idx + 1) as u32, col_idx as u16, cell.as_str())?;
            }
        }
    }

    // 自动调整列宽 (近似)
    for (col_idx, col_name) in columns.iter().enumerate() {
        let max_len = std::cmp::max(
            col_name.len(),
            rows.iter()
                .filter_map(|r| r.get(col_idx))
                .map(|c| c.len())
                .max()
                .unwrap_or(0),
        );
        let width = (max_len as f64 * 1.1 + 2.0).min(40.0);
        worksheet.set_column_width(col_idx as u16, width)?;
    }

    workbook.save(output_path)?;
    Ok(())
}

// ======================================================================
//  数据库查询辅助
// ======================================================================

/// 查询表的所有列名 (异步)。
pub async fn fetch_columns(pool: &PgPool, table_name: &str) -> Result<Vec<String>, FlowError> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT column_name FROM information_schema.columns
         WHERE table_name = $1 ORDER BY ordinal_position",
    )
    .bind(table_name)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(c,)| c).collect())
}

/// 查询预览数据 (异步)。
pub async fn fetch_preview(
    pool: &PgPool,
    table_name: &str,
    columns: &[String],
    limit: usize,
) -> Result<(Vec<String>, Vec<Vec<String>>), FlowError> {
    if columns.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let cols_str = columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "SELECT {} FROM \"{}\" LIMIT {}",
        cols_str, table_name, limit
    );

    let pg_rows = sqlx::query(&sql).fetch_all(pool).await?;

    let mut data_rows: Vec<Vec<String>> = Vec::new();
    for pg_row in &pg_rows {
        let mut row = Vec::new();
        for i in 0..columns.len() {
            // pg_row.try_get::<String, _>(i) — 需要 sqlx::Row trait
            let val: Result<String, _> = pg_row.try_get(i);
            row.push(val.unwrap_or_else(|_| "NULL".to_string()));
        }
        data_rows.push(row);
    }

    Ok((columns.to_vec(), data_rows))
}
