# Rust 学习项目

本项目是一个 Rust 学习仓库，按模块拆分，从基础概念到综合项目逐步深入。

## 项目结构

```
RustLearning/
├── basic/                     ← 零基础入门 (18个模块)
├── intermediate/              ← 中级概念 (9个模块)
├── intermediate_example/      ← 综合实践 (图书管理系统)
├── advanced/                  ← 高级概念 (9个模块)
├── advanced_data_pipeline/    ← 项目1: 数据流通 (文件 ↔ SQLite)
├── advanced_data_flow/        ← 项目2: 异步数据流 (async PG + egui)
└── README.md
```

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
| 4 | `bit_ops.rs` | 位运算 |
| 5 | `operator.rs` | 运算符: 算术/比较/逻辑/赋值 |
| 6 | `if_flow.rs` | 条件判断: if / else、if 作为表达式 |
| 7 | `chain_call.rs` | 链式调用: 字符串链/迭代器链 |
| 8 | `closure.rs` | 闭包: map/filter/sort_by |
| 9 | `while_flow.rs` | while 循环 + while let |
| 10 | `loop_flow.rs` | loop 循环 + break 返回值 |
| 11 | `for_flow.rs` | for 循环: 遍历数组/Vec/范围 |
| 12 | `match_flow.rs` | 模式匹配: match 穷尽检查/或模式/守卫 |
| 13 | `return_flow.rs` | 函数返回值: 隐式/显式 return |
| 14 | `ownership_and_refs.rs` | ⭐ 所有权与引用: move/Copy/clone/借用/切片 |
| 15 | `compound_types.rs` | 复合类型: 元组/数组/字符串 |
| 16 | `vec_type.rs` | 向量 Vec |
| 17 | `fizzbuzz.rs` | 综合练习: FizzBuzz |
| 18 | `score_analyzer.rs` | 综合练习: 成绩分析器 |

---

## intermediate/ — 中级概念

| # | 文件 | 内容 |
|---|---|---|
| 1 | `structs_and_enums.rs` | 结构体(3种) + 枚举 + impl 方法 |
| 2 | `patterns.rs` | 模式匹配深入: 解构/嵌套/@绑定/守卫 |
| 3 | `option.rs` | Option 专题: match/if let/map/and_then |
| 4 | `error_handling.rs` | 错误处理: Result/panic/`?` 运算符 |
| 5 | `lifetimes.rs` | 生命周期: 标注/省略规则/struct 引用/'static |
| 6 | `generics.rs` | 泛型: 函数/结构体/方法/Trait Bound |
| 7 | `traits.rs` | 特型: 定义/实现/默认方法/Derive 宏 |
| 8 | `vec_advanced.rs` | Vec 高级: sort/dedup/retain/windows/chunks |
| 9 | `collections.rs` | 集合: HashMap/Entry API/HashSet/BTreeMap |

---

## intermediate_example/ — 综合实践 (图书管理系统)

综合运用 basic + intermediate 全部知识点。

详见 [intermediate_example/README.md](intermediate_example/README.md)

---

## advanced/ — 高级概念

概念代码演示, 每个模块覆盖一个高级主题:

| # | 文件 | 内容 |
|---|---|---|
| 1 | `smart_pointers.rs` | Box/Deref/Drop/Rc/Arc |
| 2 | `interior_mutability.rs` | Cell/RefCell/Rc\<RefCell\> |
| 3 | `unsafe_rust.rs` | 裸指针/unsafe块/FFI 概念 |
| 4 | `macros.rs` | macro_rules! 声明宏 |
| 5 | `concurrency.rs` | thread/mpsc/Mutex/Arc |
| 6 | `async_intro.rs` | async/await/Future trait |
| 7 | `io_advanced.rs` | Read/Write trait/BufReader/Path |
| 8 | `networking.rs` | TcpListener/TcpStream/UDP/HTTP |
| 9 | `database.rs` | 文件 CRUD 持久化 |

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
cd intermediate && cargo run && cargo run -- error_handling
cd advanced && cargo run && cargo run -- smart_pointers

# 综合实践
cd intermediate_example && cargo run

# 项目实战
cd advanced_data_pipeline && cargo run -- help
cd advanced_data_flow && cargo run -- help
```

