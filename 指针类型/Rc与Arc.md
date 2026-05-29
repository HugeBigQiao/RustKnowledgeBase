---
title: "Rust Rc<T> 与 Arc<T> 共享所有权"
type: type-reference
category: 指针类型
tags:
    - rust
    - rc
    - arc
    - reference-counting
    - shared-ownership
related:
    - 所有权
    - 引用与借用
    - Box
    - 线程与通道
---

# Rust Rc\<T\> 与 Arc\<T\> 共享所有权

`Rc<T>`（Reference Counted）和 `Arc<T>`（Atomic Reference Counted）通过**引用计数**实现多个所有者共享同一份数据。当最后一个所有者离开作用域时，数据自动释放。

## 一、速览对比

| 特性 | `Rc<T>` | `Arc<T>` |
| :--- | :--- | :--- |
| 全称 | Reference Counted | Atomic Reference Counted |
| 线程安全 | **否**（单线程） | **是**（多线程） |
| 计数器操作 | 非原子操作，性能更高 | 原子操作，有少量开销 |
| 可变性 | 不可变（需配合 RefCell） | 不可变（需配合 Mutex/RwLock） |
| 类比 | C++ `std::shared_ptr`（非原子版） | C++ `std::shared_ptr` |
| 使用场景 | 单线程共享不可变数据 | 多线程共享不可变数据 |

## 二、`Rc<T>` 基本用法

```rust
use std::rc::Rc;

let a = Rc::new(String::from("共享数据"));

// clone 不会复制数据，只是增加引用计数
let b = Rc::clone(&a);  // 引用计数: 2
let c = Rc::clone(&a);  // 引用计数: 3

println!("a: {}, b: {}, c: {}", a, b, c);
println!("引用计数: {}", Rc::strong_count(&a));  // 3

// 当 b 和 c 离开作用域时，引用计数递减
// 当所有 Rc 都释放后，String 数据被释放
```

### 引用计数

```rust
use std::rc::Rc;

let a = Rc::new(42);
println!("创建后: {}", Rc::strong_count(&a));  // 1

let b = Rc::clone(&a);
println!("克隆后: {}", Rc::strong_count(&a));  // 2

{
    let c = Rc::clone(&a);
    println!("内部作用域: {}", Rc::strong_count(&a));  // 3
}
println!("内部作用域后: {}", Rc::strong_count(&a));  // 2
```

## 三、`Rc<T>` 的内部可变性

`Rc<T>` 提供的是**不可变共享**。如果需要修改内部数据，可以配合 `RefCell<T>` 使用：

```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(0));

let a = Rc::clone(&data);
let b = Rc::clone(&data);

*a.borrow_mut() = 42;

println!("a: {}", a.borrow());  // 42
println!("b: {}", b.borrow());  // 42
```

## 四、`Arc<T>` 多线程共享

```rust
use std::sync::Arc;
use std::thread;

let shared = Arc::new(vec![1, 2, 3]);
let mut handles = vec![];

for i in 0..3 {
    let clone = Arc::clone(&shared);
    handles.push(thread::spawn(move || {
        println!("线程 {} 看到的: {:?}", i, clone);
    }));
}

for handle in handles {
    handle.join().unwrap();
}
```

### `Arc<Mutex<T>>`：多线程可变共享

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}

for handle in handles {
    handle.join().unwrap();
}

println!("最终计数: {}", *counter.lock().unwrap());  // 10
```

## 五、强引用与弱引用

`Rc`/`Arc` 同时维护**强引用计数**和**弱引用计数**。

- **强引用** (`Rc::clone`)：增加 `strong_count`，阻止数据被释放
- **弱引用** (`Rc::downgrade`)：增加 `weak_count`，不阻止数据被释放，返回 `Weak<T>`

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(42);
let weak: Weak<i32> = Rc::downgrade(&strong);

// upgrade 返回 Option<Rc<T>>：如果数据还在则 Some，否则 None
if let Some(shared) = weak.upgrade() {
    println!("数据还在: {}", shared);
}

// 释放强引用后，数据被释放
drop(strong);
// weak.upgrade() 现在返回 None
```

### 弱引用的典型用途：避免引用循环

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,   // 弱引用：不阻止父节点释放
    children: RefCell<Vec<Rc<Node>>>,
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

// 父节点对子节点是强引用，子节点对父节点是弱引用
// 这样不会形成循环，父节点可以正常释放
```

## 六、选择指南

| 场景 | 推荐 |
| :--- | :--- |
| 单线程中多所有者共享不可变数据 | `Rc<T>` |
| 单线程中需要修改内部数据 | `Rc<RefCell<T>>` |
| 多线程中共享不可变数据 | `Arc<T>` |
| 多线程中需要修改数据 | `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>` |
| 需要观察但不拥有数据 | `Weak<T>` |
