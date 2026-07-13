//! 自定义错误类型.

use std::fmt;

#[derive(Debug)]
pub enum FlowError {
    Io(std::io::Error),
    Sqlx(sqlx::Error),
    Csv(csv::Error),
    Json(serde_json::Error),
    Other(String),
}

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlowError::Io(e) => write!(f, "I/O 错误: {}", e),
            FlowError::Sqlx(e) => write!(f, "数据库错误: {}", e),
            FlowError::Csv(e) => write!(f, "CSV 错误: {}", e),
            FlowError::Json(e) => write!(f, "JSON 错误: {}", e),
            FlowError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for FlowError {}

impl From<std::io::Error> for FlowError {
    fn from(e: std::io::Error) -> Self { FlowError::Io(e) }
}

impl From<sqlx::Error> for FlowError {
    fn from(e: sqlx::Error) -> Self { FlowError::Sqlx(e) }
}

impl From<csv::Error> for FlowError {
    fn from(e: csv::Error) -> Self { FlowError::Csv(e) }
}

impl From<serde_json::Error> for FlowError {
    fn from(e: serde_json::Error) -> Self { FlowError::Json(e) }
}
