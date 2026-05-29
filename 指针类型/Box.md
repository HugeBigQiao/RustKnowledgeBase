---
title: "Rust Box<T> 堆分配指针"
type: type-reference
category: 指针类型
tags:
    - rust
    - box
    - heap
    - pointer
related:
    - 引用与借用
    - 所有权
    - Rc与Arc
    - 切片
---

# Rust Box\<T\> 堆分配指针

`Box<T>` 是 Rust 中最简单的**智能指针**：它在堆上分配一个类型为 `T` 的值，并在栈上保存一个指向该值的指针。当 `Box<T>` 离开作用域时，堆内存连同栈上的指针一起被自动释放。

## 一、速览

| 特性 | 说明 |
| :--- | :--- |
| 存储位置 | 值在堆上，指针在栈上 |
| 所有者数量 | 唯一 |
| 是否线程安全 | 否（不实现 Sync） |
| 默认值 | 无 |
| 类比 | C++ 的 `std::unique_ptr<T>` |

## 二、基本用法

```rust
// 在堆上分配一个整数
let b = Box::new(5);
println!("b = {}", b);  // 自动解引用

// 在堆上分配大结构体（避免栈溢出）
let large_array = Box::new([0u8; 10_000_000]);

// 在堆上分配特型对象
let writer: Box<dyn Write> = Box::new(File::create("log.txt")?);
```

## 三、主要场景

### 3.1 将数据放在堆上

当数据过大，放在栈上可能导致栈溢出时，使用 `Box` 将其移到堆上：

```rust
// 这个结构体约 8MB，可能超过栈限制
struct LargeData {
    content: [u8; 8_000_000],
}

// 用 Box 将数据移到堆上
let data = Box::new(LargeData { content: [0; 8_000_000] });
// data 本身只占一个指针大小，LargeData 实际在堆上
```

### 3.2 递归类型

Rust 需要在编译时知道类型的大小。对于递归类型，通过 `Box` 创建一个间接层：

```rust
// 错误：List 的大小不确定（无限递归）
// enum List<T> { Cons(T, List<T>), Nil }

// 正确：用 Box 创建间接引用
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

let list = List::Cons(1,
    Box::new(List::Cons(2,
        Box::new(List::Nil))));
```

### 3.3 特型对象

`Box<dyn Trait>` 是最常用的特型对象形式：

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) { println!("汪汪!"); }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) { println!("喵喵!"); }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    for a in &animals {
        a.speak();
    }
}
```

### 3.4 转移所有权

与引用不同，`Box<T>` 拥有其所指向的数据，转移 `Box` 就会转移所有权：

```rust
let b1 = Box::new(String::from("hello"));
let b2 = b1;  // b1 所有权转移到 b2
// println!("{}", b1);  // 编译错误
```

## 四、`Box` 与 DST（动态大小类型）

`Box<[T]>` 和 `Box<str>` 允许你在堆上拥有动态大小类型的值：

```rust
// Box<[i32]>：堆上的不定长数组
let boxed_slice: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();

// Box<str>：堆上的字符串切片（比 String 更轻量，但不能修改）
let boxed_str: Box<str> = String::from("hello").into_boxed_str();
```

## 五、`Box::leak`：有意泄漏内存

`Box::leak` 将 `Box<T>` 转换为 `&'static mut T`，使其生命周期变为 `'static`：

```rust
let static_ref: &'static mut i32 = Box::leak(Box::new(42));
*static_ref = 100;  // 程序运行期间一直有效
// 注意：这块内存永远不会被释放
```

这个方法通常用于需要静态生命周期的初始化场景。
