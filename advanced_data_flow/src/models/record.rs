//! 通用数据模型(与 pipeline 项目保持一致).

use std::collections::HashMap;

/// 数据行: 列名 → 值.
pub type Row = HashMap<String, String>;

/// 数据集.
#[derive(Debug, Clone)]
pub struct DataSet {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}

impl DataSet {
    pub fn new() -> Self {
        DataSet { columns: Vec::new(), rows: Vec::new() }
    }

    pub fn from_parts(columns: Vec<String>, rows: Vec<Row>) -> Self {
        DataSet { columns, rows }
    }

    pub fn len(&self) -> usize { self.rows.len() }

    pub fn is_empty(&self) -> bool { self.rows.is_empty() }
}
