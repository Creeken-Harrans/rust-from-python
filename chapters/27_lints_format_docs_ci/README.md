# 代码质量: Lint、格式化、文档与 CI

## 目录

1. [概述](#概述)
2. [cargo fmt: 代码格式化](#cargo-fmt-代码格式化)
3. [cargo clippy: 静态分析](#cargo-clippy-静态分析)
4. [cargo check: 快速类型检查](#cargo-check-快速类型检查)
5. [cargo test: 测试框架](#cargo-test-测试框架)
6. [cargo doc: 文档生成](#cargo-doc-文档生成)
7. [Lint 级别与控制](#lint-级别与控制)
8. [CI 持续集成](#ci-持续集成)
9. [CI Pipeline 设计](#ci-pipeline-设计)
10. [GitHub Actions 实战](#github-actions-实战)
11. [为什么这些工具不是可选的](#为什么这些工具不是可选的)
12. [最佳实践](#最佳实践)
13. [常见问题](#常见问题)
14. [总结](#总结)

## 概述

软件质量不是偶然产生的，而是通过工程化手段系统性地保证的。Rust 生态系统提供了业界最完善的内置代码质量工具链：

| 工具 | 对应 Python 工具 | 功能 |
|------|-----------------|------|
| `cargo fmt` | black / ruff format | 自动代码格式化 |
| `cargo clippy` | pylint / ruff | 静态分析和 lint |
| `cargo check` | mypy | 快速类型检查（不生成二进制） |
| `cargo test` | pytest | 单元测试 + 集成测试 + 文档测试 |
| `cargo doc` | sphinx / pdoc | 从注释生成 HTML 文档 |

这些工具与 CI（持续集成）系统集成后，可以确保：
- 每次 Pull Request 自动运行完整的质量检查
- 代码风格 100% 一致，无需人工争论格式
- 潜在的 bug 在合并前被捕获
- 文档始终与代码保持同步

## cargo fmt: 代码格式化

### 是什么

`cargo fmt` 是 Rust 官方代码格式化工具，基于 `rustfmt`。它将 Rust 源代码自动重写为符合社区统一风格的格式。

### 基本用法

```bash
# 格式化当前项目的所有 Rust 文件
cargo fmt

# 仅检查格式是否正确（不修改文件，CI 中使用）
cargo fmt -- --check

# 格式化特定文件
cargo fmt -- src/main.rs

# 格式化 workspace 中的所有 crate
cargo fmt --all

# 显示将要修改的内容（dry-run）
cargo fmt -- --check
```

### 配置 rustfmt

在项目根目录创建 `rustfmt.toml`：

```toml
# rustfmt.toml
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2024"

# 控制 import 合并行为
imports_granularity = "Crate"

# 控制 match 分支的格式
match_block_trailing_comma = true

# 控制函数调用的格式
use_small_heuristics = "Max"

# 控制注释格式
wrap_comments = true
comment_width = 100
```

### 为什么使用统一格式

1. **消除风格争论**: Code Review 中不再讨论 "大括号应该放在哪里"
2. **降低认知负担**: 阅读任何 Rust 代码时，格式都是熟悉的
3. **减少 diff 噪音**: 格式变更不会混入逻辑变更中
4. **新人友好**: 新加入的开发者不需要学习特定项目的格式惯例

### 在 CI 中集成

```yaml
- name: Check formatting
  run: cargo fmt --all -- --check
```

如果任何文件格式不符合标准，该命令返回非零退出码，CI 失败。

## cargo clippy: 静态分析

### 是什么

`cargo clippy` 是 Rust 的官方 linter，包含超过 550 条 lint 规则。它不仅能捕获常见的错误模式，还能引导开发者写出更地道、更高效的 Rust 代码。

### 基本用法

```bash
# 运行 clippy
cargo clippy

# 将所有警告视为错误（CI 推荐）
cargo clippy -- -D warnings

# 针对 workspace 中的所有 crate 和所有目标类型
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 自动应用 clippy 的修复建议
cargo clippy --fix

# 运行特定 lint 组
cargo clippy -- -W clippy::pedantic
cargo clippy -- -W clippy::perf
```

### Clippy Lint 分类

| Lint 组 | 说明 | 默认状态 |
|---------|------|---------|
| `clippy::correctness` | 可能是 bug 的代码 | deny |
| `clippy::style` | 风格问题 | warn |
| `clippy::complexity` | 过于复杂的代码 | warn |
| `clippy::perf` | 性能问题 | warn |
| `clippy::pedantic` | 严格的惯用法检查 | allow |
| `clippy::nursery` | 实验性 lint | allow |
| `clippy::cargo` | Cargo.toml 相关 | warn |
| `clippy::restriction` | 限制性 lint | allow |
| `clippy::suspicious` | 可疑的代码模式 | warn |

### Clippy 常见建议示例

```rust
// Clippy 会警告:
let x = if condition { true } else { false };
// 建议改为:
let x = condition;

// Clippy 会警告:
fn foo(x: &String) { }
// 建议改为:
fn foo(x: &str) { }

// Clippy 会警告:
let x: Vec<i32> = vec![];
// 建议改为:
let x: Vec<i32> = Vec::new();

// Clippy 会警告:
match x {
    Some(v) => { /* complex body */ },
    None => {},
}
// 建议改为:
if let Some(v) = x {
    /* complex body */
}
```

### 在 CI 中使用 Clippy 的推荐配置

```yaml
- name: Clippy
  run: cargo clippy --workspace --all-targets --all-features -- -D warnings
```

- `--workspace`: 检查所有 crate
- `--all-targets`: 检查 lib, bin, test, example, bench
- `--all-features`: 启用所有 features 以确保所有代码路径被检查
- `-D warnings`: 将所有 warning 级别 lint 提升为 deny（编译失败）

这种严格配置确保没有任何 lint 警告能进入 main 分支。

## cargo check: 快速类型检查

### 是什么

`cargo check` 对代码进行类型检查，但跳过 LLVM 代码生成阶段。它比 `cargo build` 快得多（通常 2-5 倍），非常适合开发过程中的快速验证。

### 基本用法

```bash
# 快速类型检查
cargo check

# 检查 workspace 所有目标
cargo check --workspace --all-targets

# 检查特定 crate
cargo check -p my-crate

# 在开发中自动运行（配合 cargo-watch）
cargo watch -x check
```

### check vs build vs test

| 命令 | 类型检查 | 代码生成 | 运行测试 | 速度 |
|------|---------|---------|---------|------|
| `cargo check` | Yes | No | No | 最快 |
| `cargo build` | Yes | Yes | No | 中等 |
| `cargo test` | Yes | Yes | Yes | 最慢 |

在 CI 中，通常先运行 `cargo check` 而不是 `cargo build`：
- 类型检查是编译错误的主要来源
- `check` 远快于 `build`
- 早期失败，节省 CI 时间

```yaml
- name: Type check
  run: cargo check --workspace --all-targets
```

## cargo test: 测试框架

### 是什么

Rust 内置了完整的测试框架，支持：
- 单元测试（`#[test]` 和 `#[cfg(test)]` 模块）
- 集成测试（`tests/` 目录）
- 文档测试（doc-tests，`/// ``` ... /// ``` ` 中的代码）
- 基准测试（需要 nightly 或 criterion crate）

### 基本用法

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_function_name

# 运行特定 crate 的测试
cargo test -p my-crate

# 运行 workspace 全部测试
cargo test --workspace

# 显示测试输出（release 模式）
cargo test --release

# 运行文档测试
cargo test --doc

# 运行包含特定关键词的测试
cargo test factorial
```

### 测试在 CI 中的作用

```yaml
- name: Run tests
  run: cargo test --workspace
```

CI 中的测试确保：
- 新的更改没有破坏现有功能
- 边界情况被正确处理
- 文档中的代码示例依然正确

## cargo doc: 文档生成

### 是什么

`cargo doc` 从代码中的文档注释（`///` 和 `//!`）自动生成 HTML 文档。它使用 `rustdoc` 工具，将 Markdown 格式的注释转换为精美的文档页面。

### 基本用法

```bash
# 生成当前 crate 的文档
cargo doc

# 生成文档但不包含依赖的文档
cargo doc --no-deps

# 在浏览器中打开生成的文档
cargo doc --open

# 为 workspace 生成文档
cargo doc --workspace --no-deps

# 在 CI 中将文档中的警告视为错误
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

### 文档注释语法

```rust
/// 这是对下一项（函数、结构体等）的文档注释。
/// 
/// # 示例
///
/// ```
/// let x = 42;
/// assert_eq!(x, 42);
/// ```
///
/// # Panics
/// 不会 panic。
///
/// # 安全性
/// 不需要 unsafe。
pub fn my_function() { }

//! 这是模块级别的文档注释。
//! 通常放在 lib.rs 或 mod.rs 的开头。

/// 文档注释支持 Markdown:
/// - 列表项 1
/// - 列表项 2
/// 
/// **粗体** 和 *斜体*。
/// 
/// [链接](https://www.rust-lang.org)
/// 
/// `行内代码`
```

### 文档测试 (Doc-tests)

文档注释中的代码块会自动作为测试运行：

```rust
/// 计算两个数的和。
///
/// # 示例
///
/// ```
/// use my_crate::add;  // 隐含的 main 函数
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

当运行 `cargo test` 时，上面的文档示例会被编译和执行。如果 `add(2, 3)` 不等于 5，测试会失败。

文档测试的价值：
- 确保文档中的代码是正确的
- 作为 API 的使用示例
- 防止文档过时

## Lint 级别与控制

### 四个 Lint 级别

Rust 编译器支持四个 lint 级别：

| 级别 | 关键字 | 行为 |
|------|--------|------|
| allow | `#[allow(...)]` | 抑制 lint，不报告 |
| warn | `#[warn(...)]` | 发出警告，但不阻止编译 |
| deny | `#[deny(...)]` | 发出错误，阻止编译 |
| forbid | `#[forbid(...)]` | 与 deny 相同，但不能被下游覆盖 |

### 在代码中控制 Lint

#### 属性语法

```rust
// 应用于当前模块或项
#[allow(dead_code)]
fn unused_function() {
    // 这个函数即使未被调用也不会触发 dead_code 警告
}

// 应用于整个 crate（放在 lib.rs 或 main.rs 顶部）
#![deny(missing_docs)]  // 要求所有公开项必须有文档
#![allow(clippy::too_many_arguments)]  // 允许超过 7 个参数的函数
```

#### 细粒度的 Lint 控制

```rust
// 仅对特定行抑制 warning
#[allow(clippy::needless_range_loop)]
for i in 0..vec.len() {
    // 手动索引循环，但你知道自己在做什么
    println!("{}", vec[i]);
}

// 为整个 impl 块应用
#[allow(clippy::redundant_clone)]
impl MyStruct {
    fn new() -> Self {
        // 这里可能有意地使用 clone
    }
}
```

### Clippy 配置 (clippy.toml)

```toml
# clippy.toml
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 10
max-trait-bounds = 5
```

### 常见的 Lint 属性

```rust
#![deny(missing_docs)]                    // 强制文档
#![deny(unsafe_code)]                     // 禁止 unsafe 代码
#![deny(clippy::unwrap_used)]            // 禁止 unwrap
#![deny(clippy::expect_used)]            // 禁止 expect
#![warn(clippy::pedantic)]               // 启用所有严格的 lint
#![deny(clippy::all)]                    // 拒绝所有 clippy lint
#![allow(clippy::module_name_repetitions)] // 允许模块名重复
```

## CI 持续集成

### 什么是 CI

持续集成（Continuous Integration，CI）是一种软件开发实践，开发者频繁地将代码变更合并到主干分支，每次合并都通过自动化构建和测试来验证。

### CI 的价值

1. **早期发现问题**: 代码合并前自动运行检查
2. **质量门槛**: 所有检查通过才能合并
3. **一致性**: 团队成员遵循相同的质量标准
4. **自动化**: 减少人工 Code Review 的机械性检查

### Rust CI 的典型流程

```
代码提交 → PR 创建 → CI 触发
                      ↓
                 1. cargo fmt --check
                      ↓ (成功)
                 2. cargo check
                      ↓ (成功)
                 3. cargo test
                      ↓ (成功)
                 4. cargo clippy (严格模式)
                      ↓ (成功)
                 5. cargo doc
                      ↓ (成功)
                 合并到 main ✓
```

## CI Pipeline 设计

### 推荐的 CI Steps

#### 1. 格式检查
```bash
cargo fmt --all -- --check
```
目的：确保所有代码符合统一格式。
为什么放第一步：最快，且格式问题是纯机械性的。

#### 2. 类型检查
```bash
cargo check --workspace --all-targets
```
目的：快速验证代码能否通过编译。
为什么：比 `build` 快 2-5 倍，提前发现大多数编译错误。

#### 3. 测试
```bash
cargo test --workspace
```
目的：验证代码行为正确性，包括文档测试。
注意：这是最耗时的步骤，但在 CI 中不可省略。

#### 4. Lint 检查
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
目的：捕获代码异味和潜在 bug。
关键参数解释：
- `--workspace`: 检查所有 crate
- `--all-targets`: 包括 lib, bin, test, example, bench
- `--all-features`: 启用所有 feature 以确保完整覆盖
- `-D warnings`: 将所有 Clippy 警告视为编译错误

#### 5. 文档构建
```bash
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
目的：确保文档可以正确生成且文档中的代码示例可编译。
`RUSTDOCFLAGS="-D warnings"` 确保文档中的任何警告（如失效的链接）都会导致失败。

### Pipeline 优化技巧

1. **并行化**: CI 服务支持并行 job，可以将 check/test/clippy 放在不同 job 中
2. **缓存**: 缓存 `target/` 目录和 `~/.cargo/registry/`
3. **增量构建**: 在 CI 中也可启用（但跨 run 效果有限）
4. **sccache**: 使用共享编译缓存工具加速

## GitHub Actions 实战

### 配置文件

本项目包含了 `.github/workflows/rust.yml`，这是一个完整的 CI 工作流配置：

```yaml
name: Rust CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    name: Check, Test, Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - name: Check formatting
        run: cargo fmt --all -- --check
      - name: Type check
        run: cargo check --workspace --all-targets
      - name: Run tests
        run: cargo test --workspace
      - name: Clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - name: Build docs
        run: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

### 配置说明

| 配置项 | 说明 |
|--------|------|
| `on: push/pull_request` | 触发条件 |
| `runs-on: ubuntu-latest` | 运行环境 |
| `actions/checkout@v4` | 检出代码 |
| `dtolnay/rust-toolchain@stable` | 安装 Rust（推荐这个 action） |
| `components: rustfmt, clippy` | 安装额外工具组件 |

### 扩展 CI（可选）

更高级的 CI 配置可以包含：

```yaml
jobs:
  # ... 基础检查 ...

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v2
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      - name: Upload to codecov
        uses: codecov/codecov-action@v4
```

## 为什么这些工具不是可选的

### cargo fmt 不是可选的

- **消除主观性**: 代码风格不应该由个人偏好决定
- **提高 Review 效率**: Reviewer 不需要在格式上花费精力
- **降低 diff 噪声**: 格式变更和逻辑变更分离
- **社区标准**: Rust 社区高度统一使用 rustfmt

### cargo clippy 不是可选的

- **预防 bug**: Clippy 能捕获许多常见的错误模式（如错误的比较、无效的类型转换）
- **学习工具**: 对 Rust 新手来说，Clippy 是一个优秀的学习资源，通过它的建议可以学到更地道的写法
- **代码质量基准**: "零 Clippy 警告"是一个可衡量的质量标准
- **性能**: 某些 Clippy lint 直接关系到运行时性能

### cargo test 不是可选的

- **正确性保证**: 代码必须经过测试验证
- **重构安全网**: 测试让你有信心进行重构
- **文档正确性**: Doc-tests 确保文档示例始终有效
- **回归防护**: 防止修复一个 bug 时引入另一个

### cargo doc 不是可选的

- **可用性**: 没有文档的 API 几乎不能用
- **免费生成**: `cargo doc` 从已有注释自动生成，零额外工作量
- **同步保证**: 文档和代码放在一起，减少文档过时问题
- **doc-tests**: 文档中的代码自动成为测试

## 最佳实践

### 开发工作流

1. **编写代码时**: 让 `cargo check` 在编辑器中运行（通过 rust-analyzer）
2. **保存文件时**: 设置编辑器自动运行 `cargo fmt`
3. **提交前**: 手动运行 `cargo clippy` 和 `cargo test`
4. **PR 时**: CI 自动运行完整检查

### Editor 配置

VS Code 推荐配置（`.vscode/settings.json`）：

```json
{
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.extraArgs": ["--", "-D", "warnings"]
}
```

### 项目级 Lint 配置

在 `lib.rs` 或 `main.rs` 顶部添加项目级的 lint 指令：

```rust
// 适用于生产项目的推荐配置
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
```

### git hooks

使用 pre-commit hook 在本地运行检查：

```bash
#!/bin/bash
# .git/hooks/pre-commit
cargo fmt --all -- --check || exit 1
cargo clippy -- -D warnings || exit 1
cargo test || exit 1
```

## 常见问题

### Q: Clippy 太严格了，能不能放松一些？

可以。在 `Cargo.toml` 中或在代码中使用 `#[allow(...)]` 属性来放松特定的 lint：

```toml
# Cargo.toml
[lints.clippy]
too_many_arguments = "allow"
module_name_repetitions = "allow"
type_complexity = "allow"
```

当使用 `#[allow(...)]` 时，最好添加注释说明原因：
```rust
#[allow(clippy::too_many_arguments)]
// 该函数的参数都有明确的语义含义，
// 将其合并为结构体会降低调用方代码的可读性
fn complex_setup(a: i32, b: i32, c: i32, d: i32, e: i32) { }
```

### Q: 我不想让 cargo fmt 格式化某些代码块

使用 `#[rustfmt::skip]` 属性：

```rust
#[rustfmt::skip]
let matrix = [
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
];
```

### Q: CI 太慢了怎么办？

1. 使用 `cargo check` 而不是 `cargo build`
2. 缓存 `target/` 和 `~/.cargo/`
3. 使用 sccache 共享编译缓存
4. 将不同步骤拆分为独立 job 并行运行
5. 只在必要时运行完整 workspace 检查（使用 `-p` 限定范围）

### Q: 文档测试怎么写？

在文档注释中使用代码块：

```rust
/// 这是示例文档测试。
///
/// ```
/// let x = 42;
/// assert_eq!(x, 42);  // 这会作为测试运行
/// ```
```

注意事项：
- 代码块默认会被编译和执行
- 添加 `no_run` 使代码编译但不运行
- 添加 `ignore` 跳过编译
- 添加 `should_panic` 期望代码 panic

## 总结

Rust 的代码质量工具链是语言设计的重要部分，而不仅仅是附加工具：

- **cargo fmt**: 统一的代码格式，消除风格争论
- **cargo clippy**: 超过 550 条 lint 规则，静态捕获错误
- **cargo check**: 快速编译验证，开发迭代利器
- **cargo test**: 内置测试框架，含文档测试
- **cargo doc**: 从注释自动生成精美文档

将这些工具集成到 CI 流水线中，形成质量保证的自动化闭环：

```
编写代码 → 本地验证(check/clippy/test) → commit → push → PR → CI 自动检查 → 合并
```

对于从 Python 转过来的开发者：这些工具的存在意味着你不需要像在 Python 中那样单独选择和配置 black、ruff、mypy、pytest、sphinx 等工具。Cargo 统一管理和配置它们，提供了开箱即用的开发体验。

记住：**良好的代码质量不是偶然的，而是工程化的结果。**

## 参考资源

- [The rustfmt Book](https://rust-lang.github.io/rustfmt/)
- [Clippy Documentation](https://doc.rust-lang.org/clippy/)
- [rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [Cargo Guide - Testing](https://doc.rust-lang.org/cargo/guide/tests.html)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI with GitHub Actions template](https://github.com/actions-rs/meta)
