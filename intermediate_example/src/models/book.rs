use std::collections::HashSet;
use crate::models::category::Category;

/// 图书结构体: 持有书名、作者、分类等所有字段.
#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub category: Category,
    pub year: u32,
    /// 标签集合(HashSet: 不重复).
    pub tags: HashSet<String>,
}

impl Book {
    /// 构造一本新书.
    pub fn new(
        id: u32,
        title: String,
        author: String,
        category: Category,
        year: u32,
        tags: Vec<&str>,
    ) -> Self {
        Book {
            id,
            title,
            author,
            category,
            year,
            tags: tags.into_iter().map(String::from).collect(),
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] 《{}》 {}  {}  {}年  标签: {:?}",
            self.id, self.title, self.author, self.category, self.year, self.tags
        )
    }
}
