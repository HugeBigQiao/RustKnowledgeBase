---
title: "Rust panic! 与不可恢复错误"
type: type-concept
category: 错误处理
tags:
    - rust
    - error-handling
    - panic
related:
    - Result
    - Option
---

# Rust panic! 与不可恢复错误

`panic!` 用于表示**不可恢复的错误**。当程序遇到无法继续执行的严重问题时，会触发 `panic`，默认情况下展开栈并终止程序。

## 一、什么是 `panic!`

`panic!` 是一个宏，调用后程序会打印错误信息、展开栈（或直接终止），然后退出。

```rust
fn main() {
    panic!("程序遇到了无法恢复的错误");
}
```

运行时会输出类似以下信息：

```
thread 'main' panicked at src/main.rs:2:5:
程序遇到了无法恢复的错误
```

## 二、哪些情况会触发 `panic`

除了手动调用 `panic!`，以下情况也会自动触发：

数组或向量越界访问：

```rust
let v = vec![1, 2, 3];
// v[100]; // panic: index out of bounds
```

对 `Option` 或 `Result` 调用 `unwrap()` 或 `expect()` 但值为 `None` 或 `Err`：

```rust
let opt: Option<i32> = None;
// opt.unwrap(); // panic: called `Option::unwrap()` on a `None` value
```

整型溢出（仅在 debug 模式下）：

```rust
let a: u8 = 255;
// let b = a + 1; // debug 模式下 panic，release 模式下回绕
```

断言失败：

```rust
let x = 3;
assert!(x > 5, "x 的值 {} 不大于 5", x);
// panic: assertion failed: x > 5
```

手动调用 `unreachable!` 或 `todo!`：

```rust
// unimplemented!();
// todo!();
```

## 三、`unwrap` 和 `expect`

`unwrap` 尝试获取 `Option` 的 `Some` 值或 `Result` 的 `Ok` 值，失败时触发 `panic`。`expect` 与之类似，但可以附带自定义错误信息。

```rust
let opt = Some(42);
let val = opt.unwrap();
// val == 42

let res: Result<i32, &str> = Ok(10);
let num = res.expect("获取结果失败");
// num == 10
```

```rust
let opt: Option<i32> = None;
// opt.expect("值不应该为空"); // panic: 值不应该为空
```

## 四、`panic!` 的两种行为模式

在 `Cargo.toml` 中可以设置 panic 行为：

```toml
[profile.release]
panic = 'abort'
```

- **unwind（默认）**：展开栈，调用每个栈帧的 `drop` 函数，清理资源后退出。
- **abort**：直接终止程序，不展开栈，由操作系统回收内存。生成的二进制体积更小。

两种模式下程序都会终止，区别在于资源清理的方式。

## 五、什么时候用 `panic!`

以下情况适合使用 `panic!`：

- 错误是无法恢复的，程序无法继续运行
- 违反了代码的内部契约或不变量
- 测试中快速失败
- 原型开发阶段，用 `todo!()` 或 `unimplemented!()` 占位

```rust
fn process(value: i32) -> i32 {
    if value < 0 {
        panic!("value 不能为负数，实际: {}", value);
    }
    value * 2
}
```

## 六、捕获 `panic!`

在特殊场景下（如测试或 FFI 边界），可以用 `std::panic::catch_unwind` 捕获 panic。但**不建议**将其作为常规错误处理手段，Rust 的惯用方式是使用 `Result`。

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    panic!("一个被捕获的 panic");
});

match result {
    Ok(_) => println!("没有 panic"),
    Err(e) => println!("捕获到 panic: {:?}", e),
}
```

## 速记要点

- `panic!` 表示不可恢复的错误，程序会终止
- 越界、unwrap 失败、断言失败等会自动触发 `panic`
- `unwrap` 和 `expect` 是获取值的快捷方式，失败则 `panic`
- 默认展开栈释放资源，可配置为直接 abort
- 常规错误处理应使用 `Result`，而非捕获 `panic!`