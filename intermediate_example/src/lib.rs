//! intermediate_example: Rust 中级综合实践项目——图书管理系统。
//!
//! 本 crate 同时提供:
//! - 库 (lib.rs): 所有可复用的类型和逻辑.
//! - 二进制 (main.rs): CLI 入口 + 完整演示.
//!
//! ## 模块结构
//! - `models`: 数据模型 (Book, Category)
//! - `service`: 业务逻辑 (Library)
//! - `error`: 自定义错误类型

pub mod error;
pub mod models;
pub mod service;
