//! 宏: macro_rules! 声明宏.

// ===== 1. 最简单的宏 =====

/// 无参数宏, 类似编译期文本替换.
macro_rules! hello {
    () => {
        println!("Hello from macro!");
    };
}

/// 演示宏的基本调用.
fn demo_simple_macro() {
    println!("--- 简单宏 ---");
    hello!();
}

// ===== 2. 带参数的宏 =====

/// 类似 vec! 的初阶版本.
macro_rules! my_vec {
    // 捕获一个表达式, 重复 0 次或多次, 分号分隔
    ($($x:expr),* $(,)?) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

/// 演示带参数的声明宏.
fn demo_vec_macro() {
    println!("\n--- 自定义 vec! 宏 ---");

    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec![1,2,3,4,5] = {:?}", v);

    let v2 = my_vec!["a", "b", "c"];
    println!("my_vec![\"a\",\"b\",\"c\"] = {:?}", v2);
}

// ===== 3. 多模式匹配 =====

/// 根据参数个数走不同分支.
macro_rules! add {
    // 两个参数
    ($a:expr, $b:expr) => {
        $a + $b
    };
    // 三个参数
    ($a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    };
}

/// 演示宏的多模式匹配.
fn demo_multi_pattern() {
    println!("\n--- 宏多模式匹配 ---");

    let sum2 = add!(1, 2);
    let sum3 = add!(1, 2, 3);
    println!("add!(1,2) = {}", sum2);
    println!("add!(1,2,3) = {}", sum3);
}

// ===== 4. 捕获不同类型 =====

/// 宏捕获类型说明:
///   expr - 表达式
///   ty   - 类型
///   ident- 标识符
///   tt   - token 树 (最通用)
///   literal - 字面量
macro_rules! create_struct {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        struct $name {
            $($field: $type),*
        }

        impl $name {
            fn new($($field: $type),*) -> Self {
                $name { $($field),* }
            }
        }
    };
}

// 使用宏生成结构体
create_struct!(Point { x: f64, y: f64 });

/// 演示宏生成代码.
fn demo_code_gen() {
    println!("\n--- 宏代码生成 ---");

    let p = Point::new(3.0, 4.0);
    println!("create_struct! 生成的 Point{{ x: {}, y: {} }}", p.x, p.y);
}

// ===== 5. 标准宏简介 =====

/// 介绍 Rust 内置的常用宏.
fn demo_builtin_macros() {
    println!("\n--- 常用内置宏 ---");

    // println! / format! / eprintln!
    println!("  println! / format! / eprintln! -- 格式化输出");

    // vec!
    println!("  vec!      -- 创建 Vec");

    // assert! / assert_eq! / assert_ne! / debug_assert!
    println!("  assert!   -- 断言测试");

    // todo! / unimplemented! / unreachable!
    println!("  todo!     -- 标记未完成的代码");

    // dbg! / include_str! / env!
    println!("  dbg!      -- 调试打印");

    // cfg! / compile_error! / line! / column! / file!
    println!("  cfg!      -- 条件编译");

    println!("\n  // 过程宏 (proc macro) 是另一类宏:");
    println!("  // #[derive(Debug)] -- 自动实现 trait");
    println!("  // #[tokio::main]    -- 属性宏");
    println!("  // 过程宏需要单独的 proc-macro crate");
}

pub fn run() {
    demo_simple_macro();
    demo_vec_macro();
    demo_multi_pattern();
    demo_code_gen();
    demo_builtin_macros();
}
