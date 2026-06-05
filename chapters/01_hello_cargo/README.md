# 第 1 章: Hello Cargo — 理解 Rust 工具链与项目结构

---

## 本章目标

完成本章的学习之后，你将能够:

1. 理解 Rust 工具链的三大核心组件——`rustc`、`cargo`、`rustup`——各自的角色与协作关系。
2. 掌握 Cargo 项目的标准目录结构，知道每个文件和目录的用途。
3. 读懂 `Cargo.toml` 清单文件中的关键字段。
4. 理解 Debug 构建与 Release 构建的区别，以及它们在 `target/` 目录下的产物差异。
5. 熟练使用 `cargo check`、`cargo build`、`cargo run`、`cargo build --release`、`cargo clean`、`cargo fmt`、`cargo clippy` 等日常命令。
6. 能够将 Rust 的 Cargo 生态与 Python 的 pip/venv 进行类比，建立学习迁移的锚点。
7. 亲手编译并运行本章配套的示例程序，观察 stdout 与 stderr 的行为差异。

---

## 为什么需要学习这一章

如果你来自 Python 背景，你可能习惯了这样的工作流:

```bash
python my_script.py          # 直接运行
pip install requests          # 安装依赖
python -m venv .venv          # 创建虚拟环境
```

Rust 的世界则完全不同。Rust 是一门**编译型语言**，源代码必须先编译为机器码才能执行。这意味着你需要一个**编译器** (`rustc`)、一个**构建系统** (`cargo`)、以及一个**工具链管理器** (`rustup`)。

本章是整个 Rust 学习旅程的**起点**。在写任何有意义的 Rust 代码之前，你必须先理解这些工具是什么、它们之间如何协作、以及一个标准的 Rust 项目长什么样。跳过这一章直接写代码，就像跳过驾驶理论直接上路——你会不断撞上"找不到 main.rs"、"Cargo.toml 格式错误"、"为什么有两个 target 目录"这类问题。

本章的目标不是让你成为 Cargo 专家，而是建立一个**准确的心智模型 (mental model)**，让你在后续章节中遇到任何 Cargo 命令时都能立刻理解它在做什么。

---

## 背景知识: Rust 工具链全景

### 1. rustc — Rust 编译器 (Compiler)

`rustc` 是 Rust 语言的**编译器**。它负责将 `.rs` 源文件编译为可执行文件或库文件。

- **底层角色**: 类比 C/C++ 世界的 `gcc` 或 `clang`。
- **你几乎不会直接调用它**: 在绝大多数日常开发中，你通过 `cargo` 间接调用 `rustc`。直接使用 `rustc` 只在教学演示或极简脚本场景中才会出现。
- **试试看**: 你可以用 `rustc main.rs` 直接编译一个单文件程序，但一旦项目有了依赖 (dependencies)，你就必须使用 Cargo。

```bash
# 直接使用 rustc (仅限无依赖的极简场景)
rustc main.rs
./main
```

`rustc` 自身是用 Rust 编写的 (自举/bootstrap)，它的编译速度、错误信息质量、以及生成的代码优化程度在业界都享有盛誉。值得一提的是，Rust 编译器的错误信息 (error messages) 被公认为所有编程语言中**最友好**的之一——它通常会告诉你怎么修改、甚至直接给出建议的代码。

### 2. cargo — Rust 构建系统与包管理器

`cargo` 是 Rust 生态的**核心枢纽**。它同时扮演多个角色:

| 角色 | 说明 | Python 类比 |
|------|------|------------|
| 包管理器 (Package Manager) | 管理依赖、从 crates.io 下载 crate | `pip` |
| 构建系统 (Build System) | 编排编译流程、调用 `rustc` | `setuptools` / `poetry build` |
| 测试运行器 (Test Runner) | `cargo test` 执行单元测试和集成测试 | `pytest` |
| 文档生成器 (Doc Generator) | `cargo doc` 生成 HTML 文档 | `sphinx` / `pdoc` |
| 项目脚手架 (Scaffold) | `cargo new` / `cargo init` 创建新项目 | `poetry new` / `npm init` |
| 发布工具 (Publisher) | `cargo publish` 将包发布到 crates.io | `poetry publish` / `twine` |

