//! 模式匹配进阶: 解构、嵌套、绑定。
//!
//! ## basic 的 match 讲了什么
//!
//! basic/match_flow 讲的是 match 基础用法:
//!   - 值 vs 值的比较 (match x { 1 => ..., 2 => ..., _ => ... })
//!   - match 作为表达式 (let score = match grade { ... })
//!   - | 或模式 ('a' | 'e' | 'i' => ...)
//!   - ..= 范围匹配 (0..=12 => ...)
//!   - 守卫 if (n if n % 2 == 0 => ...)
//!
//! 这些的本质都是: 拿一个"简单值"和"模式"做等值比较。值是 i32、char 等标量。
//!
//! ## 本文件拓展了什么
//!
//! 当值不是标量, 而是复合类型 (struct、enum、元组) 时, match 不只是"比较",
//! 还能把复合类型的**内部数据拆出来**——这叫"解构"(destructuring)。
//!
//! 本文件在 basic 基础上新增四个能力:
//!
//!   ┌─────────────┬───────────────────────────────────────┐
//!   │ 概念          │ 做了什么                              │
//!   ├─────────────┼───────────────────────────────────────┤
//!   │ 解构结构体    │ 把 struct 的字段拆出来绑定到变量        │
//!   │ 嵌套模式      │ 在 enum 的变体里再解构内部的 struct     │
//!   │ @ 绑定       │ 既匹配模式, 又把整个值绑定到一个名字      │
//!   │ .. 忽略      │ 只解构关心的字段, 其余不管              │
//!   └─────────────┴───────────────────────────────────────┘
//!
//! 前置依赖: basic/ 中的 match; intermediate/ 中的 structs_and_enums.


// ── 类型定义 ──

#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Line(Point, Point),              // 枚举变体里嵌套了 struct
    Nothing,
}


// ═══════════════════════════════════════════════════════════════
// 第 1 节: 解构结构体
// ═══════════════════════════════════════════════════════════════
//
// basic 里 match x { 1 => ..., 2 => ... } 是比较 x 是不是 1、是不是 2。
// 这里 match 的不是"判断相等", 而是"把 struct 的字段拆出来"。

/// 演示用 match 把 struct 字段解构到变量里。
fn demo_struct_destructure() {                          // 辅助函数, 保持 run() 清晰
    let p = Point { x: 3, y: 7 };                      // p 拥有 Point (虽然字段 i32 是 Copy, 但 Point 没 derive Copy)

    // ⚠ 所有权: match p 会消耗(move) p! 解构时把字段拆出来, p 本身不再可用。
    //    Point 没实现 Copy, 所以 match 拿走了 p 的所有权。
    //    如果 Point 实现了 Copy (加 #[derive(Copy, Clone)]), match 会复制一份, p 仍可用。
    match p {
        Point { x: 0, y: 0 } => println!("原点"),       // 精确匹配: x==0 且 y==0
        Point { x, y: 0 }    => println!("在 x 轴上, x={}", x), // x 绑定到变量 x, y 精确匹配 0
        Point { x: 0, y }    => println!("在 y 轴上, y={}", y), // y 绑定到变量 y, x 精确匹配 0
        Point { x, y }       => println!("普通点: ({}, {})", x, y), // x/y 都绑定到同名变量
    }
    // 语法解读:
    //   Point { x: 0, y }    → 字段 x 必须等于 0, 字段 y 解构到变量 y
    //   Point { x, y }       → 两个字段都解构到同名变量 (字段名简写)
    // 注意和"赋值"的区别: let 里的 Point{x, y} 是创建, match 里的 Point{x, y} 是解构。
}


// ═══════════════════════════════════════════════════════════════
// 第 2 节: 嵌套模式 + 守卫 (if)
// ═══════════════════════════════════════════════════════════════
//
// 枚举变体里可能嵌套了 struct。match 可以一层一层往里拆。
// 守卫 if 在 basic 里已学过 (n if n % 2 == 0), 这里结合解构一起用:
// 先解构拿到字段, 再用 if 对字段做额外判断。

