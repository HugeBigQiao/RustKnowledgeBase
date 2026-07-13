//! SQLite 数据库操作: 建表 / 插入 / 查询 / 表列表.

use crate::error::PipelineError;
use crate::models::record::{DataSet, Row};
use rusqlite::Connection;

/// 打开(或创建)数据库连接.
pub fn open_db(db_path: &str) -> Result<Connection, PipelineError> {
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

/// 列出数据库中的所有表名.
pub fn list_tables(conn: &Connection) -> Result<Vec<String>, PipelineError> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name",
    )?;

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tables)
}

/// 把 DataSet 写入数据库表.
/// - 如果表不存在, 自动创建(所有列 TEXT 类型).
/// - 插入所有数据, 用事务包裹.
pub fn insert_dataset(
    conn: &Connection,
    table_name: &str,
    data: &DataSet,
) -> Result<usize, PipelineError> {
    if data.is_empty() {
        return Ok(0);
    }

    // 创建表 (IF NOT EXISTS)
    let col_defs: Vec<String> = data
        .columns
        .iter()
        .map(|c| format!("\"{}\" TEXT", c))
        .collect();

    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
        table_name,
        col_defs.join(", ")
    );
    conn.execute(&create_sql, [])?;

    // 插入数据(事务)
    let tx = conn.unchecked_transaction()?;

    let placeholders: Vec<&str> = data.columns.iter().map(|_| "?").collect();
    let insert_sql = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table_name,
        data.columns.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", "),
        placeholders.join(", ")
    );

    let mut count = 0;
    for row in &data.rows {
        let values: Vec<String> = data
            .columns
            .iter()
            .map(|col| row.get(col).cloned().unwrap_or_default())
            .collect();

        let params: Vec<&dyn rusqlite::types::ToSql> = values
            .iter()
            .map(|v| v as &dyn rusqlite::types::ToSql)
            .collect();

        conn.execute(&insert_sql, params.as_slice())?;
        count += 1;
    }

    tx.commit()?;
    Ok(count)
}

/// 从数据库表读取全部数据.
pub fn read_table(conn: &Connection, table_name: &str) -> Result<DataSet, PipelineError> {
    // 获取列信息
    let columns = get_table_columns(conn, table_name)?;

    // 查询全部数据
    let col_list = columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!("SELECT {} FROM \"{}\"", col_list, table_name);
    let mut stmt = conn.prepare(&sql)?;

    let column_count = columns.len();
    let rows: Vec<Row> = stmt
        .query_map([], |row| {
            let mut map = Row::new();
            for i in 0..column_count {
                let col_name = columns.get(i).cloned().unwrap_or_default();
                let val: String = row.get::<_, String>(i).unwrap_or_default();
                map.insert(col_name, val);
            }
            Ok(map)
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(DataSet::from_parts(columns, rows))
}

/// 获取表的列名列表.
fn get_table_columns(conn: &Connection, table_name: &str) -> Result<Vec<String>, PipelineError> {
    let sql = format!("PRAGMA table_info(\"{}\")", table_name);
    let mut stmt = conn.prepare(&sql)?;

    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .collect();

    if columns.is_empty() {
        return Err(PipelineError::Other(format!(
            "表 '{}' 不存在或为空",
            table_name
        )));
    }

    Ok(columns)
}

/// 获取表的行数.
pub fn count_rows(conn: &Connection, table_name: &str) -> Result<usize, PipelineError> {
    let sql = format!("SELECT COUNT(*) FROM \"{}\"", table_name);
    let count: usize = conn.query_row(&sql, [], |row| row.get(0))?;
    Ok(count)
}
