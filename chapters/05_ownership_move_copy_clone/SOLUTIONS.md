# 第五章练习答案 — 所有权、移动与复制

---

## 练习 1-1: 预测输出 —— Move 与 Copy

### 结论

`i32` 是 Copy 类型，赋值后原变量仍可用；`String` 非 Copy，赋值是 Move，原变量失效。

### 思路

1. `a = 10`，`b = a` 是 Copy（i32 实现了 Copy），`c = b` 同样是 Copy，三者独立。
2. `s = String::from("Rust")`，`t = s` 是 Move，`s` 失效；`u = t` 又是 Move，`t` 失效。
3. 取消注释 `println!("s = {}, t = {}, u = {}", s, t, u)` 后，`s` 和 `t` 都被 moved，编译器报错 `use of moved value`。
4. 只打印 `u` 可编译，因为 `u` 仍是有效的所有者。

### 参考实现（修复后，两种方法）

```rust
// 方法 A: 重新分配，不依赖旧变量
fn main() {
    let a = 10;
    let b = a;
    let c = b;
    println!("a = {a}, b = {b}, c = {c}");

    let s = String::from("Rust");
    let t = s.clone();       // 显式深拷贝
    let u = t.clone();
    println!("s = {s}, t = {t}, u = {u}");
}

// 方法 B: 全部 clone 原始值
fn main() {
    let a = 10;
    let b = a;
    let c = b;
    println!("a = {a}, b = {b}, c = {c}");

    let s = String::from("Rust");
    let t = String::from("Rust");
    let u = String::from("Rust");
    println!("s = {s}, t = {t}, u = {u}");
}
```

### 常见错误

- 以为 `let t = s;` 后 `s` 还能用 —— 这是从 Python/C++ 带来的惯性思维。Rust 的 Move 是**位拷贝 + 编译器标记原变量失效**，不涉及深拷贝。
- **Move 不是深拷贝**: Move 只复制栈上的指针/长度/容量三元组（对于 String 是 24 字节），堆上的字符串内容完全不复制。这与 C++ 拷贝构造的深拷贝完全不同。
- 误以为所有数字类型赋值都是 Move —— 实际上标量类型都实现了 Copy，赋值是复制。

### 验证方式

```bash
cargo run --bin ex1_1
# 注释掉 println! 那行编译通过
# 取消注释后编译失败，观察 use of moved value 错误
```

---

## 练习 1-2: 函数所有权追踪

### 结论

Drop 顺序是：先 `t1`（在 `take_it` 函数结束时），再 `t2`（在 `main` 结束时）。

### 思路

1. `t1` 声明后通过 `take_it(t1)` 转移所有权到函数参数 `t`。
2. `take_it` 内部 `t` 离开作用域 → `Tracker #1 被 drop`。
3. `t2` 声明在 `t1` 之后，`t2` 留在 `main` 的作用域。
4. `main` 结束时，局部变量按声明顺序逆序 drop → `t2` 先 drop，但 `t1` 早已被 drop。
5. 取消注释 `println!("t1.id = {}", t1.id);` 会报 `use of moved value`，因为 `t1` 的所有权已经给了 `take_it`。

### 参考实现（完整代码含 Drop 追踪）

```rust
struct Tracker {
    id: u32,
}

impl Drop for Tracker {
    fn drop(&mut self) {
        println!("Tracker #{} 被 drop", self.id);
    }
}

fn take_it(t: Tracker) {
    println!("take_it: 我拿到了 Tracker #{}", t.id);
    // t 在这里被 drop
}

fn main() {
    let t1 = Tracker { id: 1 };
    println!("main: 创建了 t1");
    take_it(t1);
    // println!("t1.id = {}", t1.id);  // 编译错误: t1 已 moved
    println!("main: take_it 返回了");

    let t2 = Tracker { id: 2 };
    println!("main: 创建了 t2 (不使用)");
    // t2 在这里被 drop
    println!("main: 马上结束了");
}
```

输出顺序：
```
main: 创建了 t1
take_it: 我拿到了 Tracker #1
Tracker #1 被 drop
main: take_it 返回了
main: 创建了 t2 (不使用)
main: 马上结束了
Tracker #2 被 drop
```

