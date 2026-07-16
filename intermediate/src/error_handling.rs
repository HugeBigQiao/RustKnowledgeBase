//! 错误处理专题: Rust 的报错与兜底机制全览。
//!
//! Rust 的错误处理分为三种机制, 按推荐程度从高到低:
//!
//!   1. Result<T, E> — 可恢复错误
//!      → 把错误编码到返回值里, 强制调用者处理, 不崩溃。
//!      → Result 是枚举: Ok(T) 成功 / Err(E) 失败。
//!      → 处理方式: match、unwrap、expect、unwrap_or。
//!
//!   2. ? 运算符 — 错误传播语法糖
//!      → 把"检查 → 成功继续 / 失败返回"的重复 match 压缩成一个 ?。
//!      → 本身不处理错误, 只负责向上传递。
//!
//!   3. panic! — 不可恢复错误 (兜底机制)
//!      → 程序直接崩溃, 无法挽救。类似其他语言的"未捕获异常"。
//!      → 仅用于"不应该发生"的致命场景: 逻辑 bug、违反不变量。
//!
//! 阅读顺序: 第 1 节 → 第 2 节 → 第 3 节 → 第 4 节 (综合对比)。
//!
//! 前置依赖: basic/ 中的 match; intermediate/ 中的 structs_and_enums、option.


// ═══════════════════════════════════════════════════════════════
// 第 1 节: Result<T, E> — 可恢复错误
// ═══════════════════════════════════════════════════════════════
//
// Result 是一个枚举, 把"可能失败"编码到类型系统里, 强制调用者处理:
//
//   enum Result<T, E> {
//       Ok(T),   // 成功, 携带返回值
//       Err(E),  // 失败, 携带错误信息
//   }
//
// 对比 panic: panic 是 "我死了", Result 是 "我可能失败了, 你自己看着办"。

// ── 1.1 如何创造 Result ──
//
// 关键理解: Result 是枚举, Ok 和 Err 是它的两个变体(类似 Option 的 Some/None)。
// 函数声明返回 Result<i32, String>, 意思是"返回值是一个 Result,
// 它可能是 Ok(i32) 也可能是 Err(String)"——二者取其一, 不是两个都要。
//
// 类比: 你去考试, 结果要么"通过(带分数)"要么"挂科(带原因)", 不会同时发生。
//   Ok(90)  → 通过了, 带着 90 分
//   Err("缺考") → 挂了, 原因是缺考
// 两个都是合法的 Result 值, 函数只需要返回其中一种。
//
// 语法糖: Ok(42) 完整写法是 Result::Ok(42), Rust 允许省略 Result:: 前缀。

/// 制造一个成功的 Result — 只返回 Ok 变体。
/// 虽然返回类型写的是 Result<i32, String>, 但 Ok(42) 本身就是合法的 Result 值。
/// 所有权: i32 是 Copy, 42 被复制进 Ok; Result 的所有权从函数移给调用方。
fn make_ok() -> Result<i32, String> {
    Ok(42)                                          // Ok(42) 构造的是 Result::Ok<i32, String>(42)
}

/// 制造一个失败的 Result — 只返回 Err 变体。
/// 同样, Err(String::from("出错了")) 是合法的 Result<i32, String> 值。
/// 所有权: 新建的 String 所有权进 Err → 再移出函数给调用方。
fn make_err() -> Result<i32, String> {
    Err(String::from("出错了"))                       // Err(...) 构造的是 Result::Err<i32, String>(...)
}

