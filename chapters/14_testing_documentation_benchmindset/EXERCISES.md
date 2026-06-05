# 第14章练习题：测试、文档与性能思维

## 练习说明

以下练习题旨在帮助你巩固 Rust 测试和文档的知识。在每个练习中，你将扩展 `testing_and_docs` 库的功能，并为其编写测试和文档。

完成练习后运行 `cargo test` 确保所有测试通过。部分练习需要你同时提供文档测试和单元测试。

---

## 练习1：实现并测试除法函数

### 任务

在 `src/lib.rs` 中添加一个 `divide` 函数：

- 函数签名：`pub fn divide(a: f64, b: f64) -> Option<f64>`
- 如果 `b == 0.0`，返回 `None`
- 否则返回 `Some(a / b)`

### 要求

1. 为 `divide` 编写完整的文档注释（包含 # Examples）
2. 在 `#[cfg(test)]` 模块中添加以下测试：
   - 正常除法：`divide(10.0, 2.0)` 应返回 `Some(5.0)`
   - 除以零：`divide(5.0, 0.0)` 应返回 `None`
   - 使用 `assert_eq!` 进行验证
3. 确保文档测试和单元测试都能通过

### 提示

`Option<f64>` 的比较需要注意浮点数精度。可以使用 `assert_eq!(result.unwrap(), 5.0)` 或考虑使用近似比较。

---

## 练习2：实现并测试字符串截断函数

### 任务

在 `src/lib.rs` 中添加一个 `truncate_text` 函数：

- 函数签名：`pub fn truncate_text(text: &str, max_chars: usize) -> &str`
- 如果 `text` 的字符数 <= `max_chars`，返回原字符串
- 如果超过，截取前 `max_chars` 个字符并返回截断后的切片

### 要求

1. 编写完整的文档注释（/// 形式）
2. 编写以下单元测试：
   - 文本短于限制：返回原字符串
   - 文本长于限制：返回截断后的字符串
   - 空字符串输入
   - 恰好等于限制长度
3. 在文档注释中包含一个会 panic 的示例（当 `max_chars` 恰好切在 UTF-8 字符中间）
4. 使用 `#[should_panic]` 测试这个 panic 情况

### 提示

Rust 的字符串切片是按字节操作 `&str[..n]`，注意 UTF-8 边界问题。考虑使用 `chars()` 迭代器。

---

## 练习3：编写集成测试

### 任务

在 `tests/` 目录中创建一个新文件 `workflow_test.rs`，编写一个模拟真实使用场景的集成测试。

### 要求

1. 创建 `tests/workflow_test.rs`
2. 编写 `#[test] fn test_text_processing_workflow()`：
   - 使用 `word_count` 统计一段文本的单词数
   - 使用 `is_palindrome` 检查其中的每个单词
   - 使用你实现的 `truncate_text` 截断长单词
   - 验证整个工作流的结果
3. 至少包含 3 个断言

### 提示

集成测试只能访问公共 API。考虑一个典型的文本处理流程：计数 -> 分析 -> 转换。

---

## 练习4：编写返回 Result 的测试

### 任务

为 `is_palindrome` 函数编写一个返回 `Result<(), String>` 的测试。

### 要求

1. 测试名称为 `test_palindrome_with_result`
2. 使用 `?` 操作符或显式返回 `Err` 来处理失败情况
3. 测试至少 4 个不同的字符串
4. 测试失败时提供有意义的错误消息

### 提示

```rust
#[test]
fn test_with_result() -> Result<(), String> {
    if !condition {
        return Err(String::from("reason"));
    }
    Ok(())
}
```

---

## 练习5：编写文档并验证文档测试

### 任务

为 `word_count` 函数扩展文档注释，使其包含更全面的文档测试。

### 要求

1. 在文档中至少添加 3 个不同的 `# Examples` 代码块
2. 包含一个使用 `assert_eq!` 的示例
3. 包含一个使用 `assert_ne!` 的示例
4. 运行 `cargo test --doc` 验证文档测试通过

### 提示

可以在现有文档注释基础上扩展，不需要创建新函数。

---

## 练习6：理解 should_panic 的 expected 参数

### 任务

在 `src/lib.rs` 中添加一个 `safe_sqrt` 函数：

- 函数签名：`pub fn safe_sqrt(x: f64) -> f64`
- 如果 `x < 0.0`，调用 `panic!("Cannot compute square root of negative number: {}", x)`
- 否则返回 `x.sqrt()`

### 要求

1. 编写完整的文档注释
2. 编写一个 `#[should_panic(expected = "negative")]` 测试
3. **故意**再写一个 `#[should_panic(expected = "wrong message")]` 的测试，观察它是否失败
4. 运行测试，理解 `expected` 参数的匹配机制

---

## 练习7：cargo test 命令行实践

### 任务

使用命令行完成以下操作，记录每个命令的效果：

1. 运行所有测试：`cargo test`
2. 只运行名称包含 "add" 的测试：`cargo test add`
3. 只运行单元测试（而不是集成测试）：`cargo test --lib`
4. 只运行集成测试：`cargo test --test integration_test`
5. 显示测试输出：`cargo test -- --nocapture`
6. 单线程运行测试：`cargo test -- --test-threads=1`
7. 列出所有测试但不运行：`cargo test -- --list`

### 要求

在 `EXERCISES.md`（本文件）末尾添加一段简短说明，描述每个命令执行后的结果和你观察到的现象。

---

## 练习8：生成并浏览文档

### 任务

1. 运行 `cargo doc` 生成文档
2. 运行 `cargo doc --open` 在浏览器中查看文档
3. 找到你编写的函数文档
4. 观察 `# Examples` 部分的展示效果

### 要求

回答以下问题（写入本文件末尾）：
- `///` 和 `//!` 生成的文档有什么区别？
- 文档中显示的代码示例是从哪里来的？
- 如果不写文档注释，Rustdoc 会如何处理？

---

## 练习9：性能思维 —— 基准测试入门

### 任务（选做，需要 nightly Rust）

如果你安装了 nightly 版本的 Rust，可以尝试基准测试：

```bash
rustup install nightly
```

在项目中创建 `benches/my_benchmark.rs`：

```rust
#![feature(test)]
extern crate test;
use test::Bencher;
use testing_and_docs;

#[bench]
fn bench_word_count(b: &mut Bencher) {
    let text = "The quick brown fox jumps over the lazy dog";
    b.iter(|| testing_and_docs::word_count(text));
}
```

运行基准测试：

```bash
cargo +nightly bench
```

### 要求

1. 观察基准测试的输出格式
2. 思考：如果不做基准测试，你如何"证明"某个实现更快？
3. 写下你对"过早优化"这一概念的理解（写入本文件末尾）

---

## 练习10：测试 "zero cost" 验证

### 任务

验证测试代码确实不会出现在发布构建中：

1. 运行 `cargo build --release`
2. 使用 `nm` 或 `strings` 检查编译产物，搜索测试函数名
3. 确认测试代码已被移除

```bash
cargo build --release
nm target/release/libtesting_and_docs.rlib | grep test_ 2>/dev/null || echo "未找到测试符号（符合预期）"
```

### 要求

记录你的观察结果，理解 `#[cfg(test)]` 的实际效果。

---

## 答案记录区

请在完成练习后将你的答案和观察记录在下方：

### 练习7 观察结果

```
（在此记录 cargo test 各选项的执行结果）
```

### 练习8 问题回答

```
（在此回答关于 Rustdoc 的问题）
```

### 练习9 性能思维

```
（在此写下你对过早优化的理解）
```

### 练习10 验证结果

```
（在此记录你的观察）
```
