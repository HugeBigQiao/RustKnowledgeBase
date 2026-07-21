//! 内部可变性: Cell<T> / RefCell<T> / Rc<RefCell<T>> 模式。
//!
//! ┌── 为什么需要"内部可变性" ──────────────────────────────────┐
//! │                                                              │
//! │  Rust 的核心规则: 共享 XOR 可变                              │
//! │    • 同一时刻, 一个值要么被"多个不可变引用"共享              │
//! │    •            要么被"一个可变引用"独占修改                  │
//! │    • 两者不能同时存在                                       │
//! │                                                              │
//! │  这条规则在编译时保证了内存安全, 但也阻挡了一些合法模式:     │
//! │                                                              │
//! │  1. 我有一个不可变引用, 但内部某个字段我想改                 │
//! │     → 计数器、缓存、日志开关等                              │
//! │  2. 我有多个所有者 (Rc), 它们共享同一份数据,                 │
//! │     但都想修改这份数据                                      │
//! │     → GUI 多个组件共享数据模型、图遍历标记                  │
//! │  3. 测试中需要 mock 某个功能, 但被测代码要求 &self           │
//! │     → mock 对象需要在看似"不可变"的方法里修改内部计数器     │
//! │                                                              │
//! │  "内部可变性"就是: 对外看起来是不可变借用 (&self),           │
//! │   内部实际上可以修改数据。                                   │
//! │                                                              │
//! │  关键转换: 编译时借用检查 → 运行时借用检查                  │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘

use std::cell::{Cell, RefCell};
use std::rc::Rc;

// ======================================================================
//  Part 1: Cell<T> — 适合 Copy 类型的内部可变性
// ======================================================================
//
// ┌── Cell<T> 是什么 ────────────────────────────────────────────┐
// │                                                              │
// │  Cell<T> 包装类型 T, 提供 get() 和 set() 方法。              │
// │  即使 Cell 本身是"不可变"绑定 (let x = Cell::new(...)),      │
// │  也能通过 x.set(new_val) 修改内部值。                         │
// │                                                              │
// │  原理: Cell 不返回内部数据的引用, 而是:                       │
// │    get()  → 复制整个 T 返回 (所以 T 必须 Copy)                │
// │    set(v) → 用新值 v 替换内部旧值 (旧值被 drop)              │
// │                                                              │
// │  因为永远不暴露 &T 或 &mut T, 所以不会出现"有人持有引用     │
// │  时被修改"的问题 → 编译检查通过, 不需要 unsafe。             │
// │                                                              │
// │  ⚠️ 限制: T 必须是 Copy 类型。因为 get() 复制整个值。        │
// │     如果是 String, get() 没法复制 (String: !Copy)。           │
// │                                                              │
// │  所有权分析:                                                  │
// │    Cell::new(v) → v 移入 Cell, Cell 拥有 v                    │
// │    Cell::get()  → Copy v 的副本返回, Cell 仍拥有原 v          │
// │    Cell::set(w) → 旧值 v 被 drop, w 移入 Cell, Cell 拥有 w   │
// │    Cell 被 drop  → 内部值被 drop                             │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘
//
// ┌── 什么时候用 Cell ──────────────────────────────────────────┐
// │                                                              │
// │  • 计数值 (点击次数、请求数、重试数)                          │
// │  • 布尔标志 (is_dirty, is_loading)                           │
// │  • 小型状态机的 Copy 状态                                    │
// │  • 性能计数器 (get/set 零开销, 就是直接读写)                 │
// │                                                              │
// │  不要用 Cell 的场景:                                          │
// │  • T 不是 Copy (用 RefCell)                                  │
// │  • 需要对内部数据做部分修改 (用 RefCell)                     │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘

