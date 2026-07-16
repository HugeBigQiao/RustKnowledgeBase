use std::collections::{HashMap, HashSet, BTreeMap};

use crate::error::LibraryError;
use crate::models::book::Book;
use crate::models::category::Category;

/// 图书馆: 持有全部图书(HashMap)。
/// 注意: 不再持有 next_id — ID 由调用方(主循环)通过 static mut 全局计数器管理。
pub struct Library {
    /// 图书仓库: HashMap<ID, Book>, 按 ID 快速查找。
    books: HashMap<u32, Book>,
}

impl Library {
    /// 创建空图书馆。
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    /// 返回当前藏书数量。
    pub fn book_count(&self) -> usize {
        self.books.len()
    }

    /// 添加图书, ID 由调用方指定 (来自全局计数器)。
    /// 返回: Ok(id) — 分配的 ID; Err — 标题为空或 ID 冲突。
    pub fn add_book(
        &mut self,
        id: u32,                                   // ID 由外部传入(来自 static mut 全局计数器)
        title: String,
        author: String,
        category: Category,
        year: u32,
        tags: Vec<&str>,
    ) -> Result<u32, LibraryError> {
        if title.trim().is_empty() {
            return Err(LibraryError::EmptyTitle);
        }

        let book = Book::new(title, author, category, year, tags);

        // Entry API: HashMap 的"检查并操作"统一入口。
        // self.books.entry(id) 返回 Entry 枚举, 有两个变体:
        //   Vacant(entry) → 键不存在, entry 是你"插入的口子"
        //   Occupied(entry) → 键已存在, entry 指向已有值
        // 好处: 只做一次哈希查找。先查是否存在 → 不存在就插 — 如果用
        // contains_key + insert 两步, 需要做两次哈希查找, Entry API 一次搞定。
        match self.books.entry(id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(LibraryError::DuplicateId(id))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(book);
                Ok(id)
            }
        }
    }

    /// 根据 ID 获取图书(Option).
    pub fn get_book(&self, id: u32) -> Option<&Book> {
        self.books.get(&id)
    }

    /// 删除图书, 返回被删除的 Book 或错误。
    pub fn remove_book(&mut self, id: u32) -> Result<Book, LibraryError> {
        self.books.remove(&id).ok_or(LibraryError::NotFound(id))
    }

    /// 修改图书 (原地更新), 返回修改后的 Book 引用或错误。
    /// 只更新用户提供的字段: 传 None 或空串表示"不修改此字段"。
    pub fn modify_book(
        &mut self,
        id: u32,
        title: Option<String>,                        // Option: None = 不改
        author: Option<String>,
        category: Option<Category>,
        year: Option<u32>,
        tags: Option<Vec<&str>>,
    ) -> Result<&Book, LibraryError> {
        // get_mut 返回 Option<&mut Book> — 可变借用, 可原地改字段
        let book = self.books.get_mut(&id).ok_or(LibraryError::NotFound(id))?;
        // ? 运算符: Ok → 取出值继续; Err → 立即 return 给调用方

        // 逐一检查: 调用方传了哪个字段就更新哪个字段
        if let Some(t) = title {
            if t.trim().is_empty() {
                return Err(LibraryError::EmptyTitle);
            }
            book.title = t;                          // 所有权: String 移入 book.title
        }
        if let Some(a) = author {
            book.author = a;                         // 旧 author 被 drop, 新 String 移入
        }
        if let Some(c) = category {
            book.category = c;                       // Category 不持有堆数据, 直接替换
        }
        if let Some(y) = year {
            book.year = y;                           // u32 是 Copy, 直接覆盖
        }
        if let Some(t) = tags {
            // tags: Vec<&str> → HashSet<String>  消耗迭代器, 创建新 HashSet
            book.tags = t.into_iter().map(String::from).collect();
        }

        Ok(book)                                     // 返回不可变引用 (从 &mut 自动降级)
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
