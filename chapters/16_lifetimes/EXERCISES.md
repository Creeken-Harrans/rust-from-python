# 练习：生命周期

## 练习说明

- 所有可直接运行的代码都实现在 `src/main.rs` 中（或创建新的 `.rs` 文件）
- **不能编译的代码段**（如故意展示错误的示例）请写在 `broken_examples/` 目录下，不要放在 `src/` 中
- 每题给出预期输出或预期行为
- 推荐先独立思考 10-15 分钟再参考答案思路
- 使用 `cargo run` 验证你的实现
- 使用 `cargo test` 运行单元测试
- 所有题目都应在当前 `lifetimes` crate 中完成

---

## Level 1：基础理解（3 题）

### 练习 1.1：最长字符串切片

**题目：** 实现函数 `fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str`，返回两个字符串切片中字符数较多的那个。在 `main` 中调用它并打印结果。

**预期输出示例：**

```
longest_str("hello", "world!") = "world!"
longest_str("短", "比较长的字符串") = "比较长的字符串"
```

**提示：** 这是 `longest` 函数的标准实现，使用 `.len()` 比较长度。注意 `&str` 的 `.len()` 返回的是字节长度，但对于纯 ASCII 或你关心的场景已经足够。

**推荐文件：** `src/main.rs` 中已有类似实现，你可以复制并修改。

---

### 练习 1.2：识别省略规则

**题目：** 对于下面的函数签名，判断编译器能否自动推导生命周期。如果不能，说明缺少什么信息。

```rust
// ①
fn foo(x: &str) -> &str

// ②
fn bar(x: &str, y: &str) -> &str

// ③
fn baz(x: &i32, y: &i32) -> i32

// ④
fn qux(&self, x: &str) -> &str

// ⑤
fn quux(x: &str, y: &str, z: &str) -> &str
```

**预期答案：**
- ① ✅ 可以（省略规则 2：单输入 → 赋给输出）
- ② ❌ 不可以（多个输入引用，编译器不知道关联哪个）
- ③ ✅ 可以（返回的是 `i32`，不是引用，不需要生命周期）
- ④ ✅ 可以（省略规则 3：`&self` 的生命周期赋给输出）
- ⑤ ❌ 不可以（多个输入引用 + 返回引用，编译器无法确定关联关系）

---

### 练习 1.3：结构体中的生命周期

**题目：** 定义一个结构体 `BookSlice<'a>`，包含两个字段：`title: &'a str` 和 `author: &'a str`。实现方法 `fn description(&self) -> String`，返回格式为 `"《书名》，作者：作者名"` 的字符串。

**预期使用：**

```rust
let title = String::from("Rust 程序设计");
let author = String::from("Klabnik & Nichols");
let book = BookSlice { title: &title, author: &author };
println!("{}", book.description());
// 输出：《Rust 程序设计》，作者：Klabnik & Nichols
```

**注意：** `description` 返回的是 `String`（拥有所有权的数据），不是引用，所以不需要生命周期标注。这是一个很好的模式——当你想从引用数据中创建新数据时，返回 `String` 而不是 `&str` 可以避免很多生命周期复杂性。

**推荐命令：** `cargo run`

---

## Level 2：综合应用（2 题）

### 练习 2.1：自定义 longest 逻辑

**题目：** 实现函数 `fn longest_by_key<'a, T, F>(x: &'a T, y: &'a T, f: F) -> &'a T`，其中 `F: Fn(&T) -> usize`。该函数接受两个引用和一个闭包，通过闭包提取长度进行比较，返回"较长"的那个。如果一样长，返回 `x`。

**预期使用：**

```rust
let s1 = String::from("hello");
let s2 = String::from("world!");
let result = longest_by_key(&s1, &s2, |s: &String| s.len());
assert_eq!(result, &s2);

let result2 = longest_by_key(&s1, &s2, |s: &String| s.chars().filter(|c| *c == 'l').count());
// s1 有 2 个 'l'，s2 有 1 个 'l'
assert_eq!(result2, &s1);
```

**提示：** 结合生命周期 `'a` 和泛型 `T`、`F`。注意 `F` 是闭包，使用 `Fn` trait。

**推荐命令：** `cargo test`

---

### 练习 2.2：实现一个带生命周期的迭代器适配器

**题目：** 实现一个结构体 `Lines<'a>`，它包装一个 `&'a str`，并实现 `Iterator` trait，每次迭代返回下一行（以 `\n` 分隔的切片）。要求 `Iterator::Item` 是 `&'a str`。

无需处理 `\r\n` 的 Windows 风格换行，只处理 `\n` 即可。空输入应返回空字符串。

**预期使用：**

```rust
let text = "第一行\n第二行\n第三行";
let lines = Lines::new(text);
let collected: Vec<&str> = lines.collect();
assert_eq!(collected, vec!["第一行", "第二行", "第三行"]);

// 空输入的情况
let empty = Lines::new("");
let empty_collected: Vec<&str> = empty.collect();
assert!(empty_collected.is_empty());
```

