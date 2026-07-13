//! 文件写入: DataSet → CSV / JSON / TXT.

use crate::error::PipelineError;
use crate::models::record::DataSet;
use crate::pipeline::readers::FileFormat;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 根据格式把 DataSet 写入文件.
pub fn write_file(
    data: &DataSet,
    path: &Path,
    format: &FileFormat,
) -> Result<(), PipelineError> {
    match format {
        FileFormat::Csv => write_csv(data, path),
        FileFormat::Json => write_json(data, path),
        FileFormat::Txt => write_txt(data, path),
    }
}

// ===== CSV 写入 =====

fn write_csv(data: &DataSet, path: &Path) -> Result<(), PipelineError> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    // 写入表头
    writer.write_record(&data.columns)?;

    // 写入数据行
    for row in &data.rows {
        let values: Vec<&str> = data
            .columns
            .iter()
            .map(|col| row.get(col).map(|s| s.as_str()).unwrap_or(""))
            .collect();
        writer.write_record(&values)?;
    }

    writer.flush()?;
    Ok(())
}

// ===== JSON 写入 =====

fn write_json(data: &DataSet, path: &Path) -> Result<(), PipelineError> {
    let json_rows: Vec<serde_json::Value> = data
        .rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for col in &data.columns {
                let val = row.get(col).map(|s| s.as_str()).unwrap_or("");
                map.insert(col.clone(), serde_json::Value::String(val.to_string()));
            }
            serde_json::Value::Object(map)
        })
        .collect();

    let json_str = serde_json::to_string_pretty(&json_rows)?;
    let mut file = File::create(path)?;
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

// ===== TXT 写入 =====

fn write_txt(data: &DataSet, path: &Path) -> Result<(), PipelineError> {
    let mut file = File::create(path)?;

    // 找第一列, 或 "line" 列
    let col = data.columns.first().cloned().unwrap_or_else(|| "line".to_string());

    for row in &data.rows {
        let value = row.get(&col).map(|s| s.as_str()).unwrap_or("");
        writeln!(file, "{}", value)?;
    }

    Ok(())
}
