//! Option 专题: Rust 的"可能没有值"类型。
//!
//! Option 本质是一个枚举:
//!   enum Option<T> { Some(T), None }
//!
//! 前置依赖: basic/ 中的 match、Vec; intermediate/ 中的 structs_and_enums.

// ── 辅助函数 ──

/// 安全除法: 除数为 0 返回 None, 否则返回 Some(商).
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None  // 返回 None, 调用方自己决定怎么处理.
    } else {
        Some(a / b)
    }
}

// ── run ──

/// 演示 Option 的创建、match 处理、if let/while let、常用方法。
pub fn run() {
    // ===== Option 创建 =====
    println!("===== 创建 Option =====");
    let some_val = Some(42);           // Some 包装一个值
    let none_val: Option<i32> = None;  // None 表示"没有"
    // 类型注解必要: 编译器需要知道 None 对应哪种 Option<T>.

    println!("some_val = {:?}", some_val);
    println!("none_val = {:?}", none_val);

    // 从 Vec 获取: .get() 返回 Option
    let v = vec![10, 20, 30];
    let first = v.get(0);  // Some(&10)
    let out = v.get(99);   // None(索引越界)
    println!("v.get(0)  = {:?}", first);
    println!("v.get(99) = {:?}", out);

    // ===== match 处理 Option =====
    println!("\n===== match 处理 =====");
    // match 的穷尽性保证你不会忘了处理 None.

    match first {
        Some(val) => println!("第一个元素: {}", val),
        None => println!("没有元素"),
    }

    match out {
        Some(val) => println!("第 100 个元素: {}", val),
        None => println!("索引越界!"),
    }

    // ===== if let =====
    println!("\n===== if let =====");
    // 只关心一个变体时, if let 比 match 简洁.

    // match 写法(啰嗦):
    match first {
        Some(val) => println!("match: 有值 {}", val),
        _ => {}
    }

    // if let 写法(简洁):
    if let Some(val) = first {
        println!("if let: 有值 {}", val);
    }

    // if let + else
    if let Some(val) = out {
        println!("有值: {}", val);
    } else {
        println!("没有值, 走了 else 分支");
    }

    // ===== while let =====
    println!("\n===== while let =====");
    // while let: 只要还能匹配就继续循环. 常用于"弹出直到空".

    let mut stack = vec![1, 2, 3];
    print!("出栈顺序: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("(栈已空)");

    // ===== 常用方法 =====
    println!("\n===== 常用方法 =====");

    let s = Some(100);
    let n: Option<i32> = None;

    // is_some / is_none: 判断有没有值
    println!("s.is_some()    = {}", s.is_some());
    println!("n.is_none()    = {}", n.is_none());

    // unwrap: 取出值, 如果是 None 则 panic(程序崩溃)
    println!("s.unwrap()     = {}", s.unwrap());
    // println!("{}", n.unwrap());  // 会 panic! 谨慎使用.

    // expect: 同 unwrap, 但可以自定义 panic 消息
    println!("s.expect(\"err\") = {}", s.expect("不应该失败"));

    // unwrap_or: 失败时返回默认值(不会 panic)
    println!("s.unwrap_or(0) = {}", s.unwrap_or(0));
    println!("n.unwrap_or(0) = {}", n.unwrap_or(0));

    // unwrap_or_else: 失败时调用一个闭包产生默认值(惰性求值)
    println!("n.unwrap_or_else(|| 1+2) = {}", n.unwrap_or_else(|| 1 + 2));

    // map: 如果有值, 对它做转换; 如果是 None, 保持 None.
    // 有点像"管道": Some(值) → 转换 → Some(新值)
    let doubled = s.map(|x| x * 2);
    let still_none = n.map(|x| x * 2);
    println!("s.map(|x| x*2)   = {:?}", doubled);
    println!("n.map(|x| x*2)   = {:?}", still_none);

    // and_then: 类似 map, 但闭包返回的是 Option(可能产生 None).
    // 用于"链式操作, 中间可能失败".
    let result = v.get(0)           // Some(&10)
        .map(|x| x * 3)              // Some(30)
        .and_then(|x| if x > 20 { Some(x) } else { None });
    println!("链式: {:?}", result);

    // ===== 实践: 安全除法 =====
    println!("\n===== 实践: 安全除法 =====");
    println!("10 / 2 = {:?}", safe_div(10, 2));
    println!("10 / 0 = {:?}", safe_div(10, 0));
    println!("10 / 3 = {:?}", safe_div(10, 3).unwrap_or(-1));
}
