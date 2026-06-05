# 第十章练习题 — 集合类型：Vec, String, HashMap

## 命令速查

```bash
# 编译并运行
cargo run

# 仅编译（不运行）
cargo build

# 编译 release 版本
cargo build --release

# 运行测试（如果你创建了测试）
cargo test

# 查看文档
cargo doc --open
```

---

## Level 1：基础巩固

### L1-1：Vec 基本操作

编写函数 `fn vec_operations() -> Vec<i32>`，完成以下操作：

1. 使用 `Vec::new()` 创建一个空 `Vec<i32>`
2. 使用 `push` 依次添加数字 1 到 5
3. 使用 `pop` 弹出最后一个元素，并打印它的值
4. 使用 `get` 安全地获取索引 2 和索引 10 的元素，分别处理 `Some` 和 `None`
5. 使用 `vec!` 宏创建另一个 Vec，包含 `[10, 20, 30]`
6. 使用 `extend` 将第二个 Vec 的所有元素追加到第一个 Vec
7. 使用 `for` 循环和 `iter()` 打印所有元素
8. 返回最终的 Vec

**预期输出示例**：
```
弹出的元素: 5
索引 2: 3
索引 10: None (越界安全处理)
最终 Vec: [1, 2, 3, 4, 10, 20, 30]
```

**要点**：区分 `[]` 索引（panic）和 `.get()`（返回 Option）的区别。

---

### L1-2：String 与 UTF-8 探索

编写函数 `fn explore_utf8(s: &str)`，对输入字符串完成以下任务：

1. 打印字符串的字节长度（`.len()`）和字符个数（`.chars().count()`）
2. 使用 `.bytes()` 打印每个字节的十六进制表示
3. 使用 `.chars()` 打印每个 Unicode 标量值
4. 尝试获取第一个字符：使用 `.chars().next()`
5. 说明：如果输入是 "Rust"，字节长度和字符个数相等；如果输入是 "你好"，字节长度（6）不等于字符个数（2）

**测试用例**：
```rust
explore_utf8("Rust");     // 字节: 4, 字符: 4
explore_utf8("你好");      // 字节: 6, 字符: 2
explore_utf8("a🚀b");     // 字节: ? 字符: ?  —— 试着计算一下
```

**要点**：理解 UTF-8 是变长编码，这就是 `String` 不支持索引的原因。

---

### L1-3：HashMap 基础 —— 学生成绩管理

编写函数 `fn manage_scores()` 实现一个简单的学生成绩管理系统：

1. 创建一个 `HashMap<String, u32>` 存储学生姓名和成绩
2. 插入至少 5 个学生的成绩
3. 使用 `get` 查询"张三"和"王五"的成绩
4. 使用 Entry API 的 `or_insert`：如果"赵六"不存在则插入 70 分
5. 使用 `iter()` 打印所有学生的成绩
6. 统计并打印成绩大于 80 分的学生人数

**提示**：
```rust
let count = scores.values().filter(|&&s| s > 80).count();
```

**要点**：熟练掌握 `insert`、`get`、Entry API、`iter()` 的组合使用。

---

## Level 2：综合应用

### L2-1：文本统计分析器

修改 `src/main.rs` 中的 `tokenize`、`count_frequencies`、`get_top_n` 函数，扩展为完整的文本统计分析器。

**要求**：

1. **增强 tokenize**：除了现有的分词逻辑外，额外过滤掉长度小于 2 的单词（如 "a", "is", "at" 等）
2. **添加统计功能**：编写 `fn analyze(freq: &HashMap<String, usize>)` 统计：
   - 不重复单词总数（distinct words）
   - 总单词数（total words，即所有出现次数的总和）
   - 平均出现次数（total / distinct）
   - 只出现一次的单词数量（hapax legomena）
   - 出现次数最多的单词及其次数
3. **打印格式化报告**：输出一个格式化的统计报告

**预期输出格式**：
```
========== 文本统计报告 ==========
不重复单词数: 42
总单词数: 87
平均出现次数: 2.07
仅出现一次的单词数: 28
出现次数最多的单词: "rust" (出现 6 次)
==================================
```

