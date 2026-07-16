//! ## 什么是 Rust？
//!   Rust 是一门系统级编程语言，由 Mozilla 开发，专注三个目标：  
//!     1. 内存安全 —— 不需要垃圾回收，也不会有空指针/悬垂指针  
//!     2. 零成本抽象 —— 高级语法不带来额外运行时开销  
//!     3. 并发安全 —— 编译期就杜绝数据竞争  
//!   适用场景：操作系统、浏览器引擎、Web 后端、命令行工具、嵌入式等  
//!
//! ## 什么是 Cargo？
//!   Cargo 是 Rust 的包管理器和构建工具，相当于 Node 的 npm + 构建脚本  
//!   一个 Rust 项目 = 一个 crate（编译单元），Cargo 管理一切。  
//!
//! ### Cargo 常用命令：
//!   `cargo new <name>`      创建新项目  
//!   `cargo init`             在当前目录初始化项目  
//!   `cargo build`            编译项目（debug 模式，快但未优化）  
//!   `cargo build --release`  编译项目（release 模式，优化后较慢编译但运行快）  
//!   `cargo run`              编译并运行项目  
//!   `cargo run -- xx`        运行项目并传递参数 xx  
//!   `cargo check`            只检查能不能编译（不生成二进制，速度快）  
//!   `cargo test`             运行测试（含文档测试）  
//!   `cargo doc`              生成 HTML 文档到 target/doc/  
//!   `cargo doc --open`       生成文档并在浏览器打开  
//!   `cargo clean`            删除 target/ 目录（清理编译产物）  
//!   `cargo update`           更新 Cargo.lock 中的依赖版本  
//!   `cargo fmt`              用 rustfmt 自动格式化代码  
//!   `cargo clippy`           用 clippy 做代码静态检查/改进建议  
//!
//! ## 什么是注释？
//!   注释是写给人看的说明文字，编译器完全忽略，不影响程序运行。
//!
//! ### Rust 有三种注释符号：
//!
//! | 符号 | 用途 |
//! |:---|:---|
//! | `//` | 普通注释：给开发者看，不生成文档 |
//! | `///` | 文档注释：给下一项（函数/结构体）写说明书 |
//! | `//!` | 模块文档：给整个文件/模块写说明书（放文件顶） |  
//!
//! - `//`   → 原来的普通注释，不出现在 cargo doc 里  
//! - `///`  → cargo doc 会收集它，生成 HTML 文档  
//! - `//!`  → 你正在看的这些！cargo doc 把它们当作本文件的说明书  
//!
//! ## 什么是 Doc 文档？
//!   Rust 的文档注释（/// 和 //!）是语言内置功能，不是第三方工具。
//!
//! ### 怎么生成文档？
//! - `cargo doc`             扫描源码中的 /// 和 //!，生成 HTML 到 target/doc/  
//! - `cargo doc --open`      生成后自动在浏览器打开  
//! - `cargo doc --no-deps`   只给自己代码生成文档，不包含第三方依赖  
//!
//! ### 怎么"打开"文档？
//! - 方式 1：`cargo doc --open` → 自动在浏览器打开本地文档首页  
//! - 方式 2：直接打开 `target/doc/<crate名>/index.html`  
//! - 方式 3：`rustup doc`        → 打开 Rust 标准库的离线文档  
//!
//! ### 文档里有什么？
//! - 你在 /// 和 //! 里写的所有内容（支持 Markdown）  
//! - 自动生成的函数签名、类型信息  
//! - 源码链接（点一下跳到对应行）  
//! - 搜索栏（搜函数名、类型名）  
//!
//! ## Rust 基本项目结构（cargo new 生成的长这样）
//!   my_project/          ← 项目根目录（名字你自己起）  
//!   ├── Cargo.toml       ← 项目配置文件（元数据 + 依赖）  
//!   ├── Cargo.lock       ← 依赖版本锁定文件（自动生成，别手动改）  
//!   ├── src/             ← 源码目录，所有 .rs 文件放这里  
//!   │   └── main.rs      ← 程序入口（二进制 crate）  
//!   └── target/          ← 编译产物目录（自动生成，可删除）  
//!       ├── debug/       ← cargo build 的结果（未优化）  
//!       ├── release/     ← cargo build --release 的结果（优化后）  
//!       └── doc/         ← cargo doc 生成的 HTML 文档  
//!
//! ## 什么是 src/？
//!   src 是 source 的缩写，所有 Rust 源代码（.rs 文件）都放在这个目录里。  
//!   Cargo 默认从这里找代码编译。如果你把 .rs 文件放在别处，Cargo 找不到。  
//!   你写的所有代码都在 src/ 下面——这是你的主战场。  
//!
//! ## 什么是 target/？
//!   target/ 是编译产物的存放目录，Cargo 自动创建，可以随时删掉（cargo clean）。
//!
//! ### target/ 下的子目录：
//!
//!   target/
//!   ├── debug/               ← cargo build 的输出（开发模式，编译快）  
//!   │   ├── basic.exe         ← 可执行文件（Windows 是 .exe）  
//!   │   ├── basic             ← 可执行文件（Linux/Mac 无后缀）  
//!   │   ├── basic.pdb         ← 调试符号文件（Windows，用于断点调试）  
//!   │   ├── .fingerprint/     ← 增量编译缓存（哪些文件改过要重编译）  
//!   │   ├── build/            ← 构建脚本（build.rs）的输出  
//!   │   ├── deps/             ← 依赖库的编译结果  
//!   │   └── incremental/      ← 增量编译的中间文件  
//!   ├── release/              ← cargo build --release 的输出（发布模式）  
//!   │   (结构同上，编译更慢但运行更快、体积更小)  
//!   └── doc/                  ← cargo doc 生成的 HTML 文档  
//!
//!   核心结论：target/ 可以随便删，下次 cargo build 会自动重建。
//!   提交 Git 时一定把 target/ 写进 .gitignore（太大了，没必要跟踪）。
//!
//! ## Cargo.toml 和 Cargo.lock 是什么？
//!
//! ### Cargo.toml（你写的）
//! - 项目"身份证"：包名、版本、作者、Rust edition  
//! - 依赖清单：需要哪些第三方库，版本要求是什么  
//! - 就是你在 `[dependencies]` 里写 `serde = "1.0"` 的地方  
//! - 你手动编辑，Cargo 读取  
//!
//! ### Cargo.lock（Cargo 自动生成的）
//! - 锁定依赖的精确版本（比如 serde 到底是 1.0.210 还是 1.0.215）  
//! - 保证任何人、任何机器 clone 项目后编译结果完全一致  
//! - 不要手动改这个文件！Cargo 自己维护  
//! - 二进制项目提交到 Git，库项目可以不提交  
//!
//! ### 类比理解：
//! - Cargo.toml = 你说"我要 serde 1.x 系列"（范围）  
//! - Cargo.lock = 系统记"现在装的 serde 1.0.215"（精确版本）  
//!
//! ## 哪些可以改？哪些是固定的？
//!
//! ### ✅ 你随便改的：
//! - src/ 里的所有 .rs 文件（你的代码你做主）  
//! - Cargo.toml 里的依赖、项目信息、编译配置  
//! - 你可以加 src/lib.rs（库）、tests/（集成测试）、examples/（示例）  
//!
//! ### ⚠️ 按需改的（有固定格式，不能瞎写）：
//! - Cargo.toml 必须遵循 TOML 语法  
//! - 模块路径（文件放在哪、mod 怎么声明）有固定规则  
//!
//! ### ❌ 别手动碰的：
//! - target/ 整个目录（编译产物，cargo 全权管理）  
//! - Cargo.lock（cargo 自动生成和维护）  
//! - target/ 下面任何文件都不要手动改  
//!
//!   简单记：你只写 src/ 里的代码和改 Cargo.toml 的依赖，  
//!   其他都是 cargo 的事。

