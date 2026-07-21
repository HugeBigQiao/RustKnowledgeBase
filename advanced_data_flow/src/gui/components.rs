//! 共享 UI 组件 — 跨页面复用的界面元素。
//!
//! 工科风格: 灰调为主, 蓝钢色强调, 功能性布局, 简洁无装饰。

use eframe::egui;
use eframe::egui::{Color32, RichText, Ui};

use crate::status_badge;

// ======================================================================
//  工科风格色彩常量
// ======================================================================

/// 蓝钢色 — 主要强调色
pub const STEEL_BLUE: Color32 = Color32::from_rgb(70, 130, 180);
/// 成功绿
pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(0, 170, 0);
/// 错误红
pub const ERROR_RED: Color32 = Color32::from_rgb(210, 40, 40);
/// 警告琥珀
pub const WARN_AMBER: Color32 = Color32::from_rgb(200, 140, 0);
/// 面板背景灰
pub const PANEL_BG: Color32 = Color32::from_rgb(45, 45, 48);

// ======================================================================
//  连接栏 — 页面顶部共享
// ======================================================================

/// 绘制数据库连接栏 (URL 输入 + 连接/断开按钮 + 状态)。
///
/// 返回 `true` 表示用户点击了连接/断开按钮,
/// 调用方需根据 `connected` 状态决定发送 `Connect` 或 `Disconnect` 命令。
pub fn draw_connection_bar(
    ui: &mut Ui,
    db_url: &mut String,
    connected: bool,
    loading: bool,
) -> Option<ConnectionAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        // 状态指示灯
        status_badge!(ui, connected);

        ui.separator();

        // URL 输入框
        ui.label("URL:");
        let url_response = ui.add_sized(
            [320.0, 20.0],
            egui::TextEdit::singleline(db_url)
                .font(egui::TextStyle::Body)
                .hint_text("postgres://user:pass@host:5432/dbname"),
        );
        if url_response.changed() && connected {
            // 如果已连接, 修改 URL 时需要提示
        }

        // 连接/断开按钮
        let (btn_text, tooltip) = if connected {
            ("断开", "断开当前数据库连接")
        } else {
            ("连接", "连接到指定数据库")
        };

        let btn = egui::Button::new(RichText::new(btn_text).color(if connected {
            ERROR_RED
        } else {
            STEEL_BLUE
        }))
        .min_size(egui::vec2(60.0, 20.0));

        if ui
            .add_enabled(!loading, btn)
            .on_hover_text(tooltip)
            .clicked()
        {
            action = Some(if connected {
                ConnectionAction::Disconnect
            } else {
                ConnectionAction::Connect
            });
        }

        // Loading 动画
        if loading {
            ui.add(egui::Spinner::new().size(14.0));
        }
    });

    ui.separator();

    action
}

/// 连接栏按钮的动作
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionAction {
    Connect,
    Disconnect,
}

// ======================================================================
//  数据表展示 — 中央面板
// ======================================================================

/// 用 egui Grid 绘制数据预览表。
pub fn draw_data_table(
    ui: &mut Ui,
    columns: &[String],
    rows: &[Vec<String>],
    selected_columns: Option<&[String]>,
) {
    if columns.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("暂无数据").color(Color32::GRAY));
            ui.add_space(4.0);
            ui.label("在左侧选择表并点击「加载预览」");
        });
        return;
    }

    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("data_preview_grid")
            .striped(true)
            .min_col_width(80.0)
            .max_col_width(200.0)
            .show(ui, |ui| {
                // 表头
                for col in columns {
                    let header = if let Some(sel) = selected_columns {
                        if sel.contains(col) {
                            RichText::new(col).strong().color(STEEL_BLUE)
                        } else {
                            RichText::new(col).strong()
                        }
                    } else {
                        RichText::new(col).strong()
                    };
                    ui.label(header);
                }
                ui.end_row();

                // 数据行
                for row in rows {
                    for cell in row {
                        ui.label(cell);
                    }
                    ui.end_row();
                }
            });
    });
}

// ======================================================================
//  格式标签 — 小色块标识文件类型
// ======================================================================

/// 绘制一个小型格式标签。
pub fn draw_format_badge(ui: &mut Ui, format_label: &str) {
    let color = match format_label {
        "CSV" => Color32::from_rgb(100, 160, 100),
        "JSON" => Color32::from_rgb(180, 160, 60),
        "XLSX" => Color32::from_rgb(100, 140, 180),
        _ => Color32::from_rgb(140, 140, 140),
    };

    ui.label(
        RichText::new(format!("[{}]", format_label))
            .color(color)
            .size(11.0)
            .monospace(),
    );
}
