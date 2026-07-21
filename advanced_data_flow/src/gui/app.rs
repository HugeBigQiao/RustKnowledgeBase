//! 主 App — 标签页切换、全局状态管理、异步通道、字体加载。
//!
//! 架构: GUI 主线程 ← mpsc → 后台 tokio 线程
//!
//! 工科风格: 深灰背景, 蓝钢强调, 功能区清晰分组。

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use eframe::egui;
use eframe::egui::{Align, Color32, Layout, RichText};

use crate::engine::export_engine;
use crate::engine::import_engine;
use crate::engine::{EngineCmd, EngineEvent};
use crate::gui::components::{draw_connection_bar, ConnectionAction};
use crate::gui::export_page::{draw_export_page, ExportPageState};
use crate::gui::import_page::{draw_import_page, ImportPageState};

// ======================================================================
//  标签页
// ======================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Export,
    Import,
}

// ======================================================================
//  应用状态
// ======================================================================

pub struct DataFlowApp {
    // ── 标签页 ──
    active_tab: Tab,

    // ── 连接 (共享) ──
    db_url: String,
    connected: bool,
    status: String,

    // ── 通道 ──
    cmd_tx: Sender<EngineCmd>,
    event_rx: Receiver<EngineEvent>,

    // ── 页面状态 ──
    export: ExportPageState,
    import: ImportPageState,
}

impl DataFlowApp {
    pub fn new(db_url: &str) -> Self {
        // 创建双向 mpsc 通道
        let (cmd_tx, cmd_rx) = mpsc::channel::<EngineCmd>();
        let (event_tx, event_rx) = mpsc::channel::<EngineEvent>();

        let db_url_owned = db_url.to_string();

        // 启动后台 tokio 线程
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("创建 tokio runtime 失败");

            rt.block_on(db_worker(cmd_rx, event_tx));
        });

        DataFlowApp {
            active_tab: Tab::Export,
            db_url: db_url_owned,
            connected: false,
            status: "就绪 — 点击「连接」开始".to_string(),
            cmd_tx,
            event_rx,
            export: ExportPageState::default(),
            import: ImportPageState::default(),
        }
    }
}

// ======================================================================
//  eframe::App 实现
// ======================================================================

impl eframe::App for DataFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── 轮询后台事件 ──
        self.poll_events();

        // ── 顶部连接栏 ──
        egui::TopBottomPanel::top("connection_bar").show(ctx, |ui| {
            let action = draw_connection_bar(
                ui,
                &mut self.db_url,
                self.connected,
                false, // loading 由各页面自行管理
            );

            match action {
                Some(ConnectionAction::Connect) => {
                    let _ = self
                        .cmd_tx
                        .send(EngineCmd::Connect(self.db_url.clone()));
                    self.status = "正在连接...".to_string();
                }
                Some(ConnectionAction::Disconnect) => {
                    let _ = self.cmd_tx.send(EngineCmd::Disconnect);
                    self.connected = false;
                    self.status = "已断开".to_string();
                }
                None => {}
            }
        });

        // ── 标签页选择器 ──
        egui::TopBottomPanel::top("tab_bar")
            .min_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.active_tab,
                        Tab::Export,
                        RichText::new("📤 数据导出").size(13.0),
                    );
                    ui.selectable_value(
                        &mut self.active_tab,
                        Tab::Import,
                        RichText::new("📥 数据导入").size(13.0),
                    );
                });
                ui.separator();
            });

        // ── 中央: 当前页面内容 ──
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                Tab::Export => {
                    draw_export_page(ui, &mut self.export, self.connected, &self.cmd_tx);
                }
                Tab::Import => {
                    draw_import_page(ui, &mut self.import, self.connected, &self.cmd_tx);
                }
            }
        });

        // ── 底部状态栏 ──
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 连接状态灯
                if self.connected {
                    ui.colored_label(Color32::from_rgb(0, 170, 0), "●");
                } else {
                    ui.colored_label(Color32::from_rgb(210, 40, 40), "●");
                }

                // 状态文本
                ui.label(RichText::new(&self.status).size(12.0).monospace());

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new("DataFlow Platform v0.2")
                            .size(10.0)
                            .color(Color32::GRAY),
                    );
                });
            });
        });

        // 定期刷新 (处理后台事件)
        ctx.request_repaint_after(std::time::Duration::from_millis(150));
    }
}

