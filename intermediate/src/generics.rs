//! 泛型(Generics): 一份代码处理多种类型。
//!
//! ## 什么是泛型
//!
//! 泛型是一种"类型占位"机制。写代码时用 T、K、V 这些占位符代替具体类型,
//! 编译器在编译时根据实际使用情况, 自动为每种具体类型生成一份专用代码。
//!
//! 类比: 泛型就像函数参数, 不过参数传的是"值", 泛型传的是"类型"。
//!   fn add(a: i32, b: i32) → i32     ← 参数 a,b 是"值"的占位
//!   fn add<T>(a: T, b: T) → T        ← 参数 T 是"类型"的占位
//!
//! ## 为什么需要泛型
//!
//! 没有泛型时, 想写一个"找出最大值"的函数, 你得为 i32、char、f64...
//! 每种类型各写一份, 逻辑完全一样只有类型不同 — 纯体力活。
//! 泛型让你写一份逻辑, 编译器帮你适配所有类型。
//!
//! ## 泛型可以用在哪里
//!
//!   函数:  fn largest<T>(list: &[T]) -> &T
//!   结构体: struct Point<T> { x: T, y: T }
//!   枚举:   enum Option<T> { Some(T), None }   ← 你已经用过了
//!   方法:   impl<T> Point<T> { fn x(&self) -> &T }
//!
//! 前置依赖: basic/ 中的 基础类型、Vec、函数; intermediate/ 中的 structs_and_enums。

// ═══════════════════════════════════════════════════════════════
// 第 1 节: 没有泛型 vs 有泛型 — 感受差别
// ═══════════════════════════════════════════════════════════════
//
// 任务: 写一个函数, 找出切片里最大的元素。
//
// 没有泛型时, 每种类型都要写一份:

/// 所有权: 参数 &[i32] 借切片, 返回 &i32 借自参数 → 返回值生命周期 ≤ 参数。
/// 调用方不需要交出所有权, 也不能在函数返回后释放原数据。
#[allow(dead_code)]
fn largest_i32(list: &[i32]) -> &i32 {              // 为 i32 写的
    let mut max = &list[0];                          // max: &i32, 借 list[0]
    for item in list {                               // item: &i32, 遍历时每次借一个元素
        if item > max { max = item; }                // 比较两个引用, 更新 max 指向更大者
    }
    max                                              // 返回 &i32 — 借自参数, 不创建新数据
}

/// 所有权同 largest_i32: 借入 &[char], 返回 &char。
#[allow(dead_code)]
fn largest_char(list: &[char]) -> &char {            // 为 char 写的......和上面一模一样!
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}
// 如果想支持 f64、String、自定义类型......每加一种就复制粘贴一次。
// 所有版本的逻辑完全相同, 只有类型签名不一样 — 这是典型的"代码重复"。

/// 有泛型后: 一份代码覆盖所有可比较的类型。
/// T: PartialOrd 叫"trait 约束"(trait bound):
///   "T 可以是任何类型, 但必须支持 >、< 等比较操作"。
/// 编译器看到 largest::<i32> 时, 自动生成一份 largest_i32 的代码;
/// 看到 largest::<char> 时, 自动生成一份 largest_char 的代码。
/// 这叫"单态化"(monomorphization) — 零运行时开销。
/// 所有权: &[T] 借切片, 返回 &T 借自参数 — 泛型版本, 所有权语义和 largest_i32 完全一致。
/// 泛型不影响所有权规则: 借进来, 借出去, 不创建也不消耗数据。
fn largest<T: PartialOrd>(list: &[T]) -> &T {       // T 是类型占位符, 编译时替换为具体类型
    let mut max = &list[0];                          // &T: 借 list[0]
    for item in list {                               // item: &T
        if item > max { max = item; }                // 要求 T 支持 >, 所以需要 PartialOrd
    }
    max                                              // 返回 &T — 借自参数
}

// ── 说明: 常见的 trait 约束 ──
//
//   T: PartialOrd    → 可以用 >、<、>=、<=
//   T: Clone         → 可以用 .clone()
//   T: Copy          → 赋值是复制而非 move
//   T: Debug         → 可以用 {:?} 打印
//   T: PartialOrd + Clone  → 多个约束用 + 连接
//
// trait 的完整讲解在 traits.rs, 这里先把约束当"类型必须满足的条件"理解就行。

