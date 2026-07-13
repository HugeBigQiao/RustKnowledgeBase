use std::collections::{HashMap, HashSet, BTreeMap};

use crate::error::LibraryError;
use crate::models::book::Book;
use crate::models::category::Category;

/// 图书馆: 持有全部图书(HashMap)和自增 ID.
pub struct Library {
    /// 图书仓库: HashMap<ID, Book>, 按 ID 快速查找.
    books: HashMap<u32, Book>,
    /// 下一个可用的 ID.
    next_id: u32,
}

impl Library {
    /// 创建空图书馆.
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
            next_id: 1,
        }
    }

    /// 添加图书, 返回分配的 ID.
    pub fn add_book(
        &mut self,
        title: String,
        author: String,
        category: Category,
        year: u32,
        tags: Vec<&str>,
    ) -> Result<u32, LibraryError> {
        if title.trim().is_empty() {
            return Err(LibraryError::EmptyTitle);
        }

        let id = self.next_id;
        let book = Book::new(id, title, author, category, year, tags);

        // Entry API: 确保 ID 不重复(这里实际不会重复, 但留作示范).
        match self.books.entry(id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(LibraryError::DuplicateId(id))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(book);
                self.next_id += 1;
                Ok(id)
            }
        }
    }

    /// 根据 ID 获取图书(Option).
    pub fn get_book(&self, id: u32) -> Option<&Book> {
        self.books.get(&id)
    }

    /// 删除图书, 返回被删除的 Book 或错误.
    pub fn remove_book(&mut self, id: u32) -> Result<Book, LibraryError> {
        self.books.remove(&id).ok_or(LibraryError::NotFound(id))
    }

    /// 按作者搜索(模糊匹配).
    pub fn search_by_author<'a>(&'a self, query: &str) -> Vec<&'a Book> {
        self.books
            .values()
            .filter(|b| b.author.contains(query))
            .collect()
    }

    /// 按书名搜索(模糊匹配).
    pub fn search_by_title<'a>(&'a self, query: &str) -> Vec<&'a Book> {
        self.books
            .values()
            .filter(|b| b.title.contains(query))
            .collect()
    }

    /// **泛型搜索**: 接受任意条件闭包.
    pub fn search<'a, F>(&'a self, predicate: F) -> Vec<&'a Book>
    where
        F: Fn(&&'a Book) -> bool,
    {
        self.books.values().filter(predicate).collect()
    }

    /// 列出全部图书, 按年份升序排列(Vec 高级用法).
    pub fn list_all(&self) -> Vec<&Book> {
        let mut all: Vec<&Book> = self.books.values().collect();
        all.sort_by(|a, b| a.year.cmp(&b.year).then_with(|| a.title.cmp(&b.title)));
        all
    }

    /// 按分类筛选.
    pub fn list_by_category<'a>(&'a self, category: &Category) -> Vec<&'a Book> {
        self.books
            .values()
            .filter(|b| &b.category == category)
            .collect()
    }

    /// 统计信息: 总量 + 各分类数量(BTreeMap 有序).
    pub fn stats(&self) -> LibraryStats {
        let total = self.books.len();
        let mut by_category: BTreeMap<String, usize> = BTreeMap::new();

        for book in self.books.values() {
            let cat_name = book.category.to_string();
            *by_category.entry(cat_name).or_insert(0) += 1;
        }

        // 收集所有标签(HashSet 集合运算)
        let all_tags: HashSet<&String> = self
            .books
            .values()
            .flat_map(|b| &b.tags)
            .collect();

        LibraryStats {
            total,
            by_category,
            tag_count: all_tags.len(),
        }
    }
}

/// 图书馆统计信息.
pub struct LibraryStats {
    pub total: usize,
    pub by_category: BTreeMap<String, usize>,
    pub tag_count: usize,
}

impl std::fmt::Display for LibraryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "总藏书: {} 本", self.total)?;
        writeln!(f, "标签种类: {} 个", self.tag_count)?;
        writeln!(f, "分类分布:")?;
        for (cat, count) in &self.by_category {
            writeln!(f, "  {}: {} 本", cat, count)?;
        }
        Ok(())
    }
}
