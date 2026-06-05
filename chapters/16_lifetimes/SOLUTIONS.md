# 第16章 练习答案 — 生命周期 (Lifetimes)

> 核心观念：**生命周期标注 (lifetime annotations) 描述引用之间的关系，不改变值实际存活的时间**。标注是你和编译器之间的契约，编译器验证你的承诺是否成立。

---

## 生命周期省略规则（Elision Rules）速查

编译器自动推导生命周期的 3 条规则：

| 规则 | 描述 | 示例 |
|------|------|------|
| 规则 1 | 每个引用参数各自获得独立的生命周期参数 | `fn foo(x: &str, y: &str)` → `fn foo<'a, 'b>(x: &'a str, y: &'b str)` |
| 规则 2 | 若只有一个输入生命周期参数，则赋给所有输出引用 | `fn foo(x: &str) -> &str` → `fn foo<'a>(x: &'a str) -> &'a str` |
| 规则 3 | 若有 `&self` / `&mut self`，其生命周期赋给所有输出引用 | `fn foo(&self, x: &str) -> &str` → `fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str` |

---

## 修复优先级框架（Fix-Priority Framework）

当遇到生命周期编译错误时，按以下优先级处理：

1. **检查是否真的需要返回引用**：能否返回拥有所有权的值（如 `String` 而非 `&str`）？
2. **考虑数据重组**：字段是否应该用 `Arc` 共享而非引用？
3. **最后才加生命周期标注**：仅在前两步不可行时才手动标注

---

## Level 1：基础理解

### 练习 1.1：最长字符串切片

#### 结论

`fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str` 返回两个切片中较长的那个。`'a` 将 x、y 和返回值的生命周期绑定在一起——返回值不会比两个参数中**较短的那个**活得更久。

#### 思路

使用 `.len()` 比较字节长度，返回引用。

#### 参考实现

```rust
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// 在 main 中使用:
fn main() {
    println!("longest_str(\"hello\", \"world!\") = \"{}\"",
             longest_str("hello", "world!"));
    println!("longest_str(\"短\", \"比较长的字符串\") = \"{}\"",
             longest_str("短", "比较长的字符串"));
}
```

#### 为什么这样设计

- 返回值必须和**两个**输入引用都兼容。编译器不知道运行时哪个分支会执行，所以取最保守的约束：返回值不能比最短命参数的活得久。`'a` 实际上等于 `min('x_lifetime, 'y_lifetime)`。
- `&str` 的 `.len()` 返回字节长度而非字符数。对于纯 ASCII 没问题，对 Unicode 注意这一点。

#### 常见错误

- 忘记生命周期标注：多个输入引用 + 返回引用时，编译器无法自动推导
- 误以为 `'a` 是"让值活得更久"——它只是描述关系
- 返回 `String` 而不是 `&str` 时不需要生命周期，但会有内存分配

#### 验证方式

```bash
cargo test test_longest
cargo run
```

---

### 练习 1.2：识别省略规则

#### 结论

对照 3 条省略规则逐一判断：

| 签名 | 能否推导 | 原因 |
|------|----------|------|
| ① `fn foo(x: &str) -> &str` | ✅ 可以 | 规则 2：单输入 → 赋给输出 |
| ② `fn bar(x: &str, y: &str) -> &str` | ❌ 不可以 | 多个输入引用，编译器不知道关联哪个 |
| ③ `fn baz(x: &i32, y: &i32) -> i32` | ✅ 可以 | 返回 `i32`（值，非引用），不需要生命周期 |
| ④ `fn qux(&self, x: &str) -> &str` | ✅ 可以 | 规则 3：`&self` 的生命周期赋给输出 |
| ⑤ `fn quux(x: &str, y: &str, z: &str) -> &str` | ❌ 不可以 | 多个输入引用 + 返回引用，编译器无法确定关联关系 |

#### 为什么这样设计