// ── trim / parse / map_err 是什么 ──
//
// parse_number 用到了三个链式方法, 逐一说明:
//
// 1. trim() — 去掉字符串首尾空白 (空格、换行、制表符)
//    输入: &str (借来的字符串)
//    输出: &str (仍是借用, 只是切片范围变了, 不分配新内存)
//    例: "  42 \n".trim() → "42"
//    场景: 用户输入经常带空格, 先用 trim 清理再解析。
//
// 2. parse() — 把字符串解析成目标类型
//    输入: &str (在哪个字符串上调用)
//    输出: Result<T, ParseIntError> (成功返回解析出的值, 失败返回标准库错误)
//    泛型: parse::<i32>() 解析成 i32; parse::<f64>() 解析成 f64
//    例: "42".parse::<i32>() → Ok(42); "abc".parse::<i32>() → Err(...)
//    注意: parse 是 &str 上的方法, 不消耗原字符串。
//
// 3. map_err(|e| ...) — 转换 Result 里的 Err 变体, Ok 原样保留
//    输入: Result<T, E> (消耗这个 Result)
//    闭包参数: e — Err 里的错误值 (注意: 闭包拿走 e 的所有权!)
//    输出: Result<T, NewE> (Ok 不变, Err 被闭包返回值替换)
//    例: Ok(42).map_err(|e| format!("...{}", e)) → Ok(42) 不变
//        Err(raw).map_err(|e| format!("...{}", e)) → Err("...raw...")
//    场景: 把标准库的 ParseIntError 转成自己的 String, 方便统一错误类型。
//
// 链式调用的所有权流向:
//   s.trim()           → &str (借 s, 不消耗)
//   .parse()           → Result<i32, ParseIntError> (消耗 &str 上的迭代器? 不, parse 只读)
//   .map_err(|e| ...)  → Result<i32, String> (消耗原 Result, 产生新 Result)

/// 标准库自带 Result 的经典例子: parse()。
/// 把 &str 解析成数字, trim → parse → map_err 串联处理。
/// 参数: s: &str — 借用, 调用后原字符串仍然可用。
/// 返回: Result<i32, String> — 成功值/错误信息的所有权移给调用方。
fn parse_number(s: &str) -> Result<i32, String> {
    s.trim()                                         // 1. trim: 去首尾空白, 返回 &str (仍借用 s)
        .parse()                                     // 2. parse: 尝试解析为 i32, 返回 Result<i32, ParseIntError>
        .map_err(|e| format!("'{}' 解析失败: {}", s, e))  // 3. map_err: 把 Err(ParseIntError) 转成 Err(String)
}                                                    //    |e| 闭包拿到 e (ParseIntError, 所有权被闭包拿走)
                                                     //    format! 创建新 String → 包进 Err → 返回

// ── 1.2 处理 Result: 四种写法对比 ──
//
// 前面的 make_ok() / make_err() / parse_number() 只是"生产" Result。
// 拿到 Result 之后怎么"消费"它? 这才是实际业务代码要做的事。
//
// 以下四个函数做的事: 接收一个 Result (来自其他函数的返回值),
// 然后各自用不同的方式处理它。你可以把它们的参数想象成:
//   handle_xxx(  make_ok()的返回值  )  ← 把"生产"和"处理"连起来了
//
// 四种写法的核心区别一句话:
//   A. match    → 拿出值来打印, 函数返回 ()  (最安全: 必须处理两个分支)
//   B. unwrap   → 拿出 Ok 值返回; Err 则 panic  (偷懒: 失败了就崩溃)
//   C. unwrap_or → 拿出 Ok 值返回; Err 给默认值 (不崩溃, 但要提供默认值)
//   D. unwrap_or_else → 同上, 但默认值用闭包"惰性"计算
//
//  写法 │ 参数                    │ 最终输出             │ Ok时         │ Err时
//  ─────┼─────────────────────────┼──────────────────────┼──────────────┼──────────────
//   A    │ &Result<i32,String>     │ () (打印到屏幕)       │ 打印成功值    │ 打印错误信息
//   B    │ Result<i32,String>      │ i32 或 崩溃(panic)    │ 返回 i32      │ panic! 崩溃
//   C    │ Result<i32,String> + 默认值│ i32 (永不崩溃)     │ 返回 i32      │ 返回默认值
//   D    │ Result<i32,String> + 闭包 │ i32 (永不崩溃)     │ 返回 i32      │ 执行闭包,返回其值

