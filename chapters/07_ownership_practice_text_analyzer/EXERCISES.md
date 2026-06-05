# Chapter 07 练习题：所有权实战 —— 文本分析器

本章练习题要求你修改和扩展 `text_analyzer` 项目。每道题都围绕所有权、借用、切片的正确使用。

**所有练习都在 `/chapters/07_ownership_practice_text_analyzer/` 目录下完成。**

---

## 环境准备

```bash
# 确认项目能编译运行
cd rust-from-python
cargo build -p text_analyzer
cargo run -p text_analyzer

# 运行 clippy（Rust 官方 linter）检查代码质量
cargo clippy -p text_analyzer

# 查看文档（包括你写的 doc comments）
cargo doc -p text_analyzer --open
```

---

## Level 1：基础练习（理解所有权与借用）

### 练习 1-1：添加 `count_lines` 函数

**任务**：在 `src/main.rs` 中添加一个 `count_lines(text: &str) -> usize` 函数，统计文本中的行数（以 `\n` 为分隔）。

**要求**：
1. 参数必须使用 `&str`（不要用 `String`）
2. 在函数上方添加 `///` 文档注释，解释为什么用 `&str` 而不是 `String`
3. 在文档注释中指出如果用了 `String` 参数会出现什么问题
4. 在 `main()` 中调用此函数并打印结果

**提示**：
```rust
/// 统计文本中的行数...
fn count_lines(text: &str) -> usize {
    text.lines().count()  // lines() 按换行符分割，返回每行的 &str
}
```

**验证**：
```bash
cargo build -p text_analyzer && cargo run -p text_analyzer
# 应该看到新的行数统计输出
```

---

### 练习 1-2：添加 `average_word_length` 函数

**任务**：添加 `average_word_length(text: &str) -> f64` 函数，计算所有单词的平均字符长度。

**要求**：
1. 参数用 `&str`
2. 返回值用 `f64`（浮点数，不属于借用类型，自然拥有所有权）
3. 如果文本为空（没有单词），返回 0.0
4. 在文档注释中标注：为什么 `f64` 不需要考虑借用 vs 所有权

**提示**：
- 使用 `split_whitespace()` 获取单词迭代器
- 对每个单词用 `.chars().count()` 统计字符数
- 注意整数除法会截断，需要转换为 `f64`

**验证**：
```bash
cargo run -p text_analyzer
# 输出：平均单词长度: X.XX 字符
```

---

### 练习 1-3：添加 `shortest_word` 函数

**任务**：添加 `shortest_word(text: &str) -> Option<&str>` 函数，返回最短的单词。

**要求**：
1. 签名模仿 `find_longest_word`：参数 `&str`，返回 `Option<&str>`
2. 在文档注释中解释：**为什么返回 `Option<&str>` 而不是 `Option<String>`**
3. 使用 `min_by_key` 而非 `max_by_key`
4. 如果文本没有单词，返回 `None`

**问题（在注释中回答）**：
- 如果有多个单词长度相同且都是最短，`min_by_key` 返回哪一个？（回答：第一个）
- 如果改成 `Option<String>` 会多做什么操作？（回答：不必要的 `.to_string()` 堆分配）

**验证**：
```bash
cargo run -p text_analyzer
# 输出：最短单词: "a"（或其他最短的）
```

---

## Level 2：进阶练习（综合设计决策）

### 练习 2-1：添加 `find_words_containing` 函数并论证设计

**任务**：添加 `find_words_containing(text: &str, substring: &str) -> Vec<&str>` 函数。

**功能**：返回所有包含指定子串的单词列表（借用切片）。

**要求**：
1. 先写函数签名，再写函数体
2. 在文档注释中完整论证以下设计决策：

   **a) 为什么两个参数都用 `&str`？**
   - 提示：两个参数都只需要读取，不需要拥有
   - 如果第一个参数用了 `String`，调用者还能用原数据做什么？（不能做任何事，所有权转移了）

   **b) 为什么返回 `Vec<&str>` 而不是 `Vec<String>`？**
   - 提示：每个元素都是输入文本的一部分
   - 想想文本有 10000 个单词，其中 500 个匹配：如果返回 `Vec<String>` 要做多少次堆分配？（500 次）

   **c) 为什么返回 `Vec<&str>` 而不是 `&[&str]`？**
   - 提示：结果集合是函数内部构建的新 Vec
   - 如果返回 `&[&str]`，它引用谁？函数内部的局部变量？函数返回后局部变量还在吗？

