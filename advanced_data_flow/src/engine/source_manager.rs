//! 数据源管理器 — 注册、验证、预览多个数据源。

/// 数据源信息
#[derive(Debug, Clone)]
pub struct DataSourceInfo {
    pub file_path: String,
    pub file_name: String,
    pub format: SourceFormat,
    pub columns: Vec<String>,
    pub row_count: usize,
    pub loaded: bool,
}

/// 数据源格式
#[derive(Debug, Clone, PartialEq)]
pub enum SourceFormat {
    Csv,
    Json,
    Excel,
    Txt,
}

impl SourceFormat {
    pub fn from_path(path: &str) -> Option<Self> {
        let lower = path.to_lowercase();
        if lower.ends_with(".csv") {
            Some(SourceFormat::Csv)
        } else if lower.ends_with(".json") {
            Some(SourceFormat::Json)
        } else if lower.ends_with(".xlsx") || lower.ends_with(".xls") {
            Some(SourceFormat::Excel)
        } else if lower.ends_with(".txt") {
            Some(SourceFormat::Txt)
        } else {
            None
        }
    }

    pub fn label(&self) -> &str {
        match self {
            SourceFormat::Csv => "CSV",
            SourceFormat::Json => "JSON",
            SourceFormat::Excel => "XLSX",
            SourceFormat::Txt => "TXT",
        }
    }
}

impl std::fmt::Display for SourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// 数据源管理器
pub struct SourceManager {
    pub sources: Vec<DataSourceInfo>,
}

impl SourceManager {
    pub fn new() -> Self {
        SourceManager {
            sources: Vec::new(),
        }
    }

    /// 添加一个数据源 (文件路径)。
    pub fn add_source(&mut self, path: &str) {
        if self.sources.iter().any(|s| s.file_path == path) {
            return; // 已存在
        }

        let file_name = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());

        let format = SourceFormat::from_path(path).unwrap_or(SourceFormat::Txt);

        self.sources.push(DataSourceInfo {
            file_path: path.to_string(),
            file_name,
            format,
            columns: Vec::new(),
            row_count: 0,
            loaded: false,
        });
    }

    /// 移除数据源。
    pub fn remove_source(&mut self, index: usize) {
        if index < self.sources.len() {
            self.sources.remove(index);
        }
    }
}
