# 练习：泛型与特征 (Generics & Traits)

> 从 Python 迁移到 Rust 的关键一步：理解编译期多态

---

## 难度说明

| 图标 | 难度 | 说明 |
|------|------|------|
| ⭐ | Level 1 | 修改现有代码，理解基本语法 |
| ⭐⭐ | Level 2 | 独立实现新功能，需要综合理解 |
| ⭐⭐⭐ | Level 3 | 设计型练习，需要架构思维 |
| 💡 | 思考题 | 不写代码，深层理解 |

---

## ⭐ 练习 1：实现自定义类型的 Summary

**目标**：为新的结构体实现 Summary 特征，理解 trait 实现语法。

**要求**：

创建一个新的结构体 `PodcastEpisode`，包含字段：
- `title: String`
- `host: String`
- `duration_minutes: u32`
- `transcript_preview: String`

为 `PodcastEpisode` 实现 `Summary` trait：
- `summarize()`：返回格式 `"🎙️ {title} by {host} [{duration}min]"` 
- `author()`：返回 `host` 字段

在 main() 中创建至少一个 `PodcastEpisode` 实例，测试 `generate_report()` 和 `notify()`。

**验证**：
```bash
cargo run
# 应该看到 PodcastEpisode 的摘要输出
```

**提示**：参考 `NewsArticle` 的 trait 实现方式。

---

## ⭐ 练习 2：理解 where 子句

**目标**：将 `<T: Trait>` 语法改写为 where 子句，理解语法糖的本质。

**要求**：

将 `main.rs` 中的以下函数改写为使用 where 子句：

```rust
// 改写前
pub fn generate_report<T: Summary>(item: &T) -> String { ... }

// 改写后（在你的练习代码中）
pub fn generate_report<T>(item: &T) -> String
where
    T: Summary,
{ ... }
```

同样改写 `notify` 函数。

在 main() 中添加注释说明两种语法的使用场景偏好。

**验证**：
```bash
cargo run
# 输出应与改写前完全一致
```

---

## ⭐ 练习 3：泛型结构体实践

**目标**：创建自己的泛型结构体，为它实现方法。

**要求**：

创建泛型结构体 `Pair<A, B>`（如果还不存在的话）：
```rust
struct Pair<A, B> {
    first: A,
    second: B,
}
```

为 `Pair<A, B>` 实现：
1. `new(first: A, second: B) -> Self`：构造函数
2. `swap(self) -> Pair<B, A>`：交换 first 和 second（消费 self）
3. 为 `Pair<A, B> where A: Summary, B: Summary` 实现方法 `describe()`，返回两个元素的摘要拼接

在 main() 中创建 `Pair<NewsArticle, Tweet>` 并调用 `describe()`。

**验证**：
```bash
cargo run
# 应该看到 Pair 的 describe() 输出
```

---

## ⭐⭐ 练习 4：自定义 trait 与多重约束

**目标**：定义自己的 trait，实现多重约束的泛型函数。

**要求**：

1. 定义一个新 trait `HasPriority`：
   ```rust
   trait HasPriority {
       fn priority(&self) -> u32;
       fn is_urgent(&self) -> bool {
           self.priority() >= 80  // 默认：priority >= 80 表示紧急
       }
   }
   ```

2. 为 `NewsArticle` 和 `Tweet` 实现 `HasPriority`：
   - NewsArticle 的 priority 基于内容长度（超过 200 字符 priority=70，否则 30）
   - Tweet 的 priority 基于 retweet_count（超过 1000 priority=90，超过 100 priority=50，否则 20）

3. 实现泛型函数 `print_priority_breakdown`：
   ```rust
   fn print_priority_breakdown<T>(item: &T)
   where
       T: Summary + HasPriority + Debug,
   {
       // 打印: 名称（来自 summarize）、优先级、是否紧急、调试信息
   }
   ```

4. 在 main() 中调用 `print_priority_breakdown` 验证。

**验证**：
```bash
cargo run
# 应看到优先级分解输出
```

---

## ⭐⭐ 练习 5：泛型数据容器

**目标**：实现一个泛型容器类型，结合 Iterator 和特征约束。

**要求**：

1. 创建泛型结构体 `Timeline<T>`：
   ```rust
   struct Timeline<T> {
       items: Vec<T>,
   }
   ```

2. 为 `Timeline<T>` 实现：
   - `new() -> Self`：创建空的 Timeline
   - `add(&mut self, item: T)`：添加元素
   - `len(&self) -> usize`：返回元素数量

3. 实现 `Timeline<T: Summary>` 的方法：
   - `summarize_all(&self) -> Vec<String>`：返回所有元素的摘要

4. 实现 `IntoIterator for Timeline<T>`（consuming iterator），让 Timeline 可以被 `for item in timeline` 使用。

5. 在 main() 中创建 `Timeline<Tweet>`，添加 3 条推文，用 for 循环和 summarize_all() 测试。

**验证**：
```bash
cargo run
# 应看到 3 条推文的摘要
```

---

## ⭐⭐⭐ 练习 6：设计一个内容发布系统

**目标**：综合运用泛型、特征、trait bound 设计一个小型系统。

**要求**：

设计一个内容发布系统，满足以下需求：

1. **定义 `Publishable` trait**：
   ```rust
   trait Publishable {
       fn title(&self) -> &str;
       fn body(&self) -> &str;
       fn is_published(&self) -> bool;
       fn publish(&mut self);
   }
   ```