/// 写法 A: match 穷尽匹配 — 最安全。
/// ─── 参数与输出 ───
/// 参数: &Result<i32, String> — 借用, 不消耗。相当于"借来看一眼"。
/// 输出: () (无返回值, 仅打印到屏幕)
/// Ok/Err 两个分支都必须写, 编译器强制穷尽检查。
/// ─── 所有权 ───
/// match 分支里的模式解构: Ok(v) 拿到 &i32, Err(e) 拿到 &String (都是借的引用)。
fn handle_via_match(result: &Result<i32, String>) {
    match result {
        Ok(v) => println!("  match → 成功: {}", v),     // v: &i32, 借的引用
        Err(e) => println!("  match → 失败: {}", e),     // e: &String, 也借的引用
    }
}

/// 写法 B: unwrap / expect — 偷懒, 失败就 panic。
/// ─── 参数与输出 ───
/// 参数: Result<i32, String> — 所有权移入本函数 (不是借用!)。
///       unwrap 不需要额外参数; expect 需要一个 &str 作为自定义崩溃消息。
/// 输出: i32 (成功时) 或 崩溃 (失败时)
/// ─── 所有权 ───
/// 消耗 Result, 拿走里面的 Ok 值。如果 Err 则 panic, 程序终止。
#[allow(dead_code)]
fn handle_via_unwrap(result: Result<i32, String>) -> i32 {   // ← Result 进入, 所有权归本函数
    result.unwrap()                                          // ← 消耗 Result: Ok→取出值; Err→panic
    // result 已失效, 不能再访问
}

#[allow(dead_code)]
fn handle_via_expect(result: Result<i32, String>) -> i32 {
    result.expect("这里不应该失败")                    // Err 时 panic, 消息自定义 → 更好定位
}

/// 写法 C: unwrap_or — 失败给默认值。
/// ─── 参数与输出 ───
/// 参数: Result<i32, String> (所有权移入) + 默认值 i32
///       默认值是一般的 i32 值, 如 unwrap_or(0) 中的 0。
/// 输出: i32 (永不崩溃 — 成功拿 Ok 值, 失败拿默认值)
/// ─── 写法 D: unwrap_or_else — 惰性求值版 ───
/// 和 unwrap_or 一样, 但默认值不预先算好, 而是传一个闭包。
/// 参数: Result<i32, String> + 闭包 FnOnce(E) -> i32
///       闭包接收 Err 里的错误值 (String), 返回默认 i32。
/// 输出: i32 (永不崩溃)
/// 优势: Err 时才执行闭包, 省去预先计算默认值的开销。
#[allow(dead_code)]
fn handle_via_default(result: Result<i32, String>) -> i32 {   // ← 消耗 Result
    result.unwrap_or(0)                             // 0 是 i32, Copy, 直接用
}

fn handle_via_default_lazy(result: Result<i32, String>) -> i32 {  // ← 消耗 Result
    result.unwrap_or_else(|e| {                     // |e| 闭包: e 是 Err 里的 String, 借给闭包使用
        println!("  兜底: 因为 '{}' 所以用 0 代替", e);
        0
    })
}


// ═══════════════════════════════════════════════════════════════
// 第 2 节: ? 运算符 — 错误传播
// ═══════════════════════════════════════════════════════════════
//
// ? 做的事情: Ok(v) → 取出 v 继续;  Err(e) → 立即 return Err(e) 退出。
// 本质: 把"检查错误 → 成功继续 / 失败返回"的重复 match 压缩成一个字符。
// 要求: 当前函数必须返回 Result (或 Option), 否则 ? 不知道返回什么。

