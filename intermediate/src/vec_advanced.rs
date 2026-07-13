//! 向量高级用法: Vec 的常用进阶操作。
//!
//! 前置依赖: basic/ 中的 vec_type; intermediate/ 中的 generics.

use std::collections::HashMap;

// ── 辅助函数 ──

/// 统计单词频率(Vec → HashMap 实践).
fn word_freq<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for &w in words {
        *freq.entry(w).or_insert(0) += 1;
    }
    freq
}

// ── run ──

/// 演示排序/去重/过滤/截取/窗口等 Vec 高级操作。
pub fn run() {
    // ===== 排序与反转 =====
    println!("===== 排序与反转 =====");
    let mut nums = vec![5, 2, 8, 3, 1, 8, 4];
    println!("原始: {:?}", nums);

    // sort: 升序排列(原地修改)
    nums.sort();
    println!("sort: {:?}", nums);

    // reverse: 反转顺序
    nums.reverse();
    println!("reverse: {:?}", nums);

    // sort_by: 自定义比较
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("按长度排序: {:?}", words);

    // ===== 去重 =====
    println!("\n===== 去重 =====");
    let mut dup = vec![1, 2, 2, 3, 3, 3, 4];
    println!("原始: {:?}", dup);

    // dedup: 移除连续重复(需要先 sort)
    dup.sort();
    dup.dedup();
    println!("dedup: {:?}", dup);

    // ===== 过滤与保留 =====
    println!("\n===== 过滤与保留 =====");
    let mut vals = vec![10, 15, 20, 25, 30];

    // retain: 保留满足条件的元素
    vals.retain(|x| *x % 2 == 0);
    println!("retain(偶数): {:?}", vals);

    // ===== 扩展与拼接 =====
    println!("\n===== 扩展与拼接 =====");
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // extend: 将另一个迭代器的元素追加进来
    v1.extend(&v2);
    println!("extend: {:?}", v1);

    // append: 移动另一个 Vec 的所有元素(清空源)
    let mut a = vec!['a', 'b'];
    let mut b = vec!['c', 'd'];
    a.append(&mut b);
    println!("append: a={:?}, b(空)={:?}", a, b);

    // ===== 截取与移除 =====
    println!("\n===== 截取与移除 =====");
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // drain: 移除一个范围的元素, 返回迭代器
    let removed: Vec<_> = v.drain(1..4).collect();
    println!("drain(1..4) → 保留: {:?}, 移除: {:?}", v, removed);

    // split_off: 在指定位置拆分, 返回后半部分
    let mut orig = vec![1, 2, 3, 4, 5];
    let second = orig.split_off(3);
    println!("split_off(3): {:?} 和 {:?}", orig, second);

    // ===== 窗口与分块 =====
    println!("\n===== 窗口与分块 =====");
    let data = [10, 20, 30, 40, 50];

    // windows(n): 大小为 n 的滑动窗口
    print!("windows(3): ");
    for w in data.windows(3) {
        print!("{:?} ", w);
    }
    println!();

    // chunks(n): 分成大小为 n 的块(不重叠)
    print!("chunks(2):  ");
    for c in data.chunks(2) {
        print!("{:?} ", c);
    }
    println!();

    // ===== 实践: 词频统计 =====
    println!("\n===== 实践: 词频统计 =====");
    let text = vec!["rust", "go", "rust", "python", "rust", "go", "java"];
    println!("文本: {:?}", text);

    let freq = word_freq(&text);
    // 按频率排序显示
    let mut entries: Vec<_> = freq.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in &entries {
        println!("  {}: {}", word, count);
    }
}
