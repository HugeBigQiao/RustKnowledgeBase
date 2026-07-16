//! 集合类型(Collections): HashMap, BTreeMap, HashSet, BTreeSet.
//!
//! ## 四个独立的类型, 不是变体
//!
//! HashMap / BTreeMap / HashSet / BTreeSet 各自是独立的泛型类型, 不是
//! 某个父类型的"变体"。但它们有内在联系:
//!   HashSet  ≈ HashMap<T, ()>  (只有键没有值)
//!   BTreeSet ≈ BTreeMap<T, ()>  (同上, 但有序)
//! 所以从概念上分为两组: 底层用哈希的(HashMap/HashSet)和底层用 B 树的
//! (BTreeMap/BTreeSet); 按结构又分为 Map(键→值)和 Set(只有键)。
//!
//! ## 哈希(Hash) vs B 树(BTree) — 先理解底层机制
//!
//! 哈希表: 用哈希函数把键打散到桶里。理想情况 O(1) 查找/插入。
//!          缺点: 键无序, 内存开销较大(桶数组可能有很多空位)。
//!          键必须实现 Hash + Eq trait。
//!
//! B 树:   自平衡多路搜索树, 节点里存多个键, 层级少、缓存友好。
//!          查找/插入 O(log n), 但键有序 → 支持范围查询和按序遍历。
//!          键必须实现 Ord trait。
//!
//! 选择:   无需排序 → HashMap/HashSet (更快);
//!         需要排序/范围查询 → BTreeMap/BTreeSet。
//!
//! ## Map vs Set
//!
//! Map = 键→值的映射 (电话本: 姓名→号码)
//! Set = 只有键, 保证不重复 (黑名单: 名字不能重复出现)
//! Set 本质上就是 Map 的值类型为 () 的版本。
//!
//! ## 生命周期与所有权
//!
//! insert: 键和值的所有权移入集合 (T 类型, 不是 &T)。
//! get:    返回 Option<&V>, 借用, 不转移所有权。
//! 遍历:   for (k, v) in &map → 借用每个元素。
//! 集合被 drop 时, 里面的键和值一起被释放 — 集合拥有它们。
//!
//! 前置依赖: basic/ 中的 vec_type; intermediate/ 中的 generics, traits.


use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};


// ═══════════════════════════════════════════════════════════════
// 第 1 节: HashMap — 哈希键值对 (需要 Hash + Eq)
// ═══════════════════════════════════════════════════════════════
//
// 泛型: HashMap<K, V>  其中 K: Hash + Eq, V: 无约束。
// 场景: 缓存、计数、查找表、配置表 — 任何"通过键查值"的需求。
//
// ── Hash + Eq 是什么? 为什么两个都要? ──
//
// HashMap 查找分两步:
//   Step 1 — Hash(哈希): 键→数字, 决定去哪个桶。Hash trait 要求能算出哈希值。
//   Step 2 — Eq(判等):   不同键可能撞到同一个桶(哈希碰撞), 桶里用 == 逐个比较确认。
//
// 所以 Hash + Eq = 先快速定位桶 + 精确确认身份, 缺一不可。
// 常见类型(i32, String, &str...)都实现了; 自定义类型用 #[derive(Hash, Eq, PartialEq)] 获得。

// ── 辅助: 模拟参数解析, 返回 HashMap ──

/// 用集合作为返回值: 解析 "key=value" 对, 构建 HashMap。
/// 参数: &[&str] — 借切片, 不获取所有权。调用方事后仍可使用原数据。
/// 返回: HashMap<&str, &str> — 键和值都是借来的引用, 生命周期随输入。
/// 适用: 返回集合的函数, 如果数据是借的, 写出生命周期标注。
#[allow(dead_code)]
fn parse_pairs<'a>(pairs: &[&'a str]) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();
    for pair in pairs {
        if let Some((k, v)) = pair.split_once('=') {   // "name=Alice" → k="name", v="Alice"
            map.insert(k, v);                          // 所有权的转移: k, v 都是 &str, Copy, 拷贝进 map
        }
    }
    map                                                // map 持有所有键值引用, 生命周期 ≤ pairs
}
// 注意: 返回 HashMap 时, 如果键/值是引用, 必须标注生命周期 'a。
//       如果键/值都是 String (owned), 不需要生命周期 — 但会有堆分配开销。

