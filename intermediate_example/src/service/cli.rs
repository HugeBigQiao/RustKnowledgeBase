//! CLI 修改命令: 添加 / 删除 / 修改图书。
//!
//! 核心: 所有权转移 — String 参数移入函数 → 最终移入 HashMap;
//!       可变借用 &mut Library — 写操作独占, 读操作共享;
//!       Result<T,E> — 不崩溃, 不 panic;
//!       Option<T> 做"可选更新"载体 (modify_book): None = 不改, Some(v) = 更新。

use std::io::{self, Write};

use crate::error::LibraryError;
use crate::models::category::Category;
use crate::models::library::Library;

// ── 辅助: 读取一行输入 ──
// prompt: &str — 借用, 不消耗; 返回 String (owned) — 所有权移给调用方
pub(crate) fn read_line(prompt: &str) -> String {
    // 为什么先 print 再 flush? stdout 有行缓冲, print! 没有 \n, 必须手动 flush 才能显示
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // flush → Result<()>, unwrap 取出值 (通常不会失败)

    let mut input = String::new(); // 空 String, 栈: 结构体 24B, 堆: 尚无分配
    // &mut input: 可变借用 — read_line 往 input 里追加数据 (读到 \n 为止)
    io::stdin().read_line(&mut input).unwrap();

    // trim() → &str (借用 input, 不分配新内存)
    // to_string() → 新 String (owned), 复制数据到堆上, 所有权返回给调用方
    // 返回后 input 在此函数结束时 drop
    input.trim().to_string()
}

// ── 辅助: 解析 u32 ──
// s: &str — 借用, 不消耗; 返回 Result<u32, String>
// 用 String 而非 ParseIntError 做错误类型 — 方便直接 println! 打印
pub(crate) fn parse_id(s: &str) -> Result<u32, String> {
    s.parse::<u32>() // <u32>: turbofish 指定泛型; &self = &str, 不消耗 s
        .map_err(|e| format!("'{}' 不是有效的数字: {}", s, e))
    // map_err: 只在 Err 时执行, 把 ParseIntError → String; Ok 分支原样传递
}

// ═══════════════════════════════════════════════════════════════
// 命令: add — 添加新书
// ═══════════════════════════════════════════════════════════════

/// 交互式添加图书。
/// - library: &mut Library — 可变借用, 要插入数据
/// - id: u32 — Copy, 传值时自动复制
/// 返回 Result<u32, LibraryError> — 成功返回分配的 ID
pub fn cmd_add(library: &mut Library, id: u32) -> Result<u32, LibraryError> {
    println!("══════ 添加新书 (ID: {}) ══════", id);

    // 书名: read_line → String (owned), 所有权 → title
    let title = read_line("书名: ");
    // 作者: 同理
    let author = read_line("作者: ");

    // 分类: 展示选项 → 读输入 → match 匹配
    println!("分类: 1=小说 2=科学 3=历史 4=技术 5=哲学 6=其他(自定义)");
    let cat_input = read_line("选择 (1-6 或分类名): "); // String owned → cat_input

    // as_str(): &String → &str (借用 cat_input), 生命周期 = cat_input 生命周期
    let category = match cat_input.as_str() {
        "1" => Category::Fiction,
        "2" => Category::Science,
        "3" => Category::History,
        "4" => Category::Technology,
        "5" => Category::Philosophy,
        "6" => {
            let other = read_line("  自定义分类名: "); // String owned
            Category::Other(other) // other 所有权移入 Other 变体
        }
        other => Category::from(other), // From<&str> trait: &str → Category
    }; // cat_input 不再使用, 作用域结束时 drop
    println!("  已选择: {}", category); // Display trait

    // 年份: parse → unwrap_or(0) 提供默认值, 不崩溃
    let year_str = read_line("出版年份: ");
    let year: u32 = year_str.parse().unwrap_or(0);
    // year: u32 (Copy), year_str 不再使用但还存活

    // 标签: split → map trim → filter → collect
    let tags_str = read_line("标签 (逗号分隔, 如: rust,编程): ");
    let tags: Vec<&str> = if tags_str.is_empty() {
        vec![]
    } else {
        // split(',') 返回迭代器, 元素 &str 借用 tags_str
        tags_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
        // ⚠️ 生命周期: collect 后 tags 中的 &str 借用了 tags_str
        // 但下一行 add_book 马上把 &str → String, tags_str 在此之前存活, 安全
    };

    // ── 所有权集中转移 ──
    // title/author/category/tags 所有权全部移入 add_book → Book → HashMap
    // 此后调用方不能再使用这些变量
    library.add_book(id, title, author, category, year, tags)
}

