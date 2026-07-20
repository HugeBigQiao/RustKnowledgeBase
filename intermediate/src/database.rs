//! SQLite 数据库基础: 用结构体 + 枚举封装 rusqlite 的 CRUD 操作。
//!
//! 本模块演示:
//! - 用 struct (Book, Library) 和 enum (BookStatus) 建模数据层
//! - 在 struct 上实现方法 (impl Library), 封装数据库操作
//! - 标注所有权转移、借用 (引用) 和生命周期关键点
//!
//! rusqlite 0.31 核心 API:
//! - Connection: 数据库连接 (拥有底层 sqlite3* 句柄)
//! - prepare → Statement: 预编译 SQL, 生命周期绑定于 Connection
//! - query_map → Rows: 惰性迭代器, 生命周期绑定于 Statement
//!   因此必须 collect 为 Vec<Book> 后才能释放 Statement

use rusqlite::{params, Connection, Result as SqlResult};

// ======================================================================
//  数据模型: enum + struct
// ======================================================================

/// 图书状态 (枚举 — 有限种可能值的类型安全表示)
#[derive(Debug, PartialEq)]
enum BookStatus {
    Available, // 可借
    Lended,    // 已借出
    Reserved,  // 已预约
}

impl BookStatus {
    /// &self: 不可变借用 — 只读取枚举变体, 不修改
    fn as_str(&self) -> &str {
        match self {
            BookStatus::Available => "可借",
            BookStatus::Lended => "已借出",
            BookStatus::Reserved => "已预约",
        }
    }

    /// from_str 接收 &str (借用), 返回新建的 BookStatus (所有权)
    fn from_str(s: &str) -> BookStatus {
        match s {
            "已借出" => BookStatus::Lended,
            "已预约" => BookStatus::Reserved,
            _ => BookStatus::Available,
        }
    }
}

/// 图书 (结构体 — 相关字段的组合)
///
/// 字段类型选择:
/// - id: i64 — 简单整数, Copy 类型 (按位复制, 无所有权转移)
/// - title/author: String — 堆分配字符串, 拥有数据所有权
/// - status: BookStatus — 枚举, 栈上存储, Copy 可自动实现
#[derive(Debug)]
struct Book {
    id: i64,
    title: String,   // String 拥有堆上数据的所有权
    author: String,
    status: BookStatus,
}

impl Book {
    /// title/author 参数是 String (不是 &str):
    /// 调用方把 String 所有权移入本函数 → 再移入 Book 字段
    /// 此后调用方不能再使用原来的 title/author
    fn new(id: i64, title: String, author: String) -> Book {
        Book {
            id,
            title,    // String 所有权: 参数 → Book.title
            author,   // String 所有权: 参数 → Book.author
            status: BookStatus::Available,
        }
    }

    /// 借用版本: &str 参数不获取所有权, 内部 clone 一份
    fn from_refs(id: i64, title: &str, author: &str) -> Book {
        Book {
            id,
            title: title.to_string(),   // to_string(): 从 &str 克隆 → 新 String
            author: author.to_string(),
            status: BookStatus::Available,
        }
    }
}

// ======================================================================
//  数据库层: Library 封装 Connection + CRUD 方法
// ======================================================================

/// Library: 拥有 Connection 的所有权。
///
/// conn 字段私有 — 外部只能通过 Library 的方法访问数据库,
/// 保证所有 SQL 都经过预编译 + 参数绑定 (防止注入)。
struct Library {
    conn: Connection, // Library 拥有 Connection 的所有权
}

impl Library {
    // ----- 构造: 打开/创建数据库 -----

    /// 创建内存数据库 (数据不落盘, 进程结束即销毁)。
    ///
    /// 所有权流转:
    ///   Connection::open_in_memory() → 新建 Connection
    ///   Library { conn } → Connection 所有权移入 Library
    ///   Ok(lib) → Library 所有权移交给调用方
    fn new_in_memory() -> SqlResult<Library> {
        let conn = Connection::open_in_memory()?; // conn: Connection
        let mut lib = Library { conn }; // conn 所有权: 临时变量 → Library.conn
        lib.create_table()?;            // &mut self: 可变借用
        Ok(lib)                         // lib 所有权移交给调用方
    }