### 常见错误

- 以为 `t1` 和 `t2` 都在 main 结束时逆序 drop —— 遗漏了 `t1` 已经在 `take_it` 中 drop。
- 以为传递给函数后变量还能用（Python 思维）。
- 不理解逆序 drop 规则：同一作用域内按声明逆序 drop，但一旦所有权转移，drop 发生在新作用域。

### 验证方式

```bash
cargo run  # 直接观察输出顺序
# 取消注释报错行，cargo build 观察 use of moved value
```

---

## 练习 1-3: 哪些类型是 Copy?

### 结论

| 类型 | Copy? | 理由 |
|------|-------|------|
| `u64` | 是 | 标量类型，编译器自动实现 Copy |
| `&str` | 是 | 引用（胖指针: 指针 + 长度），引用始终 Copy |
| `String` | 否 | 拥有堆数据，位拷贝会造成 double free |
| `Vec<i32>` | 否 | 拥有堆数据，与 String 同理 |
| `(i32, bool)` | 是 | 所有字段都是 Copy → 元组是 Copy |
| `(String, i32)` | 否 | 含非 Copy 字段 String → 元组非 Copy |
| `[i32; 10]` | 是 | 数组元素全为 Copy → 数组是 Copy（栈上复制） |
| `[String; 3]` | 否 | 含非 Copy 元素 → 数组非 Copy |
| `Box<i32>` | 否 | 拥有堆分配，类似 String |
| `Option<i32>` | 是 | `i32` 是 Copy → `Option<i32>` 是 Copy |
| `Option<String>` | 否 | 含非 Copy 的 String → 整体非 Copy |

### 思路

判断规则：一个类型是 Copy 当且仅当它的所有字段（或其包裹的类型）都是 Copy。堆分配类型（String、Vec、Box）都不是 Copy，因为它们的位拷贝会导致两个指针指向同一堆内存，drop 时 double free。

### 参考实现（验证程序）

```rust
fn main() {
    // 这些是 Copy —— 赋值后原变量仍可用
    let a: u64 = 42;
    let _b = a;
    println!("a = {a}");  // OK

    let s: &str = "hello";
    let _t = s;
    println!("s = {s}");  // OK

    let t: (i32, bool) = (1, true);
    let _t2 = t;
    println!("t = {t:?}"); // OK

    let arr: [i32; 10] = [0; 10];
    let _arr2 = arr;
    println!("arr = {arr:?}"); // OK

    let opt: Option<i32> = Some(10);
    let _opt2 = opt;
    println!("opt = {opt:?}"); // OK

    // 这些非 Copy —— 赋值后原变量失效
    let string = String::from("hello");
    let _string2 = string;
    // println!("string = {string}"); // ERROR: moved

    let v = vec![1, 2, 3];
    let _v2 = v;
    // println!("v = {v:?}"); // ERROR: moved

    let mixed = (String::from("hi"), 42);
    let _mixed2 = mixed;
    // println!("mixed = {mixed:?}"); // ERROR: moved

    let boxed = Box::new(42);
    let _boxed2 = boxed;
    // println!("boxed = {boxed}"); // ERROR: moved

    let opt_s = Some(String::from("hi"));
    let _opt_s2 = opt_s;
    // println!("opt_s = {opt_s:?}"); // ERROR: moved

    println!("所有 Copy 类型验证通过");
}
```

### 常见错误

- 以为 `[i32; 10]` 是堆分配的 —— 数组在栈上，`[i32; 10]` 的所有元素都在栈上，是 Copy。
- 以为 `&str` 是堆分配的 —— 它是胖指针（引用），引用本身是 Copy。
- 混淆 "包含堆数据" 和 "引用堆数据": `&str` 引用堆数据但不拥有它，所以引用本身是 Copy；`String` 拥有堆数据，所以不是 Copy。

### 验证方式

取消注释报错行，观察编译器错误信息中的 `use of moved value`，对比 Copy 类型无报错。

---

## 练习 2-1: 重构所有权设计

### 结论

函数不需要所有权时，改用借用（`&`）接收参数，消除不必要的 clone。本题在学完第 6 章引用后才能完美解决，但核心思路是：**只读访问不需要所有权**。

