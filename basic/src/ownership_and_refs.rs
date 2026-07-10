//! 所有权与引用
//!
//! Rust 没有垃圾回收(GC), 也不用手动 free 内存.
//! 而是通过"所有权"系统在编译期保证内存安全.
//! 所有权是 Rust 最核心的概念, 贯穿一切.

/// 所有权三条规则:
/// 1. Rust 中每个值都有一个"所有者"(owner).
/// 2. 同一时刻, 一个值只能有一个所有者.
/// 3. 所有者离开作用域时, 值被自动释放(drop).
///
/// 引用(&)允许你"借用"一个值而不获取所有权.
/// 借用规则:
/// - 同一时刻, 要么一个可变引用(&mut), 要么多个不可变引用(&).
/// - 引用必须始终有效(不能指向已释放的内存).
pub fn run() {
    // ===== mut 关键字: 让绑定可变 =====
    // mut 不是类型的组成部分, 而是变量"绑定"的属性.
    // let mut x = 5; 表示 x 这个绑定可变, 类型仍然是 i32.
    // mut 的作用:
    //   1. 允许重新赋值:  x = 10;
    //   2. 允许获取 &mut 可变引用:  let r = &mut x;
    //   3. 允许调用修改自身的方法(如 push):  v.push(1);
    // 没有 mut 的变量只能读, 不能改. 这是 Rust 的"默认不可变"原则.
    let mut counter = 0;
    counter = counter + 1;  // mut 才能重新赋值
    println!("counter = {} (mut 允许重新赋值)", counter);
    // let x = 5; x = 6;  // 编译报错! 没有 mut 不能改

    // ===== 方法调用: :: 与 . 的区别 =====
    // :: 是"路径分隔符", 用来导航到模块、类型、关联函数.
    //   用法: 类型::关联函数  比如 String::from("hello")
    //   from 属于 String 类型本身, 不依赖某个具体的 String 实例.
    // .  是"方法调用", 在某个具体值上调用.
    //   用法: 实例.方法()  比如 s.push_str("!")
    //   push_str 需要一个具体的 String 实例来操作.
    // 简单记忆:
    //   类型级别的功能(构造、关联常量)用 ::
    //   实例级别的功能(修改、查询自身)用 .
    // Rust 的 . 会自动解引用和自动引用:
    //   你写 s.len(), 编译器自动变成 (&s).len(), 不用手动写 &.
    let demo = String::from("demo");  // :: 调用关联函数
    println!("len = {} (. 方法调用, 自动引用)", demo.len());  // . 调用方法

    // ===== 所有权: 变量绑定 = 所有权转移 =====
    // 基本规则: 赋值会转移所有权(move).
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权转移给了 s2
    // println!("{}", s1);  // 编译报错! s1 已经失效
    println!("s2 = \"{}\" (s1 的所有权已移给 s2, s1 失效)", s2);

    // ===== Copy 类型: 不会转移所有权 =====
    // 存在栈上的简单类型实现了 Copy trait, 赋值是"复制"而不是"移动".
    let a = 42;
    let b = a;   // a 被复制了一份, 所有权还在 a 手上
    println!("a = {}, b = {} (i32 是 Copy 类型, 赋值是复制, a 还能用)", a, b);

    // Copy 类型包括: 整数、浮点、bool、char、元素全是 Copy 的元组.
    // 需要堆分配的类型(String、Vec 等) 不是 Copy, 赋值会 move.

    // ===== 函数传参也会转移所有权 =====
    // 把值传给函数, 所有权就被移进函数了.
    fn take_ownership(s: String) {
        println!("  函数拿到了所有权: \"{}\"", s);
    } // s 离开作用域, 被释放

    let msg = String::from("你好");
    take_ownership(msg);
    // println!("{}", msg);  // 编译报错! msg 的所有权已移入函数

    // ===== 浅拷贝与深拷贝 =====
    // 浅拷贝(shallow copy): 只复制栈上的数据(指针、长度、容量).
    //   String 的栈部分是 {指针, 长度, 容量}, 堆上是实际字符.
    //   浅拷贝只复制 {指针, 长度, 容量}, 两个变量指向同一块堆内存.
    //   如果两个都释放同一块内存, 就是 double-free, 导致崩溃.
    //
    // 深拷贝(deep copy): 连堆上的数据也复制一份.
    //   两个变量各自拥有独立的堆内存, 互不影响.
    //
    // Rust 的做法:
    //   - 默认 assignment 是 move(浅拷贝栈 + 让旧变量失效), 避免 double-free.
    //   - clone() 是深拷贝, 复制堆数据, 两个变量各自独立.
    //   - Copy 类型(栈上简单类型)没有堆数据, 浅拷贝 = 深拷贝, 赋值即复制.

    // ===== 不想转移所有权? 用 clone() =====
    // clone() 在堆上复制一份新数据, 各自拥有独立的所有权.
    let original = String::from("Rust");
    let cloned = original.clone();  // 深拷贝: 连堆数据一起复制
    println!("original = \"{}\", cloned = \"{}\" (clone 是深拷贝, 各自独立)", original, cloned);

    fn take_and_return(s: String) -> String {
        println!("  函数收到: \"{}\", 然后还回去", s);
        s  // 把所有权还回去(隐式返回)
    }
    let s = String::from("借用后归还");
    let s = take_and_return(s);  // 所有权出去又回来
    println!("拿回来了: \"{}\"", s);

    // ===== 引用(&): 借用而不获取所有权 =====
    // & 创建引用, 你可以"借用"值来用, 用完所有权还是别人的.
    fn print_len(s: &String) {  // 借用一个 String, 不拿所有权
        println!("  长度: {} (通过引用 & 借用, 不拿所有权)", s.len());
    } // s 离开作用域, 但只是引用, 不会释放原数据

    let name = String::from("Rustacean");
    print_len(&name);   // 传引用进去, 不转移所有权
    println!("name 还能用: \"{}\" (因为只传了引用 &)", name);

    // ===== 可变引用(&mut): 借用来修改 =====
    // 想通过引用修改值? 用 &mut.
    fn append_exclamation(s: &mut String) {
        s.push_str("!");
    }

    let mut text = String::from("Hello");
    append_exclamation(&mut text);
    println!("text = \"{}\" (通过 &mut 修改了原值)", text);

    // ===== 借用规则: 一写或多读, 不能同时 =====
    // 规则1: 同一时刻, 只能有 1 个可变引用.
    let mut data = String::from("rust");
    let r1 = &mut data;
    // let r2 = &mut data;  // 编译报错! 不能同时有两个 &mut
    r1.push_str("!");
    println!("data = \"{}\" (只有一个 &mut, 没问题)", data);

    // 规则2: 同一时刻, 可以有多个不可变引用.
    let data = String::from("readonly");
    let r1 = &data;
    let r2 = &data;
    println!("r1 = \"{}\", r2 = \"{}\" (多个 & 可以同时存在)", r1, r2);

    // 规则3: 可变引用和不可变引用不能同时存在.
    let data = String::from("mixed");
    let r1 = &data;       // 不可变借用
    // let r2 = &mut data;  // 编译报错! 不能同时有 & 和 &mut (且 data 没有 mut)
    println!("r1 = \"{}\"", r1);

    // ===== 悬垂引用: Rust 编译器会阻止 =====
    // 不能返回指向局部变量的引用, 因为局部变量在函数结束时被释放.
    // fn dangling() -> &String {
    //     let s = String::from("temp");
    //     &s  // 编译报错! s 马上被释放, 返回的 &s 指向无效内存
    // }
    println!("Rust 编译器不允许返回悬垂引用, 编译期就能发现这类 bug.");

    // ===== 切片(Slice): 引用一段连续数据 =====
    // 切片是对数组/Vec/String 中一段连续元素的引用.
    // 它不拥有数据, 只是"借来看其中一段".
    // 切片是"胖指针": 包含 {指向数据的指针, 长度}.
    //
    // 常见切片类型:
    //   &[T]    —— 数组或 Vec 的切片     (比如 &[i32])
    //   &str    —— 字符串切片            (其实 &str 就是 &[u8] 的 UTF-8 保证版)
    //
    // 切片语法: &变量[起始..结束]
    //   起始..结束 是范围, 包含起始, 不包含结束(左闭右开).

    // 数组切片
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4];  // 索引 1,2,3 → 20,30,40
    println!("arr[1..4] = {:?} (数组切片, 不拥有数据)", slice);
    println!("  原数组还能用: {:?}", arr);

    // &arr[..]  取全部
    // &arr[..3] 从头到索引 3(不含)
    // &arr[2..] 从索引 2 到尾
    println!("arr[..2]  = {:?} (从头到索引 2)", &arr[..2]);
    println!("arr[3..]  = {:?} (从索引 3 到尾)", &arr[3..]);
    println!("arr[..]   = {:?} (全部)", &arr[..]);

    // Vet 也可以切片
    let v = vec![1, 2, 3, 4, 5];
    let vs: &[i32] = &v[1..4];  // Vec 切片, 类型也是 &[i32]
    println!("vec[1..4] = {:?} (Vec 切片, 和数组切片类型一样)", vs);

    // 字符串切片: &str
    // &str 是最常见的切片类型, 其实质是 &[u8] 加上"内容必定合法 UTF-8"的保证.
    let s = String::from("Hello World");
    let hello: &str = &s[0..5];  // "Hello"
    let world: &str = &s[6..11]; // "World"
    println!("\"{}\"[0..5]  = \"{}\"", s, hello);
    println!("\"{}\"[6..11] = \"{}\"", s, world);

    // 字符串字面量本身就是 &str
    let literal: &str = "我是字符串字面量, 类型就是 &str";
    println!("字面量: \"{}\" (本身就是切片)", literal);
}
