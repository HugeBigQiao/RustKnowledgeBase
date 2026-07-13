//! 异步文件写入: DataSet → CSV / JSON / TXT.

use crate::error::FlowError;
use crate::models::record::DataSet;
use crate::pipeline::async_readers::FileFormat;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// 异步写入文件.
pub async fn write_file(data: &DataSet, path: &str, format: &FileFormat) -> Result<(), FlowError> {
    match format {
        FileFormat::Csv => write_csv(data, path).await,
        FileFormat::Json => write_json(data, path).await,
        FileFormat::Txt => write_txt(data, path).await,
    }
}

async fn write_csv(data: &DataSet, path: &str) -> Result<(), FlowError> {
    let data = data.clone();
    let path = path.to_string();

    // csv writer 是同步 IO, 用 spawn_blocking
    tokio::task::spawn_blocking(move || -> Result<(), FlowError> {
        let mut wtr = csv::Writer::from_path(&path)?;
        wtr.write_record(&data.columns)?;

        for row in &data.rows {
            let vals: Vec<&str> = data.columns.iter()
                .map(|c| row.get(c).map(|s| s.as_str()).unwrap_or(""))
                .collect();
            wtr.write_record(&vals)?;
        }
        wtr.flush()?;
        Ok(())
    })
    .await
    .map_err(|e| FlowError::Other(format!("spawn_blocking 失败: {}", e)))??;

    Ok(())
}

async fn write_json(data: &DataSet, path: &str) -> Result<(), FlowError> {
    let json_rows: Vec<serde_json::Value> = data.rows.iter().map(|row| {
        let mut map = serde_json::Map::new();
        for col in &data.columns {
            let val = row.get(col).map(|s| s.as_str()).unwrap_or("");
            map.insert(col.clone(), serde_json::Value::String(val.to_string()));
        }
        serde_json::Value::Object(map)
    }).collect();

    let json_str = serde_json::to_string_pretty(&json_rows)?;
    let mut file = File::create(path).await?;
    file.write_all(json_str.as_bytes()).await?;
    Ok(())
}

async fn write_txt(data: &DataSet, path: &str) -> Result<(), FlowError> {
    let col = data.columns.first().cloned().unwrap_or_else(|| "line".to_string());
    let mut file = File::create(path).await?;

    for row in &data.rows {
        let val = row.get(&col).map(|s| s.as_str()).unwrap_or("");
        file.write_all(format!("{}\n", val).as_bytes()).await?;
    }
    Ok(())
}
