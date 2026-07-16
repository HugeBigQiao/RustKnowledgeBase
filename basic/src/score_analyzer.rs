//! 学生成绩分析器 综合练习
//!
//! 用到的知识点:
//!   Vec, for, 闭包(sum/filter/sort_by), match 范围模式,
//!   函数(含嵌套), 所有权(引用/借用), &str/String, if 表达式,
//!   元组(返回多个值), 基本类型

/// 用 for 循环找最小值和最大值(元组返回)
fn find_min_max(scores: &Vec<i32>) -> (i32, i32) {
    // scores 是 &Vec<i32>(引用), 本身就没有所有权, 只是借来的.
    // scores[0] 返回 &i32, 但 i32 是 Copy 类型, 自动解引用复制出值, 不动原数据.
    //
    // 注: Vec 没有直接内置 .min() / .max() 方法, 需要通过迭代器:
    //   scores.iter().min()  -> Option<&i32>  (可选: .copied() 解引用)
    //   scores.iter().max()  -> Option<&i32>
    //
    // 两种写法对比:
    //   手写循环: 一次遍历找出两个值, 只需扫一遍数据. 缺点: 假设非空, 空 Vec 会 panic.
    //   迭代器:   代码短. 缺点: .min() 和 .max() 各扫一遍, 等于扫了两遍.
    //   折中方案: itertools 库的 .minmax() 一次扫出两个, 但不属于标准库.
    //   这里手写循环是为了演示 for + if, 同时一次遍历找出两个值.
    let mut min = scores[0];
    let mut max = scores[0];
    for &s in scores {
        if s < min {
            min = s;
        }
        if s > max {
            max = s;
        }
    }
    (min, max)
}

/// 打印每个人的等级
fn print_grades(scores: &Vec<i32>) {
    println!("--- 等级评定 ---");
    for &s in scores {
        let grade = match s {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        };
        // if 表达式: 根据等级选不同的提示
        let tip = if grade == 'A' { " ★" } else { "" };
        println!("  {:>3} -> {}{}", s, grade, tip);
    }
    println!();
}

/// 统计各等级人数: filter + count 一行搞定
fn print_grade_distribution(scores: &Vec<i32>) {
    // filter(|&&s| ...) 中 &&s 是什么意思?
    //   第 1 层 & : scores.iter() 产生的元素是 &i32
    //   第 2 层 & : filter 把元素"借给"闭包, 又套了一层, 变成 &&i32
    //     为什么? filter/map 等迭代器适配器不会拿走元素所有权, 而是把每个元素
    //     以"引用"的形式传给闭包. 由于 iter() 的元素本身就是 &i32, 再取引用就
    //     成了 &&i32. 详见 closure.rs 的 filter 章节.
    //   &&s 用模式解构剥掉这两层, 所以闭包里的 s 直接是 i32.
    //   注意: 这里的 && 是模式匹配语法(剥引用), 不是解引用操作符 **.
    //   &&s 的效果等价于 let s = **x; 只是更短的写法.
    //
    // 因为 s 现在是 i32, 而 contains 要求 &i32:
    //   (90..=100).contains(&s)  -> 重新取 &s 传引用
    //   而 s < 60 不需要 &, i32 可以直接比较.
    //
    // (start..=end).contains(&value): 范围方法, 判断 value 是否在 [start, end] 内.
    let a = scores.iter().filter(|&&s| (90..=100).contains(&s)).count();
    let b = scores.iter().filter(|&&s| (80..=89).contains(&s)).count();
    let c = scores.iter().filter(|&&s| (70..=79).contains(&s)).count();
    let d = scores.iter().filter(|&&s| (60..=69).contains(&s)).count();
    let f = scores.iter().filter(|&&s| s < 60).count();

    println!("--- 各等级人数 ---");
    println!("  A(90-100): {}", a);
    println!("  B(80-89) : {}", b);
    println!("  C(70-79) : {}", c);
    println!("  D(60-69) : {}", d);
    println!("  F(<60)   : {}", f);
    println!();
}

/// 计算及格率
fn calc_pass_rate(scores: &Vec<i32>) -> f64 {
    // 及格判断就是 s >= 60
    let pass_count = scores.iter().filter(|&&s| s >= 60).count();
    let total = scores.len();
    (pass_count as f64 / total as f64) * 100.0
}

/// 分析 10 个学生的成绩: 统计、等级评定、排名、及格率.
pub fn run() {
    // --- 原始成绩数据 ---
    // scores 拥有 Vec<i32>, 包含 10 个 i32 值 (Copy, 所以直接存在 Vec 堆内存里)。
    let scores = vec![85, 92, 78, 65, 95, 88, 73, 60, 98, 82];

    println!("原始成绩: {:?}", scores);
    println!("========================================\n");

    // --- 基础统计(迭代器链) ---
    // 这里都不会消耗所有权，len，iter，sum方法内部都是&self的引用。
    let count = scores.len();
    let total: i32 = scores.iter().sum();            // .iter() 借 &self → sum 消费迭代器, 不消费 Vec
    let avg = total as f64 / count as f64; // as f64: i32 转 f64 避免截断
    println!("人数: {}", count);
    println!("总分: {}", total);
    println!("平均: {:.1}\n", avg);

    // --- 最值查找(for 循环) ---
    // &scores: 借 Vec, find_min_max 返回 (i32, i32) — Copy 值, 不借原数据。
    let (min, max) = find_min_max(&scores);
    println!("最高分: {}", max);
    println!("最低分: {}\n", min);

    // --- 等级评定(match 范围模式) ---
    // &scores: 借 Vec → print_grades 里面 for &s in scores 遍历, 不消耗。
    print_grades(&scores);

    // --- 统计各等级人数(闭包 filter) ---
    // &scores: 借 Vec → filter 链式操作, 只借不消耗。
    print_grade_distribution(&scores);

    // --- 排名(闭包 sort_by) ---
    let mut ranked = scores.clone(); // clone: 深拷贝, 不影响原数据。ranked 拥有新的 Vec<i32>。
    ranked.sort_by(|a, b| b.cmp(a)); // sort_by: &mut self 可变借用, 原地排序, ranked 被修改
    println!("排名(高->低): {:?}\n", ranked);

    // --- 嵌套函数: 计算及格率 ---
    // &scores: 借 Vec → calc_pass_rate 只读操作, 返回 f64。
    let pass_rate = calc_pass_rate(&scores);
    println!("及格率: {:.1}%", pass_rate);
}
