//! 自定义错误类型: 统一处理 pipeline 中的各类错误.

use std::fmt;

/// 数据管道错误枚举.
#[derive(Debug)]
pub enum PipelineError {
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
    Csv(csv::Error),
    Json(serde_json::Error),
    /// 业务逻辑错误.
    Other(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::Io(e) => write!(f, "I/O 错误: {}", e),
            PipelineError::Sqlite(e) => write!(f, "SQLite 错误: {}", e),
            PipelineError::Csv(e) => write!(f, "CSV 解析错误: {}", e),
            PipelineError::Json(e) => write!(f, "JSON 解析错误: {}", e),
            PipelineError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PipelineError {}

// 从各类型自动转换
impl From<std::io::Error> for PipelineError {
    fn from(e: std::io::Error) -> Self {
        PipelineError::Io(e)
    }
}

impl From<rusqlite::Error> for PipelineError {
    fn from(e: rusqlite::Error) -> Self {
        PipelineError::Sqlite(e)
    }
}

impl From<csv::Error> for PipelineError {
    fn from(e: csv::Error) -> Self {
        PipelineError::Csv(e)
    }
}

impl From<serde_json::Error> for PipelineError {
    fn from(e: serde_json::Error) -> Self {
        PipelineError::Json(e)
    }
}
