# Chapter 07 练习题解答：所有权实战 —— 文本分析器

---

## Level 1：基础练习（理解所有权与借用）

---

### 练习 1-1：添加 `count_lines` 函数

#### 结论

使用 `&str` 作为参数，不需要获取数据的所有权，只需借用读取。

#### 思路

`count_lines` 只需要统计行数，不需要修改或持有数据。`&str` 是不可变借用，调用者保留所有权，函数只是临时借用。如果用 `String`，调用者要么必须 `clone()` 浪费内存，要么交出所有权后不可再用。

#### 参考实现

```rust
/// 统计文本中的行数（以 `\n` 为分隔）。
///
/// # 为什么参数用 `&str` 而不是 `String`？
///
/// 该函数只需要读取文本内容，不持有、不修改数据。使用 `&str`（不可变借用）：
/// - 调用者保留数据的**所有权**，可以在调用后继续使用原数据
/// - 不需要 `.clone()` 产生额外堆分配
/// - 兼容 `&str` 和 `&String`（通过 deref coercion 自动转换）
///
/// # 如果用 `String` 参数会出现什么问题？
///
/// ```text
/// // [错误设计] fn count_lines(text: String) -> usize { ... }
/// ```
///
/// 问题：
/// 1. **所有权转移**：调用者交出 `text` 的所有权，调用后无法再使用该数据
/// 2. **强制克隆**：如果调用者还想保留数据，必须提前 `clone()`，造成不必要的堆分配
/// 3. **灵活性降低**：不能接受 `&str` 或字符串字面量（它们是 `&str`，无法直接传给 `String` 参数）
fn count_lines(text: &str) -> usize {
    text.lines().count()
}
```

在 `main()` 中的调用：

```rust
let char_count = count_chars(text);
let line_count = count_lines(text);
println!("  行数: {}", line_count);
```

#### 为什么这样设计

- **借用 > 所有权转移**：函数只需要读取数据时，优先用引用
- **`&str` > `&String`**：`&str` 更灵活，通过 deref coercion 同时接受 `&str` 和 `&String`
- **零成本抽象**：不会产生额外的堆分配

#### 常见错误

1. **参数用了 `String`**：调用者必须转移所有权，违反最小权限原则
2. **参数用了 `&String`**：虽然不会转移所有权，但不如 `&str` 灵活（不能直接传字符串字面量）
3. **忘记在 `main()` 中调用**：函数写了但没验证

#### 验证方式

```bash
cargo build -p text_analyzer && cargo run -p text_analyzer
# 输出中应看到：行数: 2
```

---

### 练习 1-2：添加 `average_word_length` 函数

#### 结论

返回 `f64` 类型属于 Copy 类型，不需要考虑借用 vs 所有权问题。`f64` 是栈上数据，返回时自动复制。

#### 思路

遍历所有单词，统计总字符数，除以单词数量。注意整数除法截断问题，需要转换为 `f64`。

#### 参考实现

```rust
/// 计算所有单词的平均字符长度。
///
/// # 为什么 `f64` 不需要考虑借用 vs 所有权？
///
/// `f64` 实现了 `Copy` trait，是栈上的简单标量值。当函数返回 `f64` 时，
/// 它直接在栈上复制一份给调用者——不存在"借用"或"所有权转移"的语义差异。
///
/// 在 Rust 中，只有堆分配类型（`String`, `Vec<T>` 等）的借用 vs 所有权
/// 选择才需要认真权衡。`f64`, `i32`, `bool` 等基本类型都是 Copy 的，
/// 返回它们总是"转移所有权"（实际上就是复制），不会造成性能问题。
///
/// # 边界情况
///
/// 如果文本为空（没有单词），返回 0.0。
fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return 0.0;
    }
    let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
    total_chars as f64 / words.len() as f64
}
```

更简洁的写法（避免收集 Vec）：

```rust
fn average_word_length(text: &str) -> f64 {
    let mut total_chars = 0usize;
    let mut word_count = 0usize;
    for word in text.split_whitespace() {
        total_chars += word.chars().count();
        word_count += 1;
    }
    if word_count == 0 {
        return 0.0;
    }
    total_chars as f64 / word_count as f64
}
```

#### 为什么这样设计

- `f64` 是 Copy 类型，按值返回即可
- 使用 `as f64` 避免整数除法截断
- 空文本返回 `0.0` 是合理的默认行为

#### 常见错误

1. **整数除法**：`total_chars / words.len()` 得到整数 0 或 1
2. **忘记处理空文本**：除以 0 导致 panic（浮点数除法中除以 0 不会 panic，但语义错误）
3. **使用 `.len()` 而不是 `.chars().count()`**：对包含中文的文本，`.len()` 返回字节数而非字符数

#### 验证方式

```bash
cargo run -p text_analyzer
# 输出：平均单词长度: X.XX 字符
```

---

### 练习 1-3：添加 `shortest_word` 函数

#### 结论

返回 `Option<&str>`，零成本借用输入数据的一部分，不需要堆分配。

#### 思路

模仿 `find_longest_word`，用 `min_by_key` 替代 `max_by_key`。相同的 `&str` 借用模式。

#### 参考实现

```rust
/// 找出文本中最短的单词，返回它的借用切片（如果文本非空）。
///
/// # 为什么返回 `Option<&str>` 而不是 `Option<String>`？
///
/// ```text
/// // [好的设计] Option<&str> —— 零分配，直接指向原数据
/// fn shortest_word(text: &str) -> Option<&str> { ... }
///
/// // [次优设计] Option<String> —— 每次调用都要 clone() 整个单词
/// fn shortest_word(text: &str) -> Option<String> { ... }
/// ```
///
/// 因为最短单词**已经是输入文本的一部分**，返回 `&str` 直接引用原数据，
/// 不需要 `.to_string()` 在堆上分配新内存。这就是借用的威力。
///
/// # 如果有多个单词长度相同且都是最短，`min_by_key` 返回哪一个？
///
/// 返回**第一个**遇到的最短单词。`min_by_key` 和 `max_by_key` 都是稳定的：
/// 遇到相等值时保留先出现的元素。
///
/// # 如果改成 `Option<String>` 会多做什么操作？
///
/// 需要调用 `.to_string()` 对最短单词做堆分配和克隆，增加不必要的内存开销。
fn shortest_word(text: &str) -> Option<&str> {
    text.split_whitespace()
        .min_by_key(|word| word.chars().count())
}
```

在 `main()` 中的调用：

```rust
match shortest_word(text) {
    Some(shortest) => println!("  最短单词: \"{}\"", shortest),
    None => println!("  文本为空，没有单词"),
}
```

#### 为什么这样设计

- `min_by_key` 语义对应需求，代码简洁
- `Option<&str>` 是零成本返回值
- 借用规则保证返回的引用在 `text` 有效期内始终安全

#### 常见错误

1. **返回 `Option<String>`**：不必要的克隆操作
2. **返回 `&str`（没有 Option）**：无法处理空文本（Rust 没有 null）
3. **用 `min_by` 而不是 `min_by_key`**：也可以，但 `min_by_key` 更简洁

#### 验证方式

```bash
cargo run -p text_analyzer
# 输出：最短单词: "a"（或其他最短的）
```

---

## Level 2：进阶练习（综合设计决策）

---

### 练习 2-1：添加 `find_words_containing` 函数并论证设计

#### 结论

参数用 `&str` 借用、返回 `Vec<&str>` 借用集合——最大程度利用借用机制，避免不必要的数据复制。

#### 思路

函数需要：读取输入文本，查找匹配子串的单词，返回这些单词的集合。所有操作都是只读的，不需要获取所有权。返回的引用指向输入文本，所以用 `Vec<&str>`。

#### 参考实现

```rust
/// 返回文本中所有包含指定子串的单词（返回借用切片，不复制数据）。
///
/// # 设计决策论证
///
/// ## a) 为什么两个参数都用 `&str`？
///
/// 两个参数都只需要**读取**，不需要修改或持有。使用 `&str`（不可变借用）：
/// - 调用者保留 `text` 和 `substring` 的所有权，调用后可继续使用
/// - 兼容 `&str`、`&String` 和字符串字面量（通过 deref coercion）
///
/// 如果第一个参数用了 `String`，调用者交出所有权后：
/// - 不能再使用原文本（如继续传给其他分析函数）
/// - 必须提前 `clone()` 才能保留，浪费内存
///
/// ## b) 为什么返回 `Vec<&str>` 而不是 `Vec<String>`？
///
/// 返回的每个元素都是输入文本中某个单词的借用切片。
/// 如果返回 `Vec<String>`：
/// - 每个匹配的单词都需要 `to_string()` 堆分配一次
/// - 假设文本有 10000 个单词，其中 500 个匹配，就需要 **500 次堆分配**
/// - `Vec<&str>` 是 **0 次堆分配**（除了 Vec 本身的元数据）
///
/// ## c) 为什么返回 `Vec<&str>` 而不是 `&[&str]`？
///
/// `Vec<&str>` 是函数内部**新构建**的集合（筛选出匹配的单词），
/// 它在函数内部被创建，必须作为一个有所有权的值返回。
///
/// 如果返回 `&[&str]`（切片引用），它引用的是谁？
/// - 只能是函数内部的局部变量 `Vec`
/// - 函数返回后局部变量被 drop，引用就悬垂了
/// - Rust 编译器会阻止这种代码编译
///
/// # 生命周期说明
///
/// 返回的 `Vec<&str>` 中每个 `&str` 的生命周期与输入参数 `text` 绑定。
/// 只要 `text` 有效，返回的引用就有效。
fn find_words_containing<'a>(text: &'a str, substring: &str) -> Vec<&'a str> {
    text.split_whitespace()
        .filter(|word| word.contains(substring))
        .collect()
}
```

在 `main()` 中的调用：

```rust
println!("═══════════════ 包含子串的单词 ═══════════════");
let substring = "ust";
let matches = find_words_containing(text, substring);
println!("  包含 \"{}\" 的单词:", substring);
for word in &matches {
    println!("    - {}", word);
}

