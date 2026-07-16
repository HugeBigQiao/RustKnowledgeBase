//! CLI 命令处理: 每个函数对应一个用户命令。
//!
//! 本文件集中展示 intermediate 的核心概念在实战中的应用:
//!
//! **所有权 & 借用:**
//! - `&Library` — 只读查询, 不消耗 library
//! - `&mut Library` — 修改操作, 可变借用
//! - `Option<T>` — 可能不存在的值 (get_book 返回 Option<&Book>)
//! - `Result<T, E>` — 可能失败的操作 (add/remove/modify)
//!
//! **模式匹配:**
//! - `if let Some(x) = opt` — 只关心 Some 分支
//! - `match result { Ok(v) => ..., Err(e) => ... }` — 穷尽处理
//!
//! **中间知识点:**
//! - `?` 运算符 — 错误传播
//! - `String` → `&str` 转换 — trim, parse, to_lowercase
//! - 迭代器 + collect — search 返回 Vec
//! - Vec 高级 — sort_by, filter
//! - HashMap/Vec/BTreeMap — 统计信息
//! - Display trait — 格式化输出

use std::io::{self, Write};

use crate::error::LibraryError;
use crate::models::book::Book;
use crate::models::category::Category;
use crate::models::library::Library;

// ── 辅助: 读取一行输入 ──
// 所有权: 返回 String (owned), 调用方获得所有权。
fn read_line(prompt: &str) -> String {
    print!("{}", prompt);                            // &str 借用, print! 只读
    io::stdout().flush().unwrap();
    let mut input = String::new();
    // read_line 需要 &mut input — 可变借用, 追写数据到 String
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()                         // trim() 返回 &str (借用) → to_string() 创建新 String (owned)
}

// ── 辅助: 解析 u32 ──
// 所有权: s: &str — 借用, 不消耗。
fn parse_id(s: &str) -> Result<u32, String> {
    // parse::<u32>() 返回 Result<u32, ParseIntError>
    s.parse::<u32>().map_err(|e| format!("'{}' 不是有效的数字: {}", s, e))
    // map_err: 把 ParseIntError → String, 统一错误类型
}

// ═══════════════════════════════════════════════════════════════
// 命令: add — 添加新书
// ═══════════════════════════════════════════════════════════════

/// 交互式添加图书: 依次询问书名/作者/分类/年份/标签。
/// 参数: library — &mut Library, 可变借用 (要插入数据)。
///       id — 由调用方从 static mut 全局计数器获取。
/// 返回: Result<u32, LibraryError> — 成功返回分配的 ID。
pub fn cmd_add(library: &mut Library, id: u32) -> Result<u32, LibraryError> {
    println!("══════ 添加新书 (ID: {}) ══════", id);

    // 书名 — read_line 返回 String (owned), 所有权移给 title
    let title = read_line("书名: ");
    // 作者
    let author = read_line("作者: ");

    // 分类 — 展示选项, 用户输入或用数字选择
    println!("分类: 1=小说 2=科学 3=历史 4=技术 5=哲学 6=其他(自定义)");
    let cat_input = read_line("选择 (1-6 或分类名): ");
    let category = match cat_input.as_str() {
        // 数字选择 — Category 枚举变体
        "1" => Category::Fiction,
        "2" => Category::Science,
        "3" => Category::History,
        "4" => Category::Technology,
        "5" => Category::Philosophy,
        "6" => {
            let other = read_line("  自定义分类名: ");
            Category::Other(other)                   // String 移入 Other 变体
        }
        // 文字选择 — 使用 From<&str> trait 自动转换
        other => Category::from(other),             // &str → Category, Category::from 在 category.rs 实现
    };
    println!("  已选择: {}", category);             // Display trait: Category 实现了 fmt::Display

    // 年份 — parse 返回 Result, 失败给默认
    let year_str = read_line("出版年份: ");
    let year: u32 = year_str.parse().unwrap_or(0);  // unwrap_or: 解析失败给默认值 0, 不崩溃

    // 标签 — 逗号分隔, 转 Vec<&str>
    let tags_str = read_line("标签 (逗号分隔, 如: rust,编程): ");
    let tags: Vec<&str> = if tags_str.is_empty() {
        vec![]
    } else {
        // split 返回迭代器, 元素是 &str (借用 tags_str)
        // map(|s| s.trim()) 去空白 → collect 进 Vec
        // 注意: tags 的 &str 元素借用了 tags_str!
        // 但 collect 后 tags_str 不再使用, 且下面的 add_book 会立刻把 &str 转成 String,
        // 所以这里借用关系安全。
        tags_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
        // filter(|s| !s.is_empty()): 过滤空标签 (用户可能多打了逗号)
    };

    // 调用 Library::add_book — id 由参数传入, 不再内部分配
    library.add_book(id, title, author, category, year, tags)
    // 所有权转移: title/author/category/tags 的所有权移入 add_book → 移入 Book → 移入 HashMap
    // 调用方在此之后不能再访问这些变量。
}

