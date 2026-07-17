//! 数据处理: rust_xlsxwriter (Excel 写入) + polars (DataFrame 分析引擎)。
//!
//! polars 是 Rust 生态中高性能 DataFrame 库, 对标 Python 的 pandas。
//! 核心抽象: DataFrame (二维表) / Series (单列) / LazyFrame (惰性求值)。
//! 这里演示: 创建 Excel → 用 polars 读 CSV 做分析 → 把 Excel 数据桥接到 polars。

use std::fs;
use std::fs::File;

// ==================== Part 1: Excel 写入 ====================

/// 用 rust_xlsxwriter 创建 .xlsx 文件作为数据源。
fn demo_create_excel() -> String {
    println!("--- Part 1: 创建 Excel 数据 (rust_xlsxwriter) ---");

    let path = "advanced_sales.xlsx";

    {
        use rust_xlsxwriter::Format;

        let mut wb = rust_xlsxwriter::Workbook::new();
        let sheet = wb.add_worksheet();

        // 表头
        sheet.write_string(0, 0, "产品", &Format::default()).unwrap();
        sheet.write_string(0, 1, "销量", &Format::default()).unwrap();
        sheet.write_string(0, 2, "单价", &Format::default()).unwrap();

        // 数据
        let data = [
            ("键盘", 120.0, 299.0),
            ("鼠标", 350.0, 89.0),
            ("显示器", 45.0, 1899.0),
            ("耳机", 200.0, 159.0),
            ("音箱", 80.0, 499.0),
        ];
        for (i, (name, qty, price)) in data.iter().enumerate() {
            sheet.write_string((i + 1) as u32, 0, name, &Format::default()).unwrap();
            sheet.write_number((i + 1) as u32, 1, *qty, &Format::default()).unwrap();
            sheet.write_number((i + 1) as u32, 2, *price, &Format::default()).unwrap();
        }

        wb.save(path).unwrap();
        println!("  已写入 {} (5 条销售记录)", path);
    }

    // 同时生成一份 CSV (用于 polars 练习)
    let csv_path = "advanced_sales.csv";
    fs::write(
        csv_path,
        "产品,销量,单价\n\
         键盘,120,299\n\
         鼠标,350,89\n\
         显示器,45,1899\n\
         耳机,200,159\n\
         音箱,80,499\n",
    )
    .unwrap();
    println!("  已写入 {} (CSV 备份)", csv_path);

    path.to_string()
}

// ==================== Part 2: polars DataFrame 分析 ====================

fn demo_polars_csv() {
    println!("\n--- Part 2: polars DataFrame (CSV → 分析) ---");

    // 1. 读 CSV → DataFrame
    // has_header(true): 第一行是列名; infer_schema: 自动推断列类型
    let df = polars::prelude::CsvReader::new(File::open("advanced_sales.csv").unwrap())
        .has_header(true)
        .finish()
        .unwrap();

    // 2. shape() → (行数, 列数)
    let (rows, cols) = (df.height(), df.width());
    println!("行列: {} 行 × {} 列\n", rows, cols);

    // 3. 全表打印
    println!("=== 全表 ===");
    println!("{}", df);
    println!();

    // 4. head(n) → 前 N 行预览
    println!("=== head(3) 前 3 行 ===");
    println!("{}", df.head(Some(3)));
    println!();

    // 5. 列名 + 列类型
    println!("=== 列信息 ===");
    for col in df.get_columns() {
        println!("  '{}': {:?}", col.name(), col.dtype());
    }
    println!();

    // 6. describe() → 数值列统计摘要
    // 原理: 对每列数值计算 count/mean/std/min/25%/50%/75%/max
    //        polars 惰性求值, 只遍历一次数据就完成全部统计
    println!("=== describe() 数值列统计 ===");
    if let Ok(stats) = df.describe(None) {
        println!("{}", stats);
    }
    println!();

    // 7. 过滤: 价格 > 200 的产品
    use polars::prelude::*;
    let filtered = df
        .clone()
        .lazy()                          // 转为惰性求值 (构建执行计划)
        .filter(col("单价").gt(lit(200.0))) // 过滤条件: 单价 > 200
        .collect()                       // 执行计划 → DataFrame
        .unwrap();
    println!("=== 筛选: 单价 > 200 ===");
    println!("{}", filtered);
    println!();

    // 8. 聚合: 按销量排序 (降序)
    let sorted = df
        .clone()
        .lazy()
        .sort(["销量"], SortMultipleOptions::default().with_order_descending(true))
        .collect()
        .unwrap();
    println!("=== 排序: 销量降序 ===");
    println!("{}", sorted);
    println!();

    // 9. 计算新列: 销售额 = 销量 × 单价
    let with_revenue = df
        .clone()
        .lazy()
        .with_column((col("销量") * col("单价")).alias("销售额"))
        .sort(["销售额"], SortMultipleOptions::default().with_order_descending(true))
        .collect()
        .unwrap();
    println!("=== 新增列: 销售额 = 销量 × 单价 (降序) ===");
    println!("{}", with_revenue);
}

