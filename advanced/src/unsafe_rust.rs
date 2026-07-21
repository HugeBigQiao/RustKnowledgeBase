//! unsafe Rust: 为什么需要 / 裸指针 / 胖瘦指针 / 所有权 / 常用场景。
//!
//! ┌── 为什么需要 unsafe ────────────────────────────────────────┐
//! │                                                              │
//! │  Rust 的所有权 + 借用检查器保证了内存安全, 但它能验证的范围   │
//! │  是有限的。以下场景编译器"管不了"或"管了反而碍事":           │
//! │                                                              │
//! │  1. 调用其他语言 (C/C++) 的函数                              │
//! │     — Rust 不知道 C 代码会不会破坏内存, 只能信任你           │
//! │  2. 操作硬件 (MMIO、DMA、中断向量)                           │
//! │     — 硬件寄存器在特定内存地址, 编译器无法推理                │
//! │  3. 实现编译器无法表达的数据结构                              │
//! │     — 双向链表、无锁队列等, 借用检查器无法验证                │
//! │  4. 性能极致优化                                              │
//! │     — 绕过边界检查、手动 SIMD 等                             │
//! │                                                              │
//! │  unsafe 不是"关闭编译器"的开关, 而是"我来担保"的声明:        │
//! │  "这部分代码编译器你没法检查, 但我保证它遵守 Rust 规则。"    │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘
//!
//! ┌── unsafe 在编译层面发生了什么 ──────────────────────────────┐
//! │                                                              │
//! │  unsafe 块内, 编译器:                                        │
//! │    ✓ 仍然进行类型检查 (不能把 i32 赋值给 String)             │
//! │    ✓ 仍然检查语法 (分号、花括号等都照常)                     │
//! │    ✓ 仍然做所有权分析和借用检查 (除了裸指针操作)             │
//! │    ✓ 仍然优化代码                                            │
//! │    ✗ 不再阻止你解引用裸指针                                  │
//! │    ✗ 不再阻止你调用 unsafe 函数                              │
//! │    ✗ 不再阻止你访问可变静态变量                              │
//! │    ✗ 不再阻止你实现 unsafe trait                             │
//! │    ✗ 不再阻止你访问 union 字段                               │
//! │                                                              │
//! │  这五种能力称为 unsafe 的"五大超能力"。unsafe 只解锁了这     │
//! │  五项, 所有其他 Rust 安全机制照常工作。                      │
//! │                                                              │
//! │  与 C/C++ 的对比:                                            │
//! │    C/C++ 整个语言默认 unsafe (到处都可能是 UB)               │
//! │    Rust  系统默认 safe, unsafe 是明确标记的局部区域          │
//! │    这意味着出 bug 时, 搜索 "unsafe" 就能定位嫌疑代码         │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘

