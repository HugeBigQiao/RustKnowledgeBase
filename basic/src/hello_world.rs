pub fn run() {
    println!("pub 是 public 的意思，表示这个函数是公开的，可以被其他模块调用和访问");
    println!("hello_world 是函数的名称,Rust 采用 snake_case 命名法，函数名小写，单词用下划线连接");
    println!("通常每个函数都有参数，这里没有参数，所以是 ()");
    println!("fn 是函数定义关键字,main 是程序入口（程序从这里开始执行）");
    println!("{{ }} 是块(block)创建一个新的作用域");
    println!("  块本身也是表达式——里面最后一行不加分号，块就会返回那个值");
    println!("  比如 let y = {{ let x = 1; x + 2 }};  // 块返回值 3,赋给 y");
    println!("  整个 main 函数的 {{ }} 就是最大的块，里面的内容依次执行");
    println!();

    // ===== println! 宏与格式化占位 =====
    println!("Hello, world!");
    println!("println! 是一个宏，用于打印并自动换行");
    println!("{} + {} = {}", 1, 2, 3);
    println!("  上面用的是 {{}} 位置占位符：按顺序填入");
    println!("{0} + {0} = {1}", 2, 4);
    println!("  上面用的是 {{0}} 索引占位符：按位置编号填入，可复用");
    println!("{name} 说：{msg}", name = "Rust", msg = "你好");
    println!("  上面用的是 {{name}} 命名占位符：按变量名填入");
    let nums = vec![1, 2, 3];
    println!("调试输出：{:?}", nums);
    println!("  上面用的是 {{:?}} 调试占位符：打印 Debug 格式");
    println!();

    // ===== let、表达式、语句、分号 =====
    println!("let: 变量绑定关键字, let 本身是语句，不产生值，以分号结尾");
    println!("let 右边的 = 和表达式才会产生值，绑定给左边的变量名");
    println!("fn hello_world() 是一个函数声明，也是语句，不产生值");
    println!("一句话，表达式 = 有值，语句 = 没值。分号把表达式变成语句");
    println!("通常函数最后一行不用分号，那么最后一行就会产生值，如果你想要它产生值的话");
    println!("后面碰到不同的表达式和语句的时候会说明");
    println!();
    println!("print! 和 println! 的区别：换行");
    print!("print! 输出后不换行，");
    print!("所以下一个 print! 会接在后面，");
    print!("三个 print! 全挤在同一行。");
    println!();
    println!("而 println! 输出后自动换行，所以我是新起的一行。");
    println!();
    println!("宏是 Rust 中一种特殊的编译时代码生成机制，具体在宏部分讲解。");
    println!();

    println!(":: 是路径分隔符，表示进入这个模块里面找");
    println!();
}
