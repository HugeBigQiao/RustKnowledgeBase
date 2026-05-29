---
title: "Rust VecDeque<T> 双端队列"
type: type-reference
category: 集合类型
tags:
    - rust
    - vecdeque
    - deque
    - collection
related:
    - 向量
    - 集合类型
---

# Rust VecDeque\<T\> 双端队列

`VecDeque<T>` 是 Rust 标准库提供的双端队列（deque，double-ended queue 的缩写，发音为 /'dek/）。它支持在首端和尾端进行高效的添加和移除操作。

`Vec` 只支持在末尾高效操作，当程序需要一个存储"排队等候"值的地方时，`VecDeque` 是更好的选择。

## 一、与 Vec 的对比

| 特性 | `Vec<T>` | `VecDeque<T>` |
| :--- | :--- | :--- |
| 首端插入/移除 | O(n)，需平移所有元素 | O(1)，高效 |
| 尾端插入/移除 | O(1)，高效 | O(1)，高效 |
| 随机索引访问 | O(1)，连续内存 | O(1) |
| 切片方法 | 全部支持 | 部分支持（内存不连续） |
| 迭代 | 按顺序，连续内存 | 按顺序，可能回绕 |

## 二、基本操作

```rust
    use std::collections::VecDeque;

    // 创建
    let mut deque = VecDeque::new();
    let mut deque = VecDeque::with_capacity(10);
    let deque = VecDeque::from(vec![1, 2, 3, 4]);

    // 首端操作
    deque.push_front(0);     // 在首端添加
    let front = deque.pop_front();  // 从首端移除，返回 Option<T>

    // 尾端操作
    deque.push_back(5);      // 在尾端添加
    let back = deque.pop_back();    // 从尾端移除，返回 Option<T>

    // 查看首/尾（不移除）
    let f = deque.front();        // Option<&T>
    let b = deque.back();         // Option<&T>
    let f_mut = deque.front_mut(); // Option<&mut T>
    let b_mut = deque.back_mut();  // Option<&mut T>
```

## 三、内部实现：环形缓冲区

`VecDeque` 的实现是一个环形缓冲区（ring buffer）：

- 数据存储在堆上分配的一块连续内存中
- 数据不一定从该区域的起始位置开始，可以「回绕」到末尾
- 有两个私有字段标记数据在缓冲区中的首端位置和尾端位置
- 当缓冲区填满时，会分配更大的内存块

```
    缓冲区：[A][B][C][ ][ ][ ][E][F][G][H]
                    ↑尾           ↑首
    
    逻辑顺序：E, F, G, H, A, B, C
```

## 四、常用方法

`VecDeque` 实现了 `Vec` 中的许多方法：

```rust
    deque.len()         // 元素数量
    deque.is_empty()    // 是否为空
    deque.capacity()    // 当前容量
    deque.clear()       // 清空
    deque.reserve(n)    // 预留空间

    // 在任意位置插入/移除（非首尾端操作会平移元素）
    deque.insert(index, value)
    deque.remove(index)

    // 批量扩展
    deque.extend(iterable)
```

## 五、索引与迭代

```rust
    // 通过索引访问（与 Vec 相同）
    let elem = deque[2];

    // 迭代方式
    for val in &deque { }       // 共享引用迭代
    for val in &mut deque { }   // 可变引用迭代
    for val in deque { }        // 消耗型迭代（按值）

    // 迭代器方法
    deque.iter()       // &T
    deque.iter_mut()   // &mut T
```

## 六、make_contiguous 与切片方法

因为 `VecDeque` 元素不存储在连续内存中，所以无法继承切片的所有方法。可以通过 `make_contiguous()` 解决：

```rust
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);

    // 将元素重新排列到连续内存中，返回 &mut [T]
    let slice: &mut [i32] = deque.make_contiguous();

    // 现在可以调用切片方法了
    slice.sort();
    slice.reverse();
```

## 七、与 Vec 互转

```rust
    // VecDeque → Vec（O(n)，可能要重新排列）
    let vec = Vec::from(deque);

    // Vec → VecDeque（O(1)，直接转移缓冲区，不重新分配）
    let deque = VecDeque::from(vec![1, 2, 3, 4]);
```

> `VecDeque::from(vec)` 是 O(1) 操作，Rust 会直接把向量的缓冲区转移给 `VecDeque`。这使得创建带初始元素的双端队列非常方便，即使没有标准的 `vec_deque![]` 宏。

## 八、典型使用场景

```rust
    use std::collections::VecDeque;

    // 队列（FIFO）：push_back + pop_front
    let mut queue = VecDeque::new();
    queue.push_back("task1");
    queue.push_back("task2");
    while let Some(task) = queue.pop_front() {
        println!("Processing: {}", task);
    }

    // 栈（LIFO）：push_back + pop_back（等同于 Vec 的 push/pop）
    let mut stack = VecDeque::new();
    stack.push_back(1);
    stack.push_back(2);
    stack.pop_back();  // Some(2)
```
