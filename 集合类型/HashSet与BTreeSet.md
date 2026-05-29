---
title: "Rust HashSet<T> 与 BTreeSet<T>"
type: type-reference
category: 集合类型
tags:
    - rust
    - hashset
    - btreeset
    - set
    - collection
related:
    - 向量
    - HashMap与BTreeMap
    - 集合类型
---

# Rust HashSet\<T\> 与 BTreeSet\<T\>

`Set`（集合）是用于快速进行元素存在性测试的集合。`Set` 中永远不会包含相同值的多个副本。

> 在幕后，`HashSet<T>` 是对 `HashMap<T, ()>` 的浅层包装，`BTreeSet<T>` 是对 `BTreeMap<T, ()>` 的浅层包装。

## 一、特性对比

| 特性 | `HashSet<T>` | `BTreeSet<T>` |
| :--- | :--- | :--- |
| 底层结构 | 哈希表 | B 树 |
| 元素要求 | `Hash + Eq` | `Ord` |
| 迭代顺序 | 任意顺序 | 按元素排序 |
| 查找性能 | O(1) 平均 | O(log n) |
| 容量管理 | `with_capacity` | 不需容量管理 |

## 二、基本操作

```rust
    use std::collections::{HashSet, BTreeSet};

    // 创建
    let mut set = HashSet::new();
    let mut set = HashSet::with_capacity(100);
    let set: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    // 存在性检查
    let large_set: HashSet<&str> = ...;
    let found = large_set.contains(&"needle");  // 快，按哈希查找

    // 插入（返回是否新增）
    let is_new = set.insert("value");  // true 表示新插入，false 表示已存在

    // 移除（返回是否移除）
    let was_removed = set.remove(&"value");

    // 批量操作
    set.retain(|&v| v > 0);  // 保留满足条件的
    set.clear();              // 清空
    set.len();                // 元素数量
    set.is_empty();           // 是否为空
```

### 与向量查找的性能对比

```rust
    let b1 = large_vector.contains(&"needle");    // 慢，检查每个元素 O(n)
    let b2 = large_hash_set.contains(&"needle");  // 快，按哈希查找 O(1)
```

## 三、「相等但不同」值的操作

当两个值 `==` 相等但内部细节不同时（如两个内容相同的 `String` 在不同内存地址），以下方法可以访问集合中存储的**实际值**：

```rust
    // get：返回等于 value 的集合成员的引用
    let actual = set.get(&value);  // Option<&T>

    // take：移除并返回等于 value 的值
    let removed = set.take(&value);  // Option<T>

    // replace：如果已存在等值元素则替换并返回原值
    let old = set.replace(value);  // Option<T>
```

```rust
    // 示例：两个相等的 String 在堆上的地址不同
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    // s1 == s2 但 &s1 as *const _ != &s2 as *const _

    set.insert(s1);
    // get(&s2) 返回的引用指向集合中的实际值（s1）
```

## 四、迭代

```rust
    let set: HashSet<_> = vec![3, 1, 2].into_iter().collect();

    // 按值迭代（消耗 Set）
    for v in set { }

    // 按共享引用迭代
    for v in &set { }  // &T

    // 迭代器
    set.iter()  // 返回 &T 的迭代器
```

> **不支持**对集合中元素的可变引用迭代。无法获取 `&mut T`，因为修改元素可能改变其哈希值或排序位置，破坏集合内部结构。

- `HashSet` 迭代器以**任意顺序**产生值
- `BTreeSet` 迭代器**按顺序**产生值（类似已排序的向量）

## 五、集合运算

`Set` 支持对整个集合进行集合论运算。所有这些方法也都有对应的运算符简写。

```rust
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    // 交集：同时存在于 a 和 b 中的值
    let inter: Vec<_> = a.intersection(&b).collect();  // [2, 3]
    let inter_set = &a & &b;  // HashSet

    // 并集：存在于 a 或 b 中的值
    let union: Vec<_> = a.union(&b).collect();        // [1, 2, 3, 4]
    let union_set = &a | &b;  // HashSet

    // 差集：存在于 a 但不在 b 中的值
    let diff: Vec<_> = a.difference(&b).collect();     // [1]
    let diff_set = &a - &b;  // HashSet

    // 对称差集（异或）：存在于 a 或 b 但不同时存在于两者
    let sym_diff: Vec<_> = a.symmetric_difference(&b).collect();  // [1, 4]
    let sym_diff_set = &a ^ &b;  // HashSet
```

## 六、集合关系测试

```rust
    let a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let c: HashSet<_> = vec![4, 5].into_iter().collect();

    // 是否不相交（无交集）
    assert!(a.is_disjoint(&c));   // a 和 c 无共同元素

    // 子集关系
    assert!(a.is_subset(&b));     // a 的所有值都在 b 中
    assert!(b.is_superset(&a));   // b 包含 a 的所有值

    // 相等性
    assert!(a != b);
    assert!(a == a);
```

## 七、运算对照表

| 方法 | 运算符 | 说明 |
| :--- | :--- | :--- |
| `set1.intersection(&set2)` | `&set1 & &set2` | 交集 |
| `set1.union(&set2)` | `&set1 \| &set2` | 并集 |
| `set1.difference(&set2)` | `&set1 - &set2` | 差集 |
| `set1.symmetric_difference(&set2)` | `&set1 ^ &set2` | 对称差集 |
| `set1.is_disjoint(&set2)` | — | 是否无交集 |
| `set1.is_subset(&set2)` | — | 是否为子集 |
| `set1.is_superset(&set2)` | — | 是否为超集 |
| `set1 == set2` | — | 相等（相同元素集合） |

> **注意**：运算符 `&`、`|`、`-`、`^` 作用在两个**引用**上，返回的是新 `Set`；而方法返回的是迭代器，需要时才收集。
