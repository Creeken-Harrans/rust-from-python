# 第14章：测试、文档与性能思维

## 本章目标

本章深入探讨 Rust 生态中三个密切相关的核心实践：**测试（Testing）**、**文档（Documentation）** 和**性能思维（Benchmarking Mindset）**。通过本章学习，你将：

1. 掌握 Rust 的单元测试编写方法，理解 `#[test]` 属性和 `#[cfg(test)]` 条件编译
2. 理解集成测试的组织方式和外部消费者视角
3. 学会编写文档测试，让代码示例既能作为文档又能作为测试运行
4. 熟练使用各种断言宏：`assert!`、`assert_eq!`、`assert_ne!`
5. 掌握 `#[should_panic]` 的用法和预期错误消息匹配
6. 了解返回 `Result<T, E>` 的测试函数
7. 熟悉 `cargo test` 的各种命令行选项
8. 理解 Rustdoc 工具和文档注释规范
9. 建立"先测量，再优化"的性能思维
10. 理解 Rust 社区对测试的高度重视

Rust 的这些工具不是事后补救的手段，而是语言设计的一等公民。它们与编译器深度集成，让你可以在编写代码的同时验证正确性。

---

## 单元测试（Unit Test）

### 基本概念

单元测试是对代码中最小可测试单元（通常是单个函数或方法）进行验证的测试。在 Rust 中，单元测试通常与源代码放在同一个文件中，放在一个名为 `tests` 的模块里。

### #[test] 属性

使用 `#[test]` 属性标记一个函数为测试函数。当运行 `cargo test` 时，Rust 会自动发现并执行所有带有此属性的函数。

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

每个测试函数都在独立的线程中运行，一个测试的失败不会影响其他测试的执行。

### #[cfg(test)] 条件编译

`#[cfg(test)]` 是一个条件编译属性，它告诉编译器只有在运行测试时才编译被标记的模块或函数。这意味着测试代码不会被编译到最终的二进制文件中，从而不会增加发布版本的体积。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // 测试代码
    }
}
```

关键点：
- `#[cfg(test)]` 标注的代码只在 `cargo test` 时编译
- `cargo build` 不会编译这些代码
- 这确保了发布版本不包含测试逻辑

### 测试私有函数

Rust 的测试可以访问私有函数。因为测试模块通常使用 `use super::*;` 导入父模块的所有内容（包括私有项），所以你可以直接测试那些不对外暴露的内部函数。

```rust
// 私有函数 —— 不对外暴露
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_private() {
        assert_eq!(reverse_string("hello"), "olleh");
    }
}
```

这与一些其他语言不同 —— 在 Rust 中，测试是代码的一等公民，可以访问模块的私有内部。

---

## 断言宏（Assertions）

### assert!

`assert!` 宏验证一个布尔表达式为 `true`。如果表达式为 `false`，测试会 panic 并失败。

```rust
#[test]
fn test_assert() {
    let result = is_palindrome("radar");
    assert!(result);  // 测试通过
    assert!(!is_palindrome("hello"));  // 也通过
}
```

### assert_eq! 和 assert_ne!

`assert_eq!` 验证两个值相等。`assert_ne!` 验证两个值不相等。

```rust
#[test]
fn test_equality() {
    assert_eq!(add(2, 3), 5);       // 相等
    assert_ne!(add(2, 3), 6);       // 不相等
}
```

这两个宏在断言失败时会打印两个值的调试表示，帮助快速定位问题。它们要求被比较的类型实现 `PartialEq` 和 `Debug` trait。

### 自定义错误消息

所有断言宏都支持在最后添加自定义错误消息：

```rust
#[test]
fn test_with_message() {
    let result = add(2, 3);
    assert_eq!(result, 5, "期望 add(2, 3) 返回 5，但得到了 {}", result);
}
```

---

## #[should_panic] 属性

### 基本用法

`#[should_panic]` 属性用于标记那些预期会 panic 的测试。当被测试的代码按预期 panic 时，测试通过。如果代码没有 panic，测试反而失败。