/// 没有 ? 的写法: 每一步都得手动 match，错误处理和业务逻辑混在一起。
fn process_number_no_q(s: &str) -> Result<i32, String> {
    // Step 1: 解析 → 手动 match
    let num: i32 = match s.trim().parse() {         // parse 返回 Result
        Ok(n) => n,                                 // 成功 → 取值继续
        Err(e) => return Err(format!("'{}' 不是有效数字: {}", s, e)), // 失败 → 包装错误并返回
    };

    // Step 2: 业务校验
    if num <= 0 {                                   // 即使 parse 成功, 负数也不接受
        return Err(format!("{} 不是正数", num));     // 这是业务错误, 不是解析错误
    }

    // Step 3: 核心逻辑 — 翻倍
    Ok(num * 2)
}

/// 有 ? 的写法: 同上逻辑, 但 ? 替代了重复的 match { Ok(n)=>n, Err(e)=>return Err(...) }。
fn process_number(s: &str) -> Result<i32, String> {
    // --- ? 的作用: 成功则取值, 失败则提前 return ---
    let num: i32 = s.trim()
        .parse()                                         // → Result<i32, ParseIntError>
        .map_err(|e| format!("'{}' 不是有效数字: {}", s, e)) // 标准库错误 → String
        ?;                                               // ← Ok(42) 取出 42 / Err 则 return
    // ? 之后 num 就是 i32 了 (不是 Result), 可以直接用

    // --- 走读: 不同输入时 ? 的行为 ---
    //
    // 输入 "42":
    //   parse() → Ok(42) → map_err 不执行 → ? 取出 42 → 继续到 if 检查
    //   42 > 0 → 跳过 if → Ok(42*2) = Ok(84) ✓
    //
    // 输入 "-5":
    //   parse() → Ok(-5) → ? 取出 -5 → if 检查
    //   -5 <= 0 → return Err("-5 不是正数") ✗  (注意: ? 没参与, 是业务校验返回的错误)
    //
    // 输入 "abc":
    //   parse() → Err(ParseIntError) → map_err 包装成 String → Err("'abc' 不是有效数字: ...")
    //   ? 遇到 Err → 立即 return Err(...) ✗  (Step 2/Step 3 都不会执行!)

    if num <= 0 {                                       // 业务校验: ? 只能处理 Result, 这种 if 判断不行
        return Err(format!("{} 不是正数", num));         // 所以还是手动 return
    }

    Ok(num * 2)                                         // 最终成功返回
}
// --- 上层如何接收 ---
// match process_number(s) {
//     Ok(v)  => println!("结果: {}", v),   ← ? 一路上来都没出错, 最终走到 Ok 分支
//     Err(e) => println!("错误: {}", e),   ← ? 在哪一步碰到 Err, 最终都走到 Err 分支
// }
//
// 关键理解: ? 不处理错误, 只传递。
// 就像快递: ? 是快递员(一路传递), match 是收件人(最终打开决定怎么处理)。

/// 当函数只有简单一步操作时, ? 的威力最明显。
fn parse_and_double(s: &str) -> Result<i32, String> {
    let num: i32 = s.trim().parse()
        .map_err(|e| format!("'{}' 解析失败: {}", s, e))?; // 一行搞定解析 + 错误传播
    Ok(num * 2)
}


// ═══════════════════════════════════════════════════════════════
// 第 3 节: panic! — 不可恢复错误 (兜底机制)
// ═══════════════════════════════════════════════════════════════
//
// panic 是 Rust 的"崩溃"机制: 一旦触发, 程序立即终止, 栈展开并打印错误信息。
// 当前面的 Result + ? 都无法处理时, panic 是最后的兜底手段。
// 适用于"不该发生"的致命错误: 逻辑 bug、违反不变量、无法继续运行。
//
// 以下 7 个函数演示 7 种触发 panic 的方式:

/// 1. panic! 宏 — 主动触发崩溃。
#[allow(dead_code)]
fn panic_via_macro() {
    panic!("主动调用了 panic!");                     // 主动崩溃
    // 这行永远执行不到
}

/// 2. unwrap 遇到 Err — 想取 Ok 的值但拿到了 Err。
#[allow(dead_code)]
fn panic_via_unwrap() -> i32 {
    let result: Result<i32, &str> = Err("something went wrong");
    result.unwrap()                                 // ← panic: Err 上调 unwrap
}

