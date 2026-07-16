//! 链式调用：多个方法用 `.` 串联起来，每一步的返回值可以继续调用下一步。
//!
//! 链式调用的所有权规则: 每一步返回什么, 下一步就拿什么继续。
//!   返回 &str → 下一步是借用 → 原材料仍然有效
//!   返回 String → 下一步是 owned → 原材料不受影响 (新 String 独立)
//!   返回迭代器 → 下一步消费迭代器 → 原材料不受影响

/// 链式调用是 Rust 常见写法。核心：每个方法返回一个新东西，后面可以继续 `.`
pub fn run() {
    // ===== 字符串链式调用 =====
    println!("===== 字符串链式调用 =====");
    let s = "  hello Rust  ";                       // s: &str — 字符串字面量, 不可变引用
    // 三个方法连着写: trim → to_uppercase → replace — s 全程不被消耗
    let result = s.trim().to_uppercase().replace(" ", "-");
    println!("原始: \"{}\"", s);                      // s 仍可用: 链上的方法都返回新值, 不修改 s
    println!("trim → to_uppercase → replace:");
    println!("  trim()        : 去首尾空格 → \"{}\"", s.trim());          // trim() 返回 &str — 借用 s 的视图
    println!("  to_uppercase(): 转大写     → \"{}\"", s.trim().to_uppercase()); // to_uppercase() 返回 String — 新创建
    println!("  replace()     : 替换字符   → \"{}\"", result);              // replace() 返回 String — 新创建
    println!("三步连写结果: \"{}\"", result);

    // 更多字符串链:
    let name = "rust";
    println!("{}", name.to_uppercase().replace("R", "L"));
    // → "LUST"

    // ===== 迭代器链式调用 =====
    println!("\n===== 迭代器链式调用 =====");
    let nums = vec![1, 2, 3, 4, 5];                 // nums 拥有 Vec<i32> 的所有权

    // sum 消耗迭代器, 返回求和结果。底层迭代器被消费, 但 nums 不受影响 (只是借了 nums)。
    println!("求和: {}", nums.iter().sum::<i32>());   // .iter() 借 nums → sum 消费迭代器 → nums 仍在
    println!("计数: {}", nums.iter().count());

    // take 取前几个, 然后用 for 遍历
    let v = vec![10, 20, 30, 40, 50];
    print!("take(2) 取前 2 个: ");
    for x in v.iter().take(2) {                      // .iter() 借 v → .take(2) 适配 → for 再借迭代器
        print!("{} ", x);                            // x: &i32 — 借 v 内部元素的引用
    }
    println!();

    // skip 跳过前几个 — 同样是惰性适配, 不消耗 v
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
    println!("  .trim()           → 返回 &str (借用原串, 不消耗)");
    println!("  .to_uppercase()   → 返回 String (新建, 不消耗原串)");
    println!();
    println!("比如 v.iter().take(2).skip(1):");
    println!("  .iter()           → 返回迭代器 (借用 Vec, 不消耗)");
    println!("  .take(2)          → 返回新迭代器 (惰性适配, 不消耗)");
    println!("  .skip(1)          → 返回新迭代器 (惰性适配, 不消耗)");
    println!("所有权要点: 链上每个方法返回什么, 决定下一步的类型是借的还是 owned。");
}
