//! 结构体(struct) 与 枚举(enum)：Rust 两种核心自定义类型。
//!
//! ## 核心区别
//!
//! | 特性 | struct | enum |
//! |---|---|---|
//! | 含义 | "同时有这些字段" | "这些变体中选一个" |
//! | 字段 | 所有字段同时存在 | 每个变体可以有不同类型的数据 |
//! | 典型场景 | 学生、坐标、配置 | 方向、结果(成功/失败)、消息类型 |
//! | 内存 | 所有字段的值拼在一起 | 大小 = 最大变体 + 标签 |
//!
//! 前置依赖: basic/ 中的 基础类型、所有权、match、函数。


// ═══════════════════════════════════════════════════════════════
// 第 1 节: 结构体 (struct)
// ═══════════════════════════════════════════════════════════════
//
// struct 分三种: 具名字段、元组、单元。

/// 学生 — 具名字段结构体 (最常用)。字段有名字, 通过 . 访问。
///
/// #[derive(Debug)] 是什么意思?
///   #[...]  是 Rust 的属性(attribute), 给编译器发指令。
///   derive  是"自动生成代码"的宏。Debug 让结构体可以用 {:?} 打印, 方便调试。
///   没有它的话 println!("{:?}", s) 会编译报错。
///   常用的 derive 还有 Clone、PartialEq, 详见 advanced/macros.rs。
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    score: i32,
}

/// RGB 颜色 — 元组结构体: 字段没名字, 用 .0 .1 .2 按位置访问。
#[derive(Debug)]
struct Color(i32, i32, i32);

/// 标记 — 单元结构体: 没有字段, 大小 0 字节, 纯用作"标记"。
struct ReadOnly;

// ── 结构体的方法: impl 块 ──
//
// Rust 刻意把数据和行为拆开: struct 定义"存什么", impl 定义"能做什么"。
// 和 C++/Java 把字段+方法写在一个 class 里的思路不同。
//
// 这样做的三个好处:
//   1. 数据和行为解耦 — 改字段不改方法, 改实现不改数据
//   2. 可以有多个 impl 块, 分散到不同文件 — 同一个 struct 在多个地方追加方法
//   3. trait 实现(impl Trait for Type)和自身方法(impl Type)是两个维度, 不混
//
// 以下示例包含三种 self 的用法:

impl Student {
    // 关联函数(没有 self): 用 :: 调用 → Student::new("名字", 18, 85)
    // 参数: name: &str — 借用, 不获取所有权。String::from 创建新 String 并移入 struct。
    fn new(name: &str, age: i32, score: i32) -> Self {  // i32 是 Copy, 直接复制
        Student {
            name: String::from(name),                // String::from 创建新 String, 所有权移入 struct
            age,            // i32: Copy, 复制
            score,          // i32: Copy, 复制
        }
    }

    // &self: 不可变借用, 只读。调法: s.is_pass() — s 必须有效。
    fn is_pass(&self) -> bool {
        self.score >= 60                            // 只读访问, 不修改
    }

    fn intro(&self) -> String {                      // 返回 String: 新建的, 所有权移给调用方
        format!("我叫{}, {}岁, {}分", self.name, self.age, self.score)
    }

    // &mut self: 可变借用, 可修改字段。需要 let mut s; s.add_score(10);
    fn add_score(&mut self, delta: i32) {            // delta: i32 Copy, 复制进函数
        self.score += delta;
    }
}

// ── 关于可见性 (pub) ──
//
// struct 和 impl 里的成员默认都是"本模块私有"。
// 如果要在其他文件/crate 用, 需要加 pub:
//
//   pub struct Student { pub name: String, pub age: i32 }  // 类型公开, 字段也公开
//   impl Student {
//       pub fn new(...) -> Self { ... }   // 公开方法, 外部可调
//       fn helper(&self) { ... }          // 还是私有的, 本模块内调用
//   }
//
// 注意: impl 块本身没有 pub/不 pub 的说法, 可见性由每个方法自己控制。
// 同一个 impl 里可以混着 pub 方法和私有方法。
//
// ── 关于"给外部类型加方法" (孤儿规则) ──
//
// 不能给别人的类型直接加 impl。比如 impl Vec<i32> { fn my_method() } 会报错。
// 但自己 crate 里的 struct, 在任意文件都能加 impl (同一个 crate, 不受限制)。
// 想给第三方类型加新方法, 用 trait 扩展 (详见 traits.rs) 或包装类型 (newtype):

