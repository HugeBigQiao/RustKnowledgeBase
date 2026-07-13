//! 内部可变性: Cell<T> / RefCell<T> / Rc<RefCell<T>> 模式.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

// ===== 1. Cell<T>: 对 Copy 类型的内部可变性 =====

/// Cell 通过 get/set 替换整个值, 不需 &mut self.
/// 只适用于 Copy 类型.
fn demo_cell() {
    println!("--- Cell<T> ---");

    let x = Cell::new(10);
    println!("初始值: {}", x.get());

    x.set(20);
    println!("set(20) 后: {}", x.get());

    // 对比: 普通不可变绑定不能修改
    let _y = 10;
    // _y = 20;  // 编译错误: 不能修改不可变绑定

    // Cell 适合在用计数、标志位等场景
}

// ===== 2. RefCell<T>: 运行时借用检查 =====

/// RefCell 在运行时执行借用规则, 违反则 panic.
fn demo_refcell() {
    println!("\n--- RefCell<T> ---");

    let data = RefCell::new(vec![1, 2, 3]);

    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("不可变借用: {:?}", borrowed);
    } // borrowed 在这里释放

    // 可变借用
    {
        let mut borrowed_mut = data.borrow_mut();
        borrowed_mut.push(4);
        println!("可变借用后: {:?}", borrowed_mut);
    }

    println!("最终值: {:?}", data.borrow());

    // RefCell vs 编译器借用检查:
    // - 编译器: 编译期静态检查 (快, 但限制严格)
    // - RefCell: 运行时检查 (灵活, 但违规会 panic)
}

// ===== 3. Rc<RefCell<T>>: 多所有者 + 可变数据 =====

/// 标准模式: Rc 提供共享所有权, RefCell 提供内部可变性.
fn demo_rc_refcell() {
    println!("\n--- Rc<RefCell<T>> (经典组合) ---");

    let shared = Rc::new(RefCell::new(String::from("hello")));

    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    // 每个通过 RefCell 修改它的内容
    clone1.borrow_mut().push_str(" world");

    println!("clone2 看到的值: {}", clone2.borrow());
    println!("引用计数: {}", Rc::strong_count(&shared));

    // 注意事项:
    // - borrow() 和 borrow_mut() 在运行时检查, 不能同时存在
    // - 违反借用规则会 panic (不是编译错误)
    // - 多线程场景换成 Arc<Mutex<T>> / Arc<RwLock<T>>
}

pub fn run() {
    demo_cell();
    demo_refcell();
    demo_rc_refcell();
}
