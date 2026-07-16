//! Option 专题: Rust 的"可能没有值"类型。
//!
//! Option 本质是一个枚举:
//!   enum Option<T> { Some(T), None }
//!
//! 其中 T 是泛型参数——"占位符"。Option<i32> 表示 T = i32, 即"可能有 i32"。
//! Option<String> 表示 T = String, 即"可能有 String"。T 可以是任何类型。
//!
//! ## Some 和 None 是什么
//!
//! Some 和 None 是 Option 的两个变体(variant), 不是独立的类型:
//!
//!   Some(42)     → 类型是 Option<i32>, 不是"Some 类型"
//!   Some("hi")   → 类型是 Option<&str>
//!   None         → 本身没有类型, 需要上下文推断是 Option<什么> 的 None
//!
//! Some 不能单独用——它只是构造 Option 值的方式。Some(值) 的意思是:
//! "我这里有值, 值是什么你自己看"。对应其他语言里"有返回值"的正常情况。
//! None 的意思是"没有值", 替代了 null/nil/undefined。
//!
//! ## 有时候为什么能直接写 Some/None, 不需要 Option:: 前缀?
//!
//! Rust 有一个叫"prelude"(预导入)的机制: 编译器默认在每个文件顶部自动加了
//! `use std::prelude::v1::*;`。这个 prelude 里包含了:
//!   Option::Some  →  可以直接写 Some
//!   Option::None  →  可以直接写 None
//!   Result::Ok    →  可以直接写 Ok
//!   Result::Err   →  可以直接写 Err
//!
//! 所以 `Some(42)` 完整写法是 `Option::Some(42)`, Rust 帮你省掉了前缀。
//! 但你自己定义的枚举没有这个待遇——比如 `enum MyEnum { A, B }`, 你必须写
//! `MyEnum::A`, 不能只写 `A`。
//!
//! 前置依赖: basic/ 中的 match、Vec; intermediate/ 中的 structs_and_enums.


// ═══════════════════════════════════════════════════════════════
// 第 1 节: Some 和 None — 如何创建 Option
// ═══════════════════════════════════════════════════════════════

/// 返回 Option<i32> 意味着: 函数可能成功(有值), 也可能失败(没值)。
/// 参数: a, b: i32 — 都是 Copy, 只是复制进函数, 不消耗调用方的变量。
/// 返回值: Option<i32> — i32 是 Copy, Some(a/b) 复制一份; None 没有数据。
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None                     // 失败: None — 没有"值"需要返回
    } else {
        Some(a / b)              // 成功: a/b 的结果是 i32 Copy, 包进 Some
    }
}


