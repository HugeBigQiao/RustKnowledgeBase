//! 特型(Trait): 定义"共享行为"的接口，类似其他语言的 interface。
//!
//! 前置依赖: basic/ 中的 struct; intermediate/ 中的 structs_and_enums、generics.

// ── Trait 定义 ──

/// 摘要特征: 任何实现了 Summary 的类型, 都必须提供 summarize 方法.
trait Summary {
    // 方法签名(无默认实现): 实现者必须提供.
    fn summarize_author(&self) -> String;

    // 带默认实现的方法: 实现者可以覆盖, 也可以直接用默认的.
    fn summarize(&self) -> String {
        // 默认实现中调用了另一个 trait 方法.
        format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
    }
}

// ── 类型定义 ──

struct NewsArticle {
    headline: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

// ── 为类型实现 Trait ──

// 语法: impl Trait名称 for 类型名称 { ... }
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // 覆盖默认实现
    fn summarize(&self) -> String {
        format!("{} —— {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}(内容: {}字)", self.username, self.content.len())
    }

    // 不覆盖 summarize, 使用默认实现.
}

// ── Trait 作为参数 ──

/// 接受任何实现了 Summary 的类型.
/// impl Trait 语法: 参数类型是"实现了 Summary 的某个类型".
fn notify(item: &impl Summary) {
    println!("[通知] {}", item.summarize());
}

/// Trait Bound 写法(等价于 impl Trait, 但更灵活):
/// 当需要两个参数类型相同时必须用这种写法.
fn notify_same<T: Summary>(a: &T, b: &T) {
    println!("[通知A] {}", a.summarize());
    println!("[通知B] {}", b.summarize());
}

// ── run ──

/// 演示 Trait 定义、实现、默认方法、作为参数。
pub fn run() {
    // ===== 定义与实现 =====
    println!("===== Trait 定义与实现 =====");

    let article = NewsArticle {
        headline: String::from("Rust 1.80 发布"),
        author: String::from("Rust 团队"),
    };

    let tweet = Tweet {
        username: String::from("ferris"),
        content: String::from("我爱 Rust!"),
    };

    // 调用 trait 方法
    println!("文章: {}", article.summarize());
    println!("推文: {}", tweet.summarize());
    // tweet 使用了 Summary 的默认 summarize 实现.

    // ===== Trait 作为参数 =====
    println!("\n===== Trait 作为参数 =====");

    // notify 接受任何 impl Summary 的类型.
    notify(&article);
    notify(&tweet);

    // notify_same 要求两个参数类型相同.
    println!("\n--- notify_same(同类型) ---");
    let article2 = NewsArticle {
        headline: String::from("异步 Rust 入门"),
        author: String::from("社区"),
    };
    notify_same(&article, &article2);
    // notify_same(&article, &tweet);  // 报错: 类型不同

    // ===== Derive 宏 =====
    println!("\n===== 标准库 Derive Trait =====");
    println!("#[derive(Debug)]    : 自动实现 Debug, 支持 {{:?}} 打印.");
    println!("#[derive(Clone)]    : 自动实现 Clone, 支持 .clone().");
    println!("#[derive(PartialEq)]: 自动实现相等比较, 支持 == 和 !=.");
    println!("#[derive(Copy)]     : 自动实现 Copy(需所有字段都是 Copy).");
    println!();
    println!("这些 trait 是标准库提供的, derive 宏帮我们自动生成样板代码.");

    // ===== Trait 核心理解 =====
    println!("===== Trait 核心理解 =====");
    println!("Trait = 共享行为的契约.");
    println!("impl Trait for Type = 为特定类型实现这个契约.");
    println!("&impl Trait / T: Trait = 接受任何满足契约的类型.");
}
