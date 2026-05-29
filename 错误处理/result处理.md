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

## 八、处理多种 Error 类型

当函数中可能产生多种错误类型时，`?` 运算符需要将错误统一转换为函数的返回错误类型。

### 问题场景

```rust
use std::io::{self, BufRead};

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // io::Error
        numbers.push(line.parse()?);     // ParseIntError — 编译错误！
    }
    Ok(numbers)
}
```

`line_result` 的错误类型是 `io::Error`，`line.parse()` 的错误类型是 `ParseIntError`。`?` 会尝试通过 `From` 特型自动转换，但 `io::Error` 没有实现 `From<ParseIntError>`。

### 方案 1：`Box<dyn Error>` 统一错误类型

所有标准库错误类型都可以转换为 `Box<dyn std::error::Error + Send + Sync + 'static>`：

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // 自动转换
        numbers.push(line.parse()?);     // 自动转换
    }
    Ok(numbers)
}
```

**优点**：简单快捷。**缺点**：返回类型不再精确传达调用者可预期的错误类型。

**向下转型（downcast）**：如果需要在 `GenericResult` 中处理特定错误类型：

```rust
match compile_project() {
    Err(err) => {
        if let Some(mse) = err.downcast_ref::<MissingSemicolonError>() {
            insert_semicolon_in_source_code(mse.file(), mse.line())?;
            continue;
        }
        return Err(err);
    }
    Ok(()) => return Ok(()),
}
```

### 方案 2：`anyhow` crate

`anyhow` crate 提供了与 `GenericError`/`GenericResult` 非常相似但功能更丰富的错误类型：

```rust
use anyhow::{Context, Result};

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(
            line.parse()
                .with_context(|| format!("解析数字失败: {}", line))?
        );
    }
    Ok(numbers)
}
```

`anyhow::Result<T>` 等价于 `Result<T, anyhow::Error>`。`anyhow::Error` 可以容纳任何错误，并提供 `.context()` 方法附加额外信息。

### 方案 3：`thiserror` crate（自定义错误枚举）

`thiserror` 适合库代码，可以定义精确的错误类型：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("数据格式无效: {0}")]
    InvalidData(String),
}
```

`#[error("...")]` 定义 `Display` 输出，`#[from]` 自动生成 `From` 实现，使 `?` 可以自动转换。

### 方案对比

| 方案 | 适用场景 | 特点 |
| :--- | :--- | :--- |
| `Box<dyn Error>` | 快速原型、小工具 | 无需额外依赖，但丢失类型信息 |
| `anyhow` | 应用程序代码 | 易于使用，支持 `.context()`，不适合库 |
| `thiserror` | 库代码 | 类型精确，调用者可精确匹配错误类型 |

## 九、`main()` 中返回 `Result`

可以更改 `main()` 的类型签名以返回 `Result`，从而在 `main()` 中使用 `?`：

```rust
fn main() -> Result<(), TideCalcError> {
    let tides = calculate_tides()?;
    print_tides(tides);
    Ok(())
}
```

这适用于任何能用 `{:?}` 格式说明符打印的错误类型（所有标准错误类型都适用）。Rust 会在 `main()` 返回 `Err` 时打印 `Debug` 格式的错误并退出。

如果需要更友好的错误输出：

```rust
fn main() {
    if let Err(err) = calculate_tides() {
        print_error(&err);
        std::process::exit(1);
    }
}
```

其中 `print_error` 可以遍历 `.source()` 链打印完整的错误链路：

```rust
use std::error::Error;
use std::io::{Write, stderr};

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
```

## 十、声明自定义错误类型

自定义错误类型需要实现 `Display` 和 `Error` 特型：

```rust
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// 错误应该是可打印的
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// 错误应该实现 std::error::Error 特型
impl std::error::Error for JsonError {}
```

**使用 `thiserror` 简化**（推荐）：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message} ({line}, {column})")]
pub struct JsonError {
    message: String,
    line: usize,
    column: usize,
}
```

`#[derive(Error)]` 会自动生成 `Display` 和 `Error` 的实现，大幅减少样板代码。

## 十一、为什么是 `Result`

Rust 选择 `Result` 而非异常的设计要点：

| 设计目标 | 说明 |
| :--- | :--- |
| **强制处理** | 要求程序员在每个可能发生错误的地方做出决策并记录在代码中 |
| **传播简洁** | 用单个字符 `?` 即可传播错误，兼具可见性 |
| **类型透明** | 是否可能出错是函数返回类型的一部分，一眼便知 |
| **防止忽略** | Rust 检查 `Result` 值是否被使用，防止错误悄悄溜走 |
| **灵活存储** | `Result` 是普通数据类型，可存入集合、模拟"部分成功" |

## 速记要点

- `Result<T, E>` 有 `Ok(T)` 和 `Err(E)` 两个变体
- 通过 `match`、`unwrap`、`map` 等方法处理结果
- `?` 运算符传播错误，出错时提前返回；会自动通过 `From` 特型转换错误类型
- `map_err` 和 `From` trait 用于错误类型转换
- 可恢复错误用 `Result`，不可恢复用 `panic!`
- 多种错误类型统一：`Box<dyn Error>`（原型）、`anyhow`（应用）、`thiserror`（库）
- `main()` 可返回 `Result<(), E>` 以使用 `?` 运算符
- 自定义错误类型需实现 `Display` + `Error`，推荐用 `thiserror` 派生