// 边界情况测试
println!("  空文本测试: {:?}", find_words_containing("", "a"));
println!("  无匹配测试: {:?}", find_words_containing("hello world", "zzz"));
```

#### 扩展思考（可选）

如果 `text` 是函数内部创建的临时 `String`，而函数返回 `Vec<&str>`：

```rust
// 以下代码无法编译！
fn bad_example() -> Vec<&str> {
    let text = String::from("hello world");  // text 在函数内部创建
    // 返回 Vec<&str> 引用 text 的内容
    // ❌ 编译错误：text 在函数返回后被 drop，引用悬垂
    text.split_whitespace().collect()
}
```

#### 常见错误

1. **返回 `Vec<String>`**：不必要的克隆
2. **忘记标注生命周期**：编译器有时无法推断出 `'a`，需显式标注
3. **假设 `String` 参数"更安全"**：实际上 `&str` 更安全，因为不限制数据来源

#### 验证方式

```bash
cargo build -p text_analyzer && cargo run -p text_analyzer
```

---

### 练习 2-2：添加 `find_palindromic_words` 函数

#### 结论

辅助函数 `is_palindrome` 用 `&str` 参数，主函数返回 `Vec<&str>`（借用集合）。

#### 思路

`to_lowercase()` 返回 `String` 是因为 Unicode 大小写转换可能改变字节长度（如 `ß` → `SS`），必须分配新内存。回文检查需要字符级比较，先转小写再反转对比。

