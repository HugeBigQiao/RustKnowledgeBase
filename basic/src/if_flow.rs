//! 条件判断：if / else if / else
//! Rust 的 if 是一个表达式，可以产生值，这点和大多数语言不同。
//！
//！ 控制流(control flow)决定代码的执行顺序: 先做什么, 后做什么, 什么情况下跳过什么. 
//！ if 是控制流的一种. 
//！ 控制流的基本问题是: 给定一个值, 根据值的不同走不同的逻辑分支. 
//！ if 适合"是/否"判断 
//！ 演示 if 的条件判断、多分支、以及 if 作为表达式赋值的用法。 
pub fn run() {
    let x = 42;

    // -------- 单分支 if --------
    // 单分支表示我只考虑这种情况
    if x > 0 {
        println!("{} 是正数", x);
    }

    // -------- if / else --------
    // if和else则考虑满足一种情况的，让后其余情况都归为一种
    if x % 2 == 0 {
        println!("{} 是偶数", x);
    } else {
        println!("{} 是奇数", x);
    }

    // -------- if / else if / else 多分支 --------
    //这里则是考虑多种情况，else if可以有多个，else只能有一个
    let score = 85;
    if score >= 90 {
        println!("{} 分：优秀", score);
    } else if score >= 60 {
        println!("{} 分：及格", score);
    } else {
        println!("{} 分：不及格", score);
    }

    // -------- Rust里的条件必须是 bool，不会隐式转换 --------
    // C/JS 里可以 if (1) 或 if ("hello")，非零/非空即真。
    // Rust 不允许！if 后面只能是 bool（true 或 false）。
    // 下面的代码会编译报错，已注释：
    // if 1 { println!("不会编译"); }  // expected `bool`, found integer
    let flag: bool = true;
    if flag {
        println!("Rust 的条件必须是 bool 类型，不能是数字或字符串");
    }

    // -------- if 是表达式：可以用在 let 右边 --------
    let condition = true;
    let number = if condition {
        5   // 没有分号！这里是表达式，5 会被返回
    } else {
        10  // 同样没有分号
    };
    println!("if 表达式的结果：{}（condition = {}）", number, condition);

    // -------- if 表达式要求分支返回相同类型 --------
    // 下面的代码会编译报错（类型不匹配），已注释：
    // let wrong = if true { 1 } else { "hello" }; // i32 vs &str，不行！
}