fn demo_cell() {
    println!("========== Cell<T>: Copy 类型内部可变 ==========\n");

    // ── 基础用法 ──
    println!("[基础] Cell 的 get/set");
    {
        let counter = Cell::new(0);
        // counter: 不可变绑定, 但内部值可以改!

        println!("  初始: {}", counter.get());

        counter.set(10);
        println!("  set(10) 后: {}", counter.get());

        counter.set(counter.get() + 1); // 读-改-写三步
        println!("  自增后: {}", counter.get());
    }
    println!();

    // ── 对比: 普通变量不能这样 ──
    println!("[对比] 没有 Cell 时");
    {
        let x = 0;
        // x = 10;  ← 编译错误: 不可变绑定不能赋值
        // 解决方法: let mut x = 0; → 但有时你不能用 mut
        //           (比如 struct 字段, 或共享引用场景)
        println!("  let x = 0; x = 10; ← 编译错误 (需要 mut)");
    }
    println!();

    // ── 场景: 不可变引用下修改内部值 ──
    println!("[场景] 通过共享引用修改 Cell 内容");
    {
        struct Counter {
            value: Cell<u32>, // 注意: 不是 u32, 是 Cell<u32>
        }

        impl Counter {
            fn increment(&self) {
                // &self 是不可变引用, 但 Cell 允许 set!
                self.value.set(self.value.get() + 1);
            }

            fn count(&self) -> u32 {
                self.value.get()
            }
        }

        let c = Counter {
            value: Cell::new(0),
        };
        let r1 = &c;
        let r2 = &c; // 多个不可变引用可以共存
        r1.increment(); // r1: &Counter 能修改内部 Cell
        r2.increment(); // r2: &Counter 也能修改
        println!("  两个 &Counter 各自 increment → 最终: {}", c.count());
        // 所有权: c 拥有 Counter, Counter 拥有 Cell<u32>, Cell 拥有 u32
        //         没有借用冲突: Cell 不暴露 &u32 / &mut u32
    }
    println!();

    // ── 所有权流转示例 ──
    println!("[所有权] Cell 内值的所有权流转");
    {
        let c = Cell::new(String::from("hello"));
        //         ^^^^^^^^^^^^^^^^^^^^^^ String 所有权移入 Cell
        //         String: !Copy, 但 Cell 仍能存 (只是 get 不能用)

        // c.get(); ← 编译错误: String 不是 Copy, get() 没法复制

        // 但可以 replace / take:
        let old = c.replace(String::from("world"));
        // 所有权: 旧值 move 到 old, 新值 move 进 Cell
        println!("  旧值: '{}', 当前: '{}'", old, c.take());
        // take(): 取出内部值并留下 Default::default() (这里 String default = "")

        // 也可以通过 into_inner 取出所有权:
        let c2 = Cell::new(42);
        let val: i32 = c2.into_inner();
        // into_inner: Cell 被消耗, 内部值所有权移出
        println!("  into_inner() = {} (Cell 被消耗)", val);
    }
    println!();

    // [Cell 小结]
    println!("── Cell 小结 ──");
    println!("  原理: get() 复制 / set() 替换, 永远不暴露内部引用");
    println!("  要求: T: Copy (否则 get() 无法工作)");
    println!("  所有: Cell 拥有内部值, get/set 不转移所有权");
    println!("  场景: 计数/标志/小型状态 — 小且 Copy 的值");
    println!();
}

// ======================================================================
//  Part 2: RefCell<T> — 运行时借用检查
// ======================================================================
//
// ┌── RefCell<T> 是什么 ────────────────────────────────────────┐
// │                                                              │
// │  RefCell<T> 也包装类型 T, 但与 Cell 不同:                    │
// │    borrow()     → 返回 Ref<T> (不可变借用) — 类似 &T         │
// │    borrow_mut() → 返回 RefMut<T> (可变借用) — 类似 &mut T   │
// │                                                              │
// │  运行时检查 (不像编译时):                                     │
// │    • 可以有多个 borrow() 同时存在                            │
// │    • borrow() 和 borrow_mut() 不能同时存在                  │
// │    • 只能有一个 borrow_mut()                                 │
// │    • 违规 → panic! (不是编译错误)                            │
// │                                                              │
// │  所有权分析:                                                  │
// │    RefCell::new(v)  → v 移入 RefCell, RefCell 拥有 v         │
// │    borrow()         → 返回 Ref<T>, 它引用了 RefCell 内部     │
// │    borrow_mut()     → 返回 RefMut<T>, 引用 RefCell 内部      │
// │    Ref/RefMut drop  → 借用计数释放 (类似引用离开作用域)      │
// │    RefCell drop     → 内部值 drop                           │
// │                                                              │
// │  ⚠️ Ref/RefMut 的生命周期:                                    │
// │    Ref<T> 实现了 Deref<Target=T>, 可以当 &T 用               │
// │    Ref 活着 = 借用计数 +1, Ref drop = 借用释放                │
// │    → 和普通引用的作用域规则一致, 但是运行时而非编译时        │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘
//
// ┌── 什么时候用 RefCell ──────────────────────────────────────┐
// │                                                              │
// │  • 不可变引用下需要修改大/非 Copy 值 (Vec, String, HashMap) │
// │  • 测试中的 mock 对象 (需要在 &self 方法里记录调用次数)      │
// │  • 与 Rc 组合: Rc<RefCell<T>> 共享可变数据                  │
// │                                                              │
// │  与其他知识点的联动:                                          │
// │                                                              │
// │  → Rc<T>:    不可变共享, RefCell 补上"可变"                  │
// │  → Arc<T>:   多线程不可变共享                                │
// │  → Mutex<T>: 多线程下替代 RefCell (阻塞式锁, 非 panic)      │
// │  → RwLock<T>: 读写锁 — borrow()=读锁, borrow_mut()=写锁    │
// │  → Drop:     Ref/RefMut 实现 Drop 来释放借用计数            │
// │  → Deref:    Ref<T> 实现了 Deref<Target=T>                  │
// │                                                              │
// │  单线程路径: Cell → RefCell → Rc<RefCell>                   │
// │  多线程路径:       → Mutex →  Arc<Mutex>                    │
// │                        → RwLock →  Arc<RwLock>              │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘

