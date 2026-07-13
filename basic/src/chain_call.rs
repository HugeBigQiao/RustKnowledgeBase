//! 链式调用：多个方法用 `.` 串联起来，每一步的返回值可以继续调用下一步。

/// 链式调用是 Rust 常见写法。核心：每个方法返回一个新东西，后面可以继续 `.`
pub fn run() {
    // ===== 字符串链式调用 =====
    println!("===== 字符串链式调用 =====");
    let s = "  hello Rust  ";
    // 三个方法连着写: trim → to_uppercase → replace
    let result = s.trim().to_uppercase().replace(" ", "-");
    println!("原始: \"{}\"", s);
    println!("trim → to_uppercase → replace:");
    println!("  trim()        : 去首尾空格 → \"{}\"", s.trim());
    println!("  to_uppercase(): 转大写     → \"{}\"", s.trim().to_uppercase());
    println!("  replace()     : 替换字符   → \"{}\"", result);
    println!("三步连写结果: \"{}\"", result);

    // 更多字符串链:
    let name = "rust";
    println!("{}", name.to_uppercase().replace("R", "L"));
    // → "LUST"

    // ===== 迭代器链式调用 =====
    println!("\n===== 迭代器链式调用 =====");
    let nums = vec![1, 2, 3, 4, 5];

    // sum 消耗迭代器, 返回求和结果
    println!("求和: {}", nums.iter().sum::<i32>());
    println!("计数: {}", nums.iter().count());

    // take 取前几个, 然后用 for 遍历
    let v = vec![10, 20, 30, 40, 50];
    print!("take(2) 取前 2 个: ");
    for x in v.iter().take(2) {
        print!("{} ", x);
    }
    println!();

    // skip 跳过前几个
    print!("skip(2) 跳过前 2 个: ");
    for x in v.iter().skip(2) {
        print!("{} ", x);
    }
    println!();

    // 组合: skip + take
    print!("skip(1).take(2) = ");
    for x in v.iter().skip(1).take(2) {
        print!("{} ", x);
    }
    println!();

    // ===== 链式调用的本质 =====
    println!("\n===== 链式调用的本质 =====");
    println!("每一步返回新值, 所以可以继续 . 下去.");
    println!("比如 \"  hello  \".trim().to_uppercase():");
    println!("  .trim()           → 返回 &str(去空格)");
    println!("  .to_uppercase()   → 返回 String(转大写)");
    println!();
    println!("比如 v.iter().take(2).skip(1):");
    println!("  .iter()           → 返回迭代器");
    println!("  .take(2)          → 返回新迭代器(只取前 2 个)");
    println!("  .skip(1)          → 返回新迭代器(跳过第 1 个)");
}
