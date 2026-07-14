//! 闭包(closures)：匿名函数，可以捕获所在作用域里的变量。
//!
//! 闭包语法: `|参数| 函数体` 或 `|参数| { 多条语句 }`
//! 和普通 fn 的区别: 闭包可以"看到"外面定义的变量(捕获环境).

/// 演示闭包的基本语法和常见用法(map/filter).
pub fn run() {
    // ===== 闭包基本语法 =====
    println!("===== 闭包基本语法 =====");

    // 最简单的闭包: |参数| 表达式
    let add_one = |x: i32| x + 1; // 和 fn add_one(x:i32) -> i32 { x+1 } 几乎一样
    println!("|x: i32| x + 1,  输入 5 → {}", add_one(5));

    // 类型可以省略(编译器推断)
    let double = |x| x * 2;
    println!("|x| x * 2,      输入 5 → {}", double(5));

    // 多条语句用 {}
    let greet = |name| {
        let s = format!("你好, {}!", name);
        s // 最后一行返回值
    };
    println!("greet(\"Rust\") = \"{}\"", greet("Rust"));

    // 无参数闭包: ||
    let say_hi = || "Hi!";
    println!("|| \"Hi!\" → {}", say_hi());

    // ===== 闭包与函数的区别: 捕获环境变量 =====
    println!("\n===== 闭包与函数的区别: 捕获环境变量 =====");
    let base = 10;
    // 普通函数看不到外面的 base, 闭包可以:
    let add_base = |x| x + base; // 闭包"捕获"了 base
    println!("base = {}, |x| x + base,  输入 5 → {}", base, add_base(5));

    let prefix = "Result: ";
    let labeled = |v| format!("{}{}", prefix, v); // 捕获了 prefix
    println!("labeled(42) = \"{}\"", labeled(42));

    // ===== 闭包用在迭代器里 =====
    println!("\n===== 闭包用在迭代器里 =====");
    let nums = vec![1, 2, 3, 4, 5];

    // ===== map: 对每个元素做"映射/转换" =====
    // map 的意思是把集合里的每个元素按照你给的规则"变成"另一个值,
    // 就像流水线: 每个零件进去, 经过一道工序, 出来时已经变了样.
    // 元素数量不变, 只是每个元素被转换了.
    //
    // nums = [1, 2, 3, 4, 5]
    //   map(|x| x * 2) ↓
    //          [2, 4, 6, 8, 10]
    //
    // .collect() 把迭代器"收集"回 Vec(或其他集合), 因为 map 返回的还是迭代器.
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map(|x| x*2)     : {:?}", doubled);

    // ===== filter: 按条件"筛选/过滤"元素 =====
    // filter 等于一个筛子: 闭包返回 true 就留下, 返回 false 就丢掉.
    // 元素数量会变少(甚至为 0).
    //
    // nums = [1, 2, 3, 4, 5]
    //   filter(|x| x%2==0) ↓     (只要偶数)
    //              [2, 4]
    //
    // ===== 关于 `**x` 和解引用 =====
    // 这里出现了 `&&i32` 和 `**x`, 很多人第一次见到会懵, 拆开看:
    //
    //   nums.iter()         → 迭代器里每个元素是 &i32  (指向 nums 里数字的引用)
    //   .filter(|x| ...)    → 闭包参数 x 的类型是 &&i32  (迭代器"借给"闭包时又套了一层引用)
    //
    // 所以 x 的类型是 &&i32:
    //   x   → &&i32  (指向"指向 i32 的引用"的引用)
    //   *x  → &i32   (解一层, 拿到指向 i32 的引用)
    //   **x → i32    (解两层, 拿到真正的数字)
    //
    // 为什么需要 **x? 因为 `%` 取模运算符要求两边都是整数, &&i32 不能直接 %
    // Rust 不会自动解多层引用, 需要手动 `**x` 才能拿到 i32 做运算.
    //
    // 如果不想写 **, 也可以用 .iter().copied() 把 &i32 变成 i32:
    //   nums.iter().copied().filter(|x| x % 2 == 0)  ← x 是 i32, 不用 **
    let evens: Vec<&i32> = nums.iter().filter(|x| **x % 2 == 0).collect();
    println!("filter 偶数      : {:?}", evens);

    // ===== map + filter 组合(链式调用) =====
    // 迭代器可以一直 .操作, 像流水线一道工序接一道.
    // 注意执行顺序: 先 filter 后 map, 按 . 的先后顺序执行.
    let result: Vec<i32> = nums
        .iter()
        .filter(|x| **x > 2) // 第 1 步: 筛掉 ≤2 的 → [3, 4, 5]
        .map(|x| x * 10) // 第 2 步: 每个 ×10    → [30, 40, 50]
        .collect(); // 第 3 步: 收集回 Vec
    println!(">2 的 ×10      : {:?}", result);

    // ===== 排序: sort / sort_by / sort_by_key =====
    //
    // Rust 的 Vec 提供了三种排序方法:
    //
    //   sort()         -> 按元素自身的"自然顺序"排序(要求元素实现 Ord)
    //   sort_by()      -> 自定义比较规则, 闭包返回 Ordering
    //   sort_by_key()  -> 对每个元素算出一个"键值", 按键值排序(最常用!)
    //   另外还有 sort_unstable(), 算法不同但用法一样, 通常更快.
    //
    // 排序规则: 闭包比较 a 和 b, 返回一个 Ordering 枚举值:
    //   Ordering::Less    -> a < b, a 排在前面
    //   Ordering::Equal   -> a == b, 位置不变
    //   Ordering::Greater -> a > b, b 排在前面(即 a 和 b 交换)
    //
    // 怎么得到 Ordering? 用 .cmp() 方法.
    //   .cmp() 是 Rust 内置的比较方法, 几乎所有能比大小的类型(i32、usize、char、
    //   String 等)都有它. 调用 a.cmp(&b) 返回:
    //     a < b  → Ordering::Less
    //     a == b → Ordering::Equal
    //     a > b  → Ordering::Greater
    //   Ordering 是一个枚举(enum), 只有这三种可能. 枚举会在 intermediate 里细讲,
    //   这里先知道它是"三选一"的标签即可.

    let mut words = vec!["banana", "apple", "cherry"];

    // --- sort(): 默认排序(字符串按字典序) ---
    words.sort();
    println!("sort() 字典序  : {:?}", words); // ["apple", "banana", "cherry"]

    // --- sort_by(): 自定义规则 ---
    // |a, b| a.len().cmp(&b.len()) 拆解:
    //   1. a.len()  -> 取出 a 的字节长度(usize)
    //   2. b.len()  -> 取出 b 的字节长度(usize)
    //   3. .cmp(&b.len()) -> a.len().cmp(&b.len()), 比较大小, 返回 Ordering
    //   4. sort_by 根据返回值决定 a 和 b 的先后顺序
    // 结果: 按字符串长度升序, 长度相同时保持原顺序(稳定排序).
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("sort_by 按长度 : {:?}", words); // ["apple", "banana", "cherry"]

    // --- sort_by_key(): 最简洁的写法 ---
    // "键(key)" 就是排序的依据——从每个元素里提取出一个能比较的值.
    // 比如 "按长度排序", 键就是每个字符串的长度.
    //
    // sort_by_key 内部替你做了两件事:
    //   1. 对每个元素调用闭包, 算出它的键(|s| s.len())
    //   2. 按键的自然顺序(.cmp())排序
    //
    // 所以 sort_by_key(|s| s.len()) 等价于 sort_by(|a, b| a.len().cmp(&b.len())),
    // 但省去了手写 .cmp() 的麻烦, 更短更不容易出错.
    let mut words2 = vec!["banana", "apple", "cherry"];
    words2.sort_by_key(|s| s.len());
    println!("sort_by_key 键  : {:?}", words2);

    // ===== 闭包的三种类型 =====
    println!("\n===== 闭包的三种类型 =====");
    println!("编译器根据闭包怎么用变量, 自动选一种:");
    println!("  FnOnce : 拿走捕获变量的所有权(只能调用一次)");
    println!("  FnMut  : 可变借用捕获的变量(可以修改)");
    println!("  Fn     : 不可变借用捕获的变量(只读)");
    println!("一般写代码不用管, map/filter 之类的自动适配.");
}
