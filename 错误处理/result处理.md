---
title: "Rust Result 与可恢复错误"
type: type-concept
category: 错误处理
tags:
    - rust
    - error-handling
    - result
related:
    - panic
    - Option
    - 表达式总览
---

# Rust Result 与可恢复错误

`Result<T, E>` 是 Rust 中处理**可恢复错误**的核心类型。它是一个枚举，包含 `Ok(T)` 和 `Err(E)` 两个变体，强制调用者显式处理错误。

## 一、`Result` 的定义

```rust
enum Result<T, E> {
    Ok(T),   // 操作成功，包含结果值
    Err(E),  // 操作失败，包含错误信息
}
```

`T` 是成功时的返回值类型，`E` 是失败时的错误类型。两者都可以是任意类型。

## 二、基本使用

```rust
use std::fs::File;
use std::io::ErrorKind;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            File::create("hello.txt").expect("创建文件失败")
        }
        other_error => {
            panic!("打开文件时发生未知错误: {:?}", other_error);
        }
    },
};
```

通过 `match` 可以区分不同的错误类型并分别处理。

## 三、常用方法

`Result` 提供了丰富的组合子方法用于链式处理。

**`unwrap` 和 `expect`**：获取 `Ok` 中的值，如果是 `Err` 则触发 `panic`。

```rust
let res: Result<i32, &str> = Ok(42);
res.unwrap(); // 42

let res: Result<i32, &str> = Err("错误");
// res.expect("获取值失败"); // panic
```

**`unwrap_or` 和 `unwrap_or_else`**：`Err` 时提供默认值。

```rust
let res: Result<i32, &str> = Err("错误");
let val = res.unwrap_or(0);
// val == 0

let val = res.unwrap_or_else(|e| {
    println!("错误信息: {}", e);
    0
});
```

**`map` 和 `map_err`**：变换 `Ok` 或 `Err` 中的值。

```rust
let res: Result<i32, &str> = Ok(2);
let doubled = res.map(|x| x * 2);
// doubled == Ok(4)

let res: Result<i32, &str> = Err("出错了");
let msg = res.map_err(|e| format!("[严重] {}", e));
// msg == Err("[严重] 出错了")
```

**`and_then`**：链式调用可能失败的操作，如果当前是 `Ok` 则调用闭包，否则直接返回 `Err`。

```rust
fn double_if_positive(n: i32) -> Result<i32, &'static str> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err("数字必须为正数")
    }
}

let res = Ok(5).and_then(double_if_positive);
// res == Ok(10)

let res = Ok(-3).and_then(double_if_positive);
// res == Err("数字必须为正数")
```

## 四、`?` 运算符

`?` 是处理 `Result` 的语法糖。如果值是 `Ok`，解包其中的值；如果是 `Err`，则提前从当前函数返回该错误。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` 会自动调用 `From` trait 将错误类型转换为函数返回的错误类型。使用 `?` 的函数返回类型必须是 `Result` 或 `Option`。

```rust
fn first_char(s: &str) -> Option<char> {
    let ch = s.chars().next()?;
    Some(ch)
}
```

## 五、错误类型转换

使用 `map_err` 或实现 `From` trait 可以在不同错误类型之间转换。

```rust
use std::fs;
use std::io;

fn read_content() -> Result<String, String> {
    fs::read_to_string("data.txt").map_err(|e| format!("读取文件失败: {}", e))
}
```

对于库代码，通常建议使用 `thiserror` 或自定义错误枚举来统一管理错误类型。

## 六、`Result` 与 `Option` 的关系

`Result` 和 `Option` 共享类似的组合子方法。可以通过以下方式互转：

```rust
// Result → Option
let res: Result<i32, &str> = Ok(42);
let opt = res.ok(); // Some(42)

let res: Result<i32, &str> = Err("错误");
let opt = res.ok(); // None

// Option → Result
let opt = Some(42);
let res = opt.ok_or("值为空"); // Ok(42)

let opt: Option<i32> = None;
let res = opt.ok_or("值为空"); // Err("值为空")
```

## 七、什么时候用 `Result`，什么时候用 `panic!`

| 场景 | 使用 |
| :--- | :--- |
| 可预期、可恢复的错误 | `Result` |
| 不可恢复的严重错误 | `panic!` |
| 违反内部契约/不变量 | `panic!` |
| 外部输入不可信 | `Result` |
| 库代码暴露的公开 API | `Result` |

## 速记要点

- `Result<T, E>` 有 `Ok(T)` 和 `Err(E)` 两个变体
- 通过 `match`、`unwrap`、`map` 等方法处理结果
- `?` 运算符传播错误，出错时提前返回
- `map_err` 和 `From` trait 用于错误类型转换
- 可恢复错误用 `Result`，不可恢复用 `panic!`