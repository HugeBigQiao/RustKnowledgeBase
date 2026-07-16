//! 程序入口: 交互式 CLI 循环。
//!
//! 运行 `cargo run` 进入交互界面, 输入命令管理图书。
//! 所有数据仅在内存中, 退出即清空 — 这是一个临时演示项目。
//!
//! ## 涉及的知识点
//!
//! 本文件集中展示 basic 和 intermediate 的核心概念:
//!
//! **basic:**
//! - loop / match — 主循环 + 命令分发
//! - String 操作 — trim、split_whitespace、to_lowercase
//! - 所有权 — &mut self 借用, stdin 读取
//! - println!/format! — 格式化输出
//!
//! **intermediate:**
//! - static mut + unsafe — 全局可变 ID 计数器
//! - 模式匹配 — 切片模式 [cmd, arg] 分发
//! - Vec — 命令行参数切片
//!
//! ## static mut 是什么? 为什么需要 unsafe?
//!
//! `static mut NEXT_BOOK_ID: u32 = 1;` 是"全局可变的静态变量"。
//! 它存活于整个程序运行期间 ('static 生命周期), 放在数据段而非堆/栈上。
//!
//! 为什么必须 unsafe 访问? 因为多个线程同时读写 static mut 会导致数据竞争。
//! Rust 的 borrow checker 无法在编译期检查全局变量的借用规则,
//! 所以把"安全责任"交给你: 你必须自己保证不会同时读写。
//!
//! 本项目是单线程交互程序, 没有并发问题, unsafe 是安全的。
//! 实战中 prefer 用 OnceLock / Mutex / Atomic 替代 static mut。

use std::io::{self, Write};

use intermediate_example::service::cli;

/// 全局图书 ID 计数器, 初始为 1。
/// 每次添加图书时读取当前值, 然后 +1 — 保证 ID 唯一且连续。
///
/// unsafe 块中的读写: `unsafe { NEXT_BOOK_ID }` 获取值;
/// `unsafe { NEXT_BOOK_ID += 1; }` 递增。
static mut NEXT_BOOK_ID: u32 = 1;

fn main() {
    // library 拥有 Library 实例 (含 HashMap<Book> 等)
    let mut library = intermediate_example::models::library::Library::new();

    println!("╔══════════════════════════════════════╗");
    println!("║   Rust 图书管理系统 — CLI 交互版    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("输入 help 查看命令, exit 退出。");
    println!();

    // ── 主循环: 不断读取用户输入 ──
    loop {
        // 打印提示符并 flush — 确保光标前先看到 "> "
        print!("> ");
        io::stdout().flush().unwrap();

        // 读取一行输入 — read_line 追加到 String (需要 &mut self, 可变借用)
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("读取输入失败, 请重试。");
            continue;                              // continue: 跳回 loop 开头
        }

        // 所有权: trim() 返回 &str (借用 input), to_lowercase() 返回新 String (owned)
        // split_whitespace() 返回迭代器, collect 进 Vec<&str> — 每个元素借用 input
        let input = input.trim().to_lowercase();
        if input.is_empty() {
            continue;                              // 空行忽略
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        // parts 借用 input; input 存活期间 parts 才能用 — 两者在同一作用域内。

        // ── 模式匹配分发命令 ──
        // 使用切片模式匹配 [cmd], [cmd, arg], [cmd, arg, arg2] 等不同长度。
        match parts.as_slice() {
            // === 添加图书 ===
            ["add"] => {
                // 从 static mut 读取当前 ID — unsafe 块是开发者在说 "我保证不会数据竞争"
                let id = unsafe { NEXT_BOOK_ID };
                match cli::cmd_add(&mut library, id) {
                    Ok(assigned_id) => {
                        // 添加成功 → 递增全局计数器
                        unsafe { NEXT_BOOK_ID = assigned_id + 1; }
                        println!("添加成功! 分配的 ID: {}", assigned_id);
                    }
                    Err(e) => println!("添加失败: {}", e),
                }
            }

            // === 按 ID 查询 ===
            ["query", id_str] => {
                cli::cmd_query_by_id(&library, id_str);
            }

            // === 按书名搜索 (模糊) ===
            ["search", "title", keyword @ ..] => {
                let query = keyword.join(" ");     // 拼接多词为完整搜索词
                cli::cmd_search_title(&library, &query);
            }

            // === 按作者搜索 (模糊) ===
            ["search", "author", keyword @ ..] => {
                let query = keyword.join(" ");
                cli::cmd_search_author(&library, &query);
            }

            // === 列出全部图书 (按年份排序) ===
            ["list"] => {
                cli::cmd_list(&library);
            }

            // === 统计信息 (总数 + 分类分布) ===
            ["count"] | ["stats"] => {
                cli::cmd_stats(&library);
            }

            // === 删除图书 ===
            ["delete", id_str] => {
                cli::cmd_delete(&mut library, id_str);
            }

            // === 修改图书 ===
            ["modify", id_str] => {
                cli::cmd_modify(&mut library, id_str);
            }

            // === 帮助 ===
            ["help"] | ["h"] => {
                print_help();
            }

            // === 退出 ===
            ["exit"] | ["quit"] | ["q"] => {
                println!("再见! (所有数据已清空 — 本项目数据仅在内存中)");
                break;                             // break 跳出 loop
            }

            // === 未知命令 ===
            _ => {
                println!("未知命令: '{}'", input);
                println!("输入 help 查看可用命令。");
            }
        }
    } // loop 结束 — library 在这里 drop, 所有图书数据释放
}

/// 打印帮助信息。
fn print_help() {
    println!("══════════ 可用命令 ══════════");
    println!("  add                   添加新书 (交互式填写)");
    println!("  query <ID>            按 ID 查询图书");
    println!("  search title <关键词>  按书名模糊搜索");
    println!("  search author <关键词> 按作者模糊搜索");
    println!("  list                  列出全部图书 (按年份排序)");
    println!("  count (或 stats)      显示藏书统计");
    println!("  delete <ID>           删除指定图书");
    println!("  modify <ID>           修改指定图书");
    println!("  help (或 h)           显示本帮助");
    println!("  exit (或 quit/q)      退出程序");
    println!("═══════════════════════════════");
}
