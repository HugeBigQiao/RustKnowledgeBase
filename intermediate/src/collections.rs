//! 集合类型: HashMap(哈希映射)、HashSet(哈希集合)、BTreeMap(有序映射)。
//!
//! 前置依赖: basic/ 中的 vec_type; intermediate/ 中的 generics.

use std::collections::{HashMap, HashSet, BTreeMap};

// ── run ──

/// 演示 HashMap(键值对)、HashSet(不重复集合)、BTreeMap(有序映射).
pub fn run() {
    // ===== HashMap 基础 =====
    println!("===== HashMap 基础 =====");
    // HashMap: 键 → 值 的映射, 基于哈希表, 无序但查找快.

    let mut scores = HashMap::new();

    // insert: 插入键值对
    scores.insert("Alice", 85);
    scores.insert("Bob",   72);
    scores.insert("Carol", 93);
    println!("scores: {:?}", scores);

    // get: 获取值(返回 Option<&V>)
    match scores.get("Alice") {
        Some(score) => println!("Alice: {} 分", score),
        None => println!("Alice 不在其中"),
    }
    // 不存在的键返回 None
    println!("Dave: {:?}", scores.get("Dave"));

    // ===== HashMap Entry API =====
    println!("\n===== Entry API =====");
    // entry: 检查键是否存在, 然后决定插入还是修改.

    let mut map = HashMap::new();
    map.insert("a", 1);

    // or_insert: 存在则返回可变引用, 不存在则插入默认值并返回引用.
    let a = map.entry("a").or_insert(0);
    *a += 10;  // 修改已存在的值

    let b = map.entry("b").or_insert(0);
    *b += 5;   // 插入 b=0, 然后加 5

    println!("map: {:?}", map);  // {"a": 11, "b": 5}

    // ===== HashMap 遍历 =====
    println!("\n===== HashMap 遍历 =====");
    for (key, value) in &scores {
        println!("  {} → {}", key, value);
    }
    // 注意: HashMap 遍历顺序不固定.

    // ===== HashSet 基础 =====
    println!("\n===== HashSet 基础 =====");
    // HashSet: 只存键不存值, 保证元素不重复, 基于哈希表.

    let mut set = HashSet::new();

    // insert: 插入元素, 返回 true(新元素) 或 false(已存在)
    println!("insert 1: {}", set.insert(1));  // true
    println!("insert 2: {}", set.insert(2));  // true
    println!("insert 1: {}", set.insert(1));  // false(重复)

    // contains: 检查是否存在
    println!("contains(1): {}", set.contains(&1));
    println!("contains(3): {}", set.contains(&3));

    println!("set: {:?}", set);

    // ===== HashSet 集合运算 =====
    println!("\n===== HashSet 集合运算 =====");
    let a: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();

    println!("A = {:?}", a);
    println!("B = {:?}", b);

    // 并集: A ∪ B
    println!("并集(union)       : {:?}", a.union(&b).collect::<Vec<_>>());

    // 交集: A ∩ B
    println!("交集(intersection): {:?}", a.intersection(&b).collect::<Vec<_>>());

    // 差集: A - B
    println!("差集(difference)  : {:?}", a.difference(&b).collect::<Vec<_>>());

    // 对称差: (A ∪ B) - (A ∩ B)
    println!("对称差(symmetric) : {:?}", a.symmetric_difference(&b).collect::<Vec<_>>());

    // ===== BTreeMap 有序映射 =====
    println!("\n===== BTreeMap 有序映射 =====");
    // BTreeMap: 键按顺序排列的映射, 基于 B 树.

    let mut btree = BTreeMap::new();
    btree.insert("zebra",  "斑马");
    btree.insert("apple",  "苹果");
    btree.insert("monkey", "猴子");
    btree.insert("banana", "香蕉");

    // BTreeMap 遍历自动按键排序
    println!("BTreeMap(自动排序):");
    for (en, zh) in &btree {
        println!("  {} → {}", en, zh);
    }

    // 范围查询: 获取一个范围内的键值对
    println!("\n范围查询('a'..'m'):");
    for (en, zh) in btree.range("a".."m") {
        println!("  {} → {}", en, zh);
    }

    // ===== 选型总结 =====
    println!("\n===== 选型总结 =====");
    println!("HashMap : 键值对, 无序遍历快,  用于缓存/计数/查找.");
    println!("HashSet : 不重复集合,        用于去重/集合运算.");
    println!("BTreeMap: 键值对, 有序遍历,    用于需要排序/范围查询.");
    println!("BTreeSet: 有序不重复集合,     用于有序去重.");
}