// ======================================================================
//  事件处理
// ======================================================================

impl DataFlowApp {
    /// 从 event_rx 通道拉取所有待处理事件并更新状态。
    fn poll_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                // ── 连接 ──
                EngineEvent::Connected => {
                    self.connected = true;
                    self.status = "已连接 ✓".to_string();
                    // 连接成功后加载数据库列表
                    let _ = self.cmd_tx.send(EngineCmd::ListDatabases);
                }
                EngineEvent::ConnectionFailed(msg) => {
                    self.connected = false;
                    self.status = format!("连接失败: {}", msg);
                }
                EngineEvent::Disconnected => {
                    self.connected = false;
                    self.status = "已断开".to_string();
                }

                // ── 数据库列表 ──
                EngineEvent::DatabasesLoaded(dbs) => {
                    self.export.databases = dbs.clone();
                    self.import.databases = dbs;
                    self.status = "数据库列表已加载".to_string();
                }

                // ── 表列表 ──
                EngineEvent::TablesLoaded(tables) => {
                    // 根据当前选中数据库, 更新对应页面的表列表
                    if !self.export.selected_db.is_empty() {
                        self.export.tables = tables.clone();
                    }
                    if !self.import.selected_db.is_empty() {
                        self.import.tables = tables;
                    }
                    self.status = "表列表已加载".to_string();
                }

                // ── 列信息 ──
                EngineEvent::ColumnsLoaded(columns) => {
                    self.export.all_columns = columns;
                    self.export.column_selected = vec![true; self.export.all_columns.len()];
                    self.status = format!("已加载 {} 列", self.export.all_columns.len());
                }

                // ── 预览数据 ──
                EngineEvent::PreviewLoaded { columns, rows } => {
                    self.export.preview_columns = columns;
                    self.export.preview_rows = rows;
                    self.status = format!("预览已加载, {} 行", self.export.preview_rows.len());
                }

                // ── 导出 ──
                EngineEvent::ExportComplete(path) => {
                    self.export.exporting = false;
                    self.status = format!("导出完成: {}", path);
                }

                // ── 数据源加载 ──
                EngineEvent::SourceLoaded {
                    path,
                    columns,
                    row_count,
                } => {
                    // 更新对应数据源的信息
                    for src in &mut self.import.source_manager.sources {
                        if src.file_path == path {
                            src.columns = columns;
                            src.row_count = row_count;
                            src.loaded = true;
                            break;
                        }
                    }
                    self.import.recalc_merged_columns();
                    self.status = format!("数据源已加载: {} 行", row_count);
                }
                EngineEvent::SourceLoadFailed { path, error } => {
                    self.status = format!("数据源加载失败: {} — {}", path, error);
                    // 移除加载失败的数据源
                    let idx = self
                        .import
                        .source_manager
                        .sources
                        .iter()
                        .position(|s| s.file_path == path);
                    if let Some(i) = idx {
                        self.import.source_manager.remove_source(i);
                        self.import.recalc_merged_columns();
                    }
                }

                // ── 导入 ──
                EngineEvent::ImportComplete(count) => {
                    self.import.importing = false;
                    self.status = format!("导入完成: {} 个数据源", count);
                }

                // ── 进度 ──
                EngineEvent::ExportProgress(pct) => {
                    self.status = format!("导出中... {:.0}%", pct * 100.0);
                }
                EngineEvent::ImportProgress(pct) => {
                    self.status = format!("导入中... {:.0}%", pct * 100.0);
                }

                // ── 通用状态 ──
                EngineEvent::Status(msg) => {
                    self.status = msg;
                }
            }
        }
    }
}