//    // 包装类型示例 (newtype pattern):
//    struct MyVec(Vec<i32>);           // 元组结构体包一层
//    impl MyVec { fn custom(&self) { ... } }  // 给包装加方法


// ═══════════════════════════════════════════════════════════════
// 第 2 节: 枚举 (enum)
// ═══════════════════════════════════════════════════════════════
//
// enum 是"从几个选项中选一个"的类型。每个选项叫"变体"(variant)。

/// 简单枚举 — 变体不携带数据。
/// #[allow(dead_code)]: 告诉编译器"没用到也别报警告", 教学代码常用。
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]     // Copy: 简单枚举可以复制, 方便放进集合
enum Direction {
    North,
    South,
    East,
    West,
}

/// 带数据的枚举 — 每个变体可以携带不同类型的载荷(payload)。
enum WebEvent {
    Click { x: i32, y: i32 },   // 带命名字段 (像 struct)
    KeyPress(char),             // 带一个值 (像元组结构体)
    Resize(i32, i32),           // 带两个值
    Quit,                       // 无数据
}

// ── 枚举也可以有方法, 一样用 impl ──

impl WebEvent {
    fn describe(&self) -> &str {
        match self {
            WebEvent::Click { .. } => "点击",     // .. 忽略不关心的字段
            WebEvent::KeyPress(_) => "按键",       // _ 忽略单个值
            WebEvent::Resize(..) => "调整大小",     // .. 忽略所有字段
            WebEvent::Quit => "退出",
        }
    }
}


// ── 嵌套 struct 字段是 enum ──

/// 订单状态 — 一个 struct 的字段可以是 enum 类型。
/// 这是现实代码里最常见的 struct/enum 组合方式。
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]     // Copy: 可以复制, 放进集合时更方便
enum OrderStatus {
    Pending,   // 待处理
    Shipped,   // 已发货
    Delivered, // 已签收
    Cancelled, // 已取消
}

/// 订单 — status 字段的类型是 OrderStatus (enum)。
/// struct 持有 enum: "订单的状态是这几个中的一个"。
#[derive(Debug)]
struct Order {
    id: u32,
    status: OrderStatus,  // enum 类型的字段, 不是 i32/String
    amount: f64,
}

// ── 嵌套类型在函数中的使用 ──
// 当 struct 字段是 enum 时, 函数的参数和返回值怎么写?
// 和普通类型完全一样 — 类型名照写就行, 没有特殊语法。

/// 参数是包含 enum 字段的 struct → 返回 bool。
/// &Order 表示借用(不拿走所有权), 只读。
fn is_done(order: &Order) -> bool {                                // 参数: 引用, 不消耗
    // matches! 宏: "order.status 是不是这些变体之一?" → 返回 bool
    matches!(order.status, OrderStatus::Delivered | OrderStatus::Cancelled)
}

/// 返回包含 enum 字段的 struct → 返回类型写 struct 类型名。
fn create_order(id: u32, amount: f64) -> Order {                   // 返回: Order (所有权转移给调用方)
    Order { id, status: OrderStatus::Pending, amount }
}