// ======================================================================
//  Part 1: 裸指针 — *const T 与 *mut T
// ======================================================================
//
// ┌── 什么是裸指针 ──────────────────────────────────────────────┐
// │                                                              │
// │  裸指针 (raw pointer) 是"最纯粹"的指针:                        │
// │    只是一个内存地址, 没有所有权, 没有生命周期, 不参与借用检查│
// │                                                              │
// │  两种裸指针:                                                  │
// │    *const T  — 不允许通过它修改指向的值                       │
// │    *mut T    — 允许通过它修改指向的值                         │
// │                                                              │
// │  裸指针的特征:                                                │
// │    • 可以为 null (和 C 的 NULL 一样)                          │
// │    • 不保证指向有效内存 (可以悬垂)                            │
// │    • 不自动释放                                              │
// │    • 同一个位置可以同时存在多个 *mut T (无视借用规则)         │
// │    • 创建裸指针是安全的 (不需要 unsafe), 只有解引用才需要     │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘
//
// ┌── 胖指针 vs 瘦指针 ──────────────────────────────────────────┐
// │                                                              │
// │  瘦指针 (thin pointer) = 8 字节 (64位系统):                   │
// │    只存一个内存地址。                                         │
// │    例子: *const i32, *mut String, &i32, Box<f64>             │
// │                                                              │
// │  胖指针 (fat pointer) = 16 字节 (64位系统):                   │
// │    存地址 + 额外元数据。有两种:                               │
// │                                                              │
// │    1. 切片引用 &[T] / &str:                                   │
// │       地址 (8B) + 长度 (8B) = 16B                             │
// │       → 长度存在指针里, 和指针一起传递                        │
// │                                                              │
// │    2. trait object &dyn Trait / Box<dyn Trait>:              │
// │       数据地址 (8B) + 虚表指针 (8B) = 16B                     │
// │       → 虚表指针在运行时告诉 Rust 调用哪个方法实现            │
// │                                                              │
// │  关键区别:                                                    │
// │    • 裸指针 *const/*mut 永远是瘦指针 (对 Sized 类型)          │
// │    • *const [T] / *mut [T] 是胖指针 (裸指针对 unsized 类型)  │
// │    • 从 &[T] 转 *const [T] 会保留长度信息                     │
// │                                                              │
// │  验证大小:                                                    │
// │    size_of::<*const i32>()  = 8  (瘦)                        │
// │    size_of::<&[i32]>()      = 16 (胖, 地址+长度)             │
// │    size_of::<&dyn Display>() = 16 (胖, 地址+虚表)            │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘
//
// ┌── 为什么需要裸指针 ─────────────────────────────────────────┐
// │                                                              │
// │  1. FFI: C 代码只能用裸指针传数据, &T 带生命周期 C 理解不了  │
// │  2. 所有权共享: 借用检查器不允许同时存在多个 &mut T,         │
// │     但某些数据结构 (如双向链表) 确实需要                      │
// │  3. 性能: 裸指针不经过边界检查、不需要维护借用计数           │
// │  4. 自定义内存管理: 自己 alloc/dealloc 只能用裸指针          │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘
//
// ┌── 裸指针的生命周期与所有权 ──────────────────────────────────┐
// │                                                              │
// │  裸指针和引用 &T 的根本区别:                                  │
// │                                                              │
// │          &T / &mut T          *const T / *mut T               │
// │  ──────  ───────────────────  ────────────────────────        │
// │  所有   编译器保证始终有效    无 — 可以悬垂                   │
// │  生命   有生命周期标注       无 — 编译器不追踪               │
// │  借用   编译时检查          完全不管                         │
// │  释放   Drop 自动调用        不会 — 需要手动 free             │
// │  null   不可能为 null        可以为 null                      │
// │  Send   大多数自动实现       不会自动实现 (需 unsafe impl)    │
// │                                                              │
// │  关键: 裸指针指向的数据何时释放、谁负责释放 —                 │
// │        编译器都不管, 全由程序员负责。                         │
// │        这就是为什么"创建裸指针安全, 解引用需要 unsafe"。     │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘

fn demo_raw_pointers() {
    println!("========== 裸指针: *const / *mut ==========\n");

    // ── 创建裸指针 (安全操作) ──
    println!("[创建] 从引用创建裸指针");
    let mut num = 5;
    let r1: *const i32 = &num;      // &i32 → *const i32 (安全)
    let r2: *mut i32 = &mut num;    // &mut i32 → *mut i32 (安全)

    println!("  *const i32 地址: {:p}", r1);
    println!("  *mut i32   地址: {:p}", r2);
    println!("  (两个指针指向同一地址 — 无视借用规则)\n");

    // ── 解引用裸指针 (必须 unsafe) ──
    // 所有权: 裸指针不拥有 num, num 的所有者是栈帧
    //         解引用只是"读/写这个地址的内容", 不改变所有权
    println!("[解引用] 必须 unsafe 块内");
    unsafe {
        println!("  *r1 (const 读)  = {}", *r1);
        *r2 = 10;                       // 通过 *mut 写入
        // 所有权: num 仍在此处, *r2 = 10 等价于 num = 10
        println!("  *r2 (mut 写后)   = {}", *r2);
        println!("  num 本身         = {}", num);
        // num 和 *r2 是同一块内存 → 值一致
    }
    println!();

    // ── 裸指针可以为 null ──
    println!("[null] 裸指针可以为 null (引用不能)");
    let null_ptr: *const i32 = std::ptr::null();
    println!("  null_ptr: {:?}", null_ptr); // 0x0000000000000000
    // unsafe { println!("{}", *null_ptr); } ← 解引用 null → 未定义行为!
    println!("  (解引用 null 是未定义行为, 类似 C 的 segfault)\n");

    // ── 裸指针没有生命周期 ──
    println!("[生命周期] 裸指针不追踪有效期 — 可以悬垂");
    let dangling: *const i32;
    {
        let temp = 42;
        dangling = &temp; // 裸指针指向 temp
        println!("  作用域内: *dangling = {}", unsafe { *dangling });
    } // temp 被释放, dangling 变成悬垂指针
    // ⚠️ 编译通过, 但下面这行是未定义行为:
    // unsafe { println!("{:?}", *dangling); }
    println!("  作用域外: dangling 已悬垂, 解引用 = 未定义行为\n");

    // ── 胖/瘦指针大小验证 ──
    println!("[大小] 瘦指针 vs 胖指针 (64位系统)");
    println!("  *const i32       = {} 字节 (瘦 — 仅地址)", std::mem::size_of::<*const i32>());
    println!("  *const [i32]     = {} 字节 (胖 — 地址+长度)", std::mem::size_of::<*const [i32]>());
    println!("  &i32             = {} 字节 (瘦)", std::mem::size_of::<&i32>());
    println!("  &[i32]           = {} 字节 (胖)", std::mem::size_of::<&[i32]>());
    println!("  &str             = {} 字节 (胖)", std::mem::size_of::<&str>());
    println!("  Box<dyn Fn()>    = {} 字节 (胖 — 地址+虚表)", std::mem::size_of::<Box<dyn Fn()>>());

    // [裸指针小结]
    println!("\n── 裸指针小结 ──");
    println!("  特点: 无所有权 / 无生命周期 / 无借用检查 / 可为 null");
    println!("  创建: 安全 (不需要 unsafe)");
    println!("  解引用: 必须 unsafe — '编译器不检查, 我负责安全'");
    println!("  用途: FFI (C 互操作) / 自定义数据结构 / 性能关键路径");
    println!();
}

