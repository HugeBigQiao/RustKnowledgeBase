//! 函数返回值: return
//!
//! Rust 中函数有两种方式返回值: 显式 `return` 和隐式(最后一行表达式). 
//! Rust 的函数体是一个块, 块的值就是最后一行表达式(不加分号)的值. 
//! 这就是隐式返回——不需要写 return.

/// `return` 关键字用于提前退出函数, 一般在条件分支里用.
pub fn run() {
    // ===== 函数嵌套: Rust 允许在函数里定义函数 =====
    // 本文件为了演示方便, 在 run() 里嵌套定义了很多小函数.
    // 嵌套函数只在定义它的外层函数里可见, 外面访问不到.
    // 比如 add() 只能在 run() 里调用, main() 里调不到.
    // 这是 Rust 的合法语法, 常用于封装仅在此处使用的辅助逻辑.
    //
    // 嵌套函数不能加 pub. 因为 pub 控制的是"模块间可见性",
    // 嵌套函数不属于模块级别, 它只在当前函数作用域内存在,
    // 没有模块路径可以指向它, 所以 pub 没有意义, 编译器会拒绝.

    // ===== -> 和 () 说明 =====
    // -> i32  意思是"这个函数返回 i32 类型"
    // 如果不写 -> 类型, 比如 fn foo() { } , 默认返回 () .
    // () 是单元类型(unit type), 类似 C/Java 的 void, 表示"没有值".
    // 它是 Rust 唯一一个零大小类型: 不占内存, 只有一个可能的值: () .
    println!("-> 标注返回类型, 不写则默认返回 () .() 是单元类型, 类似 void.");

    // ===== 隐式返回: 最后一行不加分号 =====
    // 函数体最后一个表达式的值就是返回值, 不需要 return 关键字.
    // 注意: 如果加了分号, 就变成了语句, 返回 () 而不是你要的值.
    fn add(a: i32, b: i32) -> i32 {
        a + b  // 没有分号! 这个表达式的值会被返回
    }
    println!("add(3, 5) = {} (隐式返回, 最后一行不加分号)", add(3, 5));

    // 如果加了分号会怎样?
    // fn add_bad(a: i32, b: i32) -> i32 {
    //     a + b;  // 分号把它变成了语句, 返回 () . 编译报错!
    // }

    // ===== 显式 return: 提前退出 =====
    // return 关键字让函数立即结束, 返回一个值.
    // 函数后面的代码不会被执行.
    fn grade(score: i32) -> &'static str {
        if score < 0 {
            return "分数不能是负数!";  // 提前退出, 后面不执行
        }
        if score < 60 {
            return "不及格";
        }
        if score < 80 {
            return "良好";
        }
        "优秀"  // 所有 if 都不命中, 走到这里, 隐式返回
    }
    println!("grade(85) = {} (最后走隐式返回)", grade(85));
    println!("grade(45) = {} (中间 return 提前退出)", grade(45));
    println!("grade(-5) = {} (开头 return 提前退出)", grade(-5));

    // ===== 提前 return 的典型场景: 参数校验(卫语句) =====
    // 在函数开头用 if + return 做"卫语句"(guard clause), 提前拒绝不合法输入.
    // 这样主体逻辑不用嵌套在 else 里, 代码更扁平.
    fn greet(name: &str) {
        if name.is_empty() {
            println!("名字不能为空!");
            return;  // 提前退出, 返回 ()
        }
        // 走到这里说明 name 不为空, 放心执行主体逻辑
        println!("你好, {}!", name);
    }
    greet("Rust");
    greet("");     // 空字符串, 触发卫语句

    // ===== return 不写值 = 返回 () =====
    // 如果函数返回类型是 () 或没写返回类型, return; 就是 return ();
    fn maybe_print(flag: bool) {
        if !flag {
            return;  // 提前退出, 返回 ()
        }
        println!("这个只有 flag = true 时才打印.");
    }
    maybe_print(false);  // 什么也不打印
    maybe_print(true);   // 打印一行

    // ===== return 只退出当前函数, 不影响调用者 =====
    // 一个函数里的 return 只结束自己, 调用它的函数继续执行.
    fn double(x: i32) -> i32 {
        return x * 2;  // 从 double 返回, 不影响 caller
    }
    fn caller() {
        let a = double(3);   // double 内部的 return 只结束 double
        let b = double(5);   // caller 继续走到这里
        println!("double(3) = {}, double(5) = {}", a, b);
        println!("double 里的 return 只退出 double, caller 照常执行.");
    }
    caller();
}
