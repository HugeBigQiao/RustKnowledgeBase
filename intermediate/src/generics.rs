//! 泛型(Generics): 用同一个代码处理多种类型，避免重复。
//!
//! 前置依赖: basic/ 中的 基础类型、Vec; intermediate/ 中的 structs_and_enums.

// ── 泛型函数 ──

/// 找出切片中最大的值(泛型版).
/// T: PartialOrd 表示 T 必须支持比较(>、< 等).
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// ── 泛型结构体 ──

/// 二维坐标(单类型参数)
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/// 键值对(双类型参数)
#[allow(dead_code)]
#[derive(Debug)]
struct Pair<K, V> {
    key: K,
    value: V,
}

// ── 泛型方法 ──

impl<T> Point<T> {
    // 关联函数(构造器)
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // &self 方法
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &T { &self.y }
}

// 只为特定 T 实现方法: 当 T 支持比较时, 增加 x_is_bigger.
impl<T: PartialOrd> Point<T> {
    fn x_is_bigger(&self) -> bool {
        self.x > self.y
    }
}

impl<K, V> Pair<K, V> {
    fn new(key: K, value: V) -> Self {
        Pair { key, value }
    }
}

// ── run ──

/// 演示泛型函数、泛型结构体、泛型方法。
pub fn run() {
    // ===== 泛型函数 =====
    println!("===== 泛型函数 =====");
    // 同一个 largest 函数, 可以用于不同具体类型.
    let numbers = vec![3, 7, 2, 9, 5];
    println!("最大数字: {}", largest(&numbers));

    let chars = vec!['z', 'a', 'm', 'b'];
    println!("最大字符: {}", largest(&chars));

    // Option<T> 和 Result<T, E> 也是泛型, 你已经用过了.
    println!("\nOption<T> 和 Result<T, E> 本质就是泛型枚举.");

    // ===== 泛型结构体 =====
    println!("\n===== 泛型结构体 =====");

    // Point<T>: T 是同一个类型
    let pi = Point { x: 3, y: 7 };
    println!("pi  ({}, {})", pi.x, pi.y);

    let pf = Point { x: 1.5, y: 2.8 };
    println!("pf  ({}, {})", pf.x, pf.y);

    // 不能混合类型:
    // let p = Point { x: 3, y: 1.5 };  // 报错: x 是 i32, y 是 f64

    // Pair<K, V>: K 和 V 可以是不同类型
    let kv = Pair { key: "年龄", value: 18 };
    println!("Pair: {:?}", kv);

    // ===== 泛型方法 =====
    println!("\n===== 泛型方法 =====");
    let p1 = Point::new(10, 20);
    println!("p1.x() = {}, p1.y() = {}", p1.x(), p1.y());

    let p2 = Point::new(5, 3);
    println!("p2.x_is_bigger() = {}", p2.x_is_bigger());

    let kv2 = Pair::new("name", "Alice");
    println!("Pair::new: {:?}", kv2);

    // ===== 泛型优势总结 =====
    println!("\n===== 泛型优势 =====");
    println!("1. 避免重复: 一份 largest 代码, 适用于所有可比较的类型.");
    println!("2. 类型安全: 编译器为每种具体类型生成专用代码(单态化).");
    println!("3. 零运行时开销: 泛型在编译期展开, 不影响性能.");
}