```rust
#[test]
#[should_panic]
fn test_multiply_panics_on_negative() {
    multiply(-1, 5);  // 预期这里会 panic
}
```

### 匹配预期的错误消息

你可以使用 `expected` 参数来指定 panic 消息中必须包含的文本片段。这比单纯检查是否 panic 更精确 —— 它确保了 panic 是由你预期的原因触发的。

```rust
#[test]
#[should_panic(expected = "negative")]
fn test_multiply_panics_on_negative() {
    multiply(-1, 5);
}
```

如果代码 panic 了但消息不包含 "negative"，测试仍然会失败。这帮助你区分不同类型的 panic。

---

## 返回 Result<T, E> 的测试

除了让测试 panic 来表示失败，你还可以让测试函数返回 `Result<T, E>`。当测试返回 `Ok(())` 时表示通过，返回 `Err(...)` 时表示失败。

```rust
#[test]
fn test_as_result() -> Result<(), String> {
    if word_count("one two three") != 3 {
        return Err(String::from("word_count should return 3"));
    }
    Ok(())
}
```

这种方式的优势：
- 可以在测试中使用 `?` 操作符
- 失败时提供更结构化的错误信息
- 适合那些本身返回 `Result` 的函数

注意：使用 `Result` 的测试不能同时使用 `#[should_panic]`。

---

## 集成测试（Integration Test）

### 概念

集成测试从外部使用者的角度测试你的库。它们位于项目根目录的 `tests/` 目录中，每个 `.rs` 文件都被编译为独立的 crate。

### 与单元测试的区别

| 方面 | 单元测试 | 集成测试 |
|------|---------|---------|
| 位置 | 与源代码同文件 | `tests/` 目录 |
| 访问权限 | 可访问私有项 | 只能访问公共 API |
| 编译方式 | 作为 crate 的一部分 | 作为独立的 crate |
| 视角 | 内部实现视角 | 外部消费者视角 |

### 编写集成测试

在 `tests/` 目录中创建一个文件：

```rust
use testing_and_docs;

#[test]
fn integration_test_add_and_multiply() {
    let sum = testing_and_docs::add(5, 7);
    let product = testing_and_docs::multiply(sum, 2);
    assert_eq!(product, 24);
}
```

注意事项：
- 每个 `tests/` 下的文件都是独立的 crate，需要显式 `use` 你的库
- 只能测试公共 API
- 如果需要在多个集成测试文件中共享辅助代码，使用 `tests/common/mod.rs` 模式

### 共享测试辅助模块

```text
tests/
├── common/
│   └── mod.rs    # 共享辅助函数
├── integration_test.rs
└── another_test.rs
```

`tests/common/mod.rs` 中的函数不会被当作测试运行，因为只有 `tests/` 根目录下的 `.rs` 文件才会被识别为测试 crate。

### 运行特定集成测试

```bash
cargo test --test integration_test
```

---

## 文档测试（Documentation Test）

### 概念

Rust 最具创新性的特性之一：**文档注释中的代码示例会自动作为测试运行**。这意味着你的文档永远不会过时 —— 如果代码示例不再有效，`cargo test` 就会失败。

### 两种文档注释

#### /// - 行级文档注释

用于文档化紧跟在其后的项（函数、结构体、模块等）。

```rust
/// 返回两个整数的和。
///
/// # Examples
///
/// ```
/// let result = testing_and_docs::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`# Examples` 是一个常用的文档章节标记，其他常用章节包括 `# Panics`、`# Errors`、`# Safety`。

#### //! - 模块级文档注释

用于文档化包含它的模块或 crate。

```rust
//! # testing_and_docs
//!
//! 一个用于演示 Rust 测试和文档的小型工具库。
//!
//! # Examples
//!
//! ```
//! let result = testing_and_docs::add(2, 3);
//! assert_eq!(result, 5);
//! ```
```

`//!` 通常放在文件开头（如 `src/lib.rs` 或 `src/main.rs`），用于描述整个模块或 crate。

