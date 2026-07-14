//! 循环: for
//!
//! for 是 Rust 最常用的循环, 用于遍历集合(数组、Vec、切片等).
//! 和 while 不同: for 是"遍历每一个元素", 而不是"检查条件".
//! for 比 while 安全: 不会写死循环, 也不会出现索引越界.

/// for 的语法: `for 变量 in 可迭代对象 { ... }`
/// for 更安全, 因为:
///   - 不需要手动维护索引变量
///   - 编译器保证不会越界
///   - 遍历完后自动结束
pub fn run() {
    // ===== for 遍历数组 =====
    // 每次循环从数组里拿一个元素, 绑定到变量 x.
    let arr = [10, 20, 30, 40, 50];
    print!("数组: ");
    for x in arr {  // x 是 i32 (复制出来的值)
        print!("{} ", x);
    }
    println!();
    // for x in arr 会消耗 arr? 不会! i32 是 Copy 类型.
    // 如果是非 Copy 类型(比如 String 数组), 遍历会消耗所有权.
    // 这时要用引用遍历: for x in &arr

    // ===== for 遍历 Vec =====
    let v = vec![1, 2, 3, 4];
    print!("Vec: ");
    for n in &v {  // &v: 借用来遍历, 不消耗 v
        print!("{} ", n);
    }
    println!();
    println!("  遍历后 v 还能用: {:?} (因为用了 & 引用遍历)", v);

    // 不推荐的写法(while 模拟 for):
    // let mut i = 0;
    // while i < v.len() {
    //     println!("{}", v[i]);  // 索引可能越界, 代码也啰嗦
    //     i += 1;
    // }
    // 所以: 遍历集合用 for, 不要用 while + 索引.

    // ===== 范围: a..b 和 a..=b =====
    // a..b   : 从 a 到 b(不含), 左闭右开.
    // a..=b  : 从 a 到 b(含), 闭区间.
    print!("0..5  : ");
    for i in 0..5 {
        print!("{} ", i);  // 0 1 2 3 4
    }
    println!();

    print!("0..=5 : ");
    for i in 0..=5 {
        print!("{} ", i);  // 0 1 2 3 4 5
    }
    println!();

    // 逆向范围: (1..=4).rev()
    print!("(1..=4).rev(): ");
    for i in (1..=4).rev() {
        print!("{} ", i);  // 4 3 2 1
    }
    println!();

    // ===== for 搭配 continue 和 break =====
    // 和 while/loop 一样, for 里也可以用 continue 和 break.
    for i in 1..=10 {
        if i % 3 == 0 {
            continue;  // 3 的倍数跳过
        }
        if i > 8 {
            break;  // 超过 8 就停止
        }
        print!("{} ", i);  // 1 2 4 5 7 8
    }
    println!("(跳过 3 的倍数, >8 停止)");

    // ===== for 遍历字符串: chars() 取字符 =====
    let word = "Rust";
    print!("\"Rust\" 的每个字符: ");
    for c in word.chars() {
        print!("'{}' ", c);
    }
    println!();
    // 注意: for c in word 不行! &str 没有直接实现 IntoIterator.
    // 需要 .chars() 来取字符, 或者 .bytes() 取字节.

    // ===== 同时拿到索引和值: 手动计数 =====
    // 最简单的办法: 用 for 遍历范围, 通过索引取元素.
    let colors = vec!["红", "绿", "蓝"];
    for i in 0..colors.len() {
        println!("  colors[{}] = {}", i, colors[i]);
    }

    // ===== 迭代器(Iterator) =====
    // 迭代器是一个"数据流", 你可以一个一个地从里面拿值.
    // Rust 中所有实现了 Iterator trait 的类型都是迭代器.
    // 迭代器只有一个核心方法: .next(), 每调一次返回一个值(包在 Some 里),
    // 拿完了就返回 None.
    //
    // for 和迭代器的关系:
    //   for 是"语法糖", 它内部就是反复调用 .next() 直到返回 None.
    //   for item in xxx 等价于:
    //     { let mut it = xxx.into_iter(); loop { match it.next() { Some(v) => ..., None => break } } }
    //   所以 for 负责"循环控制", 迭代器负责"产出一串值".
    //   可以理解为: for 是发动机, 迭代器是燃料管.
    println!();
    let v = vec![1, 2, 3];

    // for 自动调用 into_iter(), 下面三种写法等价:
    //   for n in &v       -> 拿到 &i32, 不消耗 v
    //   for n in v.iter() -> 同上, 显式写法
    //   for n in v        -> 拿到 i32, 消耗 v (所有权转移)
    print!("iter() 遍历: ");
    for n in v.iter() {
        print!("{} ", n);
    }
    println!();

    // 手动模拟 for 的原理: 创建迭代器, 自己调 .next()
    let mut it = v.iter();
    println!("手动 next(): {:?}, {:?}, {:?}, {:?}",
        it.next(), it.next(), it.next(), it.next());
    // 前三个返回 Some(&1), Some(&2), Some(&3), 第四个返回 None(迭代器耗尽)
}
