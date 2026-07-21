//! 智能指针: Box<T> / Deref / Drop / Rc<T> / Arc<T>。
//!
//! ┌── 为什么需要"智能指针" ────────────────────────────────────┐
//! │                                                              │
//! │  Rust 的核心规则: 每个值有且仅有一个所有者 (owner)。           │
//! │  普通引用 &T 只是"借用", 不参与所有权。                       │
//! │                                                              │
//! │  这条规则保证了内存安全, 但也制造了几个"合法但做不到"的场景:  │
//! │                                                              │
//! │  1. 编译时大小未知 → Box<T> 把数据放堆上, 指针大小固定       │
//! │  2. 多个所有者共享同一份数据 → Rc/Arc 引用计数               │
//! │  3. 离开作用域自动释放资源 → Drop trait                      │
//! │  4. 自定义指针像普通引用一样用 → Deref trait                 │
//! │                                                              │
//! │  "智能"二字的含义: 它们不仅存储指针, 还附带额外行为           │
//! │  (自动释放、引用计数、解引用转换等), 所以比普通指针"聪明"。  │
//! │                                                              │
//! │  对比: Rust 没有 GC (垃圾回收), 但通过智能指针 + 所有权系统  │
//! │  实现了确定性的内存管理 — 何时释放是编译时确定的, 而非运行时 │
//! │  不确定地"等 GC 来收"。                                      │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::Arc;

// ======================================================================
//  Part 1: Box<T> — 堆分配, 解决"编译时大小未知"
// ======================================================================
//
//  问题:
//    Rust 必须在编译时知道每个类型的大小 (用于栈分配)。
//    但递归类型 (链表、树) 的大小理论上无限, 编译器拒绝编译:
//
//      enum List { Node(i32, List), Nil }
//      → error: recursive type has infinite size
//
//    类似地, trait object (dyn Trait) 的大小在编译时也不确定 —
//    不同实现者的结构体大小不同。
//
//  解决:
//    Box<T> 把 T 的数据分配到堆上, Box 本身只是一个固定大小的指针 (usize)。
//    编译器知道 Box<T> 的大小 (1 个指针), 不再关心 T 的实际大小。
//
//  典型场景:
//    • 递归类型 (链表、树、图节点)
//    • 大值不想放栈上 (栈空间 ~8MB, 大数组/大 struct 挪到堆)
//    • trait object: Box<dyn Trait> — 擦除具体类型, 只保留虚表指针
//    • FFI 中把所有权转交给 C 代码

/// 递归链表 — 不用 Box 编译器会拒绝。
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    /// Cons: 当前值 + 下一个节点的堆指针。
    /// Box<List> 的大小 = 1 个指针,
    /// 如果写成 Cons(i32, List) 则是无限递归 → 编译失败。
    Cons(i32, Box<List>),
    Nil,
}