### 特殊的文档测试属性

#### should_panic

在文档中标注某段代码预期会 panic：

````rust
/// # Panics
///
/// 当任一参数为负数时 panic
///
/// ```should_panic
/// testing_and_docs::multiply(-1, 5);
/// ```
````

#### no_run

编译但不运行代码示例：

````rust
/// ```no_run
/// // 这个示例会被编译但不会运行
/// let result = some_network_call();
/// ```
````

#### compile_fail

标记代码不应编译通过（用于演示编译错误）：

````rust
/// ```compile_fail
/// let x: u32 = "not a number";
/// ```
````

#### ignore

完全忽略代码示例（不编译也不运行）：

````rust
/// ```ignore
/// // 这只是伪代码演示
/// ```
````

---

## cargo test 命令详解

### 运行所有测试

```bash
cargo test
```

### 按名称过滤

```bash
# 只运行名称中包含 "add" 的测试
cargo test add

# 只运行特定测试函数
cargo test test_add

# 只运行特定模块的测试
cargo test tests::
```

### 显示输出

默认情况下，测试通过的输出会被捕获并隐藏。使用 `--nocapture` 或 `--show-output` 来显示 `println!` 输出：

```bash
cargo test -- --nocapture
cargo test -- --show-output
```

### 控制测试线程

默认情况下测试并行运行。可以控制线程数：

```bash
# 单线程顺序运行
cargo test -- --test-threads=1

# 指定线程数
cargo test -- --test-threads=4
```

单线程运行有助于调试那些相互影响的测试。

### 运行被忽略的测试

使用 `#[ignore]` 属性标记耗时较长的测试：

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 耗时操作
}
```

单独运行被忽略的测试：

```bash
cargo test -- --ignored
```

### 运行特定类型的测试

```bash
# 只运行单元测试
cargo test --lib

# 只运行特定集成测试文件
cargo test --test integration_test

# 只运行文档测试
cargo test --doc
```

### 常用组合

```bash
# 过滤并显示输出，单线程运行
cargo test palindrome -- --nocapture --test-threads=1

# 运行所有测试包括被忽略的
cargo test -- --include-ignored
```

---

## Rustdoc：文档生成工具

### 基本使用

Rustdoc 是 Rust 的内置文档生成工具，可以将文档注释转换为 HTML 文档：

```bash
# 生成文档
cargo doc

# 生成文档并包含私有项
cargo doc --document-private-items

# 生成文档并在浏览器中打开
cargo doc --open
```

生成的文档位于 `target/doc/` 目录。

### 文档约定

Rust 社区有完善的文档注释约定，常用的章节包括：

| 章节 | 用途 |
|------|------|
| `# Examples` | 代码使用示例 |
| `# Panics` | 函数可能 panic 的情况 |
| `# Errors` | 返回 Result 时可能的错误类型 |
| `# Safety` | unsafe 函数的调用前置条件 |
| `# See Also` | 相关函数或类型的引用 |
| `# Note` | 补充说明 |

### 文档链接

使用 `[text][reference]` 或 `` `TypeName` `` 语法创建文档内链接：

