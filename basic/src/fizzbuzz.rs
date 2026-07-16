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

    println!(
        "FizzBuzz: {}..={}  (Fizz={}, Buzz={})",
        start, end, fizz_num, buzz_num
    );
    println!("----------------------------------------");

    let mut results: Vec<String> = Vec::new();       // results 拥有 Vec 和里面的 String

    // ==== for + match 元组模式 ====
    // match 后面不只能放单个值, 还可以放元组!
    // (i % 3 == 0, i % 5 == 0) 会创建一个 (bool, bool) 元组,
    // 然后 match 的分支用元组模式来匹配, 一次判断两个条件:
    //   (true,  true)  → 同时是 3 和 5 的倍数
    //   (true,  false) → 只是 3 的倍数
    //   (false, true)  → 只是 5 的倍数
    //   (false, false) → 都不是
    // 这样写比嵌套 if/else 清晰得多: 四种情况一目了然, 编译器还会检查是否遗漏.
    for i in start..=end {                           // i: i32 Copy, 每次循环独立
        let text = match (i % fizz_num == 0, i % buzz_num == 0) {
            (true, true) => String::from("FizzBuzz"),// 新建 String — 所有权在 text
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (false, false) => i.to_string(),          // to_string() 新建 String
        };
        println!("  {}", text);
        results.push(text);                          // text 所有权移入 results! text 不再可用
    }

    // ── 闭包 filter: 只统计包含字母的结果 ──
    // 所有权: .iter() 借 results → filter 闭包借每个元素 → collect 产生新 Vec<&String>
    // 新 Vec 里的元素是 &String (只借不拥有), results 仍拥有所有 String。
    let special: Vec<&String> = results
        .iter()                                        // 借 results
        .filter(|s| s.as_bytes()[0].is_ascii_alphabetic()) // s: &&String, 自动解引用调用方法
        .collect();                                    // 收集为 Vec<&String>

    println!("----------------------------------------");
    println!(
        "总计 {} 个, 其中特殊 {} 个 (含 Fizz/Buzz), 数字 {} 个",
        results.len(),
        special.len(),
        results.len() - special.len(),
    );
}