fn demo_refcell() {
    println!("========== RefCell<T>: 运行时借用检查 ==========\n");

    // ── 基础用法 ──
    println!("[基础] borrow() / borrow_mut()");
    {
        let data = RefCell::new(vec![1, 2, 3]);
        // data: 不可变绑定, 但 RefCell 允许借出 &mut Vec<i32>

        // 不可变借用 (borrow)
        {
            let borrowed = data.borrow();
            // borrowed: Ref<Vec<i32>>, 实现了 Deref → &Vec<i32>
            println!("  borrow(): {:?}", borrowed);
            println!("  第 0 个元素: {}", borrowed[0]);
        } // borrowed drop → RefCell 知道 borrowing 结束

        // 可变借用 (borrow_mut)
        {
            let mut borrowed_mut = data.borrow_mut();
            // borrowed_mut: RefMut<Vec<i32>>, Deref → &mut Vec<i32>
            borrowed_mut.push(4);
            println!("  borrow_mut() push 后: {:?}", borrowed_mut);
        }

        println!("  最终: {:?}", data.borrow());
    }
    println!();

    // ── 运行时违规 → panic ──
    println!("[违规] 同时 borrow() + borrow_mut() → panic!");
    {
        let data = RefCell::new(vec![1, 2, 3]);
        let _r1 = data.borrow(); // 不可变借用 +1

        // 下面这行会 panic:
        // let _r2 = data.borrow_mut(); // ← 已有不可变借用, 不能再加可变
        // panic 信息: "already borrowed: BorrowMutError"

        println!("  (已注释掉 borrow_mut — 否则 panic)");
        println!("  RefCell 规则: 多个 borrow() 可以共存,");
        println!("               borrow() 和 borrow_mut() 不能共存");
        println!("               违反 → panic (不是编译错误)");
    }
    println!();

    // ── 场景: 不可变签名下修改集合 ──
    println!("[场景] &self 方法内修改 Vec");
    {
        struct TaskList {
            // ⚠️ 不是 Vec<String>, 而是 RefCell<Vec<String>>
            //    这样才能在 &self 方法里 push
            items: RefCell<Vec<String>>,
        }

        impl TaskList {
            fn add(&self, task: String) {
                // &self 是不可变引用, 但 RefCell 给了可变访问
                self.items.borrow_mut().push(task);
                // 所有权: task (String) → push → 移入 Vec
            }

            fn list(&self) -> Vec<String> {
                self.items.borrow().clone()
                // borrow() → Ref<Vec<String>>
                // clone()  → 复制整个 Vec → 新 Vec (独立所有权)
            }

            fn count(&self) -> usize {
                self.items.borrow().len()
            }
        }

        let tasks = TaskList {
            items: RefCell::new(Vec::new()),
        };
        tasks.add(String::from("学习 Rust"));
        tasks.add(String::from("写项目"));
        println!("  任务数: {}, 列表: {:?}", tasks.count(), tasks.list());
    }
    println!();

    // ── Drop 联动: Ref 离开作用域释放借用 ──
    println!("[Drop 联动] Ref guard 的作用域 = 借用生命周期");
    {
        let data = RefCell::new(42);
        {
            let mut r = data.borrow_mut(); // 可变借用 +1
            *r = 100; // 通过 RefMut 修改
            // r: RefMut<i32>, 实现 Drop
            // r 离开作用域 → Drop 被调用 → RefCell 记录"借用已释放"
        } // 借用结束

        println!("  借用释放后: {}", data.borrow());
        println!("  Ref/RefMut 的 Drop 就是运行时版本的"引用离开作用域"");
    }
    println!();

    // [RefCell 小结]
    println!("── RefCell 小结 ──");
    println!("  原理: borrow()/borrow_mut() 返回 guard, 运行时计数");
    println!("  优势: 不要求 Copy, 可以获得 &mut T 的等价物");
    println!("  代价: 违规 panic (不是编译错误), 轻微运行时开销");
    println!("  所有: RefCell 拥有内部值, Ref/RefMut 只是借用");
    println!("  联动: Rc<RefCell> 共享可变 / Arc<Mutex> 多线程 / Drop 释放");
    println!();
}

