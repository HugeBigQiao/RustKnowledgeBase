//! 特型(Trait): 定义类型的"共同行为"。
//!
//! ## 用继承来理解 trait
//!
//! 如果你写过 Java/C++/Python, 一定熟悉"继承":
//!
//!   class Animal {                        trait Animal {
//!       void speak() { ... }                  fn speak(&self);
//!   }                                     }
//!   class Dog extends Animal {            impl Animal for Dog {
//!       // 自动获得 speak()                    fn speak(&self) { ... }
//!   }                                     }
//!
//!   继承: "Dog 是一个 Animal"              trait: "Dog 实现了 Animal 的行为"
//!   父子层级关系 (is-a)                     能力关系 (can-do)
//!
//! 继承和 trait 都解决同一个问题: 让不同类型共享行为, 一个函数处理多种类型。
//! 但思路完全不同:
//!
//!   继承 — 自上而下设计: 先建父类, 子类继承。子类"天然就"有父类的方法。
//!   trait — 自下而上追加: 先有类型, 再给它贴 trait。类型"后天获得"方法。
//!
//! trait 比继承更灵活的地方:
//!   1. 一个类型可以实现多个 trait — 不像大多数语言只能单继承
//!   2. 可以为外部类型实现你自己的 trait — 比如给 Vec<i32> 加自定义行为
//!   3. trait 可以带默认实现 — 像抽象类, 但不需要"成为"子类
//!
//! 简单记: 继承 = 血缘关系(天生就有), trait = 技能证书(考了才有)。
//!
//! ## trait 和泛型的关系
//!
//! 泛型(generics)说"函数接受任何类型 T"。trait 约束说"T 必须能做什么"。
//! 两者配合: `fn f<T: Summary>(x: &T)` — T 是任意类型, 但必须实现 Summary。
//!
//! 前置依赖: basic/ 中的 struct、函数; intermediate/ 中的 structs_and_enums、generics。


// ═══════════════════════════════════════════════════════════════
// 第 1 节: 没有 trait vs 有 trait — 感受差别
// ═══════════════════════════════════════════════════════════════
//
// 任务: 写一个"通知"函数, 对不同类型的消息生成摘要并打印。
//
// 没有 trait 时, 每种类型各写一个函数:

// 所有权: 两个 struct 的字段都是 String (owned), struct 持有数据的所有权。
// 创建实例时 String 所有权移入 struct; struct 被 drop 时 String 一起释放。
struct NewsArticle {
    headline: String,
    author:    String,
}

struct Tweet {
    username: String,
    content:  String,
}

#[allow(dead_code)]
fn notify_article(article: &NewsArticle) {            // 专门给 NewsArticle 写一个
    println!("[通知] {} —— {}", article.headline, article.author);
}

#[allow(dead_code)]
fn notify_tweet(tweet: &Tweet) {                      // 专门给 Tweet 再写一个...
    println!("[通知] @{}: {}", tweet.username, tweet.content);
}
// 两个函数做的事一模一样 (生成摘要 → 打印), 只因为类型不同就要写两份。
// 如果再加 5 种消息类型, 就再写 5 个函数 — 典型的代码膨胀。

/// 有 trait 后: 把"生成摘要"定义成一种能力, 然后一个函数统一处理。
/// 所有权: 两个方法都是 &self (借用, 只读), 返回 String (新建的, 所有权移给调用方)。
///   返回 String 而非 &str 意味着: 不借 self 的字段, 而是创建新数据交给调用方。
trait Summary {
    // 方法签名 — 实现者必须提供
    fn summarize_author(&self) -> String;             // &self 借用 → 返回 owned String

    // 带默认实现 — 实现者可以覆盖, 也可以直接用
    fn summarize(&self) -> String {                   // &self 借用 → format! 创建新 String
        format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
    }
    // 默认实现里可以调用同一个 trait 的其他方法 (self.summarize_author())
}

// ── 为类型实现 trait ──
// 语法: impl Trait名称 for 类型名称 { 方法实现 }
// for 读作"为": impl Summary for NewsArticle = "为 NewsArticle 实现 Summary"

impl Summary for NewsArticle {
    // 所有权: &self 借用 → clone() 创建 author 的独立副本 → 所有权移给调用方。
    fn summarize_author(&self) -> String {
        self.author.clone()                          // clone 是因为要返回 String (所有权转移)
    }

