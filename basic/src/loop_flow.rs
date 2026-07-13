//! 循环: loop
//!
//! loop 是无条件循环, 一直执行直到遇到 break.
//! 和 while 不同: loop 不检查条件, 需要手动 break 退出.

/// loop 的核心特点:
///   1. 无条件循环, 不检查条件.
///   2. 退出全靠 break.
///   3. loop 本身是表达式, break 可以带返回值.
///
/// loop 适合"每次循环的退出条件各不相同, 不想统一写在开头"的场景.
pub fn run() {
    // ===== loop 基础: 无限循环 + break =====
    // loop 不检查条件, 一直执行. 退出全靠 break.
    let mut x = 1;
    loop {
        println!("  loop 中: x = {}", x);
        x += 1;
        if x > 3 {
            break;  // break 退出当前循环
        }
    }
    println!("loop 遇到 break, 退出! x = {}", x);

    // ===== break 可以返回值 =====
    // loop 本身是表达式! break 后面可以带一个值, 这个值就是 loop 的返回值.
    // 这是 loop 独有的能力——while 和 for 都不能通过 break 返回值.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;  // break 10, loop 返回 10
        }
    };
    println!("loop 的返回值: {} (break 带了值, loop 是表达式)", result);

    // ===== loop 典型场景: 做重试逻辑 =====
    // 比如反复让用户输入, 直到输入合法才退出.
    // (这里用简单示例模拟: 猜一个"随机"数)
    let secret = 7;
    let mut attempts = 0;
    let answer = loop {
        attempts += 1;
        // 模拟每次"猜"的数字: 依次猜 3, 7
        let guess = if attempts == 1 { 3 } else { 7 };
        println!("  第 {} 次猜: {} (正确答案是 {})", attempts, guess, secret);
        if guess == secret {
            break attempts;  // 猜对了, loop 返回猜的次数
        }
    };
    println!("猜对了! 共猜了 {} 次 (loop 做重试, break 返回结果)", answer);
}