#### 参考实现

```rust
/// 检查一个单词是否为回文（忽略大小写）。
///
/// # 为什么参数用 `&str`？
///
/// `is_palindrome` 只读取单词内容，不修改、不持有。`&str` 是最轻量的借用方式。
///
/// # 为什么 `to_lowercase()` 返回 `String` 而不是 `&str`？
///
/// Unicode 大小写转换可能改变字符数量和 UTF-8 编码长度。
/// 例如德语中的 `'ß'` 转大写变成 `"SS"`（1 个字符变成 2 个字符）。
/// 这种转换需要新的内存分配，所以必须返回拥有所有权的 `String`。
/// 这是 Rust "不做隐式分配" 哲学的体现——分配内存是显式的。
fn is_palindrome(word: &str) -> bool {
    let lower: String = word.to_lowercase();
    let reversed: String = lower.chars().rev().collect();
    lower == reversed
}

/// 找出文本中所有的回文单词（忽略大小写）。
///
/// 返回 `Vec<&str>`——每个回文单词都是输入文本的一部分，
/// 不需要克隆，零额外内存分配。
fn find_palindromic_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .filter(|word| is_palindrome(word))
        .collect()
}
```

在 `main()` 中的调用：

```rust
println!("═══════════════ 回文单词 ═══════════════");
let palindromes = find_palindromic_words(text);
if palindromes.is_empty() {
    println!("  没有找到回文单词");
} else {
    println!("  找到的回文单词:");
    for word in &palindromes {
        println!("    - {}", word);
    }
}
```

