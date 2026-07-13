use std::fmt;

/// 图书馆操作的自定义错误类型。
#[derive(Debug)]
pub enum LibraryError {
    /// 图书 ID 已存在
    DuplicateId(u32),
    /// 未找到指定 ID
    NotFound(u32),
    /// 无效的分类名
    InvalidCategory(String),
    /// 标题不能为空
    EmptyTitle,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryError::DuplicateId(id) => write!(f, "图书 ID {} 已存在", id),
            LibraryError::NotFound(id) => write!(f, "未找到 ID 为 {} 的图书", id),
            LibraryError::InvalidCategory(s) => write!(f, "无效的分类: {}", s),
            LibraryError::EmptyTitle => write!(f, "书名不能为空"),
        }
    }
}

impl std::error::Error for LibraryError {}
