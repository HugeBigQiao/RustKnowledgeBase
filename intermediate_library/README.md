# intermediate_library — Rust 中级综合实践

图书管理系统，综合运用 intermediate 和 basic 模块学到的全部知识。

## 运行

```bash
# 交互式 CLI 模式 (默认)
cargo run

# 或运行预设演示
cargo run -- demo
```

CLI 模式下进入交互循环, 输入命令管理图书。所有数据仅在内存中, 退出即清空。

## 整体达成效果

这是一个**可交互的图书管理 CLI 工具**, 像一个迷你版的图书馆系统:

- 输入 `add` 逐项填写信息, 添加一本新书 (分配全局唯一 ID)
- 输入 `query 3` 按 ID 精确查找
- 输入 `search title Rust` 模糊搜索书名
- 输入 `list` 列出全部藏书 (自动按年份排序)
- 输入 `count` 查看总数和分类分布
- 输入 `modify 3` 逐字段修改一本书 (留空=不修改)
- 输入 `delete 3` 删除一本书
- 输入 `exit` 退出 (数据归零, 不持久化)

从 Rust 学习的角度看, 这个项目把 20+ 个孤立的知识点**串联成了一套真实可跑的代码**:
你不再孤立地学 HashMap、学 static mut、学 ? 运算符——而是看到它们在一个完整程序里怎么配合。

---

## Rust 项目的基本框架: lib.rs vs main.rs

这是理解 Rust 项目的第一个关键概念。先从 Cargo 项目结构说起:

### 一个标准 Cargo 项目的目录

```
my_project/
├── Cargo.toml      ← 项目元信息 (名称/版本/依赖)
├── Cargo.lock      ← 依赖精确版本锁定 (自动生成)
├── README.md       ← 项目说明
├── src/            ← 所有源代码
│   ├── main.rs     ← (可选) 二进制入口
│   └── lib.rs      ← (可选) 库根文件
└── target/         ← 编译产物 (自动生成, 不要手动改)
    ├── debug/      ← cargo build 的输出
    └── release/    ← cargo build --release 的输出
```

### lib.rs 和 main.rs 的核心区别

这是 Rust 项目设计里最重要的两个文件, 它们代表**两种完全不同的角色**:

| | **lib.rs** (库 crate) | **main.rs** (二进制 crate) |
|---|---|---|
| **产物** | `.rlib` 库文件 (不可执行) | 可执行文件 (.exe / 无后缀) |
| **能被外部引用吗** | ✅ 可以 `use` 导入 | ❌ 不行 |
| **有 `fn main()` 吗** | ❌ 没有 | ✅ 必须有 |
| **作用** | 定义类型/函数/逻辑, 供外部调用 | 程序入口, 启动后调用库的功能 |
| **类比** | 一个 SDK / 工具包 / API | 一个 App / 命令行工具 |

**一句话总结:**
- `lib.rs` = 提供给别人用的**接口** (像你发布一个库给别人 `use`)
- `main.rs` = 自己 crate 内部跑的程序**入口** (`cargo run` 启动的就是它)

### 三种项目形态

一个 Cargo 项目可以同时有 lib.rs 和 main.rs, 也可以只有其中一个:

**1. 只有 main.rs — "脚本式"项目**

```
src/
  main.rs    ← 所有代码都在这里, 或通过 mod 声明子模块
```

- 只能通过 `cargo run` 执行, **不能被其他项目引用**
- 适合: 单文件脚本、命令行工具、一次性程序
- 类比: 一个 .py 脚本, 跑完就完了, 没人 import 它

**2. 只有 lib.rs — "纯库"项目**

```
src/
  lib.rs     ← 库根文件, 声明所有子模块
```

- 无需 `fn main()`, 没有可执行文件
- **专门供其他项目引用**: 别人在 `Cargo.toml` 里写 `my_lib = { path = "..." }`, 然后 `use my_lib::...`
- 适合: SDK、框架、可复用的工具库
- 类比: Python 的 `requests` 库——你 import 它, 但不直接运行它

**3. 同时有 lib.rs + main.rs — "库+应用"项目 (本项目)**

```
src/
  lib.rs     ← 库: 所有可复用的类型和逻辑
  main.rs    ← 应用: 启动并调用 lib.rs 里的东西
```

- `cargo run` 执行 main.rs 里的 `fn main()`
- 但**同时也可以被其他项目引用** lib.rs 里的所有公开类型
- 适用: 大多数"既是工具又是库"的场景

