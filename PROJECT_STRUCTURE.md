# 项目结构详解 (Project Structure)

本文档系统讲解 Rust 项目的标准结构和本教程的教学结构设计。

---

## 1. 标准 Rust 项目结构

一个典型的 Rust Cargo 项目（Package）包含以下目录和文件：

```text
my_project/                  # 项目根目录（Package）
├── Cargo.toml               # 项目清单文件（必需）
├── Cargo.lock               # 依赖锁定文件（自动生成，不要手动编辑）
├── rust-toolchain.toml      # 工具链声明（可选）
├── .gitignore               # Git 忽略规则
├── src/                     # 源代码目录
│   ├── main.rs              # 二进制 Crate 入口（Binary Crate）
│   ├── lib.rs               # 库 Crate 入口（Library Crate）
│   ├── bin/                 # 额外的二进制 Crate
│   │   └── tool.rs          # 另一个可执行程序
│   ├── models.rs            # 模块文件
│   └── services/            # 模块目录
│       ├── mod.rs           # 模块入口（或同名文件）
│       └── parser.rs        # 子模块
├── tests/                   # 集成测试
│   └── integration_test.rs  # 每个文件是一个独立的测试 Crate
├── examples/                # 示例程序
│   └── demo.rs              # 每个文件是一个独立的示例 Crate
├── benches/                 # 基准测试（需要 nightly）
│   └── benchmark.rs
└── target/                  # 构建产物目录（由 Cargo 管理）
    ├── debug/               # Debug 构建产物
    └── release/             # Release 构建产物
```

---

## 2. 核心文件详解

### 2.1 `Cargo.toml`

Cargo.toml 是项目的清单文件（Manifest），使用 TOML 格式。它告诉 Cargo：

- 项目的名称、版本和 Edition
- 依赖哪些外部 Crate
- 编译配置（Profile）
- 可选功能（Feature）

**最小示例（二进制项目）**:
```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"
```

**最小示例（库项目）**:
```toml
[package]
name = "my_lib"
version = "0.1.0"
edition = "2024"

[lib]
name = "my_lib"
path = "src/lib.rs"
```

**带依赖的示例**:
```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 2.2 `Cargo.lock`

- **自动生成**: 由 Cargo 在第一次 `cargo build` 时创建
- **精确版本**: 锁定所有依赖的精确版本，保证可重现构建
- **库 vs 应用**: 
  - 对于二进制项目（应用），通常提交 `Cargo.lock` 到版本控制
  - 对于库项目，通常不提交 `Cargo.lock`（让使用者自行解析版本）
  - 本教程每章是独立应用，所以 `Cargo.lock` 应提交
- **不要手动编辑**: `Cargo.lock` 由 Cargo 管理，手动修改会导致问题

### 2.3 `src/main.rs`

二进制 Crate 的入口文件。必须包含 `fn main()` 函数：

```rust
fn main() {
    println!("Hello, world!");
}
```

一个 Package 可以有多个二进制 Crate：
- `src/main.rs` → 默认二进制（与 Package 同名）
- `src/bin/foo.rs` → 名为 `foo` 的二进制
- `src/bin/bar/main.rs` → 名为 `bar` 的二进制（在子目录中）

### 2.4 `src/lib.rs`

库 Crate 的入口文件。定义库的公开 API：

```rust
//! 这是库的文档注释（以 //! 开头）
//! 描述整个库的用途。

/// 这是公开函数的文档注释（以 /// 开头）
pub fn public_function() -> String {
    String::from("Hello from the library!")
}

// 私有函数（没有 pub）
fn internal_helper() {
    // 只在库内部可用
}
```

一个 Package 最多可以有一个库 Crate。

### 2.5 `tests/` 目录

存放集成测试（Integration Test）。`tests/` 中的每个 `.rs` 文件都是一个独立的测试 Crate：

```rust
// tests/integration_test.rs
use my_lib;  // 像外部使用者一样导入库

