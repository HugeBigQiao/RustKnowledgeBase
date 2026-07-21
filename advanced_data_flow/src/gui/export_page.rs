//! 页面1: 数据导出 (Database → Excel)
//!
//! 功能:
//!   - 数据库/表两级下拉选择
//!   - 列多选 (可添加/移除模式)
//!   - 列间运算 (加减乘除 → 新列)
//!   - 排序 (列名 + ASC/DESC)
//!   - 行数限制
//!   - 中央数据预览表格
//!   - 导出到 Excel
//!
//! 工科风格: 左面板控制, 中央面板预览, 功能区清晰分组。

use std::sync::mpsc::Sender;

use eframe::egui;
use eframe::egui::{Color32, RichText, Ui};

use crate::engine::export_engine::{ArithmeticOp, ColumnOp, ExportConfig};
use crate::section_header;
use crate::engine::EngineCmd;
use crate::gui::components::{
    draw_data_table, ERROR_RED, STEEL_BLUE, SUCCESS_GREEN,
};

// ======================================================================
//  页面状态
// ======================================================================

pub struct ExportPageState {
    // ── 数据库选择 ──
    pub databases: Vec<String>,
    pub selected_db: String,
    pub tables: Vec<String>,
    pub selected_table: String,

    // ── 列管理 ──
    pub all_columns: Vec<String>,
    /// 用户选中的列 (checkbox)
    pub column_selected: Vec<bool>,

    // ── 列运算 ──
    pub operations: Vec<ColumnOp>,
    /// 临时编辑中的运算
    pub pending_op_col_a: String,
    pub pending_op_col_b: String,
    pub pending_op_arithmetic: ArithmeticOp,
    pub pending_op_result: String,

    // ── 排序 ──
    pub sort_column: String,
    pub sort_ascending: bool,

    // ── 行数限制 ──
    pub row_limit: String, // 编辑中的文本
    pub row_limit_val: usize,

    // ── 预览 ──
    pub preview_columns: Vec<String>,
    pub preview_rows: Vec<Vec<String>>,

    // ── 导出 ──
    pub export_path: String,
    pub exporting: bool,

    // ── 加载状态 ──
    pub loading_databases: bool,
    pub loading_tables: bool,
    pub loading_preview: bool,
}

impl Default for ExportPageState {
    fn default() -> Self {
        ExportPageState {
            databases: Vec::new(),
            selected_db: String::new(),
            tables: Vec::new(),
            selected_table: String::new(),
            all_columns: Vec::new(),
            column_selected: Vec::new(),
            operations: Vec::new(),
            pending_op_col_a: String::new(),
            pending_op_col_b: String::new(),
            pending_op_arithmetic: ArithmeticOp::Add,
            pending_op_result: String::new(),
            sort_column: String::new(),
            sort_ascending: true,
            row_limit: "500".to_string(),
            row_limit_val: 500,
            preview_columns: Vec::new(),
            preview_rows: Vec::new(),
            export_path: "export.xlsx".to_string(),
            exporting: false,
            loading_databases: false,
            loading_tables: false,
            loading_preview: false,
        }
    }
}

impl ExportPageState {
    /// 获取当前选中的列名列表
    pub fn selected_columns(&self) -> Vec<String> {
        self.all_columns
            .iter()
            .enumerate()
            .filter(|(i, _)| self.column_selected.get(*i).copied().unwrap_or(false))
            .map(|(_, c)| c.clone())
            .collect()
    }
}

// ======================================================================
//  页面绘制
// ======================================================================

/// 绘制「数据导出」页面全部 UI。
pub fn draw_export_page(
    ui: &mut Ui,
    state: &mut ExportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    // ── 左侧面板: 控制区 ──
    egui::SidePanel::left("export_control_panel")
        .min_width(260.0)
        .resizable(true)
        .show_inside(ui, |ui| {
            draw_db_selector(ui, state, connected, cmd_tx);
            ui.separator();
            draw_column_selector(ui, state, connected, cmd_tx);
            ui.separator();
            draw_operation_panel(ui, state);
            ui.separator();
            draw_export_controls(ui, state, connected, cmd_tx);
        });

    // ── 中央: 数据预览 ──
    egui::CentralPanel::default().show_inside(ui, |ui| {
        section_header!(ui, format!("数据预览 — {}", state.selected_table));
        draw_data_table(
            ui,
            &state.preview_columns,
            &state.preview_rows,
            Some(&state.selected_columns()),
        );
    });
}

// ======================================================================
//  子区域绘制
// ======================================================================