### 思路

1. `print_title` 只需读取 title，不需要拥有 Document → 改为 `&Document`。
2. `word_count` 只需读取 content，不需要拥有 Document → 改为 `&Document`。
3. main 中不再需要 clone，直接传引用。

### 参考实现

```rust
struct Document {
    title: String,
    content: String,
}

fn print_title(doc: &Document) {
    println!("标题: {}", doc.title);
}

fn word_count(doc: &Document) -> usize {
    doc.content.split_whitespace().count()
}

fn main() {
    let doc = Document {
        title: String::from("所有权入门"),
        content: String::from("Rust 的所有权系统是它最独特的特性"),
    };

    print_title(&doc);
    let count = word_count(&doc);
    println!("字数: {count}");

    // 原始 doc 仍然可用！
    println!("原始标题: {}", doc.title);
    println!("原始内容: {}", doc.content);
}
```

### 常见错误

- 遇到 move error 就加 `.clone()` —— 这是回避问题而非解决问题。clone 有性能代价（堆分配 + 数据复制）。
- 过度获取所有权：函数只需要读取数据时，参数应为 `&T` 而非 `T`。
- 在 `print_title` 和 `word_count` 中都拿走所有权，导致 main 中只能调用一个函数。

### 验证方式

```bash
cargo build  # 编译通过，无 clone
cargo run    # 输出正确，所有变量都可访问
```

---

## 练习 2-2: 文件处理模拟 (LogEntry)

### 结论

"拿走再返回"（take-and-return-back）模式是 Rust 在没有引用的前提下处理所有权的惯用手段，但非常笨拙 —— 这正是第 6 章借用要解决的问题。

### 思路

1. `create_log` 获取所有字符串所有权，转移给 LogEntry。
2. `format_log` 拿走 LogEntry 所有权，操作后返回（String, LogEntry）元组。
3. `process_logs` 用 `for` 循环遍历 Vec，消耗所有权或用索引借位处理。

### 参考实现

```rust
#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    message: String,
    level: String,
}

fn create_log(timestamp: String, message: String, level: String) -> LogEntry {
    LogEntry { timestamp, message, level }
}

fn format_log(entry: LogEntry) -> (String, LogEntry) {
    let formatted = format!(
        "[{}] [{}] {}",
        entry.timestamp, entry.level, entry.message
    );
    (formatted, entry)
}

fn process_logs(entries: Vec<LogEntry>) -> Vec<LogEntry> {
    // 方法 1: 消耗 Vec 的所有权，逐个取出
    let mut processed = Vec::new();
    for entry in entries {
        let (formatted, entry) = format_log(entry);
        println!("{formatted}");
        processed.push(entry);
    }
    // 或者用 drain 逐个移出而不消耗 Vec
    processed
}

fn main() {
    let log = create_log(
        String::from("2024-01-01"),
        String::from("Server started"),
        String::from("INFO"),
    );

    let logs = vec![log];
    let remaining = process_logs(logs);
    println!("处理后的日志: {remaining:?}");
}
```

### 常见错误

- `for entry in entries` 消耗 Vec，但如果在循环体内试图再次使用 `entries` 会失败。
- 忘记返回 LogEntry 导致后续代码无法访问。
- 使用 clone 绕过所有权 —— 失去练习意义。

### 验证方式

```bash
cargo run  # 输出格式化日志，确认 remaining 仍有数据
```

---

## 练习 3-1: 字符串池 (StringPool)

### 结论

`std::mem::take()` 是 Rust 中"移出所有权并留下默认值"的标准工具。所有权设计直接影响 API 的可用性。

### 思路

1. `add()` 中 `s.to_uppercase()` 已经消费了 `s`（to_uppercase 获取 `self` 所有权），不需要再用 `s`。
2. `take_all()` 不能用 clone —— 必须真正移出数据。`std::mem::take(&mut self.strings)` 会将 `strings` 替换为空的 `Vec::new()` 并返回原来的数据。
3. Drop 实现中打印所有被释放的字符串。

### 参考实现

