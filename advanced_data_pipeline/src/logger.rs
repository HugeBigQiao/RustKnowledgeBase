//! 操作日志: 记录每次导入/导出等操作.

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const LOG_FILE: &str = "pipeline.log";

/// 记录一条日志.
pub fn log_operation(operation: &str, details: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let entry = format!("[{}] {}: {}\n", timestamp, operation, details);

    // 追加写入日志文件
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
    {
        let _ = file.write_all(entry.as_bytes());
    }

    // 同时输出到控制台
    print!("[LOG] {}", entry);
}

/// 读取全部日志.
pub fn read_logs() -> Vec<String> {
    let path = Path::new(LOG_FILE);
    if path.exists() {
        std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect()
    } else {
        vec!["暂无日志".to_string()]
    }
}
