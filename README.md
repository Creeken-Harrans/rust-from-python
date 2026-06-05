# Rust 从 Python 起步 (Rust from Python)

一套面向具有 Python 编程基础、但从未系统学习 Rust 的学习者的完整中文教程。

---

## 教程目标

- **由浅入深**：从 Hello World 到并发和异步编程
- **理解动机**：不只讲"怎么做"，更讲"为什么这样设计"
- **Python 对照**：在每个关键概念处对比 Python，降低理解门槛
- **可运行**：每章都是独立可执行的 Cargo Package
- **可验证**：所有代码通过格式化、编译、测试和 Clippy 检查
- **工程导向**：不仅教语法，还教项目结构、测试、文档和 CI

---

## 面向人群

- ✅ 具有 Python 编程基础（了解变量、函数、类、模块）
- ✅ 接触过 C 和 C++ 基本语法（变量、指针、结构体、类），但未形成系统编程知识体系
- ✅ 想学习系统编程但无完整的 C/C++ 深度经验
- ✅ 想理解 Rust 为何被称为"内存安全且高性能"
- ✅ 愿意投入时间系统学习（不是"30 分钟速成"）
- ❌ 不适合：只想快速浏览 Rust 语法差异的读者

> **同时具有 C/C++ 基础？** 阅读 [C_CPP_TO_RUST.md](C_CPP_TO_RUST.md) 获取系统化的 C/C++ → Rust 概念对照。

---

## 推荐阅读顺序

### 开始学习前

```
README.md
→ LEARNING_GUIDE.md
→ MENTAL_MODELS.md         ← 建立正确的直觉
→ PYTHON_TO_RUST.md         ← Python 概念对照
→ C_CPP_TO_RUST.md          ← C/C++ 概念对照（如有 C/C++ 基础）
→ MISCONCEPTIONS.md         ← 常见误解清单
→ chapters/00_course_orientation
```

### 按阶段学习

教程按阶段组织，每阶段有明确的学习目标：

