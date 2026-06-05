# Chapter 07: 所有权实战 —— 文本分析器

## 本章目标

本章是本书第一个综合实践章节。前三章（Chapter 04、05、06）分别讲解了：

- **Chapter 04** — 栈、堆和 RAII：内存分配的基本模型
- **Chapter 05** — 所有权、Move、Copy、Clone：谁拥有数据，数据如何转移
- **Chapter 06** — 引用、借用、切片：如何在不转移所有权的情况下访问数据

本章的目标是：**通过构建一个真实的文本分析器，让你亲手应用所有权、借用和切片的知识**。在写代码的过程中，你会反复面临同一个问题：这个参数应该用 `&str` 还是 `String`？这个返回值应该用借用的引用还是拥有所有权的值？

## 为什么需要实践章节

学习 Rust 的所有权系统，最困难的部分不是理解概念本身，而是**在做设计决策时形成直觉**。当你习惯了 Python 或 JavaScript 的"一切皆引用"模型后，突然面对：

```rust
fn analyze(text: &str) -> ...      // 借用
fn analyze(text: String) -> ...    // 获取所有权
```

你会犹豫：到底选哪个？有什么区别？选错了会怎样？

本章通过7个分析函数 + 演示代码，让你在真实场景中反复练习这些决策。每个函数都附有文档注释，解释**为什么**选择这个签名而不是其他选项。

## 项目结构

```
chapters/07_ownership_practice_text_analyzer/
├── Cargo.toml          # 包配置
├── src/
│   └── main.rs         # 完整源代码（~300 行，含文档注释）
├── README.md           # 本章说明（本文件）
└── EXERCISES.md        # 练习题
```

## 函数详解

### 1. `count_chars(text: &str) -> usize`

**功能**：统计文本中的 Unicode 字符数（而非字节数）。

**参数选择 — `&str`**：
- 函数只需要**读取**文本，不需要修改或拥有它。
- 用 `&str` 允许调用者保留所有权，同一份数据可以传给多个分析函数。
- 如果用了 `String`，调用者每次传递都得移走所有权或用 `.clone()`。

**返回值选择 — `usize`**：
- 统计结果是一个整数，不属于文本的一部分，返回所有权是合理的。
- `usize` 是 `Copy` 类型，不存在所有权争议。

**为什么用 `.chars().count()` 而不是 `.len()`**：
- `.len()` 返回**字节数**。中文汉字在 UTF-8 编码中占 3 个字节，英文字母占 1 个字节。
- `.chars()` 遍历 Unicode 标量值，每个字符（无论中英文）都算 1 个。
- 示例文本：126 个字符，但字节数是 154，差值为 28（来自中文字符）。

**备选方案及为何拒绝**：
- `fn count_chars(text: String) -> usize` — 拒绝。调用者失去所有权，不适用于只读分析。
- `fn count_chars(text: &String) -> usize` — 拒绝。不够灵活，只能接受 `&String`，不能接受 `&str` 字面量。

---

### 2. `count_words(text: &str) -> usize`

**功能**：以空白字符为分隔符统计单词数。

**参数选择 — `&str`**：
- 同 `count_chars`，只读场景的标准选择。
- 使用 `.split_whitespace()` 而非 `.split(' ')`，好处是能处理多个连续空格、制表符和换行。

**Deref coercion 的便利**：
- Rust 会自动将 `&String` 转换为 `&str`（因为 `String` 实现了 `Deref<Target=str>`）。
- 所以 `&str` 参数可以同时接受 `&String` 和 `&str`。
- 如果写成 `&String` 参数，则只能接受 `&String`，不能接受字符串字面量 `&str`。

---

### 3. `find_longest_word(text: &str) -> Option<&str>`

**功能**：找出文本中最长的单词。返回的是**借用切片**，指向原文本的一部分。

**参数选择 — `&str`**：同上。

**返回值选择 — `Option<&str>`**：
这是本章最值得细品的签名。考虑三个层次：

- **为什么返回引用而不是 String**：最长的单词**已经存在于输入文本中**。返回 `&str` 直接指向原数据，不需要克隆或分配新内存。这是零成本抽象（zero-cost abstraction）。
- **为什么加 Option**：文本可能为空。用 `Option` 而不是空字符串，编译器强制调用者处理"不存在"的情况。
- **生命周期**：返回的 `&str` 生命周期与输入 `text` 相同。只要 `text` 有效，返回值就有效。编译器在编译时保证这一点。