```
┌─────────────────────────────────────────┐
│              外部项目                   │
│  use intermediate_library::models::...  │  ← 调用 lib.rs (接口)
│  use intermediate_library::service::... │
└──────────────┬──────────────────────────┘
               │ 引用 (Cargo.toml)
               ▼
┌──────────────────────────────────────────┐
│        intermediate_library crate        │
│                                          │
│  main.rs ──→ 启动 → 调用 → lib.rs       │
│     (自己内部跑)          (对外接口)      │
│                              │           │
│                    ┌─────────┼──────┐    │
│                    ▼         ▼      ▼    │
│                 models/  service/ error  │
└──────────────────────────────────────────┘
```

本项目正是这种结构: main.rs 只负责 CLI 循环和用户交互, 所有业务逻辑都在 lib.rs 下面。

---

## 模块架构与调用关系

### 目录结构

```
intermediate_library/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              ← 二进制入口: CLI 循环 + static mut NEXT_BOOK_ID
    ├── lib.rs               ← 库根文件: 声明三个子模块
    ├── error.rs             ← 自定义错误: LibraryError 枚举
    ├── models/
    │   ├── mod.rs           ← 声明 book / category / library
    │   ├── book.rs          ← Book 结构体 (数据)
    │   ├── category.rs      ← Category 枚举 (分类)
    │   └── library.rs       ← Library 结构体 (增删改查逻辑)
    └── service/
        ├── mod.rs           ← 声明 cli
        └── cli.rs           ← 交互式命令处理函数
```

### 模块调用链 (从上到下)

```
main.rs                          ← 层 1: 程序入口
  │  声明 static mut NEXT_BOOK_ID (全局 ID 计数器)
  │  loop { 读输入 → match 切片模式分发命令 }
  │
  ├─→ service::cli::cmd_add()    ← 层 2: 命令处理 (service/)
  │      交互式读入书名/作者/分类/年份/标签
  │      │
  │      └─→ Library::add_book() ← 层 3: 业务逻辑 (models/library.rs)
  │              │  Book::new()  ← 层 4: 构造数据 (models/book.rs)
  │              │  填 id/title/author/category/year/tags
  │              │  Entry API 检查 ID 是否重复
  │              └─→ HashMap::entry() → insert
  │
  ├─→ service::cli::cmd_query_by_id()
  │      │
  │      └─→ Library::get_book() → Option<&Book>
  │
  ├─→ service::cli::cmd_search_title/author()
  │      │
  │      └─→ Library::search_by_title/author() → Vec<&Book>
  │             生命周期: 返回的 &Book 借用 library 里的数据
  │
  ├─→ service::cli::cmd_list()
  │      └─→ Library::list_all()  (Vec sort_by 排序)
  │
  ├─→ service::cli::cmd_stats()
  │      └─→ Library::stats()     (BTreeMap 分类计数 + HashSet 标签收集)
  │
  ├─→ service::cli::cmd_delete()
  │      └─→ Library::remove_book() → Result<Book, _>
  │
  └─→ service::cli::cmd_modify()
         └─→ Library::modify_book() → Result<&Book, _>
                内部: get_mut → if let Some → 更新字段
                ? 运算符传播错误
```

### 各模块职责与知识点应用

#### `main.rs` — 程序入口 (第 1 层)

| 责任 | 知识点 |
|---|---|
| 交互式循环读输入 | **basic**: `loop`, `continue`, `break` |
| 命令分发 | **basic**: `match` + 切片模式 `[cmd, arg]` |
| 全局 ID 计数器 | **intermediate**: `static mut` + `unsafe` |
| 输入解析 | **basic**: `trim()`, `to_lowercase()`, `split_whitespace()` |

#### `service/cli.rs` — 命令处理 (第 2 层)

| 责任 | 知识点 |
|---|---|
| 交互式填写书名/作者等 | **basic**: `read_line`, `String` 所有权转移 |
| 解析数字、转换分类 | **basic**: `parse::<u32>()`, `unwrap_or()`; **intermediate**: `Category::from()` (From trait) |
| 调用 Library 方法 | **basic**: `&Library` (不可变借用), `&mut Library` (可变借用) |
| 展示结果 | **basic**: `println!`, `for` + `enumerate()` |
| 错误处理 | **intermediate**: `match Result { Ok/Err }`, `Option` 模式匹配 |
| 条件更新字段 | **basic**: `if let Some(x) = opt` |

#### `models/library.rs` — 核心业务逻辑 (第 3 层)

| 责任 | 知识点 |
|---|---|
| 图书仓库 | **intermediate**: `HashMap<u32, Book>` — K: Hash+Eq |
| 添加 + 去重检查 | **intermediate**: `Entry API` (`Occupied`/`Vacant`), 一次哈希查找 |
| 按 ID 查找 | **intermediate**: `get()` → `Option<&Book>` (生命周期: 返回值借 library) |
| 模糊搜索 | **intermediate**: `filter()` + 生命周期标注 `Vec<&'a Book>` |
| 泛型搜索 | **intermediate**: 泛型 `search<F: Fn(&&Book) -> bool>` + trait bound |
| 列表排序 | **intermediate**: `sort_by` + 闭包比较 (Vec 高级用法) |
| 分类筛选 | **basic**: `filter()`; **intermediate**: `&Category` 模式匹配 |
| 统计信息 | **intermediate**: `BTreeMap` (有序分类) + `HashSet` (标签去重) |
| 修改图书 | **intermediate**: `?` 运算符, `get_mut()`, `Option` 逐字段更新 |
| 删除图书 | **intermediate**: `remove()` → `Result<Book, _>` + `ok_or()` |