// ── 语法拆解: <T: 约束> 到底怎么写 ──
//
// 以 largest 为例, 逐段拆开:
//
//   fn largest<T: PartialOrd>(list: &[T]) -> &T
//   ~~        ~  ~~~~~~~~~~~  ~~~~~~~~~~~  ~~~
//   关键字     ^      ^            ^        ^
//   函数名 ───┘      │            │        │
//   <T> 声明泛型参数 ─┘            │        │
//   : PartialOrd 约束 T 必须可比较 ─┘        │
//   (list: &[T]) 参数里可以使用 T ──────────┘
//   -> &T 返回值里也可以使用 T ──────────────┘
//
// 四种常见写法:
//
//   fn f<T>(x: T)                     ← 无约束: T 可以是任何类型
//   fn f<T: Clone>(x: T)              ← 单一约束: T 必须能 Clone
//   fn f<T: Clone + Debug>(x: T)      ← 多约束用 + 连接
//   fn f<T>(x: T) where T: Clone+Debug ← 约束多时挪到后面 (where 子句)
//
// <T> 必须紧跟在函数名后面, 不能放在别处。
// 调用时一般不需要写 <i32>: largest(&numbers) 就够了, 编译器自动推断。


// ═══════════════════════════════════════════════════════════════
// 第 2 节: 泛型可以放在哪里
// ═══════════════════════════════════════════════════════════════
//
// 泛型几乎可以出现在任何需要写类型的地方:

// ── 2.1 泛型函数 ──
// 语法: fn 函数名<类型参数>(参数) → 返回类型

/// 交换两个值 — 任意类型都行, 因为只需要赋值, 不需要其他能力。
/// 所有权: a, b 的所有权移入函数 → 返回新元组, 所有权移给调用方。
///   对于 Copy 类型 (i32 等): 赋值 = 复制, 原来的变量仍可用。
///   对于非 Copy 类型 (String 等): 赋值 = move, 原变量失效。
fn swap<T>(a: T, b: T) -> (T, T) {                  // T 不需要任何 trait 约束
    (b, a)                                           // 就是简单的位置交换
}

/// 重复三次 — T 必须能 Clone, 因为要复制出三份。
/// 所有权: 参数 &T — 借用, 不获取 item 的所有权。
///   item.clone() 创建新的 T (独立所有权), 放进 Vec。
///   调用后 item 仍在, 不受影响。
fn triple<T: Clone>(item: &T) -> Vec<T> {            // Clone: 需要 .clone() 来复制
    vec![item.clone(), item.clone(), item.clone()]   // 三次 clone, 创建三个独立值
}

// ── 2.2 泛型结构体 ──
// 语法: struct 名称<类型参数> { 字段: 类型参数 }

/// 二维坐标 — 单类型参数 (x 和 y 必须是同一种类型)。
#[derive(Debug)]
struct Point<T> {                                    // <T> 紧跟在结构体名后面
    x: T,
    y: T,
}                                                    // Point<i32> → x,y 都是 i32

/// 键值对 — 双类型参数 (K 和 V 可以不同)。
#[allow(dead_code)]
#[derive(Debug)]
struct Pair<K, V> {                                  // 两个参数: K 和 V 各自独立
    key: K,
    value: V,
}                                                    // Pair<&str, i32> → key=&str, value=i32

/// 成绩单 — 三类型参数 (实际项目中不罕见)。
#[allow(dead_code)]
#[derive(Debug)]
struct ScoreRecord<N, S, G> {                        // 参数名可以任意, 单字母大写是惯例
    name: N,                                         // 姓名 — 可能是 String 或 &str
    score: S,                                        // 分数 — 可能是 i32 或 f64
    grade: G,                                        // 等级 — 可能是 char 或自定义枚举
}

// ── 单参数 vs 多参数的关键区别 ──
//
// Point<T>:        只有一个 T,  x 和 y 必须是同一种类型
//   Point { x: 3, y: 7 }     ✓ 都是 i32
//   Point { x: 3, y: 1.5 }   ✗ i32 和 f64 混了, 编译报错
//
// Pair<K, V>:      两个参数, K 和 V 各自独立, 可以是不同类型
//   Pair { key: "age", value: 18 }    ✓ K=&str, V=i32 (可以不同)
//   Pair { key: "x", value: "y" }     ✓ K=&str, V=&str (也可以相同)
//
// ScoreRecord<N,S,G>:  三个参数, 三者各不相干
//   ScoreRecord { name: "张三", score: 95, grade: 'A' }  ✓ N=&str, S=i32, G=char
//
// 规则一句话: 同名参数必须同类型, 不同名参数各自独立。