fn demo_box() {
    println!("========== Box<T>: 堆分配 ==========\n");

    // ── 场景 1: 把值放堆上 ──
    println!("[场景 1] 堆上分配单个值");
    let b = Box::new(42);
    //        ^^^^^^^^ 在堆上分配 i32, 返回指向它的 Box<i32>
    //        所有权: 42 的所有者是 b, b 在栈上, 42 在堆上
    println!("  Box<i32> = {}, 堆地址 = {:p}", *b, b);
    //  *b: Deref 解引用, &i32 → 按值复制 (i32 是 Copy)
    //  &(*b) 会得到堆上 42 的地址
    println!("  大小: Box<i32> = {} 字节 (一个指针)", std::mem::size_of::<Box<i32>>());
    println!();

    // ── 场景 2: 递归类型 — 不用 Box 就编译不过 ──
    println!("[场景 2] 递归链表 (必须有 Box)");
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("  链表: {:?}", list);
    // 内存布局:
    //   栈上: list (Cons 变体标签 + i32 + 堆指针)
    //   堆上: 第二个 Cons + 第三个 Cons (各自分配)
    println!();

    // ── 场景 3: trait object ──
    println!("[场景 3] Box<dyn Trait> (trait object)");
    {
        trait Draw {
            fn draw(&self) -> String;
        }
        struct Circle {
            r: f64,
        }
        struct Square {
            side: f64,
        }
        impl Draw for Circle {
            fn draw(&self) -> String {
                format!("圆形(r={})", self.r)
            }
        }
        impl Draw for Square {
            fn draw(&self) -> String {
                format!("方形(边={})", self.side)
            }
        }

        // Box<dyn Draw>: 一个指针 + 一个虚表指针 (共 16 字节 on 64bit)
        // 不同实现者大小不同, 但 Box<dyn Draw> 大小统一
        let shapes: Vec<Box<dyn Draw>> = vec![
            Box::new(Circle { r: 5.0 }),
            Box::new(Square { side: 3.0 }),
        ];
        for shape in &shapes {
            println!("  {}", shape.draw());
        }
    }
    println!();

    // ── 场景 4: 把大数组移到堆上 ──
    println!("[场景 4] 大数组挪到堆上");
    {
        // [i32; 100_000] = 400KB, 放栈上可能爆栈
        // Box<[i32; 100_000]> = 8 字节 (指针), 数据在堆上
        let big = Box::new([0u8; 100_000]);
        println!("  堆上数组长度: {} 字节 (栈上只有 8 字节指针)", big.len());
        // big 离开作用域 → Drop → 释放堆上 100KB
    }
    println!();

    // [Box 小结]
    println!("── Box 小结 ──");
    println!("  问题:  编译时大小未知 / 栈空间不足");
    println!("  解决:  数据放堆, 指针固定大小");
    println!("  特征:  单一所有者, 离开作用域自动释放堆内存");
    println!("  线程:  Send + Sync (可跨线程传递)");
    println!();
}

// ======================================================================
//  Part 2: Deref trait — 让智能指针像普通引用一样使用
// ======================================================================
//
//  问题:
//    自定义指针类型 (如 MyBox<T>) 用起来很别扭: &x 拿到的是 &MyBox<T>,
//    不能直接传给需要 &T 的函数。每次都写 &(*x) 很繁琐。
//
//  解决:
//    实现 Deref trait (提供 deref() → &T), Rust 编译器会在以下场景
//    自动插入 deref() 调用 (解引用强制转换, Deref Coercion):
//      • &T ← &Box<T>      (一层转换)
//      • &str ← &String    (String 实现了 Deref<Target=str>)
//      • &str ← &Box<String> (链式: Box→String→str)
//      • 方法调用 (自动解引用找到方法)
//      • 函数参数传递时自动匹配目标类型
//
//  注意: Deref 不等于"任意转换"。编译器只在引用传递时自动转换,
//  不会在赋值、match 等场景下强制转换。

/// 自定义智能指针, 模拟 Box<T> 的行为。
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// 实现 Deref: *my_box 等价于 *(my_box.deref())
///
/// type Target = T 告诉编译器: 解引用后的目标类型是 T
/// deref() 返回 &T — 不可变借用内部数据
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回 MyBox.0 的引用
    }
}

fn demo_deref() {
    println!("========== Deref: 解引用强制转换 ==========\n");

    // ── 基础: * 操作符 ──
    println!("[基础] * 解引用");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    // *y 等价于 *(y.deref()) — 编译器自动插入 deref() 调用
    println!("  *MyBox(5) = {}", *y);
    println!();

    // ── Deref Coercion: 函数参数 ──
    println!("[场景 1] 函数参数自动转换");
    {
        fn greet(name: &str) {
            println!("  Hello, {}!", name);
        }

        let s = MyBox::new(String::from("Rust"));
        // greet 需要 &str, 我们传入 &MyBox<String>
        // Rust 自动解引用链: &MyBox<String> → &String → &str
        greet(&s);
        // 如果没有 Deref, 必须写:
        // greet(&(*s)[..]);  等价于 greet(&(*(s.deref()))[..]);
    }
    println!();

    // ── Deref Coercion: 方法调用 ──
    println!("[场景 2] 方法调用自动解引用");
    {
        let b = Box::new(String::from("hello"));
        // b.len(): 编译器沿着 &Box<String> → &String 找 len() 方法
        // String 有 len() → 调用成功
        println!("  Box<String>.len() = {}", b.len());
        // 实际上这里也发生了: b 是 Box<String>, 但 String 有 len()
    }
    println!();

    // ── DerefMut (可变解引用) ──
    println!("[场景 3] DerefMut: 可变解引用");
    {
        use std::ops::DerefMut;

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let mut mb = MyBox::new(String::from("hello"));
        // mb.push_str(" world") — 自动解引用找到 String::push_str
        mb.push_str(" world");
        println!("  MyBox<String> 修改后: {}", *mb);
    }
    println!();

    // [Deref 小结]
    println!("── Deref 小结 ──");
    println!("  问题:  自定义指针不能直接当 &T 用");
    println!("  解决:  实现 Deref, 编译器自动在引用传递时调用 deref()");
    println!("  关键:  不是任意转换, 只在 &T ← &Pointer 的方向上工作");
    println!("  提示:  Deref 让智能指针"看起来像"它包裹的类型");
    println!();
}

