//! FizzBuzz 综合练习
//!
//! 用到的知识点:
//!   for + 范围(..=), match 元组模式, Vec, 闭包(filter),
//!   函数, 所有权(引用), if 表达式, 基本类型

/// 生成 1..=30 的 FizzBuzz 序列, 练习 for 循环、match 和闭包.
pub fn run() {
    let start = 1;
    let end = 30;
    let fizz_num = 3;
    let buzz_num = 5;

    println!("FizzBuzz: {}..={}  (Fizz={}, Buzz={})", start, end, fizz_num, buzz_num);
    println!("----------------------------------------");

    let mut results: Vec<String> = Vec::new();

    // ── for 循环 + match 元组模式 ──
    for i in start..=end {
        let text = match (i % fizz_num == 0, i % buzz_num == 0) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (false, false) => i.to_string(),
        };
        println!("  {}", text);
        results.push(text);
    }

    // ── 闭包 filter: 只统计包含字母的结果 ──
    let special: Vec<&String> = results.iter()
        .filter(|s| s.as_bytes()[0].is_ascii_alphabetic())
        .collect();
    // s.as_bytes()[0] 取第一个字节, is_ascii_alphabetic() 判断是不是字母.
    // "FizzBuzz"/"Fizz"/"Buzz" 首字母都是大写字母.

    println!("----------------------------------------");
    println!("总计 {} 个, 其中特殊 {} 个 (含 Fizz/Buzz), 数字 {} 个",
        results.len(),
        special.len(),
        results.len() - special.len(),
    );
}
