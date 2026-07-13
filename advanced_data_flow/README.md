# advanced_data_flow — 异步数据流平台

异步 PostgreSQL + 并发读写 + 写竞争策略。综合练习 async/await、tokio、sqlx、文件并发 I/O、egui 桌面界面。

## 环境准备

### PostgreSQL

需要本地 PostgreSQL 数据库。默认连接配置（学习用）：

```
postgres://postgres:123456@localhost/flowdb
```

配置方式（三选一）：

1. **`.env` 文件**（推荐）— 项目根目录已有，直接修改 `DATABASE_URL`
2. **命令行参数** — `cargo run -- -d postgres://user:pass@localhost/db <command>`
3. **环境变量** — `$env:DATABASE_URL="postgres://..."`

> ⚠️ **注意**: `.env` 文件在生产项目中**绝不**提交到 Git。此处为学习项目，密码也是演示用的，所以包含了。

### 创建数据库

```sql
-- 在 psql 或 pgAdmin 中执行
CREATE DATABASE flowdb;
```

## 运行

```bash
# 启动 egui 桌面界面
cargo run -- gui

# 导入文件到数据库
cargo run -- import -f csv -t users --file data.csv
cargo run -- import -f json -t scores --file data.json

# 导出数据库到文件
cargo run -- export -f csv -t users -o output.csv
cargo run -- export -f json -t users -o output.json

# 查看
cargo run -- list
cargo run -- show -t users

# 写竞争策略演示（核心特性）
cargo run -- contention
```

## 项目结构

```
advanced_data_flow/
├── Cargo.toml                     # tokio + sqlx(postgres) + egui/eframe + dotenv + clap
├── .env                           # 数据库连接配置 (学习用, 正式项目应 gitignore)
├── README.md
└── src/
    ├── main.rs                    # CLI 入口 (dotenv 加载 + Command 派发 + GUI 启动)
    ├── lib.rs                     # 模块声明
    ├── error.rs                   # FlowError (统一 I/O / SQL / CSV / JSON 错误)
    ├── models/
    │   ├── mod.rs
    │   └── record.rs              # DataSet / Row (通用数据模型)
    ├── pipeline/
    │   ├── mod.rs
    │   ├── async_readers.rs       # 异步文件读取 (tokio::fs + spawn_blocking)
    │   ├── async_writers.rs       # 异步文件写入 (tokio::io + spawn_blocking)
    │   └── db.rs                  # PostgreSQL 操作 (sqlx 异步连接池 + 写竞争三策略)
    ├── service/
    │   ├── mod.rs
    │   └── data_service.rs        # 业务编排 (导入/导出/并发/写竞争演示)
    └── gui/
        ├── mod.rs
        ├── intro.rs               # Rust GUI 库概览 (egui/iced/slint/tauri/druid)
        └── egui_app.rs            # egui 桌面窗口 (连接测试 + 数据查询界面)
```

## 核心特性：写竞争三种策略

### 1. 乐观锁 (Optimistic Lock)

用 `version` 列检测并发冲突。适合**读多写少**场景。

```sql
UPDATE items SET value = $1, version = version + 1
WHERE id = $2 AND version = $3
```

版本号不匹配 → 说明被其他事务抢先 → 返回冲突。

### 2. 悲观锁 (Pessimistic Lock)

`SELECT FOR UPDATE` 锁定行，其他事务排队等待。适合**写冲突频繁**场景。

```sql
BEGIN;
SELECT value FROM items WHERE id = $1 FOR UPDATE;
UPDATE items SET value = $2 WHERE id = $1;
COMMIT;
```

### 3. UPSERT (ON CONFLICT)

PostgreSQL 原生能力，有则更新无则插入。

```sql
INSERT INTO items (name, value) VALUES ($1, $2)
ON CONFLICT (name) DO UPDATE SET value = $2
```

## 分层设计

```
main.rs          CLI 派发 + dotenv 加载, 不包含业务逻辑
    ↓
gui/egui_app     独立进程: egui 桌面窗口 (无需数据库也能启动)
    ↓
service/         编排层: 组合 async_readers/writers/db
    ├── connect / show_table    基础操作
    ├── import_file / export_table  单文件导入导出
    └── demo_write_contention   写竞争演示 (10并发乐观锁 + 5并发悲观锁)
    ↓
pipeline/        能力层: 纯异步操作
    ├── async_readers   tokio::fs + spawn_blocking (CSV/JSON/TXT)
    ├── async_writers   tokio::io + spawn_blocking
    └── db              sqlx::PgPool + 动态 SQL
    ↓
models/          DataSet / Row (与 pipeline 项目共享相同模型)
error.rs         统一错误: From<Io/Csv/Json/Sqlx> → FlowError
```

## 涉及的知识点

| 知识点 | 应用位置 |
|---|---|
| async/await + tokio | 全项目异步架构 |
| sqlx (PostgreSQL) | `db.rs` — 连接池 + 动态查询 |
| spawn_blocking | `readers/writers` — CSV 同步库桥接异步 |
| JoinSet (并发) | `data_service` — 10/5 并发写竞争 |
| mpsc / channel | `concurrency.rs` (advanced 模块) |
| ON CONFLICT / FOR UPDATE | `db.rs` — 三种写竞争 SQL |
| egui (桌面 GUI) | `egui_app.rs` — 即时模式窗口 |
| dotenv | `main.rs` — 从 .env 加载配置 |
| clap (CLI) | `main.rs` — 子命令 + 环境变量 fallback |
| serde_json | `readers/writers` — JSON 解析与生成 |
| 自定义错误 + From trait | `error.rs` |
