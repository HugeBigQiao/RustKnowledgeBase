//! 自定义宏实战: macro_rules! 声明宏的高级模式。
//!
//! intermediate 的 macros_intro 介绍了宏是什么、有哪些类型。
//! 这里聚焦: 如何定义自己的宏, 捕获模式, 重复展开, 代码生成。
//!
//! # 过程宏
//! 过程宏需要单独的 proc-macro 类型 crate, 这里只做概念说明 (不实际写):
//!   - #[derive(Xxx)]:  派生宏 — 自动为 struct/enum 实现 trait
//!   - #[xxx]:           属性宏 — 如 #[tokio::main] 把 async main 转为运行时
//!   - xxx!(...):        函数式宏 — 如 sqlx::query!("SELECT ...")
//! 过程宏操作的是 TokenStream (词法单元流), 可以任意修改 AST。
//! 声明宏 (macro_rules!) 则是模式匹配驱动的文本替换。

// ===== 1. 基本捕获类型 =====

/// macro_rules! 支持的捕获类型 (fragment specifier):
///   expr   — 表达式 (最常用)
///   ident  — 标识符 (变量名/函数名/类型名)
///   ty     — 类型 (如 i32, Vec<String>)
///   tt     — Token 树 (最通用, 可匹配任何 token 序列)
///   literal- 字面量 (42, "hello", true)
///   stmt   — 语句
///   block  — 代码块 { ... }
///   pat    — 模式
///   path   — 路径 (如 std::collections::HashMap)

macro_rules! demo_fragment {
    // $x:ident — 捕获一个标识符 (必须放在 expr 前面, 因为 ident 也是合法的 expr)
    ($x:ident) => {
        println!("ident = {}", stringify!($x));
    };
    // $x:expr — 捕获一个表达式
    ($x:expr) => {
        println!("expr = {:?}", $x);
    };
}

fn demo_fragment_types() {
    println!("--- 捕获类型 (fragment specifier) ---");

    demo_fragment!(42);               // 匹配 expr 分支
    demo_fragment!(my_variable_name); // 匹配 ident 分支
}

// ===== 2. 重复模式: $(...)* 和 $(...),* =====

/// $(...)*       重复 0 次或多次, 分隔符在 (...) 内部
/// $(...),*      重复 0 次或多次, 逗号分隔
/// $(...);+      重复 1 次或多次, 分号分隔
///
/// 在展开体中用同样的 $(...)* 语法 "粘贴" 每个捕获值。

// 自定义 vec! 等价实现
macro_rules! my_vec {
    // $(...),*  : 捕获逗号分隔的表达式列表
    // $(,)?     : 允许末尾多一个逗号 (trailing comma)
    ($($x:expr),* $(,)?) => {
        {
            let mut temp = Vec::new();
            $(temp.push($x);)*  // 为每个 $x 生成一句 push
            temp
        }
    };
}

// 自定义 HashMap 字面量
macro_rules! my_map {
    // key => value 对, 逗号分隔
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn demo_repetition() {
    println!("\n--- 重复模式 ---");

    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec![1,2,3,4,5] = {:?}", v);

    let m = my_map! {
        "name" => "张三",
        "age" => "25",
    };
    println!("my_map! {{ \"name\"=>\"张三\", \"age\"=>\"25\" }} = {:?}", m);
}

// ===== 3. 多模式匹配 =====

/// 宏可以像 match 一样有多个分支, 按书写顺序匹配第一个成功的。
macro_rules! calculate {
    // 两个参数: 加法
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    // 两个参数: 乘法
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    // 三个参数: 加法
    (add $a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    };
}

fn demo_multi_pattern() {
    println!("\n--- 多模式匹配 ---");

    println!("calculate!(add 1, 2)     = {}", calculate!(add 1, 2));
    println!("calculate!(mul 3, 4)     = {}", calculate!(mul 3, 4));
    println!("calculate!(add 1, 2, 3)  = {}", calculate!(add 1, 2, 3));
}

// ===== 4. 代码生成: 用宏消除样板 =====

/// 自动为 struct 生成 new() 构造函数 + Debug 打印。
///
/// $name:ident  捕获结构体名
/// $($field:ident: $type:ty),*  捕获字段名: 类型 的列表
macro_rules! define_struct {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        // 定义结构体
        struct $name {
            $($field: $type),*
        }

        // 自动生成构造函数
        impl $name {
            fn new($($field: $type),*) -> Self {
                $name { $($field),* }
            }
        }

        // 自动生成 Debug 打印 (不用 derive, 演示原理)
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // stringify! 把 token 转成字符串
                write!(f, "{} {{ ", stringify!($name))?;
                $(
                    write!(f, "{}: {:?}, ", stringify!($field), self.$field)?;
                )*
                write!(f, "}}")
            }
        }
    };
}

