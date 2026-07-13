//! unsafe Rust: 裸指针 / Unsafe 块与函数 / FFI 概念.

// ===== 1. 裸指针: *const T 与 *mut T =====

/// 裸指针不遵守借用规则, 不自动释放, 可为 null.
fn demo_raw_pointers() {
    println!("--- 裸指针 (*const / *mut) ---");

    let mut num = 5;

    // 从引用创建裸指针 (安全操作, 不需要 unsafe)
    let r1: *const i32 = &num;
    let r2: *mut i32 = &mut num;

    println!("r1 指向的地址: {:p}", r1);
    println!("r2 指向的地址: {:p}", r2);

    // 解引用裸指针必须在 unsafe 块中
    unsafe {
        println!("*r1 = {}", *r1);
        *r2 = 10;
        println!("修改后 *r2 = {}", *r2);
    }

    // unsafe 不等于危险, 只是说"编译器, 这由我负责保证安全"
}

// ===== 2. unsafe 函数 =====

/// 声明 unsafe 函数: 调用者必须确保满足前置条件.
unsafe fn dangerous() {
    println!("在 unsafe 函数内部执行");
}

/// 演示 unsafe 块的几种用法.
fn demo_unsafe_block() {
    println!("\n--- unsafe 块 ---");

    // unsafe 块内可以做的五件事:
    // 1. 解引用裸指针
    // 2. 调用 unsafe 函数或方法
    // 3. 访问或修改可变静态变量
    // 4. 实现 unsafe trait
    // 5. 访问 union 的字段

    unsafe {
        dangerous();
    }
}

// ===== 3. 可变静态变量 =====

/// 静态变量: 固定地址, 'static 生命周期.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // 读写可变静态变量必须 unsafe
    }
}

/// 演示 static + unsafe.
fn demo_static() {
    println!("\n--- 可变静态变量 ---");

    add_to_count(3);
    unsafe {
        let count = COUNTER;
        println!("COUNTER = {}", count);
    }

    // 全局可变状态在多线程下是数据竞争隐患
    // 所以 Rust 要求 unsafe 来访问
}

// ===== 4. unsafe trait =====

/// 标记 trait: 声明实现者已确保线程安全.
/// Send/Sync 本身就是 unsafe trait, 只是编译器自动推导.
#[allow(dead_code)]
unsafe trait TrustMe {}

unsafe impl TrustMe for i32 {}

/// 演示 unsafe trait.
fn demo_unsafe_trait() {
    println!("\n--- unsafe trait ---");

    // unsafe trait 的意思是:
    // "实现者必须手动保证某些编译器无法验证的约束"
    // 标准库中 Send / Sync 就是 unsafe trait
    // 但编译器会自动为大多数类型推导, 你一般不需要手动实现

    println!("i32 实现了 TrustMe (unsafe trait)");
}

// ===== 5. FFI 概念 (Foreign Function Interface) =====

// extern 块声明外部函数 (通常来自 C 库).
unsafe extern "C" {
    // C 标准库的 abs 函数.
    fn abs(input: i32) -> i32;
}

/// 供 C 调用的函数.
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Rust 函数被 C 代码调用!");
}

/// 演示 FFI 概念.
fn demo_ffi() {
    println!("\n--- FFI (外部函数接口) ---");

    // 调用 C 标准库的 abs
    let x = -42;
    unsafe {
        println!("C abs({}) = {}", x, abs(x));
    }

    // extern "C" fn 可以:
    // - 从 Rust 调用 C 库函数
    // - 把 Rust 函数暴露给 C/C++ 调用
    // 实际项目常用: 绑定 OpenGL、SQLite、libgit2 等
}

pub fn run() {
    demo_raw_pointers();
    demo_unsafe_block();
    demo_static();
    demo_unsafe_trait();
    demo_ffi();
}
