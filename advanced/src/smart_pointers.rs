//! 智能指针: Box<T> / Deref / Drop / Rc<T> / Arc<T>.

use std::rc::Rc;
use std::sync::Arc;

// ===== 1. Box<T>: 在堆上分配值 =====

/// 递归类型必须用 Box, 因为编译器无法确定它的大小.
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    /// Cons 节点: 值 + 指向下一个节点的 Box.
    Cons(i32, Box<List>),
    /// Nil 终止.
    Nil,
}

/// 演示 Box 的两种典型用途: 递归类型 + 把大值移到堆上.
fn demo_box() {
    println!("--- Box<T> ---");

    // 用法1: 把值放在堆上
    let b = Box::new(5);
    println!("Box<i32> = {} (在堆上, 地址: {:p})", b, b);

    // 用法2: 递归类型
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("递归链表: {:?}", list);

    // Box 实现的 trait:
    // - Deref: &Box<T> 自动转 &T
    // - Drop: 离开作用域时自动释放堆内存
    // - Clone: 如果 T: Clone
}

// ===== 2. Deref 与 DerefMut: 智能指针对普通引用的解析 =====

use std::ops::Deref;

/// 自定义智能指针, 模仿 Box<T>.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 演示 Deref 强制转换: &MyBox<T> 可以当作 &T 用.
fn demo_deref() {
    println!("\n--- Deref/DerefMut ---");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y 等价于 *(y.deref())

    // 解引用强制转换: &MyBox<String> → &str
    let s = MyBox::new(String::from("hello"));
    // fn hello(name: &str) { ... } → 可以传入 &s, Rust 自动调用 deref
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet(&s); // &MyBox<String> → &String → &str (链式转换)
}

// ===== 3. Drop trait: 离开作用域时自动清理 =====

/// 带清理日志的类型.
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("[Drop] '{}' 正在被释放!", self.data);
    }
}

/// 演示 Drop 的执行时机和 std::mem::drop (非强制).
fn demo_drop() {
    println!("\n--- Drop trait ---");

    let c = CustomSmartPointer {
        data: String::from("资源A"),
    };
    let _d = CustomSmartPointer {
        data: String::from("资源B"),
    };
    println!("两个 CustomSmartPointer 已创建");

    // 不能手动调用 c.drop() (编译器禁止)
    // 但可以用 std::mem::drop 提前释放:
    drop(c);
    println!("提前释放了'资源A'");
    // _d 在函数结束时自动释放
}

// ===== 4. Rc<T>: 引用计数(单线程共享所有权) =====

/// 演示 Rc 的共享所有权和引用计数.
fn demo_rc() {
    println!("\n--- Rc<T> (单线程引用计数) ---");

    // Rc 让多个"所有者"共享同一份堆数据
    let a = Rc::new(String::from("共享数据"));
    println!("创建 a, 引用计数: {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a); // 浅拷贝: 只增加引用计数
        println!("clone 后引用计数: {}", Rc::strong_count(&a));
        println!("a 和 b 指向同一数据: '{}' == '{}'", a, b);
    } // b 离开作用域, 引用计数减 1

    println!("b 释放后引用计数: {}", Rc::strong_count(&a));
    // a 离开作用域时引用计数归零, 自动释放内存
}

// ===== 5. Arc<T>: 原子引用计数(多线程安全) =====

/// 演示 Arc 类似 Rc 但线程安全.
fn demo_arc() {
    println!("\n--- Arc<T> (多线程引用计数) ---");

    let a = Arc::new(42);
    println!("创建 Arc 包裹的值: {}, 引用计数: {}", *a, Arc::strong_count(&a));

    let b = Arc::clone(&a);
    println!("clone 后引用计数: {}", Arc::strong_count(&a));

    // Arc 的 clone 也是浅拷贝, 只增加引用计数
    // Arc 实现了 Send + Sync, Rc 没有
    // 多线程场景用 Arc, 单线程场景用 Rc

    drop(b);
    println!("释放 b 后引用计数: {}", Arc::strong_count(&a));
}

pub fn run() {
    demo_box();
    demo_deref();
    demo_drop();
    demo_rc();
    demo_arc();
}