**提示：** 结构体需要两个字段：`remaining: &'a str` 和 `done: bool`（标记是否已完成迭代）。`Iterator::next` 方法可以使用 `str::split_once('\n')` 或手动查找 `\n` 的位置。

**推荐命令：** `cargo test`

---

## Level 3：挑战题（1 题）

### 练习 3.1：上下文搜索器

**题目：** 设计一个结构体 `SearchContext<'a>`，它持有对一段文本 `&'a str` 的引用，并提供以下功能：

1. `fn new(text: &'a str) -> Self` — 创建搜索器
2. `fn find(&self, query: &str) -> Option<&'a str>` — 返回匹配到的完整行（以 `\n` 分隔的那一行）
3. `fn search_all<'b>(&'b self, query: &str) -> Vec<&'a str>` — 返回所有匹配行的引用
4. `fn excerpt(&self, query: &str, surrounding_lines: usize) -> Vec<&'a str>` — 返回匹配行以及它前后的 N 行

当 query 是空字符串时，`find` 返回 `None`。

**注意 `search_all` 中的生命周期：**
- `&'b self` 表示借用 `SearchContext` 的持续时间
- 返回 `Vec<&'a str>` 表示返回的引用和 `SearchContext` 内部的文本一样长
- 这展示了返回值的生命周期可以比方法调用的借用更长

**预期使用：**

```rust
let text = "Rust 很强大\nPython 很简单\nRust 很安全\nGo 很快\nRust 很好玩\n";
let searcher = SearchContext::new(text);

// 查找包含 "Rust" 的第一行
let first = searcher.find("Rust");
assert_eq!(first, Some("Rust 很强大"));

// 查找不存在的词
let none = searcher.find("C++");
assert_eq!(none, None);

// 查找所有包含 "Rust" 的行
let all: Vec<&str> = searcher.search_all("Rust");
assert_eq!(all, vec!["Rust 很强大", "Rust 很安全", "Rust 很好玩"]);

// 查找匹配行及其上下文（前后各 1 行）
let ctx: Vec<&str> = searcher.excerpt("安全", 1);
assert_eq!(ctx, vec!["Python 很简单", "Rust 很安全", "Go 很快"]);
```

**提示：**
- 使用 `.lines()` 或手动按 `\n` 分割文本
- 将每行的起始位置存储在 `Vec<usize>` 中以便快速查找
- `excerpt` 方法需要注意边界条件（第一行和最后一行没有前/后文）
- 空字符串作为 query 时直接返回 None/空 Vec

**推荐命令：** `cargo test`

---

## 思考题

### 思考题：为什么 Rust 选择编译时生命周期检查而不是运行时 GC？

请从以下角度思考：

1. **性能**：编译时检查和运行时 GC 的性能差异是什么？生命周期检查为什么是"零成本抽象"？
2. **确定性**：GC 的暂停时间不可预测，生命周期检查的编译时间是确定的。这对什么类型的应用很重要？
3. **与其他系统语言的比较**：C 的 `free` 问题是运行时错误，Rust 的生命周期在编译时就能捕获。哪种更好维护？
4. **与 Python 的比较**：Python 程序员从不考虑生命周期，但不代表没有成本。Python 的 GC 暂停和引用计数开销体现在哪里？
5. **设计权衡**：如果 Rust 选择 GC，它还能被称为"系统编程语言"吗？为什么？

写下你的思考（300 字以上），不需要编写代码。

---

## 推荐命令总结

```bash
# 基础：运行主程序查看所有演示
cargo run

# 测试：运行所有单元测试
cargo test

# 测试单个练习的测试（如果你把它们放在 tests/ 中或用 #[test] 标注）
cargo test test_longest
cargo test test_longest_in_slice

# 检查编译（不运行），快速验证代码是否有语法/类型/生命周期错误
cargo check

# 查看编译器的详细生命周期推导（非常有用！）
# 加上 undocumented 的 nightly flag 可以看到编译器推导出的生命周期
# rustc +nightly -Z verbose-lifetimes src/main.rs

# Clippy 检查代码质量
cargo clippy

# 格式化代码
cargo fmt
```

---

## 参考答案思路（非完整代码）

### Level 1

- **1.1**：直接使用 `.len()` 比较，返回引用。参考 `src/main.rs` 中已有的 `longest` 函数。
- **1.2**：对照省略规则的 3 条，逐一检查每个签名。
- **1.3**：用 `format!` 宏构造 `String`，返回拥有的值。

### Level 2

- **2.1**：生命周期参数 `'a` 将 x、y 和返回值绑定。泛型 `T` 是元素类型，`F: Fn(&T) -> usize` 是闭包约束。调用 `f(x)` 和 `f(y)` 比较。
- **2.2**：实现 `Iterator` trait。`type Item = &'a str;`。使用 `split_once('\n')` 处理每次迭代。

### Level 3

- **3.1**：内部可以用 `split('\n')` 预处理行，存储行和偏移量。`find` 在行中搜索，`search_all` 过滤，`excerpt` 通过索引范围取行。生命周期上，结构体的 `'a` 标注保证了返回引用的正确性。