3. 在 `main()` 中调用此函数，搜索包含某个子串的单词并打印
4. 测试边界情况：空文本、没有单词匹配、所有单词都匹配

**验证**：
```bash
cargo build -p text_analyzer && cargo run -p text_analyzer
# 应该看到匹配的单词列表
```

**扩展思考（可选，写在注释中）**：
如果 `text` 是函数内部创建的临时 `String`，而函数返回了 `Vec<&str>`，会发生什么？
（回答：编译器会报错，因为返回的引用生命周期超过了被引用数据的生命周期。）

---

### 练习 2-2：添加 `find_palindromic_words` 函数

**任务**：添加 `find_palindromic_words(text: &str) -> Vec<&str>` 函数，返回所有回文单词。

**回文定义**：正读反读相同的单词（如 "radar", "level", "noon"）。忽略大小写。

**要求**：
1. 参数：`&str`
2. 返回：`Vec<&str>`（为什么不是 `Vec<String>`？因为单词是输入文本的一部分）
3. 实现回文检查逻辑（在 `main.rs` 中添加一个辅助函数 `is_palindrome(word: &str) -> bool`）
4. 在文档注释中说明：**辅助函数 `is_palindrome` 为什么参数用 `&str`**

**提示**：
```rust
fn is_palindrome(word: &str) -> bool {
    let lower: String = word.to_lowercase();  // 这里需要 to_lowercase() 返回 String
    // 思考：为什么 to_lowercase() 返回 String 而不是 &str？
    // 回答：因为大小写转换可能改变字符数量和 UTF-8 编码，
    // 需要新的内存分配（如 ß → SS）
    let reversed: String = lower.chars().rev().collect();
    lower == reversed
}
```

**验证**：
```bash
cargo run -p text_analyzer
```

---

## Level 3：重构练习（架构级思考）

### 练习 3-1：将分析函数重构为 `TextAnalyzer` 结构体

**任务**：在 `src/main.rs` 中定义 `TextAnalyzer` 结构体，将分析函数重构为它的方法。保留原有的独立函数作为对比。

**要求**：

1. **定义结构体**：
```rust
struct TextAnalyzer {
    text: String,  // 分析器拥有文本的所有权
}
```

2. **实现构造器 `new`**：
```rust
impl TextAnalyzer {
    /// 创建一个新的分析器。
    /// 参数用 String：分析器需要拥有数据的所有权，
    /// 这样它可以在任何地方被使用，不受外部数据生命周期的限制。
    fn new(text: String) -> Self {
        TextAnalyzer { text }
    }
}
```

3. **将至少 4 个函数改写为方法**（`count_chars`, `count_words`, `find_longest_word`, `first_word`）：

```rust
impl TextAnalyzer {
    /// 为什么参数是 &self 而不是 self？
    /// 因为方法只需要读取 self.text，不需要消耗 self。
    fn count_chars(&self) -> usize {
        self.text.chars().count()
    }

    /// 为什么返回 Option<&str>？
    /// 返回的引用指向 self.text（由结构体拥有），
    /// 生命周期与 &self 借用相同。
    fn find_longest_word(&self) -> Option<&str> {
        self.text.split_whitespace()
            .max_by_key(|w| w.chars().count())
    }
}
```

4. **在文档注释中回答**：
   - 为什么 `new` 的参数是 `String` 而不是 `&str`？
   - 为什么方法用 `&self` 而不是 `self` 作为第一个参数？
   - `find_longest_word` 返回 `Option<&str>` 时，编译器如何保证引用的安全性？
   - 如果有人尝试在 `&self` 方法中修改 `self.text`，编译器会怎样？

5. **在 `main()` 中同时演示两种用法**——旧版独立函数和新版结构体方法。