#### `models/book.rs` — 数据载体 (第 4 层)

| 责任 | 知识点 |
|---|---|
| 图书字段 | **basic**: `struct` (含 `u32`, `String`), `pub` 可见性 |
| 标签集合 | **intermediate**: `HashSet<String>` — Tag 不可重复 (Hash+Eq) |
| 构造方法 | **basic**: `impl Book { fn new(...) -> Self }` |
| 格式化输出 | **intermediate**: `impl Display for Book` — Trait 实现 |

#### `models/category.rs` — 分类枚举

| 责任 | 知识点 |
|---|---|
| 分类变体 | **basic**: `enum` + 带数据的变体 `Other(String)` |
| 中文显示 | **intermediate**: `impl Display for Category` |
| 字符串转换 | **intermediate**: `impl From<&str> for Category` — 输入"小说"/"science"都能转换 |

#### `error.rs` — 自定义错误

| 责任 | 知识点 |
|---|---|
| 错误类型 | **intermediate**: 自定义 `enum LibraryError` |
| 错误信息 | **intermediate**: `impl Display` + `impl Error` |

---

## 命令列表

| 命令 | 说明 | 调用的层 |
|---|---|---|
| `add` | 添加新书 (交互式填写) | cli → library.add_book() → HashMap::entry() |
| `query <ID>` | 按 ID 查询 | cli → library.get_book() → Option<&Book> |
| `search title <词>` | 按书名模糊搜索 | cli → library.search_by_title() → Vec<&Book> |
| `search author <词>` | 按作者模糊搜索 | cli → library.search_by_author() → Vec<&Book> |
| `list` | 列出全部图书 (按年份排序) | cli → library.list_all() → sort_by |
| `count` / `stats` | 统计信息 (总数+分类分布) | cli → library.stats() → BTreeMap+HashSet |
| `delete <ID>` | 删除图书 | cli → library.remove_book() → Result<Book> |
| `modify <ID>` | 修改图书 (逐字段交互式) | cli → library.modify_book() → get_mut + ? |
| `help` / `h` | 显示帮助 | main.rs 本地 print_help() |
| `exit` / `quit` / `q` | 退出程序 | main.rs break loop |

---

## 知识点一览

### intermediate 模块

| 知识点 | 体现位置 | 作用 |
|---|---|---|
| static mut + unsafe | main.rs | 全局 ID 计数器, 跨命令保持状态 |
| HashMap + Entry API | library.rs | 图书仓库, 一次哈希完成查+插 |
| BTreeMap | library.rs stats | 分类统计按键有序输出 |
| HashSet | book.rs + library.rs stats | 标签集合去重 |
| Result + 自定义错误 | library.rs + cli.rs | 所有增删改查都用 Result 返回值 |
| ? 运算符 | library.rs modify_book | get_mut 失败自动返回错误 |
| Option | library.rs get_book | 查不到返回 None, 不崩溃 |
| 生命周期标注 | library.rs search | 返回值引用不超出 library 生命周期 |
| 泛型 + trait bound | library.rs search<F> | 接受任意条件闭包 |
| Display trait | Book / Category / LibraryStats | 统一格式化输出 |
| From trait | Category | 字符串 → 分类的自动转换 |
| Error trait | LibraryError | 使自定义错误可被 `?` 传播 |
| 切片模式匹配 | main.rs | `match [cmd, arg]` 命令分发 |
| Vec 高级 | library.rs list_all | sort_by 多条件排序 |

### basic 模块

| 知识点 | 体现位置 | 作用 |
|---|---|---|
| loop + break + continue | main.rs | 主循环持续等待命令 |
| match 模式匹配 | main.rs + cli.rs | 命令分发 + 结果处理 |
| String 操作 | cli.rs | trim / split / to_lowercase / parse |
| 所有权 & 借用 | 全项目注释 | &T 不可变借用 / &mut T 可变借用 / move |
| struct / enum / impl | book.rs / category.rs / library.rs | 数据建模 |
| Vec 基本操作 | cli.rs | split → collect 命令行参数 |
| if let | cli.rs modify | 条件更新字段 |
| println! / format! | 全项目 | 格式化输出到终端 |

---

## 项目依赖

不使用任何外部依赖, 所有代码基于 Rust 标准库。