// mod 关键字：声明一个子模块。模块是 Rust 的代码组织方式，一个文件 = 一个模块。
// mod base_type; 意思是引入 base_type.rs 这个文件作为 base_type 模块
mod base_type;
mod hello_world;
mod bit_ops;
mod operator;
mod compound_types;
mod if_flow;
mod closure;
mod while_flow;
mod loop_flow;
mod for_flow;
mod match_flow;
mod return_flow;
mod ownership_and_refs;
mod chain_call;
mod vec_type;
mod vec_advanced;
mod sandbox;
mod fizzbuzz;
mod score_analyzer;

fn main() {
    // ===== std::env::args: 读取命令行参数 =====
    // `std::env::args()` 返回迭代器，跳过第一个(程序名)
    // `.nth(1)` 取第二个参数(即用户输入的第一个参数)
    // 所有权: nth(1) 返回 Option<String> — arg 拥有这个 String
    let arg = std::env::args().nth(1);

    // match 匹配 Option 的 Some/None
    // 所有权: as_deref() 将 Option<String> → Option<&str>
    //   不拿走所有权 — 只是借用 arg 内部 String 的视图
    match arg.as_deref() {
        // `as_deref()`: Option<String> → Option<&str> (不拿走所有权)
        Some("base_type") => base_type::run(),
        Some("hello_world") => hello_world::run(),
        Some("bit_ops") => bit_ops::run(),
        Some("operator") => operator::run(),
        Some("compound_types") => compound_types::run(),
        Some("if_flow") => if_flow::run(),
        Some("closure") => closure::run(),
        Some("while_flow") => while_flow::run(),
        Some("loop_flow") => loop_flow::run(),
        Some("for_flow") => for_flow::run(),
        Some("match_flow") => match_flow::run(),
        Some("return_flow") => return_flow::run(),
        Some("ownership_and_refs") => ownership_and_refs::run(),
        Some("chain_call") => chain_call::run(),
        Some("vec_type") => vec_type::run(),
        Some("vec_advanced") => vec_advanced::run(),
        Some("sandbox") => sandbox::run(),
        Some("fizzbuzz") => fizzbuzz::run(),
        Some("score_analyzer") => score_analyzer::run(),
        Some(other) => {
            println!("未知模块: {}", other);
            print_help();
        }
        None => print_help(),
    }
}

/// 打印用法和可用模块列表
fn print_help() {
    println!("用法: cargo run -- <模块名>");
    println!("可用模块:");
    println!("  hello_world, base_type, bit_ops, operator, compound_types, if_flow");
    println!("  chain_call, closure, while_flow, loop_flow, for_flow, match_flow");
    println!("  return_flow, ownership_and_refs, vec_type");
    println!("  fizzbuzz, score_analyzer, sandbox");
}