省略规则覆盖了 **90% 的日常场景**，让你不需要手动写生命周期。只在必要时（多个输入引用 + 返回引用）才需要显式标注。

#### 常见错误

- 误以为规则 3 把输出生命周期关联到了 `x`（实际上关联到了 `&self`）
- 认为返回非引用类型时也需要生命周期标注

---

### 练习 1.3：结构体中的生命周期

#### 结论

结构体持有引用时必须标注生命周期参数。方法返回 `String`（拥有所有权的值）时不需要生命周期标注——这是一个很好的设计模式：**当你想从引用数据中创建新数据时，返回拥有的值可以避免生命周期复杂性**。

#### 参考实现

```rust
struct BookSlice<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> BookSlice<'a> {
    fn description(&self) -> String {
        format!("《{}》，作者：{}", self.title, self.author)
    }
}

// 使用:
fn main() {
    let title = String::from("Rust 程序设计");
    let author = String::from("Klabnik & Nichols");
    let book = BookSlice { title: &title, author: &author };
    println!("{}", book.description());
    // 输出：《Rust 程序设计》，作者：Klabnik & Nichols
}
```

#### 为什么这样设计

- `description()` 返回 `String` 而非 `&str`：避免了复杂的生命周期标注，让调用者独立拥有结果
- 结构体的 `'a` 约束保证 `BookSlice` 不能比它引用的数据活得更久
- 当生命周期标注开始变得复杂时，返回拥有的值通常是更好的设计

#### 常见错误

- 忘记在 `impl` 块上标注生命周期：`impl<'a> BookSlice<'a>`
- `description` 返回 `&str` 却想从 `format!` 返回（`format!` 返回 `String`，不能作为 `&str` 引用返回）

#### 验证方式

```bash
cargo run
# 输出：《Rust 程序设计》，作者：Klabnik & Nichols
```

---

## Level 2：综合应用

### 练习 2.1：自定义 longest 逻辑

#### 结论

`fn longest_by_key<'a, T, F>(x: &'a T, y: &'a T, f: F) -> &'a T`
其中 `F: Fn(&T) -> usize`。这是生命周期 + 泛型 + 闭包约束的组合使用。

#### 参考实现

```rust
fn longest_by_key<'a, T, F>(x: &'a T, y: &'a T, f: F) -> &'a T
where
    F: Fn(&T) -> usize,
{
    if f(x) >= f(y) {
        x
    } else {
        y
    }
}

// 测试:
#[test]
fn test_longest_by_key() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = longest_by_key(&s1, &s2, |s: &String| s.len());
    assert_eq!(result, &s2);

    let result2 = longest_by_key(&s1, &s2, |s: &String| s.chars().filter(|c| *c == 'l').count());
    assert_eq!(result2, &s1);
}
```

#### 为什么这样设计

- `'a` 将 `x`、`y` 和返回值的生命周期绑定
- `F: Fn(&T) -> usize` 约束闭包接收 `&T` 返回 `usize`
- "一样长时返回 x" 是一个合理的默认约定

#### 常见错误

- 闭包参数类型不匹配（如 `|s: &&String|` vs `|s: &String|`）
- 忘记 `where` 子句或约束格式错误
- 生命周期的 `'a` 和泛型 `T` 的位置混淆

#### 验证方式

```bash
cargo test test_longest_by_key
```

---

### 练习 2.2：实现一个带生命周期的迭代器适配器

#### 结论

`Lines<'a>` 持有 `&'a str` 的引用，实现 `Iterator<Item = &'a str>`。每次 `next()` 返回下一行（`\n` 分隔），不分配新内存。

#### 参考实现

