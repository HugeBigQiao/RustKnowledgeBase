//! 本模块展示 Rust 的内置标量类型：整数、浮点、布尔、字符，整数的回绕机制，类型转换、数字字面量。

/// 基础类型罗列：u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64 bool char
fn basic_type(){
    let a: u8 = 255;                            // u8：无符号 8 位整数（0 ~ 255）
    let b: i8 = -128;                           // i8：有符号 8 位整数（-128 ~ 127）
    let c: u16 = 65535;                         // u16：无符号 16 位整数（0 ~ 65535）
    let d: i16 = -32768;                        // i16：有符号 16 位整数（-32768 ~ 32767）
    let e: u32 = 4294967295;                    // u32：无符号 32 位整数（0 ~ 约 42 亿）
    let f: i32 = -2147483648;                   // i32：有符号 32 位整数，Rust 默认整数类型
    let g: u64 = 18446744073709551615;          // u64：无符号 64 位整数
    let h: i64 = -9223372036854775808;          // i64：有符号 64 位整数
    let u: usize = 42;                          // usize：无符号指针位宽整数（32/64 位取决于架构）
    let v: isize = -42;                         // isize：有符号指针位宽整数（32/64 位取决于架构）
    let i: f32 = 3.14;                          // f32：32 位浮点数（IEEE 754 单精度）
    let j: f64 = 2.71828;                       // f64：64 位浮点数（IEEE 754 双精度），Rust 默认浮点类型
    let k: bool = true;                         // bool：布尔值（true 或 false）
    let l: char = 'a';                          // char：永远是 4 字节 Unicode 标量值（不是 C 的 1 字节）
    // char 可以存: 单个字母 'a'、数字 '7'、汉字 '中'、符号 '😀' 等任何单个 Unicode 字符
    // 注意: 'ab' 不行(两个字母), "" 不行(那是字符串 str)
    println!("u8   (无符号 8 位整数) : {}", a);
    println!("i8   (有符号 8 位整数) : {}", b);
    println!("u16  (无符号 16 位整数): {}", c);
    println!("i16  (有符号 16 位整数): {}", d);
    println!("u32  (无符号 32 位整数): {}", e);
    println!("i32  (有符号 32 位整数): {}  ← Rust 默认整数类型", f);
    println!("u64  (无符号 64 位整数): {}", g);
    println!("i64  (有符号 64 位整数): {}", h);
    println!("usize(无符号指针宽整数): {}  ← 32/64 位取决于架构", u);
    println!("isize(有符号指针宽整数): {}  ← 32/64 位取决于架构", v);
    println!("f32  (32 位浮点数)     : {}", i);
    println!("f64  (64 位浮点数)     : {}  ← Rust 默认浮点类型", j);
    println!("bool (布尔值)          : {}", k);
    println!("char (Unicode 标量)    : {}  ← 固定 4 字节，存的是 U+0061", l);
}

/// 整数溢出的四种处理方式
///
/// Rust 整数溢出行为：
/// - debug 模式：直接 panic（防止隐藏 bug）
/// - release 模式：自动回绕（wrapping，性能优先）
///
/// 四种显式处理方法：
/// - wrapping_*  ：溢出时回绕（如 255u8 + 1 = 0），始终成功
/// - saturating_*：溢出时饱和（如 255u8 + 1 = 255），始终成功
/// - checked_*   ：返回 Option，溢出时 None，由调用者处理
/// - overflowing_*：返回 (值, bool)，bool 表示是否溢出
///
/// 使用场景：
/// - wrapping    → 密码学、哈希、环形缓冲区
/// - saturating  → 音频/图像处理、颜色值（不能超出 0~255）
/// - checked     → 需要明确感知溢出的业务逻辑
/// - overflowing → 需要同时拿到结果和溢出标志
fn integer_wrapping() {
    let x: u8 = 255;
    let wrapping_add = x.wrapping_add(1);
    let saturating_add = x.saturating_add(1);
    let checked_add = x.checked_add(1);
    let (result, overflowed) = x.overflowing_add(1);
    println!("\n整数溢出处理（以 u8 最大值 255 为例）");
    println!("初始值 x = {}", x);
    println!("wrapping_add(1)       : {}  ← 溢出回绕到 0", wrapping_add);
    println!("saturating_add(1)     : {}  ← 饱和在 255", saturating_add);
    println!("checked_add(1)        : {:?} ← 溢出返回 None", checked_add);
    println!("overflowing_add(1)    : ({}, {})  ← (回绕值, 是否溢出)", result, overflowed);
}