fn demo_nested_and_guard() {
    // shapes 拥有 Vec<Shape> — 所有权归 shapes。Shape 的变体数据 (如 radius: f64) 也被 shapes 拥有。
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rect { w: 3.0, h: 4.0 },
        Shape::Line(Point { x: 0, y: 0 }, Point { x: 10, y: 10 }),
        Shape::Nothing,
    ];

    // for shape in &shapes: 借 Vec, shape 是 &Shape (不可变借用)。
    // match shape: shape 是引用, match 不消耗枚举值 — 只是借来看。
    for shape in &shapes {
        match shape {
            // 解构 Circle → 拿到 radius
            Shape::Circle { radius } => {
                // 这里用了全路径写法 std::f64::consts::PI。
                //
                // std::f64::consts::PI 是 Rust 标准库内置的圆周率常量, 类型 f64。
                // 路径拆开看:
                //   std                — 标准库根
                //   f64                — f64 类型模块
                //   consts             — 常量子模块 (包含 PI、E、SQRT_2 等数学常量)
                //   PI                 — π ≈ 3.141592653589793
                // 同理 std::f32::consts::PI 是 f32 版本的 π。
                //
                // Rust 引用外部符号有两种写法:
                //   1. 顶部声明: use std::f64::consts::PI;  然后代码里直接写 PI
                //   2. 全路径:   std::f64::consts::PI      不声明, 当场写全
                //
                // 区别:
                //   - 顶部 use: 整个文件可见, 多次使用更简洁。
                //   - 全路径:   用一次写一次, 适合临时/低频/避免命名冲突。
                //
                // 全路径还能用 self/super/crate 做相对引用:
                //   self::foo    — 当前模块
                //   super::foo   — 上级模块
                //   crate::foo   — 从 crate 根开始
                //
                // 这里选全路径是因为只在一个地方用 PI, 不值得为它加一行 use。
                println!("圆, 半径={}, 面积≈{:.1}",
                    radius, std::f64::consts::PI * radius * radius);
            }

            // 先解构 Rect 拿到 w, h, 再用守卫 if 额外判断是否是正方形
            Shape::Rect { w, h } if w == h => {         // ← 守卫: 解构 + 条件
                println!("正方形, 边长={}", w);
            }
            Shape::Rect { w, h } => {                   // ← 守卫不满足时走这里
                println!("矩形, {}×{}", w, h);
            }

            // 嵌套解构: Shape::Line 里是 Point, 再把 Point 的 x,y 拆出来
            Shape::Line(
                Point { x: x1, y: y1 },                 // 第一个 Point → x1, y1
                Point { x: x2, y: y2 },                 // 第二个 Point → x2, y2
            ) => {
                println!("线段: ({},{}) → ({},{})", x1, y1, x2, y2);
            }

            Shape::Nothing => println!("空形状"),
        }
    }
    // 嵌套本质: 模式里可以继续写模式。Shape::Line(Point{x,y}, Point{x,y})
    // 就是"Shape 是 Line → Line 的两个字段是 Point → Point 的 x,y 拆出来"。
}


// ═══════════════════════════════════════════════════════════════
// 第 3 节: @ 绑定 — 匹配模式 + 绑定整个值
// ═══════════════════════════════════════════════════════════════
//
// 场景: 你既想用 ..= 范围匹配, 又想知道"实际匹配到的值是多少"。
// 不用 @ 时范围匹配会"吃掉"具体值, 你只知道它在 90~100 之间, 不知道具体是几。
// @ 语法: 变量名 @ 模式  →  "匹配这个模式, 同时把匹配到的整个值绑到变量名"
//
// 对比:
//   n @ 90..=100     →  范围检查 + 把值绑到 n (n 就是具体分数)
//   90..=100          →  只做范围检查, 拿不到具体值

