//! 页面2: 数据导入 (多数据源 → Database)
//!
//! 功能:
//!   - 数据源面板: 添加/管理多个文件 (CSV/JSON/Excel/TXT)
//!   - 目标数据库 + 表选择
//!   - 列映射 (源列 → 目标列, 自动匹配同名列)
//!   - 批量导入
//!
//! 工科风格: 上数据源, 下目标配置, 分区清晰。

use std::sync::mpsc::Sender;

use eframe::egui;
use eframe::egui::{Color32, RichText, Ui};

use crate::engine::source_manager::SourceManager;
use crate::section_header;
use crate::engine::EngineCmd;
use crate::gui::components::{
    draw_format_badge, ERROR_RED, STEEL_BLUE, SUCCESS_GREEN, WARN_AMBER,
};

// ======================================================================
//  页面状态
// ======================================================================

pub struct ImportPageState {
    // ── 数据源管理器 ──
    pub source_manager: SourceManager,

    // ── 目标数据库 ──
    pub databases: Vec<String>,
    pub selected_db: String,
    pub tables: Vec<String>,
    pub selected_table: String,
    /// 新建表名 (如果不选已有表)
    pub new_table_name: String,

    // ── 列映射 ──
    /// 所有数据源的合并列
    pub merged_columns: Vec<String>,
    /// 目标列映射 (源列 → 目标列名)
    pub column_mappings: Vec<(String, String)>,

    // ── 导入 ──
    pub importing: bool,

    // ── 加载状态 ──
    pub loading_target_dbs: bool,
    pub loading_target_tables: bool,
}

impl Default for ImportPageState {
    fn default() -> Self {
        ImportPageState {
            source_manager: SourceManager::new(),
            databases: Vec::new(),
            selected_db: String::new(),
            tables: Vec::new(),
            selected_table: String::new(),
            new_table_name: String::new(),
            merged_columns: Vec::new(),
            column_mappings: Vec::new(),
            importing: false,
            loading_target_dbs: false,
            loading_target_tables: false,
        }
    }
}

impl ImportPageState {
    /// 重新计算合并列 (所有数据源列的并集)
    pub fn recalc_merged_columns(&mut self) {
        let mut all_cols: Vec<String> = Vec::new();
        for src in &self.source_manager.sources {
            for col in &src.columns {
                if !all_cols.contains(col) {
                    all_cols.push(col.clone());
                }
            }
        }
        self.merged_columns = all_cols.clone();

        // 自动填充列映射 (同名匹配)
        self.column_mappings = all_cols
            .iter()
            .map(|c| (c.clone(), c.clone()))
            .collect();
    }
}

// ======================================================================
//  页面绘制
// ======================================================================

/// 绘制「数据导入」页面全部 UI。
pub fn draw_import_page(
    ui: &mut Ui,
    state: &mut ImportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    // ── 上半部: 数据源管理 ──
    egui::TopBottomPanel::top("import_sources_panel")
        .min_height(180.0)
        .resizable(true)
        .show_inside(ui, |ui| {
            draw_source_panel(ui, state, cmd_tx);
        });

    // ── 下半部: 目标配置 + 导入 ──
    egui::CentralPanel::default().show_inside(ui, |ui| {
        draw_target_panel(ui, state, connected, cmd_tx);
    });
}

// ======================================================================
//  数据源面板
// ======================================================================

fn draw_source_panel(ui: &mut Ui, state: &mut ImportPageState, cmd_tx: &Sender<EngineCmd>) {
    section_header!(ui, "数据源管理");

    // ── 添加数据源按钮 ──
    ui.horizontal(|ui| {
        if ui
            .button(RichText::new("+ 添加数据源").color(STEEL_BLUE))
            .clicked()
        {
            if let Some(paths) = rfd::FileDialog::new()
                .add_filter("所有支持格式", &["csv", "json", "xlsx", "xls", "txt"])
                .add_filter("CSV", &["csv"])
                .add_filter("JSON", &["json"])
                .add_filter("Excel", &["xlsx", "xls"])
                .add_filter("Text", &["txt"])
                .pick_files()
            {
                for path in paths {
                    let path_str = path.to_string_lossy().to_string();
                    state.source_manager.add_source(&path_str);
                    // 发送后台加载命令 (clone 避免借用冲突)
                    let _ = cmd_tx.send(EngineCmd::LoadSource(path_str.clone()));
                }
            }
            ui.close_menu();
        }

        ui.label(format!(
            "已添加 {} 个数据源",
            state.source_manager.sources.len()
        ));
    });

    ui.separator();

    // ── 数据源列表 ──
    if state.source_manager.sources.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(RichText::new("暂无数据源").color(Color32::GRAY));
            ui.label("点击「添加数据源」选择文件 (支持 CSV/JSON/Excel/TXT)");
        });
        return;
    }

    egui::ScrollArea::vertical()
        .max_height(120.0)
        .show(ui, |ui| {
            let mut remove_idx: Option<usize> = None;

            for (i, src) in state.source_manager.sources.iter().enumerate() {
                ui.horizontal(|ui| {
                    // 格式标签
                    draw_format_badge(ui, src.format.label());

                    // 文件名
                    ui.label(&src.file_name);

                    // 加载状态
                    if src.loaded {
                        ui.colored_label(SUCCESS_GREEN, "✓");
                        ui.label(format!("{} 列, {} 行", src.columns.len(), src.row_count));
                    } else {
                        ui.add(egui::Spinner::new().size(12.0));
                        ui.label("加载中...");
                    }

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if ui
                                .small_button(RichText::new("移除").color(ERROR_RED))
                                .clicked()
                            {
                                remove_idx = Some(i);
                            }
                        },
                    );
                });
            }

            if let Some(i) = remove_idx {
                state.source_manager.remove_source(i);
                state.recalc_merged_columns();
            }
        });
}

