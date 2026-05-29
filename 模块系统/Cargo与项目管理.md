---
title: "Rust Cargo 与项目管理"
type: knowledge
category: 模块系统
tags:
    - rust
    - cargo
    - testing
    - dependencies
    - publishing
    - workspace
related:
    - 模块系统
    - 特型与泛型
---

# Cargo 与项目管理

Rust 使用 **crate** 作为编译和分发的独立单元，Cargo 负责构建、依赖、测试和发布的全流程管理。

> 代码组织（模块定义、可见性、路径导入、属性）请参见：[模块系统](模块系统.md)。

---

## 一、crate

每个 Rust 程序都是一个 crate。crate 是编译的独立单元，包含库或可执行程序的所有源代码、测试、示例等。

### 1.1 crate 类型

- **库 crate**（`--crate-type lib`）：生成 `.rlib` 文件，不包含 `main()` 函数
- **二进制 crate**（`--crate-type bin`）：生成可执行文件，包含 `main()` 函数

```toml
# Cargo.toml
[dependencies]
num = "0.4"
image = "0.13"
crossbeam = "0.8"
```

Cargo 自动解析**传递依赖**，构建完整的**依赖图**。`.rlib` 文件包含已编译代码、类型信息、公共内联函数/泛型/宏的副本。

### 1.2 Edition（版本）

Rust 使用版本机制在不破坏现有代码的情况下演进语言：

```toml
[package]
edition = "2021"  # 2015 / 2018 / 2021
```

- 每个 crate 可以独立选择版本
- 不同版本的 crate 可以自由混用
- `cargo fix` 可自动升级旧代码到新版本

### 1.3 构建配置

```toml
[profile.dev]       # cargo build（调试版）
opt-level = 0

[profile.release]   # cargo build --release（发布版）
opt-level = 3
debug = true        # 可在发布版中启用调试符号

[profile.test]      # cargo test
```

---

## 二、测试与文档

### 2.1 单元测试

```rust
#[test]
fn math_works() {
    assert!(1.is_positive());
    assert_eq!(1 + 1, 2);
}

// 测试 panic
#[test]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    1 / 0;
}

// 返回 Result 的测试
#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}
```

**测试模块惯例：**

```rust
#[cfg(test)]  // 只在测试时编译
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        assert!(roughly_equal(std::f64::consts::PI.sin(), 0.0));
    }
}
```

### 2.2 集成测试

放在项目根目录的 `tests/` 目录中，作为独立的 crate 与你的库链接：

```rust
// tests/unfurl.rs
use fern_sim::Terrarium;

#[test]
fn test_fiddlehead_unfurling() {
    let mut world = Terrarium::load("tests/unfurl_files/fiddlehead.tm");
    assert!(world.fern(0).is_furled());
    world.apply_sunlight(Duration::from_secs(3600));
    assert!(world.fern(0).is_fully_unfurled());
}
```

运行：`cargo test --test unfurl`

### 2.3 文档型注释

```rust
/// 模拟减数分裂产生孢子
///
/// 使用示例：
/// ```
/// let spore = fern_sim::produce_spore(&mut factory);
/// ```
pub fn produce_spore(factory: &mut Sporangium) -> Spore { ... }

//! 模块/crate 级文档注释
//! 模拟蕨类植物从单个细胞开始的生长过程
```

- `///` 文档型注释 → Markdown 格式，支持 Rust 路径作为链接
- `//!` 模块/crate 级文档
- `#[doc(alias = "route")]` 添加搜索别名
- `#![doc = include_str!("../README.md")]` 包含外部文件

**隐藏测试代码行：**

```rust
///     # use fern_sim::Terrarium;  // # 开头的行在文档中隐藏但参与测试
///     # let mut tm = Terrarium::new();
///     tm.apply_sunlight(Duration::from_secs(60));
```

**代码块注解：**
- `` ```no_run `` → 编译但不运行
- `` ```ignore `` → 不编译
- `` ```text `` 或其他语言名 → 不当作 Rust 代码

### 2.4 文档测试

`cargo test` 自动将文档中的代码块编译为独立测试并运行。每个 `fn main` 包含在内的代码块被视为完整程序，不添加包装代码。

---

## 三、依赖项

### 3.1 指定来源

```toml
[dependencies]
# 从 crates.io（版本号）
image = "0.13.0"

# 从 Git 仓库
image = { git = "https://github.com/.../image.git", rev = "528f19c" }
# 可选：rev（提交）、tag（标签）、branch（分支）

# 从本地路径
image = { path = "vendor/image" }

# 本地路径优先 + 发布版本备用
image = { path = "vendor/image", version = "0.13.0" }
```

### 3.2 版本兼容规则

| 版本格式 | 兼容范围 |
| :--- | :--- |
| `0.0.x` | 不假定兼容 |
| `0.x.y`（x ≠ 0） | 兼容 0.x 系列 |
| `>=1.0` | 兼容相同主版本号 |

**精确控制：**

| 写法 | 含义 |
| :--- | :--- |
| `"=0.10.0"` | 仅 0.10.0 |
| `">=1.0.5"` | 1.0.5 及以上 |
| `">1.0.5 <1.1.9"` | 指定范围 |
| `"<=2.7.10"` | 2.7.10 及以下 |
| `"*"` | 任何版本 |

### 3.3 Cargo.lock

- 首次构建时自动生成，记录确切的依赖版本
- 后续构建使用锁定的版本，保证可重现性
- `cargo update` 升级到兼容的最新版本
- **可执行文件**项目：应提交到版本控制
- **库**项目：不必提交（下游用户的 Cargo.lock 会覆盖）

---

## 四、发布到 crates.io

```bash
cargo package              # 打包为 .crate 文件
cargo login <API_KEY>      # 登录（密钥从 crates.io 账户设置获取）
cargo publish              # 发布
```

Cargo.toml 发布所需字段：

```toml
[package]
name = "fern_sim"
version = "0.1.0"
edition = "2021"
authors = ["You <you@example.com>"]
license = "MIT"
homepage = "https://fernsim.example.com/"
repository = "https://gitlair.com/sporeador/fern_sim"
documentation = "http://fernsim.example.com/docs"
description = "Fern simulation, from the cellular level up."
```

> 发布到 crates.io 的 crate 的依赖项也应该在 crates.io 上，不能依赖 `path` 指定的本地路径。

---

## 五、工作空间

多个 crate 共享构建目录和 Cargo.lock：

```
fernsoft/
├── Cargo.toml          # [workspace] members = ["fern_sim", "fern_img", "fern_video"]
├── Cargo.lock
├── target/             # 共享构建目录
├── fern_sim/
│   ├── Cargo.toml
│   └── src/...
├── fern_img/
│   ├── Cargo.toml
│   └── src/...
└── fern_video/
    ├── Cargo.toml
    └── src/...
```

```toml
[workspace]
members = ["fern_sim", "fern_img", "fern_video"]
```

- `cargo build --workspace` → 构建所有成员
- `cargo test --workspace` → 测试所有成员
- 删除各子目录中的 Cargo.lock 和 target 目录

---

## 相关链接

- [模块系统](模块系统.md) – 模块定义、可见性、路径导入、属性
