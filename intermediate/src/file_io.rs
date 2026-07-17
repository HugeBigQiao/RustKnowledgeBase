//! 文件 I/O: 基础读写 / JSON / CSV 格式处理。
//!
//! 标准库提供最底层的字节流读写 (Part 1)。
//! 结构化格式 (JSON/CSV) 依赖第三方 crate 做序列化/反序列化 (Part 2~3)。
//! 核心概念: 序列化 = Rust 类型 → 存储格式; 反序列化 = 存储格式 → Rust 类型。
//! 贯穿全文关注: 每步读出来的是什么类型、行数/列数怎么获取、所有权如何流转。

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

// serde: Rust 标准序列化框架
//   Serialize:   把 Rust 类型 → 可传输格式 (JSON / CSV / ...)
//   Deserialize: 从可传输格式 → Rust 类型
//   #[derive] 自动生成实现代码, 无需手写
use serde::{Deserialize, Serialize};

// ==================== Part 1: 基础文件 I/O (标准库) ====================

/// 演示最底层: 文件读到的是 Vec<u8> (原始字节流)。
/// 所有权: fs::read 内部分配堆内存 → Vec<u8> 所有权 move 给调用方。
fn demo_raw_bytes() {
    println!("--- Part 1.0: 原始字节读取 ---");

    let path = "intermediate_raw.txt";
    fs::write(path, "Hello, Rust!").unwrap();

    // fs::read: 返回 Vec<u8> (owned) — 所有权从函数内部 move 出来
    let bytes: Vec<u8> = fs::read(path).unwrap();
    println!("原始字节: {:?}", bytes);    // [72, 101, 108, ...] — ASCII 码
    println!("  长度: {} 字节", bytes.len()); // 12 字节

    // from_utf8: 校验字节是否为合法 UTF-8, 返回 Result<String, _>
    // 所有权: bytes 被 move 进 from_utf8, 此后 bytes 不可用
    let s = String::from_utf8(bytes).unwrap();
    // bytes 已被消耗, println!("{:?}", bytes) 会报错: value borrowed after move
    println!("转 String: '{}'\n", s);

    fs::remove_file(path).unwrap();
}

fn demo_simple_read_write() {
    println!("--- Part 1.1: 一次性读写 ---");

    let path = "intermediate_demo.txt";

    // write: &str → 文件。文件不存在则创建, 存在则覆盖。一次全量写入。
    fs::write(path, "Hello, Rust!\n第二行\n第三行\n").unwrap();
    println!("已写入: {}", path);

    // read_to_string: 文件 → String (owned)。一次读全部到内存, 适合小文件。
    let s = fs::read_to_string(path).unwrap();
    println!("读取:\n{}", s);

    fs::remove_file(path).unwrap();
}

fn demo_file_read_write() {
    println!("\n--- Part 1.2: File 细粒度读写 ---");

    let path = "intermediate_demo2.txt";

    // File::create: 创建(或截断)文件, 返回可写句柄
    let mut file = File::create(path).unwrap();
    // write_all: Write trait 方法, 写 &[u8]。as_bytes() 把 &str → &[u8]
    file.write_all("第一行\n".as_bytes()).unwrap();

    // OpenOptions: 控制打开模式。append(true) = 追加到末尾而非覆盖
    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    writeln!(file, "追加的行").unwrap(); // writeln! 自动加 \n
    // File 离开作用域时 Drop trait 自动关闭句柄

    // 读取: File::open 只读模式 → read_to_string
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).unwrap();
    println!("内容:\n{}", s);

    fs::remove_file(path).unwrap();
}

fn demo_buffered_io() {
    println!("\n--- Part 1.3: 缓冲 I/O ---");

    let path = "intermediate_buf.txt";

    // BufWriter: 写入先到 8KB 内存缓冲, drop 时批量 flush 到磁盘
    {
        let f = File::create(path).unwrap();
        let mut w = BufWriter::new(f);
        for i in 1..=3 {
            writeln!(w, "行 {}", i).unwrap();
        }
    } // writer drop → flush → 数据落盘

    // BufReader: 读时预取 8KB 到缓冲区, 后续 read 从内存取, 减少系统调用
    {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        // lines(): 迭代器, 每次 lazily 读一行 (去掉 \n)
        for (i, line) in reader.lines().enumerate() {
            println!("  行 {}: {}", i + 1, line.unwrap());
        }
    }

    fs::remove_file(path).unwrap();
}

fn demo_path() {
    println!("\n--- Part 1.4: Path / PathBuf ---");

    // PathBuf ≈ String (拥有所有权, 可变)
    let mut pb = PathBuf::from("/home/user");
    pb.push("docs"); // push 自动加路径分隔符
    println!("PathBuf: {}", pb.display());

    // Path ≈ &str (借用, 不可变), 从 PathBuf 解引用得到
    let p: &Path = &pb;
    println!("  文件名: {:?}", p.file_name());
    println!("  扩展名: {:?}", p.extension());
    println!("  父目录: {:?}", p.parent());
}