```rust
use std::mem;

struct StringPool {
    strings: Vec<String>,
}

impl StringPool {
    fn new() -> Self {
        StringPool { strings: Vec::new() }
    }

    fn add(&mut self, s: String) {
        // s 的所有权传入，to_uppercase() 消费 s 的所有权并返回新 String
        let processed = s.to_uppercase();
        self.strings.push(processed);
        // s 已经失效，processed 的所有权在 strings 中
    }

    fn take_all(&mut self) -> Vec<String> {
        // std::mem::take: 将 self.strings 替换为默认值 (Vec::new())，
        // 返回原来的数据 —— 零 clone 移出所有权
        mem::take(&mut self.strings)
    }

    fn len(&self) -> usize {
        self.strings.len()
    }
}

impl Drop for StringPool {
    fn drop(&mut self) {
        println!("StringPool 被释放: {} 个字符串:", self.len());
        for s in &self.strings {
            println!("  - {s}");
        }
    }
}

fn main() {
    let mut pool = StringPool::new();

    pool.add(String::from("hello"));
    pool.add(String::from("rust"));
    pool.add(String::from("ownership"));

    println!("池中有 {} 个字符串", pool.len());

    let strings = pool.take_all();
    println!("取出了 {} 个字符串:", strings.len());
    for s in &strings {
        println!("  - {s}");
    }

    println!("池中还有 {} 个", pool.len()); // 0

    // pool 在这里被 drop —— strings 为空，Drop 打印空列表
    // strings 在这里被 drop —— 释放三个 String
}
```

所有权流转注释（关键点）：

```
hello (main) → pool.add() → s → to_uppercase() → processed → self.strings[0]
rust  (main) → pool.add() → s → to_uppercase() → processed → self.strings[1]
owner (main) → pool.add() → s → to_uppercase() → processed → self.strings[2]
                              ↓
              take_all(): self.strings → strings (Vec 所有权转移给 main)
                              ↓
              main 中的 strings → 离开作用域时 drop（三个 String 被释放）
```

### 常见错误

- 用 `self.strings.clone(); self.strings.clear();` 来做 take_all —— clone 做了不必要的堆分配。
- 忘记 Drop 实现中不应在 release 之后访问已释放的数据。
- `pool.add()` 获取所有权后原始数据不能用 —— 这在"池"场景合理（数据转交池管理），但若调用方仍需使用原始数据，应改为借用 `&str`。

### API 设计思考

- **合理场景**: 数据生产者将数据交给池管理，自己不再关心这些数据（如日志收集器）。
- **不合理场景**: 调用方仍需要原始数据做后续处理 —— 此时 API 应接受 `&str` 借用，内部 clone 或使用 `Cow<str>`。
- **改进方案**: 提供两个版本的 `add` —— `add_owned(String)` 获取所有权，`add_ref(&str)` 借用 + clone。

### 验证方式

```bash
cargo run --bin ex3_1
# 观察:
# - add 后 pool 大小正确
# - take_all 后 pool 为空 (len == 0)
# - strings 变量在 main 中可用
# - Drop 输出正确
```

---

## 思考题: 为什么 Rust 不让所有类型都默认实现 Copy?

### 结论

Rust 选择"默认 Move，显式 Clone"而非"默认 Copy"，是基于性能、安全和语义清晰性三位一体的设计决策。

### 回答（完整版）

**1. 性能**: 如果 `String` 是 Copy，每次赋值都会触发整个堆内容的深拷贝。一个 1MB 的字符串经过 10 次函数调用就是 10MB 的内存分配和复制，这在不知不觉中就会造成严重的性能问题。Rust 的 Move 只是栈上指针三元组的位拷贝（24 字节），堆数据完全不碰，O(1) 开销。当用户确实需要深拷贝时，通过 `.clone()` 显式声明，让性能代价可见。

**2. 安全性**: 如果 `String` 是 Copy（位拷贝复制），两个变量将指向同一块堆内存。当它们先后离开作用域时，会触发两次 `free()`——这就是经典的 double free，属于未定义行为，可能导致程序崩溃或安全漏洞。Rust 的 Move 通过编译期追踪保证只有一个所有者，消除了 double free。

