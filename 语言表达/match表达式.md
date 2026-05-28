---
title: "Rust match 表达式"
type: type-concept
category: 语言表达
tags:
    - rust
    - expression
    - match
    - pattern-matching
related:
    - 表达式总览
    - if表达式
    - 循环表达式
---

# Rust match 表达式

`match` 是 Rust 中最强大的表达式之一，它基于**模式匹配**对值进行穷尽检查，并返回匹配分支的值。编译器会强制检查所有可能的模式是否都被覆盖。

## 一、基本结构

`match` 由被匹配的值和多个**分支**组成。每个分支包含一个模式和一个表达式，模式和表达式之间用 `=>` 分隔。多个模式可以用 `|` 组合。

```rust
let number = 13;
let desc = match number {
    1 => "one",
    2 | 3 => "two or three",
    4..=10 => "four to ten",
    _ => "something else",
};
// desc == "something else"
```

- `|` 用于匹配多个值中的任意一个
- `..=` 用于匹配闭区间范围
- `_` 是通配符，匹配所有未被前面分支捕获的值

## 二、模式匹配能力

`match` 支持丰富的模式，可以匹配字面量、变量、范围，还可以解构枚举、结构体、元组和引用。

```rust
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

let c = Color::Rgb(255, 0, 0);
match c {
    Color::Red => println!("纯红"),
    Color::Green => println!("纯绿"),
    Color::Blue => println!("纯蓝"),
    Color::Rgb(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
}
```

在 `Color::Rgb(r, g, b)` 这个模式中，`r`、`g`、`b` 会绑定到枚举变体内部对应的值，可以直接在分支表达式中使用。

## 三、穷尽性检查

编译器在编译时强制要求 `match` 覆盖所有可能的模式。如果有遗漏，编译会直接报错。

```rust
let some_number = Some(5);
match some_number {
    Some(n) => println!("数字: {}", n),
    None => println!("无值"),
}
```

如果忘记处理 `None`，编译器会给出类似以下的错误提示：

```
error[E0004]: non-exhaustive patterns: `None` not covered
```

使用通配符 `_` 可以匹配剩余所有情况，避免穷举每一个具体值。

## 四、作为表达式返回值

`match` 是一个表达式，整个 `match` 的值是被选中分支的值。所有分支的返回值类型必须一致。

```rust
let opt = Some(3);
let doubled = match opt {
    Some(n) => n * 2,
    None => 0,
};
// doubled == 6
```

如果某个分支需要执行多行代码，可以用块 `{}` 包裹，块的最后一个表达式作为返回值。

```rust
let opt = Some(4);
let result = match opt {
    Some(n) => {
        let doubled = n * 2;
        doubled + 1
    }
    None => 0,
};
// result == 9
```

## 五、`if let` 语法糖

当只关心某一种模式而忽略其他时，可以用 `if let` 简化写法。它等价于只处理一个分支的 `match`，其余分支什么都不做。

```rust
let opt = Some(7);
if let Some(n) = opt {
    println!("数字: {}", n);
}
```

等价于：

```rust
let opt = Some(7);
match opt {
    Some(n) => println!("数字: {}", n),
    _ => (),
}
```

`if let` 也可以结合 `else` 处理不匹配的情况。

```rust
let opt: Option<i32> = None;
if let Some(n) = opt {
    println!("数字: {}", n);
} else {
    println!("没有值");
}
```

## 六、`while let` 条件循环

`while let` 在每次循环时检查模式是否匹配，匹配则执行循环体，不匹配则退出。常用于遍历或弹出元素直到耗尽。

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
// 依次打印 3、2、1
```

## 七、匹配引用与解构

`match` 可以和引用、可变引用配合使用。模式中可以使用 `ref` 关键字绑定引用，也可以对引用直接解构。

```rust
let point = (3, 5);
match &point {
    &(x, y) => println!("坐标: ({}, {})", x, y),
}

// 使用 ref 绑定引用而非移动
let s = String::from("hello");
match s {
    ref r => println!("{}", r), // r 是 &String 类型，s 未被移动
}
```

## 速记要点

- `match` 基于模式匹配选择分支并返回值
- 编译器强制穷尽所有可能模式，避免遗漏
- 模式包括字面量、范围、变量、解构、通配符等
- 所有分支的返回值类型必须一致
- `if let` 和 `while let` 是 `match` 的便捷写法
- `ref` 关键字用于在模式中绑定引用而不移动值