**备选方案及为何拒绝**：
- `fn find_longest_word(text: &str) -> String` — 拒绝。不必要的堆分配和克隆。`
- `fn find_longest_word(text: &str) -> &str` — 拒绝。如果文本为空，无法表达"没有单词"这个语义。
- `fn find_longest_word(text: String) -> &str` — 拒绝。无法编译。返回的引用指向函数内的局部 `String`，函数结束后 `String` 被释放，引用变成悬垂指针。Rust 编译器会拒绝此代码。

---

### 4. `first_word(text: &str) -> Option<&str>`

**功能**：返回文本中的第一个单词（借用切片）。

**设计思路与 `find_longest_word` 完全一致**：
- `&str` 参数：只读借用
- `Option<&str>` 返回：可能无值，有值时零拷贝指向原文

**额外演示**：
- 主函数中用了 `assert_eq!(first, "Rust")` 来演示：`&str` 返回值可以用 `==` 与字符串字面量直接比较。Rust 的 `==` 比较的是**内容**，不是指针地址。

---

### 5. `contains_keyword(text: &str, keyword: &str) -> bool`

**功能**：检查文本中是否包含指定关键词（区分大小写）。

**参数选择 — 两个 `&str`**：
- 两个参数都是只读的，都只需要借用。
- 如果第一个参数用了 `String`，调用者就会失去所有权，之后无法再用原数据做其他分析。

**返回值选择 — `bool`**：
- `bool` 是 `Copy` 类型，直接返回所有权即可。

**经验法则**：当一个函数有多个参数时，**逐个判断**每个参数是否需要所有权。不要因为某个参数需要所有权，就把所有参数都写成 `String`。

---

### 6. `count_word_frequency(text: &str) -> Vec<(&str, usize)>`

**功能**：统计每个单词的出现频率，按频率降序排列。

**参数选择 — `&str`**：仍然是只读借用。

**返回值选择 — `Vec<(&str, usize)>`**：
这是本章另一个关键设计决策。考虑两个问题：

1. **为什么返回 `Vec` 而不是 `&[(...)]` 切片**：频率统计表是函数内部**新构建**的数据结构（HashMap → Vec 排序），不是输入数据的一部分。如果返回 `&[(...)]`，它引用哪个变量？只能是函数内部的局部变量 —— 但函数返回后局部变量被释放，引用就悬垂了。所以必须返回拥有所有权的 `Vec`。

2. **为什么 Vec 的元素是 `(&str, usize)` 而不是 `(String, usize)`**：单词本身来自输入文本，不需要克隆。`&str` 直接指向输入文本中的单词切片。这节省了 N 次堆分配。

**注意**：这个函数假设 `text` 在返回值使用期间保持有效。如果 `text` 被释放（例如它在一个内部作用域中），`Vec` 中的 `&str` 就会变成悬垂引用。编译器会阻止这种情况发生。

**备选方案及为何拒绝**：
- `fn count_word_frequency(text: &str) -> Vec<(String, usize)>` — 次优。每个单词都 `.to_string()` 克隆一次，N 次不必要的堆分配。
- `fn count_word_frequency(text: String) -> Vec<(String, usize)>` — 次优 × 2。参数获取了不必要的所有权，返回值也不必要地克隆。

---

### 7. `summarize_text(text: &str) -> String`

**功能**：生成文本的格式化分析报告，返回拥有所有权的 `String`。

**参数选择 — `&str`**：依然是只读借用！注意：函数的返回类型不影响参数类型的选择。

**返回值选择 — `String`**：
这是**必须返回所有权**的典型场景。摘要内容是 `format!` 宏拼接出来的全新字符串，属于"从无到有合成"的数据。它不属于输入文本，没有地方可以"借用"。如果返回 `&str`，编译器会报错（引用局部变量）。

**设计洞察**：
- **参数用借用 + 返回值用所有权 = 各取所需**。不要因为函数返回 `String` 就把参数也写成 `String`，也不要因为参数是 `&str` 就强迫返回值也是 `&str`。
- 函数的输入和输出是独立的决策维度。

---

## 设计选择深度分析

### 何时使用 `&str` 参数 vs `String` 参数

| 场景 | 推荐参数类型 | 理由 |
|------|-------------|------|
| 函数只读取文本内容 | `&str` | 借用即可，不消耗数据 |
| 函数需要修改文本内容 | `&mut String` | 可变借用，修改后原处有效 |
| 函数需要存储文本（长期保留） | `String` | 需要获取所有权来保证生命周期 |
| 函数需要将文本传给另一个需要所有权的 API | `String` | 所有权需要传递 |
| 函数返回依赖于输入文本的切片 | `&str` | 返回引用时参数必须是引用 |
| 函数需要拼接、格式化、构建新文本 | 参数：`&str`，返回：`String` | 各取所需 |

**核心原则**：**尽可能用借用（`&str`），只在确实需要所有权时用 `String`。**

### 何时返回 `&str` vs `String` vs `Option<&str>`

| 场景 | 推荐返回类型 | 理由 |
|------|-------------|------|
| 结果是输入的一部分 | `&str` | 零拷贝引用 |
| 结果可能不存在（且是输入的一部分） | `Option<&str>` | 编译器强制处理 None |
| 结果是新合成的数据 | `String` | 需要所有权 |
| 结果可能需要修改 | `String` | 借用不可变 |
| 结果的生命周期独立于输入 | `String` | 借用生命周期不够长 |

**核心原则**：**返回的数据来自输入时用引用，来自函数内部合成时用所有权。**

### 何时使用 `Vec` vs 切片 `&[T]`

| 场景 | 推荐类型 | 理由 |
|------|---------|------|
| 函数需要构建新集合 | `Vec<T>` | 需要所有权来增长和返回 |
| 函数只读取集合内容 | `&[T]` (参数) | 比 `&Vec<T>` 更灵活 |
| 返回输入集合的子集 | `&[T]` 或 `Vec<&T>` | 取决于是否需要排序等处理 |
| 需要修改集合大小 | `Vec<T>` | 切片长度固定 |

**核心原则**：**参数优先用 `&[T]`（比 `&Vec<T>` 更通用），返回值优先用引用（除非数据是新构建的）。**

### 设计权衡总结表

| 维度 | 借用（Borrow） | 所有权（Own） |
|------|---------------|--------------|
| 内存分配 | 零额外分配 | 可能触发堆分配 |
| 灵活性 | 高（可接受多种引用类型） | 低（只能接受所有权） |
| 生命周期约束 | 受参数生命周期限制 | 独立于参数 |
| 安全性 | 编译器验证不存在悬垂引用 | 无悬垂风险 |
| 典型开销 | O(1) 指针复制 | O(n) 数据克隆 |
| 何时使用 | 只读访问、临时计算 | 持久存储、数据转换 |

---

## 运行命令和预期输出

### 编译和运行

```bash
# 进入工作区目录
cd rust-from-python

