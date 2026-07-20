# Rust 学习项目

本项目是一个 Rust 学习仓库，按模块拆分，从基础概念到综合项目逐步深入。

## 项目结构

```
RustLearning/
├── basic/                     ← 零基础入门 (19个模块)
├── intermediate/              ← 中级概念 (12个模块)
├── intermediate_library/      ← 综合实践 (图书管理系统)
├── advanced/                  ← 高级概念 (8个模块)
├── advanced_data_pipeline/    ← 项目1: 数据流通 (文件 ↔ SQLite)
├── advanced_data_flow/        ← 项目2: 异步数据流 (async PG + egui)
└── README.md
```

## 前置环境

运行本项目前, 需要先装好 Rust 工具链和编辑器支持.

### 1. 安装 Rust

推荐通过 [rustup](https://rustup.rs) 安装, 它会统一管理 rustc(编译器)、cargo(包管理器)、标准库, 以及后续的版本切换.

```bash
# Windows: 下载 rustup-init.exe 运行, 按提示选择 "1) Proceed with installation"
# Linux / macOS: 终端中执行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

装完后验证:

```bash
rustc --version   # 应显示类似 rustc 1.85.0
cargo --version   # 应显示类似 cargo 1.85.0
```

### 2. 配置镜像源 (中国大陆推荐)

Rust 默认从 [crates.io](https://crates.io) 下载依赖, 国内访问可能较慢。
推荐配置国内镜像加速。

编辑 `~/.cargo/config.toml` (如果不存在则创建):

**Windows** (`%USERPROFILE%\.cargo\config.toml`):
```powershell
mkdir -p $env:USERPROFILE\.cargo
notepad $env:USERPROFILE\.cargo\config.toml
```

**Linux / macOS** (`~/.cargo/config.toml`):
```bash
mkdir -p ~/.cargo
nano ~/.cargo/config.toml
```

写入以下内容 (以清华 TUNA 镜像为例, 使用 sparse 协议):

```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

> **sparse 协议**: Cargo 1.68+ 默认支持的索引协议, 按需逐文件下载而非克隆整个 git 仓库,
> 速度更快, 也是官方推荐的镜像方式。旧版无 `sparse+` 前缀的 git 地址已被多数镜像停用。

> **其他可用镜像**:
> - 中科大: `sparse+https://mirrors.ustc.edu.cn/crates.io-index/`
> - 上海交大: `sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/`
>
> 换回官方源: 删除 `~/.cargo/config.toml` 或注释掉 `replace-with` 行即可。

### 3. 编辑器支持 (rust-analyzer)

`rust-analyzer` 是 Rust 的 LSP 服务器, 提供代码补全、跳转、实时报错等功能.

- **Zed**: 内置支持, 无需额外安装.
- **VS Code**: 安装 `rust-analyzer` 扩展即可, 它会自动下载.
- **手动安装**: `rustup component add rust-analyzer`(一般不需要, 编辑器已集成).

### 4. C++ 编译环境 (Windows 必装)

部分 crate(如 `rusqlite`、`openssl`、`egui` 的某些后端)在编译时需要调用 C/C++ 编译器. **Linux/macOS 通常已自带**, Windows 需要额外安装:

**Windows:**

下载 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), 安装时勾选 **"C++ 生成工具"** 工作负载, 并确认右侧选中以下组件:

- MSVC v143 - VS 2022 C++ x64/x86 生成工具
- Windows 11 SDK (或 Windows 10 SDK)
- C++ CMake tools for Windows

装完后重启终端, 验证:

```bash
where cl   # 应能找到 cl.exe
```

> 如果已经装了 Visual Studio 2022, 只需确认安装了 **"使用 C++ 的桌面开发"** 工作负载即可.

**Linux:**

```bash
# Debian/Ubuntu
sudo apt install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel
```

**macOS:**

```bash
xcode-select --install
```

> **注意**: `intermediate`(rusqlite)、`advanced_data_pipeline`(SQLite) 和 `advanced_data_flow`(PostgreSQL + egui) 三个项目需要 C++ 环境。
> 如果只学习 `basic` / `intermediate` / `advanced` 的概念代码, 纯 Rust 就能编译, 不需要 C++ 环境.

---

## 前置知识

### 堆(Heap)和栈(Stack)

Rust 的所有权系统重度依赖堆和栈的概念, 建议先了解:

- **栈(Stack)**: 内存中一块"后进先出"的区域. 数据大小必须在编译期确定.
  整数、浮点、bool、char、数组等大小固定的类型存在栈上.
  分配和释放极快, 函数调用结束自动弹出.

- **堆(Heap)**: 内存中一块"按需分配"的区域. 用于存放编译期大小未知或
  运行时可变的数据. String、Vec 的实际内容存在堆上, 栈上只放指向堆的
  指针+长度+容量.

- **为什么重要**: Rust 通过所有权追踪堆数据的"主人", 在主人离开作用域时
  自动释放堆内存.

---

## basic/ — 零基础入门

面向零基础, 每个 `.rs` 文件一个主题, 通过 `pub fn run()` 运行时输出教学.