    /// 打开(或创建)文件数据库 — 数据持久化到磁盘。
    fn open(path: &str) -> SqlResult<Library> {
        let conn = Connection::open(path)?;
        let mut lib = Library { conn };
        lib.create_table()?;
        Ok(lib)
    }

    /// &mut self: 需要可变借用 —
    ///   因为 execute 会修改底层的 SQLite 连接状态
    fn create_table(&mut self) -> SqlResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id     INTEGER PRIMARY KEY AUTOINCREMENT,
                title  TEXT NOT NULL,
                author TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT '可借'
            )",
            [], // params![] 的简写: 无绑定参数
        )?;
        Ok(())
    }

    // ----- 1. 新增 (INSERT) — &mut self -----

    /// 添加图书 — 参数用 &str (借用, 不获取所有权)。
    ///
    /// 所有权分析:
    ///   &mut self — 可变借用, 因为要修改数据库状态
    ///   title: &str, author: &str — 不可变借用, 调用方仍可继续使用
    ///   返回 i64 — 新记录的 ID, 所有权交给调用方
    fn add_book(&mut self, title: &str, author: &str) -> SqlResult<i64> {
        self.conn.execute(
            "INSERT INTO books (title, author, status) VALUES (?1, ?2, '可借')",
            params![title, author], // params! 宏: 引用传入, 不消耗 title/author
        )?;
        Ok(self.conn.last_insert_rowid()) // last_insert_rowid() 是 Copy (i64)
    }

    // ----- 2. 查询 (SELECT) — &self + 生命周期分析 -----

    /// 按 ID 查询单本书。
    ///
    /// query_row: 查询零或一行, 0 行返回 Error::QueryReturnedNoRows
    fn get_book(&self, id: i64) -> SqlResult<Option<Book>> {
        // prepare: stmt 的生命周期绑定于 &self.conn (不可变借用)
        // ⚠️ 生命周期: Statement<'conn> → 引用 self.conn, 不能比 conn 活得久
        let mut stmt = self.conn
            .prepare("SELECT id, title, author, status FROM books WHERE id = ?1")?;

        // query_row: 消费 stmt 的借用, 返回 Result<T, Error>
        // 闭包 |row| 捕获 row, row 引用了 stmt 内部缓冲区
        // 但闭包内立即 collect 为 Book, 不再持有引用 → 安全
        let result = stmt.query_row(params![id], |row| {
            Ok(Book {
                id: row.get::<_, i64>(0)?,       // row.get: 从行缓冲区读取, Copy 类型
                title: row.get::<_, String>(1)?, // String: 从 SQLite 复制 → 新 String (所有权)
                author: row.get::<_, String>(2)?,
                status: BookStatus::from_str(&row.get::<_, String>(3)?),
            })
        });

        match result {
            Ok(book) => Ok(Some(book)), // Some(book): book 所有权移入 Some → 返回给调用方
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// 查询全部图书。
    ///
    /// ⚠️ 生命周期关键点:
    ///   conn.prepare() → Statement 借用 conn
    ///   stmt.query_map() → Rows 借用 stmt
    ///   Rows 是惰性迭代器: 遍历时才真正读数据
    ///   必须在 stmt 被释放前 collect 为 Vec<Book>
    ///   否则会报 "statement 已被 drop 但 rows 还在引用它"
    fn list_books(&self) -> SqlResult<Vec<Book>> {
        let mut stmt = self.conn
            .prepare("SELECT id, title, author, status FROM books ORDER BY id")?;

        // query_map: 返回 Rows<'stmt>, 生命期绑定于 stmt
        let rows = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                status: BookStatus::from_str(&row.get::<_, String>(3)?),
            })
        })?; // ? 传播 Error, Rows 继续存活

        // collect():
        //   遍历 rows 迭代器, 每行执行闭包 → Book
        //   收集到 Vec<Book> (全部所有权独立)
        //   collect 完成后, rows 可以安全释放 → stmt 可以安全释放
        let books: SqlResult<Vec<Book>> = rows.collect();

        // fn 结束时 stmt 被 drop
        // 但 Vec<Book> 已拥有所有数据, 不再依赖 stmt → 生命周期安全 ✅
        books // books 所有权移交给调用方
    }

    /// 按作者搜索 — 演示 &str 引用在查询中的使用。
    ///
    /// author: &str — 借用, 函数结束后 author 仍然可用
    fn search_by_author(&self, author: &str) -> SqlResult<Vec<Book>> {
        let mut stmt = self.conn
            .prepare("SELECT id, title, author, status FROM books WHERE author = ?1")?;

        let rows = stmt.query_map(params![author], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                status: BookStatus::from_str(&row.get::<_, String>(3)?),
            })
        })?;

        rows.collect()
    }

    // ----- 3. 更新 (UPDATE) — &mut self -----

    /// 更新图书信息。
    ///
    /// 返回 bool: true = 确实更新了某行 / false = 没找到该 ID
    fn update_book(
        &mut self,
        id: i64,
        new_title: &str,   // &str: 借用, 不获取所有权
        new_author: &str,
    ) -> SqlResult<bool> {
        let affected = self.conn.execute(
            "UPDATE books SET title = ?1, author = ?2 WHERE id = ?3",
            params![new_title, new_author, id],
        )?; // execute 返回 usize (受影响行数)
        Ok(affected > 0)
    }

    // ----- 4. 删除 (DELETE) — &mut self -----

    fn delete_book(&mut self, id: i64) -> SqlResult<bool> {
        let affected = self.conn
            .execute("DELETE FROM books WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    // ----- 5. 统计 -----

    fn count_books(&self) -> SqlResult<i64> {
        // query_row: 恰好一行一列, 无需 Statement 中间变量
        self.conn.query_row(
            "SELECT COUNT(*) FROM books",
            [],
            |row| row.get(0), // 类型推断: 从返回类型推导为 i64
        )
    }

    // ----- 6. 事务: 批量操作的原子性 -----

    /// 事务演示 — 一组写入要么全成功, 要么全回滚。
    ///
    /// &mut self: 事务需要可变借用 (会修改数据库)
    fn seed_sample_data(&mut self) -> SqlResult<()> {
        // Transaction::new: 从 &mut Connection 创建事务
        // tx 拥有 &mut conn 的独占借用
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO books (title, author, status) VALUES (?1, ?2, ?3)",
            params!["Rust 程序设计", "Klabnik", "可借"],
        )?;
        tx.execute(
            "INSERT INTO books (title, author, status) VALUES (?1, ?2, ?3)",
            params!["Rust 入门秘籍", "张三", "已借出"],
        )?;
        tx.execute(
            "INSERT INTO books (title, author, status) VALUES (?1, ?2, ?3)",
            params!["Rust 异步编程", "Klabnik", "已预约"],
        )?;

        // commit: 提交事务, 三条 INSERT 原子生效
        tx.commit()?;
        // 若之前任何一步 ? 返回错误, tx 在此处被 drop → 自动 rollback
        Ok(())
    }
}