# 编译本章代码
cargo build -p text_analyzer

# 运行
cargo run -p text_analyzer

# 以 release 模式编译（优化）
cargo build -p text_analyzer --release

# 运行测试（如果添加了测试）
cargo test -p text_analyzer
```

### 预期输出概要

程序将依次输出以下内容：

1. **字符统计**：Unicode 字符数（126）vs 字节数（154），说明 `.len()` 和 `.chars().count()` 的区别
2. **单词统计**：17 个单词（按空白字符分割）
3. **最长单词**：`同时支持中文文本的处理和分析`（14 个 Unicode 字符），类型是 `&str`
4. **第一个单词**：`Rust`，用 `assert_eq!` 验证
5. **关键词搜索**：分别检查 "Rust"、"Python"、"guarantees"、"中文" 是否存在
6. **词频统计**：按频率降序排列的单词频率表
7. **文本摘要**：格式化报告，返回的是拥有所有权的 `String`
8. **所有权演示**：对比 `&str` 参数（借后可用）和 `String` 参数（传后消失）
9. **次优设计演示**：对比 `Option<&str>`（零分配）和 `Option<String>`（不必要的克隆）

---

## 代码讲解（按函数分组）

### 组 1：纯读取函数（count_chars, count_words, contains_keyword）

这三个函数代表最常见的模式：
- 参数都是 `&str`（只读借用）
- 返回值都是简单类型（`usize`, `bool`）
- 调用者可以在一次运行中多次调用这些函数，每次都传入同一个 `&text`

**对应的 Python 代码**：

```python
def count_chars(text: str) -> int:
    return len(text)

def count_words(text: str) -> int:
    return len(text.split())

def contains_keyword(text: str, keyword: str) -> bool:
    return keyword in text