| # | 文件 | 内容 |
|---|---|---|
| 1 | `main.rs` | 程序入口: pub/fn/块、println! 格式化、let/语句/表达式/分号 |
| 2 | `hello_world.rs` | Hello World: 基础概念独立演示 |
| 3 | `base_type.rs` | 基础类型: 整数/浮点/bool/char、类型转换 |
| 4 | `compound_types.rs` | 复合类型: 元组/数组/字符串(先认识类型, 再学所有权) |
| 5 | `bit_ops.rs` | 位运算 |
| 6 | `operator.rs` | 运算符: 算术/比较/逻辑/赋值 |
| 7 | `if_flow.rs` | 条件判断: if / else、if 作为表达式 |
| 8 | `while_flow.rs` | while 循环 |
| 9 | `loop_flow.rs` | loop 循环 + break 返回值 |
| 10 | `for_flow.rs` | for 循环: 遍历数组/Vec/范围 |
| 11 | `match_flow.rs` | 模式匹配: match 穷尽检查/或模式/守卫 |
| 12 | `return_flow.rs` | 函数返回值: 隐式/显式 return |
| 13 | `ownership_and_refs.rs` | ⭐ 所有权与引用: move/Copy/clone/借用/切片 |
| 14 | `vec_type.rs` | 向量 Vec (堆分配, 依赖所有权概念) |
| 15 | `chain_call.rs` | 链式调用: 字符串链/迭代器链 |
| 16 | `closure.rs` | 闭包: map/filter/sort_by (依赖迭代器与借用) |
| 17 | `fizzbuzz.rs` | 综合练习: FizzBuzz |
| 18 | `score_analyzer.rs` | 综合练习: 成绩分析器 |
| 19 | `vec_advanced.rs` | Vec 高级: sort/dedup/retain/windows/chunks |

---

## intermediate/ — 中级概念

| # | 文件 | 内容 |
|---|---|---|
| 1 | `static_and_const.rs` | const 与 static: 编译期常量/全局变量/与 let 的区别/单例模式 |
| 2 | `structs_and_enums.rs` | 结构体(3种) + 枚举 + impl 方法 |
| 3 | `patterns.rs` | 模式匹配深入: 解构/嵌套/@绑定/守卫 |
| 4 | `option.rs` | Option 专题: match/if let/map/and_then |
| 5 | `error_handling.rs` | 错误处理: Result/panic/`?` 运算符 |
| 6 | `generics.rs` | 泛型: 函数/结构体/方法/Trait Bound |
| 7 | `traits.rs` | 特型: 定义/实现/默认方法/Derive 宏 |
| 8 | `lifetimes.rs` | 生命周期: 标注/省略规则/struct 引用/'static |
| 9 | `collections.rs` | 集合: HashMap/Entry API/HashSet/BTreeMap |
| 10 | `file_io.rs` | 文件 I/O: 基础读写 + JSON/CSV 格式序列化、行数列数探查、所有权流转 |
| 11 | `database.rs` | SQLite 入门: rusqlite CRUD、参数化查询、事务 |
| 12 | `macros_intro.rs` | 宏基础: 宏 vs 函数、声明宏/过程宏、内置宏一览 |

---

## intermediate_library/ — 综合实践 (图书管理系统)

综合运用 basic + intermediate 全部知识点。

详见 [intermediate_library/README.md](intermediate_library/README.md)

---

## advanced/ — 高级概念

概念代码演示, 每个模块覆盖一个高级主题:

| # | 文件 | 内容 |
|---|---|---|
| 1 | `smart_pointers.rs` | Box/Deref/Drop/Rc/Arc |
| 2 | `interior_mutability.rs` | Cell/RefCell/Rc\<RefCell\> |
| 3 | `unsafe_rust.rs` | 裸指针/unsafe块/FFI 概念 |
| 4 | `macros.rs` | 自定义宏实战: 捕获类型/重复模式/代码生成/递归宏 |
| 5 | `concurrency.rs` | thread/mpsc/Mutex/Arc |
| 6 | `async_intro.rs` | async/await/Future trait |
| 7 | `networking.rs` | TcpListener/TcpStream/UDP/HTTP |
| 8 | `data_processing.rs` | 数据处理: rust_xlsxwriter (Excel 写入) + polars (DataFrame 分析) |

> 这些模块只演示概念, 深度实战请看下方两个项目.

---

## advanced_data_pipeline/ — 数据流通 (文件 ↔ SQLite)

**练习重点**: CLI 参数解析、文件 I/O、SQLite、日志系统。

```bash
cd advanced_data_pipeline
cargo run -- import -f csv -t users --file data.csv
cargo run -- export -f json -t users -o out.json
cargo run -- list && cargo run -- show -t users && cargo run -- log
```

详见 [advanced_data_pipeline/README.md](advanced_data_pipeline/README.md)

---

## advanced_data_flow/ — 异步数据流 (async PG + 并发)

**练习重点**: async/await、tokio、sqlx(PostgreSQL)、并发写竞争、egui 桌面界面。

```bash
cd advanced_data_flow
cargo run -- gui                     # 启动 egui 桌面窗口
cargo run -- import -f csv -t users --file data.csv
cargo run -- contention              # 写竞争三策略演示
```

详见 [advanced_data_flow/README.md](advanced_data_flow/README.md)

---

## 使用方法

```bash
# 概念学习 (basic/intermediate/advanced)
cd basic && cargo run && cargo run -- ownership_and_refs
cd intermediate && cargo run && cargo run -- file_io
cd advanced && cargo run && cargo run -- data_processing

# 综合实践
cd intermediate_library && cargo run

# 项目实战
cd advanced_data_pipeline && cargo run -- help
cd advanced_data_flow && cargo run -- help
```
