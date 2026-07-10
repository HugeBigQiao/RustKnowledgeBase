//! 复合类型: 元组、数组、字符串
//!
//! 标量类型(整数、浮点、bool、char)一次只存一个值.
//! 复合类型一次可以存多个值, 按组织方式不同分为三类.

/// 元组(Tuple):
///   定长, 元素可以是不同类型. 用 `()` 括起来, 逗号分隔.
///   访问用 `.0` `.1` 按位置索引, 也可以用模式解构.
///
/// 数组(Array):
///   定长, 所有元素必须是同一类型. 用 `[]` 括起来.
///   存在栈上, 大小编译期确定. 类型写作 `[T; N]`.
///
/// 字符串:
///   `&str` 是字符串切片(借用, 不可变引用), 通常指向字面量.
///   `String` 是堆分配的、可增长的 UTF-8 字符串, 有所有权.
pub fn run() {
    // ===== 元组(Tuple) =====
    // 定长, 一旦创建就不能增减元素. 每个位置类型可以不同.
    let tup: (i32, f64, char) = (42, 3.14, 'R');
    println!("元组: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("  用 .0 .1 .2 按位置索引访问");

    // 模式解构: 一次性把元组的各个位置拆出来
    let (x, y, z) = tup;
    println!("  解构: x = {}, y = {}, z = {} (let (x, y, z) = tup)", x, y, z);

    // 空元组 () 就是单元类型, 也是单元值.
    let unit: () = ();
    println!("  空元组 () = {:?}, 单元类型, 类似 void", unit);

    // ===== 数组(Array) =====
    // 定长, 所有元素同类型. 存在栈上.
    let arr: [i32; 3] = [10, 20, 30];
    println!("\n数组: [{}, {}, {}]", arr[0], arr[1], arr[2]);
    println!("  长度: {} (编译期确定, 不能改变)", arr.len());

    // 快速创建: [初始值; 数量]
    let zeros = [0; 5];  // 5 个 0
    println!("  [0; 5] = [{}, {}, {}, {}, {}]", zeros[0], zeros[1], zeros[2], zeros[3], zeros[4]);

    // 超出索引会 panic(运行时崩溃), Rust 不做越界检查的静默忽略.
    // let crash = arr[10];  // 编译通过, 运行时会 panic!

    // ===== 字符串: char、&str、String 三者的关系 =====
    // Rust 中和"文字"相关的类型有三个, 容易混淆. 先记住:
    //   char   = 单个 Unicode 字符, 单引号, 比如 '中' 'a' '7' '\u{1F600}'
    //   &str   = 字符串切片, 借用, 双引号, 比如 "hello" "你好"
    //   String = 拥有所有权的字符串, 堆上, 可变可增长
    //
    // 三者的关系:
    //   "hello"         → &str     (字面量, 编译期写死在二进制里)
    //   String::from("x") → String   (&str 转 String, 堆上新建)
    //   &some_string     → &str     (String 自动转 &str, 解引用强制转换)
    //   'a'.to_string()  → String   (char 转 String)
    //   "abc".chars()    → 逐个拿出 char  (遍历 &str 的字符)
    //
    // 一句总结: char 是单字符, &str 是借来的字符串, String 是自己的字符串.

    // --- char: 单个 Unicode 字符 ---
    let c1: char = 'A';        // 英文字母
    let c2: char = '中';       // 中文汉字
    let c3: char = '7';        // 数字(作为字符, 不是数值 7)
    let c4: char = '😀';       // emoji(也是单个 Unicode 字符)
    println!("char: '{}' '{}' '{}' '{}' (字母/汉字/数字/emoji 都是单个 char)", c1, c2, c3, c4);
    println!("  char 占 4 字节, 存的是 Unicode 标量值");
    println!("  char 和 String 完全不同: char 是单字符, String 是一串字符");

    // --- &str: 字符串切片, 借用, 不拥有数据 ---
    let s1: &str = "Hello, Rust!";  // 字面量, 编译期存入二进制
    println!("\n&str : \"{}\" (字符串切片, 借用, 不可变)", s1);
    // &str 不能修改, 因为它是借来的:
    // s1.push_str("!");  // 编译报错! &str 没有 push_str 方法

    // --- String: 拥有所有权的堆分配字符串 ---
    // 创建 String 的三种方式:
    let s2 = String::from("hello");      // 方式1: String::from(&str)
    let s3 = "world".to_string();        // 方式2: &str.to_string()
    let mut s4 = String::new();           // 方式3: 空 String
    s4.push_str("rust");                  // 往空 String 追加内容
    println!("String: \"{}\" \"{}\" \"{}\" (堆分配, 有所有权)", s2, s3, s4);

    // String 可变, 可以修改:
    let mut s5 = String::from("Hello");
    s5.push_str(", World");   // 追加 &str
    s5.push('!');              // 追加单个 char
    println!("  修改后: \"{}\" (push_str 加 &str, push 加 char)", s5);

    // --- &str 和 String 互转 ---
    // String → &str: 用 & 引用即可, 自动转换(解引用强制转换)
    let owned = String::from("我有所有权");
    let borrowed: &str = &owned;  // &String 自动变 &str
    println!("  &String -> &str: \"{}\" (. 自动转换)", borrowed);

    // &str → String: 用 to_string() 或 String::from()
    let literal: &str = "字面量";
    let owned1 = literal.to_string();     // 方式1
    let owned2 = String::from(literal);   // 方式2
    println!("  &str -> String: to_string() 或 String::from() 都可以");
    println!("    \"{}\" \"{}\" (各自拥有独立内存)", owned1, owned2);

    // --- String 的常用方法 ---
    // 方法调用用 . 号, 这是第次系统接触 Rust 的方法调用.
    // .len() : 字节长度(不是字符个数!)
    //   UTF-8 编码: 英文 1 字节, 中文 3 字节, emoji 4 字节.
    let text = String::from("Rust语言");
    println!("\n常用方法:");
    println!("  \"{}\".len()       = {} (字节长度: R(1)+u(1)+s(1)+t(1)+语(3)+言(3) = 10)", text, text.len());
    println!("  \"{}\".is_empty()  = {} (是否为空)", text, text.is_empty());
    println!("  \"{}\".contains(\"us\") = {} (是否包含子串)", text, text.contains("us"));
    println!("  \"{}\".replace(\"Rust\", \"C++\") = \"{}\" (替换)", text, text.replace("Rust", "C++"));
    println!("  \"Rust\".to_uppercase() = \"{}\" (转大写)", String::from("Rust").to_uppercase());
    println!("  \" Rust \".trim()      = \"{}\" (去首尾空格)", String::from(" Rust ").trim());

    // --- char 和 String 的互转 ---
    // char → String: to_string()
    let ch = 'R';
    let char_to_string: String = ch.to_string();
    println!("\nchar -> String: '{}'.to_string() = \"{}\"", ch, char_to_string);

    // String → 逐个 char: .chars() 方法返回一个迭代器(后面循环细讲)
    // 这里先看一眼: 把 String 拆成一个个 char.
    let word = String::from("Rust");
    print!("  \"{}\" 的每个字符: ", word);
    for c in word.chars() {
        print!("'{}' ", c);
    }
    println!();

    // ===== mut 与复合类型 =====
    // 元组: mut 让整个绑定可重新赋值, 字段也能单独修改.
    let mut t = (1, 2);
    t.0 = 10;  // mut 元组可以改字段
    println!("\nmut 元组: ({}, {}) (mut 可以改字段)", t.0, t.1);
    t = (100, 200);  // mut 还可以整体重新赋值
    println!("  整体重新赋值: ({}, {})", t.0, t.1);

    // 数组: mut 让数组元素可以修改.
    let mut arr = [1, 2, 3];
    arr[0] = 99;  // mut 数组可以改元素
    println!("mut 数组: [{}, {}, {}] (mut 可以改元素)", arr[0], arr[1], arr[2]);
    // 但不能改变长度: arr.push(4); // 编译报错! 数组没有 push

    // &str: &str 是不可变引用, 不能用 mut &str 来修改它指向的内容.
    // 但可以用 mut 让变量指向不同的 &str:
    let mut s = "hello";
    println!("mut &str: \"{}\"", s);
    s = "world";  // mut 让变量指向不同的字符串切片
    println!("  指向新切片: \"{}\" (不是修改了原内容, 是换了个指向)", s);
    // s.push_str("!");  // 编译报错! &str 没有 push_str, 不管有没有 mut

    // String: 必须有 mut 才能调用修改型方法(push_str、push 等).
    // 没有 mut 的 String 只能读, 不能改.
    // let s = String::from("x"); s.push('y'); // 编译报错! s 没有 mut
    let mut owned = String::from("Hello");
    owned.push_str(", Rust!");  // mut 才能 push_str
    println!("mut String: \"{}\" (mut 才能调用 push_str)", owned);

    // ===== 向量(Vec) 独立一个文件, 详见 vec_type.rs =====
}
