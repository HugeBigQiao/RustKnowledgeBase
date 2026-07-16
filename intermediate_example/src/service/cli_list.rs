//! CLI 展示命令: 列表 + 统计。
//!
//! 核心: Vec sort_by 排序; BTreeMap 有序输出 (分类统计);
//!       HashSet 去重 (标签); Display trait 统一格式化。

use crate::models::book::Book;
use crate::models::library::Library;

// ═══════════════════════════════════════════════════════════════
// 命令: list — 列出全部图书
// ═══════════════════════════════════════════════════════════════

/// 列出全部图书, 按年份排序。
/// - library: &Library — 不可变借用, 只读
pub fn cmd_list(library: &Library) {
    // list_all: &self → Vec<&Book>
    // 内部: values() → collect → sort_by 排序
    // 返回的 &Book 生命周期 = library 生命周期
    let books: Vec<&Book> = library.list_all();

    if books.is_empty() {
        println!("图书馆是空的, 先用 add 命令添加图书吧。");
        return; // 提前退出, books 在此 drop (Vec 释放, 但 &Book 引用不影响 library)
    }

    println!("══════ 全部藏书 ({} 本, 按年份排序) ══════", books.len());

    // iter().enumerate(): 产出 (usize, &&Book)
    // &&Book → 自动 deref → &Book → Display::fmt
    for (i, book) in books.iter().enumerate() {
        println!("  #{:<3} {}", i + 1, book); // {:<3}: 左对齐占 3 格
    }
} // books 离开作用域, Vec drop (内部 &Book 引用不影响 library)

// ═══════════════════════════════════════════════════════════════
// 命令: count/stats — 显示统计信息
// ═══════════════════════════════════════════════════════════════

/// 显示藏书统计: 总数 + 各分类数量 (BTreeMap 有序) + 标签种类 (HashSet 去重)。
/// - library: &Library — 不可变借用
pub fn cmd_stats(library: &Library) {
    // stats: &self → LibraryStats (owned, 非引用!)
    // 内部数据是克隆/收集出来的, 独立于 library — 即使 library 被修改也不受影响
    let stats = library.stats();

    println!("══════ 图书馆统计 ══════");
    println!("总藏书: {} 本", library.book_count()); // usize Copy, 自动复制

    // stats 自动借用为 &LibraryStats → Display::fmt
    // fmt 接收 &self, stats 不会被消费
    println!("{}", stats);
} // stats drop: BTreeMap + HashSet + String 全部释放, library 不受影响