// ── 辅助: 构建并返回 HashMap, 带默认值 ──

/// 新建 map 并填充默认值: 常见于"配置表"模式。
#[allow(dead_code)]
fn make_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    config                                              // 所有权转移出去, config 不再可用
}

// ── 辅助: from 迭代器 ──

/// 从键值对数组直接构建 HashMap: from_iter / collect。
#[allow(dead_code)]
fn make_from_pairs() -> HashMap<&'static str, i32> {
    HashMap::from([                                     // from 接受 [(K, V); N]
        ("Alice", 85),
        ("Bob",   72),
        ("Carol", 93),
    ])
}
// 也可以用: [(...)].into_iter().collect::<HashMap<_, _>>();
// from 写静态数组更方便, collect 适合从迭代器动态生成。

pub fn run() {
    // ===== HashMap 基础 =====
    println!("===== HashMap 基础 =====");

    // new() 创建空 map。K, V 由后续 insert 推断。
    let mut scores = HashMap::new();

    // insert(k, v): 键值所有权移入 map。k 和 v 不能再单独使用。
    // 参数: K, V (非引用时 move; 引用 &T 时 Copy 一份引用本身)。
    // 返回: Option<V> — 如果键已存在, 返回旧值; 否则返回 None。
    scores.insert("Alice", 85);                         // "Alice" 是 &str, Copy, 借用进 map
    scores.insert("Bob",   72);
    scores.insert("Carol", 93);
    println!("scores: {:?}", scores);                  // 打印顺序不固定: HashMap 无序

    // get(&key): 通过键查值。参数: &K (借用键), 不对 map 做修改。
    // 返回: Option<&V> — 找到了返回不可变引用, 没找到返回 None。
    // 所有权: 调用前后 map 和调用方都不丢失所有权。
    match scores.get("Alice") {                         // "Alice" 是 &str, 自动转 &K
        Some(score) => println!("Alice: {} 分", score), // score 是 &i32
        None => println!("Alice 不在其中"),
    }
    println!("Dave: {:?}", scores.get("Dave"));         // None

    // entry() 系列 — 见下节
    // remove(&key): 删键并返回旧值 Option<V>。(本 demo 省去)

    // ===== HashMap Entry API =====
    // entry(key): 检查键是否存在的统一入口, 返回 Entry 枚举。
    // Entry 提供链式操作: or_insert / or_insert_with / and_modify。
    println!("\n===== Entry API =====");

    let mut map = HashMap::new();
    map.insert("a", 1);

    // or_insert(default): 存在 → 返回 &mut V (可变引用, 可修改);
    //                     不存在 → 插入 default, 再返回 &mut V。
    // 参数: V (default 值, 如果插入则 move 进 map)。
    let a = map.entry("a").or_insert(0);                // "a" 已存在, 返回 &mut 1
    *a += 10;                                           // * 解引用, 修改为 11

    let b = map.entry("b").or_insert(0);                // "b" 不存在, 插入 0, 返回 &mut 0
    *b += 5;                                            // 修改为 5

    println!("map: {:?}", map);                         // {"a": 11, "b": 5}
    // Entry API 经典模式: 计数 map.entry(word).or_insert(0) += 1;

    // ===== HashMap 遍历 =====
    // 所有权: for (k, v) in &map → 借用, map 之后仍可用。
    //        for (k, v) in map  → 消耗性遍历, map 被 move, 之后不可用。
    println!("\n===== HashMap 遍历 =====");
    for (key, value) in &scores {
        println!("  {} → {}", key, value);
    }
    println!("→ 顺序不固定 (HashMap 无序)");


    // ═══════════════════════════════════════════════════════════
    // 第 2 节: BTreeMap — 有序键值对 (需要 Ord)
    // ═══════════════════════════════════════════════════════════
    //
    // 泛型: BTreeMap<K, V>  其中 K: Ord, V: 无约束。
    // API 几乎和 HashMap 相同: insert/get/entry/remove 完全一致。
    // 差别: 需要 Ord 而非 Hash+Eq; 遍历按键升序; 支持 range 范围查询。
    // 场景: 需要按顺序处理键时 (排行榜、时间线、字典序展示)。
    //
    // ── Ord 是什么? 和 PartialOrd 什么关系? ──
    //
    // Ord = 全序: 任意两个值都能比大小 (<, >, == 三选一, 无歧义)。
    // PartialOrd = 偏序: 有些值"无法比较"。例: f64::NaN ≠ 任何值(包括自己),
    //   所以 f64 只有 PartialOrd, 不能直接当 BTreeMap 键。
    //
    // BTree 是排序树 — 键的位置由比较决定, 有"无法比较"的值就崩了。
    // 所以 BTree 要求 Ord (全序), 不认 PartialOrd。
    //
    // trait 继承链: Ord → PartialOrd + Eq → PartialEq。
    // 类型要当 BTree 键, 实际需四个 trait:
    //   #[derive(Ord, PartialOrd, Eq, PartialEq)] 一行搞定。
    // 常见: i32, String, &str, char。(f32/f64 不实现 Ord)

    println!("\n===== BTreeMap — 有序映射 =====");

    let mut btree = BTreeMap::new();
    btree.insert("zebra",  "斑马");
    btree.insert("apple",  "苹果");
    btree.insert("monkey", "猴子");
    btree.insert("banana", "香蕉");
    // 插入顺序任意, 但存储时自动按 K 排序 (字典序)

    println!("遍历 (自动按键排序):");
    for (en, zh) in &btree {
        println!("  {} → {}", en, zh);                 // apple → banana → monkey → zebra
    }

    // range(start..end): BTreeMap 独有的范围查询。
    // 参数: Range<K> — 键的范围 (按 Ord 比较, 不是索引)。
    // 返回: 迭代器, 按序遍历该范围内的键值对。
    // 所有权: 借用 btree, 不消耗。
    println!("\n范围查询 ('a'..='m'):");               // 'a'..='m' 包含两端
    for (en, zh) in btree.range("a"..="m") {           // "a" <= k <= "m"
        println!("  {} → {}", en, zh);                 // apple, banana (monkey 的 m 在边界内)
    }


    // ═══════════════════════════════════════════════════════════
    // 第 3 节: HashSet — 哈希集合 (需要 Hash + Eq)
    // ═══════════════════════════════════════════════════════════
    //
    // 泛型: HashSet<T>  其中 T: Hash + Eq。
    // 本质: HashMap<T, ()> — 只有键, "值" 是空元组 ()。
    // 场景: 去重、成员判断(白名单/黑名单)、集合运算。

    println!("\n===== HashSet — 哈希集合 =====");

    let mut set = HashSet::new();

    // insert(v): 插入元素。参数: T — 所有权移入。
    // 返回: bool — true=新元素插入成功, false=已存在(重复)。
    // 这个返回值是判断"是否首次添加"的常用手段。
    println!("insert 1: {}", set.insert(1));            // true  ← 新元素
    println!("insert 2: {}", set.insert(2));            // true
    println!("insert 1: {}", set.insert(1));            // false ← 重复, 不插入

    // contains(&v): 检查是否存在。参数: &T — 借用检查。
    println!("contains(&1): {}", set.contains(&1));     // true
    println!("contains(&3): {}", set.contains(&3));     // false

    println!("set: {:?}", set);

    // ── 集合运算 ──
    // 集合运算全部返回迭代器, 不分配新 Set。需要 collect 才变成具体类型。
    println!("\n集合运算:");
    let a: HashSet<_> = [1, 2, 3, 4].into_iter().collect(); // 从数组构建
    let b: HashSet<_> = [3, 4, 5, 6].into_iter().collect();

    println!("A = {:?}", a);
    println!("B = {:?}", b);

    // union(&other)         — A ∪ B  参数: &HashSet  (借用)
    // intersection(&other)  — A ∩ B
    // difference(&other)    — A \ B  (在 A 不在 B)
    // symmetric_difference  — (A\B) ∪ (B\A)  即异或
    // 所有权: 都只借用, 不消耗原集合。

    println!("并集 union:        {:?}", a.union(&b).collect::<Vec<_>>());
    println!("交集 intersection:  {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("差集 difference:    {:?}", a.difference(&b).collect::<Vec<_>>());
    println!("对称差 symmetric:   {:?}", a.symmetric_difference(&b).collect::<Vec<_>>());


    // ═══════════════════════════════════════════════════════════
    // 第 4 节: BTreeSet — 有序集合 (需要 Ord)
    // ═══════════════════════════════════════════════════════════
    //
    // 泛型: BTreeSet<T>  其中 T: Ord。
    // 本质: BTreeMap<T, ()> — 有序版 HashSet。
    // 场景: 需要有序地去重 (如"列出所有出现过的字母, 按字典序输出")。

    println!("\n===== BTreeSet — 有序集合 =====");

    let words = ["rust", "go", "python", "rust", "go", "java"];
    // 从迭代器构建 BTreeSet → 自动去重 + 排序
    let unique: BTreeSet<_> = words.into_iter().collect();  // into_iter 拿所有权, 源数组 move 进 set
    println!("去重+有序: {:?}", unique);                  // "go", "java", "python", "rust"

    // API 和 HashSet 相同: insert, contains, remove, 集合运算。
    let mut btset = BTreeSet::new();
    btset.insert(30);
    btset.insert(10);
    btset.insert(20);
    btset.insert(10);                                    // 重复 — 不会插入
    println!("BTreeSet: {:?}", btset);                   // [10, 20, 30] — 自动排序

    // 独有方法: 
    //   .first() / .last() — 返回最小/最大元素的 Option<&T>
    //   .range(start..end) — 按键范围查询
    println!("最小: {:?}", btset.first());               // Some(10)
    println!("最大: {:?}", btset.last());                // Some(30)

    println!("\n范围 15..: {:?}",                         // 15 到结尾
        btset.range(15..).collect::<Vec<_>>());          // [20, 30]


    // ═══════════════════════════════════════════════════════════
    // 选型总结
    // ═══════════════════════════════════════════════════════════
    println!("\n===== 选型总结 =====");
    println!();
    println!("  类型       结构     底层   键约束     场景");
    println!("  ─────────────────────────────────────────────────────");
    println!("  HashMap    键→值   哈希   Hash+Eq   缓存/计数/查找表");
    println!("  BTreeMap   键→值   B树    Ord       排行榜/字典序/范围查询");
    println!("  HashSet    只有键  哈希   Hash+Eq   去重/黑名单/集合运算");
    println!("  BTreeSet   只有键  B树    Ord       有序去重/前缀范围");

    println!("\n方法构建要点:");
    println!("  • 返回集合的函数: 键/值为引用时标注生命周期; 为 owned 时无需标注。");
    println!("  • 参数用 &[] 或 &[引用]: 借用不获取所有权, 灵活通用。");
    println!("  • 用 from 或 collect 构建: 比循环 insert 更简洁。");
    println!("  • Entry API: 一行搞定\"存在则改、不存在则插\"的逻辑。");
}
