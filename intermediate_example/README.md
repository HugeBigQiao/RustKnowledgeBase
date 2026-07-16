# intermediate_example — Rust 中级综合实践

图书管理系统，综合运用 intermediate 和 basic 模块学到的全部知识。

## 运行

```bash
# 交互式 CLI 模式 (默认)
cargo run

# 或运行预设演示
cargo run -- demo
```

CLI 模式下进入交互循环, 输入命令管理图书。所有数据仅在内存中, 退出即清空。

## 命令列表

| 命令 | 说明 |
|---|---|
| `add` | 添加新书 (交互式填写) |
| `query <ID>` | 按 ID 查询 |
| `search title <词>` | 按书名模糊搜索 |
| `search author <词>` | 按作者模糊搜索 |
| `list` | 列出全部图书 (按年份排序) |
| `count` / `stats` | 统计信息 (总数+分类分布) |
| `delete <ID>` | 删除图书 |
| `modify <ID>` | 修改图书 (逐字段交互式) |
| `help` / `h` | 显示帮助 |
| `exit` / `quit` / `q` | 退出程序 |

## 项目结构

```
intermediate_example/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs         # 交互式 CLI 循环 + static mut NEXT_BOOK_ID
    ├── lib.rs          # 库根文件 (模块声明 + 公共导出)
    ├── error.rs        # 自定义错误类型 (LibraryError)
    ├── models/
    │   ├── mod.rs
    │   ├── book.rs     # Book 结构体
    │   ├── category.rs # Category 枚举
    │   └── library.rs  # Library 结构体 + 增删改查方法
    └── service/
        ├── mod.rs
        ├── cli.rs      # 交互式命令处理 (add/query/list/delete/modify/...)
        └── demo.rs     # 预设演示流程
```

## 涉及的知识点

### intermediate 知识点

| 知识点 | 体现位置 |
|---|---|
| static mut + unsafe | main.rs — 全局 ID 计数器 |
| HashMap + Entry API | library.rs — 图书仓库 + add_book 去重 |
| BTreeMap | library.rs — stats 分类统计 (有序) |
| HashSet | book.rs + stats — 标签集合 + 去重统计 |
| Result + 自定义错误 | library.rs + cli.rs — 增删改查全部用 Result |
| ? 运算符 | library.rs modify_book — 错误传播 |
| Option | library.rs get_book → `Option<&Book>` |
| 生命周期标注 | library.rs search — `Vec<&'a Book>` |
| 泛型 | library.rs search<F> — 接受任意条件闭包 |
| Trait | Display (Book/Category/LibraryStats), From (Category), Error (LibraryError) |
| 模式匹配 | main.rs 切片模式分发; cli.rs match 处理命令结果 |
| Vec 高级 | library.rs list_all — sort_by 排序 |

### basic 知识点

| 知识点 | 体现位置 |
|---|---|
| loop + match | main.rs — 主循环 + 命令分发 |
| String 操作 | cli.rs — trim, split, to_lowercase, parse |
| 所有权 & 借用 | 全项目注释 — &T 借用, &mut T 可变借用, move |
| 结构体/枚举/impl | book.rs, category.rs, library.rs |
| Vec 基本操作 | cli.rs — split+collect, push |
| if let | cli.rs cmd_modify — 条件更新字段 |
| println!/format! | 全项目 — 格式化输出 |

## 项目依赖

本项目不使用任何外部依赖, 所有代码基于 Rust 标准库。