// ═══════════════════════════════════════════════════════════════
// run — 按节演示
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    // ===== 第 1 节: 结构体 =====
    println!("===== 结构体(struct) =====");

    // 1. 具名字段结构体 — 创建
    // name: String — 所有权移入 struct。所有字段都被 struct 拥有。
    let mut p1 = Student {
        name: String::from("小明"),                  // String::from 新建 String, 所有权移入 p1
        age: 18,                                     // i32: Copy, 复制
        score: 85,                                   // i32: Copy, 复制
    };
    println!("p1: {}", p1.intro());

    // mut 允许修改字段 — &mut self 方法才能调用
    p1.score = 90;                                 // 直接修改字段 (i32 Copy, 赋值 = 复制新值)
    println!("  修改后: {:?}", p1);

    // 字段简写: 变量名 == 字段名时省略冒号
    let name = String::from("小红");                 // name 拥有这个 String
    let age = 17;
    let p2 = Student { name, age, score: 92 };     // name 所有权移入 p2! 之后不能再访问 name 变量
    // println!("{}", name);                       // ❌ name 已被 move 进 p2
    println!("p2: {:?}", p2);

    // 结构体更新语法: ..现有实例, 批量复制未指定的字段
    // ⚠ 所有权关键: String 等非 Copy 字段会被 move, 源实例的该字段失效!
    let p3 = Student {
        name: String::from("小刚"),                 // 新字段, 覆盖了 p1.name
        ..p1                                        // 其余字段 (age: i32, score: i32) 从 p1 复制
    };                                              // i32 是 Copy → 复制; 如果 p1.name 没被覆盖, 它会被 move!
    // println!("{:?}", p1);                       // ❌ 如果 p1.name 被 move 了, p1 就不能整体使用了
    println!("p3(..p1): {:?}", p3);                // 但这里 p1.name 被覆盖了, 所以 p1 还能用局部字段

    // 2. 元组结构体 — .0 .1 .2 访问
    let red = Color(255, 0, 0);
    println!("\n元组结构体: R={}, G={}, B={}", red.0, red.1, red.2);

    // 3. 单元结构体 — 无字段, 标记用途
    let _ro = ReadOnly;                             // _ 前缀: 我知道不会用, 别报 warning
    println!("单元结构体: 无字段, 大小 0 字节, 用作标记.");

    // ===== 结构体方法 =====
    println!("\n--- 结构体方法 ---");

    // :: 调用关联函数 (没有 self 的) — 返回新 Student, 所有权交给 s
    let s = Student::new("小李", 16, 78);            // name: &str "小李" 借给 new, new 内部建 String 并移入 s
    println!("{:?}  及格? {}", s, s.is_pass());     // . 调用方法: &self 借用, s 仍可用

    // &mut self: 需要 mut 绑定。可变借用期间, 不能有其他引用。
    let mut s2 = Student::new("小王", 15, 55);
    s2.add_score(10);                               // &mut self → 可变借用, 可改字段
    println!("加分后: {} 分", s2.score);

    // ===== 第 2 节: 枚举 =====
    println!("\n===== 枚举(enum) =====");

    // 简单枚举 + match
    let dir = Direction::North;
    println!("方向: {:?}", dir);
    match dir {
        Direction::North => println!("  往北"),
        Direction::South => println!("  往南"),
        Direction::East => println!("  往东"),
        Direction::West => println!("  往西"),
    }

    // 带数据的枚举 — 创建
    // 每个变体创建的枚举值都是独立的 owned 值。
    let click = WebEvent::Click { x: 100, y: 200 };// x, y: i32 Copy, 复制
    let key = WebEvent::KeyPress('A');              // 'A': char Copy
    let resize = WebEvent::Resize(800, 600);         // i32 Copy
    let quit = WebEvent::Quit;                       // 无数据, 单元变体

    // match 解构变体中的数据 — 这里传引用, 不消耗枚举值
    for event in &[&click, &key, &resize, &quit] {   // &[&WebEvent; 4] — 双层借用
        handle_event(event);                         // event: &&WebEvent → 自动解引用为 &WebEvent
    }

    // ===== struct vs enum 对比 =====
    println!("\n===== struct vs enum 对比 =====");
    println!("struct: \"同时有\" — 学生 \x1b[1;37m同时有\x1b[0m name + age + score");
    println!("enum:   \"选一个\" — 事件可能是 Click \x1b[1;37m或\x1b[0m KeyPress \x1b[1;37m或\x1b[0m Quit");

    // ===== 第 3 节: struct 与 enum 互相嵌套 =====
    println!("\n===== 第 3 节: struct ↔ enum 嵌套 =====");

    // ── 场景 1: struct 字段是 enum (最常见) ──
    // 每个订单"同时有" id + 状态 + 金额; 状态是"选一个"。
    println!("--- 场景 1: struct 字段是 enum ---");

    let o1 = create_order(1, 99.9);                                 // 返回 Order, status=DefaultPending
    println!("新订单: id={}, 状态={:?}, 金额={}", o1.id, o1.status, o1.amount);
    println!("  已完成? {} (状态是 Pending)", is_done(&o1));          // 传 &Order 引用

    let o2 = Order { id: 2, status: OrderStatus::Delivered, amount: 50.0 };
    println!("旧订单: id={}, 状态={:?}, 金额={}", o2.id, o2.status, o2.amount);
    println!("  已完成? {} (状态是 Delivered)", is_done(&o2));

    // 修改 enum 字段: 和修改 i32 一样, struct 需要 mut
    let mut o3 = create_order(3, 30.0);
    println!("\n改前: {:?}", o3.status);
    o3.status = OrderStatus::Shipped;                                // 把字段从 Pending 改成 Shipped
    println!("改后: {:?}", o3.status);

    // ── 场景 2: enum 变体里放 struct（WebEvent 已经是例子, 这里回顾） ──
    println!("\n--- 场景 2: enum 变体里放 struct ---");
    // WebEvent::Click { x, y } → 变体内嵌了匿名 struct
    // patterns.rs 里还有 Shape::Line(Point{...}, Point{...}) → 变体内嵌命名 struct
    println!("WebEvent::Click {{ x: 10, y: 20 }} — 变体里嵌套了匿名结构体");
    println!("patterns.rs 的 Shape::Line(Point, Point) — 变体里嵌套了命名结构体");

    // 函数签名总结:
    println!("\n--- 嵌套类型的函数签名 ---");
    println!("  参数: fn f(order: &Order) {{}}  — struct 包 enum, 照写类型名");
    println!("  返回: fn f() -> Order {{}}       — 同上, 返回值和普通类型一样");
    println!("  访问: order.status               — 用 . 一路点进去");
    println!("  修改: order.status = Shipped      — struct 得是 mut");
    println!("  判断: matches!(order.status, ...) — 一行搞定变体比较");

    // ===== 第 4 节: struct/enum 放进集合 =====
    println!("\n===== 第 4 节: struct/enum 在集合中 =====");

    // ── 1. Vec<结构体> ──
    println!("--- Vec<Student> ---");
    let students = vec![
        Student::new("张三", 15, 85),
        Student::new("李四", 16, 55),
        Student::new("王五", 17, 92),
    ];
    println!("全班 {} 人:", students.len());
    for s in &students {                                            // 遍历 &Vec<Student>
        println!("  {} — {} 分, 及格? {}", s.name, s.score, s.is_pass());
    }

    // ── 2. 按字段查找: find / filter ──
    // 用 Vec 的迭代器方法, 接闭包做条件判断。
    println!("\n--- 按字段查找 ---");

    // find: 返回第一个满足条件的 Option<&Student>
    if let Some(s) = students.iter().find(|s| s.is_pass()) {        // 找第一个及格的
        println!("第一个及格的: {} ({} 分)", s.name, s.score);
    }

    // filter: 过滤出所有满足条件的
    let passed: Vec<_> = students.iter()
        .filter(|s| s.is_pass())                                     // 保留及格的
        .collect();                                                  // 收集回 Vec
    println!("及格人数: {} / {}", passed.len(), students.len());

    // 手写循环等价写法(不用闭包):
    // let mut count = 0;
    // for s in &students { if s.is_pass() { count += 1; } }

    // ── 3. 枚举数组 ──
    println!("\n--- 枚举数组 ---");
    let path = [Direction::North, Direction::East, Direction::South]; // [Direction; 3]
    print!("路径: ");
    for d in &path {                                                // 遍历 &[Direction]
        print!("{:?} → ", d);
    }
    println!("终点");

    // 数组中查找 (用 contains, 因为 Direction 实现了 PartialEq)
    let goes_north = path.contains(&Direction::North);               // 是否包含 North?
    println!("路径往北? {}", goes_north);

    // ── 4. 元组包含 struct/enum ──
    println!("\n--- 元组包含 struct/enum ---");
    let pair: (Student, OrderStatus) = (
        Student::new("赵六", 18, 78),
        OrderStatus::Pending,
    );
    println!("元组: (学生={}, 状态={:?})", pair.0.name, pair.1);     // .0 取 Student, .1 取 OrderStatus

    // ── 5. struct 的字段也可以是集合 ──
    println!("\n--- struct 的字段是集合 ---");
    println!("struct 里可以放 Vec、HashMap 等任何类型。比如:");
    println!("struct Class {{");
    println!("    name: String,");
    println!("    students: Vec<Student>,  // 字段是集合!");
    println!("}}");

    // 解构/匹配: 从集合取出的 struct/enum, 怎么拆?
    println!("\n--- 从集合取出的值怎么解构? ---");
    println!("  详见 patterns.rs 的解构章节。这里提一嘴:");
    println!("  for s in &students {{  match s.score {{  // 遍历 Vec<Student>, match 字段");
    println!("      90..=100 => ..., _ => ...");
    println!("  }} }}");
}

/// 处理 WebEvent — match 根据不同变体提取数据。
/// 参数: &WebEvent — 借用, 调用后原枚举值仍可用。
fn handle_event(event: &WebEvent) {
    print!("[{}] ", event.describe());              // describe 也是 &self 借用, 不消耗
    match event {                                     // event 是 &WebEvent, 解构出的字段也是借的引用
        WebEvent::Click { x, y } => println!("位置: ({}, {})", x, y),  // x, y: &i32 (借的引用)
        WebEvent::KeyPress(c) => println!("字符: '{}'", c),             // c: &char (借的引用)
        WebEvent::Resize(w, h) => println!("尺寸: {}x{}", w, h),        // w, h: &i32
        WebEvent::Quit => println!("再见!"),
    }
}