2. **定义 `ContentPipeline<T: Publishable + Summary>` 结构体**：
   - 字段：`drafts: Vec<T>`, `published: Vec<T>`
   - 方法：
     - `new() -> Self`
     - `add_draft(&mut self, item: T)`：添加草稿
     - `publish_all(&mut self)`：将所有未发布的草稿标记为发布，并移到 published 列表
     - `list_published(&self) -> Vec<String>`：返回所有已发布内容的摘要
     - `stats(&self) -> (usize, usize)`：返回 (草稿数, 已发布数)

3. **为 NewsArticle 实现 Publishable**（NewsArticle 新增一个 `published: bool` 字段）：
   - `title()` 返回 `headline`
   - `body()` 返回 `content`
   - `is_published()` 返回 `published`
   - `publish()` 将 `published` 设为 `true`

4. **为 Tweet 实现 Publishable**（Tweet 新增一个 `published: bool` 字段）：
   - `title()` 返回 `"Tweet by @{username}"`
   - `body()` 返回 `content`
   - 其余类似

5. 在 main() 中创建 `ContentPipeline<NewsArticle>` 和 `ContentPipeline<Tweet>`，演示完整的发布流程。

**验证**：
```bash
cargo run
# 应看到完整的发布系统输出
```

**提示**：
- 这个练习需要修改现有的 `NewsArticle` 和 `Tweet` 结构体（或创建新的变体）。
- 考虑是否需要 `Clone` 约束。
- `stats()` 是很简单的，`list_published()` 需要 `T: Summary`。

---

## 💡 思考题：零成本抽象的边界

**不写代码，思考并回答以下问题**（可以在代码中添加注释回答）：

### 问题 1：单态化的代价

Rust 的泛型通过单态化实现零成本抽象，但代价是编译时间增加和二进制体积膨胀。

- 假设你有一个泛型函数 `process<T: Summary>(item: &T)`，在程序中用 20 种不同的具体类型调用了它。编译器会生成多少个函数副本？
- 在什么场景下这种膨胀会成为实际问题？有什么缓解策略？

### 问题 2：静态分派 vs 动态分派的取舍

- 如果你需要把 `NewsArticle`、`Tweet`、`BlogPost` 放在同一个 `Vec` 中，你应该使用什么类型？为什么不能用 `Vec<impl Summary>`？
- 使用 trait object 会失去什么优化机会？

### 问题 3：Rust trait 与 Python Protocol 的根本区别

Python 3.8+ 的 `typing.Protocol` 语法上与 Rust trait 非常相似，但本质上不同：

```python
# Python
class Summary(Protocol):
    def summarize(self) -> str: ...

# Rust
trait Summary {
    fn summarize(&self) -> String;
}
```

- 解释这两种机制在**编译/运行时的行为**上的根本区别。
- Python 的 Protocol 能带来什么好处？它不能带来什么（与 Rust trait 相比）？
- 如果你从 Python 迁移到 Rust，思维上最需要调整的是什么？

### 问题 4：孤儿规则的设计理由

孤儿规则 (Orphan Rule) 禁止为外来类型实现外来特征。如果 Rust 没有这个规则，会出现什么问题？请给出一个具体的冲突场景示例。

---

## 迁移思维练习

> 以下问题帮助你思考 C++ 模板和 Python 鸭子类型如何重新建模为 Rust 的泛型与 Trait 约束。

### 问题 1：C++ 模板和 Rust 泛型的主要设计差异在哪里？

C++ 模板采用"模板 = 替换 + 重新编译"的模式——编译器在实例化时才检查类型是否满足所需的操作，错误信息可能指向模板定义的内部（即臭名昭著的"模板错误墙"）。Rust 的泛型 + Trait Bound 模式要求在定义时声明类型必须满足哪些约束，编译器在**调用点**就能指出"这个类型不满足 Trait X"。这两种设计对代码可维护性和错误信息的质量分别有什么影响？如果你从 C++ 模板代码迁移到 Rust，你在"调试编译错误"上会有怎样的体验变化？

**提示**：Rust 的 Trait Bound 将契约写在签名里，编译器可以在调用侧报错"T 没有实现 Summary"，而不需要展开模板内部再层层嵌套报错。

### 问题 2：Python 鸭子类型需要多少运行时检查，Rust 的 Trait Bound 如何提前到编译期？

Python 的鸭子类型意味着"如果它走路像鸭子、叫起来像鸭子，就可以当鸭子用"——类型安全完全依赖运行时 `AttributeError` 和 `hasattr` 检查。假设你要写一个接收"可摘要"对象的函数，在 Python 中你需要做哪些运行时防护（`hasattr(obj, 'summarize')`、`try/except AttributeError`）？Rust 的 `T: Summary` 如何将这些检查全部消除为编译期的类型验证？这种"提前到编译期"的做法对大型项目重构（比如修改 Summarize trait 的方法签名）有什么好处？

**提示**：编译期检查意味着重构时编译器会精确地告诉你每个受影响的调用点，不会遗漏任何一个；Python 的重构往往需要依赖单元测试的覆盖率。

---

## 推荐命令

```bash
# 编译并运行（快速检查）
cargo run

# 仅检查编译（不生成二进制，更快）
cargo check

# 查看编译器展开的宏和泛型（需要 nightly）
cargo rustc -- -Z unpretty=expanded

# 查看文档
cargo doc --open

# 代码风格检查
cargo clippy

# 自动格式化
cargo fmt
```