// ======================================================================
//  Part 2: unsafe 块与 unsafe 函数 — 语法与编译行为
// ======================================================================

/// unsafe 函数: 调用者必须用 unsafe { } 包裹调用。
///
/// 声明 unsafe fn 的意思是:
///   "这个函数有编译器无法验证的前置条件, 调用者需要自行保证。"
unsafe fn dangerous_op(ptr: *const i32) -> i32 {
    // 函数体内可以写 unsafe 操作, 不需要再加 unsafe 块
    // (整个函数体隐式 unsafe)
    if ptr.is_null() {
        return 0; // 防御性检查: 拒绝 null
    }
    *ptr // 解引用裸指针 — 这在普通 fn 里不行
}

fn demo_unsafe_block() {
    println!("========== unsafe 块 / unsafe 函数 ==========\n");

    // ── unsafe 块的语法 ──
    println!("[语法] unsafe 块的写法");
    let x = 42;
    unsafe {
        // 花括号内就是 unsafe 作用域
        // 所有普通 Rust 语法照常: 类型检查、借用检查、生命周期...
        // 只是额外允许那五种操作
        let val = dangerous_op(&x);
        println!("  dangerous_op(&42) = {}", val);
    }
    // 离开 unsafe 块后, 五种能力收回, 一切恢复安全检查
    println!();

    // ── 编译层面: unsafe 不关闭检查 ──
    println!("[编译] unsafe 块内依然有类型检查");
    unsafe {
        let a: i32 = 10;
        // let b: String = a; ← 类型不匹配, 编译错误!
        println!("  i32 = {}, 不能赋值给 String (类型检查照常)", a);
    }
    println!();

    // ── 借用检查在 unsafe 块内仍然有效 ──
    println!("[编译] 借用检查依旧 (普通引用, 非裸指针)");
    {
        let mut s = String::from("hello");
        let r1 = &s;
        unsafe {
            println!("  r1 = '{}'", r1);
            // let r2 = &mut s; ← 编译错误! 已有不可变借用
            // 借用检查器不管你是不是在 unsafe 块里 — 它只管 &T / &mut T
        }
    }
    println!("  (&T / &mut T 的借用规则在 unsafe 内不变)\n");

    // ── unsafe trait ──
    println!("[unsafe trait] Send / Sync 就是 unsafe trait");
    {
        // Send: 标记"此类型可以安全地在线程间转移所有权"
        // Sync: 标记"此类型的引用可以安全地在线程间共享"
        //
        // 它们是 unsafe trait, 因为编译器无法自动验证:
        //   "这个类型内部真的没有数据竞争? 真的线程安全?"
        //
        // 大多数类型由编译器自动实现 Send/Sync。
        // 只有包含裸指针、Cell 等的类型需要手动 unsafe impl。
        println!("  Send/Sync 是 unsafe trait — 编译器自动推导安全类型");
        println!("  手动 impl 需要 unsafe: '我保证这个类型线程安全'");
    }
    println!();
}

// ======================================================================
//  Part 3: 可变静态变量 — 全局可变状态
// ======================================================================