fn demo_at_binding() {
    let scores = vec![95, 82, 60, 45, 88];             // scores 拥有 Vec<i32>

    // for &s in &scores: &scores 借 Vec, &s 模式解构剥掉引用 → s: i32 (Copy, 独立复制)
    // match s: s 是 i32 Copy, match 只是比较值, 不消耗所有权。
    for &s in &scores {
        match s {
            n @ 90..=100 => println!("{} 分: 优秀 (具体值 n={})", s, n),
            n @ 60..=89  => println!("{} 分: 及格 (具体值 n={})", s, n),
            n @ 0..=59   => println!("{} 分: 不及格 (具体值 n={})", s, n),
            _ => println!("{} 分: 无效", s),
        }
    }
    // n @ 90..=100: "匹配 90~100 范围, 同时把实际值绑到变量 n"
    // 不用 @ 的话要这样写:
    //   90..=100 => { let n = s; println!("{}", n); }  ← 在 => 右边手动绑定, 啰嗦
}


// ═══════════════════════════════════════════════════════════════
// 第 4 节: .. 忽略 — 只解构关心的字段
// ═══════════════════════════════════════════════════════════════
//
// 结构体有 5 个字段但你只关心 1 个? 用 .. 忽略其余。
// 和 basic 里 _ 的区别: _ 忽略单个值, .. 忽略任意多个字段。
//
//   Point { x: 0, .. }    →  只检查 x==0, y 不管
//   Point { .. }          →  匹配所有 Point, 不拿任何字段

fn demo_ignore() {
    let p = Point { x: 5, y: 10 };                    // p 拥有 Point
    // ⚠ match p 消耗(move) p — 同 demo_struct_destructure 的说明。
    //    这里 p 是最后一个使用, move 进去没问题。
    match p {
        Point { x: 0, .. } => println!("在 y 轴上 (x=0)"),     // 不管 y
        Point { y: 0, .. } => println!("在 x 轴上 (y=0)"),     // 不管 x
        Point { x, .. }    => println!("x={} (不关心 y)", x),   // 只要 x, 忽略 y
    }

    // .. 也可以和 _ (单个忽略) 结合:
    //   Shape::Line(Point { x, .. }, _)  → 只拿起点 x, 忽略起点 y 和整个终点
}

// ── 本节可选: 试试换掉上面例子的 .., 用逐个字段写全, 对比哪个更简洁 ──


// ═══════════════════════════════════════════════════════════════
// 附: | 或模式 (basic 已学, 这里快速复习)
// ═══════════════════════════════════════════════════════════════
//
// | 在 basic 里用于简单值: 'a' | 'e' | 'i' => ...
// 在进阶里同样可以和解构组合:
//   Point { x: 0, .. } | Point { y: 0, .. } => println!("在轴上")
// 多个复杂模式用 | 合并到一个分支。

fn demo_or_pattern() {
    // &[1,2,3,4,5]: 数组字面量, 临时借用。c: i32 Copy, 匹配不涉及所有权。
    for &c in &[1, 2, 3, 4, 5] {
        match c {
            1 | 3 | 5 => println!("{} 是奇数", c),
            2 | 4     => println!("{} 是偶数", c),
            _ => {}
        }
    }
}


// ═══════════════════════════════════════════════════════════════
// run — 按节调用
// ═══════════════════════════════════════════════════════════════

pub fn run() {
    println!("===== 第 1 节: 解构结构体 =====");
    demo_struct_destructure();

    println!("\n===== 第 2 节: 嵌套模式 + 守卫 =====");
    demo_nested_and_guard();

    println!("\n===== 第 3 节: @ 绑定 =====");
    demo_at_binding();

    println!("\n===== 第 4 节: .. 忽略 =====");
    demo_ignore();

    println!("\n===== 附: | 或模式 (basic 复习) =====");
    demo_or_pattern();
}
