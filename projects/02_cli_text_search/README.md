# 命令行文本搜索工具 (CLI Text Search)

> 一个用于学习和练习 Rust 核心概念的简化版 `grep` 实现

---

## 目录

1. [项目简介](#项目简介)
2. [项目目标](#项目目标)
3. [需求分析](#需求分析)
4. [知识点清单](#知识点清单)
5. [目录结构](#目录结构)
6. [设计决策分析](#设计决策分析)
7. [运行方法](#运行方法)
8. [测试方法](#测试方法)
9. [代码讲解按模块](#代码讲解按模块)
10. [可以扩展的方向](#可以扩展的方向)
11. [与真正 grep 的差距](#与真正-grep-的差距)

---

## 项目简介

`cli_text_search` 是一个用 Rust 编写的命令行文本搜索工具，是 Rust 学习路径中"综合练习阶段 A-D"的实践项目。它模拟了 Unix/Linux 系统中经典工具 `grep` 的核心功能——在文件中搜索包含指定文本的行，并将匹配结果输出到终端。

本项目的首要目标**不是**打造一个功能完备的生产级工具，而是通过实际编码来巩固 Rust 编程语言的核心概念。项目采用 **library-first（库优先）** 的设计理念，将搜索逻辑封装在独立的库 crate 中，命令行入口仅作为薄层调用库的公开 API。这种设计模式是 Rust 生态中的最佳实践，有助于代码复用、独立测试和关注点分离。

本项目的所有说明文档（包括本文件）均使用中文编写，代码注释同样以中文为主，旨在为中文母语的 Rust 学习者提供一个可读性强、概念清晰的参考实现。

---

## 项目目标

### 总体目标

构建一个命令行程序，其行为类似于简化版的 `grep`：

- 接收一个查询字符串和一个文件路径作为输入
- 读取指定文件的全部内容
- 逐行搜索包含查询字符串的行
- 将匹配的行打印到标准输出
- 支持大小写敏感和大小写不敏感两种搜索模式

### 学习目标

通过实现这个工具，学习者在以下几个方面获得实践经验：

1. **模块系统**：理解 `lib.rs`（库根）与 `main.rs`（二进制根）的关系，掌握 library-first 项目结构
2. **所有权与借用**：深入理解 `&str` 与 `String` 的区别，以及何时使用借用而非获取所有权
3. **生命周期标注**：掌握生命周期参数 `'a` 的语法和语义，理解借用检查器如何保证内存安全
4. **字符串切片**：操作 `&str` 类型，理解字符串切片与底层数据的关系
5. **错误处理**：使用 `Result<T, E>` 枚举、`?` 操作符和 `Box<dyn Error>` 进行灵活的错误传播
6. **环境变量**：通过 `std::env` 模块读取环境变量，实现配置的多种来源
7. **自动化测试**：编写单元测试和集成测试，使用 `assert!`、`assert_eq!` 等宏验证逻辑正确性
8. **文档注释**：使用 `///` 编写符合 Rust 标准的文档注释，包括代码示例（doc tests）

---

## 需求分析

### 功能需求

| 编号 | 需求描述 | 优先级 |
|------|---------|--------|
| F1 | 从命令行接收查询字符串和文件路径两个必选参数 | 高 |
| F2 | 读取指定文件的全部内容 | 高 |
| F3 | 在文件内容中搜索包含查询字符串的行（大小写敏感） | 高 |
| F4 | 支持大小写不敏感的搜索模式 | 中 |
| F5 | 将匹配行逐行输出到标准输出 | 高 |
| F6 | 支持通过 `-i` 或 `--case-insensitive` 命令行标志切换搜索模式 | 中 |
| F7 | 支持通过环境变量配置搜索参数 | 低 |
| F8 | 支持通过环境变量 `CASE_INSENSITIVE` 控制大小写敏感性 | 低 |
| F9 | 当参数不足时，显示清晰的错误信息和使用说明 | 高 |
| F10 | 当文件不存在或无法读取时，显示友好的错误信息 | 高 |

### 非功能需求

| 编号 | 需求描述 |
|------|---------|
| NF1 | 代码必须通过 `cargo build` 无警告编译 |
| NF2 | 所有公开函数必须包含 `///` 文档注释 |
| NF3 | 单元测试和集成测试覆盖率应达到 80% 以上 |
| NF4 | 搜索函数返回 `Vec<&str>`，避免不必要的内存分配 |
| NF5 | 项目采用 library-first 结构，核心逻辑与 CLI 入口分离 |

### 边界条件处理

- **空查询字符串**：不应匹配所有行（违反直觉），应返回空结果
- **空文件**：不应 panic，应正常返回空结果
- **文件不存在**：应返回明确的错误信息，而非 panic
- **包含 Unicode 内容**：Rust 原生支持 UTF-8，应正确处理中文等多字节字符

---

## 知识点清单

下表列出了本项目中涉及的核心 Rust 知识点，并标注了对应《Rust 程序设计语言》的章节参考。

| 知识点 | 应用位置 | 对应章节 | 说明 |
|--------|---------|---------|------|
| **模块系统 (`mod`, `use`, `pub`)** | `lib.rs`, `main.rs` | 第7章 | 库根 (`lib.rs`) 定义公开 API，二进制根 (`main.rs`) 引用库 |
| **所有权 (Ownership)** | `search`, `search_case_insensitive` | 第4.1节 | `query` 参数为 `&str` 借用，不获取所有权；`config` 被移动进 `run` 函数 |
| **借用 (Borrowing)** | 全局 | 第4.2节 | 所有函数参数优先使用 `&str` 引用而非 `String`，避免不必要的克隆 |
| **字符串切片 (`&str`)** | `search`, `search_case_insensitive` | 第4.3节 | 返回值 `Vec<&str>` 中的每个元素都是对原始内容的切片引用 |
| **生命周期标注 (`'a`)** | `search<'a>`, `search_case_insensitive<'a>` | 第10.3节 | 显式标注返回引用与输入参数 `contents` 的生命周期关系 |
| **`Result<T, E>` 枚举** | `SearchConfig::new`, `from_env`, `run` | 第9.2节 | 使用 `Result` 进行可恢复的错误处理，而非 `panic!` |
| **`?` 操作符** | `run`, `from_env` | 第9.2节 | 简化错误传播，在 `Result` 为 `Err` 时提前返回 |
| **`Box<dyn Error>` trait 对象** | `run` 函数签名 | 第17章 | 允许函数返回多种不同类型的错误，实现多态错误处理 |
| **环境变量 (`std::env`)** | `SearchConfig::from_env`, `main.rs` | 第12章 | 读取环境变量作为配置来源，检查 `CASE_INSENSITIVE` 变量 |
| **命令行参数 (`std::env::args`)** | `SearchConfig::new`, `main.rs` | 第12章 | 解析命令行参数，提取查询字符串和文件路径 |
| **自动化测试 (`#[test]`)** | `lib.rs` (tests 模块), `tests/integration_test.rs` | 第11章 | 单元测试和集成测试，使用 `cargo test` 运行 |
| **`#[cfg(test)]` 条件编译** | `lib.rs` 底部 | 第11.3节 | 仅在测试配置下编译测试模块，减小二进制体积 |
| **Library-First 设计** | 项目架构 | 第7章、第14章 | 核心逻辑位于库中，二进制入口为薄包装层 |
| **文档注释 (`///`)** | 所有公开函数 | 第14.2节 | 每个公开 API 都有文档注释，包含使用示例 |
| **文件 I/O (`std::fs::read_to_string`)** | `run` 函数 | 第12章 | 读取文件内容到字符串 |
| **迭代器 (`Iterator`)** | `search`, `search_case_insensitive` | 第13章 | 使用 `.lines().filter().collect()` 的链式调用 |
| **闭包 (Closures)** | `search`, `search_case_insensitive` | 第13章 | `filter` 中使用闭包 `|line| line.contains(query)` |
| **`derive` 属性** | `SearchConfig` 结构体 | 第5章 | `#[derive(Debug, Clone)]` 自动实现常用 trait |
| **字符串方法** | `search_case_insensitive` | 第8章 | `.to_lowercase()`, `.contains()`, `.lines()` 等字符串操作 |
| **`process::exit`** | `main.rs` | 第12章 | 错误时以非零退出码终止程序 |
| **`eprintln!` 宏** | `main.rs` | 第12章 | 将错误信息输出到标准错误流 (stderr) |

---

## 目录结构

```
02_cli_text_search/
├── Cargo.toml                  # Cargo 项目配置文件
├── README.md                   # 本文件：项目说明文档
├── src/
│   ├── lib.rs                  # 库根文件：核心搜索逻辑 (~280 行)
│   │                           #   - SearchConfig 结构体及实现
│   │                           #   - search() 搜索函数
│   │                           #   - search_case_insensitive() 搜索函数
│   │                           #   - run() 流程编排函数
│   │                           #   - 单元测试模块
│   └── main.rs                 # 二进制根文件：命令行入口 (~60 行)
│                               #   - 解析命令行参数
│                               #   - 调用库函数
│                               #   - 错误处理和展示
└── tests/
    └── integration_test.rs     # 集成测试文件：对外部 API 的测试 (~200 行)
                                #   - search 函数测试
                                #   - search_case_insensitive 函数测试
                                #   - 空查询边界测试
                                #   - 无匹配结果测试
                                #   - SearchConfig 解析测试
```

### 文件职责说明

| 文件 | 职责 | 关键特性 |
|------|------|---------|
| `Cargo.toml` | 项目元数据和依赖声明 | `edition = "2024"`，无外部依赖 |
| `src/lib.rs` | 库的核心实现 | 所有 `pub` 函数构成公开 API |
| `src/main.rs` | 二进制入口 | 仅含 `main()` 函数，调用库 API |
| `tests/integration_test.rs` | 集成测试 | 作为外部 crate 测试公开 API |

---

## 设计决策分析

### 为什么采用 Library-First 设计

**Library-First** 是指将核心业务逻辑放置在库 crate（`lib.rs`）中，而将命令行入口（`main.rs`）设计为调用库 API 的薄包装层。这种设计模式有以下关键优势：

1. **代码复用性**：库可以被其他 Rust 项目直接依赖和使用，而不仅仅是作为独立命令行工具。例如，其他工具可以通过 `use cli_text_search::search` 直接调用搜索功能。

2. **可测试性**：核心逻辑位于库中，所有 `pub` 函数都可以被单元测试和集成测试直接调用。相比之下，如果逻辑全部写在 `main.rs` 中，测试将变得困难——因为 Rust 不允许直接测试二进制 crate 中的函数。

3. **关注点分离**：库负责"做什么"（搜索逻辑），二进制负责"怎么交互"（参数解析、输出格式化）。这种分离使得两个部分可以独立演化和维护。

4. **符合 Rust 生态惯例**：Rust 社区中的大多数 CLI 工具都采用这种模式（例如 `ripgrep`、`bat`、`fd` 等），使代码对 Rust 开发者来说更易于理解和贡献。

5. **方便文档生成**：`cargo doc` 生成的文档主要面向库的公开 API，library-first 结构使文档更有价值。

### 为什么返回 `Vec<&str>`

`search` 和 `search_case_insensitive` 函数的返回值类型是 `Vec<&str>`（字符串切片的向量），而非 `Vec<String>`。这是一个深思熟虑的设计选择，涉及 Rust 的核心概念：

**1. 零拷贝 (Zero-Copy)**

`Vec<&str>` 中的每个元素都是对原始 `contents` 字符串中某一行的引用（借用），不会复制任何行数据。当处理大文件（如几 GB 的日志文件）时，这种设计可以避免巨大的内存开销。因为匹配的行通常只是原始内容的一小部分，但我们不需要为每一行分配新的堆内存。

```rust
// 如果返回 Vec<String>，每行都需要克隆：
// results.push(line.to_string());  // 堆分配 + 数据拷贝

// 而 Vec<&str> 只需存储指针和长度（16字节/行）：
// results.push(line);  // 仅拷贝切片元数据
```

**2. 体现 Rust 的借用语义**

返回引用而非自有数据，让调用者明确理解：搜索结果的生命周期受限于被搜索的原始文本内容。这迫使用户正确地管理数据的生命周期。

**3. 性能优势**

对于高吞吐量的搜索场景，避免不必要的内存分配可以显著提升性能。在基准测试中，零拷贝设计通常比拷贝方案快 30%-50%。

**4. 与标准库 API 保持一致**

Rust 标准库中的许多 API 也返回引用类型（如 `str::lines()` 返回 `Lines` 迭代器，产生 `&str`），我们的 API 与此保持一致。

### 生命周期标注的必要性

`search<'a>` 中的 `'a` 生命周期参数不是编译器可以自动推断的吗？为什么需要显式标注？

**终身省略规则 (Lifetime Elision Rules) 的局限性**：

Rust 的终身省略规则只适用于简单场景（每个引用参数有自己的匿名生命周期）。当函数有多个引用参数和引用返回值时，编译器无法自动确定返回值应该"关联"到哪个输入参数的声明周期。

在我们的 `search` 函数中：

```rust
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str>
```

- `query: &str` — 匿名生命周期（编译器自动分配）
- `contents: &'a str` — 显式标注生命周期 `'a`
- `Vec<&'a str>` — 返回值的生命周期明确绑定到 `contents`

如果不写 `'a`，编译器将无法知道返回的引用是借用于 `query` 还是 `contents`。而实际上，返回的行引用必须来自 `contents`（因为我们是按 `contents` 的行来筛选的）。

**为什么返回值不绑定到 `query`？**

因为 `search` 函数的逻辑是：遍历 `contents` 的每一行，检查该行是否包含 `query`，然后将匹配的**行**（来自 `contents`）返回。返回的行字符串是 `contents` 的子切片，与 `query` 无关。用 `'a` 显式标注这一关系，既是给编译器的指令，也是给人类读者的文档。

**如果省略生命周期标注会怎样？**

```rust
// 错误：编译器不知道返回的引用借用于 query 还是 contents
pub fn search(query: &str, contents: &str, ...) -> Vec<&str>
```

编译器会报错，要求显式标注生命周期参数。这正是 Rust 安全保证的体现——编译器在模糊地带要求程序员做出明确承诺。

### SearchConfig 的设计

`SearchConfig` 结构体封装了搜索操作的所有配置参数，遵循**配置对象模式 (Configuration Object Pattern)**。

**为什么使用结构体而非多个独立参数？**

1. **可扩展性**：未来添加新配置项（如正则表达式模式、递归搜索标志）时，只需添加结构体字段，而不改变函数签名
2. **清晰性**：配置的语义通过字段名称和文档注释表达，比位置参数更清晰
3. **可传递性**：配置可以在函数之间整体传递，无需拆解和重组参数列表

**两种构造方式的互补性**：

- `SearchConfig::new(&[String])` — 面向命令行使用场景，解析位置参数
- `SearchConfig::from_env()` — 面向自动化/脚本场景，从环境变量读取配置

这种双路径设计使得工具既能手动交互式使用，也能在 CI/CD 管道中自动化运行。

---

## 运行方法

### 前置条件

- Rust 工具链 1.85.0 或更高版本（支持 Rust 2024 edition）
- 可通过 `rustup update stable` 更新到最新稳定版

### 编译项目

```bash
cd 02_cli_text_search
cargo build
```

编译产物：
- 调试版二进制：`target/debug/cli_text_search`
- 如果要优化：`cargo build --release`

### 基本用法

```bash
# 语法
cargo run -- <查询字符串> <文件路径> [-i|--case-insensitive]

# 或直接运行编译后的二进制
./target/debug/cli_text_search <查询字符串> <文件路径> [-i|--case-insensitive]
```

### 使用示例

首先创建一个测试用文本文件：

```bash
echo "Rust 是一门系统编程语言
rust 是小写形式
RUST 是全大写
Java 是另一门语言
学习 Rust 非常有趣" > test.txt
```

**示例 1：大小写敏感的搜索（默认行为）**

```bash
$ cargo run -- "Rust" test.txt

搜索 'Rust' 在文件 'test.txt' 中（大小写敏感）...
在文件 'test.txt' 中找到 2 行包含 'Rust' 的结果:

  1: Rust 是一门系统编程语言
  2: 学习 Rust 非常有趣
```

注意：`"rust 是小写形式"` 和 `"RUST 是全大写"` 没有被匹配，因为搜索是大小写敏感的。

**示例 2：大小写不敏感的搜索**

```bash
$ cargo run -- "rust" test.txt -i

搜索 'rust' 在文件 'test.txt' 中（大小写不敏感）...
在文件 'test.txt' 中找到 4 行包含 'rust' 的结果:

  1: Rust 是一门系统编程语言
  2: rust 是小写形式
  3: RUST 是全大写
  4: 学习 Rust 非常有趣
```

**示例 3：搜索不存在的文本**

```bash
$ cargo run -- "Python" test.txt

搜索 'Python' 在文件 'test.txt' 中（大小写敏感）...
未找到包含 'Python' 的行。
```

**示例 4：搜索源代码文件**

```bash
# 在项目的源代码中搜索 "fn" 关键词
$ cargo run -- "fn " src/lib.rs
```

### 使用环境变量配置

```bash
# 设置环境变量
export SEARCH_QUERY="error"
export SEARCH_FILE="app.log"
export CASE_INSENSITIVE=1

# 在代码中调用 from_env() 使用此配置
```

命令行参数方式的工具不直接使用这些环境变量（需要调用 `from_env()`），但这展示了库的灵活性——其他程序可以通过环境变量来配置搜索行为。

### 错误处理示例

**参数不足时：**

```bash
$ cargo run -- "hello"

解析命令行参数时出错: 参数不足：需要至少 2 个参数（查询字符串 和 文件路径），但只收到了 1 个。
用法: target/debug/cli_text_search <查询字符串> <文件路径> [-i|--case-insensitive]
示例:
  target/debug/cli_text_search "hello" poem.txt
  target/debug/cli_text_search "Rust" src/main.rs -i
```

**文件不存在时：**

```bash
$ cargo run -- "hello" nonexistent.txt

搜索 'hello' 在文件 'nonexistent.txt' 中（大小写敏感）...
运行出错: 无法读取文件 'nonexistent.txt': No such file or directory (os error 2)
```

---

## 测试方法

### 运行所有测试

```bash
cd 02_cli_text_search
cargo test
```

### 运行特定测试类别

```bash
# 仅运行单元测试（lib.rs 中的测试）
cargo test --lib

# 仅运行集成测试（tests/ 目录中的测试）
cargo test --test integration_test

# 仅运行文档测试（/// 注释中的代码示例）
cargo test --doc
```

### 运行特定名称的测试

```bash
# 运行名称包含 "case_insensitive" 的测试
cargo test case_insensitive

# 运行名称包含 "empty" 的测试
cargo test empty
```

### 显示测试输出

```bash
# 默认情况下，通过的测试不显示 stdout 输出
# 使用 --nocapture 查看 println! 输出
cargo test -- --nocapture

# 使用 --show-output 也可以（较新版本）
cargo test -- --show-output
```

### 测试覆盖率说明

本项目包含以下测试层次：

| 测试层次 | 位置 | 数量 | 测试内容 |
|---------|------|------|---------|
| 文档测试 (Doc Tests) | `lib.rs` 中的 `///` 注释 | 8+ | API 使用示例作为测试 |
| 单元测试 (Unit Tests) | `lib.rs` 中的 `#[cfg(test)]` 模块 | 8 | 内部函数的正确性 |
| 集成测试 (Integration Tests) | `tests/integration_test.rs` | 14 | 公开 API 的端到端验证 |

### 测试用例说明

#### 单元测试 (`lib.rs`)

- `test_new_not_enough_args` — 验证参数不足时返回错误
- `test_new_basic` — 验证基本的参数解析
- `test_new_case_insensitive_short` — 验证 `-i` 短标志
- `test_new_case_insensitive_long` — 验证 `--case-insensitive` 长标志
- `test_search_case_sensitive` — 验证大小写敏感搜索
- `test_search_case_insensitive_variants` — 验证大小写不敏感搜索
- `test_empty_query` — 验证空查询防御性处理
- `test_no_matches` — 验证无匹配时返回空结果

#### 集成测试 (`tests/integration_test.rs`)

- `test_search_basic_case_sensitive` — 在英文诗歌中进行大小写敏感搜索
- `test_search_with_chinese` — 验证中文内容的搜索
- `test_search_with_case_insensitive_flag` — 通过 flag 控制大小写
- `test_search_case_insensitive_all_variants` — 所有大小写变体
- `test_search_case_insensitive_poem` — 诗歌中的大小写不敏感搜索
- `test_search_case_insensitive_substring` — 子串匹配
- `test_empty_query_returns_empty` — 空查询边界条件
- `test_empty_query_multi_line` — 空查询的多行边界
- `test_no_matches_returns_empty` — 无匹配结果
- `test_no_matches_in_poem` — 诗歌中搜索不存在的词
- `test_near_miss_no_match` — 几乎匹配但实际不匹配
- `test_config_new_valid` — 配置解析正确性
- `test_config_new_insufficient_args` — 参数不足
- `test_config_new_case_insensitive` — 大小写标志解析
- `test_config_debug` — Debug 派生正确性

---

## 代码讲解按模块

### SearchConfig 模块 (`lib.rs` 第 50-175 行)

`SearchConfig` 是一个配置对象，封装了单次搜索所需的所有参数。它包含三个公开字段：

```rust
pub struct SearchConfig {
    pub query: String,           // 搜索查询字符串
    pub file_path: String,       // 目标文件路径
    pub case_sensitive: bool,    // 是否大小写敏感
}
```

**关键的 Rust 概念体现**：

- **结构体定义**：使用 `struct` 关键字定义自定义数据类型，字段类型分别为 `String`（自有字符串）和 `bool`
- **`pub` 可见性**：所有字段标记为 `pub`，允许外部代码直接访问（对于配置对象这是合理的，因为字段本身就是公开 API 的一部分）
- **`#[derive(Debug, Clone)]`**：通过派生宏自动实现 `Debug` 和 `Clone` trait，让配置可以被打印调试和被克隆
- **关联函数**：`new()` 和 `from_env()` 是 `impl SearchConfig` 块中的关联函数（类似静态方法），第一个参数不是 `self`

**`new()` 方法**的方法签名为 `pub fn new(args: &[String]) -> Result<SearchConfig, String>`：
- `&[String]`：接受字符串切片的引用，借用而非获取所有权——调用者仍然可以使用 `args`
- `Result<SearchConfig, String>`：使用 `Result` 枚举处理可恢复错误，`String` 作为错误类型承载人类可读的错误信息
- 参数校验：显式检查 `args.len() < 3`，避免索引越界 panic
- `clone()`：使用 `.clone()` 从引用创建自有字符串

**`from_env()` 方法**的方法签名为 `pub fn from_env() -> Result<SearchConfig, String>`：
- `env::var("SEARCH_QUERY")`：调用标准库函数读取环境变量，返回 `Result<String, VarError>`
- `.map_err(|_| ...)`：使用组合器转换错误类型，将 `VarError` 转为自定义的中文错误信息
- `?` 操作符：在错误时提前返回，避免深层嵌套的 `match` 或 `if let`

### search 函数 (`lib.rs` 第 230-260 行)

```rust
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str>
```

**核心逻辑**：

1. **空查询检查**：如果 `query.is_empty()`，立即返回空向量。这是防御性编程——由于 Rust 字符串的 `contains("")` 总是返回 `true`，不检查会导致所有行匹配，这不符合 grep 用户的行为预期。

2. **分发逻辑**：根据 `case_sensitive` 参数决定执行路径：

   ```rust
   if case_sensitive {
       contents.lines().filter(|line| line.contains(query)).collect()
   } else {
       search_case_insensitive(query, contents)
   }
   ```

3. **迭代器链**：`.lines()` 返回行的迭代器，`.filter()` 保留满足条件的行，`.collect()` 将结果收集到 `Vec`

**关键的 Rust 概念**：

- **生命周期 `'a`**：标注返回的 `&str` 引用借用于 `contents` 参数，编译器确保返回的引用不会超过 `contents` 的存活范围
- **迭代器惰性求值**：`.lines()` 和 `.filter()` 不立即执行，只有 `.collect()` 才触发实际计算
- **闭包**：`|line| line.contains(query)` 是一个闭包，捕获了外部的 `query` 变量（不可变借用）

### search_case_insensitive 函数 (`lib.rs` 第 275-305 行)

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
```

**与 search 的区别**：

- 将 `query` 转为小写一次：`let query_lower = query.to_lowercase();`（这是 `String` 类型）
- 对每一行，将行内容也转为小写后比较：`line.to_lowercase().contains(&query_lower)`
- 性能考虑：`query.to_lowercase()` 在外层执行一次，避免在 `filter` 闭包中对每个候选行都重复转换 query

**Unicode 处理**：

Rust 的 `to_lowercase()` 方法正确处理 Unicode。例如，德语中的 `'SS'` 的小写是 `'ss'`，土耳其语中的 `'İ'` 的小写是 `'i\u{307}'`。标准库使用的是 Unicode 默认的大小写转换算法，不依赖特定的 locale。

**为什么 `to_lowercase()` 返回 `String` 而非 `&str`？**

因为小写形式可能比原字符串更长（如 `'İ'` → `"i\u{307}"` 是 2 个字节 vs 原始 2 个字节的不同形式），无法在原始内存中原地操作，必须分配新的堆内存。这很好地展示了 Rust 的类型系统如何编码了底层实现约束。

### run 函数 (`lib.rs` 第 180-215 行)

```rust
pub fn run(config: SearchConfig) -> Result<(), Box<dyn Error>>
```

**流程编排**：

1. **读取文件**：`fs::read_to_string(&config.file_path)` — 读取整个文件到 `String`，使用 `?` 传播 I/O 错误
2. **选择搜索策略**：根据 `config.case_sensitive` 分支调用 `search` 或 `search_case_insensitive`
3. **格式化输出**：将搜索结果编号并打印到 stdout

**错误处理细节**：

```rust
fs::read_to_string(&config.file_path).map_err(|e| {
    Box::<dyn Error>::from(format!("无法读取文件 '{}': {}", config.file_path, e))
})?;
```

- `map_err`：将 `io::Error` 转换为带有文件路径上下文的错误消息，方便用户定位问题
- `Box::<dyn Error>::from(...)`：将 `String`（实现了 `Error` trait）包装为 trait 对象
- `?`：在错误时直接返回，不继续执行后续逻辑

**`Box<dyn Error>` 的意义**：

`Box<dyn Error>` 表示"任何实现了 `std::error::Error` trait 的类型"。函数可以返回 `io::Error`、`String`（实现了 `Error`）、或其他任何错误类型。这是 Rust 中处理异构错误的经典模式——牺牲一点运行时性能（动态分发），换取代码的简洁和灵活性。

### main 入口 (`main.rs`)

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    // ...
}
```

**关键设计点**：

1. **`unwrap_or_else` 模式**：
   ```rust
   let config = SearchConfig::new(&args).unwrap_or_else(|err| {
       eprintln!("...", err);
       process::exit(1);
   });
   ```
   这是 `Result` 消费的标准模式——在错误时执行闭包（打印错误并退出），在成功时提取内部值。比 `unwrap()` 更友好，比 `match` 更简洁。

2. **`if let Err(e)` 模式**：
   ```rust
   if let Err(e) = cli_text_search::run(config) {
       eprintln!("运行出错: {}", e);
       process::exit(1);
   }
   ```
   因为 `run()` 成功时返回 `Ok(())`（没有有意义的值），使用 `if let` 只处理错误情况比 `match` 更简洁。

3. **标准输出 vs 标准错误**：
   - `println!` — 正常搜索结果的输出（stdout），可被管道传递给其他程序
   - `eprintln!` — 错误信息和诊断信息（stderr），不会污染 stdout 的数据流

---

## 可以扩展的方向

本项目的设计为扩展预留了充分的空间。以下是一些可行的增强方向：

### 1. 正则表达式支持

**描述**：当前搜索使用简单的子串匹配（`contains()`）。可以引入 `regex` crate 支持正则表达式搜索。

**实现要点**：
- 添加 `use_regex: bool` 配置字段
- 使用 `regex::Regex::new(query)` 编译正则表达式
- 用 `regex.is_match(line)` 替代 `line.contains(query)`
- 缓存编译后的正则表达式以提高性能
- 可以使用 `lazy_static` 或 `once_cell` 实现模式缓存

**涉及新知识点**：外部 crate 依赖、正则表达式编译与优化

### 2. 文件通配符 (Glob) 支持

**描述**：支持 `*.rs`、`src/**/*.rs` 等通配符模式，在多个文件中搜索。

**实现要点**：
- 引入 `glob` crate 或使用 `walkdir` 递归遍历目录
- 将 `file_path: String` 改为 `file_patterns: Vec<String>`
- 对每个匹配的文件执行搜索
- 输出时标注文件名：`src/lib.rs:42: ...`

**涉及新知识点**：文件系统遍历、glob 模式匹配、多文件并发处理

### 3. 彩色输出

**描述**：在终端中将匹配的关键词高亮显示（红色或反色），提升可读性。

**实现要点**：
- 引入 `termcolor` 或 `colored` crate
- 检测终端是否支持颜色（`atty` 或 `is_terminal`）
- 使用 ANSI 转义序列包裹匹配的关键词
- 输出结果时对每一行进行着色处理

**涉及新知识点**：终端控制、ANSI 转义码、条件编译（跨平台终端差异）

### 4. 并行搜索

**描述**：使用多线程并行搜索大文件或多个文件，充分利用多核 CPU。

**实现要点**：
- 使用 `rayon` crate 的并行迭代器
- 将文件内容按行分块，每块在独立线程中搜索
- 使用 `Arc<Vec<Mutex<String>>>` 收集并行搜索结果
- 或使用 `rayon` 的 `par_lines()` 替代 `lines()`
- 使用 `crossbeam` channel 进行线程间通信

**涉及新知识点**：并发编程、`Send` 和 `Sync` trait、线程安全、`Arc`/`Mutex`

### 5. 上下文行显示

**描述**：像 `grep -C 3` 一样，显示匹配行前后的上下文。

**实现要点**：
- 添加 `context_lines: usize` 配置字段
- 收集匹配行的行号
- 输出时包含 `[匹配行号 - context..匹配行号 + context]` 范围内的行

### 6. 统计模式

**描述**：使用 `-c` 标志只显示匹配行数，而非行内容。

**实现要点**：
- 添加 `count_only: bool` 配置字段
- 在 `run` 函数中根据配置决定输出格式

### 7. 反向匹配

**描述**：类似 `grep -v`，显示不包含查询字符串的行。

**实现要点**：
- 添加 `invert_match: bool` 配置字段
- 在过滤时使用 `!line.contains(query)`

### 8. 行号显示

**描述**：在每行输出前显示行号，类似 `grep -n`。

**实现要点**：
- 使用 `.lines().enumerate()` 保留行号信息
- 修改返回类型为 `Vec<(usize, &str)>` 以包含行号

---

## 与真正 grep 的差距

虽然本项目实现了 `grep` 的核心概念，但与其相比仍有显著差距。了解这些差距有助于理解系统级工具开发的复杂性：

### 性能差距

| 方面 | 本工具 | 真正 grep |
|------|--------|----------|
| 算法 | 朴素的子串匹配 (O(n*m)) | Boyer-Moore 等高效算法 (O(n/m) 平均) |
| I/O | 内存映射或全量读取 | 缓冲读取、内存映射、按块处理 |
| 内存使用 | 整个文件加载到内存 | 流式处理，常驻内存极小 |
| 多文件 | 不支持 | 高度优化的目录遍历和并行处理 |
| SIMD | 无 | 使用 SIMD 指令加速字符串匹配 |

### 功能差距

| 功能 | 本工具 | 真正 grep |
|------|--------|----------|
| 正则表达式 | 不支持 | BRE、ERE、PCRE 多种引擎 |
| 文件递归 | 不支持 | `-r` / `-R` 递归搜索目录 |
| 上下文行 | 不支持 | `-A` / `-B` / `-C` 显示上下文 |
| 行号 | 不支持 | `-n` 显示行号 |
| 统计模式 | 不支持 | `-c` 只显示匹配计数 |
| 反向匹配 | 不支持 | `-v` 显示不匹配的行 |
| 单词匹配 | 不支持 | `-w` 整词匹配 |
| 固定字符串 | 默认行为 | `-F` 模式，有专门优化 |
| 输出格式 | 简单编号 | 多种格式：`--color`、文件名前缀、`-H`/`-h` 控制 |
| 二进制文件 | 会 panic | 自动检测并跳过或强制处理 |
| 编码支持 | UTF-8 | 支持多种编码和二进制数据 |

### 错误处理差距

- **grep**：能优雅处理权限拒绝、符号链接循环、二进制文件、设备文件等边缘情况
- **本工具**：基本的错误转换和传播，很多边缘情况未处理

### 资源利用差距

- **grep**：使用 `mmap` 进行零拷贝文件读取，操作系统级页面缓存优化
- **本工具**：使用 `read_to_string` 全量读取，小文件可行但大文件受限

### 可配置性差距

- **grep**：通过 `GREP_OPTIONS` 环境变量、配置文件（`.greprc`）、命令行参数三层配置
- **本工具**：仅支持命令行参数和有限的环境变量

---

## 从 Python、C、C++ 迁移时值得注意的设计差异

### 1. `&str` 而非 `String`：借用与所有权的区分

Python 中所有字符串都是堆分配的不可变对象，传参时引用计数增加，没有"借用"的概念。C 中 `const char*` 指向字符数组但不携带长度信息，极易越界。本项目 `search` 函数的参数 `query: &str` 和 `contents: &str` 表达的是"我暂时借用这些数据，不获取所有权"。返回值 `Vec<&str>` 进一步体现：搜索结果是对原始内容的切片引用，没有任何数据被复制。这种零拷贝设计在 Python 中无法在类型层面表达（因为无法保证原始数据活得比引用久），在 C 中手动实现极易出错。Rust 的生命周期系统在编译期就确保了这些引用始终有效。

### 2. `Result` 而非 `try/except` 处理文件操作

Python 用 `try/except FileNotFoundError` 处理文件 I/O，C 检查 `fopen` 返回 `NULL` 并读取 `errno`。Rust 的 `fs::read_to_string` 返回 `Result<String, io::Error>`，强制调用方处理两种可能。本项目的 `run` 函数用 `?` 运算符传播错误，用 `map_err` 为错误添加上下文（文件名）。关键区别：Rust 编译器不会让你"忘记"检查错误——如果一个函数返回 `Result`，你不处理它编译器就会警告或报错。Python 的异常和 C 的错误码都不会在编译期强制你处理所有的错误路径。

### 3. 模块组织与 Python 的"文件即模块"差异

Python 中每个 `.py` 文件自动成为一个模块（module），导入路径对应文件系统路径。Rust 的模块系统完全不同：文件本身不是自动模块，你需要通过 `mod` 关键字显式声明模块树。本项目中 `main.rs` 和 `lib.rs` 是 Cargo 约定的两个 crate 根（二进制根和库根），它们通过 `use cli_text_search::Foo` 而不是文件路径来引用。这种"显式声明"的设计让模块结构独立于文件系统布局，且在编译时就能检查模块图中的所有依赖关系。

### 4. 测试内建于编译器（`cargo test`），无需 pytest

Python 项目通常额外安装 `pytest` 才能获得结构化的测试体验。Rust 的测试框架直接内置于语言和编译器：`#[test]` 属性标记测试函数，`#[cfg(test)]` 控制测试代码的条件编译，`cargo test` 一条命令运行所有测试。本项目的单元测试（在 `lib.rs` 中）、集成测试（在 `tests/` 目录中）和文档测试（`///` 注释中的代码示例）三种层次全部无需额外依赖。编译期还能自动捕获测试代码中的类型错误，这在 Python 的"先运行测试才能发现语法错误"的流程中不存在。

### 5. 生命周期标注：借用的显式契约

`search<'a>` 中的 `'a` 是 Rust 特有的生命周期参数。Python 程序员不需要关心"这个引用指向的数据能活多久"，因为 GC 兜底；C 程序员需要精确管理但完全靠人工记忆和约定。Rust 取其折中：编译器自动推导大多数生命周期，但当返回引用的来源不明确时（两个引用参数，编译器不知道返回引用绑定于谁），就要求程序员显式标注。`Vec<&'a str>` 明确告诉所有读者：返回的引用借用于 `contents` 参数，在 `contents` 被释放前有效。这是类型系统层面的文档，不可违反、不可忽略。

---

## 总结

`cli_text_search` 是一个精心设计的教学项目，它在 ~300 行 Rust 代码中展示了现代系统编程的核心概念：所有权、借用、生命周期、错误处理、迭代器、测试和模块组织。虽然功能有限，但其架构设计遵循 Rust 生态最佳实践，代码质量达到了可发布的库的标准。

通过完成这个项目，学习者应该能够：

1. 独立搭建 library-first 的 Rust 项目
2. 理解并正确使用生命周期标注
3. 编写包含单元测试和集成测试的健壮代码
4. 使用 `Result` 和 `?` 进行惯用的错误处理
5. 阅读和理解 Rust 编译器关于借用和生命周期的错误信息

---

## 参考资料

- [Rust 程序设计语言 (The Book)](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- [ripgrep 源码](https://github.com/BurntSushi/ripgrep) — 高性能 Rust grep 实现
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
