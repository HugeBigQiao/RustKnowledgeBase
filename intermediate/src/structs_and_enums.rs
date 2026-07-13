//! 结构体(struct) 与 枚举(enum)：Rust 两种核心自定义类型。
//!
//! ## 核心区别
//!
//! | 特性 | struct | enum |
//! |---|---|---|
//! | 含义 | "同时有这些字段" | "这些变体中选一个" |
//! | 字段 | 所有字段同时存在 | 每个变体可以有不同类型的数据 |
//! | 典型场景 | 学生信息、坐标、配置 | 方向、结果(成功/失败)、消息类型 |
//! | 内存 | 所有字段的值拼在一起 | 大小 = 最大变体 + 标签 |
//!
//! 前置依赖: basic/ 中的 基础类型、所有权、match、函数。

// ── 结构体定义 ──

/// 学生(具名字段结构体)
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    score: i32,
}

/// 颜色 RGB(元组结构体: 字段没名字, 用 .0 .1 .2 访问)
#[derive(Debug)]
struct Color(i32, i32, i32);

/// 标记(单元结构体: 没有字段)
struct ReadOnly;

// ── 结构体方法 ──

impl Student {
    fn new(name: &str, age: i32, score: i32) -> Self {
        Student { name: String::from(name), age, score }
    }

    // &self: 不可变借用, 只读
    fn is_pass(&self) -> bool { self.score >= 60 }

    fn intro(&self) -> String {
        format!("我叫{}, {}岁, {}分", self.name, self.age, self.score)
    }

    // &mut self: 可变借用, 可修改字段
    fn add_score(&mut self, delta: i32) { self.score += delta; }
}

// ── 枚举定义 ──

/// 简单枚举(变体不携带数据)
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Direction { North, South, East, West }

/// 带数据的枚举: 每个变体可以携带不同类型的载荷(payload)
enum WebEvent {
    Click { x: i32, y: i32 },  // 带命名字段(像结构体)
    KeyPress(char),             // 带一个值(像元组结构体)
    Resize(i32, i32),           // 带两个值
    Quit,                       // 无数据
}

// ── 枚举方法 ──

impl WebEvent {
    fn describe(&self) -> &str {
        match self {
            WebEvent::Click { .. } => "点击",
            WebEvent::KeyPress(_) => "按键",
            WebEvent::Resize(..) => "调整大小",
            WebEvent::Quit => "退出",
        }
    }
}

// ── run ──

/// 对比演示结构体和枚举的定义、创建、方法。
pub fn run() {
    // ===== 结构体: 创建与使用 =====
    println!("===== 结构体(struct) =====");

    // 1. 具名字段结构体
    let mut p1 = Student { name: String::from("小明"), age: 18, score: 85 };
    println!("p1: {}", p1.intro());

    // mut 可以修改字段
    p1.score = 90;
    println!("  修改后: {:?}", p1);

    // 字段简写(变量名==字段名时可省略冒号)
    let name = String::from("小红");
    let age = 17;
    let p2 = Student { name, age, score: 92 };
    println!("p2: {:?}", p2);

    // 结构体更新语法: 从现有实例复制其余字段
    let p3 = Student { name: String::from("小刚"), ..p1 };
    println!("p3(..p1): {:?}", p3);
    // 注意: ..p1 中非 Copy 字段(如 String)会被 move.

    // 2. 元组结构体
    let red = Color(255, 0, 0);
    println!("\n元组结构体: R={}, G={}, B={}", red.0, red.1, red.2);

    // 3. 单元结构体
    let _ro = ReadOnly;
    println!("单元结构体: 无字段, 大小 0 字节, 用作标记.");

    // ===== 结构体方法 =====
    println!("\n----- 结构体方法 -----");
    let s = Student::new("小李", 16, 78);
    println!("{:?}  及格? {}", s, s.is_pass());
    let mut s2 = Student::new("小王", 15, 55);
    s2.add_score(10);
    println!("加分后: {}分", s2.score);

    // ===== 枚举: 创建与使用 =====
    println!("\n===== 枚举(enum) =====");

    // 简单枚举
    let dir = Direction::North;
    println!("方向: {:?}", dir);
    match dir {
        Direction::North => println!("  往北"),
        Direction::South => println!("  往南"),
        Direction::East  => println!("  往东"),
        Direction::West  => println!("  往西"),
    }

    // 带数据的枚举
    let click = WebEvent::Click { x: 100, y: 200 };
    let key = WebEvent::KeyPress('A');
    let resize = WebEvent::Resize(800, 600);
    let quit = WebEvent::Quit;

    // match 解构变体中的数据
    for event in &[&click, &key, &resize, &quit] {
        handle_event(event);
    }

    // ===== struct vs enum 对比总结 =====
    println!("\n===== struct vs enum 对比 =====");
    println!("struct: \"同时有\" —— 学生 \x1b[1;37m同时有\x1b[0m name + age + score");
    println!("enum:   \"选一个\" —— 事件可能是 Click \x1b[1;37m或\x1b[0m KeyPress \x1b[1;37m或\x1b[0m Quit");
}

fn handle_event(event: &WebEvent) {
    print!("[{}] ", event.describe());
    match event {
        WebEvent::Click { x, y } => println!("位置: ({}, {})", x, y),
        WebEvent::KeyPress(c)    => println!("字符: '{}'", c),
        WebEvent::Resize(w, h)   => println!("尺寸: {}x{}", w, h),
        WebEvent::Quit           => println!("再见!"),
    }
}
