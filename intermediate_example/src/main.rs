//! 程序入口: 交互式 CLI 循环。
//!
//! main.rs 是二进制 crate 入口 (`cargo run` 启动的第一个函数)。
//! 负责初始化/循环/分发命令/清理, 业务逻辑委托给 service 层。
//!
//! # static mut + unsafe
//! `static mut NEXT_BOOK_ID` — 全局可变计数器, 存于数据段, 'static 生命周期。
//! 必须 unsafe 读写: 编译器无法在编译期验证全局变量的借用规则,
//! unsafe { ... } 块 = 你向编译器承诺"我不会在多线程同时读写"。
//! 本项目是单线程 CLI, 安全; 实战 prefer OnceLock / Mutex / Atomic。

use std::io::{self, Write};

// main.rs 属 binary crate, 引用 lib.rs 需通过 crate 名 (Cargo.toml 中的 name)
use intermediate_example::service::cli;
use intermediate_example::service::cli_list;
use intermediate_example::service::cli_query;

/// 全局图书 ID 计数器, 初始 1。存于数据段, 'static 生命周期。
/// 读: `unsafe { NEXT_BOOK_ID }`; 写: `unsafe { NEXT_BOOK_ID += 1; }`
static mut NEXT_BOOK_ID: u32 = 1;

fn main() {
    // mut: 后续需要 &mut library 来添加/删除/修改
    let mut library = intermediate_example::models::library::Library::new();

    println!("╔══════════════════════════════════════╗");
    println!("║   Rust 图书管理系统 — CLI 交互版    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("输入 help 查看命令, exit 退出。");
    println!();

    loop { // 无限循环, 只有 break 才能跳出
        // print! 不自动换行 → 必须手动 flush 确保 prompt 在输入前显示
        print!("> ");
        io::stdout().flush().unwrap();

        // read_line: &mut input 可变借用, 往 input 末尾追加数据 (读到 \n)
        let mut input = String::new(); // 空 String, 栈: 结构体 24B, 堆: 尚无
        if io::stdin().read_line(&mut input).is_err() {
            println!("读取输入失败, 请重试。");
            continue; // 跳回 loop 顶部
        }

        // trim(): &str 借用 input; to_lowercase(): 返回新 String (owned)
        // let input = ... 遮蔽: 新 String 绑定同名变量, 原 String drop
        let input = input.trim().to_lowercase();
        if input.is_empty() { continue; } // 空行忽略

        // split_whitespace(): 迭代器, 元素 &str 借用 input
        // ⚠️ 生命周期: parts 中的 &str 借用了 input, input 必须比 parts 活得久
        let parts: Vec<&str> = input.split_whitespace().collect();

        // as_slice(): &Vec → &[&str], 切片模式分发命令
        // _ 通配符匹配所有未覆盖情况 (编译器检查穷尽性)
        match parts.as_slice() {
            ["add"] => {
                let id = unsafe { NEXT_BOOK_ID }; // 读 static mut (Copy)
                // &mut library: 可变借用 — 要插入数据
                match cli::cmd_add(&mut library, id) {
                    Ok(assigned_id) => {
                        unsafe { NEXT_BOOK_ID = assigned_id + 1; } // 写 static mut
                        println!("添加成功! 分配的 ID: {}", assigned_id);
                    }
                    Err(e) => println!("添加失败: {}", e),
                }
            }

            // id_str: &str, 借用 parts/input, 不消耗
            ["query", id_str] => {
                cli_query::cmd_query_by_id(&library, id_str);
            }

            // keyword @ .. : keyword 绑定剩余元素 (&[&str])
            ["search", "title", keyword @ ..] => {
                let query = keyword.join(" "); // join → String (owned)
                cli_query::cmd_search_title(&library, &query); // &query: 借用
            }

            ["search", "author", keyword @ ..] => {
                let query = keyword.join(" ");
                cli_query::cmd_search_author(&library, &query);
            }

            ["list"] => {
                cli_list::cmd_list(&library); // 不可变借用
            }

            ["count"] | ["stats"] => {
                cli_list::cmd_stats(&library);
            }

            ["delete", id_str] => {
                cli::cmd_delete(&mut library, id_str); // 可变借用
            }

            ["modify", id_str] => {
                cli::cmd_modify(&mut library, id_str);
            }

            ["help"] | ["h"] => { print_help(); }

            ["exit"] | ["quit"] | ["q"] => {
                println!("再见! (所有数据已清空 — 本项目数据仅在内存中)");
                break; // 跳出 loop → main 函数结束
            }

            _ => {
                println!("未知命令: '{}'", input);
                println!("输入 help 查看可用命令。");
            }
        } // ← match 结束
        // 每次迭代末: parts (Vec<&str>) drop, input (String) drop
        // library 定义在 loop 外部, 存活

    } // ← loop 结束 (break)
    // library 离开作用域 → Library drop → HashMap drop → 每个 Book drop
    // 所有堆内存释放; NEXT_BOOK_ID 由 OS 回收
}

fn print_help() {
    // 所有参数: &str 字面量 ('static), 无堆分配, 无所有权转移
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
