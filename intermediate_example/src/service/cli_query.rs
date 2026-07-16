//! CLI 查询命令: 按 ID 精确查询、按书名/作者模糊搜索。
//!
//! 核心: Option<T> 处理"可能有值/可能没有";
//!       生命周期 — Vec<&Book> 里的引用不能比 library 活得久;
//!       所有查询都只读 (&Library), 不需要 &mut。

use crate::models::book::Book;
use crate::models::library::Library;
use super::cli::parse_id; // 共享辅助函数, pub(crate) 可见

// ═══════════════════════════════════════════════════════════════
// 命令: query <ID> — 按 ID 查询
// ═══════════════════════════════════════════════════════════════

/// 按 ID 查询图书。
/// - library: &Library — 不可变借用, 只读
/// - id_str: &str — 借用, 不消耗原 String
pub fn cmd_query_by_id(library: &Library, id_str: &str) {
    // parse_id: &str → Result<u32, String> (id_str 借用, 不消耗)
    match parse_id(id_str) {
        Ok(id) => { // id: u32 (Copy), 模式匹配自动复制
            // get_book: &self → Option<&Book>
            // 返回的 &Book 生命周期 = library 生命周期
            match library.get_book(id) {
                Some(book) => println!("{}", book), // &Book 借给 Display::fmt
                None => println!("未找到 ID={} 的图书。", id),
            }
        }
        Err(e) => println!("{}", e), // e: String, 所有权从 Err 移出, 借给 println!, 之后 drop
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: search title <关键词> — 按书名模糊搜索
// ═══════════════════════════════════════════════════════════════

/// 按书名模糊搜索。
/// - library: &Library — 不可变借用
/// - query: &str — 搜索词, 借用
pub fn cmd_search_title(library: &Library, query: &str) {
    // search_by_title: &self, query: &str → Vec<&Book>
    // 返回的 &Book 生命周期 = library 生命周期 (编译器推断)
    let results: Vec<&Book> = library.search_by_title(query);

    print_search_results(query, "书名", &results); // &results: 借用, 不消耗 Vec
}

// ═══════════════════════════════════════════════════════════════
// 命令: search author <关键词> — 按作者搜索
// ═══════════════════════════════════════════════════════════════

/// 按作者模糊搜索。生命周期约束同 search_title。
pub fn cmd_search_author(library: &Library, query: &str) {
    let results: Vec<&Book> = library.search_by_author(query);
    // &Book 借用 library 里的数据, 生命周期 = library 生命周期
    print_search_results(query, "作者", &results);
}

// ═══════════════════════════════════════════════════════════════
// 辅助: 统一打印搜索结果
// ═══════════════════════════════════════════════════════════════

/// 打印搜索结果。
/// - results: &[&Book] — 双重借用!
///   外层 & : 不消耗 Vec (Vec 仍归调用方)
///   内层 &Book: 不消耗 library 里的 Book
///   用 &[T] 而非 &Vec<T>: 更通用, Vec 自动 Deref → 切片
fn print_search_results(query: &str, field: &str, results: &[&Book]) {
    println!("搜索{}含 '{}': {} 本", field, query, results.len());

    if results.is_empty() {
        println!("  无匹配结果。");
    } else {
        // iter() 迭代 &[&Book] → 每个元素是 &&Book
        // enumerate() 附加索引: (usize, &&Book)
        for (i, book) in results.iter().enumerate() {
            // i: usize (Copy), book: &&Book → 自动 deref → &Book → Display
            println!("  [{}] {}", i + 1, book);
        }
    }
}
