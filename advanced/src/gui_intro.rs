//! Rust GUI 桌面开发入门 — egui 框架介绍与实践。
//!
//! ┌── Rust GUI 生态概览 ────────────────────────────────────────┐
//! │                                                              │
//! │  Rust 的 GUI 框架大致分为三类:                                │
//! │                                                              │
//! │  1. 「原生绑定」— 包装 C/C++ GUI 库                          │
//! │     • gtk-rs: GTK 绑定, Linux 原生外观, 跨平台但 Windows     │
//! │       上需要安装 GTK 运行时 (体积大, 配置烦琐)               │
//! │     • fltk-rs: FLTK 绑定, 极轻量 (~1MB), 但现代感不足       │
//! │     • qt-rs / cxx-qt: Qt 绑定, 功能全但依赖 Qt SDK           │
//! │                                                              │
//! │  2. 「平台抽象」— Rust 实现 + 平台原生渲染                   │
//! │     • iced:   Elm 架构, 响应式, 类型安全的声明式布局         │
//! │     • druid:  数据驱动, 类似 React 的单向数据流              │
//! │     • slint:  声明式 .slint 标记语言, UI 与逻辑分离          │
//! │                                                              │
//! │  3. 「Web 技术」— HTML/CSS 渲染 + Rust 逻辑                  │
//! │     • tauri:  前端用 HTML/CSS/JS, 后端 Rust, 像 Electron    │
//! │       但更轻量 (二进制 ~5MB vs Electron ~100MB+)             │
//! │     • dioxus: React 风格, 支持 Web/Desktop/Mobile            │
//! │                                                              │
//! │  4. 「即时模式」— 每帧重绘整个 UI, 无状态管理负担            │
//! │     • egui:   纯 Rust, 即时模式 GUI, 极简 API, 适合工具/     │
//! │       仪表盘/调试界面。Rerun、Bevy Editor 等知名项目使用。   │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘
//!
//! ┌── 为什么先学 egui ──────────────────────────────────────────┐
//! │                                                              │
//! │  1. 零外部依赖 (纯 Rust 实现)                                │
//! │     — 不像 gtk/fltk/qt 需要安装 C/C++ 系统库                 │
//! │     — 一个 `cargo run` 就能跑, 无需配置环境                  │
//! │                                                              │
//! │  2. 即时模式 (Immediate Mode) 概念简单                       │
//! │     — 没有"组件树"、"事件路由"、"回调地狱"                  │
//! │     — UI = 每帧调用一次的函数, 所见即所写                    │
//! │     — 新手友好: 不需要理解复杂的状态管理框架                 │
//! │                                                              │
//! │  3. API 极简, 上手快                                         │
//! │     — `ui.button("确定")` 一行就是一个按钮                   │
//! │     — 状态直接存在 struct 里, 不需要状态管理库               │
//! │                                                              │
//! │  4. 后续项目会用到                                           │
//! │     — 后面的实践项目 (数据浏览器、日志查看器) 都将用 egui    │
//! │     — 学到的是可以立刻实战的技能                             │
//! │                                                              │
//! └──────────────────────────────────────────────────────────────┘

// ======================================================================
//  Part 1: 即时模式 vs 保留模式
// ======================================================================
//
//  保留模式 (Retained Mode, 如 Qt/GTK/iced):
//    • 创建 UI 控件 → 控件作为对象保留在内存中
//    • 状态分离: UI 状态存在于控件对象内, 业务状态在另一处
//    • 更新: 通过回调/信号/set_state 修改控件的属性
//    • 典型代码: button.set_enabled(false); label.set_text("new");
//
//  即时模式 (Immediate Mode, 如 egui/Dear ImGui):
//    • 每帧重新"声明"整个 UI
//    • 状态合一: 业务状态就是 UI 状态 (存在你的 struct 里)
//    • 更新: 直接修改你的变量, 下一帧自动反映在 UI 上
//    • 典型代码: if ui.button("删除").clicked() { items.remove(idx); }

