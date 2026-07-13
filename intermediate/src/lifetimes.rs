//! 生命周期(Lifetime): 标注引用的有效范围，让借用检查器验证安全。
//!
//! 生命周期不改变引用的存活时间, 只是给编译器提供信息,
//! 让它确认"返回的引用不会比被借用的值活得更久"。
//!
//! 前置依赖: basic/ 中的 ownership_and_refs; intermediate/ 中的 structs_and_enums.

// ── 生命周期标注语法 ──
// 格式: &'a T  读作"生命周期 a 的 T 引用"
// 标注写在尖括号中: fn foo<'a>(x: &'a str) -> &'a str

/// 返回两个字符串切片中较长的一个。
/// 'a 表示: 返回的引用和参数引用有相同的生命周期。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// 总是返回第一个参数(不需要标注第二个参数的生命周期)。
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// ── 带生命周期的结构体 ──

/// 摘录: 持有外部文本的引用, 所以需要生命周期标注.
#[derive(Debug)]
struct Excerpt<'a> {
    // 这个引用不能比结构体本身活得更久.
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(part: &'a str) -> Self {
        Excerpt { part }
    }

    // 根据省略规则, 这里可以省略生命周期标注.
    fn get(&self) -> &str {
        self.part
    }
}

// ── 生命周期省略规则 ──
// Rust 有三条省略规则, 满足条件时编译器自动推断:
// 1. 每个引用参数获得独立的生命周期.
// 2. 如果只有一个输入生命周期, 它被赋给所有输出.
// 3. 如果有 &self/&mut self, 它的生命周期赋给所有输出.

/// 可以省略标注: 只有 &self 一个输入, 规则3适用.
/// 等价于: fn announce_and_get<'a>(&'a self, msg: &str) -> &'a str
impl<'a> Excerpt<'a> {
    fn announce_and_get(&self, msg: &str) -> &str {
        println!("{}", msg);
        self.part
    }
}

// ── run ──

/// 演示生命周期标注语法、省略规则、结构体中的生命周期。
pub fn run() {
    // ===== 为什么需要生命周期 =====
    println!("===== 为什么需要生命周期 =====");
    // 借用检查器需要确保: 任何引用都不会比它指向的值活得更久.
    // 当函数返回一个引用时, 编译器必须知道这个引用来自哪个参数.
    println!("当返回引用时, 编译器需要知道它和哪个参数共享生命周期.");

    // ===== 函数中的生命周期 =====
    println!("\n===== 函数生命周期标注 =====");

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = longest(&s1, &s2);
    println!("longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);
    // result 的生命周期 = s1 和 s2 中较短的那个.

    // 展示生命周期约束:
    {
        let s3 = String::from("hi");
        let r = longest(&s1, &s3);
        println!("longest(\"{}\", \"{}\") = \"{}\"", s1, s3, r);
        // r 不能比 s3 活得更久, 所以不能在外层作用域用 r.
    }
    // println!("{}", r);  // 报错: s3 已经离开作用域.

    let r2 = first(&s1, &s2);
    println!("first  = \"{}\"", r2);

    // ===== 结构体中的生命周期 =====
    println!("\n===== 结构体中的生命周期 =====");
    let text = String::from("Rust 是一门系统编程语言, 安全且高效.");
    let excerpt = Excerpt::new(&text[..20]);

    println!("摘录: {:?}", excerpt);
    println!("get(): {}", excerpt.get());
    excerpt.announce_and_get("读取摘录...");

    // excerpt 不能比 text 活得更久:
    // drop(text);  // 如果在这里释放 text, excerpt 就悬垂了.
    println!("text 存活中, excerpt 安全.");

    // ===== 静态生命周期 =====
    println!("\n===== 'static 生命周期 =====");
    // 'static: 存活于整个程序运行期间.
    // 字符串字面量天然是 &'static str:
    let literal: &'static str = "这是一个静态字符串";
    println!("static 字面量: {}", literal);

    // ===== 核心理解 =====
    println!("\n===== 生命周期核心理解 =====");
    println!("生命周期标注 = 告诉编译器引用之间的关系.");
    println!("它不延长任何东西的存活时间, 只是帮编译器做静态检查.");
    println!("省略规则让大部分简单场景不需要手动标注.");
    println!("只有当编译器无法推断时才需要显式写出来.");
}
