//! 数据流平台 CLI — 异步 PostgreSQL + 并发读写 + 写竞争策略.
//!
//! 数据库连接通过 .env 文件配置 (默认: postgres://postgres:123456@localhost/flowdb).
//! 也可通过 -d 参数或 DATABASE_URL 环境变量覆盖.
//!
//! 用法:
//!   cargo run -- gui              启动 egui 桌面界面
//!   cargo run -- import -f csv -t users --file data.csv
//!   cargo run -- contention        演示三种写竞争策略

use clap::Parser;

use advanced_data_flow::gui::app;
use advanced_data_flow::pipeline::async_readers::FileFormat;
use advanced_data_flow::service::data_service;

/// 数据流平台: 异步文件 ↔ PostgreSQL + 并发 + 写竞争.
#[derive(Parser)]
#[command(name = "flow")]
#[command(about = "异步数据流平台", long_about = None)]
struct Cli {
    /// PostgreSQL 连接串 (默认从 .env 读取 DATABASE_URL).
    #[arg(short, long, env = "DATABASE_URL")]
    db: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// 导入文件到 PostgreSQL
    Import {
        /// 文件格式
        #[arg(short, long)]
        format: FileFormat,
        /// 源文件路径
        #[arg(long)]
        file: String,
        /// 目标表名
        #[arg(short, long)]
        table: String,
    },
    /// 导出数据库表到文件
    Export {
        /// 导出格式
        #[arg(short, long)]
        format: FileFormat,
        /// 源表名
        #[arg(short, long)]
        table: String,
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
    },
    /// 列出所有表
    List,
    /// 查看表内容
    Show {
        /// 表名
        #[arg(short, long)]
        table: String,
    },
    /// 演示写竞争策略 (乐观锁 / 悲观锁 / UPSERT)
    Contention,
    /// 启动 egui 桌面界面
    Gui,
}

fn get_db_url(cli: &Cli) -> String {
    cli.db
        .clone()
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .unwrap_or_else(|| {
            eprintln!("✗ 未找到数据库连接配置!");
            eprintln!("  1. 创建 .env 文件: DATABASE_URL=postgres://postgres:123456@localhost/flowdb");
            eprintln!("  2. 或通过 -d 参数指定: cargo run -- -d postgres://... <command>");
            std::process::exit(1);
        })
}

#[tokio::main]
async fn main() {
    // 加载 .env 文件 (如果存在)
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    // GUI 模式: 不需要连接数据库, 直接启动窗口
    if let Some(Command::Gui) = cli.command {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:123456@localhost/flowdb".to_string());

        println!("启动 egui 桌面界面...");
        println!("数据库 URL: {}", db_url);
        if let Err(e) = app::run(&db_url) {
            eprintln!("egui 启动失败: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let db_url = get_db_url(&cli);

    // 连接数据库
    let pool = match data_service::connect(&db_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("✗ 连接数据库失败: {}", e);
            eprintln!("  提示: 检查 PostgreSQL 是否运行, 以及连接配置是否正确");
            eprintln!("  当前 URL: {}", db_url);
            std::process::exit(1);
        }
    };

    let command = cli.command.unwrap_or_else(|| {
        println!("未指定命令. 用法:\n");
        println!("  cargo run -- gui          启动 egui 桌面界面");
        println!("  cargo run -- import ...   导入文件到数据库");
        println!("  cargo run -- export ...   导出数据库到文件");
        println!("  cargo run -- list         列出所有表");
        println!("  cargo run -- show ...     查看表内容");
        println!("  cargo run -- contention   写竞争策略演示");
        println!();
        println!("详细信息: cargo run -- help");
        std::process::exit(0);
    });

    let result = match command {
        Command::Import { format, file, table } => {
            data_service::import_file(&pool, &file, &format, &table).await
        }
        Command::Export { format, table, output } => {
            data_service::export_table(&pool, &table, &output, &format).await
        }
        Command::List => {
            data_service::list_tables(&pool).await
        }
        Command::Show { table } => {
            data_service::show_table(&pool, &table).await
        }
        Command::Contention => {
            data_service::demo_write_contention(&pool).await
        }
        Command::Gui => {
            // 已在上面处理, 不会到这里
            unreachable!()
        }
    };

    if let Err(e) = result {
        eprintln!("\n✗ 错误: {}", e);
        std::process::exit(1);
    }
}