// ======================================================================
//  Part 2: egui 的局限性 (选型必读)
// ======================================================================
//
//  即时模式有取舍, egui 并非万能, 需要了解它不擅长什么:
//
//  1. 非原生外观
//     • egui 自己绘制所有控件 (不调用 OS 原生组件)
//     • 无法获得 Windows/macOS/Linux 的原生窗口控件风格
//     • 有暗色/亮色主题, 但不如 CSS 灵活
//
//  2. 布局能力有限
//     • 没有 CSS Flexbox / Grid 那样的复杂布局引擎
//     • 布局靠 ui.horizontal() / ui.columns() 等基础原语拼凑
//     • 复杂的自适应性布局需要手动计算坐标
//
//  3. 性能在复杂场景会退化
//     • 每帧重绘全部 UI, 即使只有一个小区域变化
//     • UI 极复杂 (1000+ widget) 时 CPU 开销明显
//     • 不过对工具类/仪表盘界面通常够用
//
//  4. 动画支持较弱
//     • 没有内置的过渡动画系统 (淡入/滑动等)
//     • 需要自己用 lerp + ctx.request_repaint() 手动实现
//
//  5. 富文本排版受限
//     • 不支持同一行混用粗体/斜体/颜色 (只有基础 label)
//     • 不支持从右向左 (RTL) 的文字排版
//     • 复杂文档场景 → 考虑 tauri + HTML/CSS 更合适
//
//  6. 无障碍 (Accessibility) 不完善
//     • 屏幕阅读器支持有限
//     • 键盘导航需要手动实现 (Tab 切换焦点等)
//
//  7. 移动端/触屏不理想
//     • egui 可通过 eframe 编译到 Web (WebGL), 但即时模式
//       不是触屏/移动端的最佳范式
//     • 触屏 UI 推荐用保留模式框架或 Flutter
//
//  8. 单窗口为主
//     • eframe 默认只管理一个窗口
//     • 多窗口场景需要自己管理 (或换 winit + egui_winit)
//
//  选型建议:
//    适合: 内部工具 / 调试面板 / 数据仪表盘 / 简易编辑器 / 游戏 UI
//    不适合: 面向消费者的产品 / 文档型应用 / 复杂动画 / 移动端
//    其他框架何时选: tauri → 需要 Web 前端 + 复杂布局
//                     iced → 需要原生外观 + 响应式架构
//                     slint → 需要声明式 UI + 嵌入式/低资源

// ======================================================================
//  Part 3: egui 核心类型与方法
// ======================================================================
//
//  核心类型 (由 eframe/egui 提供):
//
//  ┌────────────────┬───────────────────────────────────────────┐
//  │ 类型           │ 作用                                      │
//  ├────────────────┼───────────────────────────────────────────┤
//  │ egui::Context   │ 全局画布上下文; 管理字体/样式/输入状态    │
//  │ egui::Ui        │ 布局容器; 所有 widget 方法都挂在它上面    │
//  │ egui::Response   │ widget 的返回值; .clicked() / .changed() │
//  │ egui::Frame     │ 窗口框架; 控制标题/大小/关闭行为          │
//  │ eframe::App     │ 应用 trait; 实现 update() 方法            │
//  │ egui::Id        │ widget 唯一标识; 用于状态持久化           │
//  └────────────────┴───────────────────────────────────────────┘
//
//  核心方法 (在 Ui 上调用):
//
//  ┌──────────────────────┬─────────────────────────────────────┐
//  │ 类别                 │ 方法                                │
//  ├──────────────────────┼─────────────────────────────────────┤
//  │ 文本                 │ ui.label(), ui.heading(),           │
//  │                      │ ui.monospace(), ui.code()           │
//  │ 按钮                 │ ui.button(), ui.small_button()      │
//  │ 输入框               │ ui.text_edit_singleline(),          │
//  │                      │ ui.text_edit_multiline()            │
//  │ 数值输入             │ ui.add(egui::Slider::new())         │
//  │                      │ ui.add(egui::DragValue::new())      │
//  │ 选择                 │ ui.checkbox(), ui.radio()           │
//  │                      │ egui::ComboBox::new()               │
//  │ 布局                 │ ui.horizontal(), ui.vertical()      │
//  │                      │ ui.columns(), ui.group()            │
//  │ 容器                 │ egui::ScrollArea, egui::Window,     │
//  │                      │ egui::CentralPanel, SidePanel       │
//  │ 分隔/间距            │ ui.separator(), ui.add_space()      │
//  └──────────────────────┴─────────────────────────────────────┘