// ==================== Part 3: Excel → calamine → polars ====================

/// 演示多工具协作: calamine 读 Excel → 转 polars DataFrame。
fn demo_excel_to_polars(excel_path: &str) {
    println!("\n--- Part 3: Excel → polars (calamine 桥接) ---");

    // 1. calamine 读取 xlsx
    use calamine::Reader;
    let mut wb = calamine::open_workbook_auto(excel_path).unwrap();
    println!("Sheet: {:?}", wb.sheet_names());

    if let Some(Ok(range)) = wb.worksheet_range_at(0) {
        let rows: Vec<Vec<String>> = range
            .rows()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        calamine::Data::String(s) => s.clone(),
                        calamine::Data::Float(f) => format!("{}", f),
                        calamine::Data::Int(i) => format!("{}", i),
                        _ => String::new(),
                    })
                    .collect()
            })
            .collect();

        // 2. 手动构建 polars DataFrame
        // 第一行是 header, 后续是数据
        if rows.len() > 1 {
            use polars::prelude::*;

            let headers = &rows[0];
            let data_rows = &rows[1..];

            // 为每列创建 Series (类型根据列名推断)
            let mut columns: Vec<Column> = Vec::new();
            for col_idx in 0..headers.len() {
                let col_name: &str = &headers[col_idx];

                if col_name == "产品" {
                    let vals: Vec<&str> = data_rows.iter().map(|r| r[col_idx].as_str()).collect();
                    columns.push(Column::new(col_name.into(), vals.as_slice()));
                } else {
                    // 销量/单价 → f64
                    let vals: Vec<f64> = data_rows
                        .iter()
                        .map(|r| r[col_idx].parse::<f64>().unwrap_or(0.0))
                        .collect();
                    columns.push(Column::new(col_name.into(), vals));
                }
            }

            let df = DataFrame::new(columns).unwrap();
            println!("行列: {} 行 × {} 列", df.height(), df.width());
            println!("{}", df);
            println!();
            println!("说明: calamine 读 Excel 得到原始矩阵数据");
            println!("      手动构建 polars DataFrame 后, 即可用 Part 2 的筛选/排序/聚合能力");
        }
    }
}

// ==================== 汇总 ====================

pub fn run() {
    println!("══════════ 数据处理: Excel + polars ══════════\n");

    let excel_path = demo_create_excel();
    demo_polars_csv();
    demo_excel_to_polars(&excel_path);

    // 清理
    let _ = fs::remove_file(&excel_path);
    let _ = fs::remove_file("advanced_sales.csv");

    println!("\n══════════ 总结 ══════════");
    println!("工具              用途              核心抽象");
    println!("────────────────────────────────────────────");
    println!("rust_xlsxwriter    Excel 写入         Workbook / Worksheet");
    println!("calamine           Excel 读取         行列矩阵 (Range<Data>)");
    println!("polars             DataFrame 分析    DataFrame / Series / LazyFrame");
    println!();
    println!("工作流: 数据源(Excel/CSV/DB) → 读入 → polars DataFrame → 筛选/排序/聚合 → 导出");
}
