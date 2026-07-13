//! 闭包(closures)：匿名函数，可以捕获所在作用域里的变量。
//!
//! 闭包语法: `|参数| 函数体` 或 `|参数| { 多条语句 }`
//! 和普通 fn 的区别: 闭包可以"看到"外面定义的变量(捕获环境).

/// 演示闭包的基本语法和常见用法(map/filter).
pub fn run() {
    // ===== 闭包基本语法 =====
    println!("===== 闭包基本语法 =====");

    // 最简单的闭包: |参数| 表达式
    let add_one = |x: i32| x + 1;  // 和 fn add_one(x:i32) -> i32 { x+1 } 几乎一样
    println!("|x: i32| x + 1,  输入 5 → {}", add_one(5));

    // 类型可以省略(编译器推断)
    let double = |x| x * 2;
    println!("|x| x * 2,      输入 5 → {}", double(5));

    // 多条语句用 {}
    let greet = |name| {
        let s = format!("你好, {}!", name);
        s  // 最后一行返回值
    };
    println!("greet(\"Rust\") = \"{}\"", greet("Rust"));

    // 无参数闭包: ||
    let say_hi = || "Hi!";
    println!("|| \"Hi!\" → {}", say_hi());

    // ===== 闭包与函数的区别: 捕获环境变量 =====
    println!("\n===== 闭包与函数的区别: 捕获环境变量 =====");
    let base = 10;
    // 普通函数看不到外面的 base, 闭包可以:
    let add_base = |x| x + base;  // 闭包"捕获"了 base
    println!("base = {}, |x| x + base,  输入 5 → {}", base, add_base(5));

    let prefix = "Result: ";
    let labeled = |v| format!("{}{}", prefix, v);  // 捕获了 prefix
    println!("labeled(42) = \"{}\"", labeled(42));

    // ===== 闭包用在迭代器里 =====
    println!("\n===== 闭包用在迭代器里 =====");
    let nums = vec![1, 2, 3, 4, 5];

    // map: 对每个元素调用闭包, 返回值组成新迭代器
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map(|x| x*2)     : {:?}", doubled);
    // map 需要一个闭包, 告诉它"怎么转换每个元素".

    // filter: 闭包返回 bool, true 保留, false 丢弃
    let evens: Vec<&i32> = nums.iter().filter(|x| **x % 2 == 0).collect();
    println!("filter(|x| x%2==0): {:?}", evens);
    // 这儿 **x: x 是 &&i32(iter 给的引用), 需要两层解引用才能拿到 i32.

    // filter + map 组合
    let result: Vec<i32> = nums.iter()
        .filter(|x| **x > 2)  // 先过滤: 只要 > 2
        .map(|x| x * 10)       // 再转换: 乘 10
        .collect();
    println!(">2 的 ×10      : {:?}", result);

    // 用在 sort 里: sort_by 需要一个比较闭包
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));  // 按长度排序
    println!("按长度排序: {:?}", words);

    // ===== 闭包的三种类型 =====
    println!("\n===== 闭包的三种类型 =====");
    println!("编译器根据闭包怎么用变量, 自动选一种:");
    println!("  FnOnce : 拿走捕获变量的所有权(只能调用一次)");
    println!("  FnMut  : 可变借用捕获的变量(可以修改)");
    println!("  Fn     : 不可变借用捕获的变量(只读)");
    println!("一般写代码不用管, map/filter 之类的自动适配.");
}