// ==================== Part 2: JSON 读写 (serde + serde_json) ====================

/// 定义一个同时支持 序列化(→JSON) 和 反序列化(←JSON) 的结构体。
/// Serialize:   struct → JSON 字符串
/// Deserialize: JSON 字符串 → struct
/// Debug:       方便 println!("{:?}") 打印
/// PartialEq:   方便 assert_eq! 断言
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

fn demo_json_serialize() {
    println!("\n--- Part 2.1: JSON 序列化 (Rust → JSON) ---");

    let book = Book {
        title: "Rust 程序设计".to_string(),
        author: "Klabnik".to_string(),
        year: 2018,
    };

    // to_string: struct → JSON 字符串 (紧凑格式, 无缩进)
    // 原理: serde_json 遍历 struct 字段, 按字段名生成 JSON key-value
    let json = serde_json::to_string(&book).unwrap();
    println!("紧凑: {}", json);

    // to_string_pretty: 带缩进和换行, 人类可读
    let pretty = serde_json::to_string_pretty(&book).unwrap();
    println!("美化:\n{}", pretty);

    // to_writer: 直接写到文件 (不经过中间 String)
    // 原理: serde_json 内部调用 Write trait 逐字节写入
    let f = File::create("intermediate_book.json").unwrap();
    let mut w = BufWriter::new(f);
    serde_json::to_writer_pretty(&mut w, &book).unwrap();
    println!("已写入 intermediate_book.json");
}

fn demo_json_deserialize() {
    println!("\n--- Part 2.2: JSON 反序列化 (JSON → Rust) ---");

    let json_str = r#"{"title":"Rust 异步编程","author":"Klabnik","year":2023}"#;

    // from_str: JSON 字符串 → struct
    // 原理: serde_json 解析 JSON token 流, 按字段名匹配 struct 字段,
    //       字段类型由 struct 定义决定 (year 解析为 u32, title 解析为 String)
    let book: Book = serde_json::from_str(json_str).unwrap();
    println!("解析结果: {:?}", book);
    println!("  书名: {}", book.title);
    println!("  作者: {}", book.author);
    println!("  年份: {}", book.year);

    // from_reader: 从文件读取 JSON → struct (流式, 不先全读入 String)
    let f = File::open("intermediate_book.json").unwrap();
    let reader = BufReader::new(f);
    let book2: Book = serde_json::from_reader(reader).unwrap();
    println!("从文件读回: {:?}", book2);

    fs::remove_file("intermediate_book.json").unwrap();
}

fn demo_json_value() {
    println!("\n--- Part 2.3: JSON Value (不定义 struct 也能解析) ---");

    let json_str = r#"{"name":"张三","scores":[90,85,88]}"#;

    // serde_json::Value: 枚举, 可表示任意 JSON (Object/Array/String/Number/Bool/Null)
    // 不需要预先定义 struct, 适合动态 JSON 或只取部分字段的场景
    let v: serde_json::Value = serde_json::from_str(json_str).unwrap();

    // 按 key 访问, 返回 Option<&Value>
    println!("name: {}", v["name"]);
    println!("scores: {:?}", v["scores"]);
    // as_array(): 把 Value 转为 &[Value], 是 Option (不是数组就 None)
    println!("  第一科: {}", v["scores"].as_array().unwrap()[0]);
}

// ==================== Part 3: CSV 读写 (csv crate) ====================

// CSV (Comma-Separated Values): 逗号分隔的纯文本表格格式。
// 本质: 每行一条记录, 逗号分隔字段, 第一行通常为列名 (header)。
// 优势: 人类可读, Excel 可打开, 流式读写 (不占大内存)。

fn demo_csv_write() {
    println!("\n--- Part 3.1: CSV 写入 ---");

    let path = "intermediate_data.csv";

    // Writer::from_writer: 包装任意 Write 实现 (File / Vec<u8> / ...)
    let f = File::create(path).unwrap();
    let mut wtr = csv::Writer::from_writer(f);

    // 写 header: 列名行
    wtr.write_record(&["书名", "作者", "年份"]).unwrap();

    // 写数据行: 每行一个 &[&str]
    wtr.write_record(&["Rust 程序设计", "Klabnik", "2018"]).unwrap();
    wtr.write_record(&["Rust 异步编程", "Klabnik", "2023"]).unwrap();
    wtr.write_record(&["Rust 入门", "张三", "2022"]).unwrap();

    // flush: 确保缓冲数据写入磁盘
    wtr.flush().unwrap();
    println!("已写入 {}", path);

    // 看看文件内容
    println!("文件内容:");
    println!("{}", fs::read_to_string(path).unwrap());
}