/// 数据库 + 表 两级选择器
fn draw_db_selector(
    ui: &mut Ui,
    state: &mut ExportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    section_header!(ui, "数据源");

    // 数据库下拉
    ui.horizontal(|ui| {
        ui.label("数据库:");
        egui::ComboBox::from_id_salt("export_db_selector")
            .selected_text(if state.selected_db.is_empty() {
                "-- 选择数据库 --"
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
                        // 加载该数据库的表列表
                        let _ = cmd_tx.send(EngineCmd::ListTables(db.clone()));
                    }
                }
            });

        // 刷新按钮
        if ui
            .add_enabled(connected, egui::Button::new("刷新"))
            .clicked()
        {
            let _ = cmd_tx.send(EngineCmd::ListDatabases);
        }
    });

    ui.add_space(4.0);

    // 表下拉
    ui.horizontal(|ui| {
        ui.label("表名:  ");
        egui::ComboBox::from_id_salt("export_table_selector")
            .selected_text(if state.selected_table.is_empty() {
                "-- 选择表 --"
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
                        // 加载列信息
                        let _ = cmd_tx.send(EngineCmd::FetchColumns {
                            db_name: state.selected_db.clone(),
                            table_name: table.clone(),
                        });
                    }
                }
            });
    });
}

/// 列多选 + 加载预览按钮
fn draw_column_selector(
    ui: &mut Ui,
    state: &mut ExportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    section_header!(ui, format!("列选择 (已选 {})", state.selected_columns().len()));

    if state.all_columns.is_empty() {
        ui.label(RichText::new("请先选择表和数据库").color(Color32::GRAY).size(12.0));
        return;
    }

    // 全选 / 全不选
    ui.horizontal(|ui| {
        if ui.small_button("全选").clicked() {
            for v in &mut state.column_selected {
                *v = true;
            }
        }
        if ui.small_button("全不选").clicked() {
            for v in &mut state.column_selected {
                *v = false;
            }
        }
    });

    ui.add_space(2.0);

    // 列 checkbox 列表
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .show(ui, |ui| {
            for (i, col) in state.all_columns.iter().enumerate() {
                let mut selected = state.column_selected.get(i).copied().unwrap_or(false);
                if ui.checkbox(&mut selected, col.as_str()).changed() {
                    if i < state.column_selected.len() {
                        state.column_selected[i] = selected;
                    }
                }
            }
        });

    ui.add_space(4.0);

    // 加载预览按钮
    if ui
        .add_enabled(
            connected && !state.selected_table.is_empty() && !state.selected_columns().is_empty(),
            egui::Button::new(RichText::new("加载预览").color(STEEL_BLUE)),
        )
        .clicked()
    {
        let limit: usize = state.row_limit.parse().unwrap_or(500);
        let _ = cmd_tx.send(EngineCmd::FetchPreview {
            db_name: state.selected_db.clone(),
            table_name: state.selected_table.clone(),
            columns: state.selected_columns(),
            limit,
        });
    }
}