/// 全局可变变量: 'static 生命周期, 固定地址。
/// 读写必须在 unsafe 块内 — 因为多线程下是数据竞争。
static mut GLOBAL_COUNTER: u32 = 0;
//          ^^^ mut 声明为可变

fn add_to_counter(inc: u32) {
    unsafe {
        GLOBAL_COUNTER += inc;
        // 所有权: 静态变量不属于任何函数, 程序级别的全局所有
        // 生命周期: 'static — 从程序启动到退出一直存活
    }
}

fn demo_static() {
    println!("========== 可变静态变量 ==========\n");

    println!("[全局可变状态] static mut GLOBAL_COUNTER = 0");

    add_to_counter(5);
    add_to_counter(3);

    unsafe {
        println!("  当前值: {}", GLOBAL_COUNTER);
        println!("  地址固定: {:p}", std::ptr::addr_of!(GLOBAL_COUNTER));
    }

    println!("\n  为什么需要 unsafe:");
    println!("    全局可变 + 多线程 = 数据竞争 (典型 UB)");
    println!("    unsafe 提醒你: '这里可能有多线程问题, 你确定?'");
    println!("    实际项目用 AtomicU32 / Mutex<u32> 代替 static mut\n");
}

// ======================================================================
//  Part 4: FFI (Foreign Function Interface) — 跨语言互操作
// ======================================================================

// extern 块: 声明来自 C 库的函数。
// unsafe extern "C": edition 2024 要求显式 unsafe
unsafe extern "C" {
    // C 标准库的 abs 函数 — Rust 不知道它的实现是否安全
    fn abs(input: i32) -> i32;
}

/// 暴露给 C 调用的 Rust 函数。
/// #[unsafe(no_mangle)]: 保留函数名 (不被 Rust 内部改名)
/// extern "C": 用 C 的 ABI (函数调用约定)
#[unsafe(no_mangle)]
pub extern "C" fn add_from_c(a: i32, b: i32) -> i32 {
    a + b
}

fn demo_ffi() {
    println!("========== FFI: 调用 C 代码 ==========\n");

    println!("[调用 C 标准库]");
    unsafe {
        println!("  C abs(-42) = {}", abs(-42));
        // 所有权: C 函数不接受也不返回 Rust 所有权
        //        i32 是 Copy 类型, 传值就是复制
    }
    println!();

    println!("[暴露 Rust 函数给 C]");
    println!("  pub extern \"C\" fn add_from_c(a, b) → {}", add_from_c(3, 4));
    println!();

    println!("  FFI 常见场景:");
    println!("    • 调用 SQLite / OpenGL / libgit2 等 C 库");
    println!("    • Python 调用 Rust (PyO3 内部用 FFI)");
    println!("    • 嵌入式开发 (操作硬件寄存器)");
    println!("    • 操作系统内核开发");
    println!();
}

// ======================================================================
//  总结
// ======================================================================

pub fn run() {
    println!("══════════ unsafe Rust 全面讲解 ══════════\n");

    demo_raw_pointers();
    demo_unsafe_block();
    demo_static();
    demo_ffi();

    println!("══════════ unsafe 核心要点 ══════════\n");

    println!("  1. unsafe = \"编译器无法验证, 我来保证\"");
    println!("     NOT = \"关闭所有安全检查\"\n");

    println!("  2. unsafe 只解锁五种能力:");
    println!("     解引用裸指针 / 调用 unsafe 函数 / 访问 static mut");
    println!("     实现 unsafe trait / 访问 union 字段\n");

    println!("  3. unsafe 块内: 类型检查、借用检查、生命周期仍生效");
    println!("     (只对裸指针放行, 普通 &T 照查不误)\n");

    println!("  4. 裸指针 vs 引用:");
    println!("     引用 = 编译器保证安全 (所有/生命/借用)");
    println!("     裸指针 = 你保证安全 (可为 null, 可悬垂, 无自动释放)\n");

    println!("  5. 胖指针 vs 瘦指针:");
    println!("     瘦 = 地址 8B (*const T, &T, Box<T>)");
    println!("     胖 = 地址+长度 16B (&[T], &str) 或 地址+虚表 16B (&dyn Trait)\n");

    println!("  6. 实际原则:");
    println!("     • 尽量不用 unsafe — 标准库已封装了安全的抽象");
    println!("     • 必须用时, 把 unsafe 包围在最小范围内");
    println!("     • 用安全的 API 包装 unsafe 实现 (如 Vec 内部用裸指针)");
    println!("     • unsafe 块外应该保证任何输入都不会触发 UB");
}