// ═══════════════════════════════════════════════════════════════
// 命令: query <ID> — 按 ID 查询
// ═══════════════════════════════════════════════════════════════

/// 按 ID 查询图书。
/// 参数: library — &Library (不可变借用, 只读)。
///       id_str — &str (借用, 不消耗)。
pub fn cmd_query_by_id(library: &Library, id_str: &str) {
    match parse_id(id_str) {
        Ok(id) => {
            // get_book 返回 Option<&Book> — 找到了返回 Some, 没找到返回 None
            match library.get_book(id) {
                Some(book) => println!("{}", book),  // Display trait: Book 实现了 fmt::Display
                None => println!("未找到 ID={} 的图书。", id),
            }
        }
        Err(e) => println!("{}", e),
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: search title <关键词> — 按书名搜索
// ═══════════════════════════════════════════════════════════════

/// 按书名模糊搜索。
/// 参数: query — &str (借用), 在 search_by_title 内部与每个书名比较。
pub fn cmd_search_title(library: &Library, query: &str) {
    // search_by_title 返回 Vec<&Book> — 元素是 &Book (借用 library 里的数据)
    // 生命周期: 返回的引用不能超过 library 的生命周期 (编译器自动检查)
    let results: Vec<&Book> = library.search_by_title(query);

    print_search_results(query, "书名", &results);
}

// ═══════════════════════════════════════════════════════════════
// 命令: search author <关键词> — 按作者搜索
// ═══════════════════════════════════════════════════════════════

/// 按作者模糊搜索。
pub fn cmd_search_author(library: &Library, query: &str) {
    let results: Vec<&Book> = library.search_by_author(query);
    print_search_results(query, "作者", &results);
}

/// 辅助: 打印搜索结果。
fn print_search_results(query: &str, field: &str, results: &[&Book]) {
    // &[&Book]: 借切片, 不消耗 Vec — 调用后原 Vec 仍可用
    println!("搜索{}含 '{}': {} 本", field, query, results.len());
    if results.is_empty() {
        println!("  无匹配结果。");
    } else {
        for (i, book) in results.iter().enumerate() {
            // enumerate(): (索引, 元素) 迭代器 — 给结果编号
            println!("  [{}] {}", i + 1, book);
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: list — 列出全部图书
// ═══════════════════════════════════════════════════════════════

/// 列出全部图书 (按年份+书名排序)。
pub fn cmd_list(library: &Library) {
    let books: Vec<&Book> = library.list_all();
    // list_all 内部: collect 所有 &Book → sort_by 排序 (闭包比较年份)
    // 返回 Vec<&Book>: 元素是 &Book, 借用 library 里的数据

    if books.is_empty() {
        println!("图书馆是空的, 先用 add 命令添加图书吧。");
        return;
    }

    println!("══════ 全部藏书 ({} 本, 按年份排序) ══════", books.len());
    for (i, book) in books.iter().enumerate() {
        // ID 不在 Book 里 — 需要从底层查。这里用位置序号替代。
        println!("  #{:<3} {}", i + 1, book);
        // {:<3}: 左对齐占 3 格, 对齐美观
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: count/stats — 显示统计信息
// ═══════════════════════════════════════════════════════════════

/// 显示藏书统计: 总数 + 各分类数量 (BTreeMap 有序) + 标签种类 (HashSet 去重)。
pub fn cmd_stats(library: &Library) {
    // stats() 返回 LibraryStats: 内部含 BTreeMap 分类计数 + HashSet 标签收集
    let stats = library.stats();

    // 简单计数 — 直接取 len()
    println!("══════ 图书馆统计 ══════");
    println!("总藏书: {} 本", library.book_count());

    // 详细统计 — LibraryStats 实现了 Display trait
    // BTreeMap 保证分类按字典序输出 (HashMap 无序!)
    println!("{}", stats);
}

// ═══════════════════════════════════════════════════════════════
// 命令: delete <ID> — 删除图书
// ═══════════════════════════════════════════════════════════════

/// 删除指定 ID 的图书。
/// 参数: library — &mut Library, 可变借用 (要删除数据)。
pub fn cmd_delete(library: &mut Library, id_str: &str) {
    match parse_id(id_str) {
        Ok(id) => {
            // remove_book 返回 Result<Book, LibraryError>
            // 所有权: Book 的所有权从 HashMap 中移出, 交给调用方。
            match library.remove_book(id) {
                Ok(book) => println!("已删除: {} (ID={})", book.title, id),
                // book 在此离开作用域 → Book 被 drop → 其 String 字段释放
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => println!("{}", e),
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: modify <ID> — 修改图书
// ═══════════════════════════════════════════════════════════════

/// 交互式修改图书: 先展示当前信息, 再逐字段询问 (回车跳过=不修改)。
pub fn cmd_modify(library: &mut Library, id_str: &str) {
    let id = match parse_id(id_str) {
        Ok(id) => id,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // 先展示当前信息 — 确认用户要改哪本
    match library.get_book(id) {
        Some(book) => println!("当前信息: {}", book),
        None => {
            println!("未找到 ID={} 的图书。", id);
            return;
        }
    }

    println!("请逐项输入新值 (直接回车=不修改):");
    println!("──────────────────────────────");

    // 书名 — 回车跳过 → 传 None
    let title_input = read_line("新书名 [回车跳过]: ");
    let title = if title_input.is_empty() { None } else { Some(title_input) };
    // Option<String>: None = 不修改此字段; Some(s) = 更新为 s

    // 作者
    let author_input = read_line("新作者 [回车跳过]: ");
    let author = if author_input.is_empty() { None } else { Some(author_input) };

    // 分类 — 也支持回车跳过
    println!("新分类: 1=小说 2=科学 3=历史 4=技术 5=哲学 6=其他 [回车跳过]");
    let cat_input = read_line("选择: ");
    let category: Option<Category> = if cat_input.is_empty() {
        None                                          // None → 不修改分类
    } else {
        Some(match cat_input.as_str() {
            "1" => Category::Fiction,
            "2" => Category::Science,
            "3" => Category::History,
            "4" => Category::Technology,
            "5" => Category::Philosophy,
            "6" => {
                let other = read_line("  自定义分类名: ");
                Category::Other(other)
            }
            other => Category::from(other),
        })
    };

    // 年份
    let year_input = read_line("新出版年份 [回车跳过]: ");
    let year: Option<u32> = if year_input.is_empty() {
        None
    } else {
        Some(year_input.parse().unwrap_or(0))
    };

    // 标签
    let tags_input = read_line("新标签 (逗号分隔) [回车跳过]: ");
    let tags: Option<Vec<&str>> = if tags_input.is_empty() {
        None
    } else {
        // split → map trim → filter 非空 → collect
        Some(tags_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect())
    };

    // 调用 modify_book — 所有权: 所有 Some 值移入 library
    match library.modify_book(id, title, author, category, year, tags) {
        Ok(book) => {
            println!("──────────────────────────────");
            println!("修改成功: {}", book);
        }
        Err(e) => println!("修改失败: {}", e),
    }
}

