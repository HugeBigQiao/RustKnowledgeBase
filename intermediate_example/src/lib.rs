//! intermediate_example: Rust 中级综合实践项目——图书管理系统 (CLI 交互版)。
//!
//! 运行 `cargo run` 进入交互界面, 输入命令管理图书。
//! 所有数据仅在内存中, 退出即清空。
//!
//! 本 crate 同时提供:
//! - 库 (lib.rs): 所有可复用的类型和逻辑.
//! - 二进制 (main.rs): 交互式 CLI 循环入口.
//!
//! ## 模块结构
//! - `models`: 数据模型 (Book, Category, Library)
//! - `service`: 命令处理 (cli — 交互式命令; demo — 预设演示)
//! - `error`: 自定义错误类型

pub mod error;
pub mod models;
pub mod service;
