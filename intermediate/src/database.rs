//! SQLite 数据库基础: 用 rusqlite 实现 CRUD。
//!
//! rusqlite 是 Rust 最流行的 SQLite 绑定, 纯 Rust 实现, 无需安装外部数据库。
//! 核心: Connection (连接) → execute (执行) → query_map (查询映射)。

use rusqlite::{params, Connection, Result as SqlResult};

// ===== 初始化: 内存数据库 =====

/// :memory: 是不落盘的内存数据库, 程序结束即销毁。
/// 换成文件路径 (如 "data.db") 即可持久化, API 完全一样。
fn init_db() -> SqlResult<Connection> {
    let conn = Connection::open_in_memory()?; // ? 传播 rusqlite::Error

    // execute: 执行不返回数据的 SQL (CREATE / INSERT / UPDATE / DELETE)
    // params![] 为空参数列表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id    INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL
        )",
        [], // 无参数
    )?;

    Ok(conn) // 所有权移交给调用方
}

// ===== 1. 插入 (INSERT) =====

/// execute + params! 绑定参数, 防止 SQL 注入。
fn insert_book(conn: &Connection, title: &str, author: &str) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO books (title, author) VALUES (?1, ?2)", // ?1, ?2 是位置参数
        params![title, author], // 按位置绑定, 自动转义
    )?;

    // last_insert_rowid(): 获取刚插入行的 rowid (自增键)
    Ok(conn.last_insert_rowid())
}

// ===== 2. 查询全部 (SELECT) =====

/// query_map: 对每行结果调用闭包, 映射为 Rust 类型。
fn query_all(conn: &Connection) -> SqlResult<Vec<(i64, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, title, author FROM books")?;

    // query_map: 迭代器, 每行 → 执行闭包 → 收集为 Vec
    let rows = stmt.query_map([], |row| {
        // row.get(n): 按列索引获取值, 泛型参数指定 Rust 类型
        Ok((
            row.get::<_, i64>(0)?,    // 第 0 列: id → i64
            row.get::<_, String>(1)?, // 第 1 列: title → String
            row.get::<_, String>(2)?, // 第 2 列: author → String
        ))
    })?;

    // collect → Result<Vec<T>, Error>, 用 ? 传播错误
    let books: SqlResult<Vec<_>> = rows.collect();
    books
}

// ===== 3. 按条件查询 (WHERE) =====

/// params 绑定查询条件。
fn query_by_author(conn: &Connection, author: &str) -> SqlResult<Vec<String>> {
    let mut stmt = conn.prepare("SELECT title FROM books WHERE author = ?1")?;

    let titles = stmt.query_map(params![author], |row| row.get::<_, String>(0))?;

    titles.collect()
}

// ===== 4. 更新 (UPDATE) =====

/// execute 返回受影响行数。
fn update_book(conn: &Connection, id: i64, new_title: &str) -> SqlResult<usize> {
    // execute 返回 usize = affected rows (受影响的行数)
    conn.execute(
        "UPDATE books SET title = ?1 WHERE id = ?2",
        params![new_title, id],
    )
}

// ===== 5. 删除 (DELETE) =====

fn delete_book(conn: &Connection, id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM books WHERE id = ?1", params![id])
}

// ===== 6. 事务 (Transaction) =====

/// 事务: 一组操作要么全部成功, 要么全部回滚。
/// conn.execute 默认每条 SQL 自动提交, 事务可以手动控制提交。
fn demo_transaction(conn: &mut Connection) -> SqlResult<()> {
    // 开始事务
    let tx = conn.transaction()?;

    tx.execute("INSERT INTO books (title, author) VALUES (?1, ?2)", params!["事务书A", "作者A"])?;
    tx.execute("INSERT INTO books (title, author) VALUES (?1, ?2)", params!["事务书B", "作者B"])?;

    // commit: 提交事务, 两条 INSERT 同时生效
    tx.commit()?;
    // 如果中间 ? 遇到错误, tx 离开作用域时自动 rollback

    Ok(())
}

// ===== 7. 演示流程 =====

pub fn run() {
    println!("--- SQLite 数据库 CRUD (rusqlite) ---\n");

    // 初始化内存数据库
    let conn = init_db().expect("初始化数据库失败");
    println!("数据库已初始化 (内存模式, 程序结束即销毁)\n");

    // [1] INSERT — 插入数据
    println!("[1. INSERT]");
    let id1 = insert_book(&conn, "Rust 程序设计", "Klabnik").unwrap();
    let id2 = insert_book(&conn, "Rust 入门秘籍", "张三").unwrap();
    let id3 = insert_book(&conn, "Rust 异步编程", "Klabnik").unwrap();
    println!("  插入了 3 本书, ID: {}, {}, {}\n", id1, id2, id3);

    // [2] SELECT — 查询全部
    println!("[2. SELECT 全部]");
    let all = query_all(&conn).unwrap();
    for (id, title, author) in &all {
        println!("  [{}] {} — {}", id, title, author);
    }
    println!();

    // [3] 按条件查询 — WHERE + 参数绑定
    println!("[3. SELECT WHERE author = 'Klabnik']");
    let titles = query_by_author(&conn, "Klabnik").unwrap();
    for t in &titles {
        println!("  {}", t);
    }
    println!();

    // [4] UPDATE — 更新
    println!("[4. UPDATE]");
    let affected = update_book(&conn, id2, "Rust 进阶秘籍").unwrap();
    println!("  更新了 {} 行", affected);
    let all = query_all(&conn).unwrap();
    for (id, title, author) in &all {
        println!("  [{}] {} — {}", id, title, author);
    }
    println!();

    // [5] DELETE — 删除
    println!("[5. DELETE]");
    let affected = delete_book(&conn, id3).unwrap();
    println!("  删除了 {} 行 (ID: {})", affected, id3);
    let all = query_all(&conn).unwrap();
    for (id, title, author) in &all {
        println!("  [{}] {} — {}", id, title, author);
    }
    println!();

    // [6] 事务 — 批量操作原子性
    println!("[6. 事务 (Transaction)]");
    let mut conn2 = Connection::open_in_memory().unwrap();
    conn2.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, author TEXT)", []).unwrap();
    demo_transaction(&mut conn2).unwrap();
    let count: i64 = conn2.query_row("SELECT COUNT(*) FROM books", [], |row| row.get(0)).unwrap();
    println!("  事务提交后, 表中共 {} 条记录\n", count);

    // 总结
    println!("══════════ 说明 ══════════");
    println!("• rusqlite: Rust 绑定 SQLite, 无需外部数据库服务");
    println!("• params![]: 参数化查询, 防 SQL 注入, 自动类型转换");
    println!("• query_map: 迭代映射, 每行执行闭包 → Rust 类型");
    println!("• transaction: 事务, commit 提交 / drop 自动回滚");
    println!("• 真实项目常用: sqlx (异步), diesel (ORM), sea-orm (async ORM)");
    println!();
    println!("对比之前 advanced 的 .txt 文本 CRUD:");
    println!("  .txt 版: 手动 split('|') 解析, 全量读写, 无类型安全");
    println!("  rusqlite: SQL 标准, 参数绑定, 增量查询, 事务支持");
}
