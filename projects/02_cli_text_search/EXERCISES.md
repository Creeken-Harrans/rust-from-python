# CLI 文本搜索工具 (cli_text_search) — 练习指南

## 项目概述

你已阅读了 `README.md`、`src/lib.rs` 和 `src/main.rs`。这个简化版 `grep` 采用了 **library-first** 设计，核心搜索逻辑在 `lib.rs` 中。请在不直接复制粘贴的前提下完成以下练习。

---

## Level 1: 基础练习（理解现有代码）

### L1-1: 运行与观察

1. 创建一个测试文件 `test.txt`，包含中英文混合内容（至少 10 行）。
2. 运行 `cargo run -- "搜索词" test.txt` 执行大小写敏感搜索。
3. 运行 `cargo run -- "搜索词" test.txt -i` 执行大小写不敏感搜索。
4. 运行 `cargo test` 观察所有测试通过。
5. 尝试搜索一个不存在的文件，观察错误信息是否从 stderr 输出。

**学习点**: cargo run 参数传递（`--` 分隔符）、stdout vs stderr。

### L1-2: 追踪 `&str` 的生命周期

在 `search<'a>` 函数中：
- `query: &str` 的生命周期是什么？（编译器自动推断为匿名生命周期）
- `contents: &'a str` 的生命周期 `'a` 是谁标注的？（程序员显式标注）
- 返回值 `Vec<&'a str>` 的 `'a` 绑定到哪个参数？（`contents`）

回答以下问题：
1. 如果不写 `'a`，编译器能自动推断吗？在 `src/lib.rs` 中临时移除 `'a` 并运行 `cargo check` 观察编译错误。
2. 为什么返回值绑定到 `contents` 而不是 `query`？（提示：返回的行数据来自哪里？）
3. 将临时修改恢复后，运行 `cargo check` 确保编译通过。

**学习点**: 生命周期省略规则的限制、显式标注的必要性。

### L1-3: 理解 library-first 结构

回答以下问题：
1. `main.rs` 中通过 `use cli_text_search::SearchConfig;` 导入。这个路径名 `cli_text_search` 来自哪里？（提示：检查 `Cargo.toml` 的 `[package] name`）
2. 如果想把 `search()` 函数也放在 `main.rs` 中直接调用，为什么不行？（提示：Rust 不允许测试二进制 crate 中的函数）
3. `tests/integration_test.rs` 作为独立 crate 运行，它如何访问 `cli_text_search` 的 API？

**学习点**: Cargo crate 命名、二进制 vs 库 crate、集成测试隔离。

### L1-4: 添加 `-n` 标志（显示行号）

为工具添加 `-n` / `--line-number` 标志，使输出包含原始文件行号：

```
在文件 'test.txt' 中找到 2 行包含 'Rust' 的结果:

  1: (行 3) Rust 是一门系统编程语言
  2: (行 7) 学习 Rust 非常有趣
```

实现要点：
- `SearchConfig` 中添加 `line_numbers: bool` 字段
- `SearchConfig::new()` 中解析新标志
- 需要修改 `search()` 返回类型吗？当前返回 `Vec<&str>`，不包含行号。两种方案：
  - 方案 A: 改返回类型为 `Vec<(usize, &str)>`（含行号）
  - 方案 B: 在 `run()` 中用 `.enumerate()` 计算行号
- 方案 B 的问题：如果 `-n` 标记关闭，行号不需要被计算。但 `enumerate()` 本身零开销。

**学习点**: API 设计权衡、向后兼容性、迭代器 `enumerate()`。

### L1-5: 处理搜索词含空格的场景

当前实现中 `search()` 使用 `line.contains(query)` 做子串匹配。创建一个包含前导/尾随空行的测试文件，验证：
- 空行是否会被匹配？（不应该，因为行本身不包含内容）
- 仅含空格的行：`"   "` 是否被匹配？
- 如何改进 `search()` 以避免匹配纯空白行？

**学习点**: `str::trim()`、`str::is_empty()`、防御性编程。

---

## Level 2: 功能扩展（编写新代码）

### L2-1: 实现 `-c` 计数模式

添加 `-c` / `--count` 标志，输出匹配行数量而非行内容：

```
$ cargo run -- "Rust" test.txt -c
在文件 'test.txt' 中找到 3 行包含 'Rust' 的结果。
```

（不显示具体行内容）

实现要点：
- `SearchConfig` 添加 `count_only: bool`
- `run()` 中根据配置决定输出格式
- 在 `main.rs` 的参数解析中识别新标志

**学习点**: 输出格式控制、配置驱动行为。

### L2-2: 实现 `-v` 反向匹配

添加 `-v` / `--invert-match` 标志，显示**不**包含搜索词的行：

```
$ cargo run -- "Rust" test.txt -v
  1: Java 是另一门语言
  2: Python 也很流行
```

实现要点：
- `SearchConfig` 添加 `invert_match: bool`
- 在 `search()` 中将 `filter(|line| line.contains(query))` 改为条件过滤
- 不能直接用 `!line.contains(query)` 替换 —— 需要根据配置选择过滤逻辑

**两种实现方案**：
- 方案 A: 在 `search()` 中用 `if config.invert_match` 分支
- 方案 B: 调用时将 filter 闭包作为参数传入（函数式风格）