// ======================================================================
//  后台 worker (在独立线程 + tokio runtime 中运行)
// ======================================================================

async fn db_worker(cmd_rx: Receiver<EngineCmd>, event_tx: Sender<EngineEvent>) {
    use sqlx::postgres::PgPoolOptions;

    let mut pool: Option<sqlx::PgPool> = None;

    for cmd in cmd_rx {
        match cmd {
            // ── 连接 ──
            EngineCmd::Connect(url) => {
                let _ = event_tx.send(EngineEvent::Status(format!("正在连接 {} ...", url)));
                match PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                {
                    Ok(p) => {
                        pool = Some(p);
                        let _ = event_tx.send(EngineEvent::Connected);
                    }
                    Err(e) => {
                        let _ = event_tx.send(EngineEvent::ConnectionFailed(e.to_string()));
                    }
                }
            }

            EngineCmd::Disconnect => {
                pool = None;
                let _ = event_tx.send(EngineEvent::Disconnected);
            }

            // ── 列出数据库 ──
            EngineCmd::ListDatabases => {
                if let Some(ref p) = pool {
                    match sqlx::query_as::<_, (String,)>(
                        "SELECT datname FROM pg_database
                         WHERE datistemplate = false
                         ORDER BY datname",
                    )
                    .fetch_all(p)
                    .await
                    {
                        Ok(rows) => {
                            let dbs: Vec<String> = rows.into_iter().map(|(n,)| n).collect();
                            let _ = event_tx.send(EngineEvent::DatabasesLoaded(dbs));
                        }
                        Err(e) => {
                            let _ = event_tx
                                .send(EngineEvent::Status(format!("查询数据库列表失败: {}", e)));
                        }
                    }
                }
            }

            // ── 列出表 ──
            EngineCmd::ListTables(_db_name) => {
                if let Some(ref p) = pool {
                    match sqlx::query_as::<_, (String,)>(
                        "SELECT tablename FROM pg_tables
                         WHERE schemaname = 'public'
                         ORDER BY tablename",
                    )
                    .fetch_all(p)
                    .await
                    {
                        Ok(rows) => {
                            let tables: Vec<String> = rows.into_iter().map(|(t,)| t).collect();
                            let _ = event_tx.send(EngineEvent::TablesLoaded(tables));
                        }
                        Err(e) => {
                            let _ = event_tx
                                .send(EngineEvent::Status(format!("查询表列表失败: {}", e)));
                        }
                    }
                }
            }

            // ── 查询列信息 ──
            EngineCmd::FetchColumns {
                db_name: _,
                table_name,
            } => {
                if let Some(ref p) = pool {
                    match export_engine::fetch_columns(p, &table_name).await {
                        Ok(cols) => {
                            let _ = event_tx.send(EngineEvent::ColumnsLoaded(cols));
                        }
                        Err(e) => {
                            let _ = event_tx
                                .send(EngineEvent::Status(format!("查询列信息失败: {}", e)));
                        }
                    }
                }
            }

            // ── 查询预览 ──
            EngineCmd::FetchPreview {
                db_name: _,
                table_name,
                columns,
                limit,
            } => {
                if let Some(ref p) = pool {
                    match export_engine::fetch_preview(p, &table_name, &columns, limit).await {
                        Ok((cols, rows)) => {
                            let _ =
                                event_tx.send(EngineEvent::PreviewLoaded { columns: cols, rows });
                        }
                        Err(e) => {
                            let _ = event_tx
                                .send(EngineEvent::Status(format!("查询预览失败: {}", e)));
                        }
                    }
                }
            }

            // ── 执行导出 ──
            EngineCmd::Export(config) => {
                if let Some(ref p) = pool {
                    let _ = export_engine::run_export(p, config, &event_tx).await;
                }
            }

            // ── 加载数据源 ──
            EngineCmd::LoadSource(path) => {
                // 尝试读取文件并分析列和行数
                match load_source_info(&path).await {
                    Ok((columns, row_count)) => {
                        let _ = event_tx.send(EngineEvent::SourceLoaded {
                            path,
                            columns,
                            row_count,
                        });
                    }
                    Err(e) => {
                        let _ = event_tx.send(EngineEvent::SourceLoadFailed {
                            path,
                            error: e.to_string(),
                        });
                    }
                }
            }

            // ── 执行导入 ──
            EngineCmd::Import(config) => {
                if let Some(ref p) = pool {
                    let _ = import_engine::run_import(p, config, &event_tx).await;
                }
            }
        }
    }
}

