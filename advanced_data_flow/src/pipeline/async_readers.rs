//! 异步文件读取: CSV / JSON / TXT → DataSet.

use crate::error::FlowError;
use crate::models::record::{DataSet, Row};
use std::collections::HashSet;
use tokio::fs;

/// 文件格式枚举.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FileFormat {
    Csv,
    Json,
    Txt,
}

/// 异步读取文件.
pub async fn read_file(path: &str, format: &FileFormat) -> Result<DataSet, FlowError> {
    match format {
        FileFormat::Csv => read_csv(path).await,
        FileFormat::Json => read_json(path).await,
        FileFormat::Txt => read_txt(path).await,
    }
}

// ===== 异步 CSV =====

async fn read_csv(path: &str) -> Result<DataSet, FlowError> {
    let content = fs::read_to_string(path).await?;

    // csv crate 是同步的, 用 spawn_blocking 避免阻塞异步运行时
    // (小文件直接在当前 task 里跑也行)
    let result = tokio::task::spawn_blocking(move || -> Result<DataSet, FlowError> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let headers: Vec<String> = reader
            .headers()?
            .iter()
            .map(|h| h.to_string())
            .collect();

        let mut rows = Vec::new();
        for record in reader.records() {
            let record = record?;
            let mut row = Row::new();
            for (i, val) in record.iter().enumerate() {
                let col = headers.get(i).cloned().unwrap_or_else(|| format!("col{}", i));
                row.insert(col, val.to_string());
            }
            rows.push(row);
        }
        Ok(DataSet::from_parts(headers, rows))
    })
    .await
    .map_err(|e| FlowError::Other(format!("spawn_blocking 失败: {}", e)))??;

    Ok(result)
}

// ===== 异步 JSON =====

async fn read_json(path: &str) -> Result<DataSet, FlowError> {
    let content = fs::read_to_string(path).await?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    let arr = match json {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json],
        _ => return Err(FlowError::Other("JSON 格式: 需要对象数组或对象".into())),
    };

    if arr.is_empty() {
        return Ok(DataSet::new());
    }

    // 收集列名
    let mut columns = Vec::new();
    let mut seen = HashSet::new();
    for item in &arr {
        if let serde_json::Value::Object(map) = item {
            for k in map.keys() {
                if seen.insert(k.clone()) {
                    columns.push(k.clone());
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

// ===== 异步 TXT =====

async fn read_txt(path: &str) -> Result<DataSet, FlowError> {
    let content = fs::read_to_string(path).await?;
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