#### 为什么这样设计

- `to_lowercase()` 返回 `String` 是必要的——Unicode 的原因
- 在 `is_palindrome` 内部做分配是合理的（局部变量，用完即释放）
- `find_palindromic_words` 仍返回 `Vec<&str>`，不克隆原文本

#### 常见错误

1. **忘记转小写**：`"Radar"` 应该是回文，但直接比 `"Radar" != "radaR"` 会失败
2. **用 `.len()` 比较长度**：应该用 `.chars().count()`
3. **不必要的提前收集**：不需要把所有单词先收集到 Vec 再过滤

#### 验证方式

```bash
cargo run -p text_analyzer
# 应该看到回文单词输出
```

---

## Level 3：重构练习（架构级思考）

---

### 练习 3-1：将分析函数重构为 `TextAnalyzer` 结构体

#### 结论

结构体持有 `String`（所有权），方法通过 `&self` 借用访问。这体现了 Rust 中"按需获取权限"的设计——结构体拥有数据，方法根据需求选择 `&self`、`&mut self` 或 `self`。

#### 思路

将分散的独立函数组织为结构体的方法，让数据（text）和行为（分析方法）内聚在一起。保留旧函数作为对比，展示两种设计风格。

#### 参考实现

```rust
// ============================================================================
// TextAnalyzer 结构体
// ============================================================================

/// 文本分析器 —— 持有文本的所有权，提供各种分析方法。
///
/// # 设计理念
///
/// `TextAnalyzer` 拥有文本数据的所有权（`text: String`），
/// 这样分析器可以在任何地方被使用，不受外部数据生命周期的限制。
///
/// # 与独立函数的对比
///
/// | 方面 | 独立函数 | 结构体方法 |
/// |------|---------|-----------|
/// | 数据所有权 | 调用者持有 | 结构体持有 |
/// | 生命周期依赖 | 返回值生命周期绑定到输入参数 | 返回值生命周期绑定到 `&self` |
/// | 使用场景 | 一次性分析 | 对一个文本做多次分析 |
/// | 灵活性 | 更高（可分析任意 `&str`） | 更低（绑定到一个文本） |
struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    /// 创建一个新的文本分析器。
    ///
    /// # 为什么 `new` 的参数是 `String` 而不是 `&str`？
    ///
    /// 分析器需要**持有**文本数据的所有权，这样才能保证在分析器的
    /// 整个生命周期内数据都有效。如果用 `&str`，分析器会受限于
    /// 外部数据的生命周期——外部数据被释放后分析器就不可用了。
    ///
    /// 这体现了 Rust 的核心原则：**谁拥有数据，谁就决定数据的生命周期**。
    fn new(text: String) -> Self {
        TextAnalyzer { text }
    }

    /// 统计 Unicode 字符数。
    ///
    /// # 为什么参数是 `&self` 而不是 `self`？
    ///
    /// 方法只需要**读取** `self.text`，不需要消耗 `self`。
    /// `&self` 是不可变借用，允许多次调用，不会转移所有权。
    fn count_chars(&self) -> usize {
        self.text.chars().count()
    }

    /// 统计单词数。
    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// 统计行数。
    fn count_lines(&self) -> usize {
        self.text.lines().count()
    }

    /// 找出最长的单词，返回借用切片。
    ///
    /// # 编译器如何保证引用的安全性？
    ///
    /// 返回的 `Option<&str>` 生命周期与 `&self` 的借用相同。
    /// 编译器通过生命周期分析保证：
    /// - 在 `&self` 借用活跃期间，`self` 不能被修改或 drop
    /// - 返回的引用不会逃出 `&self` 借用的生命周期
    ///
    /// # 如果尝试在 `&self` 方法中修改 `self.text`？
    ///
    /// 编译器会报错：不能通过 `&self`（不可变借用）修改字段。
    /// 需要将方法改为 `&mut self` 才能修改。
    fn find_longest_word(&self) -> Option<&str> {
        self.text.split_whitespace()
            .max_by_key(|w| w.chars().count())
    }

    /// 返回第一个单词。
    fn first_word(&self) -> Option<&str> {
        self.text.split_whitespace().next()
    }

    /// 检查是否包含指定关键词。
    fn contains_keyword(&self, keyword: &str) -> bool {
        self.text.contains(keyword)
    }

    /// 统计词频，返回 Vec<(&str, usize)>。
    fn count_word_frequency(&self) -> Vec<(&str, usize)> {
        use std::collections::HashMap;

        let mut freq: HashMap<&str, usize> = HashMap::new();
        for word in self.text.split_whitespace() {
            *freq.entry(word).or_insert(0) += 1;
        }

        let mut result: Vec<(&str, usize)> = freq.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        result
    }

    /// 生成摘要报告，返回拥有所有权的 String。
    fn summarize(&self) -> String {
        let char_count = self.count_chars();
        let word_count = self.count_words();
        let longest = self.find_longest_word();

        let mut summary = format!(
            "═══ 文本分析摘要（结构体版本）═══\n\
             字符数: {}\n单词数: {}\n",
            char_count, word_count
        );

        match longest {
            Some(w) => summary.push_str(
                &format!("最长单词: {} ({} 字符)\n", w, w.chars().count())
            ),
            None => summary.push_str("最长单词: (无)\n"),
        }

        summary
    }
}
```