```

**区别**：Python 中所有参数都是"引用传递"（实际上是传对象引用），不存在所有权概念。你不能"移走"一个字符串的所有权。这是 Rust 特有的概念，也是它能在没有 GC 的情况下管理内存的关键。

### 组 2：返回借用的函数（find_longest_word, first_word）

这两个函数展示了 Rust 最强大的特性之一：**安全地返回对输入数据的引用**。

- `Option<&str>` 表示"可能不存在的借用切片"
- 零运行时开销：返回的只是一个指针和长度
- 编译器在编译时保证引用不会悬垂

**对应的 Python 代码**：

```python
def find_longest_word(text: str) -> str | None:
    words = text.split()
    if not words:
        return None
    return max(words, key=len)

def first_word(text: str) -> str | None:
    words = text.split()
    if not words:
        return None
    return words[0]
```

**关键的差异**：Python 返回的是**新的字符串对象**（interned 的情况除外），因为 Python 的字符串是不可变的，切片操作 `text[0:4]` 会创建新对象。Rust 的 `&str` 是真正的零拷贝引用 —— 它只是一个指向原数据某处的指针和长度。

### 组 3：返回拥有数据的函数（count_word_frequency, summarize_text）

这两个函数展示了返回值需要所有权的情况。

- `count_word_frequency` 返回 `Vec<(&str, usize)>`：Vec 本身需要所有权（新数据结构），但元素中的单词引用仍指向原文本。
- `summarize_text` 返回 `String`：完全新合成的数据，必须拥有所有权。

**对应的 Python 代码**：

```python
def count_word_frequency(text: str) -> list[tuple[str, int]]:
    from collections import Counter
    return Counter(text.split()).most_common()

def summarize_text(text: str) -> str:
    return f"Character count: {len(text)}\nWord count: {len(text.split())}\n..."
```

**关键的差异**：在 Rust 中，`count_word_frequency` 返回的单词切片（`&str`）的生命周期绑定了输入 `text`。在 Python 中，`most_common()` 返回的字符串是字典中的 key（本质上也是引用），但因为 Python 有 GC，你不需要显式管理这些生命周期。

---

## 与 Python 的核心对照

### 所有权模型的对比

| 特性 | Rust | Python |
|------|------|--------|
| 所有权概念 | 编译时强制执行 | 无（GC 管理） |
| 借用 | 显式 `&T`，编译时验证 | 隐式，运行时引用计数 |
| 切片 | `&str` 零拷贝引用 | 字符串切片创建新对象 |
| 移动语义 | 默认 move，`=` 后原变量失效 | 引用计数减一，原变量仍有效 |
| 复制 | 只有 `Copy` 类型隐式复制 | 赋值只是增加引用计数 |
| Option | 性能等同指针，编译器强制检查 | `Optional[T]` 但运行时检查非强制 |

### Python 开发者最容易犯的5个错误

1. **以为 `=` 是复制**：Rust 中 `=` 默认是 move（转移所有权），除非类型实现了 `Copy`。

2. **参数过度使用 `String`**：Python 中你不用担心参数类型（都是引用）。在 Rust 中，不必要地使用 `String` 参数会让调用者失去数据所有权。

3. **返回值过度使用 `String`**：Python 中返回字符串子串会自动创建新对象。在 Rust 中，如果子串来自输入，返回 `&str` 是更高效的选择。

4. **忽略生命周期**：Python 中所有对象的生命周期由 GC 管理，你不需要思考"这个引用在函数返回后还有效吗"。Rust 中，编译器会强制执行生命周期规则。

5. **对 Option 不够重视**：Python 中常返回 `None` 或空字符串来表示"没有结果"，但编译器不会强制检查。Rust 的 `Option` 在编译时强制调用者处理两种情况。

---

## 常见错误设计选择及分析

### 错误 1：参数全部使用 `String`

```rust
// 错误设计 —— 每个参数都获取所有权
fn count_chars(text: String) -> usize { ... }
fn count_words(text: String) -> usize { ... }
fn find_longest_word(text: String) -> Option<String> { ... }
```

**问题分析**：
1. 调用者调用第一个函数后失去 `text` 的所有权，无法再调用后续函数。
2. 必须在每次调用前 `.clone()`，造成 N 次不必要的堆分配。
3. 函数体内部不需要所有权（只读），获取所有权是过度设计。

**改进后**：

```rust
// 改进设计 —— 参数用借用，返回值按需选择
fn count_chars(text: &str) -> usize { ... }
fn count_words(text: &str) -> usize { ... }
fn find_longest_word(text: &str) -> Option<&str> { ... }
```

---

### 错误 2：返回值全部使用 `String`

```rust
// 错误设计 —— 不必要地克隆所有返回数据
fn first_word(text: &str) -> String {
    text.split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()  // 不必要的克隆！
}
```

**问题分析**：
1. `first_word` 返回的是输入文本的一部分，不需要克隆。
2. 每次调用都在堆上分配新的 `String`，增加内存压力。
3. 如果调用者只需要读取结果（例如打印），克隆是完全浪费的。

**改进后**：

```rust
// 改进设计 —— 返回借用切片
fn first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}
```

---

### 错误 3：混淆参数和返回值的决策

```rust
// 错误设计 —— 不一致的选择
fn summarize_text(text: String) -> &str {
    // 因为参数获取了所有权，无法再借用 text 来返回引用？
    // 实际上可以（因为 text 在函数内），但这很令人困惑。
    // 而且调用者失去了 text 的所有权，summary 返回后 text 被 drop。
    // 总之这个设计自相矛盾。
}
```

**问题分析**：
1. 参数获取所有权但函数只需要读取，设计不合理。
2. 返回 `&str` 但引用了参数（拥有所有权的局部变量），函数返回后局部变量被释放，无法编译通过。

**改进后**：

```rust
// 改进设计 —— 参数借用 + 返回所有权
fn summarize_text(text: &str) -> String {
    format!("... {}", text)
}
```

---

### 错误 4：使用 `&String` 而非 `&str`

```rust
// 次优设计 —— 不够灵活
fn count_chars(text: &String) -> usize {
    text.chars().count()
}

