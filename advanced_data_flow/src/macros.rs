//! 自定义宏集合 — 减少 GUI 重复代码, 提高可读性。
//!
//! 所有宏都接受 `$ui:expr` 作为第一个参数 (egui::Ui 引用),
//! 因为 egui 是即时模式, 宏本质上是将一组 UI 调用「打包」成单个语义化调用。
//!
//! # 使用方式
//! ```ignore
//! use crate::macros::*;  // 或 use crate::{section_header, status_badge, ...};
//! ```

// ---------------------------------------------------------------------------
//  section_header! — 统一的分区标题 + 分隔线
// ---------------------------------------------------------------------------
/// 绘制一个带标题和分隔线的区块头。
///
/// # 示例
/// ```ignore
/// section_header!(ui, "数据库连接");
/// ```
#[macro_export]
macro_rules! section_header {
    ($ui:expr, $title:expr) => {
        $ui.add_space(6.0);
        $ui.label(egui::RichText::new($title).size(13.0).strong());
        $ui.separator();
        $ui.add_space(2.0);
    };
}

// ---------------------------------------------------------------------------
//  status_badge! — 连接状态指示灯
// ---------------------------------------------------------------------------
/// 显示连接状态: 绿色圆点 + "已连接" 或 红色圆点 + "未连接"。
///
/// # 示例
/// ```ignore
/// status_badge!(ui, self.connected);
/// ```
#[macro_export]
macro_rules! status_badge {
    ($ui:expr, $connected:expr) => {
        if $connected {
            $ui.colored_label(egui::Color32::from_rgb(0, 170, 0), "●");
            $ui.label("已连接");
        } else {
            $ui.colored_label(egui::Color32::from_rgb(210, 40, 40), "●");
            $ui.label("未连接");
        }
    };
}

// ---------------------------------------------------------------------------
//  labeled_input! — 标签 + 单行文本输入的固定布局
// ---------------------------------------------------------------------------
/// 在一行内绘制标签和输入框。
///
/// # 示例
/// ```ignore
/// labeled_input!(ui, "URL:", &mut self.db_url);
/// ```
#[macro_export]
macro_rules! labeled_input {
    ($ui:expr, $label:expr, $value:expr) => {
        $ui.horizontal(|ui| {
            ui.label($label);
            ui.text_edit_singleline($value);
        });
    };
}

// ---------------------------------------------------------------------------
//  combo_box! — egui ComboBox 简写
// ---------------------------------------------------------------------------
/// 下拉选择框, 带标签。
///
/// # 示例
/// ```ignore
/// combo_box!(ui, "数据库", &mut self.selected_db, &self.databases);
/// ```
#[macro_export]
macro_rules! combo_box {
    ($ui:expr, $label:expr, $selected:expr, $items:expr) => {
        egui::ComboBox::from_label($label)
            .selected_text($selected.as_str())
            .show_ui($ui, |ui| {
                for item in $items {
                    ui.selectable_value($selected, item.clone(), item.as_str());
                }
            });
    };
}

// ---------------------------------------------------------------------------
//  action_button! — 标准操作按钮 (支持禁用态 + loading 文本)
// ---------------------------------------------------------------------------
/// 绘制一个操作按钮, 在 loading 时禁用并显示加载文本。
///
/// # 示例
/// ```ignore
/// if action_button!(ui, "连接", self.loading, "连接中...").clicked() {
///     // do connect
/// }
/// ```
#[macro_export]
macro_rules! action_button {
    ($ui:expr, $label:expr, $loading:expr, $loading_text:expr) => {{
        let text = if $loading { $loading_text } else { $label };
        $ui.add_enabled(!$loading, egui::Button::new(text))
    }};
    ($ui:expr, $label:expr, $loading:expr) => {
        action_button!($ui, $label, $loading, "处理中...")
    };
}

// ---------------------------------------------------------------------------
//  monospace_label! — 等宽字体标签 (用于数据展示)
// ---------------------------------------------------------------------------
/// 以等宽字体显示文本, 适合显示代码或数据。
#[macro_export]
macro_rules! monospace_label {
    ($ui:expr, $text:expr) => {
        $ui.label(egui::RichText::new($text).monospace());
    };
}

// ---------------------------------------------------------------------------
//  error_to_status! — Result → 状态字符串转换
// ---------------------------------------------------------------------------
/// 将 `Result<T, E>` 转换为成功/失败的状态描述。
/// 不用于 UI 绘制, 而是返回 String 用于底部状态栏。
///
/// # 示例
/// ```ignore
/// let msg = error_to_status!(&result, "导出完成", "导出失败");
/// self.status = msg;
/// ```
#[macro_export]
macro_rules! error_to_status {
    ($result:expr, $ok_msg:expr, $err_prefix:expr) => {
        match $result {
            Ok(_) => $ok_msg.to_string(),
            Err(e) => format!("{}: {}", $err_prefix, e),
        }
    };
}
