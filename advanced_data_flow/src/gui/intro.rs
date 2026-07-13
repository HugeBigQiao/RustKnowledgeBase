//! Rust GUI 库介绍(桌面应用).

/// 介绍 Rust 主流桌面 GUI 方案.
pub fn show_guide() {
    println!("=== Rust 桌面 GUI 库概览 ===\n");

    println!("1. egui (推荐入门)");
    println!("   - 即时模式(immediate mode), 代码最简洁");
    println!("   - 纯 Rust, 跨平台 (Windows/macOS/Linux/Web)");
    println!("   - 适合: 工具面板、调试界面、数据可视化");
    println!("   - 依赖: eframe (egui 的框架壳)");
    println!("   - Cargo.toml: egui = \"0.28\", eframe = \"0.28\"");
    println!();

    println!("2. iced");
    println!("   - Elm 架构(Model-View-Update), 类型安全");
    println!("   - 响应式布局, 内置组件丰富");
    println!("   - 适合: 正式桌面应用、复杂交互界面");
    println!("   - Cargo.toml: iced = \"0.12\"");
    println!();

    println!("3. slint");
    println!("   - 声明式 UI(.slint 文件定义界面, Rust 写逻辑)");
    println!("   - 内置编译器, 性能优秀");
    println!("   - 适合: 嵌入式设备 UI、跨平台桌面应用");
    println!("   - Cargo.toml: slint = \"1\"");
    println!();

    println!("4. tauri");
    println!("   - 用 Web 技术(HTML/CSS/JS)写 UI, Rust 做后端");
    println!("   - 打包体积小(系统自带 WebView)");
    println!("   - 适合: 已有前端经验、需要跨平台发布");
    println!("   - Cargo.toml: tauri = \"2\"");
    println!();

    println!("5. druid");
    println!("   - 原生渲染, 类似 Flutter 的数据驱动架构");
    println!("   - 由 Rust 社区推动(Xi Editor 编辑器项目)");
    println!("   - 目前维护较慢, 适合学习参考");
    println!();

    println!("选型建议:");
    println!("  快速原型/工具  → egui");
    println!("  正式桌面应用   → iced 或 slint");
    println!("  有前端经验     → tauri");
    println!();

    println!("egui 最小示例(约 20 行):");
    println!("```rust");
    println!("use eframe::egui;");
    println!();
    println!("fn main() -> eframe::Result<()> {{");
    println!("    eframe::run_native(");
    println!("        \"My App\",");
    println!("        eframe::NativeOptions::default(),");
    println!("        Box::new(|_cc| Ok(Box::new(MyApp::default()))),");
    println!("    )");
    println!("}}");
    println!();
    println!("struct MyApp {{ name: String }}");
    println!();
    println!("impl eframe::App for MyApp {{");
    println!("    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{");
    println!("        egui::CentralPanel::default().show(ctx, |ui| {{");
    println!("            ui.heading(\"Hello egui!\");");
    println!("            ui.text_edit_singleline(&mut self.name);");
    println!("            if ui.button(\"Click\").clicked() {{");
    println!("                println!(\"Hi {{}}\", self.name);");
    println!("            }}");
    println!("        }});");
    println!("    }}");
    println!("}}");
    println!("```");
}

/// 运行 egui 演示窗口(需要 eframe 依赖, 默认不编译).
/// 如需体验, 在 Cargo.toml 取消 egui/eframe 注释后运行.
pub fn run_egui_demo() {
    println!("\negui 演示窗口默认不启用(需要额外编译依赖).");
    println!("如需体验:");
    println!("  1. Cargo.toml 添加 egui + eframe 依赖");
    println!("  2. cargo run -- gui");
}
