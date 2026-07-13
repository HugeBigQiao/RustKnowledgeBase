//! 模式匹配深入：match 不只是"等于判断", 还能解构、嵌套、绑定。
//!
//! 前置依赖: basic/ 中的 match 基础, intermediate/ 中的 structs_and_enums.

// ── 类型定义 ──

#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Line(Point, Point),
    Nothing,
}

// ── run ──

/// 演示结构体解构、嵌套模式、@ 绑定、守卫(if)、.. 忽略、|
pub fn run() {
    // ===== 解构结构体 =====
    println!("===== 解构结构体 =====");
    let p = Point { x: 3, y: 7 };

    // match 中把结构体字段"拆出来"绑定到变量
    match p {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x, y: 0 }    => println!("在 x 轴上, x={}", x),
        Point { x: 0, y }    => println!("在 y 轴上, y={}", y),
        Point { x, y }       => println!("普通点: ({}, {})", x, y),
    }
    // 注意: Point { x, y } 是"把字段 x 绑定到变量 x, 字段 y 绑定到变量 y"

    // ===== 嵌套模式 =====
    println!("\n===== 嵌套模式 =====");
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rect { w: 3.0, h: 4.0 },
        Shape::Line(Point { x: 0, y: 0 }, Point { x: 10, y: 10 }),
        Shape::Nothing,
    ];

    for shape in &shapes {
        match shape {
            // 匹配 Circle 同时取出 radius
            Shape::Circle { radius } => {
                println!("圆, 半径={}, 面积≈{:.1}", radius, std::f64::consts::PI * radius * radius);
            }
            // 匹配 Rect, 同时用守卫判断是不是正方形
            Shape::Rect { w, h } if w == h => {
                println!("正方形, 边长={}", w);
            }
            Shape::Rect { w, h } => {
                println!("矩形, {}×{}", w, h);
            }
            // 嵌套解构: Shape::Line 内部是 Point, 再解构 Point 的字段
            Shape::Line(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) => {
                println!("线段: ({},{}) → ({},{})", x1, y1, x2, y2);
            }
            Shape::Nothing => println!("空形状"),
        }
    }

    // ===== @ 绑定 =====
    println!("\n===== @ 绑定 =====");
    // @: 既匹配模式, 又把整个值绑定到一个变量名.
    // 语法: 变量名 @ 模式

    let scores = vec![95, 82, 60, 45, 88];

    for &s in &scores {
        match s {
            n @ 90..=100 => println!("{} 分: 优秀(n={})", s, n),
            n @ 60..=89  => println!("{} 分: 及格(n={})", s, n),
            n @ 0..=59   => println!("{} 分: 不及格(n={})", s, n),
            _ => println!("{} 分: 无效", s),
        }
        // n @ 90..=100: 既检查范围, 又把实际值绑定到 n.
    }

    // ===== | 或模式 =====
    println!("\n===== | 或模式 =====");
    for &c in &[1, 2, 3, 4, 5] {
        match c {
            1 | 3 | 5 => println!("{} 是奇数", c),
            2 | 4     => println!("{} 是偶数", c),
            _ => {}
        }
    }

    // ===== .. 忽略剩余字段 =====
    println!("\n===== .. 忽略剩余字段 =====");
    let p2 = Point { x: 5, y: 10 };
    match p2 {
        Point { x: 0, .. } => println!("在 y 轴上(x=0)"),
        Point { y: 0, .. } => println!("在 x 轴上(y=0)"),
        Point { x, .. }    => println!("x={} (不关心 y)", x),
    }
    // .. 表示"我不关心其余字段". 在结构体和枚举变体中都可以用.

    // ===== match 守卫 if =====
    println!("\n===== match 守卫 =====");
    let pair = (3, -2);
    match pair {
        (x, y) if x == y      => println!("相等: ({}, {})", x, y),
        (x, y) if x + y == 0  => println!("互为相反数: ({}, {})", x, y),
        (x, y) if x > 0 && y > 0 => println!("都为正: ({}, {})", x, y),
        (x, y)                => println!("普通: ({}, {})", x, y),
    }
    // 守卫 if: 模式匹配后再加一个条件, 不满足条件就算匹配失败.
}