**3. 语义清晰性**: `Copy` 特质的语义是"位拷贝即可安全复制"。对于 String，其内部结构是 `{ ptr, len, cap }`，位拷贝只复制这三个字段而不复制堆内容（浅拷贝），这违反了"赋值即独立副本"的直觉。Python 的字符串语义上是不可变的且由 GC 管理，所以"看起来是复制"没问题；Rust 没有 GC，必须区分浅拷贝和深拷贝。

- **Move 不是深拷贝**: Rust 的 Move（如 `let t = s;`）是编译器层面的"所有权转移标记"——底层就是 memcpy 24 字节（指针 + 长度 + 容量），然后编译器将原变量标记为失效。堆上的字符串数据完全未被复制。这与深拷贝（需要 malloc + memcpy 整个堆内容）有本质区别。

**4. 设计哲学**: "默认 Move" 把最危险的操作（共享堆数据）变成需要显式声明的操作。在 C++ 中，默认是拷贝，程序员必须记得 `std::move`；在 Rust 中，默认是 Move，程序员必须显式 `.clone()`。Rust 的设计理念是：让安全、高效的操作成为默认，让有代价的操作需要显式声明。

**Rust Move 不等于 C++ std::move**: C++ 的 `std::move` 只是将左值转换为右值引用，原对象仍然存在（处于"有效但未指定状态"），可以继续使用——这是程序员的纪律问题，编译器不保证你不会用。Rust 的 Move 在编译期强制原变量失效，任何后续使用都是编译错误。两者看似相似，本质完全不同：一个是库函数 + 程序员自律，一个是编译器保证 + 类型系统强制。

---

## 迁移思维练习答案

### 1. C++ 中 shared_ptr 共享数据的方式，在 Rust 中应该怎么重新建模？

C++ 的 shared_ptr 对应 Rust 的 Rc<T>（单线程）或 Arc<T>（多线程）。但 Rust 中应优先考虑：能否使用借用（&T）替代共享所有权？能否让数据只有一个明确的所有者？shared_ptr 在 C++ 中常被用作"万能指针"——因为不确定谁最后使用数据就直接共享。在 Rust 中，共享所有权是显式选择，有其代价：引用计数的运行开销、无法直接可变访问内部数据（需要 Cell/RefCell/Mutex 配合）。迁移时应该先梳理数据的真实生命周期，而非盲目用 Rc 替代 shared_ptr。

### 2. 遇到所有权错误就用 clone() 解决的做法有什么问题？

clone() 绕过了设计问题而不是解决了它。每次 clone 都意味着堆分配和数据复制，在循环或高频调用中累积的成本可能很高。更重要的是，频繁 clone 通常意味着没有正确使用借用（&T），或者数据结构的所有权设计需要重新考虑。正确的做法是：先尝试借用（加生命周期参数或重组代码），再考虑重组数据结构（让上层持有数据，下层只借用），clone 是最后的选择。

### 3. Rust 的 Move 语义和 C++ 的 std::move 有什么关键区别？

Rust 的 Move 是默认行为：当你不实现 Copy trait 时，赋值、传参、返回都会 Move，原绑定立即失效，编译器保证后续不再使用。C++ 的 std::move 只是将左值转换为右值引用，原对象仍"存在"（处于"有效但未指定"状态），可以继续使用——这完全是程序员的纪律问题。Rust 的 Move 是位拷贝加编译期追踪，零运行时开销；C++ 的移动构造可能有编译器生成的代码。

---

## 练习提交检查清单

- [x] 练习 1-1: 能解释 Copy 和 Move 的行为差异
- [x] 练习 1-2: 能正确预测 Drop 的调用顺序
- [x] 练习 1-3: 能准确判断 10 种类型的 Copy 属性
- [x] 练习 2-1: 能重构代码消除不必要的 clone
- [x] 练习 2-2: 实现了 LogEntry 相关函数, 理解所有权"拿走再还"模式
- [x] 练习 3-1: 修复了 StringPool, 理解了所有权在 API 设计中的角色
- [x] 思考题: 写出了分析（含 Move 不是深拷贝 和 Rust Move 不等于 C++ std::move）

---

*练习是掌握 Rust 所有权的唯一捷径。每一个 move error 都是你通往 Rust 思维模式的阶梯。*
