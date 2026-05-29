---
title: "Rust BinaryHeap<T> 二叉堆"
type: type-reference
category: 集合类型
tags:
    - rust
    - binaryheap
    - heap
    - priority-queue
    - collection
related:
    - 向量
    - 集合类型
---

# Rust BinaryHeap\<T\> 二叉堆

`BinaryHeap<T>` 是 Rust 标准库提供的优先队列（priority queue）实现。元素组织保持松散的排序，最大值总是「冒泡」到堆的顶部。

堆中元素必须实现 `Ord` 特型，这使得 `BinaryHeap` 可用作工作队列——按优先级处理任务。

## 一、核心方法

```rust
    use std::collections::BinaryHeap;

    // 创建
    let mut heap = BinaryHeap::new();
    let mut heap = BinaryHeap::with_capacity(10);
    let heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    // push：向堆中添加一个值
    heap.push(10);

    // peek：返回对堆中最大值的引用（不移除）
    let top = heap.peek();  // Option<&T>

    // peek_mut：返回对最大值的可变引用
    // 可以检查最大值后决定是否弹出
    if let Some(mut top) = heap.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);  // 从堆中弹出该值
        }
    }

    // pop：移除并返回最大值
    let max = heap.pop();  // Option<T>
```

## 二、使用示例

```rust
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));

    // 移除 9 后，8 上浮到顶部
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));
```

## 三、常用方法

```rust
    heap.len()          // 元素数量
    heap.is_empty()     // 是否为空
    heap.capacity()     // 当前容量
    heap.clear()        // 清空
    heap.reserve(n)     // 预留空间
    heap.shrink_to_fit() // 收缩容量

    // 将另一个堆的所有元素移动过来
    heap.append(&mut other_heap);
```

## 四、迭代

```rust
    // BinaryHeap 实现了 .iter()，但迭代顺序是任意的，不是从大到小
    for val in heap.iter() {
        // 元素以任意顺序出现
    }

    // 按优先级顺序消耗：使用 while let 循环
    while let Some(task) = heap.pop() {
        handle(task);
    }
```

> **关键点**：`BinaryHeap.iter()` 返回的迭代器**不会**按从大到小的优先级顺序生成元素。要按优先级顺序处理，请使用 `while let Some(item) = heap.pop()` 循环。

## 五、优先队列模式

`BinaryHeap` 可用作工作队列，按优先级处理任务：

```rust
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    // 定义任务结构体，按优先级排序
    #[derive(Eq, PartialEq)]
    struct Task {
        priority: u8,
        description: String,
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // 高优先级 > 低优先级
            self.priority.cmp(&other.priority)
        }
    }
    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(Task { priority: 5, description: "一般任务".into() });
    heap.push(Task { priority: 10, description: "紧急任务".into() });

    // 最高优先级的任务最先被处理
    while let Some(task) = heap.pop() {
        println!("处理: {}", task.description);
    }
```

> **小技巧**：如果想获得最小堆（最小值在顶部），可以用 `std::cmp::Reverse` 包装元素：`BinaryHeap::from(vec![Reverse(2), Reverse(3), Reverse(1)])`。