在 `main()` 中对比两种用法：

```rust
println!("═══════════════ 10. 结构体方法 vs 独立函数 ═══════════════");

// 旧版：独立函数
println!("--- 独立函数版本 ---");
let text_slice = SAMPLE_TEXT;
println!("字符数: {}", count_chars(text_slice));
println!("单词数: {}", count_words(text_slice));

// 新版：结构体方法
println!("\n--- TextAnalyzer 结构体版本 ---");
let analyzer = TextAnalyzer::new(SAMPLE_TEXT.to_string());
println!("字符数: {}", analyzer.count_chars());
println!("单词数: {}", analyzer.count_words());
println!("最长单词: {:?}", analyzer.find_longest_word());
println!("{}", analyzer.summarize());
```

#### 为什么这样设计

- `new(text: String)` 获取所有权：结构体是数据的"家"
- `&self` 方法只读访问：符合最小权限原则
- `&self` 方法中不能修改 `self.text`：编译器强制
- 返回的 `Option<&str>` 安全：生命周期与 `&self` 绑定

#### 常见错误

1. **`new` 参数用 `&str`**：结构体无法脱离外部生命周期独立存在
2. **方法用 `self` 而不是 `&self`**：调用后结构体被消费，不能再次使用
3. **在 `&self` 方法中修改字段**：编译错误，编译器强制不可变借用

#### 验证方式

```bash
cargo build -p text_analyzer
cargo run -p text_analyzer
# 应该看到两种用法的输出
```

---

## 思考题解答

### 思考题：函数签名中的生命周期权衡

**1. 版本 A 有什么优势？**

- **零分配**：`Vec<&str>` 返回的是输入文本的引用，不需要额外堆分配
- **灵活性**：调用者保留原数据的所有权，可以继续使用或传给其他函数
- **兼容性**：`&str` 参数可以接受 `&str`、`&String`、字符串字面量

**2. 版本 A 有什么限制？**

- **生命周期耦合**：返回的 `Vec<&str>` 不能比输入数据活得更长
- 当原 `text` 被释放后，返回的所有引用都变成悬垂指针——编译器在编译时阻止这种用法

**3. 什么场景下必须使用版本 B？**

当输入数据的生命周期不够长时。例如：
- `text` 来自一个 HTTP 响应的临时缓冲区，需要在响应处理完之前释放
- `text` 来自一个临时文件读入的内存映射，映射即将关闭
- 函数需要将结果存储到结构体字段中，该字段需要独立于输入数据存活

在这些情况下，必须克隆数据获取所有权。

**4. 如果 `extract_words_a` 接受 `&str`，但调用者只有一个 `String`**

编译器通过 **deref coercion** 自动将 `&String` 转换为 `&str`。调用者只需传 `&my_string` 即可，一切都自动完成。

**5. 如果 `extract_words_b` 的参数改成 `&str`，但保留 `Vec<String>` 返回值**

这是合理的设计折中：参数选择正确（借用输入），返回值拥有所有权（因为是新构建的数据，且可能与输入数据解耦）。适合"从输入生成新数据"的场景。

---

## 推荐的学习流程确认

- [x] Level 1 全部完成：`count_lines`, `average_word_length`, `shortest_word`
- [x] Level 2 全部完成：`find_words_containing`, `find_palindromic_words`
- [x] Level 3 重构完成：`TextAnalyzer` 结构体与方法
- [x] 思考题已回答
