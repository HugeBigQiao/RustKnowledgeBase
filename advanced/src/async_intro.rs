//! 异步编程: async fn / .await / Future trait.

// 注意: 标准库提供了 Future trait 和 async/await 语法,
// 但没有内置运行时(runtime). 实际执行需要引入 tokio 或 async-std.
// 本文档演示语法概念, 运行需要外部 crate (注释说明).

// ===== 1. 同步 vs 异步对比 =====

/// 模拟耗时 I/O 操作的同步版本.
#[allow(dead_code)]
fn sync_read_file(path: &str) -> String {
    // 实际场景中可能是 fs::read_to_string(path)
    std::thread::sleep(std::time::Duration::from_millis(100));
    format!("[同步] 从 {} 读取的内容", path)
}

/// 模拟耗时 I/O 操作的异步版本.
/// async fn 返回 impl Future<Output = T>.
#[allow(dead_code)]
async fn async_read_file(path: &str) -> String {
    // 实际场景中可能是 tokio::fs::read_to_string(path).await
    // 这里用异步 sleep 模拟 (需要 tokio 的 sleep 函数)
    format!("[异步] 从 {} 读取的内容", path)
}

/// 演示 async fn 的声明语法.
fn demo_async_syntax() {
    println!("--- async fn 语法 ---");

    // async fn 本质上是返回 impl Future 的语法糖:
    // async fn foo() -> T    等价于    fn foo() -> impl Future<Output = T>

    println!("async fn 是一个返回 Future 的函数");
    println!("调用 async fn 不会执行, 只是构造了一个 Future");
    println!("必须 .await 或交给运行时才能实际执行");
}

// ===== 2. .await 串行 vs join! 并发 =====

/// 演示多个异步操作可以并发执行.
/// (伪代码, 实际需要运行时)
fn demo_await_pattern() {
    println!("\n--- .await 执行模型 ---");

    println!("示例代码 (需要 tokio):");
    println!("```");
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    // 串行: 逐个等待");
    println!("    let a = read_file(\"a.txt\").await;");
    println!("    let b = read_file(\"b.txt\").await;");
    println!("    // 总耗时 = t_a + t_b");
    println!("");
    println!("    // 并发: 同时启动, 一起等待");
    println!("    let (a, b) = tokio::join!(");
    println!("        read_file(\"a.txt\"),");
    println!("        read_file(\"b.txt\"),");
    println!("    );");
    println!("    // 总耗时 ≈ max(t_a, t_b)");
    println!("}}");
    println!("```");
}

// ===== 3. Future trait 概念 =====

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 极简 Future 实现: 返回固定值.
#[allow(dead_code)]
struct ImmediateValue {
    value: i32,
    done: bool,
}

impl Future for ImmediateValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.done {
            Poll::Ready(self.value)
        } else {
            self.done = true;
            Poll::Pending
        }
    }
}

/// 演示 Future trait 的 Poll 机制.
fn demo_future_trait() {
    println!("\n--- Future trait ---");

    println!("Future trait 的核心:");
    println!("  fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>");

    println!("\nPoll 枚举:");
    println!("  Ready(T)   -- 已完成, 返回结果");
    println!("  Pending    -- 未完成, 下次再问");

    println!("\n执行模型:");
    println!("  1. 运行时(executor)调用 poll()");
    println!("  2. 如果 Ready → 返回结果");
    println!("  3. 如果 Pending → Future 把自己注册到 waker,");
    println!("     等事件发生再被唤醒");

    // 注意: 实际项目中你几乎不需要手工实现 Future
    // async fn 和 .await 已经帮你完成了所有底层工作
}

// ===== 4. async 生命周期 =====

/// async 块/函数的生命周期注意事项.
fn demo_async_lifetime() {
    println!("\n--- async 与生命周期 ---");

    println!("async 函数中返回引用需要显式标注生命周期:");

    println!("```");
    println!("async fn foo(x: &str) -> &str {{  // 编译错误!");
    println!("    // async fn 返回的 Future 可能包含对参数的引用");
    println!("    x");
    println!("}}");
    println!("");
    println!("// 正确写法:");
    println!("fn foo(x: &str) -> impl Future<Output = &str> + '_ {{");
    println!("    async move {{ x }}");
    println!("}}");
    println!("```");
}

pub fn run() {
    demo_async_syntax();
    demo_await_pattern();
    demo_future_trait();
    demo_async_lifetime();
    println!();
    println!("注意: 标准库没有异步运行时.");
    println!("实际项目推荐 tokio (最流行) 或 async-std.");
    println!("后面的网站项目会结合 tokio 实战 async/await.");
}