// ======================================================================
//  Part 3: Drop trait — 离开作用域时自动清理
// ======================================================================
//
//  问题:
//    Box/Rc/File/Lock 等类型持有了堆内存、文件句柄、锁等"资源"。
//    如果只是把内存标记为"未使用", 资源不会自动释放 (文件不会关闭、
//    锁不会释放、堆内存泄漏)。需要一段"清理代码"在值死亡时执行。
//
//  解决:
//    Drop trait: 值离开作用域时, Rust 自动调用 drop(&mut self)。
//    不需要手动写, 编译器保证调用。类似 C++ 的析构函数, 但是确定性的 —
//    你永远知道它什么时候执行 (变量作用域结束时)。
//
//  与其他语言的对比:
//    C++:   析构函数, 确定性, 但需要手动管理内存
//    Java:  finalize (已废弃), 不确定何时执行
//    Go:    defer, 函数级而非值级
//    Rust:  Drop, 确定性 + 自动, 编译器插入调用

/// 模拟数据库连接 — 需要显式关闭的资源。
struct DbConnection {
    name: String,
    connected: bool,
}

impl DbConnection {
    fn connect(name: &str) -> DbConnection {
        println!("  [构造] 连接数据库 '{}' ...", name);
        DbConnection {
            name: name.to_string(),
            connected: true,
        }
    }

    fn query(&self, sql: &str) {
        if self.connected {
            println!("  [查询] 执行: {}", sql);
        }
    }
}

impl Drop for DbConnection {
    /// 离开作用域时自动调用。
    ///
    /// &mut self → 可以修改/读取自己 (但通常只做清理)
    fn drop(&mut self) {
        if self.connected {
            println!("  [Drop] 断开数据库 '{}' — 连接已关闭", self.name);
            self.connected = false;
        }
    }
}

/// 模拟文件句柄 — 展示 Drop 的释放顺序。
struct OpenFile {
    path: String,
}

impl OpenFile {
    fn open(path: &str) -> OpenFile {
        println!("  [构造] 打开文件 '{}'", path);
        OpenFile {
            path: path.to_string(),
        }
    }
}

impl Drop for OpenFile {
    fn drop(&mut self) {
        println!("  [Drop] 关闭文件 '{}'", self.path);
    }
}

fn demo_drop() {
    println!("========== Drop: 自动清理 ==========\n");

    // ── 基本用法 ──
    println!("[基本] 自动调用 Drop");
    {
        let conn = DbConnection::connect("mydb");
        conn.query("SELECT * FROM users");
        // conn 在此离开作用域 → Drop::drop() 自动调用
        println!("  (即将离开作用域...)");
    }
    println!("  (作用域已结束)\n");

    // ── 释放顺序: 后创建先释放 ──
    println!("[顺序] Drop 按栈顺序执行 (后创建先释放)");
    {
        let _f1 = OpenFile::open("/tmp/a.txt");
        let _f2 = OpenFile::open("/tmp/b.txt");
        let _f3 = OpenFile::open("/tmp/c.txt");
        println!("  (作用域将结束 — Drop 顺序: c → b → a)");
    }
    println!("  (作用域已结束)\n");

    // ── 提前释放: std::mem::drop ──
    println!("[提前释放] std::mem::drop()");
    {
        let conn = DbConnection::connect("tempdb");
        // conn.drop(); ← 编译错误! 不能手动调用 drop()
        // 正确做法: std::mem::drop (标准库函数, 接受所有权并立即丢弃)
        std::mem::drop(conn);
        // conn 的所有权已移入 drop(), 此后不能再用 conn
        println!("  (conn 已提前释放)");
    }
    println!();

    // [Drop 小结]
    println!("── Drop 小结 ──");
    println!("  问题:  持有资源的类型需要清理 (关闭文件、释放锁、断开连接)");
    println!("  解决:  实现 Drop, 编译器保证在值死亡时调用 drop(&mut self)");
    println!("  规则:  不能手动调用 drop(), 用 std::mem::drop() 提前释放");
    println!("  顺序:  变量按创建的反序释放 (后进先出, 类似栈)");
    println!("  对比:  确定性析构 — 不是'等 GC', 而是'现在立刻'");
    println!();
}

