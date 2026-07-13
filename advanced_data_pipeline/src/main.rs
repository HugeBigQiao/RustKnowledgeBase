//! 数据管道 CLI — 文件 ↔ SQLite 双向流通.
//!
//! 支持格式: CSV / JSON / TXT.
//! 每条操作都有日志记录.

use clap::Parser;

use advanced_data_pipeline::pipeline::readers::FileFormat;
use advanced_data_pipeline::service::pipeline_service;

/// 数据管道: 文件 ↔ SQLite 数据库.
#[derive(Parser)]
#[command(name = "pipeline")]
#[command(about = "文件 ↔ SQLite 数据管道", long_about = None)]
struct Cli {
    /// 数据库文件路径(默认: pipeline.db).
    #[arg(short, long, default_value = "pipeline.db")]
    db: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// 导入文件到数据库
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
    /// 查看操作日志
    Log,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Command::Import { format, file, table } => {
            pipeline_service::import_file(Some(&cli.db), std::path::Path::new(file), format, table)
        }
        Command::Export { format, table, output } => {
            pipeline_service::export_table(Some(&cli.db), table, std::path::Path::new(output), format)
        }
        Command::List => {
            pipeline_service::show_tables(Some(&cli.db))
        }
        Command::Show { table } => {
            pipeline_service::show_table(Some(&cli.db), table)
        }
        Command::Log => {
            pipeline_service::show_log();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("\n✗ 错误: {}", e);
        std::process::exit(1);
    }
}