// ======================================================================
//  Part 4: 常规项目结构
// ======================================================================
//
//  一个典型 egui 项目的文件布局:
//
//   my_egui_app/
//   ├── Cargo.toml          → 依赖: eframe = "0.30"
//   ├── src/
//   │   ├── main.rs          → 入口: eframe::run_native(...)
//   │   ├── app.rs           → 主应用: impl eframe::App for MyApp
//   │   ├── state.rs         → 应用状态: struct AppState { ... }
//   │   ├── widgets/         → 自定义 widget 组件
//   │   │   ├── mod.rs
//   │   │   └── data_table.rs
//   │   └── theme.rs         → 自定义样式/字体
//   └── assets/              → 字体文件/图标
//
//  关键: App trait 只需要实现一个方法:
//
//     impl eframe::App for MyApp {
//         fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//             // 每帧调用, 在这里声明 UI
//             egui::CentralPanel::default().show(ctx, |ui| {
//                 ui.heading("Hello, egui!");
//             });
//         }
//     }

// ======================================================================
//  Part 5: Demo — 图书信息录入表单
// ======================================================================

/// 一本书的记录。
#[derive(Clone, Debug)]
struct BookEntry {
    title: String,
    author: String,
    year: u32,
}

/// 应用状态 — 即时模式中, 所有可变数据都存这里。
///
/// 没有"状态管理库"、没有"事件总线" — 就一个普通 struct。
struct BookFormApp {
    // —— 输入框当前值 (直接绑定 struct 字段) ——
    title_input: String,
    author_input: String,
    year_input: u32,

    // —— 已提交的图书列表 ——
    books: Vec<BookEntry>,

    // —— UI 辅助 ——
    show_list: bool,
    status_msg: String,
}

impl Default for BookFormApp {
    fn default() -> Self {
        Self {
            title_input: String::new(),
            author_input: String::new(),
            year_input: 2025,
            books: Vec::new(),
            show_list: true,
            status_msg: String::from("请填写图书信息"),
        }
    }
}

impl eframe::App for BookFormApp {
    /// update() 每帧 (约 60fps) 调用一次。
    ///
    /// &mut self: 可变借用 — 可以直接修改应用状态
    /// ctx: &Context — 全局上下文, 提供输入、字体、样式等信息
    /// _frame: &mut Frame — 窗口控制 (关闭、设置标题)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── 左侧面板: 录入表单 ──
        egui::SidePanel::left("form_panel")
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.heading("图书信息录入");
                ui.separator();

                // text_edit_singleline: 单行文本输入
                // 所有权: &mut String — 直接修改 struct 字段, 不需要回调
                ui.label("书名:");
                ui.text_edit_singleline(&mut self.title_input);

                ui.label("作者:");
                ui.text_edit_singleline(&mut self.author_input);

                ui.label(format!("出版年份: {}", self.year_input));
                // Slider: 一个拖动滑块
                // ui.add() 的泛型方式可添加任意 egui::Widget
                ui.add(egui::Slider::new(&mut self.year_input, 1900..=2025));

                ui.add_space(10.0);

                // 提交按钮
                // button().clicked() 在当前帧判断是否被点击
                if ui.button("提交").clicked() {
                    if self.title_input.trim().is_empty() {
                        self.status_msg = String::from("书名不能为空!");
                    } else {
                        self.status_msg = format!("已添加: '{}'", self.title_input);
                        self.books.push(BookEntry {
                            title: std::mem::take(&mut self.title_input),
                            //      ^^^^^^^^^^^ take(): 取出 String 所有权并留下空串
                            author: std::mem::take(&mut self.author_input),
                            year: self.year_input,
                        });
                    }
                }

                // 清空按钮
                if ui.button("清空表单").clicked() {
                    self.title_input.clear();
                    self.author_input.clear();
                    self.year_input = 2025;
                    self.status_msg = String::from("表单已清空");
                }