比较两种方案的可读性和可扩展性。

**学习点**: 闭包作为策略、函数式 vs 命令式。

### L2-3: 添加上下文行（`-A`、`-B`、`-C`）

实现类似 grep 的上下文行功能：

```
$ cargo run -- "Rust" test.txt -B 1 -A 1
  行2: 前面一行（不匹配但展示上下文）
  行3: Rust 是一门系统编程语言  ← 匹配行
  行4: 后面一行（不匹配但展示上下文）
```

实现要点：
- 不能只返回匹配行，需要知道匹配行的**索引**
- 收集所有匹配行的行号集合 `HashSet<usize>`
- 遍历所有行，如果行号在 [匹配行号 - B, 匹配行号 + A] 范围内则输出
- 处理相邻匹配行导致的上下文重叠（去重）

**学习点**: 窗口算法、HashSet 去重、迭代器 `enumerate()`。

### L2-4: 多文件搜索

修改程序支持同时搜索多个文件：

```
$ cargo run -- "Rust" file1.txt file2.txt file3.txt
```

实现要点：
- `SearchConfig.file_path: String` 改为 `file_paths: Vec<String>`
- 参数解析：`args[2..]` 中不以 `-` 开头的视为文件路径
- `run()` 中对每个文件调用搜索，输出时标注文件名
- 错误处理：一个文件读取失败不应阻止其他文件的搜索

**学习点**: Vec 解析、错误恢复策略、输出格式。

---

## Level 3: 设计思维（架构与扩展）

### L3-1: 正则表达式支持

当前使用 `str::contains()` 做简单子串匹配。设计引入 `regex` crate 的正则搜索方案：

```rust
// 期望的用法
cargo run -- -e "R.st" test.txt  // -e 表示用正则
cargo run -- -e "\d{4}-\d{2}-\d{2}" log.txt  // 搜索日期格式
```

设计要点：
- 新增 `SearchMode` 枚举：
  ```rust
  enum SearchMode {
      Literal(String),     // 当前的字面量搜索
      Regex(regex::Regex), // 正则搜索
  }
  ```
- 如何优化正则的重复编译？如果搜索 100 个文件都使用同一正则，应该在文件遍历之前编译一次。
- `Cargo.toml` 中如何添加 `regex` 依赖？

**学习点**: 枚举携带数据、正则编译与缓存、依赖管理。

### L3-2: 文件递归搜索

设计 `-r` / `--recursive` 标志，递归搜索目录中的所有文件：

```
$ cargo run -- "fn main" src/ -r
src/lib.rs:5: pub fn search<'a>(...
src/main.rs:3: fn main() {
tests/integration_test.rs:12: fn test_search() {
```

设计要点：
- 使用 `std::fs::read_dir` 还是引入 `walkdir` crate？
- 如何排除 `.git`、`target` 等目录？（`.gitignore` 规则？）
- 如何处理符号链接循环？
- 大目录中线程模型：每个文件一个线程 vs 线程池？

**不要求完整实现**，给出架构图和关键数据结构定义。

**学习点**: 文件系统遍历、递归、线程池设计。

### L3-3: 流式处理大文件

当前实现用 `fs::read_to_string()` 将整个文件读入内存。对于 GB 级日志文件这是不可行的。设计流式处理方案：

- 使用 `BufReader` 逐行读取，边读边搜索边输出
- 需要修改 `search()` 和 `run()` 的签名
- 流式版本的生命周期标注会有何不同？
- 输出顺序：如何保证多文件流式处理时的输出顺序？

**学习点**: BufReader、流式 vs 批处理、输出缓冲。

### L3-4: 插件式搜索策略

设计一个可扩展的搜索策略架构，允许用户自定义搜索行为：

```rust
trait SearchStrategy {
    fn matches(&self, line: &str) -> bool;
    fn name(&self) -> &str;  // 策略名称，用于日志
}

struct LiteralSearch { query: String }
struct RegexSearch { pattern: Regex }
struct FuzzySearch { query: String, threshold: f64 }
```

- 每个策略实现 `SearchStrategy` trait
- `SearchConfig` 持有 `Box<dyn SearchStrategy>`
- 如何通过命令行参数选择合适的策略？
- 策略组合：能否同时使用多个策略？（AND/OR 逻辑）

**学习点**: trait 对象、动态分发、策略模式。

---

## 检查清单

完成上述练习后，你应该能够：

- [ ] 理解 `lib.rs` 与 `main.rs` 的职责分离
- [ ] 手动标注生命周期参数 `'a` 并解释其绑定关系
- [ ] 正确选择 `&str` vs `String` 在函数参数和返回值中的使用
- [ ] 使用 `Box<dyn Error>` 统一不同类型错误
- [ ] 通过 `#[cfg(test)]` 编写单元测试和文档测试
- [ ] 扩展 `SearchConfig` 结构体添加新配置项
- [ ] 修改迭代器链 (`.lines().filter().collect()`) 实现新过滤逻辑
- [ ] 处理文件 I/O 错误而不 panic
- [ ] 设计 trait 对象替代硬编码搜索逻辑
- [ ] 评估流式处理的性能与实现复杂度权衡