// ======================================================================
//  Part 4: Rc<T> — 单线程引用计数, 共享所有权
// ======================================================================
//
//  问题:
//    普通所有权规则: 一个值只能有一个 owner。
//    但有些场景天然需要"一群人共用一份数据":
//      • 图结构: 多个节点可以指向同一个节点
//      • GUI: 多个组件共享同一个数据模型
//      • 游戏: 多个实体引用同一份资源 (纹理、音效)
//      • 编译器: AST 中多个位置引用同一个符号表条目
//
//    &T 引用做不到 — 它不参与所有权, 无法保证数据还活着。
//    "谁来负责释放?" 在单 owner 模型下没有答案。
//
//  解决:
//    Rc<T> = Reference Counted (引用计数)。
//    每次 Rc::clone() → 引用计数 +1。
//    每个 Rc 离开作用域 → 计数 -1。
//    最后一个 Rc 离开 → 计数归零 → 自动释放 T。
//
//    本质上: 把单 owner 扩展为"计数器 owner" — 谁都不独有,
//    但计数器集体决定数据何时释放。
//
//  ⚠️ 限制: Rc<T> 只能在单线程中使用 (!Send + !Sync)。
//    因为引用计数用普通的 usize 操作, 不是原子的。
//    同时两个线程 clone 同一个 Rc → 计数可能丢失 → double-free。
//    多线程场景用 Arc<T> (见 Part 5)。

