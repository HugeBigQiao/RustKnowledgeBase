//! 向量高级用法: Vec 的常用进阶操作。
//!
//! 前置依赖: basic/ 中的 vec_type; intermediate/ 中的 generics.

use std::collections::HashMap;

// ── 辅助函数 ──

/// 统计单词频率(Vec → HashMap 实践)。
/// 参数: &[&str] — 切片引用, 不获取所有权, 调用后 words 可继续用。
/// 返回: HashMap<&str, usize> — 键是单词引用(生命周期跟随输入), 值是出现次数。
/// 核心: entry(w).or_insert(0) — 有则返回值的可变引用, 无则插入默认值 0 再返回引用。
///       *freq.entry(w).or_insert(0) += 1 — 解引用后 +1。
fn word_freq<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for &w in words {                                // &w 模式: 遍历时自动解引用, w 是 &str
        *freq.entry(w).or_insert(0) += 1;            // entry+or_insert: HashMap 经典"计数"模式
    }
    freq
}

// ── run ──

/// 演示排序/去重/过滤/截取/窗口等 Vec 高级操作。
pub fn run() {
    // ==================== 排序与反转 ====================
    // 所有权: 全部是 &mut self, 原地修改, 不会转移所有权。
    println!("===== 排序与反转 =====");
    let mut nums = vec![5, 2, 8, 3, 1, 8, 4];
    println!("原始: {:?}", nums);
    
    // sort(): 升序排列。要求 T: Ord (元素能比大小)。
    // 元素类型若没实现 Ord, 用 sort_by 或 sort_by_key 代替。
    nums.sort();
    println!("sort: {:?}", nums);
    
    // reverse(): 首尾颠倒, 纯位置操作, 不要求 Ord。
    nums.reverse();
    println!("reverse: {:?}", nums);
    
    // sort_by(|a, b| ...): 自定义比较规则。
    // 闭包参数: 两个不可变引用 &T, &T。
    // 返回 Ordering (通过 .cmp() 获得): Less / Equal / Greater。
    // 适用场景: 按结构体字段排、按长度排、按自定义权重排。
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));       // 闭包捕获: 无外部变量
    println!("按长度排序: {:?}", words);
    
    // sort_by_key(|item| key): sort_by 的简化版, 只指定提取 key, 用 key 比较。
    // 等价 words.sort_by_key(|s| s.len()); — 不需要手动调 .cmp()。
    // 适用: key 是 Copy 类型 (i32/usize/char 等) 或实现了 Ord。

    // ==================== 去重 ====================
    // dedup(): 移除连续重复。只删相邻的重复, 不排序的话可能漏删。
    // 所有权: &mut self, 原地操作。返回什么都不做 (没有新 Vec)。
    // 所以去重前通常先 sort(), 把相同元素排到一起。
    println!("\n===== 去重 =====");
    let mut dup = vec![1, 2, 2, 3, 3, 3, 4];
    println!("原始: {:?}", dup);
    
    dup.sort();                                       // 先排序, 相同元素聚拢
    dup.dedup();                                      // 移除相邻重复
    println!("dedup: {:?}", dup);
    
    // 想保留原始顺序的同时去重? dedup 做不到, 需要其他方案:
    // 1. 用 HashSet 记录已见过的元素 (适合小数据)
    // 2. 用 retain + HashSet 结合 (保留第一次出现)

    // ==================== 过滤与保留 ====================
    // retain(|x| bool): 保留闭包返回 true 的元素, 其余删除。
    // 所有权: &mut self, 原地操作。原 Vec 被修改, 不需要收集新 Vec。
    // 闭包参数: &T (不可变引用), 不能修改元素。
    // 适用: 按条件精简数据, 比 filter().collect() 更省内存 (不分配新 Vec)。
    println!("\n===== 过滤与保留 =====");
    let mut vals = vec![10, 15, 20, 25, 30];
    
    vals.retain(|x| *x % 2 == 0);                    // *x 解引用: 闭包拿到的是 &i32, 需要 * 拿到值
    println!("retain(偶数): {:?}", vals);
    // 对比: 如果用 filter: let evens: Vec<_> = vals.iter().filter(|x| **x % 2 == 0).collect();
    // retain 原地改, filter 产生新 Vec → 有大量数据时 retain 更省内存。

    // ==================== 扩展与拼接 ====================
    // extend 和 append 都往 Vec 加元素, 但所有权行为完全不同。
    println!("\n===== 扩展与拼接 =====");
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    
    // extend(iter): 接收任何可迭代的东西 (实现了 IntoIterator)。
    // 所有权: 只借 v2, 遍历后把元素复制/克隆进 v1。v2 仍然可用。
    // 参数: &v2 产生迭代器, 每次 yield &i32, v1 里存的是解引用后的 i32。
    v1.extend(&v2);                                   // &v2 → 借用, v2 还在
    println!("extend: {:?}", v1);
    // println!("{:?}", v2);                         // v2 依然可用!
    
    // append(&mut other): 把另一个 Vec 的元素"搬"过来。
    // 所有权: 移动! 源 Vec 被清空 (变成空 vec), 元素的所有权转移到目标。
    // 参数: &mut other — 需要可变引用, 因为要对源做清空操作。
    // 适用: 两个 Vec 彻底合并、不再需要源数据时 (比 extend 少一次拷贝)。
    let mut a = vec!['a', 'b'];
    let mut b = vec!['c', 'd'];
    a.append(&mut b);                                 // b 的元素被移走, b 变空
    println!("append: a={:?}, b(空)={:?}", a, b);
    // b 此时 = [], 但变量 b 本身仍然有效 (绑定还在, 只是内容空了)。

    // ==================== 截取与移除 ====================
    // drain 和 split_off 都从 Vec 取出一部分, 区别在于取走的方式。
    println!("\n===== 截取与移除 =====");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    
    // drain(range): 移除指定范围 [start..end), 返回一个迭代器。
    // 所有权: 元素的拥有权从 Vec 转移到迭代器, 可以用 .collect() 收回来。
    // 原 Vec 里该范围的元素被删除, 剩余元素前移 (类似 remove 的批量版)。
    // 参数: Range 类型 (1..4  表示索引 [1, 2, 3])。
    let removed: Vec<_> = v.drain(1..4).collect();   // 取走索引 1、2、3
    println!("drain(1..4) → 保留: {:?}, 移除: {:?}", v, removed);
    
    // split_off(at): 在索引 at 处切一刀, 后半部分 [at..] 移入新 Vec。
    // 所有权: &mut self, 分出去的 Vec 完全独立 (新 owner)。
    // 参数: usize — 从哪里切 (0 = 全部分走; len = 全保留, 返回空)。
    // 适用: 分块处理, 比如把任务队列按批次拆分。
    let mut orig = vec![1, 2, 3, 4, 5];
    let second = orig.split_off(3);                  // 从索引 3 处切开
    println!("split_off(3): {:?} 和 {:?}", orig, second);
    // 对比: drain 取走中间一段 (剩余连起来), split_off 取走后半段 (简单二分)。

    // ==================== 窗口与分块 ====================
    // windows 和 chunks 都不改变原数据, 返回的是只读切片迭代器。
    // 所有权: 都是 &self (不可变借用), 不移动数据。
    println!("\n===== 窗口与分块 =====");
    let data = [10, 20, 30, 40, 50];
    
    // windows(n): 大小为 n 的滑动窗口, 步长为 1 (重叠)。
    // 返回 &[T] — 每个窗口是对原数据的只读视图。
    // 参数: usize n — 窗口大小。n > len 时返回空迭代器。
    // 适用: 检查相邻元素 (如"有没有连续的三个递增?")、滑动平均。
    print!("windows(3): ");
    for w in data.windows(3) {                       // [10,20,30],[20,30,40],[30,40,50]
        print!("{:?} ", w);
    }
    println!();
    
    // chunks(n): 分成大小为 n 的非重叠块, 最后一块可能不足 n。
    // 返回 &[T] — 同样是对原数据的只读视图。
    // 参数: usize n — 每块大小。
    // 适用: 分页显示、分批处理 (一次处理固定数量)。
    print!("chunks(2):  ");
    for c in data.chunks(2) {                        // [10,20],[30,40],[50]
        print!("{:?} ", c);
    }
    println!();
    
    // 配套: chunks_exact(n) — 只返回刚好 n 个的块, 最后不足 n 的丢弃。
    //       rchunks(n) — 从尾部开始分块, 最后剩余部分在最前面。

    // ==================== 实践: 词频统计 ====================
    // 综合运用: Vec + HashMap + 迭代器闭包。
    println!("\n===== 实践: 词频统计 =====");
    let text = vec!["rust", "go", "rust", "python", "rust", "go", "java"];
    println!("文本: {:?}", text);
    
    let freq = word_freq(&text);
    
    // freq 是 HashMap, 不能直接 sort。转到 Vec 再排:
    //   freq.iter() → 产生 (&K, &V) 迭代器 (借用)
    //   .collect()  → 收集为 Vec<(&str, &usize)>
    let mut entries: Vec<_> = freq.iter().collect();
    // 按值从大到小: b.1.cmp(a.1) (b 在前 = 降序)
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in &entries {
        println!("  {}: {}", word, count);
    }
}