一个常见的误解是认为 `cargo` *替换* 了 `rustc`。实际上 `cargo` 是 `rustc` 的**上层封装**——它读取 `Cargo.toml`、解析依赖、确定编译顺序，然后为每个 crate 调用 `rustc`。

```bash
# 你可以用 --verbose 看到 cargo 实际执行的 rustc 命令
cargo build --verbose
```

Cargo 的配置文件是 `Cargo.toml`，使用 TOML (Tom's Obvious, Minimal Language) 格式。TOML 被选中的原因是它比 JSON 更适合人类编写配置文件，比 YAML 更简洁且没有缩进陷阱。

### 3. rustup — Rust 工具链安装器

`rustup` 是 Rust 工具链的**安装和管理器**。它的职责是:

- 安装 Rust 工具链 (包括 `rustc`、`cargo`、`rust-std` 等)
- 管理多个工具链版本 (stable / beta / nightly)
- 更新工具链 (`rustup update`)
- 添加编译目标 (`rustup target add`)
- 管理 Rust 组件 (`rustup component add`)

```bash
# 安装 nightly 工具链
rustup toolchain install nightly

# 查看当前安装的工具链
rustup show

# 为特定项目覆盖工具链
rustup override set nightly
```

`rustup` 的核心理念是**组件化 (component)**。一个完整的工具链包含多个组件:

| 组件 (Component) | 说明 |
|------------------|------|
| `rustc` | Rust 编译器 |
| `cargo` | 构建系统与包管理器 |
| `rust-std` | 标准库 (每个编译目标一套) |
| `rustfmt` | 代码格式化工具 |
| `clippy` | 代码静态分析 / lint 工具 |
| `rust-docs` | 本地离线文档 |
| `rust-analyzer` | IDE 语言服务器 (LSP) |

你可以在任何时刻查看已安装的组件:

```bash
rustup component list --installed
```

### 4. rustfmt — 代码格式化工具

`rustfmt` 是 Rust 的**官方代码格式化工具**。它的设计哲学是:

- **零配置**: 默认风格就是标准风格，不需要争论花括号换不换行。
- **幂等 (idempotent)**: 多次运行 `cargo fmt` 结果一致。
- **通过 Cargo 调用**: `cargo fmt` (底层调用 `rustfmt`)。

```bash
cargo fmt              # 格式化当前项目所有 .rs 文件
cargo fmt -- --check   # 仅检查格式，不修改 (CI 用)
```

Rust 社区已经不存在 "tabs vs spaces" 或 "花括号风格" 的争论——`rustfmt` 就是最终裁决者。这让代码审查可以专注于逻辑而非格式。

### 5. clippy — 代码静态分析 / Lint 工具

`clippy` 是 Rust 的**官方 lint 工具**。它提供的远不止基本的语法检查:

- **代码风格**: 发现不地道的 Rust 写法
- **性能优化**: 标志低效的代码模式
- **潜在错误**: 检测常见逻辑错误
- **可读性**: 建议更清晰的写法
- **教育性**: 每条 lint 都有详细解释

```bash
cargo clippy                        # 运行 clippy 检查
cargo clippy -- -D warnings         # 将警告升级为错误 (推荐 CI 使用)
cargo clippy --fix                  # 自动修复一些问题
```

对于 Rust 学习者来说，`clippy` 是一个**免费的老师**——它不仅告诉你哪里不好，还会解释为什么不好以及怎么写更好。强烈建议在每一章练习中都运行一次 `cargo clippy`。

---

## 核心术语中英对照

学习 Rust 不可避免地要接触英文资料和社区。以下是本章涉及的核心术语对照表:

| 中文 | English | 说明 |
|------|---------|------|
| 包 | Package | 由 `Cargo.toml` 定义的完整项目，可包含多个 crate |
| 箱 | Crate | 编译单元，一个 `.rs` 入口文件 = 一个 crate |
| 二进制箱 | Binary Crate | 入口为 `main.rs`，编译为可执行文件 |
| 库箱 | Library Crate | 入口为 `lib.rs`，编译为 `.rlib` 或 `.so` 等库文件 |
| 版次 | Edition | Rust 语言版本的命名机制 (2015 / 2018 / 2021 / 2024) |
| 调试构建 | Debug Build | `cargo build` 的默认模式，包含调试信息，未优化 |
| 发布构建 | Release Build | `cargo build --release`，启用编译器优化 |
| 清单文件 | Manifest | 即 `Cargo.toml`，描述项目的元数据和依赖 |
| 依赖 | Dependency | 项目所依赖的外部 crate |
| 箱注册中心 | Crate Registry | crates.io，Rust 社区的公共包仓库 |
| 工作空间 | Workspace | 多个 package 共享同一个 `target/` 目录和 `Cargo.lock` |
| 特性标志 | Feature Flag | 条件编译选项，在 `Cargo.toml` 中定义 |
| 锁定文件 | Lock File | `Cargo.lock`，记录依赖树的精确版本 |

---

## 标准 Cargo Package 目录结构

```
hello_cargo/                        ← Package 根目录 (项目名)
│
├── Cargo.toml                      ← 项目清单 (Manifest)
│   ┌─────────────────────────────────────────
│   │ [package]        项目元数据
│   │ [dependencies]   生产依赖 (运行时必需的 crate)
│   │ [dev-dependencies] 开发依赖 (测试用，不发布)
│   │ [build-dependencies] 构建依赖 (build.rs 用)
│   │ [features]       条件编译特性
│   │ [profile.*]      编译配置 (优化级别等)
│   │ [workspace]      工作空间声明
│   └─────────────────────────────────────────
│
├── Cargo.lock                      ← 依赖锁定文件 (自动生成，不要手动编辑)
│
├── src/                            ← 源代码目录 (source)
│   ├── main.rs                     ← 二进制箱入口 (binary crate root)
│   │                                   一个 Package 可以有多个 binary crate，
│   │                                   放在 src/bin/ 目录下即可
│   ├── lib.rs                      ← 库箱入口 (library crate root, 可选)
│   └── bin/                        ← 额外的二进制箱入口 (可选)
│       └── another_tool.rs
│
├── tests/                          ← 集成测试 (integration tests)
│   └── integration_test.rs         ← 每个文件编译为一个独立的 crate
│
├── examples/                       ← 示例程序 (examples)
│   └── demo.rs                     ← cargo run --example demo
│
├── benches/                        ← 性能基准测试 (benchmarks)
│   └── benchmark.rs                ← cargo bench
│
├── build.rs                        ← 构建脚本 (可选, 在编译前执行)
│
└── target/                         ← 编译输出目录 (自动生成，加入 .gitignore)
    ├── debug/                      ← cargo build 的输出
    │   ├── hello_cargo             ← 可执行文件
    │   ├── deps/                   ← 依赖编译的中间产物
    │   └── incremental/            ← 增量编译缓存
    └── release/                    ← cargo build --release 的输出
        └── hello_cargo             ← 优化后的可执行文件 (更小更快)
```

### 重要概念: Package vs Crate

很多初学者混淆 **包 (Package)** 和 **箱 (Crate)** 这两个概念:

- **Package (包)**: 是 Cargo 层面的概念。一个 package 由一个 `Cargo.toml` 文件定义，是一个完整的、可分发的项目单元。
- **Crate (箱)**: 是 `rustc` 编译器层面的概念。一个 crate 对应一个 `.rs` 入口文件 (`lib.rs` 或 `main.rs`)，是编译的最小单元。

一个 Package 可以包含:
- **0 或 1 个** library crate
- **任意多个** binary crate

最简单的情况 (就是本章):
```
1 个 Package = 1 个 Binary Crate
```

常见的中型项目:
```
1 个 Package = 1 个 Library Crate + 1 个 Binary Crate
```

---

## Cargo.toml 字段详解

本章示例程序的 `Cargo.toml` 如下:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"
description = "Hello Cargo - 理解 Rust 工具链与项目结构"
```

### [package] — 包元数据

| 字段 | 必填 | 说明 |
|------|------|------|
| `name` | 是 | 包名。用于 `cargo build` 输出文件名、`cargo publish` 到 crates.io 时的标识符。只能使用小写字母、数字和连字符。 |
| `version` | 是 | 遵循 SemVer (语义化版本) 格式: `MAJOR.MINOR.PATCH`。`0.1.0` 表示初期开发阶段，API 尚不稳定。 |
| `edition` | 是 | Rust 语言版次。不同版次允许不兼容的语法变更，但同一编译器中所有版次的代码可以无缝混用。可选值: `"2015"`、`"2018"`、`"2021"`、`"2024"`。本书使用 `"2024"`。 |
| `description` | 否 | 简短描述。crates.io 上展示用。 |
| `authors` | 否 | 作者列表 (2018 edition 后变为可选字段)。 |
| `license` | 否 | 许可证 (SPDX 标识符，如 `"MIT"`、`"Apache-2.0"`)。发布到 crates.io 时建议填写。 |
| `repository` | 否 | 源码仓库 URL。 |
| `documentation` | 否 | 文档 URL。 |
| `readme` | 否 | README 文件路径 (默认 `README.md`)。 |

### [dependencies] — 依赖声明

本章示例程序没有外部依赖，所以 `Cargo.toml` 中没有 `[dependencies]` 段。但这是 Cargo.toml 中**最常用**的部分，格式如下:

```toml
[dependencies]
serde = "1.0"                           # 语义化版本范围: >=1.0.0, <2.0.0
serde = { version = "1.0", features = ["derive"] }  # 带 feature
rand = "0.8"                             # 基本
tokio = { version = "1", features = ["full"] }      # 完整写法
my-crate = { path = "../my-crate" }      # 本地路径依赖
my-crate = { git = "https://github.com/..." }       # Git 依赖
```

还有两类特殊依赖:

```toml
[dev-dependencies]    # 仅在测试和示例中可用，不参与正常编译
pretty_assertions = "1"

[build-dependencies]  # 仅在 build.rs 构建脚本中可用
cc = "1"
```

---

## Debug vs Release 构建的区别

Rust (通过 Cargo) 提供了两种构建配置 (profile):

| 特性 | Debug (`cargo build`) | Release (`cargo build --release`) |
|------|----------------------|----------------------------------|
| 优化级别 | `opt-level = 0` (无优化) | `opt-level = 3` (激进优化) |
| 调试信息 | `debug = true` (完整 DWARF) | `debug = false` (精简) |
| 编译速度 | 快 | 慢 (约 2-10 倍) |
| 二进制体积 | 大 (含调试符号) | 小 |
| 运行速度 | 慢 (未优化) | 快 (已优化) |
| 溢出检查 | `overflow-checks = true` | `overflow-checks = false` |
| 何时使用 | 日常开发、调试、测试 | 发布、性能测试、部署 |

### 为什么日常开发用 Debug?

- **编译快**: 你每次 `cargo run` 等待的时间更短，反馈循环更快。
- **调试信息完整**: 可以用 `gdb` / `lldb` 进行源码级调试。
- **整数溢出检查**: Debug 模式下整数溢出会 panic，帮你提前发现 bug。

### 为什么发布用 Release?

- **运行快**: 编译器进行了大量优化 (内联、循环展开、死代码消除、向量化等)。
- **体积小**: 去除调试信息、链接时优化 (LTO) 可以显著减小二进制体积。
- **适合分发**: 给用户的就是 Release 版本。

你可以自定义这些配置——在 `Cargo.toml` 中添加 `[profile.dev]` 或 `[profile.release]` 段:

```toml
[profile.release]
opt-level = 3       # 最大优化 (默认)
lto = true          # 链接时优化 (进一步减小体积/提升速度)
codegen-units = 1   # 更少并行代码生成单元 → 更多优化机会
panic = "abort"     # panic 时直接 abort (减少二进制体积)
```

---

## target/debug/ vs target/release/

`target/` 目录是 Cargo 的**编译输出根目录**。它是自动生成的，**绝不应该提交到版本控制** (`.gitignore` 中应包含 `target/`)。

```bash
# 清理所有编译输出
cargo clean
```

两个子目录的内容:

### target/debug/

```bash
cargo build   # 输出到 target/debug/
```

```
target/debug/
├── hello_cargo         ← 你的可执行文件
├── deps/               ← 所有依赖编译的中间产物
│   ├── lib*.rlib       ← 依赖的 rlib (Rust 库) 文件
│   └── ...
├── incremental/        ← 增量编译缓存 (加快后续编译)
├── build/              ← build.rs 构建脚本的输出
└── .fingerprint/       ← Cargo 用于判断是否需要重新编译的指纹
```

### target/release/

```bash
cargo build --release  # 输出到 target/release/
```

结构类似，但二进制文件通常**小很多** (有时可达 10 倍以上) 且**快很多** (有时可达 100 倍以上，取决于计算密集程度)。

### 快速验证

```bash
# 对比二进制体积
ls -lh target/debug/hello_cargo
ls -lh target/release/hello_cargo

# 对比运行速度 (用 time 命令)
time target/debug/hello_cargo
time target/release/hello_cargo
```

---

## 运行命令速查

本章涉及的所有 Cargo 命令:

### 核心命令

| 命令 | 功能 | 何时使用 |
|------|------|---------|
| `cargo check` | 检查代码能否编译 (不生成二进制) | 快速验证代码正确性，比 `cargo build` 快很多 |
| `cargo build` | Debug 构建 | 需要运行或调试时 |
| `cargo run` | Debug 构建并运行 | 日常开发最常用 |
| `cargo build --release` | Release 构建 (优化) | 发布、性能测试 |
| `cargo clean` | 清理 target/ 目录 | 释放磁盘空间、解决缓存问题 |
| `cargo fmt` | 格式化代码 | 提交代码前 |
| `cargo clippy` | 静态分析 / lint | 提交代码前、学习 Rust 最佳实践 |
| `cargo test` | 运行测试 | 每次修改后 |
| `cargo doc --open` | 生成并打开文档 | 查阅依赖的 API 文档 |
| `cargo update` | 更新 Cargo.lock 中的依赖版本 | 需要升级依赖时 |

### 其他常用命令

| 命令 | 功能 |
|------|------|
| `cargo new <name>` | 创建新的 binary project |
| `cargo new --lib <name>` | 创建新的 library project |
| `cargo init` | 在当前目录初始化 Cargo 项目 |
| `cargo add <crate>` | 添加依赖 (需要 cargo-edit 插件, 较新 Rust 已内置) |
| `cargo tree` | 显示依赖树 |
| `cargo metadata` | 输出项目元数据 (JSON 格式, 供工具链使用) |

---

## 预期输出

运行本章示例程序，你会看到类似如下的输出:

```text
[LOG]  程序启动，构建时间: 2026-06-05T18:30:00+08:00
[LOG]  输出到 stderr 的日志不会干扰 stdout 的数据流
╔══════════════════════════════════════╗
║     Hello, Rust!                     ║
║     欢迎来到 Rust 工具链的世界       ║
║     🚀 Built with cargo 🚀            ║
╚══════════════════════════════════════╝

这个程序由 Cargo 构建。
如果你看到了这行输出，说明 cargo build && cargo run 成功了！

═══════════════════════════════════════════
  构建信息 (Build Info)
═══════════════════════════════════════════
  构建时间 (BUILD_TIME):     2026-06-05T18:30:00+08:00
  编译期计算结果:             91
  (42 * 2 + 7 = 91 — 这个值在编译期就已确定)

═══════════════════════════════════════════
  Rust 工具链 (Toolchain) 三大组件
═══════════════════════════════════════════

1. rustc — Rust 编译器 (Compiler)
   ┌─────────────────────────────────────
   │ 将 .rs 源文件编译为可执行文件或库。
   │ ...
   └─────────────────────────────────────
... (省略后续输出)
```

注意输出中的 `[LOG]` 行——它们被输出到了 **stderr** 而非 stdout。你可以用以下命令验证:

```bash
# 只看到 stdout (stderr 被丢弃)
cargo run 2>/dev/null

# 只看到 stderr (stdout 被丢弃)
cargo run 1>/dev/null
```

---

## 代码讲解 (按逻辑模块)

### 模块 1: 常量定义

```rust
const BUILD_TIME: &str = "2026-06-05T18:30:00+08:00";
const THE_ANSWER: i32 = compute_at_compile_time();
```

- `const` 定义的是**编译期常量**，其值在编译时确定，运行时不能修改。
- `const` 和 `let` 的区别:
  - `let` 在函数内使用，绑定运行时值。
  - `const` 在模块级别使用，值必须在编译期可知。
- `&str` 是字符串切片类型，指向不可变的 UTF-8 字符串数据。Rust 中几乎所有"字符串常量"都是 `&str`。

### 模块 2: const fn — 编译期函数

```rust
pub const fn compute_at_compile_time() -> i32 {
    let base: i32 = 42;
    let doubled: i32 = base * 2; // 84
    let offset: i32 = 7;
    doubled + offset // 91    ← 注意: 没有分号 = 返回值
}
```

要点:
- `const fn` 意味着这个函数**可以在编译时被调用**。编译器会在编译时执行它，然后把结果直接嵌入二进制。
- 函数体**最后一行没有分号**——在 Rust 中，这意味着它是**表达式 (expression)** 而不是语句 (statement)，表达式的值会作为函数的返回值。等价于 `return doubled + offset;`。
- 类型注解 `i32`: 32 位有符号整数。Rust 有明确的大小约定: `i8`、`i16`、`i32`、`i64`、`i128`、`isize` (指针大小)。

### 模块 3: get_toolchain_info() — 返回 String

```rust
fn get_toolchain_info() -> String {
    let mut info = String::new();
    info.push_str("...");
    info
}
```

要点:
- `-> String` 声明函数返回一个 `String` 类型（堆分配的、可变的 UTF-8 字符串）。
- `let mut info` — `mut` 关键字让变量可变。Rust 默认所有变量不可变 (immutable)，这是 Rust 安全哲学的一部分。
- `String::new()` 创建一个空字符串。`::` 用于访问类型上的关联函数（类似其他语言的静态方法）。
- `.push_str(...)` 向字符串追加内容。
- 最后的 `info` (没有分号) 是返回值表达式。

对比 Python:

```python
# Python
def get_toolchain_info() -> str:
    info = ""
    info += "..."
    return info

# Rust — 默认不可变，追加需要 mut
fn get_toolchain_info() -> String {
    let mut info = String::new();
    info.push_str("...");
    info
}
```

### 模块 4: explain_package_structure() — 打印目录结构

这个函数纯粹调用 `println!` 来输出一个 ASCII 目录树。它没有返回值 (`-> ()` 可省略，`()` 是单元类型 unit type，类似 Python 的 `None` 但类型不同)。

### 模块 5: main() — 程序入口

```rust
fn main() {
    eprintln!("[LOG]  程序启动，构建时间: {BUILD_TIME}");
    println!("Hello, Rust!");
    // ...
}
```

要点:
- Rust 程序的入口必须是 `fn main()`。没有参数 (如果需要命令行参数，使用 `std::env::args()`)。
- `eprintln!` 输出到 stderr (标准错误)，`println!` 输出到 stdout (标准输出)。
- `{BUILD_TIME}` 是格式化字符串中的变量插值——Rust 的 `println!` 宏支持将变量名直接嵌入花括号 (Rust 1.58+)，比 C 的 `printf` 和 Python 的 f-string 都更简洁。
- 宏 (macro) 以 `!` 结尾: `println!`、`eprintln!`、`format!` 等。宏在编译时展开为代码，不是运行时函数调用。

---

## 与 Python 的对照

如果 Python 是你最熟悉的语言，这张对照表可以帮助你快速建立心智模型:

| 概念 | Python | Rust | 说明 |
|------|--------|------|------|
| 包管理器 | `pip` | `cargo` | Cargo 功能远超 pip，它同时管理构建、测试、发布 |
| 虚拟环境 | `venv` / `virtualenv` | `Cargo.toml` + `Cargo.lock` | Rust 不需要虚拟环境——依赖是项目级别的，通过 `Cargo.lock` 锁定版本 |
| 环境隔离 | `.venv/` 目录 | 无需 — 编译时静态链接 | Rust 的依赖在编译时解决，运行时不需要额外环境 |
| 依赖声明 | `requirements.txt` / `pyproject.toml` | `Cargo.toml` 的 `[dependencies]` | Cargo.toml 融合了 `pyproject.toml` 和 `requirements.txt` |
| 包索引 | PyPI (pypi.org) | crates.io | crates.io 上每个 crate 都自动生成文档 |
| 版本管理 | `pyenv` | `rustup` | rustup 同时管理编译器和标准库 |
| 格式化 | `black` / `ruff format` | `rustfmt` (`cargo fmt`) | rustfmt 零配置，风格统一 |
| Lint | `ruff` / `flake8` / `pylint` | `clippy` (`cargo clippy`) | clippy 有 550+ lint 规则且持续增长 |
| 入口点 | `if __name__ == "__main__"` | `fn main()` | Rust 的 main 必须存在（binary crate） |
| 标准输出 | `print()` | `println!()` | Rust 是宏，因为需要编译时格式化检查 |
| 标准错误 | `print(..., file=sys.stderr)` | `eprintln!()` | Rust 内置专用宏 |
| 文档 | docstring / Sphinx | `///` doc comment → `cargo doc` | Rust 文档注释是语言的一部分，且支持 Markdown + 代码测试 |

### 工作流对比

**Python:**

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python main.py
```

**Rust:**

```bash
# 不需要创建虚拟环境
cargo build   # 自动下载依赖并编译
cargo run     # 编译并运行
```

简洁性的背后是 Rust 的**编译时依赖解析**和**静态链接**模型。你不需要在部署时处理 Python 版本、虚拟环境、或系统库兼容性问题——Rust 编译出来的是一个**独立的二进制文件**。

---

## 常见错误

### 错误 1: 忘记保存文件就运行 cargo

```text
error: could not find `Cargo.toml` in ...
```

**原因**: 你不在项目的根目录。确保 `pwd` 输出包含 `Cargo.toml` 的那个目录。

### 错误 2: Cargo.toml 语法错误

```text
error: failed to parse manifest at ...
TOML parse error at line 3, column 1
```

**原因**: TOML 格式有误。常见问题:
- 字符串必须用引号括起来。
- section header `[package]` 不能有空格。
- edition 值必须用引号: `edition = "2024"` 而不是 `edition = 2024`。

### 错误 3: edition 不匹配

```text
error: Edition 2024 is unstable and only available for nightly builds
```

**原因**: 你的 Rust 版本太旧。运行 `rustup update stable` 更新，或者改用 `edition = "2021"`。

**检查你的 Rust 版本:**

```bash
rustc --version
# 应输出类似: rustc 1.85.0 (2026-01-01)
# Rust 2024 Edition 从 Rust 1.85 开始稳定支持
```

### 错误 4: 没有 main 函数

```text
error: `main` function not found in crate `hello_cargo`
```

**原因**: binary crate 必须有 `fn main()`。检查 `src/main.rs` 是否存在且包含 `fn main() {}`。

### 错误 5: 直接 rustc 编译带 Cargo.toml 的项目

```bash
rustc src/main.rs    # 可以编译，但没有用到 Cargo.toml
```

这不报错，但你**绕过了 Cargo**——依赖不会被下载，`Cargo.toml` 中的配置也不会生效。对于有 `Cargo.toml` 的项目，始终使用 `cargo build`。

### 错误 6: 语法错误 — 缺少分号

```rust
let x = 5    // 错误: 缺少分号
let y = 6;
```

```text
error: expected `;`, found `let`
```

Rust 的语句**必须以分号结尾**。唯一的例外是函数或代码块的最后一行（表达式返回值）。

### 错误 7: 可变性错误

```rust
let s = String::new();
s.push_str("hello");  // 错误: s 不可变
```

```text
error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
```

修改: `let mut s = String::new();`。这是 Rust 学习中最常见的编译错误之一。

---

## 练习建议

1. **修改并重新编译**: 修改 `main.rs` 中的某个字符串，重新 `cargo run`，观察变化。体会"修改 → 编译 → 运行"的循环。
2. **对比 Debug vs Release**: 分别运行 `cargo build` 和 `cargo build --release`，用 `ls -lh target/debug/hello_cargo target/release/hello_cargo` 对比二进制体积。
3. **格式化代码**: 故意打乱 `main.rs` 的缩进和空格，然后运行 `cargo fmt`，观察 rustfmt 如何自动修复。
4. **运行 clippy**: `cargo clippy` 查看是否有建议。
5. **探索 target 目录**: `ls -R target/debug/` 看看编译到底产生了哪些文件。
6. **查看 cargo 实际执行的命令**: `cargo build --verbose` 观察底层 `rustc` 调用。
7. **用 cargo clean 然后重新构建**: 对比 `cargo clean` 前后 `target/` 目录的变化，以及第二次 `cargo build` 的速度提升 (增量编译)。

---

## 本章小结

- **rustc** 是编译器，**cargo** 是构建系统 + 包管理器，**rustup** 是工具链管理器。三者协作形成完整的 Rust 开发环境。
- 标准 Cargo 项目由一个 `Cargo.toml` 和 `src/` 目录组成，编译输出放在自动生成的 `target/` 目录中。
- `package` vs `crate`: package 是项目概念 (由 Cargo.toml 定义)，crate 是编译概念 (由 main.rs 或 lib.rs 定义)。
- Debug 构建 (`cargo build`) 用于日常开发，快速编译、完整调试信息；Release 构建 (`cargo build --release`) 用于发布，优化运行速度和体积。
- `cargo fmt` 和 `cargo clippy` 是编码质量的左膀右臂，建议每次提交前运行。
- Rust 的工具链理念是**零配置**和**约定优于配置**——`Cargo.toml` 通常只需几行，标准目录结构开箱即用。
- 与 Python 生态相比，Cargo 是一个更统一、更集成的工具——它替代了 `pip` + `setuptools` + `venv` + `pyenv` 的功能集合。

---

## 下一章衔接

在下一章中，我们将深入 Rust 的类型系统:

- **基本数据类型**: 整数 (`i32`, `u64`)、浮点数 (`f32`, `f64`)、布尔 (`bool`)、字符 (`char`)
- **复合类型**: 元组 (`tuple`) 和数组 (`array`)
- **变量绑定**: `let`、`mut`、`const`、`static` 的区别与使用场景
- **函数**: 参数、返回值、表达式与语句的区别
- **所有权 (Ownership) 初探**: Rust 最核心的内存管理机制

准备好了吗？让我们进入第 2 章——**Rust 类型系统与所有权**。

---

*本章示例代码位于: `chapters/01_hello_cargo/`*