    // 所有权: &self 借用 → format! 创建新 String → 返回 (不借 self 字段)。
    fn summarize(&self) -> String {                  // 覆盖默认实现: 用"标题 — 作者"格式
        format!("{} —— {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    // 所有权: &self 借用, format! 创建新 String 返回。
    fn summarize_author(&self) -> String {
        format!("@{}(内容: {}字)", self.username, self.content.len())
    }
    // 不写 summarize → 使用 Summary 的默认实现, 行为: "(阅读更多来自 @user(...) 的内容...)"
}

// ── 实际调用 ──
//
// trait 定义完后, 使用方式和普通方法完全一样:
//
//   let article = NewsArticle { ... };
//   let tweet = Tweet { ... };
//
//   article.summarize();          // → "Rust 1.80 发布 —— Rust 团队"  (NewsArticle 自己的实现)
//   article.summarize_author();   // → "Rust 团队"                     (clone 出 author)
//   tweet.summarize();            // → "(阅读更多来自 @ferris(...) 的内容...)"  (默认实现!)
//   tweet.summarize_author();     // → "@ferris(内容: 8字)"            (Tweet 自己的实现)
//
// 同一个 .summarize() 调用, article 和 tweet 的表现不同 — 这就是"多态"。
//
// 还可以把 trait 作为参数传给通用函数:
//
//   fn notify(item: &impl Summary) { println!("[通知] {}", item.summarize()); }
//   notify(&article);  // ✓ NewsArticle 实现了 Summary
//   notify(&tweet);    // ✓ Tweet 实现了 Summary
//
// 不管将来加多少种消息类型, notify 一行都不用改, 只需要为新类型 impl Summary。

// ── trait 到底什么时候值得用? ──
//
// 你的疑问很对: 上面例子只有 2 种类型, trait 方案反而比手写两个函数代码更多。
// 那什么时候 trait 才"划算"? 看以下四个场景:
//
// 场景 1: 类型数量多 (≥3 个时开始回本)
//   ┌─────────────────────────────────────────────────────────────┐
//   │ 没有 trait: 10 种消息 = 10 个 notify_xxx 函数              │
//   │ 有 trait:   10 种消息 = 10 个 impl Summary + 1 个 notify   │
//   │             每个 impl 只需写摘要逻辑, notify 一份就够了      │
//   │ 更重要的是: 哪天要改通知格式, 只改 notify 一处, 不改 10 个  │
//   └─────────────────────────────────────────────────────────────┘
//
// 场景 2: 写库/框架给别人用 (无法预知所有类型)
//   你写了一个日志库, 想接受"任何能转成字符串的数据":
//     fn log(data: &impl Display) { ... }
//   标准库的 i32, String 能 Display; 用户自己的 struct 也能 Display。
//   你不能穷举所有类型 — trait 是"协议": 遵守就能用, 不限制你是谁。
//
// 场景 3: 混合存放不同类型 (trait object)
//   想把 NewsArticle 和 Tweet 放进同一个 Vec:
//     let items: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet)];
//     for item in &items { println!("{}", item.summarize()); }
//   不用 trait 的话, Vec 只能放一种类型 (Vec<NewsArticle>), 做不到混合。
//
// 场景 4: 复用标准库基础设施
//   #[derive(Debug, Clone, PartialEq)] 让你一行代码获得打印/克隆/比较能力。
//   如果没有 trait, 你得为每个 struct 自己实现 println! 和 ==。
//
// 简单结论:
//   只有 2~3 个类型且不会扩展 → 手写函数更省代码, 完全 OK。
//   类型 ≥4 或需要扩展或写库或混合存放 → trait 是正确方案。
//   当前示例用了 2 个类型只是演示, 重在讲语法 — 实际项目里 trait 的收益大得多。

// ── 语法拆解: impl Trait for Type ──
//
//   impl Summary for NewsArticle { ... }
//   ~~~~ ~~~~~~~ ~~~ ~~~~~~~~~~~
//    ^     ^     ^       ^
//   关键字 │     │       └── 目标类型 (谁获得这个能力)
//   trait名 ┘     │
//   for = "为" ───┘
//


// ═══════════════════════════════════════════════════════════════
// 第 2 节: trait 作为参数 — 三种写法
// ═══════════════════════════════════════════════════════════════
//
// 有了 Summary trait 后, 写函数时可以接受"任何实现了 Summary 的类型"。
// 三种写法等价, 适用场景不同:

/// 写法 A: impl Trait — 最简洁, 适合简单参数。
/// "item 的类型是某个实现了 Summary 的类型"。
/// 所有权: &impl Summary — 借用, 不消耗。summarize() 是 &self 方法。
fn notify(item: &impl Summary) {                     // 读作: item 是 "任何 impl Summary 的东西"
    println!("[通知] {}", item.summarize());
}

/// 写法 B: Trait Bound — 更灵活, 适合多个参数需要同类型时。
/// 读作: "对于任意实现了 Summary 的类型 T, 接受两个 &T"。
/// 所有权: &T 借用, summarize() 是 &self 方法 → 不消耗参数。
fn notify_same<T: Summary>(a: &T, b: &T) {          // a 和 b 必须是同一种具体类型
    println!("[通知A] {}", a.summarize());
    println!("[通知B] {}", b.summarize());
}

/// 写法 C: where 子句 — 约束多时最清晰。
///
/// where 是什么? 一个关键字, 把 trait 约束从尖括号里"搬"到函数签名后面。
/// 作用: 把"类型参数声明"和"约束条件"分开写, 不挤在一起。
///
/// 语法:       where 类型参数: trait约束
///             where T: Summary        ← T 必须实现 Summary
///             where T: Summary + Clone ← 多个 trait 用 + 连
///             where T: Summary, U: Summary ← 多个参数用逗号隔开
///
/// 对比:
///   fn f<T: Summary + Clone>(x: &T)            ← 约束少, 直接写尖括号
///   fn f<T>(x: &T) where T: Summary + Clone     ← 约束挪到后面, 同义
///   fn f<T: A+B+C, U: D+E>(x: &T, y: &U)       ← 又长又挤
///   fn f<T, U>(x: &T, y: &U) where T: A+B+C, U: D+E  ← 清爽
fn notify_pair<T, U>(a: &T, b: &U)                   // 两个参数可以不同类型
where
    T: Summary,                                      // T 要能生成摘要
    U: Summary,                                      // U 也要能生成摘要
{
    println!("[通知1] {}", a.summarize());
    println!("[通知2] {}", b.summarize());
}

// ── impl Trait vs Trait Bound 什么时候用哪个? ──
//
//   fn f(x: &impl Summary)           ← 只有一个参数时, 最常见
//   fn f<T: Summary>(x: &T)          ← 同上, 等价
//   fn f<T: Summary>(a: &T, b: &T)   ← 多个参数必须是同类型时, 只能用 Trait Bound
//   fn f(x: &impl Summary, y: &impl Summary) ← 两个参数可以是不同类型


// ═══════════════════════════════════════════════════════════════
// 第 3 节: trait 作为返回值 — impl Trait
// ═══════════════════════════════════════════════════════════════
//
// 函数不仅可以接受 impl Trait, 还可以返回 impl Trait:
// "我返回某个实现了 Summary 的类型, 但不说具体是哪个"。

/// 返回 impl Trait: 调用方只知道"这东西能 summarize()",
/// 不知道具体是哪个类型。所有返回路径必须返回同一种具体类型。
/// 所有权: 函数内部创建 NewsArticle (所有权在函数), 返回时所有权移给调用方。
///   NewsArticle 里的 String 字段也一并转移 — 调用方完全拥有返回值的所有数据。
fn make_summary(breaking: bool) -> impl Summary {     // 返回 "某个 impl Summary 的类型"
    if breaking {
        NewsArticle {
            headline: String::from("突发新闻"),
            author: String::from("编辑部"),
        }
    } else {
        NewsArticle {
            headline: String::from("简讯"),           // 必须和上面同类型!
            author: String::from("快报"),
        }
    }
    // 不能 if 返回 NewsArticle, else 返回 Tweet — 编译器要求单一具体类型。
    // 需要使用不同具体类型时, 用 trait object: Box<dyn Summary> (详见高级阶段)
}


// ═══════════════════════════════════════════════════════════════
// 第 4 节: trait 进阶 — 孤儿规则 & 标准库 trait
// ═══════════════════════════════════════════════════════════════
//
// ── 孤儿规则 (Orphan Rule) ──
// 你不能同时为"外部 trait + 外部类型"写 impl。
//
//   impl Display for Vec<i32>   ✗ 编译报错! Display 和 Vec 都来自标准库
//   impl Summary for Vec<i32>   ✓ Summary 是你定义的 (类型是外部的也可以)
//   impl Display for MyType     ✓ MyType 是你定义的 (trait 是外部的也可以)
//
// 规则: trait 和类型至少有一个是你自己定义的。
// 目的: 防止两个 crate 为同一个组合写了不同的 impl, 产生冲突。

/// 为 Vec<i32> 实现 Summary — 合法! 因为我们定义了 Summary trait。
/// 所有权: &self 借用 Vec<i32>, format! 创建新 String 返回。Vec 本身不动。
impl Summary for Vec<i32> {                          // Vec 是标准库的, Summary 是我们的 → OK
    fn summarize_author(&self) -> String {
        format!("长度为 {} 的向量", self.len())
    }
}

// ── derive 宏 — 自动实现标准库 trait ──
// 你已经用过了: #[derive(Debug, Clone, PartialEq)]
// 这些是编译器帮你自动生成的样板代码, 背后就是 impl Debug for Xxx { ... }
//
// 常用 derive:
//   Debug      → {:?} 打印
//   Clone      → .clone() 复制
//   Copy       → 赋值时复制而非 move (要求所有字段都是 Copy)
//   PartialEq  → == 和 != 比较


// ═══════════════════════════════════════════════════════════════
// run — 按节演示
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    // ===== 第 1 节: 没有 trait vs 有 trait =====
    println!("===== 第 1 节: 没有 trait vs 有 trait =====");

    // article 拥有 NewsArticle — 含两个 String 字段。
    let article = NewsArticle {
        headline: String::from("Rust 1.80 发布"),
        author: String::from("Rust 团队"),
    };

    // tweet 拥有 Tweet — 含两个 String 字段。
    let tweet = Tweet {
        username: String::from("ferris"),
        content: String::from("我爱 Rust!"),
    };

    // 不用 trait: 各自调各自的
    println!("不用 trait:");
    notify_article(&article);                        // &article 借用, article 仍可用
    notify_tweet(&tweet);                            // &tweet 借用

    // 用 trait: 同一个方法, 不同表现
    println!("\n用 trait (各自实现了 summarize):");
    println!("  文章: {}", article.summarize());     // &self 借用, article 仍可用
    println!("  推文: {}", tweet.summarize());        // Tweet 使用默认实现

    // ===== 第 2 节: trait 作为参数 =====
    println!("\n===== 第 2 节: trait 作为参数 =====");

    // 写法 A: impl Trait — 所有权: &T 借用, 不消耗。
    println!("--- 写法 A: impl Trait ---");
    notify(&article);                                // NewsArticle 实现了 Summary → OK
    notify(&tweet);                                  // Tweet 实现了 Summary → OK

    // 写法 B: Trait Bound (两个参数必须同类型)
    println!("\n--- 写法 B: Trait Bound (同类型) ---");
    let article2 = NewsArticle {
        headline: String::from("异步 Rust 入门"),
        author: String::from("社区"),
    };
    notify_same(&article, &article2);                // 两个都是 &NewsArticle → OK
    // notify_same(&article, &tweet);                // ❌ 类型不同!

    // 写法 C: where (两个参数可以不同类型)
    println!("\n--- 写法 C: where (可以不同类型) ---");
    notify_pair(&article, &tweet);                   // &NewsArticle + &Tweet → OK

    // ===== 第 3 节: trait 作为返回值 =====
    println!("\n===== 第 3 节: trait 作为返回值 =====");
    // make_summary 内部创建 NewsArticle, 所有权移给 s。s 拥有返回的结构体。
    let s = make_summary(true);                      // 返回某个 impl Summary 的东西
    println!("make_summary(true) → {}", s.summarize()); // &self 借用
    // s 离开作用域时, NewsArticle 及其 String 字段全部 drop。
    // 注意: 这里不能写成 let s: NewsArticle = ..., 因为返回类型被擦除了。
    // 调用方只知道"它能 summarize()", 不知道具体是 NewsArticle 还是 Tweet。

    // ===== 第 4 节: 进阶 =====
    println!("\n===== 第 4 节: 孤儿规则 & derive =====");
    let v = vec![1, 2, 3, 4, 5];                     // v 拥有 Vec<i32>
    // summarize() 是 &self 借用, v 仍可用。
    println!("Vec<i32>.summarize() → {}", v.summarize());
    // Vec<i32> 是标准库类型, 但 Summary 是我们定义的 → 可以 impl

    println!("\n--- derive 宏 ---");
    println!("#[derive(Debug, Clone, PartialEq)] — 自动为你的类型实现标准库 trait。");
    println!("本质上就是编译器帮你写好了 impl Debug for Xxx {{ ... }}。");

    // ===== 总结 =====
    println!("\n===== 总结 =====");
    println!("trait       = 一组行为约定 (类似 interface)");
    println!("impl X for Y = 为类型 Y 实现 trait X (签契约)");
    println!("&impl Trait  = 接受任何实现了该 trait 的类型 (收契约)");
    println!("孤儿规则     = 不能为别人的类型实现别人的 trait (防冲突)");
}
