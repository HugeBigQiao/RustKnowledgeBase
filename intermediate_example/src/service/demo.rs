//! 业务编排层: 将各个 model 组合成完整的演示流程.

use crate::models::category::Category;
use crate::models::library::Library;

/// 运行完整演示, 覆盖所有 intermediate 知识点.
pub fn run() {
    println!("╔══════════════════════════════════════╗");
    println!("║   Rust 图书管理系统 — 综合实践      ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // ===== 1. 创建图书馆(结构体 + impl) =====
    let mut lib = Library::new();
    println!("--- 创建空图书馆 ---");
    println!("图书馆已初始化, 当前藏书: {}本", lib.stats().total);

    // ===== 2. 添加图书(Result 错误处理) =====
    println!("\n--- 添加图书 ---");
    match lib.add_book(
        "Rust 程序设计".into(),
        "Klabnik".into(),
        Category::Technology,
        2018,
        vec!["rust", "编程", "入门"],
    ) {
        Ok(id) => println!("添加成功, ID={}", id),
        Err(e) => println!("添加失败: {}", e),
    }

    lib.add_book(
        "三体".into(),
        "刘慈欣".into(),
        Category::Fiction,
        2008,
        vec!["科幻", "中国"],
    ).ok();

    lib.add_book(
        "人类简史".into(),
        "Harari".into(),
        Category::History,
        2014,
        vec!["历史", "科普"],
    ).ok();

    lib.add_book(
        "算法导论".into(),
        "CLRS".into(),
        Category::Technology,
        2009,
        vec!["算法", "计算机科学", "经典"],
    ).ok();

    lib.add_book(
        "苏菲的世界".into(),
        "Gaarder".into(),
        Category::Philosophy,
        1991,
        vec!["哲学", "小说", "入门"],
    ).ok();

    lib.add_book(
        "时间简史".into(),
        "Hawking".into(),
        Category::Science,
        1988,
        vec!["物理", "科普", "经典"],
    ).ok();

    // ===== 3. 尝试错误输入(错误处理演示) =====
    println!("\n--- 错误处理演示 ---");
    match lib.add_book("".into(), "?".into(), Category::Fiction, 2000, vec![]) {
        Ok(id) => println!("添加成功 ID={}", id),
        Err(e) => println!("预期错误: {}", e),
    }

    match lib.remove_book(999) {
        Ok(_) => println!("删除成功"),
        Err(e) => println!("预期错误: {}", e),
    }

    // ===== 4. 按 ID 查询(Option) =====
    println!("\n--- Option 查询 ---");
    match lib.get_book(2) {
        Some(book) => println!("找到: {}", book),
        None => println!("未找到"),
    }
    match lib.get_book(999) {
        Some(book) => println!("找到: {}", book),
        None => println!("ID=999: 不存在 (Option::None)"),
    }

    // ===== 5. 搜索(生命周期 + 泛型) =====
    println!("\n--- 模糊搜索 ---");
    let results = lib.search_by_title("Rust");
    println!("搜索书名含'Rust': {} 本", results.len());
    for b in &results {
        println!("  {}", b);
    }

    let results = lib.search_by_author("Hawking");
    println!("搜索作者含'Hawking': {} 本", results.len());
    for b in &results {
        println!("  {}", b);
    }

    // 泛型搜索: 2000 年之后出版的技术书
    println!("\n--- 泛型搜索(2000年后+技术) ---");
    let results = lib.search(|b| b.year > 2000 && matches!(b.category, Category::Technology));
    println!("2000年后出版的技术书: {} 本", results.len());
    for b in &results {
        println!("  {}", b);
    }

    // ===== 6. 列表排序(Vec 高级) =====
    println!("\n--- 全部图书(按年份排序) ---");
    for b in lib.list_all() {
        println!("  {}", b);
    }

    // ===== 7. 按分类筛选 =====
    println!("\n--- 技术类图书 ---");
    let tech = lib.list_by_category(&Category::Technology);
    for b in &tech {
        println!("  {}", b);
    }

    // ===== 8. 统计(HashMap + BTreeMap + HashSet) =====
    println!("\n--- 图书馆统计 ---");
    println!("{}", lib.stats());

    // ===== 9. 删除图书 =====
    println!("--- 删除图书 ---");
    match lib.remove_book(3) {
        Ok(book) => println!("已删除: {}", book.title),
        Err(e) => println!("{}", e),
    }
    println!("删除后: {} 本", lib.stats().total);

    // ===== 10. 枚举模式匹配 =====
    println!("\n--- 模式匹配示例 ---");
    let cats = vec![
        Category::Fiction,
        Category::Technology,
        Category::Other("漫画".into()),
    ];
    for cat in &cats {
        match cat {
            Category::Fiction => println!("  分类: 小说"),
            Category::Technology => println!("  分类: 技术"),
            Category::Other(s) => println!("  分类: 自定义({})", s),
            _ => println!("  分类: 其他"),
        }
    }

    // ===== 总结 =====
    println!("\n══════════════════════════════════════");
    println!("  演示完成! 涉及的知识点:");
    println!("  struct/enum/impl, 模式匹配, Option, Result/?, ");
    println!("  生命周期, 泛型, Trait(Display/From/Error), ");
    println!("  Vec 高级(sort/filter), HashMap/Entry API, ");
    println!("  HashSet, BTreeMap, 自定义错误.");
    println!("══════════════════════════════════════");
}