1. **[阶段 A](COURSE_MAP.md#阶段-a认识-rust-与-cargo)**：认识 Rust 工具链和基本语法
2. **[阶段 B](COURSE_MAP.md#阶段-brust-的核心所有权系统)**：**核心！** 所有权、借用、切片——Rust 的基石
3. **[阶段 C](COURSE_MAP.md#阶段-c建模模式匹配与常用数据结构)**：结构体、枚举、集合、模式匹配
4. **[阶段 D](COURSE_MAP.md#阶段-d错误处理与工程拆分)**：错误处理、模块系统、测试
5. **[阶段 E](COURSE_MAP.md#阶段-e泛型trait-与生命周期)**：泛型、Trait、生命周期
6. **[阶段 F](COURSE_MAP.md#阶段-f函数式风格智能指针与资源管理)**：闭包、迭代器、智能指针
7. **[阶段 G](COURSE_MAP.md#阶段-g并发与异步)**：线程、信道、async/await
8. **[阶段 H](COURSE_MAP.md#阶段-h宏unsafe-与底层边界)**：宏、Unsafe Rust
9. **[阶段 I](COURSE_MAP.md#阶段-icargo-工程能力)**：Cargo 高级特性
10. **[阶段 J](COURSE_MAP.md#阶段-j综合实战项目)**：5 个综合项目

详细课程地图见 [COURSE_MAP.md](COURSE_MAP.md)。

### Python 开发者优先路径

如果你是 Python 开发者，建议对以下章节投入额外时间：
- **05-07**（所有权、借用）—— 这是最大的思维转变
- **09**（Option 与模式匹配）—— 改变你对 None 的理解
- **12**（错误处理）—— 从异常到 Result 的思维转换
- **15-16**（泛型与生命周期）—— 类型系统的深度理解

见 [LEARNING_GUIDE.md](LEARNING_GUIDE.md) 和 [PYTHON_TO_RUST.md](PYTHON_TO_RUST.md) 了解更多。

---

## 快速开始

### 前置要求

- Rust 工具链（推荐通过 [rustup](https://rustup.rs/) 安装）
- 如果尚未安装，参见 [INSTALL_RUST.md](INSTALL_RUST.md)（如不存在，系统会自动提示安装方法）

### 运行第一章

```bash
cd rust-from-python

# 运行课程导览
cargo run -p course_orientation

# 运行 Hello Cargo
cargo run -p hello_cargo
```

### 运行所有检查

```bash
# Linux/macOS
./scripts/check_all.sh

# Windows PowerShell
./scripts/check_all.ps1
```

---

## 如何阅读每章

每章是一个独立的 Cargo Package，包含：

```
chapters/XX_chapter_name/
├── Cargo.toml          # 项目清单
├── src/
│   ├── main.rs         # 可运行代码
│   └── lib.rs          # 库代码（部分章节）
├── tests/              # 测试（部分章节）
├── README.md           # 📖 主要学习材料（从这里开始！）
├── EXERCISES.md        # ✏️ 练习题
├── SOLUTIONS.md        # 💡 参考答案
├── examples/           # 📦 可运行参考示例（部分章节）
└── broken_examples/    # ⚠️ 故意错误示例（学习对照）
```

**推荐学习流程**：
1. 先运行 `cargo run -p <package_name>` 看输出
2. 再读 `README.md` 理解概念
3. 然后看源码，对照讲解
4. 独立完成 `EXERCISES.md` 中的练习
5. 使用 `cargo check` 或 `cargo test` 验证自己的代码
6. 对比 `SOLUTIONS.md` 参考答案，分析设计决策差异
7. 重新独立实现，巩固理解

## 练习与参考答案

每章提供：
- `EXERCISES.md`：结构化练习题（基础 → 组合 → 设计思考）
- `SOLUTIONS.md`：完整参考答案（含思路、实现、设计分析）
- `examples/`：部分章节提供可独立运行的教学示例
- `broken_examples/`：隔离保存的错误示例，说明编译器保护了什么

**建议使用方式**：先独立完成练习，再阅读答案。参考答案用于比较思路和设计决策，不代表所有题目只有一种正确写法。对于代码题，建议先比较思路，再自行重新实现，而不是直接复制。

---

## 为什么每章是独立 Cargo Package？

### 设计理由

1. **独立可运行**：每章可以脱离其他章节独立运行
2. **清晰的边界**：章节之间不互相依赖，强调每章知识独立性
3. **真实工程感**：每章看起来像一个真实的 Rust 项目（有自己的 Cargo.toml）
4. **统一验证**：根级 Workspace 可一次性对所有章节执行检查

### Workspace、Package、Crate、Module 的关系

```
Workspace (工作空间) ── 本教程的根目录
├── 多个 Package (包) ── 每章是一个 Package
│   ├── Binary Crate (二进制箱) ── src/main.rs
│   ├── Library Crate (库箱) ── src/lib.rs (可选)
│   └── Module Tree (模块树) ── mod 声明
```

详细解释见第 13 章和第 26 章。

---

## 如何处理第三方依赖

- **前半部分章节**（00-20）：尽量只使用标准库
- **后半部分章节**（21+）：逐步引入第三方 Crate（serde, clap, tokio 等）
- **离线环境**：核心教学章节可在离线环境完成；需要网络的章节会在 README 中注明
- **依赖无法下载**：项目会提供基于标准库的替代方案

---

## 常用命令速查

```bash
# 运行某章
cargo run -p <package_name>

# 测试某章
cargo test -p <package_name>

# 检查某章
cargo check -p <package_name>

# 检查所有章节
cargo check --workspace --all-targets

# 测试所有章节
cargo test --workspace

# 格式化所有代码
cargo fmt --all

# Clippy 检查所有代码
cargo clippy --workspace --all-targets --all-features

# 生成文档
cargo doc --workspace --no-deps --open
```

完整命令参考见 [COMMANDS.md](COMMANDS.md)。

---

## 目录总览

```
rust-from-python/
├── README.md                    # 本文件
├── COURSE_MAP.md                # 课程地图
├── LEARNING_GUIDE.md            # 学习指南
├── MENTAL_MODELS.md             # Rust 学习思维模型
├── PYTHON_TO_RUST.md            # Python → Rust 概念对照表
├── C_CPP_TO_RUST.md             # C/C++ → Rust 概念系统对照
├── MISCONCEPTIONS.md            # 常见误解澄清
├── GLOSSARY.md                  # 中英术语表
├── COMMANDS.md                  # 命令速查
├── TROUBLESHOOTING.md           # 排错指南
├── PROGRESS.md                  # 创建与改进进度
├── VALIDATION.md                # 验证报告
├── Cargo.toml                   # Virtual Workspace
├── rust-toolchain.toml          # 工具链声明
├── .gitignore
├── chapters/                    # 28 个教学章节
│   ├── 00_course_orientation/
│   ├── 01_hello_cargo/
│   ├── ...
│   └── 27_lints_format_docs_ci/
├── projects/                    # 5 个综合项目
│   ├── 01_guessing_game/
│   ├── 02_cli_text_search/
│   ├── 03_todo_cli/
│   ├── 04_parallel_text_stats/
│   └── 05_mini_kv_store/
├── scripts/
│   ├── check_all.sh
│   └── check_all.ps1
└── broken_examples/             # 故意错误的教学示例
```

---

## 工具链要求

- **Rust Edition**: 2024
- **工具链**: stable
- **最低 Rust 版本**: 1.82.0（支持 Rust 2024 Edition）
- **组件**: rustfmt, clippy

---

## 许可与贡献

本教程仅供学习使用。欢迎提出改进建议和错误修正。

---

## 致谢

本教程的设计借鉴了以下优秀资源：
- [The Rust Programming Language](https://doc.rust-lang.org/book/) (The Book)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/) by Google
- [Rustlings](https://github.com/rust-lang/rustlings)

---

**开始学习**：[第 00 章 - 课程导览](chapters/00_course_orientation/)

```bash
cargo run -p course_orientation
```