```rust
struct Lines<'a> {
    remaining: &'a str,
    done: bool,
}

impl<'a> Lines<'a> {
    fn new(text: &'a str) -> Self {
        Lines {
            remaining: text,
            done: text.is_empty(),
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.remaining.split_once('\n') {
            Some((line, rest)) => {
                self.remaining = rest;
                Some(line)
            }
            None => {
                self.done = true;
                Some(self.remaining)
            }
        }
    }
}

// 测试:
#[test]
fn test_lines() {
    let text = "第一行\n第二行\n第三行";
    let lines = Lines::new(text);
    let collected: Vec<&str> = lines.collect();
    assert_eq!(collected, vec!["第一行", "第二行", "第三行"]);
}

#[test]
fn test_lines_empty() {
    let empty = Lines::new("");
    let empty_collected: Vec<&str> = empty.collect();
    assert!(empty_collected.is_empty());
}
```

#### 为什么这样设计

- `type Item = &'a str` 将返回引用的生命周期绑定到输入文本
- `done` 标记处理了空输入和最后一行（没有 `\n` 结尾的情况）
- `split_once('\n')` 返回 `Option<(&str, &str)>`，零分配

#### 常见错误

- 空输入时应直接标记 `done = true`，否则 `next()` 会返回 `Some("")`
- 忘记 `impl<'a> Iterator for Lines<'a>` 中的生命周期标注
- `type Item` 的关联类型声明

#### 验证方式

```bash
cargo test test_lines
```

---

## Level 3：挑战题

### 练习 3.1：上下文搜索器

#### 结论

`SearchContext<'a>` 持有 `&'a str` 文本引用，提供 `find`、`search_all`、`excerpt` 三个搜索方法。注意 `search_all<'b>(&'b self, ...) -> Vec<&'a str>` 中返回的引用生命周期 `'a`（来自结构体）比方法调用的借用 `'b` 更长——这是典型的"返回值生命周期独立于方法接收者"模式。

#### 参考实现

```rust
struct SearchContext<'a> {
    text: &'a str,
}

impl<'a> SearchContext<'a> {
    fn new(text: &'a str) -> Self {
        SearchContext { text }
    }

    fn find(&self, query: &str) -> Option<&'a str> {
        if query.is_empty() {
            return None;
        }
        self.text.lines().find(|line| line.contains(query))
    }

    fn search_all<'b>(&'b self, query: &str) -> Vec<&'a str> {
        if query.is_empty() {
            return Vec::new();
        }
        self.text.lines().filter(|line| line.contains(query)).collect()
    }

    fn excerpt(&self, query: &str, surrounding_lines: usize) -> Vec<&'a str> {
        if query.is_empty() {
            return Vec::new();
        }
        let lines: Vec<&str> = self.text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains(query) {
                let start = i.saturating_sub(surrounding_lines);
                let end = std::cmp::min(i + surrounding_lines + 1, lines.len());
                return lines[start..end].to_vec();
            }
        }
        Vec::new()
    }
}

// 测试:
#[test]
fn test_search_context() {
    let text = "Rust 很强大\nPython 很简单\nRust 很安全\nGo 很快\nRust 很好玩\n";
    let searcher = SearchContext::new(text);

    assert_eq!(searcher.find("Rust"), Some("Rust 很强大"));
    assert_eq!(searcher.find("C++"), None);

    let all: Vec<&str> = searcher.search_all("Rust");
    assert_eq!(all, vec!["Rust 很强大", "Rust 很安全", "Rust 很好玩"]);

    let ctx: Vec<&str> = searcher.excerpt("安全", 1);
    assert_eq!(ctx, vec!["Python 很简单", "Rust 很安全", "Go 很快"]);
}
```

#### 为什么这样设计

- `SearchContext` 不拥有文本，只借用——零拷贝设计
- `search_all` 返回 `Vec<&'a str>`，生命周期是 `'a`（结构体的生命周期）而非 `'b`（方法的借用生命周期）——这意味着返回的引用在结构体存在期间始终有效
- `excerpt` 使用 `saturating_sub` 处理边界，避免 underflow
- 空 query 直接返回空结果，避免无意义匹配

#### 常见错误

- 混淆 `'a` 和 `'b` 的含义
- `excerpt` 中 `start` 可能 underflow（需要用 `saturating_sub`）
- 没有处理空 query 的情况
- 在 `search_all` 中错误地将生命周期绑定到 `&'b self`（会导致返回的引用比需要的时间更短）

