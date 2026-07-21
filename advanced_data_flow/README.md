# advanced_data_flow — 异步数据流平台

异步 PostgreSQL + 双页面工科风 GUI + 自定义宏 + Excel 导入导出 + 列运算。

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
# ★ 启动桌面 GUI（双页面：导出 / 导入）
cargo run -- gui

# CLI 模式
cargo run -- import -f csv -t users --file data.csv
cargo run -- import -f json -t scores --file data.json
cargo run -- export -f csv -t users -o output.csv
cargo run -- export -f json -t users -o output.json
cargo run -- list
cargo run -- show -t users
cargo run -- contention
```

## GUI 双页面设计（工科风格）

启动 `cargo run -- gui` 后，顶部有两个标签页：

### 📤 页面1：数据导出（DB → Excel）

```
┌─ 连接栏 ──────────────────────────────────────────┐
│ ● 已连接 | URL: [postgres://...]  [连接/断开]      │
├─ 左侧控制面板 ───────┬─ 中央预览 ──────────────────┤
│ 数据源               │ 数据预览 — items             │
│  数据库: [flowdb ▼]  │ ┌──────┬───────┬─────────┐  │
│  表名:   [items ▼]   │ │  id  │ name  │  value  │  │
│                      │ ├──────┼───────┼─────────┤  │
│ 列选择 (已选 3)       │ │  1   │ test  │  hello  │  │
│  ☑ id  ☑ name  ☑ val│ │  2   │ demo  │  world  │  │
│  [全选] [全不选]      │ └──────┴───────┴─────────┘  │
│  [加载预览]           │                             │
│                      │                             │
│ 数据运算              │                             │
│  列间运算:            │                             │
│  [col_a ▼] [+ ▼] [col_b ▼]                        │
│  → 结果列名: [new_col] [+添加运算]                  │
│  排序: [name ▼] [ASC ↑] [DESC ↓]                   │
│  行数限制: [500]                                    │
│                      │                             │
│ 导出                  │                             │
│  路径: [export.xlsx] [...]                         │
│  [────── 导出到 Excel ──────]                       │
└──────────────────────┴─────────────────────────────┘
│ 状态栏: ● 已加载 items 表, 共 2 行                   │
```

**功能**:
- 数据库/表两级下拉选择（连接后自动加载）
- 列多选（全选/全不选 + 单列切换）
- 列间四则运算（加减乘除 → 新列）
- 排序（指定列 + ASC/DESC）
- 行数限制
- 中央实时预览
- 导出到 Excel（含表头加粗/蓝钢底色/自动列宽）

### 📥 页面2：数据导入（Sources → DB）

```
┌─ 数据源管理 ───────────────────────────────────────┐
│ [+ 添加数据源]  已添加 2 个数据源                    │
│ ┌─────────────────────────────────────────────────┐│
│ │ [CSV] users.csv   ✓ 4 列, 150 行       [移除]  ││
│ │ [JSON] scores.json ✓ 3 列, 80 行       [移除]  ││
│ └─────────────────────────────────────────────────┘│
├─ 目标数据库 ───────────────────────────────────────┤
│ 数据库: [flowdb ▼] [刷新]                           │
│ 目标表: [items ▼] 或新建: [new_table]               │
│                                                    │
│ 列映射                                              │
│   源列              → 目标列                        │
│   id                → [id           ]              │
│   name              → [name         ]              │
│   score             → [value        ]              │
│                                                    │
│ [────────── 导入到数据库 ──────────]                 │
└────────────────────────────────────────────────────┘
│ 状态栏: ● 导入完成: 2 个数据源, 共 230 行            │
```

**功能**:
- 多数据源管理（CSV/JSON/Excel/TXT，逐个添加/移除）
- 文件格式自动识别 + 彩色格式标签
- 目标数据库/表下拉选择（支持新建表）
- 列映射（自动同名匹配，可手动编辑）
- 批量导入 + 动态建表

## 项目结构

```
advanced_data_flow/
├── Cargo.toml
├── .env
├── README.md
└── src/
    ├── main.rs                  # CLI 入口 (dotenv + clap 子命令 + GUI 启动)
    ├── lib.rs                   # 模块声明
    ├── error.rs                 # FlowError (Io/Csv/Json/Sqlx/Xlsx/Other)
    ├── macros.rs                # ★ 自定义宏 (7个)
    │
    ├── gui/                     # UI 层 — 纯界面, 不碰业务逻辑
    │   ├── mod.rs
    │   ├── app.rs               # 主 App: 标签页/通道/字体加载
    │   ├── export_page.rs       # 页面1: 导出全部 UI
    │   ├── import_page.rs       # 页面2: 导入全部 UI
    │   └── components.rs        # 共享组件: 连接栏/数据表/格式标签
    │
    ├── engine/                  # 业务引擎层 — 不依赖 egui
    │   ├── mod.rs               # 通道消息 EngineCmd/EngineEvent
    │   ├── export_engine.rs     # 导出: 查库→运算→排序→写 Excel
    │   ├── import_engine.rs     # 导入: 读源→映射→建表→写入
    │   └── source_manager.rs    # 数据源注册管理
    │
    ├── models/
    │   ├── mod.rs
    │   └── record.rs            # DataSet / Row (通用数据模型)
    ├── pipeline/
    │   ├── mod.rs
    │   ├── async_readers.rs     # 异步文件读取 (CSV/JSON/TXT)
    │   ├── async_writers.rs     # 异步文件写入
    │   └── db.rs                # PostgreSQL (乐观锁/悲观锁/UPSERT)
    └── service/
        ├── mod.rs
        └── data_service.rs      # 业务编排 (CLI 导入/导出/写竞争)
```

## 分层架构

```
┌──────────────────────────────────────────┐
│  main.rs   CLI 入口 (clap + dotenv)       │
├──────────────────────────────────────────┤
│  gui/      egui 桌面界面 (双页面)          │  ← 纯 UI, 通过 mpsc 与引擎通信
│            app → export_page / import_page│
│            components (共享组件)           │
├──────────────────────────────────────────┤
│  engine/   业务引擎 (不依赖 egui)          │  ← 后台 tokio 线程执行
│            export_engine / import_engine  │
│            source_manager                │
├──────────────────────────────────────────┤
│  pipeline/  底层能力 (纯异步 I/O + DB)     │
│  service/   业务编排 (CLI 用)              │
├──────────────────────────────────────────┤
│  models/  通用数据模型                    │
│  error.rs 统一错误类型                    │
│  macros.rs 自定义宏                       │
└──────────────────────────────────────────┘
```

**异步架构**: GUI 主线程 ← `mpsc::channel` → 后台 tokio 线程。GUI 永不阻塞, DB 操作在后台异步执行, 结果通过事件通道回传。

## 自定义宏（7个）

宏定义在 `src/macros.rs`，通过 `#[macro_export]` 导出，全 crate 可用。

| 宏 | 用途 | 示例 |
|----|------|------|
| `section_header!` | 统一分区标题 + 分隔线 | `section_header!(ui, "数据库连接");` |
| `status_badge!` | 绿/红色连接状态灯 | `status_badge!(ui, self.connected);` |
| `labeled_input!` | 标签 + 输入框组合 | `labeled_input!(ui, "URL:", &mut url);` |
| `combo_box!` | 下拉选择框简写 | `combo_box!(ui, "数据库", &mut db, &dbs);` |
| `action_button!` | 按钮（禁用 + loading） | `action_button!(ui, "连接", loading, "连接中...");` |
| `monospace_label!` | 等宽字体标签 | `monospace_label!(ui, "data");` |
| `error_to_status!` | Result → 状态字符串 | `error_to_status!(result, "成功", "失败");` |

> 宏设计遵循「将一组 UI 调用打包成单个语义化调用」原则，大幅减少 egui 即时模式的重复代码。

## 核心特性：写竞争三种策略

### 1. 乐观锁 (Optimistic Lock)

用 `version` 列检测并发冲突。适合**读多写少**场景。

```sql
UPDATE items SET value = $1, version = version + 1
WHERE id = $2 AND version = $3
```

### 2. 悲观锁 (Pessimistic Lock)

`SELECT FOR UPDATE` 锁定行，其他事务排队。适合**写冲突频繁**场景。

```sql
BEGIN;
SELECT value FROM items WHERE id = $1 FOR UPDATE;
UPDATE items SET value = $2 WHERE id = $1;
COMMIT;
```

### 3. UPSERT (ON CONFLICT)

PostgreSQL 原生，有则更新无则插入。

```sql
INSERT INTO items (name, value) VALUES ($1, $2)
ON CONFLICT (name) DO UPDATE SET value = $2
```

## 涉及的知识点

| 知识点 | 应用位置 |
|---|---|
| async/await + tokio | 全项目异步架构 + 后台 tokio 线程 |
| sqlx (PostgreSQL) | engine + pipeline — 连接池 + 动态查询 |
| egui (桌面 GUI) | gui/ — 双页面即时模式窗口 |
| mpsc 通道 | app.rs — GUI ↔ 引擎异步通信 |
| 自定义宏 `macro_rules!` | macros.rs — 7 个 UI/业务宏 |
| Excel 读写 | rust_xlsxwriter (写) + calamine (读) |
| 文件对话框 | rfd — 原生文件选择器 |
| 解耦架构 | UI(gui/) 完全不依赖业务逻辑(engine/) |
| 动态 SQL + 列映射 | import_engine — 多源合并写入 |
| 列间运算 + 排序 | export_engine — 四则运算 + 数字/字符串排序 |
| ON CONFLICT / FOR UPDATE | db.rs — 三种写竞争 SQL |
| clap (CLI) | main.rs — 子命令 + 环境变量 |
| serde_json | pipeline — JSON 解析与生成 |
| 自定义错误 + From trait | error.rs — 统一 5 种错误 |
