//! 并发: thread::spawn / channel / Mutex / Arc.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// ===== 1. 线程创建 =====

/// 使用 thread::spawn 创建线程, JoinHandle 等待结束.
fn demo_thread_spawn() {
    println!("--- 线程创建 ---");

    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  子线程: 第 {} 步", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    // 主线程继续执行
    println!("  主线程: 不等待子线程");

    // join() 阻塞等待子线程结束
    handle.join().unwrap();
    println!("  主线程: 子线程已结束");
}

// ===== 2. move 闭包传所有权 =====

/// 线程闭包默认借用, 用 move 强制转移所有权.
fn demo_move_closure() {
    println!("\n--- move 闭包 ---");

    let v = vec![1, 2, 3];

    // 错误: 闭包可能比主线程活得更久, 不能借用
    // thread::spawn(|| { println!("{:?}", v); });

    // 正确: move 把 v 的所有权移入线程
    let handle = thread::spawn(move || {
        println!("  子线程拥有 v = {:?}", v);
    });

    handle.join().unwrap();
    // println!("{:?}", v); // 错误: v 已被移动
}

// ===== 3. Channel: mpsc 消息传递 =====

/// mpsc = multiple producer, single consumer
/// 类似 Go 的 channel、Erlang 的 mailbox.
fn demo_channel() {
    println!("\n--- Channel (mpsc) ---");

    // tx = 发送端, rx = 接收端
    let (tx, rx) = mpsc::channel();

    // 生产者线程
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("线程1"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    // 另一个生产者
    thread::spawn(move || {
        let vals = vec![
            String::from("消息"),
            String::from("来自"),
            String::from("线程2"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    // 主线程作为消费者
    for received in rx {
        println!("  收到: {}", received);
    }
}

// ===== 4. Mutex<T>: 互斥锁 =====

/// Mutex 保证同一时刻只有一个线程能访问数据.
/// 通常配合 Arc 实现多线程共享.
fn demo_mutex() {
    println!("\n--- Mutex<T> + Arc ---");

    // Arc<Mutex<T>> 是多线程共享可变数据的经典模式
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  线程 {} 把计数增加到 {}", i, *num);
            // MutexGuard 离开作用域自动释放锁
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终计数: {}", *counter.lock().unwrap());

    // 对比:
    // Rc<RefCell<T>>     → 单线程内部可变
    // Arc<Mutex<T>>      → 多线程互斥访问
    // Arc<RwLock<T>>     → 多线程读写锁 (多个读/单个写)
}

// ===== 5. Send 与 Sync trait =====

/// Send: 类型的所有权可以在线程间传递.
/// Sync: 类型的引用可以在线程间共享.
fn demo_send_sync() {
    println!("\n--- Send 与 Sync ---");

    // 大多数 Rust 类型都实现了 Send 和 Sync
    // 编译器自动推导, 无需手动实现

    // 没有 Send 的例子: Rc<T> (引用计数非原子操作)
    // 没有 Sync 的例子: RefCell<T> (运行时借用检查非线程安全)
    // 所以多线程要用 Arc 替代 Rc, Mutex 替代 RefCell

    println!("  Send: 所有权可跨线程传递 (i32, String, Vec, Arc<Mutex<T>> ...)");
    println!("  Sync: 引用可跨线程共享   (i32, &str, Mutex<T>, Atomic* ...)");
    println!("  !Send: Rc<T>");
    println!("  !Sync: Rc<T>, RefCell<T>, Cell<T>");
}

pub fn run() {
    demo_thread_spawn();
    demo_move_closure();
    demo_channel();
    demo_mutex();
    demo_send_sync();
}