let my_str = "hello";  // &str 类型
// count_chars(&my_str);  // 编译错误！期望 &String，得到 &&str
// count_chars(my_str);   // 编译错误！期望 &String，得到 &str
```

**问题分析**：
1. `&String` 只能接受 `&String`，不能接受 `&str`。
2. 而 `&str` 可以同时接受 `&String`（通过 deref coercion）和 `&str`。

**改进后**：

```rust
// 改进设计 —— 用 &str，更灵活
fn count_chars(text: &str) -> usize {
    text.chars().count()
}
```

---

## 本章小结

本章通过构建一个文本分析器，综合练习了以下核心概念：

### 所有权（Ownership）
- `String` 拥有数据，`&str` 借用数据
- 函数参数用 `String` 意味着获取所有权，调用者无法再使用原数据
- 所有权是 Rust 在没有 GC 的情况下管理内存的基础

### 借用（Borrowing）
- `&str` 是不可变借用，函数只读取数据而不获取所有权
- 借用的数据可以被多次借给不同的函数
- 借用检查器在编译时保证借用的安全性

### 切片（Slices）
- `&str` 是字符串切片，指向某个字符串数据的一部分
- 切片是零成本的视图（view），不分配新内存
- 返回切片时，生命周期必须比输入数据短或相等

### 设计决策框架

当你编写 Rust 函数时，按以下顺序思考签名设计：

1. **函数需要修改数据吗？** 是 → `&mut T`；否 → `&T`
2. **函数需要存储数据吗？** 是 → `T`（获取所有权）；否 → 回到步骤1
3. **返回值是输入数据的一部分吗？** 是 → 返回借用 `&T`；否 → 返回所有权 `T`
4. **结果可能不存在吗？** 是 → 包装在 `Option<T>` 中

---

## 下一章衔接

在下一章（Chapter 08: 结构体、方法和关联函数），我们将学习如何把这些分析函数组织成一个结构体：

```rust
// 从独立函数升级到结构体方法
struct TextAnalyzer {
    text: String,  // 分析器拥有数据的所有权
}

impl TextAnalyzer {
    fn new(text: String) -> Self { ... }
    fn count_chars(&self) -> usize { ... }    // &self = 借用自身
    fn find_longest_word(&self) -> Option<&str> { ... }
}
```

这引入了新的所有权思考：`&self` 是什么？为什么方法用 `&self` 而不是 `self`？结构体拥有 `String` 字段时，如何安全地返回 `&str` 引用？这些问题将在 Chapter 08 中详细解答。

继续前进之前，请确保你完成了 [EXERCISES.md](EXERCISES.md) 中的练习题。实践是掌握所有权的唯一途径。