/// 3. expect 遇到 Err — 同上, 但可自定义崩溃消息, 方便定位。
#[allow(dead_code)]
fn panic_via_expect() -> i32 {
    let result: Result<i32, &str> = Err("未连接数据库");
    result.expect("启动失败: 数据库不可用")            // ← panic 消息: "启动失败: 数据库不可用: ..."
}

/// 4. 数组越界 — 访问不存在的索引。arr[idx] 是索引操作, 借数组但不转移所有权。
/// idx >= len 时 panic (不是编译时检查, 是运行时)。
#[allow(dead_code)]
fn panic_via_out_of_bounds(idx: usize) -> i32 {
    let arr = [1, 2, 3];                             // 栈上的定长数组
    arr[idx]                                         // 索引访问: 不消耗 arr, 返回 &i32 或 panic
}                                                    // usize 是 Copy, idx 不消耗

/// 5. 断言失败 — assert! 条件不满足。
#[allow(dead_code)]
fn panic_via_assert() {
    assert!(2 + 2 == 5, "数学崩了: 2 + 2 不等于 5"); // 编译通过, 运行时条件为 false 就 panic
}

/// 6. unreachable! — 标记"理论上不可达"的代码路径。
#[allow(dead_code)]
fn panic_via_unreachable() -> &'static str {
    match 42 {
        0..=50 => "小",
        51..=100 => "大",
        _ => unreachable!("42 不可能不在 0~100 里"), // 走到这里说明代码逻辑有 bug
    }
}

/// 7. todo! — 标记未完成, 编译通过但运行即崩, 方便开发时占位。
#[allow(dead_code)]
fn panic_via_todo() -> String {
    todo!("这个函数还没写, 先占个位置")               // 编译 OK, 运行 panic
}


// ═══════════════════════════════════════════════════════════════
// 第 4 节: 综合对比
// ═══════════════════════════════════════════════════════════════
//
// 三种机制的选择策略:
//
//   Result → 调用者可能想处理的可恢复错误 (首选)
//           例: 用户输入无效、网络超时、文件未找到
//
//   ?      → Result 的传播工具, 本身不处理错误, 只负责层层上传 (中间层标配)
//           例: 任何需要把错误往上传递的中间函数
//
//   panic  → "程序不应该继续运行"的致命错误 (最后兜底)
//           例: 配置文件缺失且无默认值、逻辑不可能到达的代码、违反不变量

/// 综合示例: 安全除法 — 用 Result 处理除零, 而不是 panic 崩溃。
fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(format!("除零错误: {} / {}", a, b))       // 返回错误, 不崩溃
    } else {
        Ok(a / b)                                     // 正常结果
    }
}