// ── 2.3 泛型枚举 ──
// 你已经用过了! Option<T> 和 Result<T, E> 本质就是标准库定义的泛型枚举:
//
//   enum Option<T> { Some(T), None }
//   enum Result<T, E> { Ok(T), Err(E) }
//
// 自定义泛型枚举也一样:

#[allow(dead_code)]
enum Either<L, R> {                                  // "要么是左边, 要么是右边"
    Left(L),
    Right(R),
}                                                    // Either<String, i32> → Left("hi") 或 Right(42)

// ── 2.4 泛型方法 (impl 块) ──
// 语法: impl<类型参数> 类型名<类型参数> { ... }

impl<T> Point<T> {                                   // impl<T> 声明"这个块里 T 是泛型参数"
    /// 所有权: x, y 的所有权移入 Point — 构造后 x,y 变量不能再单独使用。
    fn new(x: T, y: T) -> Self {                     // 关联函数 — 构造器
        Point { x, y }
    }

    /// 所有权: &self 借用 → 返回 &T 借自 self 内部字段。
    fn x(&self) -> &T { &self.x }                    // 返回 &T (泛型引用)
    fn y(&self) -> &T { &self.y }
}

// 可以只为"满足特定约束的 T"额外添加方法:
impl<T: PartialOrd> Point<T> {                       // 只有当 T 支持比较时, 才有这个方法
    /// 所有权: &self 借用, 只读比较字段, 不修改也不消耗。
    fn x_is_bigger(&self) -> bool {
        self.x > self.y                              // 需要 T: PartialOrd
    }
}

impl<K, V> Pair<K, V> {                               // impl<K,V>: 声明"这个块里 K,V 是泛型"
    /// 所有权: key, value 的所有权移入 Pair — 同 Point::new。
    fn new(key: K, value: V) -> Self {               // 构造器: 参数类型和 struct 的类型参数一致
        Pair { key, value }
    }
}


// ═══════════════════════════════════════════════════════════════
// 第 3 节: where 子句 — 当约束多到写不下时
// ═══════════════════════════════════════════════════════════════
//
// 当类型参数多、约束复杂时, <T: A + B + C> 会变得很难读。
// where 把约束挪到后面, 每行一个, 干净清晰。
//
//   难读: fn foo<T: Clone + Debug, K: PartialEq + Debug>(a: T, b: K) { ... }
//   清晰: fn foo<T, K>(a: T, b: K)
//          where T: Clone + Debug,
//                K: PartialEq + Debug
//          { ... }

/// 用 where 子句约束泛型 — 多个参数多个条件时用这种写法最易读。
/// 所有权: &T, &U 都是借用; t.clone() 创建新的 T (独立所有权), 返回给调用方。
#[allow(dead_code)]
fn debug_and_clone<T, U>(t: &T, u: &U) -> T
where
    T: Clone + std::fmt::Debug,                      // T 要能 Clone 且能 Debug 打印
    U: std::fmt::Debug,                              // U 只要能 Debug 打印就行
{
    println!("t = {:?}, u = {:?}", t, u);            // 需要 T: Debug, U: Debug
    t.clone()                                        // 需要 T: Clone → 创建独立副本, 所有权移给调用方
}


// ═══════════════════════════════════════════════════════════════
// 第 4 节: 什么时候用泛型 — 场景清单
// ═══════════════════════════════════════════════════════════════
//
// 问自己一个问题: "这份逻辑, 换个类型还能不能用?"
// 如果能 → 写泛型。以下是最常见的三类场景:
//
// 1. 算法/工具函数 — 排序、查找、最大/最小值, 跟元素类型无关
// 2. 容器/数据结构 — 栈、队列、树、缓存, 跟存储的数据类型无关
// 3. 自定义类型 — 坐标、配置、键值对, 字段类型不固定时


