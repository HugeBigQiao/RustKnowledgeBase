---
title: "Rust if 表达式"
type: type-concept
category: 语言表达
tags:
    - rust
    - expression
    - if
    - control-flow
related:
    - 表达式总览
    - match表达式
    - 循环表达式
---

# Rust if 表达式

`if` 在 Rust 中是一个**表达式**，可以从两个分支中返回值，而不仅仅是控制流语句。所有分支必须产生相同类型的值。

## 一、基本用法

`if` 后面跟一个 `bool` 类型的条件，条件不需要括号包裹。每个分支用花括号 `{}` 包裹代码块。

```rust
let condition = true;

if condition {
    println!("条件为真");
} else {
    println!("条件为假");
}
```

## 二、作为表达式返回值

`if` 的每个分支可以产生一个值，整个 `if` 表达式的值就是被选中的那个分支的值。

```rust
let condition = true;
let number = if condition {
    42   // 分支无分号，作为返回值
} else {
    0
};
// number == 42
```

分支代码块的最后一行不加分号，这个值就会作为该分支的返回值。

## 三、多重条件 `else if`

多个条件判断可以用 `else if` 串联。

```rust
let score = 85;
let grade = if score >= 90 {
    'A'
} else if score >= 80 {
    'B'
} else if score >= 70 {
    'C'
} else {
    'F'
};
// grade == 'B'
```

## 四、分支类型必须一致

所有分支的返回值类型必须相同，否则编译器会报错。

```rust
// 错误示例：类型不匹配
// let x = if true { 42 } else { "hello" };

// 正确：所有分支均为 i32
let x = if true { 42 } else { 0 };
```

## 五、`if` 与 `let` 结合

`if` 表达式可以直接用于 `let` 语句的右侧，让变量初始化更简洁。

```rust
let is_even = true;
let num = if is_even { 2 } else { 1 };
// num == 2
```

这种方式保证了变量一定会被初始化，不存在未初始化的状态。

## 六、不返回值的 `if`

当 `if` 只用于控制流程、不关心返回值时，每个分支返回的是单元类型 `()`。

```rust
let mut flag = false;
if flag {
    println!("flag is true");
} else {
    println!("flag is false");
}
// 整个 if 表达式的类型是 ()
```

## 七、`if` 与 `if let` 的区别

`if let` 是 `match` 的语法糖，用于匹配单一模式。它不属于 `if` 表达式本身的范畴。

```rust
let opt = Some(7);
if let Some(n) = opt {
    println!("数字: {}", n);
}
// 等价于 match opt { Some(n) => ..., _ => {} }
```

详见 [match 表达式](match表达式.md)。

## 速记要点

- `if` 是表达式，可以返回值用于赋值
- 条件不需要括号，必须是 `bool` 类型
- 所有分支必须返回相同类型
- 分支最后一行不加分号即为返回值
- `if let` 是 `match` 的语法糖，不是 `if` 表达式的一部分