**提示**：用 `freq.values().sum::<usize>()` 求总单词数，用浮点数计算平均值。

---

### L2-2：简易通讯录系统

设计一个通讯录系统，使用 `HashMap<String, Contact>` 存储联系人信息。

**要求**：

1. 定义 `Contact` 结构体：
```rust
struct Contact {
    name: String,
    phone: String,
    email: String,
}
```

2. 实现以下函数：
```rust
// 添加联系人（如果已存在则更新）
fn add_contact(book: &mut HashMap<String, Contact>, contact: Contact);

// 删除联系人，返回被删除的联系人信息（如果有）
fn remove_contact(book: &mut HashMap<String, Contact>, name: &str) -> Option<Contact>;

// 搜索联系人，支持按姓名模糊搜索（包含子串即可）
fn search_contacts(book: &HashMap<String, Contact>, query: &str) -> Vec<&Contact>;

// 列出所有联系人（按姓名排序）
fn list_contacts(book: &HashMap<String, Contact>) -> Vec<&Contact>;
```

3. 在 `main` 函数中演示所有操作：添加 5 个联系人，搜索、删除、列出

**要点**：
- Entry API 用于 `add_contact`（存在则更新）
- 理解 `search_contacts` 返回 `Vec<&Contact>` 的生命周期 —— 引用的生命周期不能超过 `book` 的生命周期
- 排序需要先收集到 Vec 再 sort

---

## Level 3：所有权挑战

### L3-1：共享字符串存储

设计一个系统，使用 `Vec<String>` 存储一组字符串，同时使用 `HashMap<&str, Vec<usize>>` 建立字符索引（字符到字符串位置的映射）。

**背景**：在实际系统中，我们可能需要在内存中保留一份数据，同时建立多个索引加速查找。由于 Rust 的所有权限制，同时持有数据和对数据的引用需要仔细设计。

**要求**：

1. 创建一个 `StringPool` 结构体：
```rust
struct StringPool {
    data: Vec<String>,               // 拥有所有字符串
    index: HashMap<String, Vec<usize>>,  // 单词→出现位置的索引
}
```

2. 实现以下方法：
```rust
impl StringPool {
    // 创建空池
    fn new() -> Self;

    // 添加一个字符串，自动更新索引
    fn add(&mut self, s: String);

    // 根据单词查询它在哪些位置出现过
    fn find(&self, word: &str) -> Option<&Vec<usize>>;

    // 获取池中字符串总数
    fn len(&self) -> usize;

    // 根据位置获取字符串
    fn get(&self, pos: usize) -> Option<&str>;
}
```

3. **关键挑战**：`find` 返回的是 `Option<&Vec<usize>>`，这个不可变借用的生命周期绑定到了 `&self`。如果想在持有这个引用的同时调用 `add`（可变借用），编译器会报错。请思考：
   - 这种情况在实际系统中如何解决？
   - 什么设计方案可以避免这种借用冲突？

4. 在 `main` 中演示 `StringPool` 的使用：
   - 添加一段英文文本的所有单词
   - 查询某个单词出现的所有位置
   - 打印每个位置的上下文（该位置的单词前后各一个单词）

**提示**：索引中用 `String` 而非 `&str` 作为键，这样 `index` 拥有自己的键副本，不依赖 `data` 中的数据。

---

## 思考题

### 深度思考：为什么 Rust 的 String 不支持索引，而 Python 的 str 支持？

从以下几个维度分析：

1. **底层数据结构**：Python 的 str 内部使用什么编码？在 Python 3.3+ 中 (PEP 393)，Python 使用了"灵活字符串表示"（Flexible String Representation），根据字符串内容自动选择 1 字节、2 字节或 4 字节的编码。这与 Rust 始终使用 UTF-8 有何根本区别？

2. **语言设计哲学**：
   - Rust 的设计目标是什么？（零成本抽象、安全、性能）
   - Python 的设计目标是什么？（易用性、开发效率）
   - 如果你要支持 `s[0]`，Rust 需要做什么权衡？