// ═══════════════════════════════════════════════════════════════
// run
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    // ===== Option 创建 =====
    println!("===== 创建 Option =====");

    let some_val = Some(42);               // Some(42) 构造一个 Option<i32>, 编译器推断 T = i32
    let none_val: Option<i32> = None;      // None 需要类型注解: 编译器不知道它是 Option<什么> 的 None
    // 如果只写 let x = None; → 编译报错! 因为 None 可以是 Option<i32>、Option<String>、...

    println!("some_val = {:?} (类型: Option<i32>)", some_val);
    println!("none_val = {:?} (类型: Option<i32>)", none_val);

    // 从 Vec 获取: .get() 返回 Option, 因为索引可能越界
    // 所有权: .get(i) 借 v, 返回 Option<&i32> — 里面的引用借自 v。
    let v = vec![10, 20, 30];
    let first = v.get(0);                  // Option<&i32> — 借 v 内部元素, v 仍可用
    let out = v.get(99);                   // None
    println!("v.get(0)  = {:?}", first);
    println!("v.get(99) = {:?}", out);

    // ===== match 处理 Option =====
    println!("\n===== match 处理 =====");
    // match 是处理 Option 最基础的方式: 编译器强制你覆盖 Some 和 None 两个分支。

    match first {
        Some(val) => println!("第一个元素: {}", val),  // 匹配成功分支
        None => println!("没有元素"),                   // 匹配失败分支 — 不能漏!
    }

    match out {
        Some(val) => println!("第 100 个元素: {}", val),
        None => println!("索引越界!"),
    }

    // ===== if let: match 的简化 — 只关心一个分支 =====
    println!("\n===== if let: 只关心一个分支 =====");
    //
    // if let 做的事: 把 match 的"一个分支 + _ 兜底"压缩成一行。
    // 等价关系:
    //
    //   match expr {           ──→   if let 模式 = expr {
    //       模式 => { ... }              ...
    //       _ => {}                 }
    //   }
    //
    // 什么时候用: 你只关心"有没有值", 不关心"没值时做什么"。

    // match 写法 — 两分支都得写, 但 _ => {} 完全是空的:
    match first {
        Some(val) => println!("match: 有值 {}", val),
        _ => {}                                     // 占位而已, 删不掉
    }

    // if let 写法 — 等价上面, 省掉了 _ => {}:
    if let Some(val) = first {                      // "如果 first 是 Some, 把值绑到 val"
        println!("if let: 有值 {}", val);
    }                                               // 不需要 else, 不匹配时直接跳过

    // if let + else — 等价 match + 两个分支都有内容:
    //
    //   match expr {
    //       模式  => { ... }
    //       _     => { ... }    ← else 替代了这个 _
    //   }
    if let Some(val) = out {
        println!("有值: {}", val);                   // Some 分支
    } else {
        println!("没有值, 走了 else 分支");           // None 分支 (= match 里的 _)
    }

    // ===== while let: 循环 + 模式匹配 =====
    println!("\n===== while let: 匹配到才循环 =====");

    let mut stack = vec![1, 2, 3];                   // stack 拥有 Vec

    // while let 典型场景: pop() 返回 Option<T> — 元素所有权从 Vec 移给 top。
    // 每次 pop 成功, top (i32) 所有权从 Vec 转移到循环变量。
    print!("出栈顺序: ");
    while let Some(top) = stack.pop() {             // pop() → 每次移出一个元素
        print!("{} ", top);                          // top: i32, 所有权在循环体内
    }                                                 // top 离开作用域, i32 被 drop
    println!("(栈已空)");

    // ===== 常用方法 =====
    println!("\n===== 常用方法 =====");

    let s = Some(100);
    let n: Option<i32> = None;

    // is_some / is_none: 判断有没有值 (不取出值)
    println!("s.is_some()    = {}", s.is_some());
    println!("n.is_none()    = {}", n.is_none());

    // unwrap: 取出值, 如果是 None 则 panic (崩溃)
    println!("s.unwrap()     = {}", s.unwrap());
    // println!("{}", n.unwrap());          // 会 panic! 谨慎使用

    // expect: 同 unwrap, 但可以自定义 panic 消息 (方便定位)
    println!("s.expect(\"err\") = {}", s.expect("不应该失败"));

    // unwrap_or: 失败时返回默认值 (不会 panic)
    println!("s.unwrap_or(0) = {}", s.unwrap_or(0));
    println!("n.unwrap_or(0) = {}", n.unwrap_or(0));

    // unwrap_or_else: 失败时调用闭包产生默认值 (惰性, 只有 None 时才执行)
    println!("n.unwrap_or_else(|| 1+2) = {}", n.unwrap_or_else(|| 1 + 2));

    // map: 如果有值, 对它做转换; None 保持 None。
    // 所有权: map 消耗 Option, 内部值传给闭包。闭包返回新值包进 Some。
    let doubled = s.map(|x| x * 2);                // s: i32 Copy → x: i32, map 返回 Option<i32>
    let still_none = n.map(|x| x * 2);             // n 是 None → 闭包不执行, 返回 None
    println!("s.map(|x| x*2)   = {:?}", doubled);
    println!("n.map(|x| x*2)   = {:?}", still_none);

    // and_then: 类似 map, 但闭包返回的是 Option (链式操作中某一环可能产生 None)
    let result = v.get(0)           // Option<&i32> — 借用 v
        .map(|x| x * 3)              // Some(30): &i32 * 3 → i32 × 3 = 30
        .and_then(|x| if x > 20 { Some(x) } else { None }); // x: i32 — 这里已脱离 v, 不再借用

    // ===== 实践: 安全除法 =====
    println!("\n===== 实践: 安全除法 =====");
    println!("10 / 2 = {:?}", safe_div(10, 2));
    println!("10 / 0 = {:?}", safe_div(10, 0));
    println!("10 / 3 = {:?} (unwrap_or 兜底 -1)", safe_div(10, 3).unwrap_or(-1));
}