// ======================================================================
//  数据源加载辅助
// ======================================================================

/// 分析数据源文件: 返回 (列名列表, 行数)。
async fn load_source_info(path: &str) -> Result<(Vec<String>, usize), String> {
    let lower = path.to_lowercase();

    if lower.ends_with(".csv") {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)
            .map_err(|e| format!("CSV 读取失败: {}", e))?;

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| format!("CSV 表头读取失败: {}", e))?
            .iter()
            .map(|s| s.to_string())
            .collect();

        let count = reader.records().count();
        Ok((headers, count))
    } else if lower.ends_with(".json") {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("JSON 读取失败: {}", e))?;

        let value: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("JSON 解析失败: {}", e))?;

        // 期望 JSON 数组
        if let Some(arr) = value.as_array() {
            let mut cols: Vec<String> = Vec::new();
            if let Some(first) = arr.first() {
                if let Some(obj) = first.as_object() {
                    cols = obj.keys().cloned().collect();
                }
            }
            Ok((cols, arr.len()))
        } else {
            Ok((Vec::new(), 1))
        }
    } else if lower.ends_with(".xlsx") || lower.ends_with(".xls") {
        // calamine 读取 Excel
        use calamine::{open_workbook, Reader, Xlsx};
        let mut workbook: Xlsx<_> =
            open_workbook(path).map_err(|e| format!("Excel 读取失败: {}", e))?;

        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            let mut rows_iter = range.rows();
            // 第一行作为表头
            let headers: Vec<String> = rows_iter
                .next()
                .map(|row| {
                    row.iter()
                        .map(|cell| cell.to_string())
                        .collect()
                })
                .unwrap_or_default();

            let count = rows_iter.count();
            Ok((headers, count))
        } else {
            Ok((Vec::new(), 0))
        }
    } else {
        // TXT: 按行读取
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("TXT 读取失败: {}", e))?;
        let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
        Ok((vec!["content".to_string()], lines.len()))
    }
}

// ======================================================================
//  字体加载 — 解决中文方块问题
// ======================================================================

/// 尝试加载系统中文字体。
fn setup_chinese_fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    // Windows / Linux / macOS 中文字体路径
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/System/Library/Fonts/PingFang.ttc",
    ];

    for path in &font_paths {
        if let Ok(data) = std::fs::read(path) {
            // egui 0.31: FontData::from_owned 返回 FontData
            // font_data 的类型可能是 BTreeMap<String, FontData>
            fonts
                .font_data
                .insert("chinese".to_string(), egui::FontData::from_owned(data).into());

            // 把中文字体放在字体列表最前面
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                family.insert(0, "chinese".to_string());
            }
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                family.insert(0, "chinese".to_string());
            }

            return fonts;
        }
    }

    eprintln!("[Font] 未找到中文字体文件, 中文可能显示为方块");
    fonts
}

// ======================================================================
//  启动入口
// ======================================================================

/// 启动 egui 窗口。
pub fn run(db_url: &str) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    let db_url = db_url.to_string();

    eframe::run_native(
        "DataFlow Platform — 数据流平台",
        options,
        Box::new(move |cc| {
            // 安装中文字体 (在首次渲染前)
            let font_defs = setup_chinese_fonts();
            cc.egui_ctx.set_fonts(font_defs);

            Ok(Box::new(DataFlowApp::new(&db_url)))
        }),
    )
}
