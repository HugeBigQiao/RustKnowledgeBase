//! 程序入口: 交互式 CLI 循环。
//!
//! main.rs 是二进制 crate 入口 (`cargo run` 启动的第一个函数)。
//! main只有当你需要把这个crate当作一个需要启动的程序的时候才有用
//! 单纯的库crate，不用写main
//! 负责初始化/循环/分发命令/清理, 业务逻辑委托给 service 层。
//!
//! # static mut + unsafe
//! `static mut NEXT_BOOK_ID` — 全局可变计数器, 存于数据段, 'static 生命周期。
//! 必须 unsafe 读写: 编译器无法在编译期验证全局变量的借用规则,
//! unsafe { ... } 块 = 你向编译器承诺"我不会在多线程同时读写"。
//! 本项目是单线程 CLI, 安全; 实战 prefer OnceLock / Mutex / Atomic。

use std::io::{self, Write};

// main.rs 属 binary crate, 引用 lib.rs 需通过 crate 名 (Cargo.toml 中的 name)
use intermediate_library::service::cli;
use intermediate_library::service::cli_list;
use intermediate_library::service::cli_query;

/// 全局图书 ID 计数器, 初始 1。存于数据段, 'static 生命周期。
/// 读: `unsafe { NEXT_BOOK_ID }`; 写: `unsafe { NEXT_BOOK_ID += 1; }`
static mut NEXT_BOOK_ID: u32 = 1;

fn main() {
    // mut: 后续需要 &mut library 来添加/删除/修改
    let mut library = intermediate_library::models::library::Library::new();

    println!("╔══════════════════════════════════════╗");
    println!("║   Rust 图书管理系统 — CLI 交互版    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("输入 help 查看命令, exit 退出。");
    println!();

    loop { // 无限循环, 只有 break 才能跳出
        // 打印提示符 — print! 不自动加 \n
        print!("> ");

        // stdout(): 获取标准输出句柄 — "句柄"是操作系统的概念, 可以理解为"通往终端的通道"
        // flush(): 默认 stdout 有行缓冲: 数据先存缓冲区, 遇到 \n 才真正写到终端
        //          print! 没有 \n, 不触发刷新 → 必须手动 flush 把缓冲区内容强制写出
        // unwrap(): 取出 Result 中的 Ok 值; 如果 flush 失败则 panic (stdout 几乎不会失败)
        io::stdout().flush().unwrap();

        let mut input = String::new(); // String::new(): 创建空 String, 堆上无分配

        // stdin(): 获取标准输入句柄 — 同样是"通道", 方向是从键盘到程序
        // read_line(&mut input): 从 stdin 读一行到 input 末尾 (含 \n), 参数 &mut 是因为要写入
        // is_err(): 检查 Result 是否为 Err — 等效于 "如果失败了" (如 Ctrl+Z 关闭管道)
        if io::stdin().read_line(&mut input).is_err() {
            println!("读取输入失败, 请重试。");
            continue; // 跳回 loop 顶部, 开始下一次输入
        }

        // ── 预处理: 去空白+转小写 → 跳过空行 → 分词 → 切片模式匹配命令 ──

        // trim(): 去掉首尾空白 (\n 等), 返回 &str 借用 input
        // let input = ... 遮蔽: 新 String 绑定同名变量, 原 String (含 \n) drop
        let input = input.trim().to_lowercase();
        if input.is_empty() { continue; } // 空行忽略

        // split_whitespace(): 迭代器, 每次返回单个元素 &str 借用 input
        // ⚠️ 生命周期: parts 中的 &str 借用了 input, input 必须比 parts 活得久
        // collect(): 收集迭代器结果并转 Vec, 消耗 input
        let parts: Vec<&str> = input.split_whitespace().collect();

        // parts: Vec<&str>, 用户输入按空格分割后的词列表 (如 ["add"] 或 ["query", "3"])
        // as_slice(): 把 &Vec<&str> 转成 &[&str] (切片), 给 match 做模式匹配
        //
        // "切片模式分发": match 根据切片的长度和内容, 把控制流导向不同的处理分支
        //   比如 ["add"] → 添加图书,  ["query", id] → 查询,
        //        ["search", "title", 词..] → 书名搜索
        // _ 匹配所有未覆盖情况 — 编译器强制穷尽所有可能, 漏写分支会报错
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