/// 运算面板: 列间四则运算 + 排序
fn draw_operation_panel(ui: &mut Ui, state: &mut ExportPageState) {
    section_header!(ui, "数据运算");

    // ── 列间四则运算 ──
    ui.label(RichText::new("列间运算").size(12.0).strong());

    let cols: Vec<String> = state.all_columns.clone();
    let col_names: Vec<&str> = cols.iter().map(|s| s.as_str()).collect();

    ui.horizontal(|ui| {
        // 列A
        egui::ComboBox::from_id_salt("op_col_a")
            .selected_text(if state.pending_op_col_a.is_empty() {
                "col_a"
            } else {
                &state.pending_op_col_a
            })
            .width(80.0)
            .show_ui(ui, |ui| {
                for c in &col_names {
                    ui.selectable_value(&mut state.pending_op_col_a, c.to_string(), *c);
                }
            });

        // 运算符
        egui::ComboBox::from_id_salt("op_arithmetic")
            .selected_text(state.pending_op_arithmetic.symbol())
            .width(40.0)
            .show_ui(ui, |ui| {
                for op in &[ArithmeticOp::Add, ArithmeticOp::Sub, ArithmeticOp::Mul, ArithmeticOp::Div] {
                    ui.selectable_value(
                        &mut state.pending_op_arithmetic,
                        op.clone(),
                        op.symbol(),
                    );
                }
            });

        // 列B
        egui::ComboBox::from_id_salt("op_col_b")
            .selected_text(if state.pending_op_col_b.is_empty() {
                "col_b"
            } else {
                &state.pending_op_col_b
            })
            .width(80.0)
            .show_ui(ui, |ui| {
                for c in &col_names {
                    ui.selectable_value(&mut state.pending_op_col_b, c.to_string(), *c);
                }
            });
    });

    ui.horizontal(|ui| {
        ui.label("→ 结果列名:");
        ui.text_edit_singleline(&mut state.pending_op_result);

        if ui
            .add_enabled(
                !state.pending_op_col_a.is_empty()
                    && !state.pending_op_col_b.is_empty()
                    && !state.pending_op_result.is_empty(),
                egui::Button::new("添加运算"),
            )
            .clicked()
        {
            state.operations.push(ColumnOp::Arithmetic {
                col_a: state.pending_op_col_a.clone(),
                op: state.pending_op_arithmetic.clone(),
                col_b: state.pending_op_col_b.clone(),
                result_col: state.pending_op_result.clone(),
            });
            state.pending_op_result.clear();
        }
    });

    // 已添加的运算列表
    if !state.operations.is_empty() {
        ui.add_space(2.0);
        ui.label("已添加的运算:");
        let mut remove_idx: Option<usize> = None;
        for (i, op) in state.operations.iter().enumerate() {
            let ColumnOp::Arithmetic {
                col_a,
                op,
                col_b,
                result_col,
            } = op;
            ui.horizontal(|ui| {
                ui.label(format!("  {col_a} {} {col_b} → {result_col}", op.symbol()));
                if ui
                    .small_button(RichText::new("✕").color(ERROR_RED))
                    .clicked()
                {
                    remove_idx = Some(i);
                }
            });
        }
        if let Some(i) = remove_idx {
            state.operations.remove(i);
        }
    }

    ui.add_space(6.0);

    // ── 排序 ──
    ui.label(RichText::new("排序").size(12.0).strong());
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("sort_column")
            .selected_text(if state.sort_column.is_empty() {
                "-- 列 --"
            } else {
                &state.sort_column
            })
            .width(120.0)
            .show_ui(ui, |ui| {
                for c in &col_names {
                    ui.selectable_value(&mut state.sort_column, c.to_string(), *c);
                }
            });

        if ui
            .selectable_label(state.sort_ascending, "ASC ↑")
            .clicked()
        {
            state.sort_ascending = true;
        }
        if ui
            .selectable_label(!state.sort_ascending, "DESC ↓")
            .clicked()
        {
            state.sort_ascending = false;
        }
    });

    ui.add_space(4.0);

    // ── 行数限制 ──
    ui.horizontal(|ui| {
        ui.label("行数限制:");
        ui.add_sized(
            [60.0, 18.0],
            egui::TextEdit::singleline(&mut state.row_limit).hint_text("500"),
        );
        if let Ok(v) = state.row_limit.parse::<usize>() {
            state.row_limit_val = v;
        }
    });
}

/// 导出路径 + 导出按钮
fn draw_export_controls(
    ui: &mut Ui,
    state: &mut ExportPageState,
    connected: bool,
    cmd_tx: &Sender<EngineCmd>,
) {
    section_header!(ui, "导出");

    // 输出路径
    ui.horizontal(|ui| {
        ui.label("路径:");
        ui.text_edit_singleline(&mut state.export_path);
        if ui.small_button("...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Excel", &["xlsx"])
                .set_file_name("export.xlsx")
                .save_file()
            {
                state.export_path = path.to_string_lossy().to_string();
            }
        }
    });

    ui.add_space(4.0);

    let can_export = connected
        && !state.selected_db.is_empty()
        && !state.selected_table.is_empty()
        && !state.selected_columns().is_empty();

    if ui
        .add_enabled(
            can_export && !state.exporting,
            egui::Button::new(RichText::new("导出到 Excel").color(SUCCESS_GREEN).size(13.0))
                .min_size(egui::vec2(200.0, 28.0)),
        )
        .clicked()
    {
        state.exporting = true;
        let config = ExportConfig {
            db_name: state.selected_db.clone(),
            table_name: state.selected_table.clone(),
            columns: state.selected_columns(),
            operations: state.operations.clone(),
            sort_column: if state.sort_column.is_empty() {
                None
            } else {
                Some(state.sort_column.clone())
            },
            sort_ascending: state.sort_ascending,
            row_limit: Some(state.row_limit_val),
            output_path: state.export_path.clone(),
        };
        let _ = cmd_tx.send(EngineCmd::Export(config));
    }

    if state.exporting {
        ui.add_space(4.0);
        ui.label(RichText::new("导出中...").color(STEEL_BLUE).size(12.0));
        ui.add(egui::Spinner::new().size(14.0));
    }
}
