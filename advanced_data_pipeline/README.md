# advanced_data_pipeline — 数据流通

文件 ↔ SQLite 双向数据管道。综合练习文件 I/O、命令行参数解析、数据库操作、日志记录。

## 功能

| 功能 | 说明 |
|---|---|
| 文件 → SQLite | 支持 CSV / JSON / TXT 三种格式导入 |
| SQLite → 文件 | 导出任意表到 CSV / JSON / TXT |
| 操作日志 | 每次导入/导出自动记录到 `pipeline.log` |
| CLI 命令 | 基于 clap 的命令行界面 |

## 运行

```bash
# 导入 CSV 到数据库
cargo run -- import -f csv -t users --file data/users.csv

# 导入 JSON 到数据库
cargo run -- import -f json -t scores --file data/scores.json

# 查看数据库中有哪些表
cargo run -- list

# 查看某个表的内容
cargo run -- show -t users

# 导出表到文件
cargo run -- export -f csv -t users -o users_export.csv
cargo run -- export -f json -t users -o users_export.json

# 查看操作日志
cargo run -- log
```

## 项目结构

```
advanced_data_pipeline/
├── Cargo.toml                     # rusqlite + csv + serde_json + chrono + clap
├── README.md
└── src/
    ├── main.rs                    # CLI 入口 (clap derive, 命令派发)
    ├── lib.rs                     # 库根 (模块声明)
    ├── error.rs                   # 统一错误类型 (PipelineError)
    ├── logger.rs                  # 操作日志 (pipeline.log + 控制台输出)
    ├── models/
    │   ├── mod.rs
    │   └── record.rs              # 通用数据模型 (DataSet / Row)
    ├── pipeline/
    │   ├── mod.rs
    │   ├── readers.rs             # 文件读取 (CSV / JSON / TXT → DataSet)
    │   ├── writers.rs             # 文件写入 (DataSet → CSV / JSON / TXT)
    │   └── db.rs                  # SQLite 操作 (建表/插入事务/查询/计数)
    └── service/
        ├── mod.rs
        └── pipeline_service.rs    # 业务编排 (导入/导出/查看/日志)
```

## 分层设计

```
main.rs         只做参数解析和命令派发, 不包含业务逻辑
    ↓
service/        编排层: 组合 readers/writers/db 完成具体业务流程
    ↓
pipeline/       能力层: 每个文件只有"做什么", 没有"何时做"
    ├── readers  纯函数: Path → Result<DataSet>
    ├── writers  纯函数: &DataSet + Path → Result<()>
    └── db       SQLite 操作: Connection → CRUD
    ↓
models/         数据模型: DataSet / Row (不含业务逻辑)
    ↓
error.rs        统一错误: 覆盖 I/O / SQLite / CSV / JSON
logger.rs       横切关注点: 所有操作自动打日志
```

## 涉及的知识点

| 知识点 | 应用位置 |
|---|---|
| `Result` + 自定义错误 | `error.rs` — `From` 自动转换 |
| Trait 实现 | `PipelineError: Display + Error + From` |
| 文件 I/O | `readers.rs` / `writers.rs` — 多格式读写 |
| 数据库 CRUD | `db.rs` — SQLite 建表/事务/查询 |
| 命令行 (clap) | `main.rs` — derive 模式, subcommand |
| HashMap 做通用 Row | `models/record.rs` |
| 日志系统 | `logger.rs` — 文件追加 + 控制台同步 |
| 模块组织 | `lib.rs` → `models/` → `pipeline/` → `service/` |
# advanced_data_pipeline — 数据流通管道

文件 ↔ SQLite 双向数据管道，支持 CSV / JSON / 纯文本三种格式。

每条操作自动记录日志（时间戳 + 操作类型 + 详情）。

## 项目结构

```
advanced_data_pipeline/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs                      # CLI 入口 (clap derive)
    ├── lib.rs                       # 模块声明
    ├── error.rs                     # PipelineError (统一错误)
    ├── logger.rs                    # 文件日志 (pipeline.log)
    ├── models/
    │   └── record.rs                # DataSet + Row 通用数据模型
    ├── pipeline/
    │   ├── readers.rs               # FileFormat 枚举 + CSV/JSON/TXT 读取
    │   ├── writers.rs               # CSV/JSON/TXT 写入
    │   └── db.rs                    # SQLite: 建表/插入(事务)/查询
    └── service/
        └── pipeline_service.rs      # import_file / export_table 编排
```

## 快速开始

```bash
cd advanced_data_pipeline

# 导入: 文件 → SQLite
cargo run -- import -f csv -t users --file data.csv
cargo run -- import -f json -t scores --file data.json
cargo run -- import -f txt -t notes --file notes.txt

# 导出: SQLite → 文件
cargo run -- export -f json -t users -o users_out.json
cargo run -- export -f csv -t users -o users_out.csv

# 查看
cargo run -- list               # 列出所有表
cargo run -- show -t users      # 查看表内容
cargo run -- log                # 查看操作日志
```

## 支持的命令

| 命令 | 说明 | 示例 |
|---|---|---|
| `import` | 文件导入数据库 | `-f csv -t users --file data.csv` |
| `export` | 数据库导出到文件 | `-f json -t users -o out.json` |
| `list` | 列出所有表 | |
| `show` | 查看表内容 | `-t users` |
| `log` | 操作日志 | |

## 涉及的知识点

| 知识点 | 体现位置 |
|---|---|
| `clap` derive CLI | main.rs: 子命令 + 参数解析 |
| 自定义错误 (From trait) | error.rs: Io/Sqlite/Csv/Json 统一转换 |
| 文件 I/O + 日志追加 | logger.rs: OpenOptions::append |
| CSV 读写 | pipeline/readers.rs + writers.rs |
| JSON 解析 (serde_json::Value) | pipeline/readers.rs: 动态对象数组 |
| SQLite (rusqlite) | pipeline/db.rs: 动态建表 + 事务插入 |
| 动态 SQL (表名列名拼接) | pipeline/db.rs: format! 构建查询 |
| 分层架构 (models/pipeline/service) | 项目整体结构 |
