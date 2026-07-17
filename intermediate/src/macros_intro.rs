//! 宏基础: Rust 宏是什么、有哪些类型、用来干什么。
//!
//! 宏 = 编译期代码生成。在编译阶段展开为 Rust 代码, 然后再编译。
//! 函数做不到的事情 (变长参数、编译期计算、代码模板), 宏可以。

// ===== 1. 宏 vs 函数 =====

fn demo_macro_vs_function() {
    println!("--- 宏 vs 函数 ---");

    // 函数: 固定参数个数和类型, 运行期调用
    println!("  函数: fn add(a: i32, b: i32) -> i32  ← 参数数量和类型写死");

    // 宏: 变长参数, 编译期展开
    println!("  宏:   println!(\"{{}} + {{}} = {{}}\", 1, 2, 3) ← 参数可以 1 个到 N 个");

    // 核心区别:
    // 1. 宏展开发生在编译期, 函数调用发生在运行期
    // 2. 宏可以接收可变数量的参数 (println! / vec!)
    // 3. 宏可以生成代码 (自定义 derive, 如 #[derive(Debug)])
    // 4. 宏在调用点展开, 不涉及函数调用的栈帧开销 (但可能增大二进制体积)
}

// ===== 2. 宏的两大类型 =====

fn demo_macro_types() {
    println!("\n--- 宏的两大类型 ---");

    // A. 声明宏 (Declarative Macro) — macro_rules!
    println!("  A. 声明宏 (macro_rules!):");
    println!("     写法: macro_rules! 名称 {{ (模式) => {{ 展开代码 }}; }}");
    println!("     例子: vec!, println!, format!, assert_eq!");
    println!("     特点: 模式匹配驱动, 用 $ 捕获输入, 用 $(...)* 重复");

    println!();

    // B. 过程宏 (Procedural Macro) — proc-macro crate
    println!("  B. 过程宏 (proc-macro):");
    println!("     需要单独的 proc-macro 类型 crate");
    println!("     三种形式:");
    println!("       - #[derive(Xxx)]:    派生宏 (自动实现 trait, 如 Debug/Clone)");
    println!("       - #[xxx]:            属性宏 (如 #[tokio::main], #[serde(rename)])");
    println!("       - xxx!(...):         函数式宏 (如 sqlx::query!)");
    println!("     特点: 操作 TokenStream, 可以任意操作 AST, 能力最强");
}

// ===== 3. 常用内置宏 =====

fn demo_builtin_macros() {
    println!("\n--- 常用内置宏 ---");

    // println! / format! — 格式化输出
    println!("  println!(\"x = {{}}\", x)  格式化打印到 stdout");
    println!("  format!(\"x = {{}}\", x)   返回 String, 不打印");
    println!("  eprintln!(\"err!\")        打印到 stderr");

    // vec! — 创建 Vec
    let v = vec![1, 2, 3]; // 等价于: let mut v = Vec::new(); v.push(1); v.push(2); v.push(3);
    println!("  vec![1,2,3] = {:?}", v);

    // assert! 系列 — 测试断言
    println!("  assert!(x > 0)            断言为真, 否则 panic");
    println!("  assert_eq!(a, b)         断言相等");
    println!("  debug_assert!(...)        debug 模式有效, release 模式不编译");

    // todo! / unimplemented! — 占位
    println!("  todo!()                  标记未完成代码, 编译通过但运行 panic");
    println!("  unimplemented!()         同上, 语义是 '尚未实现'");

    // dbg! — 调试打印
    let x = 42;
    println!("  dbg!(x) = {:?}", dbg!(x)); // 打印 文件:行号 + 表达式 + 值

    // 编译期工具
    println!("  cfg!(target_os = \"linux\")  条件编译判断, 返回 bool");
    println!("  env!(\"CARGO_PKG_NAME\")     读取编译期环境变量");
    println!("  include_str!(\"file.txt\")  把文件内容编译进二进制 (静态嵌入)");
    println!("  file!() / line!() / column!()  当前文件名/行号/列号");
}

// ===== 4. 写一个简单的声明宏 =====

/// 自定义一个 say_hello! 宏, 无参数。
macro_rules! say_hello {
    () => {
        println!("你好, 这是我定义的第一个宏!");
    };
}

/// 自定义 min! 宏, 支持 2 个参数。
/// expr: 捕获表达式; $a/$b: 给捕获的表达式命名, 在展开体中使用。
macro_rules! my_min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
}

fn demo_custom_macro() {
    println!("\n--- 自定义宏示例 ---");

    // 调用无参宏 — 注意: 调用宏用 !, 括号可换成 [] 或 {}
    say_hello!();

    let min = my_min!(10, 20);
    println!("  my_min!(10, 20) = {}", min);

    let min2 = my_min!("hello", "world");
    println!("  my_min!(\"hello\", \"world\") = {}", min2);
}

// ===== 5. 宏不能做什么 =====

fn demo_macro_limits() {
    println!("\n--- 宏的局限性 ---");

    println!("  1. 宏展开后代码膨胀, 增大二进制体积 (特别是大量调用时)");
    println!("  2. 声明宏不支持类型检查 — 调用时才发现类型错误, 错误信息指向展开后的代码, 难读");
    println!("  3. 宏不能用 IDE 跳转定义 (展开后的代码不在源码中)");
    println!("  4. 声明宏不能操作作用域内的变量 (不是闭包)");
    println!("  5. 过程宏需要单独 crate, 增加编译时间");
    println!();
    println!("  经验法则: 能用函数解决的不要用宏。");
    println!("  用宏的场景: 消除样板代码、变长参数、编译期计算、derive 宏。");
}

pub fn run() {
    demo_macro_vs_function();
    demo_macro_types();
    demo_builtin_macros();
    demo_custom_macro();
    demo_macro_limits();
}