// ======================================================================
//  演示入口
// ======================================================================

pub fn run() {
    println!("--- SQLite 数据库: 结构体 + 枚举 CRUD ---\n");

    // [0] 构造 Library
    let mut lib = Library::new_in_memory().expect("创建数据库失败");
    // lib: Library, 包含 Connection 的所有权
    println!("[0] 内存数据库已创建 (进程结束即销毁)\n");

    // [1] 批量插入 (事务)
    println!("[1] 事务批量插入");
    lib.seed_sample_data().unwrap();
    println!("  已插入 3 本书\n");

    // [2] SELECT 全部
    println!("[2] SELECT 全部 (list_books)");
    {
        // all_books: Vec<Book>, Book.title/author 是 String (拥有所有权)
        let all_books = lib.list_books().unwrap();
        // -- 所有权说明 --
        // list_books 内部: rows collect 出的每个 Book 都是全新 String
        //               original 的 &str 引用不再存在, Vec<Book> 完全自拥
        for book in &all_books {
            // &book: 不可变借用, 不消耗 all_books
            // book.title: &String → 自动解引用为 &str 参与打印
            println!("  [{:?}] {} — {}  [{}]",
                book.id, book.title, book.author, book.status.as_str());
        }
        // for 循环结束, &book 借用释放, all_books 仍然可用
    } // all_books 离开作用域, Vec<Book> 被 drop, 释放内存
    println!();

    // [3] 按 ID 查单本 — 演示 Option<Book> 所有权
    println!("[3] 按 ID 查询单本 (get_book)");
    {
        let book_opt = lib.get_book(1).unwrap();
        // book_opt: Option<Book>, 所有权已交给此作用域
        match &book_opt {
            // &book_opt: 借用匹配, 不消耗 book_opt
            Some(book) => {
                // book: &Book — 不可变借用
                println!("  找到: {} ({}), 状态: {}",
                    book.title, book.author, book.status.as_str());
            }
            None => println!("  未找到"),
        }

        // 移动所有权示例:
        if let Some(book) = book_opt {
            // book: Book — book_opt 被解构, 所有权移入 book
            let title = book.title; // String 所有权: book.title → title
            // book.title 已失效, 但 book.author 仍可用
            println!("  解构: title = \"{}\", author = \"{}\"", title, book.author);
        }
    }
    println!();

    // [4] UPDATE
    println!("[4] UPDATE (update_book)");
    {
        let ok = lib.update_book(2, "Rust 进阶之路", "李四").unwrap();
        println!("  更新 ID=2: {} (true=成功, false=未找到)", ok);

        // 验证: 再次查询 ID=2
        if let Some(book) = lib.get_book(2).unwrap() {
            println!("  验证: [2] {} — {}", book.title, book.author);
        }
    }
    println!();

    // [5] DELETE
    println!("[5] DELETE (delete_book)");
    {
        let ok = lib.delete_book(3).unwrap();
        println!("  删除 ID=3: {} (true=成功, false=未找到)", ok);
        println!("  当前总数: {}", lib.count_books().unwrap());
    }
    println!();

    // [6] 按作者搜索 (search_by_author)
    println!("[6] 按作者搜索 (search_by_author)");
    {
        let author = String::from("Klabnik"); // author: String, 堆分配
        let books = lib.search_by_author(&author).unwrap();
        // &author: 不可变借用 → 传给 search_by_author
        // author 仍然可用, 所有权未被转移
        println!("  作者 \"{}\" 的书:", author);
        for b in &books {
            println!("    [{}] {}", b.id, b.title);
        }
    }
    println!();

    // [7] 所有权对比: &str 借用 vs String 转移
    println!("[7] 所有权对比: &str vs String");
    {
        // --- 方式 A: &str 借用 ---
        let t = String::from("Rust 模式匹配");
        let a = String::from("王五");
        lib.add_book(&t, &a).unwrap();
        // &t, &a: 借用, 不转移所有权
        println!("  A (&str 借用): 插入后 t='{}', a='{}' 仍可用", t, a);
        // t, a 仍在此处可用 ✅

        // --- 方式 B: String 转移 ---
        let t2 = String::from("Rust 宏编程");
        let a2 = String::from("赵六");
        let book = Book::new(0, t2, a2);
        // t2, a2 的所有权已移入 Book → 不能再使用
        println!("  B (String 转移): Book {{ title: '{}', author: '{}' }}",
            book.title, book.author);
        // println!("{}", t2); ← 编译错误: t2 已被移动
    }
    println!();

    // ===== 总结 =====
    println!("══════════ 所有权 & 生命周期要点 ══════════");
    println!();
    println!("  1. Library 拥有 Connection 的所有权 (字段 conn: Connection)");
    println!("  2. &self 方法 = 不可变借用, 可并发读 (SELECT)");
    println!("  3. &mut self 方法 = 可变借用, 独占访问 (INSERT/UPDATE/DELETE)");
    println!("  4. &str 参数 = 借用, 调用方保留所有权; String 参数 = 转移");
    println!("  5. Statement 生命周期绑定于 Connection: 必须在此之前 collect");
    println!("  6. Rows collect → Vec<Book>: Book 拥有独立数据, 不再依赖 Statement");
    println!("  7. Option<Book>: match 解构时 Book 所有权可逐字段移出");
    println!("  8. 事务 Transaction: commit 提交, drop 自动 rollback");
    println!();
    println!("  rusqlite: 参数绑定 (params!), 防注入, 可编译为嵌入数据库");
    println!("  生产环境推荐: sqlx (异步) / diesel (ORM) / sea-orm (async ORM)");
}
