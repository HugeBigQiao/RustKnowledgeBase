//! 学生成绩分析器 综合练习
//!
//! 用到的知识点:
//!   Vec, for, 闭包(sum/filter/sort_by), match 范围模式,
//!   函数(含嵌套), 所有权(引用/借用), &str/String, if 表达式,
//!   元组(返回多个值), 基本类型

/// 分析 10 个学生的成绩: 统计、等级评定、排名、及格率.
pub fn run() {
    // ── 原始成绩数据 ──
    let scores = vec![85, 92, 78, 65, 95, 88, 73, 60, 98, 82];

    println!("原始成绩: {:?}", scores);
    println!("========================================\n");

    // ── 基础统计(迭代器链) ──
    print_basic_stats(&scores);

    // ── 最值查找(for 循环) ──
    let (min, max) = find_min_max(&scores);
    println!("最高分: {}", max);
    println!("最低分: {}\n", min);

    // ── 等级评定(match 范围模式) ──
    print_grades(&scores);

    // ── 统计各等级人数(闭包 filter) ──
    print_grade_distribution(&scores);

    // ── 排名(闭包 sort_by) ──
    let mut ranked = scores.clone(); // clone: 深拷贝, 不影响原数据
    ranked.sort_by(|a, b| b.cmp(a)); // 从大到小排序
    println!("排名(高→低): {:?}\n", ranked);

    // ── 嵌套函数: 计算及格率 ──
    let pass_rate = calc_pass_rate(&scores);
    println!("及格率: {:.1}%", pass_rate);
}

// ── 辅助函数 ──

/// 用迭代器链计算总和、平均、计数
fn print_basic_stats(scores: &Vec<i32>) {
    let count = scores.len();
    let total: i32 = scores.iter().sum();
    let avg = total as f64 / count as f64; // as f64: i32 转 f64 避免截断
    println!("人数: {}", count);
    println!("总分: {}", total);
    println!("平均: {:.1}\n", avg);
}

/// 用 for 循环找最小值和最大值(元组返回)
fn find_min_max(scores: &Vec<i32>) -> (i32, i32) {
    let mut min = scores[0];
    let mut max = scores[0];
    for &s in scores {
        if s < min { min = s; }
        if s > max { max = s; }
    }
    (min, max)
}

/// match 范围模式: 分数 → 等级
fn get_grade(score: i32) -> char {
    match score {
        90..=100 => 'A',
        80..=89  => 'B',
        70..=79  => 'C',
        60..=69  => 'D',
        _        => 'F',
    }
}

/// 打印每个人的等级
fn print_grades(scores: &Vec<i32>) {
    println!("--- 等级评定 ---");
    for &s in scores {
        let grade = get_grade(s);
        // if 表达式: 根据等级选不同的提示
        let tip = if grade == 'A' { " ★" } else { "" };
        println!("  {:>3} → {}{}", s, grade, tip);
    }
    println!();
}

/// 闭包 filter: 统计各等级人数
fn print_grade_distribution(scores: &Vec<i32>) {
    // 嵌套函数: 闭包简化版计数(只在这个函数里用)
    fn count_if(scores: &Vec<i32>, check: fn(i32) -> bool) -> usize {
        scores.iter().filter(|&&s| check(s)).count()
    }

    let a = count_if(scores, |s| (90..=100).contains(&s));
    let b = count_if(scores, |s| (80..=89).contains(&s));
    let c = count_if(scores, |s| (70..=79).contains(&s));
    let d = count_if(scores, |s| (60..=69).contains(&s));
    let f = count_if(scores, |s| s < 60);

    println!("--- 各等级人数 ---");
    println!("  A(90-100): {}", a);
    println!("  B(80-89) : {}", b);
    println!("  C(70-79) : {}", c);
    println!("  D(60-69) : {}", d);
    println!("  F(<60)   : {}", f);
    println!();
}

/// 计算及格率(嵌套函数示例)
fn calc_pass_rate(scores: &Vec<i32>) -> f64 {
    // 嵌套函数: 只在 calc_pass_rate 内部使用, 不能加 pub.
    fn is_pass(score: i32) -> bool {
        score >= 60
    }

    let pass_count = scores.iter().filter(|&&s| is_pass(s)).count();
    let total = scores.len();
    (pass_count as f64 / total as f64) * 100.0
}