3. **性能分析**：
   - Python 的 `s[0]` 时间复杂度是多少？（提示：考虑不同内部编码的情况）
   - Rust 如果支持 `s[0]`，时间复杂度会是多少？O(1) 还是 O(n)？
   - 如果 Rust 采用类似 Python 的内部编码，对 Rust 的"零成本抽象"目标会有什么影响？

4. **替代方案**：Rust 提供了哪些方式来"按位置获取字符"？它们的时间复杂度分别是多少？

5. **实际场景**：在什么情况下你需要按索引访问字符串？在 Rust 中你通常用什么方式替代？

**建议**：写一段 200 字左右的分析，不用代码，重点是理解两种语言在设计上的权衡。

---

## 参考答案要点

### L1-1 参考思路
```rust
fn vec_operations() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for i in 1..=5 { v.push(i); }
    if let Some(popped) = v.pop() {
        println!("弹出的元素: {popped}");
    }
    match v.get(2) {
        Some(val) => println!("索引 2: {val}"),
        None => println!("索引 2: None"),
    }
    match v.get(10) {
        Some(val) => println!("索引 10: {val}"),
        None => println!("索引 10: None (越界安全处理)"),
    }
    let v2 = vec![10, 20, 30];
    v.extend(v2);
    print!("最终 Vec: ");
    for val in v.iter() { print!("{val} "); }
    println!();
    v
}
```

### L1-2 参考思路
```rust
fn explore_utf8(s: &str) {
    println!("字符串: \"{s}\"");
    println!("  字节长度 (len): {}", s.len());
    println!("  字符个数 (chars): {}", s.chars().count());
    print!("  字节: ");
    for b in s.bytes() { print!("{b:#x} "); }
    println!();
    print!("  字符: ");
    for c in s.chars() { print!("{c} "); }
    println!();
    match s.chars().next() {
        Some(c) => println!("  第一个字符: {c}"),
        None => println!("  字符串为空"),
    }
    println!();
}
// "a🚀b": 字节 6 (a=1, 🚀=4, b=1), 字符 3
```

### L2-1 提示：平均值计算
```rust
let total: usize = freq.values().sum();
let distinct = freq.len();
let avg = total as f64 / distinct as f64;
println!("平均出现次数: {avg:.2}");
```

### L2-2 提示：生命周期标注
`search_contacts` 返回 `Vec<&Contact>`，其中引用的生命周期绑定到 `book`：
```rust
fn search_contacts<'a>(book: &'a HashMap<String, Contact>, query: &str) -> Vec<&'a Contact> {
    book.values()
        .filter(|c| c.name.contains(query))
        .collect()
}
```

### L3-1 关键提示
`StringPool` 中 `index` 使用 `HashMap<String, Vec<usize>>`（Owned 键）而非 `HashMap<&str, Vec<usize>>`，解决了索引持有对 data 引用的问题。`find` 返回的 `&Vec<usize>` 不依赖任何借用，因为它借用的是 `index` 中 owned 的 `Vec<usize>`。

---

## 学习检查清单

完成本章后，你应该能够回答以下问题：

- [ ] `Vec::get` 和 `v[i]` 的区别是什么？什么时候用哪个？
- [ ] 为什么 `String` 不支持索引？如何获取 UTF-8 字符串中的第 n 个字符？
- [ ] `str`、`&str`、`String`、`&String` 之间是什么关系？
- [ ] `HashMap::get` 返回什么类型？如果键不存在会怎样？
- [ ] Entry API 解决了什么问题？`or_insert` 和 `or_insert_with` 的区别是什么？
- [ ] `iter()`、`iter_mut()`、`into_iter()` 的区别是什么？
- [ ] 为什么在 HashMap 中优先使用 `String` 而不是 `&str` 作为键？
- [ ] 如果向 `Vec<String>` 中 `push` 一个 `String`，原来的 `String` 变量还能使用吗？
- [ ] `format!` 宏和 `+` 运算符在所有权处理上的区别是什么？
- [ ] 如何安全地从 Vec 中移除某个索引的元素？

---

祝你练习愉快！记住：Rust 编译器是你最严格但也最有帮助的老师——仔细阅读每一个编译错误，它们都包含了解决问题的方法。
