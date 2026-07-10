# Rust 学习项目

本项目是一个 Rust 学习仓库，按模块拆分，从基础概念到项目逐步深入。

## 项目结构

```
RustLearning/
├── basic/           ← Rust 基础（注释即文档）
├── ...              ← 后续学习项目（每个都是一个独立 crate）
└── README.md
```

### basic/ 模块列表

| 文件 | 内容 |
|---|---|
| `main.rs` | 程序入口: pub/fn/块、println! 格式化、let/语句/表达式/分号、print! vs println! |
| `base_type.rs` | 基础类型: 整数/浮点/bool/char、整数溢出处理、数字字面量(进制)、类型转换(as/try_into) |
| `bit_ops.rs` | 位运算: & / \| / ^ / ! / << / >> |
| `if_flow.rs` | 条件判断: if / else if / else、if 作为表达式、条件必须为 bool |
| `match_flow.rs` | 模式匹配: match 表达式、穷尽检查、`_` / `=>`、`\|` 或模式、`..=` 范围、守卫 if |
| `return_flow.rs` | 函数返回值: 隐式返回 vs 显式 return、`->` / `()` 说明、卫语句、mut 嵌套函数 |
| `ownership_and_refs.rs` | 所有权与引用: move/Copy/clone、深拷贝浅拷贝、`&` / `&mut` 借用规则、切片 slice |
| `compound_types.rs` | 复合类型: 元组/数组/char-&str-String 三者关系、String 方法调用、mut 与复合类型 |
| `vec_type.rs` | 向量 Vec: 创建/push/pop、安全访问 get、len/capacity 扩容、所有权、遍历 |

### 推荐学习顺序

按下方顺序阅读, 后面的模块会用到前面的概念:

```
1. main.rs                   → 先了解 Rust 代码长什么样
2. base_type.rs              → 基础类型, 一切数据的基础
3. bit_ops.rs                → 位运算, 和整数密切相关
4. if_flow.rs                → 条件判断, 最简单的控制流
5. match_flow.rs             → 模式匹配, 更强大的控制流
6. return_flow.rs            → 函数怎么返回值
7. ownership_and_refs.rs     → ⭐ 核心! 所有权/引用/切片
8. compound_types.rs         → 元组/数组/字符串 (依赖所有权)
9. vec_type.rs               → 向量 (依赖所有权和复合类型)
```

`ownership_and_refs.rs` 是整个 Rust 的核心, 建议多看几遍.
后面的 `compound_types.rs` 和 `vec_type.rs` 大量用到所有权概念.

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
  自动释放堆内存. 如果分不清哪些数据在堆上哪些在栈上, 就很难理解为什么
  i32 赋值后原变量还能用, 而 String 赋值后原变量就失效了.

> 如果对堆和栈还不太熟悉, 可以先搜索"编程 堆和栈的区别"补充一下.
> 这是计算机基础概念, 对学习任何系统级语言都有帮助.

## 使用方法

### 运行时学习（推荐）

```bash
cd basic
cargo run      # 终端逐模块输出教学说明和代码演示
```

### 生成文档

```bash
cd basic
cargo doc --open       # 生成 HTML 文档并在浏览器打开
```

每个模块的 `//!` 和 `///` 注释会被 `rustdoc` 渲染成结构化说明页面。

## 内容来源

主要参考 **Rust 官方设计手册**及标准库文档，结合实践整理。