// ═══════════════════════════════════════════════════════════════
// run — 按节演示
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    // ===== 第 1 节: 没有泛型 vs 有泛型 =====
    println!("===== 第 1 节: 没有泛型 vs 有泛型 =====");

    let numbers = vec![3, 7, 2, 9, 5];               // numbers 拥有 Vec<i32>
    let chars = vec!['z', 'a', 'm', 'b'];            // chars 拥有 Vec<char>

    // 不用泛型: 各自调各自的
    println!("不用泛型:");
    // &numbers: 借 Vec → 返回 &i32 (借自 numbers)。numbers 仍可用。
    println!("  largest_i32  → {}", largest_i32(&numbers));  // 专门给 i32 的函数
    println!("  largest_char → {}", largest_char(&chars));   // 专门给 char 的函数

    // 用泛型: 同一个函数
    println!("\n用泛型 (同一个 largest 函数):");
    // 所有权: 和上面一样, &numbers/&chars 只是借用。
    println!("  largest<i32>  → {}", largest(&numbers));     // 编译器自动推成 largest::<i32>
    println!("  largest<char> → {}", largest(&chars));       // 编译器自动推成 largest::<char>

    println!("\n结论: {} 行重复代码 → {} 行泛型代码, 覆盖所有类型。",
        "6×N", "6×1");

    // ===== 第 2 节: 泛型可以放在哪里 =====
    println!("\n===== 第 2 节: 泛型的放置位置 =====");

    // 泛型函数
    println!("--- 泛型函数 ---");
    // swap(1, 2): i32 是 Copy, 值被复制进函数, 返回的元组是全新的。
    let (a, b) = swap(1, 2);
    println!("swap(1, 2) = ({}, {})", a, b);
    // swap("你好", "世界"): &str 是 Copy (胖指针复制), 不影响原数据。
    let (x, y) = swap("你好", "世界");
    println!("swap(\"你好\", \"世界\") = (\"{}\", \"{}\")", x, y);

    // triple(&'R'): &char 借用, triple 内部 clone 三次 → 创建三个新 char。
    println!("triple(&'R') = {:?}", triple(&'R'));          // 需要 Clone

    // 泛型结构体
    println!("\n--- 泛型结构体 ---");
    let pi = Point { x: 3, y: 7 };                         // T = i32 (Copy) → pi 拥有 Point
    println!("Point<i32>: ({}, {})", pi.x, pi.y);          // .x 返回 &i32, 借 pi

    let pf = Point { x: 1.5, y: 2.8 };                     // T = f64 → pf 拥有 Point
    println!("Point<f64>: ({}, {})", pf.x, pf.y);

    // Point<T> 要求 x 和 y 同类型, 混着写会报错:
    // let bad = Point { x: 3, y: 1.5 };                   // ❌ x 推断 i32, y 推断 f64, 不一致

    let kv: Pair<&str, i32> = Pair { key: "年龄", value: 18 };
    println!("Pair<&str, i32>: {:?}", kv);

    // 泛型方法
    println!("\n--- 泛型方法 ---");
    // Point::new(10, 20): i32 Copy, 复制进 Point。Point 的所有权从 new 移给 p1。
    let p1 = Point::new(10, 20);                            // T = i32
    println!("Point::new(10, 20): x={}, y={}", p1.x(), p1.y()); // x(),y(): &self 借用

    // Point::new(5, 3): 同上, p2 拥有 Point。x_is_bigger() 借 &self。
    let p2 = Point::new(5, 3);
    println!("p2.x_is_bigger() = {} (需要 T: PartialOrd)", p2.x_is_bigger());

    // Pair::new: key, value (&str Copy) 复制进 Pair。pkv 拥有 Pair。
    let pkv = Pair::new("语言", "Rust");
    println!("Pair::new: {:?}", pkv);

    // ===== 第 3 节: where 子句 =====
    println!("\n===== 第 3 节: where 子句 =====");
    println!("当约束多、参数多时, where 比 <T: A+B+C> 更容易读。");
    println!("示例见 debug_and_clone 函数 (编译过了, 这里不运行)。");
    println!("对比:");
    println!("  臃肿: fn f<T: Clone+Debug, U: Debug>(...");
    println!("  清爽: fn f<T, U>(...) where T: Clone+Debug, U: Debug");

    // ===== 第 4 节: 什么时候用 =====
    println!("\n===== 第 4 节: 什么时候用泛型 =====");
    println!("问自己: \"这份逻辑, 换个类型还能不能用?\"");
    println!("  ✓ 算法/工具 → 排序、查找、最大最小值");
    println!("  ✓ 容器/结构 → Vec<T>、HashMap<K,V>、Option<T>");
    println!("  ✓ 自定义类型 → 坐标 Pair、缓存 Cache<T>、结果包装器");
    println!("单态化保证: 泛型在编译期展开成具体代码, 运行时零开销。");
}