                ui.separator();
                ui.label(&self.status_msg);
            });

        // ── 中央区域: 图书列表 ──
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("已录入图书");
                // checkbox 返回 &mut bool, clicked() 判断状态变化
                ui.checkbox(&mut self.show_list, "显示列表");
            });

            ui.separator();

            if self.show_list && !self.books.is_empty() {
                // ScrollArea: 内容超出时自动出现滚动条
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // egui::Grid: 表格状布局
                    egui::Grid::new("book_grid")
                        .striped(true) // 斑马条纹
                        .show(ui, |ui| {
                            // 表头
                            ui.monospace("书名");
                            ui.monospace("作者");
                            ui.monospace("年份");
                            ui.monospace("操作");
                            ui.end_row();

                            for (i, book) in self.books.iter().enumerate() {
                                ui.label(&book.title);
                                ui.label(&book.author);
                                ui.label(book.year.to_string());
                                // small_button: 比 button 更紧凑
                                if ui.small_button("删除").clicked() {
                                    self.books.remove(i);
                                    self.status_msg = format!("已删除: '{}'", book.title);
                                }
                                ui.end_row();
                            }
                        });
                });
            } else if self.books.is_empty() {
                ui.label("暂无图书, 请在左侧录入");
            }

            ui.add_space(20.0);
            ui.separator();
            // 底部状态栏
            ui.label(format!(
                "共 {} 本书 | 即时模式 GUI — 状态直接绑定 struct 字段",
                self.books.len()
            ));
        });

        // 快捷键: Ctrl+Q 退出
        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            std::process::exit(0);
        }
    }
}

// ======================================================================
//  Part 6: 常用模式速查
// ======================================================================

/// 纯文档函数 — 归纳 egui 开发中的常见模式。
fn _patterns_guide() {
    // 这段代码不会被运行, 仅作为注释式参考。

    // 1. 布局容器
    //    ui.horizontal(|ui| { ... })     — 水平排列
    //    ui.vertical(|ui| { ... })       — 垂直排列 (默认)
    //    ui.columns(2, |cols| { ... })   — 等宽多列

    // 2. 条件渲染 (即时模式的特色)
    //    if show_settings {
    //        ui.label("高级设置");
    //    }
    //    — 不需要 "隐藏/显示" 属性, 直接 if 决定是否调用 widget

    // 3. 使用 ui.add() 添加非标准 widget
    //    ui.add(egui::Image::new(image_texture));       // 图片
    //    ui.add(egui::ProgressBar::new(0.7));           // 进度条
    //    ui.add(egui::Spinner::new());                  // 加载动画
    //    ui.add(egui::ColorEdit::new(&mut color));      // 颜色选择器

    // 4. 模态弹窗
    //    if show_confirm {
    //        egui::Window::new("确认").show(ctx, |ui| {
    //            ui.label("确定删除?");
    //            if ui.button("确定").clicked() { ... }
    //        });
    //    }

    // 5. 自定义样式
    //    ctx.set_visuals(egui::Visuals::dark());       // 暗色主题
    //    ctx.style_mut(|s| { s.spacing.item_spacing = egui::vec2(8.0, 4.0); });

    // 6. 菜单栏
    //    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
    //        egui::menu::bar(ui, |ui| {
    //            ui.menu_button("文件", |ui| { ... });
    //        });
    //    });
}

// ======================================================================
//  总结
// ======================================================================

pub fn run() {
    println!("══════════ Rust GUI 桌面开发 — egui 入门 ══════════\n");

    println!("  其他 GUI 框架:");
    println!("    原生绑定:  gtk-rs (GTK), fltk-rs (极轻量)");
    println!("    平台抽象:  iced (Elm风), druid (数据驱动), slint (声明式)");
    println!("    Web 技术:  tauri (Electron替代), dioxus (React风)");
    println!("    即时模式:  egui ★ — 本篇重点\n");

    println!("  为什么选 egui:");
    println!("    1. 零外部依赖 — 纯 Rust, cargo run 直接跑");
    println!("    2. 即时模式 — 没有组件树/事件路由, 所见即所写");
    println!("    3. 状态直接绑定 — struct 字段就是 UI 状态, 无需额外框架");
    println!("    4. 后续项目会继续用 — 数据浏览器、日志查看器等\n");

    println!("  启动 Demo 应用...");
    println!("  (一个图书录入表单窗口, 展示了: 输入框/滑块/按钮/表格/布局)\n");

    // eframe::run_native 打开原生窗口, 进入事件循环
    // 参数说明:
    //   "egui_demo"      — 窗口标题
    //   native_options    — 窗口大小等配置
    //   Box::new(...)     — App 实例 (所有权移入框架)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 480.0])
            .with_min_inner_size([600.0, 360.0]),
        ..Default::default()
    };

    // run_native 会阻塞当前线程, 直到窗口关闭
    eframe::run_native(
        "egui 图书录入 Demo — Rust GUI 入门",
        options,
        Box::new(|_cc| {
            // _cc: CreationContext — 包含 egui::Context, 可用于初始设置
            Ok(Box::<BookFormApp>::default())
        }),
    )
    .expect("egui 窗口启动失败");
}

