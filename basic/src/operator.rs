//! Rust 运算符：算术、比较、逻辑、赋值。

/// 演示 Rust 的各种运算符与运算符重载。
pub fn run() {
    let x = 10;
    let y = 3;

    // ===== 算术运算符 =====
    println!("===== 算术运算符 =====");
    println!("x = {}, y = {}", x, y);
    println!("+ (加):  {} + {} = {}", x, y, x + y);
    println!("- (减):  {} - {} = {}", x, y, x - y);
    println!("* (乘):  {} * {} = {}", x, y, x * y);
    println!("/ (除):  {} / {} = {}", x, y, x / y);
    println!("  ⚠️ 整数除法会截断小数，不是四舍五入");
    println!("% (取余): {} % {} = {}", x, y, x % y);

    // 浮点除法不会截断
    println!();
    println!("浮点除法: 10.0 / 3.0 = {}", 10.0 / 3.0);

    // ===== 比较运算符（返回 bool）=====
    println!("\n===== 比较运算符（返回 bool）=====");
    println!("x == y : {}", x == y);
    println!("x != y : {}", x != y);
    println!("x < y  : {}", x < y);
    println!("x > y  : {}", x > y);
    println!("x <= y : {}", x <= y);
    println!("x >= y : {}", x >= y);

    // ===== 逻辑运算符 =====
    println!("\n===== 逻辑运算符 =====");
    let t = true;
    let f = false;
    println!("true && false = {}", t && f);
    println!("  && (逻辑与)：两边都为 true 才是 true，短路（左边 false 就不看右边）");
    println!("true || false = {}", t || f);
    println!("  || (逻辑或)：任意一边为 true 就是 true，短路（左边 true 就不看右边）");
    println!("!true = {}", !t);
    println!("  ! (逻辑非)：取反");

    // ===== 赋值运算符（复合赋值）=====
    println!("\n===== 赋值运算符（复合赋值）=====");
    let mut n = 10;
    // 这里使用了可变变量，后续的赋值运算符会修改变量的值。具体的可变变量会在后续讲解
    println!("初始 n = {}", n);
    n += 5;
    println!("n += 5  → n = {}", n);
    n -= 3;
    println!("n -= 3  → n = {}", n);
    n *= 2;
    println!("n *= 2  → n = {}", n);
    n /= 4;
    println!("n /= 4  → n = {}", n);
    n %= 5;
    println!("n %= 5  → n = {}", n);

}
