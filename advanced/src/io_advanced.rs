//! 文件 I/O: Read/Write trait / BufReader/BufWriter / Path.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

// ===== 1. 文件基本读写 =====

/// 演示 fs::read_to_string / fs::write (一次性读取全部).
fn demo_simple_read_write() {
    println!("--- 文件基本读写 ---");

    let path = "advanced_demo.txt";

    // 写入
    let content = "Hello, Rust!\n第二行\n第三行\n";
    fs::write(path, content).unwrap();
    println!("已写入: {}", path);

    // 读取全部
    let read_back = fs::read_to_string(path).unwrap();
    println!("读取全部:\n{}", read_back);

    // 删除临时文件
    fs::remove_file(path).unwrap();
}

// ===== 2. File + Read/Write trait =====

/// 用 File 对象实现更细粒度的读写.
fn demo_file_read_write() {
    println!("\n--- File 细粒度读写 ---");

    let path = "advanced_demo2.txt";

    // 创建并写入
    let mut file = File::create(path).unwrap();
    file.write_all("逐行写入\n第二行\n".as_bytes()).unwrap();

    // 追加写入
    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    writeln!(file, "追加的行").unwrap();

    // 读取
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("文件内容:\n{}", contents);

    fs::remove_file(path).unwrap();
}

// ===== 3. BufReader / BufWriter =====

/// BufReader 逐行读取, BufWriter 缓冲写入.
fn demo_buffered_io() {
    println!("\n--- BufReader / BufWriter ---");

    let path = "advanced_buf.txt";

    // BufWriter: 写入缓冲到内存, 累积一定量后批量写入磁盘
    {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        for i in 1..=5 {
            writeln!(writer, "第 {} 行", i).unwrap();
        }
        // writer 离开作用域时自动 flush
    }

    // BufReader: 逐行读取, 减少系统调用
    {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            println!("  行 {}: {}", i + 1, line.unwrap());
        }
    }

    fs::remove_file(path).unwrap();
}

// ===== 4. Path / PathBuf =====

/// Path 是不可变路径引用, PathBuf 是可变的拥有路径.
fn demo_path() {
    println!("\n--- Path / PathBuf ---");

    // PathBuf ~ String (可变, 拥有所有权)
    let mut pb = PathBuf::from("/home/user");
    pb.push("documents");
    pb.push("rust_notes.md");
    println!("PathBuf: {}", pb.display());

    // Path ~ &str (不可变引用)
    let p: &Path = &pb;
    println!("  文件名: {:?}", p.file_name());
    println!("  扩展名: {:?}", p.extension());
    println!("  父目录: {:?}", p.parent());

    // 常用方法
    println!("  is_absolute: {}", p.is_absolute());
    println!("  exists:      {}", p.exists());

    // PathBuf / Path 的关系类似 String / &str
    // PathBuf 可以解引用为 Path: &PathBuf → &Path
}

// ===== 5. 标准输入输出 =====

/// stdin / stdout / stderr 的使用.
fn demo_stdio() {
    println!("\n--- 标准输入输出 ---");

    // stdout 通常用 println! 宏即可
    // 需要更精细控制时用 std::io::stdout()
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "  (通过 stdout().lock() 写入)").unwrap();

    // stderr
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "  (通过 stderr().lock() 写入)").unwrap();

    // stdin (不做交互, 只演示代码)
    println!("  读取 stdin 示例 (需要交互输入):");
    println!("  let stdin = io::stdin();");
    println!("  let mut line = String::new();");
    println!("  stdin.read_line(&mut line)?;");
}

pub fn run() {
    demo_simple_read_write();
    demo_file_read_write();
    demo_buffered_io();
    demo_path();
    demo_stdio();
}
