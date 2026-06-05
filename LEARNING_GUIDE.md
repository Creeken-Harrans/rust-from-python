# 学习指南 (Learning Guide)

写给 Python 学习者的 Rust 自学指南。

---

## 如何使用本教程

### 推荐学习流程

每章按以下顺序学习：

1. **先读 README.md 的"本章目标"** — 了解要学什么，建立预期
2. **运行代码** — `cargo run`，看到实际输出，建立感性认识
3. **回头精读 README.md** — 理解概念、规则和设计动机
4. **阅读源码** — 对照 README 中的讲解看 `src/main.rs`
5. **修改代码实验** — 改变参数、添加功能、故意制造错误看编译器反应
6. **完成练习** — 独立完成 `EXERCISES.md` 中的题目
7. **自我验证** — 使用 `cargo check` 或 `cargo test` 验证代码
8. **对照答案** — 查看 `SOLUTIONS.md`，比较设计决策差异
9. **重新实现** — 不复制，独立重新实现核心练习
10. **记笔记** — 用自己的话总结核心概念

> **重要**：第 6-9 步是学习闭环。跳过第 6 步（独立完成）直接看答案，学习效果将大打折扣。参考答案用于比较思路，不是用来复制的。

### 练习答案使用指南

每章提供 `SOLUTIONS.md` 作为参考答案。建议：

1. 先独立完成 `EXERCISES.md` 中全部练习
2. 练习过程中尽量不查阅 `SOLUTIONS.md`
3. 遇到阻塞时，优先查阅 README、源码或编译器错误信息
4. 完成后再打开 `SOLUTIONS.md`，逐题对比思路
5. 关注"为什么这样设计"——答案不仅展示怎么做，更解释决策过程
6. 对于编代码题：理解思路后，关闭答案，重新从零实现

### 建议节奏

- 每天 1-2 章，不要贪多
- 所有权（05-07）这三章可以放慢到每天一章
- 每完成一个阶段后停一天，用综合项目巩固
- 遇到困难时不要跳过——Rust 的概念是层层递进的

---

## 遇到编译错误时

### Rust 的错误信息是你的朋友

Rust 的编译器错误信息被广泛认为是业界最好的之一。每条错误通常包含：

1. **错误描述** — 发生了什么
2. **代码位置** — 在哪一行
3. **原因解释** — 为什么会出错
4. **修复建议** — `help:` 开头的行会给出具体建议
5. **错误码** — 如 `E0382`，可以用 `rustc --explain E0382` 查看详细解释

### 错误阅读策略

```bash
# 1. 先看第一个错误（后面的往往是级联错误）
# 2. 阅读 help: 建议
# 3. 用 rustc --explain 查看详细解释
rustc --explain E0382

# 4. 如果还是不理解，回到对应章节重读
# 5. 用 cargo check 快速迭代（不需要完整编译）
cargo check
```

### 常见错误速查

| 错误码 | 含义 | 参考章节 |
|--------|------|---------|
| E0382 | use of moved value | [05](chapters/05_ownership_move_copy_clone/) |
| E0502 | cannot borrow as mutable and immutable | [06](chapters/06_references_borrowing_slices/) |
| E0499 | cannot borrow as mutable more than once | [06](chapters/06_references_borrowing_slices/) |
| E0507 | cannot move out of borrowed content | [05](chapters/05_ownership_move_copy_clone/), [09](chapters/09_enums_option_pattern_matching/) |
| E0597 | borrowed value does not live long enough | [06](chapters/06_references_borrowing_slices/), [16](chapters/16_lifetimes/) |
| E0106 | missing lifetime specifier | [16](chapters/16_lifetimes/) |
| E0277 | the trait bound is not satisfied | [15](chapters/15_generics_traits_trait_bounds/) |
| E0308 | mismatched types | [02](chapters/02_variables_and_types/) |
| E0369 | binary operation cannot be applied | [02](chapters/02_variables_and_types/) |

更多错误信息参考 [TROUBLESHOOTING.md](TROUBLESHOOTING.md)。

---

## 学习提醒

### 如果你有 C/C++ 背景

- **不要机械映射**：不要把 C++ 经验机械映射到 Rust。相似的语法并不意味着相同的语义。参见 [C_CPP_TO_RUST.md](C_CPP_TO_RUST.md) 了解系统化的概念对照。
- **Rust Move ≠ C++ std::move**：两者有相似目的，但机制和规则不同。
- **RAII 不是 Rust 独有的**：C++ 是 RAII 的起源语言，但 Rust 将 RAII 与所有权系统深度结合。
- **不要遇到报错就堆砌 `clone()`、`Rc<RefCell<_>>` 或 `unsafe`**：这些是工具，不是设计方案。
- **优先写小程序并观察编译器反馈**：编译器是你最好的老师。

### 如果只有 Python 背景

- **不要一次完全掌握生命周期**：生命周期标注是 Rust 的高级特性，大多数代码不需要显式标注。先理解所有权和借用，生命周期会自然到位。
- **接受显式**：从 Python 的"隐式便利"转向 Rust 的"显式控制"。更多的键盘输入换来更多的编译期保证。

