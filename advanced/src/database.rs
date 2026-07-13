//! 数据持久化: 基于文件的简单 CRUD.

// 标准库不包含 JSON 解析或数据库驱动.
// 这里演示用文件 + 自定义文本格式做增删改查.
// 真实项目建议: serde + serde_json (序列化) 或 rusqlite (SQLite).

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

/// 数据库文件路径.
const DB_PATH: &str = "advanced_db.txt";

// ===== 1. 写入记录 =====

/// 追加一条记录到文件末尾.
fn insert(key: &str, value: &str) {
    // OpenOptions::append: 追加模式, 文件不存在则创建
    let mut file = OpenOptions::new()
        .create(true)    // 不存在则创建
        .append(true)    // 追加到末尾
        .open(DB_PATH)
        .unwrap();

    // 格式: key|value (用 | 分隔, 一行一条记录)
    writeln!(file, "{}|{}", key, value).unwrap();
    println!("  INSERT: {} → {}", key, value);
}

// ===== 2. 读取全部记录 =====

/// 从文件读取所有记录到 HashMap.
fn read_all() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // 文件不存在则返回空 map
    if let Ok(file) = File::open(DB_PATH) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some((k, v)) = line.split_once('|') {
                    map.insert(k.to_string(), v.to_string());
                }
            }
        }
    }

    map
}

/// 查询指定 key 的值 (Option 模式).
fn select(key: &str) -> Option<String> {
    let map = read_all();
    map.get(key).cloned()
}

// ===== 3. 更新记录 =====

/// 全量重写: 读入 → 修改 → 写回.
fn update(key: &str, new_value: &str) -> bool {
    let mut map = read_all();

    if map.contains_key(key) {
        map.insert(key.to_string(), new_value.to_string());
        write_all(&map);
        println!("  UPDATE: {} → {}", key, new_value);
        true
    } else {
        println!("  UPDATE 失败: key '{}' 不存在", key);
        false
    }
}

// ===== 4. 删除记录 =====

/// 读入 → 移除 → 写回.
fn delete(key: &str) -> bool {
    let mut map = read_all();

    if map.remove(key).is_some() {
        write_all(&map);
        println!("  DELETE: {} 已删除", key);
        true
    } else {
        println!("  DELETE 失败: key '{}' 不存在", key);
        false
    }
}

// ===== 5. 辅助: 全量写入 =====

/// 把 HashMap 写回文件 (覆盖).
fn write_all(map: &HashMap<String, String>) {
    let mut file = File::create(DB_PATH).unwrap();
    for (k, v) in map {
        writeln!(file, "{}|{}", k, v).unwrap();
    }
}

// ===== 6. 演示流程 =====

/// 清理测试数据.
fn cleanup() {
    let _ = fs::remove_file(DB_PATH);
}

/// 完整的 CRUD 流程演示.
fn demo_crud() {
    println!("--- 文件数据库 CRUD ---");
    println!("存储格式: key|value (每行一条)");

    // Create
    println!("\n[1. CREATE/INSERT]");
    insert("name", "张三");
    insert("age", "25");
    insert("city", "北京");

    // Read
    println!("\n[2. READ/SELECT]");
    match select("name") {
        Some(v) => println!("  SELECT name → {}", v),
        None => println!("  SELECT name → 不存在"),
    }
    match select("email") {
        Some(v) => println!("  SELECT email → {}", v),
        None => println!("  SELECT email → 不存在"),
    }

    // Update
    println!("\n[3. UPDATE]");
    update("age", "26");
    update("email", "不存在会失败"); // 不存在

    // Delete
    println!("\n[4. DELETE]");
    delete("city");
    delete("city"); // 重复删除

    // 列出所有记录
    println!("\n[5. 所有记录]");
    let all = read_all();
    for (k, v) in &all {
        println!("  {}: {}", k, v);
    }

    println!("\n说明:");
    println!("  这是最简单的文件存储方案, 适合:");
    println!("  - 配置文件 (.toml/.json/.yaml)");
    println!("  - 小型 Key-Value 存储");
    println!("  - 日志文件");
    println!("  真实项目推荐:");
    println!("  - serde + serde_json: JSON 序列化/反序列化");
    println!("  - rusqlite: 嵌入式 SQLite 数据库");
    println!("  - diesel / sqlx: ORM / 异步 SQL");
    println!("  - sled: 纯 Rust 嵌入式 KV 数据库");
}

pub fn run() {
    demo_crud();
    cleanup();
}
