---
title: "Rust HashMap<K,V> 与 BTreeMap<K,V>"
type: type-reference
category: 集合类型
tags:
    - rust
    - hashmap
    - btreemap
    - map
    - collection
related:
    - 向量
    - HashSet与BTreeSet
    - 集合类型
---

# Rust HashMap\<K, V\> 与 BTreeMap\<K, V\>

`Map`（映射）是键-值对（称为**条目**，entry）的集合。任何两个条目都不会有相同的键，且条目始终按某种数据结构组织，以便通过键高效查找值。

Rust 提供两种 `Map` 类型：

| 特性 | `HashMap<K, V>` | `BTreeMap<K, V>` |
| :--- | :--- | :--- |
| 底层结构 | 哈希表 | B 树 |
| 键要求 | `Hash + Eq` | `Ord` |
| 迭代顺序 | 任意顺序 | 按键排序 |
| 查找性能 | O(1) 平均 | O(log n) |
| 容量管理 | `with_capacity` / `shrink_to_fit` | 不需容量管理 |

> Rust 标准库采用 B 树而非平衡二叉树，因为 B 树在现代硬件上具有更好的**局部性**（内存访问分组而非分散），能减少 CPU 缓存未命中。

## 一、创建与基本操作

```rust
    use std::collections::{HashMap, BTreeMap};

    // 创建空 Map
    let mut map = HashMap::new();
    let mut map = HashMap::with_capacity(100);  // 预分配容量（仅 HashMap）
    let mut map = BTreeMap::new();

    // 从迭代器收集
    let map: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();
```

```rust
    let mut map = HashMap::new();

    // 插入/更新
    map.insert("key", "value");  // 返回旧值 Option<V>

    // 查询
    let v = map.get(&"key");         // Option<&V>
    let v = map.get_mut(&"key");     // Option<&mut V>
    let has = map.contains_key(&"key");  // bool

    // 通过索引访问（键不存在时会 panic）
    let v = map["key"];

    // 移除
    let v = map.remove(&"key");           // Option<V>
    let (k, v) = map.remove_entry(&"key").unwrap();  // Option<(K, V)>

    // 基本信息
    let n = map.len();
    let empty = map.is_empty();
```

> `get` / `contains_key` / `remove` 的键参数不一定需要是确切的 `&K` 类型。在 `HashMap<String, V>` 上可以传 `&str`，因为 `String` 实现了 `Borrow<&str>`。

## 二、条目 API（Entry API）

条目 API 是 `HashMap` 和 `BTreeMap` 最强大的特性之一，它消除了冗余的查找操作。

```rust
    let mut map = HashMap::new();

    // entry：获取指定键的条目，返回 Entry 枚举（Occupied 或 Vacant）
    map.entry("key")
```

**三个核心方法：**

```rust
    // or_insert：如果为空则插入值，返回 &mut V
    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    for name in ballots {
        let count = vote_counts.entry(name).or_insert(0);
        *count += 1;
    }

    // or_default：如果为空则插入 Default::default()
    let count = map.entry("key").or_default();  // 要求 V: Default

    // or_insert_with：如果为空则调用闭包生成值
    let set = word_occurrence
        .entry(word)
        .or_insert_with(HashSet::new);
    set.insert(file.clone());
```

**and_modify：修改已存在的值**

```rust
    // 统计单词出现频率（一次查找完成）
    let mut word_frequency: HashMap<&str, u32> = HashMap::new();
    for c in text.split_whitespace() {
        word_frequency.entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
```

> `Entry` 实际上是对 `Map` 中某个位置的可变引用，该位置要么被占用（Occupied），要么是空的（Vacant）。只要 `Entry` 存在，它就拥有对 `Map` 的独占访问权。

## 三、批量操作

```rust
    // extend：从迭代器批量插入
    map.extend(vec![("c", 3), ("d", 4)]);

    // append：将另一个 Map 的所有条目移动过来，之后 map2 为空
    map.append(&mut map2);

    // retain：保留满足条件的条目
    map.retain(|&k, &mut v| v > 0);

    // clear：清空
    map.clear();
```

**BTreeMap 独有的 split_off：**

```rust
    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let right = map.split_off(&"b");
    // map 保留键 < "b" 的条目：{"a": 1}
    // right 包含其余的：{"b": 2, "c": 3}
```

## 四、迭代

```rust
    let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into();

    // 按值迭代（消耗 Map）
    for (k, v) in map { }

    // 按共享引用迭代
    for (k, v) in &map { }  // (&K, &V)

    // 按可变引用迭代（只能修改值，不能修改键）
    for (k, v) in &mut map { }  // (&K, &mut V)

    // 迭代器方法
    map.iter()        // (&K, &V)
    map.iter_mut()    // (&K, &mut V)
    map.keys()        // 只有键的迭代器
    map.values()      // 只有值的迭代器
    map.values_mut()  // 只有值的可变迭代器

    // 消耗型迭代
    map.into_iter()     // (K, V)
    map.into_keys()     // 只有键
    map.into_values()   // 只有值
```

> **迭代顺序差异**：`HashMap` 以任意顺序迭代，`BTreeMap` 按键的顺序迭代。

## 五、哈希与 Eq 要求

`HashMap` 的键必须实现 `Hash` 和 `Eq`。大多数内置类型已实现，自定义类型可以派生：

```rust
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum MuseumNumber { ... }
```

> **关键约束**：如果 `a == b`，则必须 `hash(a) == hash(b)`。如果手动实现 `PartialEq`，也必须手动实现 `Hash` 以保持一致性。

**手动实现 Hash：**

```rust
    use std::hash::{Hash, Hasher};

    impl Hash for Artifact {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            // 只将与 == 相关的数据提供给 hasher
            self.id.hash(hasher);
        }
    }
```

## 六、自定义哈希算法

Rust 默认使用 SipHash-1-3 算法，速度很快且有 HashDoS 防护。对于小型键（整数、短字符串），可使用更快的算法：

```rust
    // 使用 fnv crate 的 FNV 哈希
    use fnv::FnvHashMap;
    let mut map: FnvHashMap<&str, i32> = FnvHashMap::default();

    // 本质上等价于
    // type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;
```

哈希计算协议：`BuildHasher`（可重用算法配置）→ `build_hasher()` → `Hasher`（单次使用）→ `hash()` 填入数据 → `finish()` 返回 `u64`。