#### 验证方式

```bash
cargo test test_search_context
```

---

## 思考题：为什么 Rust 选择编译时生命周期检查而不是运行时 GC？

### 1. 性能

编译时检查是**零成本抽象**：所有验证在编译期完成，运行时无额外开销。GC 需要暂停程序、扫描内存、标记可达对象——这些暂停在实时系统、游戏、高频交易中不可接受。生命周期标注不产生任何运行时代码：`longest<'a>` 编译后的机器码和手写指针操作完全一致。

### 2. 确定性

GC 暂停时间是不可预测的（取决于堆大小、对象数量）。生命周期检查的编译时间是确定的（只取决于代码复杂度，不影响运行时）。这对于：
- **实时系统**（自动驾驶、航空）：GC 暂停可能导致灾难
- **嵌入式**（资源受限）：GC 的内存开销不可接受
- **音频/视频处理**：帧率要求严格的确定性延迟

### 3. 与其他系统语言的比较

C 的 `free` 问题发生在运行时——程序崩溃或数据损坏。Rust 在编译时捕获 use-after-free 和 double-free。编译期错误比运行时崩溃更好维护：错误信息精确到行，开发者立即得到反馈。

### 4. 与 Python 的比较

Python 程序员不需要写生命周期标注，但代价是：
- **引用计数开销**：每次赋值、传参都涉及原子增/减引用计数
- **GC 暂停**：循环引用需要分代 GC 扫描
- **运行时错误**：`None` 对象的方法调用、已关闭文件的读写等运行时错误
- Rust 把这些检查提前到编译期：编辑器里标红的代码不会进入生产环境

### 5. 设计权衡

如果 Rust 选择 GC：
- 它将成为"又一个带 GC 的安全语言"，失去零成本抽象的核心卖点
- 无法用于操作系统内核、嵌入式、WebAssembly 等无 GC 环境
- 失去与 C ABI 的无缝交互能力
- Rust 的选择：**用编译期的复杂度换取运行时的简单性和安全性**

---

## 迁移思维练习答案

### 1. 在 C/C++ 中哪些场景会出现"use-after-free"，Rust 如何通过生命周期防止？

最常见的有三类：返回局部变量的指针/引用（悬挂指针）、释放后继续使用（尤其在复杂控制流中，如 free 后偶然路径仍访问）、迭代器失效（如 vector 扩容后旧指针/引用全部失效）。Rust 的生命周期系统在编译期追踪每个引用的有效范围，使用借用检查器验证：任何引用的生命周期必须短于被引用数据的生命周期。任何可能访问已释放数据的代码都会成为编译错误，而非运行时的未定义行为（UB）。

### 2. 什么时候不应该返回引用，而应该返回拥有所有权的值？

当被引用的数据在函数返回后不存在时（如局部变量），必须返回拥有所有权的值。当调用者需要独立拥有数据、而借用会增加生命周期复杂度时。当返回引用会导致复杂的生命周期标注（多个参数之间的生命周期关系），而性能收益不大时。返回 `String` 而不是 `&str`、返回 `Vec<T>` 而不是 `&[T]` 通常让 API 更简单、更灵活——调用者不用关心数据来自哪里、什么时候被释放。先写正确的代码（返回所有权），再在 profiling 确认瓶颈后进行优化。

### 3. 生命周期标注 `'a` 为什么让人困惑，如何理解它？

生命周期标注 `'a` 不改变代码的运行时行为——它只是告诉编译器"这几个引用之间有某种存活时间上的关系"。常见的困惑在于以为 `'a` 在"延长"生命，实际上标注在描述"两个引用至少同时存活的最小时间范围"。理解的关键：标注是你和编译器之间的契约——你说"这个输出引用的数据，至少活得和这个输入引用一样久"，编译器去验证你的承诺是否成立。