/// 数字字面量：不同进制表示及格式化输出 
/// Rust 支持四种进制字面量：
/// - 十进制：`123` 或 `1_000_000`（下划线分隔，增强可读性）
/// - 十六进制：`0xFF`
/// - 八进制：`0o77`
/// - 二进制：`0b1111_0000`
fn numeric_literals() {
    let dec = 255;              // 十进制
    let hex = 0xff;             // 十六进制
    let oct = 0o377;            // 八进制
    let bin = 0b1111_1111;      // 二进制
    let big = 1_000_000u32;     // 下划线分隔，增强可读性

    println!("\n数字字面量（不同进制表示同一个值）");
    println!("------------------------------------------------------------");
    println!("十进制 {}  |  {:b} (二进制) |  {:o} (八进制) |  {:x} (十六进制)", dec, dec, dec, dec);
    println!("十六进制 0x{:x} = {} (十进制)", hex, hex);
    println!("八进制 0o{:o} = {} (十进制)", oct, oct);
    println!("二进制 0b{:b} = {} (十进制)", bin, bin);
    println!("下划线分隔: {} (等价于 1000000)", big);
}

/// 类型转换注意事项 
/// Rust 的类型转换非常严格：
/// - 可以用 `as` 做显式转换，但会直接截断/丢失数据，不报警
/// - 大转小：高位被砍掉（如 u16 的 300 转 u8 = 44）
/// - 有符号 ↔ 无符号：位模式不变，但解释不同
/// - 浮点 → 整数：直接丢掉小数部分（不是四舍五入）
/// - 推荐用 `try_into()` 或 `From`/`Into` 做安全转换
fn type_conversion() {
    println!("\n类型转换注意事项");
    println!("------------------------------------------------------------");

    // 大转小：截断高位
    let large: u16 = 300;
    let small: u8 = large as u8; // 300 % 256 = 44
    println!("u16({}) as u8 = {}  ← 高位被截断（300 % 256 = 44）", large, small);

    // 有符号 → 无符号：位模式直接解释
    let neg: i8 = -1;           // 补码：1111_1111
    let unsigned: u8 = neg as u8; // 当成无符号 = 255
    println!("i8({}) as u8 = {}  ← 补码位模式不变，解释不同", neg, unsigned);

    // 无符号 → 有符号（超出范围）
    let big_u: u8 = 255;
    let signed: i8 = big_u as i8; // 255 超出 i8 范围（-128~127），位模式当补码 = -1
    println!("u8({}) as i8 = {}  ← 超出范围，位模式当补码", big_u, signed);

    // 浮点 → 整数：截断小数
    let pi: f64 = 3.99;
    let int_pi: i32 = pi as i32; // 直接丢掉小数 = 3（不四舍五入）
    println!("f64({}) as i32 = {}  ← 直接截断小数（不是四舍五入）", pi, int_pi);

    // -------- 安全转换：try_into() --------
    let source: u16 = 42;
    let safe: Result<u8, _> = source.try_into(); // Ok(42)，因为在 u8 范围内
    println!("u16({}) try_into u8 = {:?}  ← 在范围内则 Ok", source, safe);

    let overflow_source: u16 = 500;
    let fail: Result<u8, _> = overflow_source.try_into(); // Err，因为 500 > 255
    println!("u16({}) try_into u8 = {:?}  ← 超出范围则 Err", overflow_source, fail);
}

/// 逐个声明 Rust 各基础类型的变量，覆盖整数族、浮点、布尔、char，以及整数的回绕机制、类型转换、数字字面量
pub fn run() {
    basic_type();
    integer_wrapping();
    numeric_literals();
    type_conversion();
}

