# intermediate_example — Rust 中级综合实践

图书管理系统，综合运用 intermediate 模块学到的全部知识。

## 运行

```bash
# 运行完整演示
cargo run

# 或显式指定
cargo run -- demo
```

---

## Rust 项目结构说明

一个标准的 Rust 项目通常包含以下要素:

### 目录结构

```
intermediate_example/
├── Cargo.toml          # 项目元信息(名称/版本/依赖)
├── README.md           # 项目说明(你正在看的这个)
└── src/
    ├── main.rs         # 二进制入口(只做 CLI 派发)
    ├── lib.rs          # 库根文件(模块声明 + 公共导出)
    ├── error.rs        # 自定义错误类型
    ├── models/         # 数据模型(结构体/枚举 + 纯方法, 无调用逻辑)
    │   ├── mod.rs      # 模块声明
    │   ├── book.rs     # Book 结构体(字段 + impl)
    │   ├── category.rs # Category 枚举(变体 + Display/From trait)
    │   └── library.rs  # Library 结构体(数据容器 + 增删改查方法)
    └── service/        # 业务编排(组装 model, 组织流程)
        ├── mod.rs
        └── demo.rs     # 演示流程: 创建→增删改查→搜索→统计
```

### lib.rs 和 main.rs 的区别

这是 Rust 项目最核心的概念之一:

| | lib.rs (库 crate) | main.rs (二进制 crate) |
|---|---|---|
| **产物** | `.rlib` 库文件 | 可执行文件 (`.exe` / 无后缀) |
| **能被其他项目引用吗** | ✅ 可以 `use` 导入 | ❌ 不行 |
| **有 `fn main()` 吗** | ❌ 不需要 | ✅ 必须有 |
| **作用** | 提供可复用的类型和函数 | 程序入口, 启动并调用库 |
| **类比** | 一个 SDK / 工具包 | 一个 App / 脚本 |

一个项目(Package) 可以同时有两者:

- **开发者 A** 把它当 App: `cargo run` → 执行 `main.rs`
- **开发者 B** 把它当库: 在 `Cargo.toml` 中 `intermediate_example = { path = "..." }` → `use intermediate_example::...`

本项目的分层:
- `main.rs`: **只管 CLI 派发**, 一行逻辑代码都没有
- `lib.rs`: 声明模块, 导出公共 API
- `models/`: 定义"是什么"(结构体/枚举/方法), 不关心"什么时候调用"
- `service/`: 定义"怎么做"(编排流程), 把 model 组合起来完成具体任务

### mod.rs 是什么? 为什么需要它?

`models/mod.rs` 长这样:

```rust
pub mod book;
pub mod category;
pub mod library;
```

它的作用:**告诉 Rust 编译器"这个目录是一个模块, 里面有哪些子模块"**。

**类比其他语言:**

| 语言 | 机制 |
|---|---|
| Rust | `models/mod.rs` 声明 `pub mod xxx;` |
| Python | `models/__init__.py` (可以是空文件, 也可以 `from .book import Book`) |
| JavaScript | `models/index.js` (re-export: `export * from './book'`) |
| Java | 包名就是目录名, 不需要额外文件 |

**为什么 Rust 选择显式声明:** Rust 强调"一切都要明说"——模块不会自动被发现, 必须在 `mod.rs` (或 `lib.rs` 中) 显式写 `mod xxx;`。这带来了两个好处:
1. **代码可读性**: 看 `mod.rs` 就知道这个模块下有哪些子模块, 不用翻目录
2. **编译控制**: 未声明的 `.rs` 文件不会被编译, 方便做条件编译或临时禁用

> 新版本的 Rust (2018 edition 之后) 也可以省略 `mod.rs`, 改成在 `lib.rs` 中写 `mod models { pub mod book; ... }` 或使用 `models.rs` + `models/` 的并列布局, 但 `mod.rs` 仍然是社区最主流的写法.

### 模块的可见性

```rust
mod book;           // 私有: 只有当前 crate 内部能访问
pub mod book;       // 公开: 外部 crate 可以通过 use 访问
pub(crate) mod book;// 仅 crate 内部公开(不暴露给外部)
```

### Package vs Crate vs Module

| 层级 | 说明 | 示例 |
|---|---|---|
| Package | 一个 Cargo 项目, 含 1 个 `Cargo.toml` | 本项目 |
| Crate | 编译单元, 有 lib crate 和 bin crate | `lib.rs` + `main.rs` |
| Module | 代码组织单元, 用 `mod` 声明 | `models/book.rs` |

