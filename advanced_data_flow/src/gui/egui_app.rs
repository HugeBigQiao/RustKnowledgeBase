//! egui 桌面窗口: 连接状态监视 + 数据查看器.

use eframe::egui;

/// 应用状态.
#[derive(Default)]
pub struct FlowApp {
    /// 输入的数据库连接串.
    db_url: String,
    /// 要查询的表名.
    table_name: String,
    /// 状态信息.
    status: String,
}

impl FlowApp {
    pub fn new(db_url: &str) -> Self {
        FlowApp {
            db_url: db_url.to_string(),
            table_name: String::new(),
            status: "就绪 — 点击「测试连接」开始".to_string(),
        }
    }
}

impl eframe::App for FlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("advanced_data_flow — 数据流平台");

            ui.separator();

            // ---- 数据库连接配置 ----
            ui.horizontal(|ui| {
                ui.label("数据库 URL:");
                ui.text_edit_singleline(&mut self.db_url);
            });

            ui.horizontal(|ui| {
                if ui.button("测试连接").clicked() {
                    self.status = format!(
                        "正在连接 {} ... (GUI 演示: 实际连接需要在异步运行时中执行)",
                        self.db_url
                    );
                }

                if ui.button("初始化 Schema").clicked() {
                    self.status = "Schema 初始化请求已发送 (演示模式)".to_string();
                }
            });

            ui.separator();

            // ---- 数据查询 ----
            ui.horizontal(|ui| {
                ui.label("表名:");
                ui.text_edit_singleline(&mut self.table_name);
            });

            ui.horizontal(|ui| {
                if ui.button("查询数据").clicked() {
                    if self.table_name.is_empty() {
                        self.status = "请先输入表名".to_string();
                    } else {
                        self.status = format!(
                            "查询表 '{}' (演示: 实际需要异步 PG 连接)",
                            self.table_name
                        );
                    }
                }

                if ui.button("列出所有表").clicked() {
                    self.status = "获取表列表 (演示模式)".to_string();
                }
            });

            ui.separator();

            // ---- 状态栏 ----
            ui.label(egui::RichText::new("状态:").strong());
            ui.label(&self.status);

            ui.separator();

            // ---- 说明 ----
            ui.collapsing("关于", |ui| {
                ui.label("Rust 数据流平台 — 综合练习项目");
                ui.label("• PostgreSQL 异步操作 (sqlx + tokio)");
                ui.label("• 文件并发读写 (CSV/JSON/TXT)");
                ui.label("• 写竞争策略 (乐观锁/悲观锁/UPSERT)");
                ui.label("• egui 桌面界面演示");
                ui.label("");
                ui.label("提示: GUI 仅演示界面结构, 数据库操作请用 CLI 模式.");
            });
        });

        // 每秒刷新一次
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

/// 启动 egui 窗口.
pub fn run(db_url: &str) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let db_url = db_url.to_string();
    eframe::run_native(
        "advanced_data_flow — 数据流平台",
        options,
        Box::new(move |_cc| Ok(Box::new(FlowApp::new(&db_url)))),
    )
}