fn demo_rc() {
    println!("========== Rc<T>: 单线程引用计数 ==========\n");

    // ── 基础: 共享不可变数据 ──
    println!("[基础] 共享不可变数据");
    {
        let shared = Rc::new(String::from("公共配置"));
        // shared: Rc<String>, 现在计数 = 1

        println!("  初始: 计数 = {}, 数据 = '{}'",
            Rc::strong_count(&shared), shared);

        {
            let clone1 = Rc::clone(&shared);
            // Rc::clone: 只增加计数, 不复制 String 内容 (浅拷贝)
            println!("  clone1 后: 计数 = {}", Rc::strong_count(&shared));

            let clone2 = Rc::clone(&shared);
            println!("  clone2 后: 计数 = {}", Rc::strong_count(&shared));

            println!("  三个 Rc 指向同一份 '{}'", clone1);
        } // clone1, clone2 离开作用域, 计数 -2

        println!("  子作用域结束后: 计数 = {}", Rc::strong_count(&shared));
    } // shared 离开作用域, 计数归零 → String 释放
    println!();

    // ── 场景: 图结构 — 多个节点指向同一个 ──
    println!("[场景] 有向图 (多节点指向同一节点)");
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            next: Vec<Rc<Node>>, // Vec<Rc<Node>>: 多个后继可能指向同一节点
        }

        // a ↓
        // b ← c (c 指向 b, a 也指向 b)
        let a = Rc::new(Node {
            value: 1,
            next: Vec::new(),
        });
        let b = Rc::new(Node {
            value: 2,
            next: Vec::new(),
        });
        let mut c = Node {
            value: 3,
            next: Vec::new(),
        };

        // a 和 c 都指向 b
        Rc::get_mut(&mut Rc::clone(&a)).unwrap(); // 无操作, 仅演示 &mut

        // 用 Rc 包裹 a 才能放进 Vec<Rc<Node>>
        // (这里直接用 b 的 Rc)

        c.next.push(Rc::clone(&b)); // c → b
        // a 也和 b 共享 (但这里 a.next 为空)

        println!("  a 引用计数: {}", Rc::strong_count(&a));
        println!("  b 引用计数: {} (被 c 引用)", Rc::strong_count(&b));
        println!("  c.next: 指向 b (共享)");
    }
    println!();

    // ── 循环引用与 Weak<T> ──
    println!("[问题] 循环引用 — 内存泄漏风险");
    {
        // ⚠️ 如果 Rc 互相引用 → 计数永远不为 0 → 内存泄漏
        // 解决: Weak<T> — 不增加 strong_count, 用前需 upgrade() 判断是否存活

        #[derive(Debug)]
        struct TreeNode {
            value: i32,
            parent: Option<Weak<TreeNode>>,   // Weak: 不增加计数, 避免循环
            children: Vec<Rc<TreeNode>>,       // Rc: 拥有子节点的所有权
        }

        let root = Rc::new(TreeNode {
            value: 1,
            parent: None,
            children: Vec::new(),
        });

        let child = Rc::new(TreeNode {
            value: 2,
            // parent: Rc::downgrade → Weak<TreeNode>, 不增加 strong_count
            parent: Some(Rc::downgrade(&root)),
            children: Vec::new(),
        });

        println!("  root strong_count = {} (子节点不增加, 因为用 Weak)",
            Rc::strong_count(&root));
        println!("  child strong_count = {}",
            Rc::strong_count(&child));

        // Weak::upgrade() → Option<Rc<T>>
        // 如果原 Rc 还活着 → Some; 如果已被释放 → None
        if let Some(parent) = child.parent.as_ref().unwrap().upgrade() {
            println!("  child.parent 仍存活, value = {}", parent.value);
        }

        println!("  ⚠️ 没有 Weak 会怎样?  root → child (Rc)  child → root (Rc)");
        println!("      互相持有 → 计数永远 ≥ 1 → 内存泄漏");
    }
    println!();

    // ── Rc + RefCell (组合模式) ──
    println!("[提示] Rc<RefCell<T>> — 共享可变数据");
    println!("  Rc 提供不可变共享, RefCell 提供运行时可变检查");
    println!("  → 详见 interior_mutability 模块\n");

    // [Rc 小结]
    println!("── Rc 小结 ──");
    println!("  问题:  单 owner 规则下, 如何让多个部分共享同一份数据?");
    println!("  解决:  引用计数 — clone() 只 +1 计数, 最后一个释放数据");
    println!("  限制:  单线程 (!Send, !Sync), 引用计数非原子操作");
    println!("  注意:  循环引用 → 内存泄漏, 用 Weak<T> 打破循环");
    println!();
}

// ======================================================================
//  Part 5: Arc<T> — 多线程原子引用计数
// ======================================================================
//
//  问题:
//    Rc 的引用计数不是原子操作, 不能在多线程中使用。
//    而多线程共享数据是非常常见的需求:
//      • 多线程处理同一份配置
//      • 线程池共享缓存
//      • 并发 HTTP 服务共享路由表
//      • 多 worker 共享只读数据
//
//  解决:
//    Arc<T> = Atomic Reference Counted。
//    和 Rc 的唯一区别: 引用计数用原子指令 (AtomicUsize) 操作。
//    原子操作有轻微性能开销, 但 Rc 不具备此能力。
//
//  Arc 实现了 Send + Sync (当 T: Send + Sync 时),
//  可以安全地在线程间共享。