// ═══════════════════════════════════════════════════════════════
// run — 按 4 个节依次演示
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    // ===== 第 1 节: Result =====
    println!("===== 第 1 节: Result — 可恢复错误 =====");

    // 创造 Result — 返回值的所有权从函数移出
    println!("--- 创造 Result ---");
    println!("  Ok:  {:?}", make_ok());               // make_ok() 返回 Result, 所有权交给 println! 消耗
    println!("  Err: {:?}", make_err());
    println!("  parse(\"42\"):  {:?}", parse_number("42"));   // "42" 是 &str 字面量, 借给 parse_number
    println!("  parse(\"abc\"): {:?}", parse_number("abc"));

    // 处理 Result: 四种写法对比
    println!("\n--- 处理 Result: 四种写法对比 ---");
    let r_ok: Result<i32, String> = Ok(42);           // r_ok 拥有这个 Result
    let r_err: Result<i32, String> = Err(String::from("数据损坏"));

    println!("写法 A: match (最安全, 编译器强制处理两个分支)");
    handle_via_match(&r_ok);                          // &r_ok → 借用, r_ok 仍可用
    handle_via_match(&r_err);                         // &r_err → 借用

    println!("\n写法 B: unwrap / expect (失败就 panic, 这里只演示 Ok 的情况)");
    println!("  Ok → unwrap()  = {}", r_ok.clone().unwrap());  // clone() → 复制一份 Result, 消耗克隆品
    // println!("{}", r_err.unwrap());              // ← Err 上调 unwrap 会崩溃

    println!("\n写法 C/D: unwrap_or / unwrap_or_else (给默认值, 不崩溃)");
    println!("  Ok  → unwrap_or(0)  = {}", r_ok.clone().unwrap_or(0));
    println!("  Err → unwrap_or(0)  = {}", r_err.clone().unwrap_or(0));
    println!("  Err → unwrap_or_else = {}",
        handle_via_default_lazy(r_err.clone()));

    // ===== 第 2 节: ? 运算符 =====
    println!("\n===== 第 2 节: ? 运算符 — 错误传播 =====");
    let test_cases = vec!["42", "-5", "abc"];        // Vec<&str>: 三个字符串字面量, 都是 &'static str (Copy)

    println!("--- 没有 ? (手动 match, 6 行代码只做了 1 件事) ---");
    for s in &test_cases {                            // &test_cases → 借用 Vec, 不消耗
        match process_number_no_q(s) {                 // s 是 &&str, 传 &str 给函数 (自动解引用)
            Ok(v) => println!("  process_no_q(\"{}\") = {}", s, v),
            Err(e) => println!("  process_no_q(\"{}\") → 错误: {}", s, e),
        }
    }

    println!("\n--- 有 ? (自动传播, 1 个字符替代 6 行) ---");
    for s in &test_cases {
        match process_number(s) {
            Ok(v) => println!("  process(\"{}\") = {}", s, v),
            Err(e) => println!("  process(\"{}\") → 错误: {}", s, e),
        }
    }

    println!("\n--- parse + ? 结合 (单步操作的 ? 更简洁) ---");
    for s in &test_cases {
        match parse_and_double(s) {
            Ok(v) => println!("  \"{}\" → {}", s, v),
            Err(e) => println!("  \"{}\" → 错误: {}", s, e),
        }
    }

    // ===== 第 3 节: panic =====
    println!("\n===== 第 3 节: panic! — 兜底机制 =====");
    println!("当 Result 和 ? 都不适用时, panic 是最后手段。");
    println!("7 种触发方式 (全部被注释, 取消注释可看效果):");
    println!("  1. panic!(\"消息\")      → 主动崩溃");
    println!("  2. Err.unwrap()         → 想取 Ok 却拿到 Err");
    println!("  3. Err.expect(\"...\")   → 同上, 带自定义消息");
    println!("  4. arr[越界索引]         → 数组越界");
    println!("  5. assert!(...)          → 断言失败");
    println!("  6. unreachable!()        → 标记不可达代码");
    println!("  7. todo!()               → 标记未完成 (开发占位)");
    // 取消注释可看崩溃效果: panic!("演示: 主动触发 panic");

    // ===== 第 4 节: 综合对比 =====
    println!("\n===== 第 4 节: 综合对比 =====");
    println!("安全除法 — 用 Result 处理除零而非 panic:");
    let divs = [(10, 2), (7, 3), (5, 0)];             // [(i32, i32); 3] — 栈上定长数组, 每个元素是 Copy 元组
    for (a, b) in &divs {                              // &divs → 借用数组; (a, b) 解构出 &i32
        match safe_div(*a, *b) {                       // *a, *b → i32 (Copy 类型, 解引用同时复制)
            Ok(v) => println!("  {} / {} = {}", a, b, v),
            Err(e) => println!("  {} / {} → 错误: {}", a, b, e),
        }
    }

    println!("\n--- 选择策略 ---");
    println!("  Result → 首选: 调用者可能想处理的错误");
    println!("  ?      → 中间层: 自己不处理, 向上传递");
    println!("  panic  → 兜底: 程序不该继续的致命错误");
}
