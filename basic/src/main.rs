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
mod bit_ops;
mod if_flow;
mod match_flow;
mod return_flow;
mod ownership_and_refs;
mod compound_types;
mod vec_type;

fn main() {
    println!("pub 是 public 的意思，表示这个函数是公开的，可以被其他模块调用和访问");
    println!("hello_world 是函数的名称,Rust 采用 snake_case 命名法，函数名小写，单词用下划线连接");
    println!("通常每个函数都有参数，这里没有参数，所以是 ()");
    println!("fn 是函数定义关键字,main 是程序入口（程序从这里开始执行）");
    println!("{{ }} 是块(block)创建一个新的作用域");
    println!("  块本身也是表达式——里面最后一行不加分号，块就会返回那个值");
    println!("  比如 let y = {{ let x = 1; x + 2 }};  // 块返回值 3,赋给 y");
    println!("  整个 main 函数的 {{ }} 就是最大的块，里面的内容依次执行");
    println!();

    // ===== println! 宏与格式化占位 =====
    println!("Hello, world!");
    println!("println! 是一个宏，用于打印并自动换行");
    println!("{} + {} = {}", 1, 2, 3);
    println!("  上面用的是 {{}} 位置占位符：按顺序填入");
    println!("{0} + {0} = {1}", 2, 4);
    println!("  上面用的是 {{0}} 索引占位符：按位置编号填入，可复用");
    println!("{name} 说：{msg}", name = "Rust", msg = "你好");
    println!("  上面用的是 {{name}} 命名占位符：按变量名填入");
    let nums = vec![1, 2, 3];
    println!("调试输出：{:?}", nums);
    println!("  上面用的是 {{:?}} 调试占位符：打印 Debug 格式");
    println!();

    // ===== let、表达式、语句、分号 =====
    println!("let: 变量绑定关键字, let 本身是语句，不产生值，以分号结尾");
    println!("let 右边的 = 和表达式才会产生值，绑定给左边的变量名");
    println!("fn hello_world() 是一个函数声明，也是语句，不产生值");
    println!("一句话，表达式 = 有值，语句 = 没值。分号把表达式变成语句");
    println!("通常函数最后一行不用分号，那么最后一行就会产生值，如果你想要它产生值的话");
    println!("后面碰到不同的表达式和语句的时候会说明");
    println!();
    println!("print! 和 println! 的区别：换行");
    print!("print! 输出后不换行，");
    print!("所以下一个 print! 会接在后面，");
    print!("三个 print! 全挤在同一行。");
    println!();
    println!("而 println! 输出后自动换行，所以我是新起的一行。");
    println!();
    println!("宏是 Rust 中一种特殊的编译时代码生成机制，具体在宏部分讲解。");
    println!();

    println!(":: 是路径分隔符，表示进入这个模块里面找");
    println!();
    println!("--- base_type:基础类型、溢出、进制、类型转换 ---");
    base_type::run();
    println!("--- bit_ops:位运算(与、或、异或、非、移位)---");
    bit_ops::run();
    println!("--- if_flow:条件判断(if/else/if 表达式)---");
    if_flow::run();
    println!("--- match_flow:模式匹配(match/守卫/范围)---");
    match_flow::run();
    println!("--- return_flow:函数返回值(隐式/显式/提前return)---");
    return_flow::run();
    println!("--- ownership_and_refs:所有权/引用/借用---");
    ownership_and_refs::run();
    println!("--- compound_types:元组/数组/字符串---");
    compound_types::run();
    println!("--- vec_type:向量(Vec)---");
    vec_type::run();
}