// ======================================================================
//  Part 3: Rc<RefCell<T>> — 多所有者共享可变数据
// ======================================================================
//
// ┌── 为什么需要这个组合 ──────────────────────────────────────┐
// │                                                              │
// │  Rc<T> 的问题: 提供共享所有权, 但只能读 (Deref 给 &T)       │
// │  RefCell<T> 的问题: 提供内部可变, 但只能有一个所有者        │
// │                                                              │
// │  组合: Rc<RefCell<T>> = 共享 + 可变                          │
// │    多个 Rc 指向同一个 RefCell, 每个都可以 borrow_mut() 修改  │
// │                                                              │
// │  所有权层次:                                                  │
// │    Rc ──拥有──▶ RefCell ──拥有──▶ T                         │
// │    (引用计数)    (借用计数)     (实际数据)                    │
// │                                                              │
// │  Rc<RefCell<T>> 被 drop:                                     │
// │    1. Rc 引用计数 -1                                        │
// │    2. 如果计数归零 → drop Rc                                │
// │    3. Rc drop → drop RefCell                                │
// │    4. RefCell drop → drop T (内部值)                        │
// │                                                              │
// │  ⚠️ 与 Arc<Mutex<T>> 的区别:                                  │
// │    Rc<RefCell>  → 单线程, 违规 panic, 无阻塞                 │
// │    Arc<Mutex>   → 多线程, lock() 阻塞等待, 防数据竞争        │
// │    Arc<RwLock>  → 多线程, read()/write(), 读多写少优化       │
// │                                                              │
// └──────────────────────────────────────────────────────────────┘