**验证**：
```bash
cargo build -p text_analyzer
cargo run -p text_analyzer
# 应该看到两种用法的输出
```

---

## 思考题

### 思考题：函数签名中的生命周期权衡

阅读以下两个函数签名：

```rust
// 版本 A：借用输入，借用输出
fn extract_words_a(text: &str) -> Vec<&str> { ... }

// 版本 B：获取所有权，返回所有权
fn extract_words_b(text: String) -> Vec<String> { ... }
```

**问题**：

1. 版本 A 有什么**优势**？（提示：性能、灵活性）

2. 版本 A 有什么**限制**？（提示：当原 `text` 被释放后，返回的 `Vec<&str>` 还能用吗？）

3. 什么场景下你会**必须**使用版本 B 而不是版本 A？
   （提示：如果 `text` 来自一个 HTTP 响应，你需要在函数返回后释放原始缓冲区……）
   （回答：当输入数据生命周期太短，无法满足输出引用的生命周期要求时）

4. 如果 `extract_words_a` 接受 `&str`，但调用者只有一个 `String`，编译器会怎样？
   （回答：自动通过 deref coercion 将 `&String` 转换为 `&str`，一切正常。）

5. 如果 `extract_words_b` 的参数改成 `&str`，但保留 `Vec<String>` 返回值，这个设计如何评价？
   （回答：参数选择正确（借用），返回值也可以接受（内部构建新数据），这是合理的折中。）

---

## 推荐的学习流程

1. **先完成 Level 1 的全部练习**（估计用时：30-45 分钟）
   - 这三个练习让你熟悉基本的参数/返回值选择模式

2. **再完成 Level 2 的两个练习**（估计用时：45-60 分钟）
   - 这两个练习要求你**论证设计决策**，不只是写代码
   - 写文档注释的过程就是巩固理解的过程

3. **最后完成 Level 3 的重构练习**（估计用时：60-90 分钟）
   - 这个练习引入结构体和方法，是 Chapter 08 的预热
   - 重点理解 `&self` 和 `self` 的区别

4. **阅读思考题并在注释中写下答案**（估计用时：15-20 分钟）
   - 思考题帮你从更高层次理解生命周期权衡

---

## 常用命令速查

```bash
# 编译本章代码
cargo build -p text_analyzer

# 编译并运行
cargo run -p text_analyzer

# 检查代码风格和潜在问题
cargo clippy -p text_analyzer

# 运行 clippy 并将警告视为错误（严格要求）
cargo clippy -p text_analyzer -- -D warnings

# 格式化代码
cargo fmt -p text_analyzer

# 生成并查看文档
cargo doc -p text_analyzer
cargo doc -p text_analyzer --open

# 仅检查编译（不生成二进制，更快）
cargo check -p text_analyzer
```

---

## 常见问题 FAQ

**Q: 我的代码编译失败，错误信息是 "cannot return value referencing local variable"，怎么办？**

A: 这个错误意味着你试图返回一个指向函数内部局部变量的引用。检查你的返回值：如果它引用了函数内部创建的 `String` 或 `Vec`，改成返回拥有所有权的类型（`String` 或 `Vec`），或者确保引用指向的是参数数据。

**Q: 什么时候用 `split_whitespace()` 什么时候用 `split(' ')`？**

A: `split_whitespace()` 更好，因为它能处理多个连续空格、制表符（`\t`）、换行符（`\n`）等。`split(' ')` 只能处理单个空格，多个连续空格会产生空字符串元素。

**Q: 我的函数返回 `Vec<&str>`，但编译器说 lifetime mismatch，怎么回事？**

A: 检查 `Vec` 中的 `&str` 引用的是哪里的数据。它们应该引用传入的参数数据，而不是函数内部创建的临时变量。如果数据来源没问题，检查是否省略了返回值中元素的类型标注。

**Q: 为什么我不能在已经从 `String` 借用了 `&str` 的同时修改 `String`？**

A: 这是 Rust 的借用规则：不能同时存在不可变借用和可变借用。这是为了防止数据竞争和悬垂指针。当你有 `&str` 指向一个 `String` 时，该 `String` 被"冻结"——不能修改、不能 drop，直到所有借用结束。
