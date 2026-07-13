//! 错误处理专题: Result、panic!、unwrap vs expect、? 运算符。
//!
//! Result 本质是一个枚举:
//!   enum Result<T, E> { Ok(T), Err(E) }
//!
//! 前置依赖: basic/ 中的 match; intermediate/ 中的 structs_and_enums、option.

use std::fs;

// ── 辅助函数 ──

/// 演示函数: 使用 panic!(会崩溃, 不要实际调用)
#[allow(dead_code)]
fn panic_example() -> i32 {
    panic!("这是一个 panic 演示");
    // panic! 之后的代码不会执行.
}

/// 没有 ? 的写法: 每一步都要手动 match.
fn read_and_double_no_q(path: &str) -> Result<i32, String> {
    // Step 1: 读取文件
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return Err(format!("读取文件失败: {}", e)),
    };

    // Step 2: 去除空白后解析为数字
    let trimmed = content.trim();
    let num: i32 = match trimmed.parse() {
        Ok(n) => n,
        Err(e) => return Err(format!("解析数字失败: {}", e)),
    };

    // Step 3: 返回加倍结果
    Ok(num * 2)
}

/// 有 ? 的写法: 错误自动向上传播.
fn read_and_double(path: &str) -> Result<i32, String> {
    // ? 自动处理错误: Ok(v) 取出 v, Err(e) 则 return Err(e.into()).
    let content = fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let trimmed = content.trim();
    let num: i32 = trimmed.parse().map_err(|e| format!("解析数字失败: {}", e))?;
    Ok(num * 2)
}

/// 解析字符串并翻倍
fn parse_and_double(s: &str) -> Result<i32, String> {
    // parse() 返回 Result, ? 直接传播错误.
    let num: i32 = s.parse().map_err(|e| format!("解析 \"{}\" 失败: {}", s, e))?;
    Ok(num * 2)
}

/// 读取文件内容
fn read_file_content(path: &str) -> Result<String, String> {
    // 简单封装, 错误转换.
    fs::read_to_string(path).map_err(|e| format!("无法读取文件: {}", e))
}

// ── run ──

/// 演示 Result 的创建与 match 处理、panic!/unwrap/expect 的区别、? 运算符。
pub fn run() {
    // ===== Result 创建 =====
    println!("===== Result 创建 =====");
    let ok_val: Result<i32, &str> = Ok(42);
    let err_val: Result<i32, &str> = Err("出错了");
    println!("ok_val  = {:?}", ok_val);
    println!("err_val = {:?}", err_val);

    // 类型注解: 需要同时指定 T(成功类型) 和 E(错误类型).

    // ===== match 处理 Result =====
    println!("\n===== match 处理 Result =====");
    // 和 Option 一样, match 的穷尽性保证你不会忘了处理 Err.

    match ok_val {
        Ok(v) => println!("成功, 值为 {}", v),
        Err(e) => println!("失败, 原因: {}", e),
    }

    match err_val {
        Ok(v) => println!("成功, 值为 {}", v),
        Err(e) => println!("失败, 原因: {}", e),
    }

    // ===== panic! 宏 =====
    println!("\n===== panic! =====");
    // panic!: 主动让程序崩溃, 打印信息并展开调用栈.
    // 用于"不可恢复的错误"场景: 数组越界、除零、断言失败等.
    println!("panic! 是 Rust 的'程序崩溃'机制.");
    println!("触发方式: panic!(\"消息\"), 数组越界, unwrap() 遇到 None/Err 等.");
    // println!("{}", panic_example()); // 取消注释会崩溃

    // ===== unwrap vs expect =====
    println!("\n===== unwrap vs expect =====");
    // unwrap: 取出 Ok 的值, 如果是 Err 就 panic.
    // expect: 同 unwrap, 但可以自定义 panic 消息(出问题时更容易定位).

    let ok: Result<i32, &str> = Ok(100);
    let err: Result<i32, &str> = Err("数据损坏");

    // unwrap: 成功时返回值, 失败时 panic("called `Result::unwrap()` on an `Err` value: ...")
    println!("ok.unwrap()  = {}", ok.unwrap());
    // println!("{}", err.unwrap()); // 取消注释会 panic!

    // expect: 成功时返回值, 失败时 panic(自定义消息)
    println!("ok.expect(\"不应该失败\") = {}", ok.expect("不应该失败"));
    // println!("{}", err.expect("读取配置失败")); // panic!("读取配置失败: ...")

    // 区别总结:
    println!("\n--- unwrap vs expect 总结 ---");
    println!("unwrap:  简单粗暴, 出错信息不够友好, 适合原型阶段.");
    println!("expect:  带自定义消息, 出问题时一眼看出哪里崩了, 推荐使用.");
    println!("原则:    能用 expect 就别用 unwrap, 除非绝对不会失败.");

    // ===== unwrap_or / unwrap_or_else =====
    println!("\n===== unwrap_or / unwrap_or_else =====");
    // 不想 panic 时, 可以给默认值(和 Option 一样).

    println!("ok.unwrap_or(0)  = {}", ok.unwrap_or(0));
    println!("err.unwrap_or(0) = {}", err.unwrap_or(0));
    println!("err.unwrap_or_else(|e| {{ println!(\"错误: {{}}\", e); 0 }}) = {}",
        err.unwrap_or_else(|e| { println!("  错误: {}", e); 0 }));

    // ===== ? 运算符 =====
    println!("\n===== ? 运算符 =====");
    // ?: 如果是 Ok(v) → 取出 v 继续; 如果是 Err(e) → 提前返回 Err(e) 给调用者.
    // 这是 Rust 错误处理最常用的语法糖, 让错误传播变得简洁.

    // 没有 ? 的写法(啰嗦):
    println!("--- 没有 ? 运算符 ---");
    match read_and_double_no_q("data.txt") {
        Ok(v) => println!("结果: {}", v),
        Err(e) => println!("错误: {}", e),
    }

    // 有 ? 的写法(简洁):
    println!("\n--- 有 ? 运算符 ---");
    match read_and_double("data.txt") {
        Ok(v) => println!("结果: {}", v),
        Err(e) => println!("错误: {}", e),
    }

    // ===== 实践: parse 与 ? 结合 =====
    println!("\n===== 实践: 字符串 → 数字 =====");
    let inputs = vec!["42", "0", "abc"];
    for s in &inputs {
        match parse_and_double(s) {
            Ok(v) => println!("\"{}\" → {}", s, v),
            Err(e) => println!("\"{}\" → 错误: {}", s, e),
        }
    }

    // ===== 实践: 读取文件 =====
    println!("\n===== 实践: 读取文件 =====");
    match read_file_content("data.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取失败: {}", e),
    }

    match read_file_content("不存在的文件.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取失败: {}", e),
    }
}