fn demo_rc_refcell() {
    println!("========== Rc<RefCell<T>>: 共享可变数据 ==========\n");

    // ── 基础 ──
    println!("[基础] 多个所有者修改同一份数据");
    {
        let shared = Rc::new(RefCell::new(String::from("hello")));
        //           ^^^^^   ^^^^^^^^
        //           引用计数  运行时借用计数
        // 所有权: shared 拥有 Rc, Rc 拥有 RefCell, RefCell 拥有 String

        let clone1 = Rc::clone(&shared);
        let clone2 = Rc::clone(&shared);
        // Rc::clone: 只增加 Rc 引用计数, 不复制内部数据

        println!("  初始: '{}', Rc计数 = {}", shared.borrow(), Rc::strong_count(&shared));

        // clone1 修改 — 通过 RefCell 的 borrow_mut
        clone1.borrow_mut().push_str(" world");
        // 所有权: push_str 借用内部 String 的可变引用 → 在原值上追加

        // clone2 立即看到修改 — 它们指向同一块内存
        println!("  clone1 修改后, clone2 看到: '{}'", clone2.borrow());
        // 所有权: clone2.borrow() → Ref<String> (不可变借用)
        //         打印后 Ref drop → 借用释放

        println!("  修改后 Rc计数 = {}", Rc::strong_count(&shared));
    }
    println!();

    // ── 场景: 图遍历 — 多个节点共享同一份访问标记 ──
    println!("[场景] 图结构: 多个节点共享可变状态");
    {
        #[derive(Debug)]
        struct GraphNode {
            id: usize,
            visited: Rc<RefCell<bool>>, // 共享访问标记
            neighbors: Vec<Rc<RefCell<GraphNode>>>, // 共享邻居
        }

        // 共享的 visited 标志
        let visited_flag = Rc::new(RefCell::new(false));

        let node_a = Rc::new(RefCell::new(GraphNode {
            id: 1,
            visited: Rc::clone(&visited_flag),
            neighbors: vec![],
        }));

        // node_a.borrow_mut() → RefMut<GraphNode>
        // 可以修改 node_a 内部字段
        node_a.borrow_mut().visited.replace(true);
        // replace: 拿出旧值, 放入新值 (所有权: true 移入, false 移出)

        println!("  node_a visited = {}", node_a.borrow().visited.get());
        println!("  visited_flag = {} (同一份数据)", visited_flag.borrow());
    }
    println!();

    // ── 所有权层次示意 ──
    println!("[所有权层次] Rc → RefCell → T");
    {
        let r = Rc::new(RefCell::new(String::from("data")));
        {
            let mut bm = r.borrow_mut();
            // bm: RefMut<String> — 可变借用 String
            // Rc 计数: 1 (仍被 r 持有)
            // RefCell 借用计数: 1 (被 bm 持有)
            bm.push_str(" modified");
        } // bm drop → RefCell 借用计数归零
        println!("  最终: '{}'", r.borrow());
    } // r drop → Rc 计数归零 → RefCell drop → String drop
    println!();

    // ── 知识点联动图 ──
    println!("[知识点联动] 从 Cell 到 Arc<Mutex> 的演进");
    println!("  ┌────────────┬───────────────┬──────────────────┐");
    println!("  │ 数据量级   │ 单线程         │ 多线程            │");
    println!("  ├────────────┼───────────────┼──────────────────┤");
    println!("  │ Copy/小值  │ Cell           │ (仍需同步原语)    │");
    println!("  │ 非Copy/大值│ RefCell        │ Mutex / RwLock   │");
    println!("  │ 共享所有权 │ Rc<RefCell>   │ Arc<Mutex>        │");
    println!("  └────────────┴───────────────┴──────────────────┘");
    println!();

    // [Rc<RefCell> 小结]
    println!("── Rc<RefCell> 小结 ──");
    println!("  组合: Rc 提供共享所有权, RefCell 提供内部可变性");
    println!("  所有: Rc → RefCell → T, 三层嵌套所有权");
    println!("  限制: 单线程 (!Send + !Sync)");
    println!("  升级: 多线程用 Arc<Mutex<T>> 或 Arc<RwLock<T>>");
    println!("  注意: 循环引用 Rc<RefCell> 会内存泄漏 (用 Weak 打破)");
    println!();
}

// ======================================================================
//  总结
// ======================================================================

pub fn run() {
    println!("══════════ 内部可变性: Cell / RefCell / Rc<RefCell> ══════════\n");

    demo_cell();
    demo_refcell();
    demo_rc_refcell();

    println!("══════════ 核心要点 ══════════\n");

    println!("  1. 为什么需要内部可变性:");
    println!("     Rust 的'共享 XOR 可变'规则在编译时阻止了合法模式");
    println!("     (计数/缓存/mock/共享可变数据)\n");

    println!("  2. Cell<T> (Copy 类型):");
    println!("     原理: get() 复制 / set() 替换, 不暴露引用");
    println!("     所有: Cell 拥有 T; get/set 不转移所有权");
    println!("     场景: 计数器、标志位、小型 Copy 状态\n");

    println!("  3. RefCell<T> (运行时检查):");
    println!("     原理: borrow()/borrow_mut() 运行时计数");
    println!("     所有: RefCell 拥有 T; Ref/RefMut 只借用");
    println!("     联动: Drop (释放借用) / Deref (透明使用)");
    println!("     场景: 大值修改、测试 mock、与 Rc 组合\n");

    println!("  4. Rc<RefCell<T>> (共享+可变):");
    println!("     原理: Rc 共享所有 + RefCell 内部可变");
    println!("     所有: Rc → RefCell → T (三层嵌套)");
    println!("     对比: Arc<Mutex> 是多线程等价物");
    println!("     注意: 循环引用会内存泄漏 (用 Weak 打破)\n");

    println!("  5. 对比: 编译时 vs 运行时借用检查");
    println!("     编译器: 零开销, 限制严格 (有时候"明明没问题"也不行)");
    println!("     RefCell: 轻微开销, 更灵活 (违规 panic, 不是编译错误)");
    println!("     选择: 能编译期检查就编译期, 不能就用 RefCell");
}
