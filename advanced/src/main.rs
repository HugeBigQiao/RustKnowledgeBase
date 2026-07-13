//! Rust 高级概念模块入口.
//!
//! 用法:
//!   cargo run                   列出所有模块
//!   cargo run -- smart_pointers  运行指定模块
//!
//! 每个模块覆盖一个高级主题的基本概念和代码演示.
//! 深入的实战用法放到后续的网站项目中.

mod smart_pointers;
mod interior_mutability;
mod unsafe_rust;
mod macros;
mod concurrency;
mod async_intro;
mod io_advanced;
mod networking;
mod database;
mod sandbox;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "smart_pointers" => smart_pointers::run(),
        "interior_mutability" => interior_mutability::run(),
        "unsafe_rust" => unsafe_rust::run(),
        "macros" => macros::run(),
        "concurrency" => concurrency::run(),
        "async_intro" => async_intro::run(),
        "io_advanced" => io_advanced::run(),
        "networking" => networking::run(),
        "database" => database::run(),
                "sandbox" => sandbox::run(),
        _ => {
            println!("未知模块: {}\n", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("=== Rust Advanced 模块列表 ===\n");
    println!("  smart_pointers      智能指针 (Box/Deref/Drop/Rc/Arc)");
    println!("  interior_mutability 内部可变性 (Cell/RefCell/Rc<RefCell>)");
    println!("  unsafe_rust         unsafe Rust (裸指针/unsafe 块/FFI)");
    println!("  macros              声明宏 (macro_rules!)");
    println!("  concurrency         并发 (thread/channel/Mutex/Arc)");
    println!("  async_intro         异步 (async/await/Future trait)");
    println!("  io_advanced         文件 I/O (Read/Write/BufReader/Path)");
    println!("  networking          网络 (TCP/UDP/HTTP)");
    println!("  database            数据持久化 (文件 CRUD)");
    println!();
    println!("用法: cargo run -- <模块名>");
    println!("示例: cargo run -- smart_pointers");
}