// 用宏一键生成结构体 + 构造函数 + Debug
define_struct!(Point { x: f64, y: f64 });
define_struct!(Person { name: String, age: u32, city: String });

fn demo_code_gen() {
    println!("\n--- 代码生成 (消除样板) ---");

    let p = Point::new(3.0, 4.0);
    println!("Point: {:?}", p);

    let person = Person::new("张三".to_string(), 25, "北京".to_string());
    println!("Person: {:?}", person);
}

// ===== 5. 递归宏 =====

/// 宏可以递归调用自己, 用于处理不定长参数。
/// 注意: 需要两个分支 — 递归终止条件 + 递归步。
macro_rules! sum {
    // 终止条件: 只有一个参数
    ($x:expr) => {
        $x
    };
    // 递归步: 第一个 + 剩余的和
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

fn demo_recursive() {
    println!("\n--- 递归宏 ---");

    println!("sum!(1)        = {}", sum!(1));
    println!("sum!(1, 2)     = {}", sum!(1, 2));
    println!("sum!(1, 2, 3)  = {}", sum!(1, 2, 3));
    println!("sum!(1,2,3,4)  = {}", sum!(1, 2, 3, 4));
}

// ===== 6. tt 捕获: 最灵活的匹配 =====

/// tt (Token Tree) 可以匹配任意 token 序列, 包括括号配对。
/// 常用于 DSL (领域特定语言) 风格的宏。
macro_rules! html {
    // 捕获标签名 + 内容 (内容可以是任意 token 树)
    ($tag:ident { $($content:tt)* }) => {
        {
            print!("<{}>", stringify!($tag));
            html!(@inner $($content)*);
            println!("</{}>", stringify!($tag));
        }
    };
    // 内部辅助: 处理纯文本
    (@inner $text:literal) => {
        print!("{}", $text);
    };
    // 内部辅助: 递归处理嵌套标签 (tt 可匹配另一个 html! 调用展开的结果)
    (@inner $($t:tt)*) => {
        // 这里简化处理, 实际项目建议用专门的模板引擎
        print!("{}", stringify!($($t)*));
    };
}

fn demo_tt_capture() {
    println!("\n--- tt 捕获 (DSL 风格) ---");
    println!("(tt 宏展示概念, 实际渲染略)");
    println!("宏定义:");
    println!("  html!(div {{ \"Hello\" }})");
    println!("  展开为: print!(\"<div>Hello</div>\\n\")");

    // 实际调用一次, 消除 unused 警告
    html!(div { "Hello from tt macro" });
}

// ===== 7. 宏的常见陷阱 =====

fn demo_pitfalls() {
    println!("\n--- 宏的常见陷阱 ---");

    println!("  1. 宏卫生 (Hygiene):");
    println!("     宏内定义的变量默认不会污染外部作用域");
    println!("     但宏内使用的标识符从宏定义处解析, 非调用处");
    println!();
    println!("  2. 调用位置的类型推断:");
    println!("     宏在调用点展开, 类型检查在展开后进行");
    println!("     错误信息指向展开后的代码, 可能很难读");
    println!("     调试技巧: cargo expand 查看宏展开结果");
    println!();
    println!("  3. 声明宏不能做的事:");
    println!("     - 不能操作宏调用处的局部变量");
    println!("     - 不能获取调用处的类型信息");
    println!("     - 不能实现 #[derive] (那是过程宏的领域)");
    println!();
    println!("  4. 何时用声明宏 vs 过程宏:");
    println!("     声明宏: 简单模式匹配 + 文本替换就够用");
    println!("     过程宏: 需要操作类型信息、修改 AST、实现 derive");
}

pub fn run() {
    demo_fragment_types();
    demo_repetition();
    demo_multi_pattern();
    demo_code_gen();
    demo_recursive();
    demo_tt_capture();
    demo_pitfalls();
}