// ═══════════════════════════════════════════════════════════════
// 命令: delete <ID> — 删除图书
// ═══════════════════════════════════════════════════════════════

/// 删除指定 ID 的图书。
/// - library: &mut Library — 可变借用 (要从 HashMap 中 remove)
/// - id_str: &str — 借用, 只解析不消耗
pub fn cmd_delete(library: &mut Library, id_str: &str) {
    match parse_id(id_str) { // id_str: &str 借用, parse 只读
        Ok(id) => { // id: u32 (Copy)
            // remove_book: &mut self → Result<Book, LibraryError>
            // Book 的所有权从 HashMap 中移出!
            match library.remove_book(id) {
                Ok(book) => {
                    // book: Book (owned!), book.title 借给 println!
                    println!("已删除: {} (ID={})", book.title, id);
                } // book 离开作用域 → Book drop, 所有 String 字段释放
                Err(e) => println!("{}", e), // e: LibraryError, 借给 println!
            }
        }
        Err(e) => println!("{}", e), // e: String owned, 从 Err 移出, 借给 println!, 后 drop
    }
}

// ═══════════════════════════════════════════════════════════════
// 命令: modify <ID> — 修改图书
// ═══════════════════════════════════════════════════════════════

/// 交互式修改图书: 先展示当前信息, 再逐字段询问 (回车跳过 = 不修改)。
/// - library: &mut Library — 可变借用
/// - id_str: &str — 借用
///
/// Option 作"可选更新"载体: None = 不修改此字段, Some(v) = 更新为 v。
pub fn cmd_modify(library: &mut Library, id_str: &str) {
    // ── 解析 ID ──
    let id = match parse_id(id_str) {
        Ok(id) => id, // id 作为 match 表达式的值
        Err(e) => { println!("{}", e); return; }
    };

    // ── 展示当前信息 ──
    // get_book: &self → Option<&Book>; &Book 借用 library 里的数据
    match library.get_book(id) {
        Some(book) => println!("当前信息: {}", book), // Display trait
        None => { println!("未找到 ID={} 的图书。", id); return; }
    } // get_book 的不可变借用结束

    println!("请逐项输入新值 (直接回车=不修改):");
    println!("──────────────────────────────");

    // ── 书名 ──
    let title_input = read_line("新书名 [回车跳过]: "); // String owned
    // is_empty → None (不修改); 否则 Some(输入) — 所有权移入 Some
    let title = if title_input.is_empty() { None } else { Some(title_input) };

    // ── 作者 ──
    let author_input = read_line("新作者 [回车跳过]: ");
    let author = if author_input.is_empty() { None } else { Some(author_input) };

    // ── 分类 ──
    println!("新分类: 1=小说 2=科学 3=历史 4=技术 5=哲学 6=其他 [回车跳过]");
    let cat_input = read_line("选择: ");
    let category: Option<Category> = if cat_input.is_empty() {
        None
    } else {
        Some(match cat_input.as_str() {
            "1" => Category::Fiction,
            "2" => Category::Science,
            "3" => Category::History,
            "4" => Category::Technology,
            "5" => Category::Philosophy,
            "6" => { let other = read_line("  自定义分类名: "); Category::Other(other) }
            other => Category::from(other), // From trait
        })
    };

    // ── 年份 ──
    let year_input = read_line("新出版年份 [回车跳过]: ");
    let year: Option<u32> = if year_input.is_empty() { None }
                            else { Some(year_input.parse().unwrap_or(0)) };

    // ── 标签 ──
    let tags_input = read_line("新标签 (逗号分隔) [回车跳过]: ");
    let tags: Option<Vec<&str>> = if tags_input.is_empty() {
        None
    } else {
        // ⚠️ 生命周期: Vec<&str> 中的 &str 借用了 tags_input
        // modify_book 内部会把 &str → String, tags_input 在此之前存活, 安全
        Some(tags_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect())
    };

    // ── 调用 modify_book ──
    // 签名: fn modify_book(&mut self, id: u32, title: Option<String>, ...)
    //           → Result<&Book, LibraryError>
    // 所有权: id/year (Copy) 自动复制; Option 中 Some(v) 移入 library
    // 返回值: Ok(&book) — &Book 借用 library 里已修改的数据
    match library.modify_book(id, title, author, category, year, tags) {
        Ok(book) => {
            println!("──────────────────────────────");
            println!("修改成功: {}", book);
        }
        Err(e) => println!("修改失败: {}", e),
    }
    // title_input/author_input/... 中已移入 Option 的数据不受影响 (所有权已转移)
    // 未移入的数据 (None 分支) 在此 drop
}