fn demo_arc() {
    println!("========== Arc<T>: 多线程引用计数 ==========\n");

    // ── 基础 ──
    println!("[基础] Arc 和 Rc 用法完全一样, 但线程安全");
    {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        println!("  初始: 计数 = {}, 数据 = {:?}",
            Arc::strong_count(&data), data);

        let clone = Arc::clone(&data);
        println!("  clone 后: 计数 = {}", Arc::strong_count(&data));

        // 基础操作: Arc 实现了 Deref, 可以透明使用内部 Vec
        println!("  第一个元素: {}", data[0]);
        println!("  长度: {}", data.len());

        drop(clone);
        println!("  clone 释放后: 计数 = {}", Arc::strong_count(&data));
    }
    println!();

    // ── 场景: 多线程共享配置 ──
    println!("[场景] 多线程共享只读配置");
    {
        let config = Arc::new(String::from("debug_mode=true"));

        // 多个线程各拿到一个 Arc 的 clone
        let handles: Vec<_> = (0..3)
            .map(|i| {
                let cfg = Arc::clone(&config); // 计数 +1
                // cfg 所有权移入线程 (move)
                std::thread::spawn(move || {
                    // Arc 实现了 Deref, *cfg 拿到 &String
                    println!("  线程 {} 读取: {}", i, *cfg);
                    // cfg 离开作用域 → 计数 -1
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        println!("  所有线程结束后: 计数 = {} (只剩原始 config)",
            Arc::strong_count(&config));
    }
    println!();

    // ── 对比 Rc vs Arc ──
    println!("[对比] Rc vs Arc");
    println!("  ┌─────────┬──────────────┬──────────────┐");
    println!("  │         │ Rc<T>        │ Arc<T>       │");
    println!("  ├─────────┼──────────────┼──────────────┤");
    println!("  │ 计 数   │ usize        │ AtomicUsize  │");
    println!("  │ 线 程   │ 单线程       │ 多线程       │");
    println!("  │ Send    │ ✗            │ ✓ (T: Send)  │");
    println!("  │ Sync    │ ✗            │ ✓ (T: Sync)  │");
    println!("  │ 性 能   │ 无开销       │ 原子指令开销 │");
    println!("  │ 用 法   │ 单线程共享   │ 跨线程共享   │");
    println!("  └─────────┴──────────────┴──────────────┘");
    println!();

    // [Arc 小结]
    println!("── Arc 小结 ──");
    println!("  问题:  多线程如何共享数据所有权?");
    println!("  解决:  Arc = Rc + 原子计数, 线程安全");
    println!("  组合:  Arc<Mutex<T>> (可变共享) 或 Arc<RwLock<T>> (读多写少)");
    println!("  开销:  原子操作略慢于普通操作, 但通常可接受");
    println!();
}

// ======================================================================
//  总结: 五大智能指针对比
// ======================================================================

pub fn run() {
    demo_box();
    demo_deref();
    demo_drop();
    demo_rc();
    demo_arc();

    println!("══════════ 五大智能指针总结 ══════════\n");

    println!("  每种智能指针解决一种"所有权规则做不到"的问题:\n");

    println!("  ┌────────┬──────────────────┬───────────────────────────────┐");
    println!("  │ 类型   │ 解决什么问题      │ 一句话                        │");
    println!("  ├────────┼──────────────────┼───────────────────────────────┤");
    println!("  │ Box    │ 编译时不知道大小  │ 把数据放堆上, 指针大小固定    │");
    println!("  │ Deref  │ 自定义指针不好用  │ 自动把 &Pointer 转为 &T       │");
    println!("  │ Drop   │ 资源需要清理      │ 离开作用域时自动执行清理代码  │");
    println!("  │ Rc     │ 需要多所有者      │ 引用计数, 最后一个释放 (单线程)│");
    println!("  │ Arc    │ 跨线程多所有者    │ 原子引用计数, 线程安全        │");
    println!("  └────────┴──────────────────┴───────────────────────────────┘\n");

    println!("  常用组合模式:");
    println!("    Rc<RefCell<T>>   → 单线程共享可变数据");
    println!("    Arc<Mutex<T>>    → 多线程共享可变数据");
    println!("    Arc<RwLock<T>>   → 多线程读多写少共享");
    println!("    Box<dyn Trait>   → 类型擦除 + 堆分配\n");

    println!("  核心思想: Rust 不是'没有 GC 所以写起来难受',");
    println!("  而是'用智能指针精确控制所有权的转移和释放'。");
    println!("  每个智能指针都是一份清晰的文档, 告诉读代码的人:");
    println!("  '这些数据是怎么被拥有、共享和释放的'。");
}