```rust
/// 返回两个数的和。另见 [`multiply`]。
///
/// [`multiply`]: fn@multiply
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## 性能思维（Performance Mindset）

### 黄金法则：先测量，再优化

> "Premature optimization is the root of all evil." — Donald Knuth
> "过早优化是万恶之源。"

在你开始优化代码之前，必须先用数据确认瓶颈所在。猜测性能问题是开发者最常见也最昂贵的错误之一。

### Rust 的性能哲学

Rust 的性能哲学可以概括为以下几点：

1. **零成本抽象（Zero-Cost Abstractions）**：高级抽象不应该带来运行时开销。Rust 的迭代器、闭包等在编译后与手写循环没有性能差异。

2. **默认栈分配**：Rust 默认在栈上分配数据，避免不必要的堆分配开销。

3. **无 GC 停顿**：通过所有权系统管理内存，没有垃圾回收的运行时开销。

4. **编译时求值**：尽可能在编译时完成计算（如 `const fn`）。

### 测量工具

- **cargo bench**：内置基准测试框架（需 nightly 或使用 criterion 等第三方库）
- **perf /火焰图**：Linux 下的性能分析工具
- **cargo instruments**：macOS 上的性能分析
- **heaptrack / valgrind**：内存分析工具

### 不要过早优化的原因

1. 你的直觉关于瓶颈的判断通常是错的
2. 优化过的代码通常更难理解和维护
3. 在错误的路径上优化是浪费时间
4. 编译器优化可能比你手动的"优化"更好
5. 代码的正确性远比微小的性能提升重要

### 何时考虑优化

- 你通过实际测量确认了性能瓶颈
- 性能问题影响了用户体验
- 代码处于成熟稳定阶段，API 不太会改变
- 你有明确的性能目标和指标

---

## Rust 的测试文化

### 社区价值观

Rust 社区对测试有着罕见的热情和高标准。这源于以下几个因素：

1. **编译器的安全性承诺**：Rust 保证内存安全和线程安全，但逻辑正确性需要通过测试来保证。

2. **工具链集成**：测试是 `cargo` 的原生功能，不是第三方插件。创建新项目后 `cargo test` 开箱即用。

3. **文档作为测试**：文档测试的发明意味着你写的每个文档示例都会被执行验证。这天然地鼓励了高质量文档。

4. **零成本**：测试代码在发布构建中完全不存在，没有性能惩罚。

5. **编译器驱动开发**：Rust 编译器检查非常严格，结合测试可以极大减少运行时错误。

### Rust vs Python 测试对比

| 方面 | Rust | Python |
|------|------|--------|
| 测试框架 | 内置（cargo test） | 第三方（pytest, unittest） |
| 发现机制 | 编译时自动发现 #[test] | 约定命名（test_*.py） |
| 文档测试 | 原生支持 | doctest 模块 |
| 私有函数测试 | 同一 crate 内可测试 | 可通过 `_` 前缀约定访问 |
| 并行执行 | 默认多线程 | 需配置（pytest-xdist） |
| 编译检查 | 测试代码也需通过类型检查 | 运行时才知道类型错误 |
| 发布构建 | 测试代码完全移除 | 测试文件仍可被导入 |

---

## 核心术语表

| 术语 | 英文 | 说明 |
|------|------|------|
| 单元测试 | Unit Test | 对最小可测试单元的验证，与源码同文件 |
| 集成测试 | Integration Test | 从外部使用者视角测试公共 API，位于 tests/ 目录 |
| 文档测试 | Documentation Test | 文档注释中的代码示例，自动作为测试运行 |
| 测试属性 | #[test] | 标记函数为测试函数 |
| 条件编译 | #[cfg(test)] | 只在测试时编译代码 |
| 断言 | assert! / assert_eq! / assert_ne! | 验证条件的宏 |
| 预期 panic | #[should_panic] | 标记测试预期 panic |
| Rustdoc | Rustdoc | Rust 内置文档生成工具 |
| 行文档 | /// | 文档化紧跟的项 |
| 模块文档 | //! | 文档化包含它的模块 |
| 基准测试 | Benchmark | 测量代码性能的测试 |

---

## 本章项目结构

```text
testing_and_docs/
├── Cargo.toml
├── src/
│   ├── lib.rs        # 库代码 + 单元测试
│   └── main.rs       # 简单运行器
├── tests/
│   └── integration_test.rs  # 集成测试
├── README.md         # 本章说明（本文件）
└── EXERCISES.md      # 练习题
```

运行以下命令体验 Rust 的测试生态：

```bash
# 运行所有测试（单元测试 + 集成测试 + 文档测试）
cargo test

# 查看完整输出
cargo test -- --nocapture

# 生成并查看文档
cargo doc --open

# 只运行文档测试
cargo test --doc
```