fn demo_csv_read_manual() {
    println!("\n--- Part 3.2: CSV 读取 (逐行手动) ---");

    let path = "intermediate_data.csv";
    let f = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(f);

    // headers(): 读取第一行作为列名, 返回 &StringRecord
    // 原理: 第一行解析为 header map, 后续可以用列名索引
    println!("列名: {:?}", rdr.headers().unwrap());

    // records(): 迭代器, 每次返回 csv::StringResult<StringRecord>
    // 原理: 流式解析, 每次只读一行进内存, 不会一次性加载整个文件
    for result in rdr.records() {
        let record = result.unwrap(); // StringRecord: 一行数据
        // record.get(i): 按列索引取值, 返回 Option<&str>
        println!(
            "  '{}' — '{}' ({})",
            record.get(0).unwrap(),
            record.get(1).unwrap(),
            record.get(2).unwrap(),
        );
    }
}

/// 数据探查: 如何获取行数/列数, 关注所有权的流转。
fn demo_csv_inspect() {
    println!("\n--- Part 3.2b: 数据探查 (行数/列数/所有权) ---");

    let path = "intermediate_data.csv";

    // ── 第一遍: 只看结构信息 (列数 + 行数) ──
    {
        let f = File::open(path).unwrap();
        let mut rdr = csv::Reader::from_reader(f);

        // headers(): 返回 &StringRecord — 借用 rdr 内部缓冲区
        // ⚠️ 生命周期: headers 借用的数据属于 rdr, rdr 必须先于 headers 存在
        //             headers 不能比 rdr 活得久 (编译器强制)
        let headers = rdr.headers().unwrap();
        println!("列数: {}  (通过 headers.len())", headers.len());
        println!("列名: {:?}", headers);

        // records().count(): 遍历所有行, 返回行数。
        // ⚠️ count() 消耗迭代器 — rdr 的字节被逐行 move 进 StringRecord, 读完即止
        //     此后 rdr 不能再读 (迭代器已耗尽)
        let row_count = rdr.records().count();
        println!("数据行数: {}  (通过 records().count())", row_count);
    } // rdr drop, f drop → 文件关闭

    // ── 第二遍: 逐行查看详情 ──
    // 必须重新打开文件 — 上一遍的 rdr 已消耗, File 已关闭
    {
        let f = File::open(path).unwrap();
        let mut rdr = csv::Reader::from_reader(f);

        for result in rdr.records() {
            let record = result.unwrap();
            // record: StringRecord (owned) — 拥有本行所有字段的字符串数据
            // record.get(i): 返回 Option<&str> — 借用 record 内的字节
            // ⚠️ &str 生命周期: 从 record 借来, 不能比 record 活得久
            println!(
                "  本行列数: {}, 字段: {:?}",
                record.len(),
                (0..record.len())
                    .map(|i| record.get(i).unwrap())
                    .collect::<Vec<_>>()
            );
        }
    }
}

/// 配合 serde, CSV 可以直接反序列化为 Vec<Struct>。
/// 原理: csv crate 内部调用 serde, 按 header 列名自动匹配 struct 字段。
#[derive(Debug, Deserialize, Serialize)]
struct CsvRecord {
    #[serde(rename = "书名")]
    title: String,
    #[serde(rename = "作者")]
    author: String,
    #[serde(rename = "年份")]
    year: u32,
}

fn demo_csv_read_serde() {
    println!("\n--- Part 3.3: CSV 反序列化 (serde 集成) ---");

    let path = "intermediate_data.csv";
    let f = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(f);

    // deserialize: 每行自动反序列化为 CsvRecord
    // header 列名通过 #[serde(rename)] 映射到 struct 字段名
    for result in rdr.deserialize() {
        let record: CsvRecord = result.unwrap();
        println!("  '{}' — '{}' ({})", record.title, record.author, record.year);
    }

    fs::remove_file(path).unwrap();
}

// ==================== 汇总 ====================

pub fn run() {
    println!("══════════ 文件 I/O 全面指南 ══════════\n");

    demo_raw_bytes();
    demo_simple_read_write();
    demo_file_read_write();
    demo_buffered_io();
    demo_path();

    println!("\n══════════ JSON 读写 ══════════");
    demo_json_serialize();
    demo_json_deserialize();
    demo_json_value();

    println!("\n══════════ CSV 读写 ══════════");
    demo_csv_write();
    demo_csv_inspect();
    demo_csv_read_manual();
    demo_csv_read_serde();

    println!("\n══════════ 总结 ══════════");
    println!("格式      写入               读取              行/列获取");
    println!("────────────────────────────────────────────────────");
    println!("纯文本    fs::write          fs::read_to_string  .lines().count() / .len()");
    println!("JSON      serde_json          serde_json         数组 .len() / 对象字段数");
    println!("CSV       csv::Writer         csv::Reader         records().count() / headers().len()");
    println!();
    println!("Excel/polars 数据处理 → 见 advanced/data_processing");
}
