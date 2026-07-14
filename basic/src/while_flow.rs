//! 循环: while
//!
//! while 在条件为 true 时反复执行一段代码.
//! 每次循环前检查条件, 条件为 false 就退出.

/// while 适合"不知道循环多少次, 只知道什么时候停"的场景.
/// 语法: `while 条件 { 循环体 }`
///
/// 和 if 的区别: if 最多执行一次, while 可能执行多次.
pub fn run() {
    // ===== while 基础: 条件循环 =====
    // 语法: while 条件 { 循环体 }
    // 每次循环开始前检查条件, 为 true 就执行循环体, 为 false 就退出.
    let mut count = 3;
    while count > 0 {
        println!("  count = {} (向下倒数)", count);
        count -= 1;  // 每次减 1, 直到 count = 0 时条件为 false, 退出
    }
    println!("while 循环结束! count = {} (条件为 false, 退出)", count);

    // ===== while 典型场景: 不知道循环次数 =====
    // 比如模拟翻倍直到超过 100:
    let mut n = 3;
    while n <= 100 {
        n *= 2;  // 3 → 6 → 12 → 24 → 48 → 96 → 192 → 退出
    }
    println!("从 3 开始翻倍, 直到超过 100: {} (3→6→12→...→192 退出)", n);

    // 比如累加直到超过目标值(不知道加到第几个数会超过):
    let mut sum = 0;
    let mut i = 0;
    while sum <= 50 {
        i += 1;
        sum += i;
    }
    println!("1+2+...+{} = {} (首次超过 50, 退出)", i, sum);

    // ===== continue: 跳过本次循环 =====
    // continue 让循环立即跳到下一次迭代, 不执行 continue 后面的代码.
    let mut i = 0;
    while i < 6 {
        i += 1;
        if i % 2 == 0 {
            continue;  // 偶数跳过, 不打印
        }
        println!("  奇数: {} (continue 跳过了偶数)", i);
    }
    println!("从 1 到 6, 只打印奇数(偶数被 continue 跳过)");
}