#[test]
fn test_public_api() {
    assert_eq!(my_lib::public_function(), "Hello from the library!");
}
```

集成测试不能测试私有函数（只能通过公开 API 测试）。

### 2.6 `examples/` 目录

存放示例程序。每个 `.rs` 文件编译为独立的可执行文件：

```rust
// examples/demo.rs
fn main() {
    println!("This is an example!");
}
```

运行示例：
```bash
cargo run --example demo
```

### 2.7 `target/` 目录

Cargo 的构建输出目录：

```text
target/
├── debug/               # Debug 构建（cargo build / cargo run）
│   ├── my_app           # 可执行文件
│   ├── libmy_lib.rlib   # 库文件
│   ├── deps/            # 依赖的编译产物
│   ├── examples/        # 示例的编译产物
│   └── incremental/     # 增量编译缓存
└── release/             # Release 构建（cargo build --release）
    ├── my_app           # 优化后的可执行文件
    └── ...
```

- 可以用 `cargo clean` 清空整个 `target/`
- 一般将 `target/` 加入 `.gitignore`

---

## 3. Package、Crate、Module 的关系

这是初学者最容易混淆的概念层次：

```
Package (包)
├── 一个 Cargo.toml
├── 可以包含一个或多个 Crate
│
├── Library Crate (库箱) ─ 最多一个
│   └── 由 src/lib.rs 定义
│   └── Module Tree
│       ├── 公开模块 (pub mod)
│       └── 私有模块 (mod)
│
└── Binary Crate (二进制箱) ─ 可以多个
    ├── src/main.rs (默认)
    └── src/bin/*.rs (额外)
```

**记忆方法**:
- **Package** = 一个 Cargo 项目（对应一个 Cargo.toml）
- **Crate** = Rustc 的编译单元（编译器一次处理一个 Crate）
- **Module** = 代码组织单元（在 Crate 内部）

---

## 4. Workspace（工作空间）

多个相关的 Package 可以放在一个 Workspace 中：

```text
demo_workspace/                     # Workspace 根目录
├── Cargo.toml                      # [workspace] 声明（无 [package]）
├── Cargo.lock                      # 所有成员共享
├── crates/
│   ├── core_lib/                   # Package 1: 库
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── cli_app/                    # Package 2: 二进制
│       ├── Cargo.toml
│       └── src/main.rs
└── target/                         # 所有成员共享
```

Workspace 的好处：
- 共享同一个 `Cargo.lock`（一致的依赖版本）
- 共享同一个 `target/`（节省磁盘空间和编译时间）
- 可以按路径引用成员（`core_lib = { path = "../core_lib" }`）
- 一条命令操作所有成员（`cargo test --workspace`）

---

## 5. 本教程的结构设计

### 为什么每章是独立的 Cargo Package？

本教程采用"每章一个独立 Package + 根级 Virtual Workspace"的结构：

```text
rust-from-python/
├── Cargo.toml                      # [workspace]，无 [package]
├── rust-toolchain.toml
├── chapters/
│   ├── 00_course_orientation/      # 独立 Package
│   ├── 01_hello_cargo/             # 独立 Package
│   └── ...
├── projects/
│   ├── 01_guessing_game/           # 独立 Package
│   └── ...
└── scripts/
```

**设计理由**:

1. **独立可运行**: 每章可以单独 `cd` 进入运行，不依赖其他章节
2. **清晰的依赖边界**: 章节之间不互相依赖，强调每章知识的独立性
3. **统一验证**: 根级 Workspace 可以一次性对所有章节执行 `cargo check`、`cargo test`
4. **真实工程感**: 每章看起来像一个真实的 Rust 项目
5. **增量学习**: 学习者在任何一章停下来都能得到一个完整的、可运行的程序

### 这种设计的代价

- 每章都有独立的 `Cargo.toml`，有重复信息
- 没有跨章共享代码（这其实是设计选择：教学不应产生链式依赖）
- `target/` 会有很多编译产物（可通过根级 Workspace 共享 `target/` 缓解）

---

## 6. 本章总结

| 组件 | 作用 | 类比 |
|------|------|------|
| Package | 一个 Cargo 项目 | Python 的一个包项目 |
| Crate | 编译单元 | Python 的一个 `.py` 文件（不准确但接近） |
| Module | 代码组织 | Python 的一个模块 |
| Workspace | 多个 Package 的集合 | Python 的 monorepo |
| Cargo.toml | 项目清单 | `pyproject.toml` |
| Cargo.lock | 依赖锁定 | `requirements.lock` |
| src/main.rs | 二进制入口 | `__main__.py` |
| src/lib.rs | 库入口 | `__init__.py` |
| tests/ | 集成测试 | `tests/` 目录 |