// ======================================================================
//  目标配置面板
// ======================================================================

fn draw_target_panel(
    ui: &mut Ui,
    state: &mut ImportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    section_header!(ui, "目标数据库");

    // ── 数据库选择 ──
    ui.horizontal(|ui| {
        ui.label("数据库:");
        egui::ComboBox::from_id_salt("import_target_db")
            .selected_text(if state.selected_db.is_empty() {
                "-- 选择 --"
            } else {
                &state.selected_db
            })
            .show_ui(ui, |ui| {
                for db in &state.databases {
                    if ui
                        .selectable_label(state.selected_db == *db, db.as_str())
                        .clicked()
                    {
                        state.selected_db = db.clone();
                        state.tables.clear();
                        state.selected_table.clear();
                        let _ = cmd_tx.send(EngineCmd::ListTables(db.clone()));
                    }
                }
            });

        if ui
            .add_enabled(connected, egui::Button::new("刷新"))
            .clicked()
        {
            let _ = cmd_tx.send(EngineCmd::ListDatabases);
        }
    });

    ui.add_space(4.0);

    // ── 表选择 ──
    ui.horizontal(|ui| {
        ui.label("目标表:");
        egui::ComboBox::from_id_salt("import_target_table")
            .selected_text(if state.selected_table.is_empty() {
                "-- 已有表 --"
            } else {
                &state.selected_table
            })
            .show_ui(ui, |ui| {
                for table in &state.tables {
                    if ui
                        .selectable_label(state.selected_table == *table, table.as_str())
                        .clicked()
                    {
                        state.selected_table = table.clone();
                        state.new_table_name.clear();
                        // 加载目标表的列信息用于映射
                        let _ = cmd_tx.send(EngineCmd::FetchColumns {
                            db_name: state.selected_db.clone(),
                            table_name: table.clone(),
                        });
                    }
                }
            });

        ui.label("或新建:");
        ui.text_edit_singleline(&mut state.new_table_name);
    });

    ui.separator();

    // ── 列映射 ──
    section_header!(ui, "列映射");

    if state.merged_columns.is_empty() {
        ui.label(
            RichText::new("添加数据源后, 此处显示列映射配置")
                .color(Color32::GRAY)
                .size(12.0),
        );
    } else {
        // 表头
        ui.horizontal(|ui| {
            ui.label(RichText::new("源列").strong());
            ui.add_space(40.0);
            ui.label(RichText::new("→ 目标列").strong());
        });
        ui.separator();

        egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
            for (i, (src_col, target_col)) in state.column_mappings.clone().iter().enumerate()
            {
                ui.horizontal(|ui| {
                    ui.label(src_col.as_str());
                    ui.label("→");
                    let mut edited = target_col.clone();
                    if ui
                        .add_sized(
                            [120.0, 18.0],
                            egui::TextEdit::singleline(&mut edited),
                        )
                        .changed()
                    {
                        if i < state.column_mappings.len() {
                            state.column_mappings[i].1 = edited;
                        }
                    }
                });
            }
        });
    }

    ui.add_space(8.0);

    // ── 导入按钮 ──
    let can_import = connected
        && !state.source_manager.sources.is_empty()
        && !state.selected_db.is_empty()
        && (!state.selected_table.is_empty() || !state.new_table_name.is_empty())
        && state.source_manager.sources.iter().all(|s| s.loaded);

    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                can_import && !state.importing,
                egui::Button::new(
                    RichText::new("导入到数据库")
                        .color(SUCCESS_GREEN)
                        .size(13.0),
                )
                .min_size(egui::vec2(180.0, 28.0)),
            )
            .clicked()
        {
            state.importing = true;
            let target_table = if state.new_table_name.is_empty() {
                state.selected_table.clone()
            } else {
                state.new_table_name.clone()
            };

            let paths: Vec<String> = state
                .source_manager
                .sources
                .iter()
                .map(|s| s.file_path.clone())
                .collect();

            let _ = cmd_tx.send(EngineCmd::Import(
                crate::engine::import_engine::ImportConfig {
                    source_paths: paths,
                    target_db: state.selected_db.clone(),
                    target_table,
                    column_mapping: state.column_mappings.clone(),
                },
            ));
        }

        if state.importing {
            ui.add(egui::Spinner::new().size(14.0));
            ui.label(RichText::new("导入中...").color(STEEL_BLUE));
        }
    });

    if !can_import && !state.source_manager.sources.is_empty() {
        ui.add_space(2.0);
        ui.label(
            RichText::new("提示: 请确保已连接数据库, 选择目标, 且所有数据源加载完成")
                .color(WARN_AMBER)
                .size(11.0),
        );
    }
}
