---
title: "Rust return 表达式"
type: type-concept
category: 控制流
tags:
    - rust
    - expression
    - return
    - control-flow
related:
    - 表达式总览
    - 循环表达式
    - 错误处理
---

# Rust return 表达式

`return` 用于**提前从函数中返回值**。它本身是一个表达式，但具有特殊的行为：执行到 `return` 时，当前函数的执行会立即终止，并将 `return` 后面的值作为函数的返回值传回调用者。

## 一、基本用法

`return` 后面跟一个值，这个值会成为函数的返回值。`return` 后面的代码不会被执行。

```rust
fn early_exit(x: i32) -> i32 {
    if x > 10 {
        return x; // 提前返回，后面的代码不执行
    }
    x + 1 // 尾表达式，仅在 x <= 10 时执行
}

let result = early_exit(15);
// result == 15

let result = early_exit(5);
// result == 6
```

## 二、`return` 与尾表达式的区别

Rust 函数默认返回**最后一个表达式的值**，这称为尾表达式。尾表达式不能有分号。`return` 则用于在函数中间位置提前退出并返回值。

```rust
// 使用尾表达式
fn implicit(x: i32) -> i32 {
    x + 1 // 没有分号，作为返回值
}

// 使用 return 提前返回
fn explicit(x: i32) -> i32 {
    if x < 0 {
        return 0; // 提前返回
    }
    x + 1 // 尾表达式
}
```

两种方式可以混合使用，但通常建议简单函数用尾表达式，需要提前退出的情况用 `return`。

## 三、`return` 在闭包中的行为

闭包中的 `return` **只从闭包返回**，不会跳出外层函数。

```rust
fn outer() -> i32 {
    let add = |a: i32, b: i32| -> i32 {
        return a + b; // 从闭包返回
    };
    let sum = add(3, 4);
    sum + 10 // 外层函数的尾表达式
}
// outer 返回 17
```

如果需要从外层函数提前返回，不能在闭包内部使用 `return`，而应在闭包外部根据闭包的结果进行判断。

```rust
fn process(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        let doubled = num * 2;
        if doubled > 100 {
            return Some(doubled); // 从外层函数 process 返回
        }
    }
    None
}
```

## 四、`return` 与 `!` 发散类型

`return` 表达式本身的类型是**发散类型** `!`。发散类型意味着该表达式永远不会产生一个具体的值，因为执行到它函数就结束了。因此 `return` 可以出现在任何需要任意类型的地方。

```rust
fn get_value(use_default: bool) -> i32 {
    let result = if use_default {
        return 0; // return 的类型是 !，可以兼容 if 分支的 i32 类型
    } else {
        42
    };
    // 如果上面 return 了，这里不会执行
    result
}
```

与 `return` 类似，`panic!`、`break`（在 `loop` 中）、`continue` 等也具有发散类型。

## 五、`return` 与错误处理

`return` 常用于错误处理中提前返回 `Err` 或 `None`。配合 `?` 运算符可以简化这种写法。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("除数不能为零".to_string());
    }
    Ok(a / b)
}
```

使用 `?` 运算符可以实现相同的提前返回效果，且更简洁。

```rust
fn read_file_content() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("hello.txt")?;
    Ok(content)
}
```

`?` 运算符本质上就是 `return Err(...)` 的语法糖，详见 [错误处理](../错误处理/result处理.md)。

## 六、不返回值的函数中的 `return`

在返回单元类型 `()` 的函数中，`return` 后面可以不跟值，或跟一个 `()`。

```rust
fn print_positive(x: i32) {
    if x <= 0 {
        return; // 等价于 return ();
    }
    println!("正数: {}", x);
}
```

## 速记要点

- `return` 用于提前从函数返回值，后续代码不执行
- 函数默认返回尾表达式的值，`return` 是可选的显式方式
- 闭包中的 `return` 只退出闭包，不影响外层函数
- `return` 表达式的类型是 `!`（发散类型），可兼容任何类型
- `?` 运算符是 `return Err(...)` 的语法糖
- 返回 `()` 的函数中 `return` 可以省略值
