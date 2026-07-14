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

//
// 关于 #[derive(Debug)]:
//   #[...]  是 Rust 的属性(attribute), 给编译器或工具发指令.
//   derive  是"自动生成 trait 实现"的宏. Debug trait 让结构体/枚举
//           可以用 {:?} 或 {:#?} 格式打印, 方便调试.
//   如果没有 #[derive(Debug)], println!("{:?}", s) 会编译报错.
//   除了 Debug, 常用的 derive 还有 Clone、PartialEq 等.
//   宏(macro)的详细介绍在 advanced/macros.rs, 这里先会用就行.
//
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

//
// ── 结构体方法 ──
//
// 为什么 struct 和 impl 是分开的?
//
//   和 C++/Java 里"字段和方法写在一个 class 里"不同，Rust 刻意把数据和
//   行为拆成了两部分: struct 定义"存什么数据", impl 定义"能做什么操作"。
//
//   这样做的三个好处:
//
//   1. 数据和行为解耦
//      改字段（比如给 Student 加一个 email）不会碰方法签名；
//      改实现（比如让 is_pass 用新的评分标准）不会破坏数据结构。
//      两者各自独立演化，互不干扰。
//
//   2. 可以多次 impl，分散到多个文件
//      同一个类型可以有多个 impl 块:
//
//        // student.rs
//        impl Student { fn new(...) -> Self { ... } }
//        // student_ext.rs
//        impl Student { fn rank(&self) -> char { ... } }  // 另一个文件继续加方法
//
//      ── 关于可见性(pub) ──
//      struct 和 impl 里的方法默认都是"本模块私有"。
//      如果要在其他文件/其他 crate 里使用，需要加 pub:
//
//        pub struct Student { pub name: String, ... }  // 结构体公开, 字段也公开
//        impl Student {
//            pub fn new(...) -> Self { ... }  // 公开方法, 外部可调用
//            fn secret(&self) { ... }         // 还是私有的, 只有本模块能调
//        }
//
//      ── impl 块本身能不能 pub? ──
//      impl 块没有 pub / 不 pub 的说法，可见性由方法自己控制。
//      一个 impl 块里可以混着 pub 方法和私有方法:
//
//        impl Student {
//            pub fn public_api(&self) { ... }   // 任何人都能调
//            fn internal_helper(&self) { ... }  // 只有本模块能调
//        }
//
//      这样同一个 impl 块里既有公开入口又有私有辅助, 灵活控制。
//
//      注意: pub 管的是"能不能用", 不是"能不能改"。
//      即使字段是 pub 的, 也只有通过 &mut self 才能修改它。
//
//      ── 关于"扩展"外部类型 ──
//      "不能给别人的类型加方法"中,"别人"是指外部 crate (标准库、第三方库),
//      不是你项目里的另一个文件。
//
//      你自己 crate 里的 struct, 不管定义在哪个文件, 在任意文件里都能
//      继续加 impl 块——因为都是同一个 crate, 不受孤儿规则约束:
//
//        // a.rs 定义了 struct MyStruct;
//        // b.rs 里完全可以直接写:
//        impl MyStruct { fn extra_method(&self) { ... } }  // OK! 同一个 crate
//
//      但如果类型来自外部 crate(标准库的 Vec、第三方的库), 直接 impl 就报错:
//
//        impl Vec<i32> { fn my_method(&self) { } }  // 编译报错! 孤儿规则
//
//      那怎么给外部类型加新方法? 两种方式:
//
//        a) 通过 trait 扩展 (推荐, 详见 traits.rs):
//           trait MyExt { fn do_thing(&self); }
//           impl MyExt for Vec<i32> { fn do_thing(&self) { ... } }  // OK!
//
//        b) 包装类型 (类似装饰器模式):
//           struct MyVec(Vec<i32>);  // 元组结构体包一层
//           impl MyVec { fn custom(&self) { ... } }  // 给包装加方法
//           // 缺点: 没法直接用 Vec 原有的方法, 得手动转发
//
//   3. 特型实现(impl Trait for Type)和自身方法(impl Type)是两个维度。
//      不会出现"这个方法是类自带的还是接口给的"的混乱。
//
// 简而言之:
//   struct = 这个类型长什么样（数据）
//   impl   = 这个类型能做什么 （行为）
//
impl Student {
    fn new(name: &str, age: i32, score: i32) -> Self {
        Student {
            name: String::from(name),
            age,
            score,
        }
    }

    // &self: 不可变借用, 只读
    fn is_pass(&self) -> bool {
        self.score >= 60
    }

    fn intro(&self) -> String {
        format!("我叫{}, {}岁, {}分", self.name, self.age, self.score)
    }

    // &mut self: 可变借用, 可修改字段
    fn add_score(&mut self, delta: i32) {
        self.score += delta;
    }
}

// ── 枚举定义 ──

/// 简单枚举(变体不携带数据)
///
/// #[allow(dead_code)] 也是属性(attribute), 和 #[derive] 同类.
/// 作用: 告诉编译器"这个类型/函数定义了但没用到, 别报警告".
/// dead_code = "写了但没被调用的代码". 这里 Direction 只定义了类型,
/// 没有创建它的实例, 编译器会 warn, 加 allow 就沉默了.
/// 教学代码里常用, 实际项目里如果有没用的代码建议删掉而不是加 allow.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// 带数据的枚举: 每个变体可以携带不同类型的载荷(payload)
enum WebEvent {
    Click { x: i32, y: i32 }, // 带命名字段(像结构体)
    KeyPress(char),           // 带一个值(像元组结构体)
    Resize(i32, i32),         // 带两个值
    Quit,                     // 无数据
}

// ── 枚举方法 ──
//
// 枚举的方法也在 impl 块里, 和结构体一样——数据(struct/enum)和行为(impl)分开。

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
    let mut p1 = Student {
        name: String::from("小明"),
        age: 18,
        score: 85,
    };
    println!("p1: {}", p1.intro());

    // mut 可以修改字段
    p1.score = 90;
    println!("  修改后: {:?}", p1);

    // 字段简写(变量名==字段名时可省略冒号)
    let name = String::from("小红");
    let age = 17;
    let p2 = Student {
        name,
        age,
        score: 92,
    };
    println!("p2: {:?}", p2);

    // 结构体更新语法: 从现有实例复制其余字段
    let p3 = Student {
        name: String::from("小刚"),
        ..p1
    };
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
        Direction::East => println!("  往东"),
        Direction::West => println!("  往西"),
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
    println!(
        "enum:   \"选一个\" —— 事件可能是 Click \x1b[1;37m或\x1b[0m KeyPress \x1b[1;37m或\x1b[0m Quit"
    );
}

fn handle_event(event: &WebEvent) {
    print!("[{}] ", event.describe());
    match event {
        WebEvent::Click { x, y } => println!("位置: ({}, {})", x, y),
        WebEvent::KeyPress(c) => println!("字符: '{}'", c),
        WebEvent::Resize(w, h) => println!("尺寸: {}x{}", w, h),
        WebEvent::Quit => println!("再见!"),
    }
}
