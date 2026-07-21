//! 业务引擎层 — 通道消息类型 + 引擎子模块。
//!
//! GUI 层通过 mpsc 通道向引擎发送命令, 引擎在后台 tokio 线程中异步执行,
//! 结果通过事件通道回传 GUI。

pub mod export_engine;
pub mod import_engine;
pub mod source_manager;

// ======================================================================
//  通道消息: GUI → 后台引擎
// ======================================================================

/// GUI 发给后台引擎的命令
#[derive(Debug)]
pub enum EngineCmd {
    /// 连接数据库
    Connect(String),
    /// 断开连接
    Disconnect,
    /// 列出所有数据库
    ListDatabases,
    /// 列出指定数据库的表
    ListTables(String),
    /// 查询表的列信息
    FetchColumns {
        db_name: String,
        table_name: String,
    },
    /// 查询预览数据
    FetchPreview {
        db_name: String,
        table_name: String,
        columns: Vec<String>,
        limit: usize,
    },
    /// 执行导出
    Export(export_engine::ExportConfig),
    /// 加载数据源文件 (分析列和行数)
    LoadSource(String),
    /// 执行导入
    Import(import_engine::ImportConfig),
}

// ======================================================================
//  通道消息: 后台引擎 → GUI
// ======================================================================

/// 后台引擎发回 GUI 的事件
#[derive(Debug, Clone)]
pub enum EngineEvent {
    // ── 连接 ──
    Connected,
    ConnectionFailed(String),
    Disconnected,

    // ── 数据加载 ──
    DatabasesLoaded(Vec<String>),
    TablesLoaded(Vec<String>),
    ColumnsLoaded(Vec<String>),
    PreviewLoaded {
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    },

    // ── 导出 ──
    ExportProgress(f32),   // 0.0 ~ 1.0
    ExportComplete(String), // 输出路径

    // ── 导入 ──
    SourceLoaded {
        path: String,
        columns: Vec<String>,
        row_count: usize,
    },
    SourceLoadFailed {
        path: String,
        error: String,
    },
    ImportProgress(f32),
    ImportComplete(usize), // 导入行数

    // ── 通用 ──
    Status(String),
}