---

## Python 学习者的特别建议

### 1. 不要急于滥用 `clone()`

```rust
// ❌ 不好的做法：每次编译错误都用 clone 解决
let s1 = String::from("hello");
let s2 = s1.clone();  // 能用，但是否必要？
let s3 = s1.clone();  // 又 clone 一次？

// ✅ 好的做法：思考是否可以用借用
let s1 = String::from("hello");
let s2 = &s1;  // 借用，零成本
let s3 = &s1;  // 多个不可变借用，没问题
```

**`clone()` 的适用场景**：
- 确实需要一个独立拥有所有权的副本
- 性能不敏感的初始化代码
- 数据结构需要存储值的副本

**`clone()` 的误用场景**：
- 只是为了"让编译通过"
- 可以改用引用的地方
- 在热路径中不必要地复制数据

**经验法则**：如果发现自己频繁使用 `clone()`，停下来思考设计是否需要调整。

### 2. 不要把借用检查器视为敌人

借用检查器（Borrow Checker）不是在刁难你——它在保护你免于：

- **Use-after-free**（使用已释放的内存）：C/C++ 中极难调试的 Bug
- **Double-free**（重复释放）：导致崩溃或安全漏洞
- **Data races**（数据竞争）：并发编程中最隐蔽的问题

每个被拒绝的代码模式都对应着一个真实的安全隐患。当你习惯了借用检查器的规则后，你会开始信任它——它能通过编译的代码，至少不会出现这些内存问题。

### 3. 重新理解"变量"

Python 中的变量是"标签"——你可以随时把标签贴到不同的对象上。

Rust 中的变量是"绑定"——它与所有权（Ownership）关联。赋值可能意味着：
- 移动所有权（Move）：原变量不再有效
- 复制（Copy）：两者都有效（仅限简单类型）
- 借用（Borrow）：临时访问，不影响所有权

这种差异是 Python → Rust 最核心的思维转变。

### 4. 接受函数签名的"重"

Python 函数签名很轻：
```python
def process(data):
    ...
```

Rust 函数签名更重，但携带了更多信息：
```rust
fn process(data: &[String]) -> Result<Vec<&str>, ProcessError> {
    ...
}
```

看一眼签名就知道：
- 函数借用了 String 切片（不获取所有权）
- 可能失败（返回 Result）
- 成功时返回 &str 切片向量（不分配新 String）
- 失败时返回 ProcessError

**这些信息在 Python 中需要读文档或源码才能知道——在 Rust 中它们直接编码在类型签名里。**

### 5. 理解编译期的价值

Python 鼓励"快速迭代、运行时发现错误"。
Rust 鼓励"编译时发现问题、运行时安全执行"。

代价是：写 Rust 代码初期会花更多时间"让编译器满意"。
回报是：一旦编译通过，代码在内存安全和线程安全方面是经过验证的。

**这是一种不同的生产力模型**——前期投入换取后期信心。

---

## 如何使用 Cargo 快速迭代

```bash
# 开发时的最佳实践：
# 1. 用 cargo check 快速验证（比 cargo build 快很多）
cargo check

# 2. 确认无误后再运行
cargo run

# 3. 提交前做完整检查
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 使用本地 Rust 文档

Rust 安装了完整的离线文档：

```bash
# 打开 Rust 官方书籍（The Book）
rustup doc --book

# 打开标准库文档
rustup doc --std

# 打开 Rust 参考手册
rustup doc --reference

# 用 cargo doc 打开本教程的文档
cargo doc --open
```

标准库文档中每个类型和函数都有详细的说明和示例，是日常开发的重要参考。

---

## 什么时候适合开始写自己的项目？

建议在完成以下章节后开始尝试自己的小项目：

**最低要求**（约 15 小时学习）：
- 00-07（基础 + 所有权）
- 08-09（结构体 + 枚举）
- 12（错误处理）

**推荐要求**（约 25 小时学习）：
- 以上全部
- 10（集合类型）
- 13（模块系统）
- 15（泛型与 Trait）

此时你应该能做到：
- 定义自己的数据结构
- 处理文件的读写
- 实现基本的命令行工具
- 写出通过编译的、内存安全的代码

---

## 使用测试确认理解

```bash
# 运行某章的测试
cargo test -p chapter_name

# 运行所有章节的测试
cargo test --workspace

# 在文档中查找用法
cargo doc --open
```

每学完一章，试着不看教程自己重新实现该章的核心功能。如果能独立写出并通过编译，说明你已经掌握了。

---

## 社区资源

- [Rust 官方论坛](https://users.rust-lang.org/) — 友善的社区，适合提问
- [Rust 官方 Discord](https://discord.gg/rust-lang) — 实时交流
- [The Rust Book](https://doc.rust-lang.org/book/) — 官方入门书籍
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 通过示例学习
- [Rustlings](https://github.com/rust-lang/rustlings) — 交互式小练习
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/) — Google 的 Rust 培训教程

---

祝你学习愉快！记住：每个 Rust 程序员都经历过和借用检查器"搏斗"的阶段——这段经历会让你成为更好的程序员。
