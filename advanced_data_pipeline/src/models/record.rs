//! 通用数据记录: 一行 = 若干列名 → 值.

use std::collections::HashMap;

/// 数据行: 列名 → 字符串值.
pub type Row = HashMap<String, String>;

/// 数据集: 列名列表 + 多行数据.
#[derive(Debug, Clone)]
pub struct DataSet {
    /// 列名(保持顺序).
    pub columns: Vec<String>,
    /// 数据行.
    pub rows: Vec<Row>,
}

impl DataSet {
    /// 创建空数据集.
    pub fn new() -> Self {
        DataSet {
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// 从列名和行创建.
    pub fn from_parts(columns: Vec<String>, rows: Vec<Row>) -> Self {
        DataSet { columns, rows }
    }

    /// 行数.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// 是否为空.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