### 项目依赖

本项目不使用任何外部依赖, 所有代码基于 Rust 标准库.

## 涉及的知识点

| 知识点 | 体现位置 |
|---|---|
| struct | Book, Library, LibraryStats |
| enum | Category, LibraryError |
| impl 方法 | Book::new, Library::add_book 等 |
| 模式匹配 | match 处理错误/命令/枚举 |
| Option | get_book → `Option<&Book>` |
| Result + 自定义错误 | add/remove → `Result<_, LibraryError>` |
| Display / From trait | Category, Book, LibraryError |
| 生命周期标注 | search 返回 `Vec<&'a Book>` |
| 泛型 | `search<F>` 接受任意条件闭包 |
| Vec 高级 | sort_by 排序, filter 过滤 |
| HashMap + Entry API | 图书仓库 + add_book 去重检查 |
| HashSet | 标签集合 + stats 标签收集 |
| BTreeMap | stats 分类统计有序排列 |
# intermediate_example — Rust 中级综合实践

图书管理系统，综合运用 intermediate 模块学到的全部知识。

## 运行

```bash
# 运行完整演示
cargo run

# 或显式指定
cargo run -- demo
```

---

## Rust 项目结构说明

一个标准的 Rust 项目通常包含以下要素:

### 目录结构

```
intermediate_example/
├── Cargo.toml          # 项目元信息(名称/版本/依赖)
├── README.md           # 项目说明(你正在看的这个)
└── src/
    ├── main.rs         # 二进制入口(可执行文件)
    ├── lib.rs          # 库根文件(模块声明 + 公共导出)
    ├── error.rs        # 自定义错误类型
    ├── models/         # 数据模型模块
    │   ├── mod.rs      # 模块声明(pub mod book; pub mod category;)
    │   ├── book.rs     # Book 结构体
    │   └── category.rs # Category 枚举
    └── service/        # 业务逻辑模块
        ├── mod.rs
        └── library.rs  # Library 核心逻辑
```

### 关键概念

**lib.rs vs main.rs**

- `lib.rs` 定义库(library crate), 提供可复用的公开类型和函数.
- `main.rs` 是二进制入口(binary crate), 负责启动和调用.
- 同一个项目可同时拥有两者: `main.rs` 通过 `use crate名::...` 调用 lib 中定义的内容.

**crate 名称**

- `Cargo.toml` 中 `name` 字段决定的 crate 名, 在 `main.rs` 中以 `use <name>::...` 引用.
- 本项目名为 `intermediate_example`, 因此 main.rs 中写:
  ```rust
  use intermediate_example::service::library::Library;
  ```

**模块系统(mod.rs)**

- 将子模块放在以模块名命名的目录下, 用 `mod.rs` 声明.
- 例如 `models/` 目录下的 `mod.rs` 写 `pub mod book;`, Rust 就会去加载 `models/book.rs`.
- 声明为 `pub mod` 的模块才能被外部访问.

**包(Package) vs 库(Crate) vs 模块(Module)**

| 层级 | 说明 | 示例 |
|---|---|---|
| Package | 一个 Cargo 项目, 含 1 个 Cargo.toml | 本项目 |
| Crate | 编译单元, 有 lib crate 和 bin crate | `lib.rs` + `main.rs` |
| Module | 代码组织单元, 用 `mod` 声明 | `models/book.rs` |

### 项目依赖

本项目不使用任何外部依赖, 所有代码基于 Rust 标准库.

## 涉及的知识点

| 知识点 | 体现位置 |
|---|---|
| struct | Book, LibraryStats |
| enum | Category, LibraryError |
| impl 方法 | Book::new, Library::add_book 等 |
| 模式匹配 | match 处理错误/命令/枚举 |
| Option | get_book 返回 Option<&Book> |
| Result + 自定义错误 | add_book/remove_book 返回 Result |
| Display / From trait | Category, Book, LibraryError |
| 生命周期标注 | search/search_by_author 返回引用 |
| 泛型 | Library::search<F> 泛型搜索 |
| Vec 高级 | sort_by 排序, filter 过滤 |
| HashMap | books: HashMap<u32, Book> |
| Entry API | add_book 中检查重复 |
| HashSet | tags: HashSet<String>, stats 标签收集 |
| BTreeMap | stats 中按分类排序计数 |
