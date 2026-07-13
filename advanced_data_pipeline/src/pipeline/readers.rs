//! 文件读取: CSV / JSON / TXT → DataSet.

use crate::error::PipelineError;
use crate::models::record::{DataSet, Row};
use std::fs;
use std::path::Path;

/// 支持的文件格式.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FileFormat {
    Csv,
    Json,
    Txt,
}

/// 根据格式读取文件, 返回 DataSet.
pub fn read_file(path: &Path, format: &FileFormat) -> Result<DataSet, PipelineError> {
    match format {
        FileFormat::Csv => read_csv(path),
        FileFormat::Json => read_json(path),
        FileFormat::Txt => read_txt(path),
    }
}

// ===== CSV 读取 =====

fn read_csv(path: &Path) -> Result<DataSet, PipelineError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    // 获取列名
    let headers: Vec<String> = reader
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut row = Row::new();
        for (i, value) in record.iter().enumerate() {
            let col_name = headers.get(i).cloned().unwrap_or_else(|| format!("col{}", i));
            row.insert(col_name, value.to_string());
        }
        rows.push(row);
    }

    Ok(DataSet::from_parts(headers, rows))
}

// ===== JSON 读取(对象数组) =====

fn read_json(path: &Path) -> Result<DataSet, PipelineError> {
    let content = fs::read_to_string(path)?;
    let json_value: serde_json::Value = serde_json::from_str(&content)?;

    // 期望格式: [ {"k1":"v1","k2":"v2"}, ... ]
    match json_value {
        serde_json::Value::Array(arr) => {
            parse_json_array(arr)
        }
        // 如果是单对象, 包装成数组
        serde_json::Value::Object(_) => {
            parse_json_array(vec![json_value])
        }
        _ => Err(PipelineError::Other(
            "JSON 格式不支持: 需要对象数组或单个对象".into(),
        )),
    }
}

fn parse_json_array(arr: Vec<serde_json::Value>) -> Result<DataSet, PipelineError> {
    if arr.is_empty() {
        return Ok(DataSet::new());
    }

    // 收集所有出现过的键作为列名(保持首次出现顺序)
    let mut columns: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for item in &arr {
        if let serde_json::Value::Object(map) = item {
            for key in map.keys() {
                if seen.insert(key.clone()) {
                    columns.push(key.clone());
                }
            }
        }
    }

    let mut rows = Vec::new();
    for item in &arr {
        if let serde_json::Value::Object(map) = item {
            let mut row = Row::new();
            for col in &columns {
                let val = map.get(col).map_or(String::new(), |v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => String::new(),
                    other => other.to_string(),
                });
                row.insert(col.clone(), val);
            }
            rows.push(row);
        }
    }

    Ok(DataSet::from_parts(columns, rows))
}

// ===== 纯文本读取(每行一条) =====

fn read_txt(path: &Path) -> Result<DataSet, PipelineError> {
    let content = fs::read_to_string(path)?;
    let columns = vec!["line".to_string()];

    let rows: Vec<Row> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut row = Row::new();
            row.insert("line".to_string(), line.to_string());
            row
        })
        .collect();

    Ok(DataSet::from_parts(columns, rows))
}
