# 第六章练习答案 — 引用、借用与切片

---

## 练习 1.1: 识别引用类型

### 结论

`&T` 是不可变借用（只读），`&mut T` 是可变借用（可修改），两者不能在同一作用域重叠。

### 思路

逐行分析每个变量的类型和可变性：

- `a = &original`: 不可变借用，类型 `&String`。共享引用，不能修改。
- `b = &mut original`: 可变借用，类型 `&mut String`。但 `a` 仍活跃 → 违反"不能同时有共享引用和可变引用"规则 → 编译失败。
- `c = original`: 所有权转移，类型 `String`。`c` 现在是所有者，但此后 `original` 失效。声明没有 `mut`，不能修改。
- `d = String::from("Python")`: 所有者，`mut d` 故可修改。
- `e = &d`: 不可变借用，类型 `&String`。但此时 `original` 的 `&mut` `b` 的作用域是否结束了？若 NLL 未覆盖可能仍有冲突。

### 参考实现（完整标注类型）

```rust
fn main() {
    let mut original = String::from("Rust");

    let a = &original;          // 类型: &String, 不可变借用, 不能修改
    // let b = &mut original;   // 类型: &mut String, 可变借用 → 编译失败!
                                  // 原因: a 仍然活跃, 违反借用规则
    let c = original;           // 类型: String, 所有权转移, 无 mut 不能修改
    let mut d = String::from("Python"); // 类型: String, 有 mut 可修改
    let e = &d;                 // 类型: &String, 不可变借用, 不能修改

    println!("a = {a}, c = {c}, d = {d}, e = {e}");
}
```

**哪一行会编译失败**: `let b = &mut original;` 因为 `a`（共享引用）和 `b`（可变引用）同时存在。

### 常见错误

- 以为 `&String` 和 `String` 是同一类型 —— 引用和被引用者是不同的类型。
- 以为 `let mut d` 后 `e = &d` 就可以通过 `e` 修改 —— `&T` 共享引用永远不可变。
- 忽略 NLL（非词法生命周期）：在某些 Rust 版本中，如果 `a` 在 `b` 创建前已不再使用，NLL 可能允许编译。

### 验证方式

```bash
cargo build  # 观察借用冲突错误
# 注释掉 a 或 b 其中一个，再编译通过
```

---

## 练习 1.2: 修复借用错误

### 结论

将 `r1` 的使用移到 `r2` 创建之前即可。NLL 允许在某个引用不再使用后创建"冲突"的新引用。

### 思路

借用规则：可以有多个 `&T`，或一个 `&mut T`，但不能同时存在。`r1`（共享引用）和 `r2`（可变引用）冲突。解决方法：在使用 `r2` 之前让 `r1` "结束"——在 `r2` 声明之前使用 `r1`。Rust 的 NLL 会根据实际最后使用的位置（而非作用域结束）判断引用的生命周期。

### 参考实现

```rust
fn append_world() -> String {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{r1}");          // r1 在这里最后被使用
    // NLL: r1 的实际生命周期到此结束
    let r2 = &mut s;           // ✅ 现在可以创建可变引用了
    r2.push_str(" world");
    let result = s;            // 所有权转移
    result
}

fn main() {
    let s = append_world();
    println!("结果: {s}");     // 输出: hello world
}
```

另一种解法：用花括号限制 `r2` 的作用域：
```rust
fn append_world() -> String {
    let mut s = String::from("hello");
    let r1 = &s;
    {
        let r2 = &mut s;       // r2 的作用域在花括号内
        r2.push_str(" world");
    }                          // r2 在这里结束
    println!("{r1}");          // r1 仍然可用
    let result = s;
    result
}
```

### 常见错误

- 用 `.clone()` 或移除引用来"修复"——回避了借用规则的学习目标。
- 以为必须用 `unsafe` —— 完全不需要。
- 不理解 NLL：Rust 2018 之后引用并不是在作用域结束才 drop，而是在最后一次使用后立即"结束"。

### 验证方式

```bash
cargo run  # 输出 "结果: hello world"
```

---

## 练习 1.3: 元音计数 (count_vowels)

### 结论

**`&str` 优于 `&String`**: `&str` 可直接接受 `String` 的引用（通过 Deref 自动转换）、字符串字面量（`"..."` 本身是 `&str`）、以及子切片。`&String` 只能接受 `&String`，调用方需要显式操作。

### 思路

- 使用 `s.chars()` 迭代 Unicode 字符（不是字节）。
- 元音字母用 match 或 contains 判断。
- `to_ascii_lowercase()` 处理大小写。
- 不分配新 String，全程借用。

### 为什么 &str 比 &String 更好

`&str` 是更通用的类型：它是"字符串数据的借用视图"。通过 `Deref<Target=str>`，任何 `&String` 可以自动转换为 `&str`（这叫 Deref coercion）。但反过来不行：`&str` 不能自动变成 `&String`。所以接受 `&str` 的函数可以接受：`&String`、字面量 `"hello"`、子切片 `&s[0..3]`、`Cow<str>` 的引用等；而接受 `&String` 的版本只能接受 `&String`，调用时需要 `&my_string` 而 `my_string` 必须恰好是 `String` 类型。这违反了 Rust 社区的最佳实践：**尽可能接受借用视图（`&str`）而非具体类型（`&String`）**。

### 参考实现

```rust
fn count_vowels(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.chars()
        .filter(|c| vowels.contains(&c.to_ascii_lowercase()))
        .count()
}

// 或使用 for 循环版本（练习要求）
fn count_vowels_loop(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => {}
        }
    }
    count
}

// 对比: &String 版本的局限
fn count_vowels_string(s: &String) -> usize {
    // 相同的实现，但只能接受 &String
    count_vowels(s.as_str()) // 内部还得转 &str
}

fn main() {
    assert_eq!(count_vowels("Hello World!"), 3);   // e, o, o
    assert_eq!(count_vowels("Rust"), 1);            // u
    assert_eq!(count_vowels("AEIOU"), 5);           // 全部
    assert_eq!(count_vowels("xyz"), 0);             // 没有
    println!("所有测试通过！");

    // &String 版本的调用差异
    let s = String::from("Hello");
    count_vowels(&s);         // ✅ &str 版本: 直接传 &s, Deref 自动转换
    // count_vowels("Hello"); // ✅ &str 版本: 字面量直接传
    count_vowels_string(&s);  // ✅ &String 版本: 可以
    // count_vowels_string("Hello"); // ❌ &String 版本: 字面量不行! "Hello" 是 &str
}
```

### 常见错误

- 参数声明为 `&String` —— 应声明为 `&str`，更通用。
- 用 `s.bytes()` 而非 `s.chars()` —— 会漏掉多字节 Unicode 字符。
- 忘记 `to_ascii_lowercase()` 导致大写元音不被计数。

### 验证方式

```bash
cargo test  # 或用 assert_eq! 在 main 中验证
```

---

## 练习 2.1: split_at_utf8 切片分割

### 结论

`is_char_boundary()` 是 Rust 提供的关键 UTF-8 安全检查 —— 在非字符边界切割字符串切片是未定义行为（panic）。`&str` 的子切片是零拷贝的借用视图。

### 思路

1. 使用 `s.is_char_boundary(mid)` 检查字节索引是否落在有效字符边界。
2. 有效则返回 `Some((&s[..mid], &s[mid..]))` —— 两个切片都引用原始数据。
3. 无效返回 `None`。
4. 不分配新 String —— 整个操作是零拷贝的。

### 参考实现

```rust
fn split_at_utf8(s: &str, mid: usize) -> Option<(&str, &str)> {
    if s.is_char_boundary(mid) {
        Some((&s[..mid], &s[mid..]))
    } else {
        None
    }
}

fn main() {
    // 英文：字符边界 = 字节边界
    assert_eq!(split_at_utf8("hello", 2), Some(("he", "llo")));

    // 中文：每个字 3 字节
    let chinese = "你好世界";
    assert_eq!(split_at_utf8(chinese, 6), Some(("你好", "世界")));
    // 在"好"字的第二个字节处切割——不合法！
    assert_eq!(split_at_utf8(chinese, 4), None);

    // 边界情况
    assert_eq!(split_at_utf8("hello", 0), Some(("", "hello")));
    assert_eq!(split_at_utf8("hello", 5), Some(("hello", "")));
    assert_eq!(split_at_utf8("hello", 6), None); // 超出长度

    println!("所有测试通过！");
}
```

### 常见错误

- 直接用 `&s[..mid]` 而不先检查 `is_char_boundary` —— 在错误的字节位置切片会 panic。
- 混淆字节索引和字符索引：中文一个字符占 3 个 UTF-8 字节。
- 忘记处理 `mid > s.len()` 的情况（索引超出范围也会 panic）。

### 验证方式

```bash
cargo run  # 所有 assert_eq! 通过
```

---

## 练习 2.2: 借阅追踪器 (BookShelf)

### 结论

`Cell<T>` 提供"内部可变性"（Interior Mutability）—— 允许在 `&self` 方法中修改内部状态（借阅计数），而借出的 `&str` 引用仍然遵守借用规则。

### 思路

1. `BookShelf` 包含 `content: String` 和 `borrow_count: Cell<usize>`。
2. `try_borrow(&self)` 通过 `Cell::get()` 读取计数，若小于 MAX_READERS 则 `Cell::set()` 加一并返回引用。
3. `Cell<usize>` 是 Copy 类型，`get()`/`set()` 不涉及借用冲突。
4. 因为返回的 `&str` 生命周期与 `&self` 相关，当 `book_shelf` 在 `main` 中保持 alive 时，所有借用的引用都有效。

### 参考实现

```rust
use std::cell::Cell;

struct BookShelf {
    content: String,
    borrow_count: Cell<usize>,
}

impl BookShelf {
    fn new(content: &str) -> Self {
        BookShelf {
            content: content.to_string(),
            borrow_count: Cell::new(0),
        }
    }

    fn try_borrow(&self) -> Option<&str> {
        let count = self.borrow_count.get();
        if count >= MAX_READERS {
            None
        } else {
            self.borrow_count.set(count + 1);
            Some(&self.content)
        }
    }
}

const MAX_READERS: usize = 3;

fn main() {
    let shelf = BookShelf::new("The Rust Book");

    let r1 = shelf.try_borrow();
    let r2 = shelf.try_borrow();
    let r3 = shelf.try_borrow();
    let r4 = shelf.try_borrow(); // 应该返回 None！

    println!("r1: {:?}", r1);   // Some("The Rust Book")
    println!("r2: {:?}", r2);   // Some("The Rust Book")
    println!("r3: {:?}", r3);   // Some("The Rust Book")
    println!("r4: {:?}", r4);   // None
}
```

### 常见错误

- 试图在 `&self` 方法中使用 `&mut self.borrow_count` —— 违反借用规则。
- 忘记 Cell 需要 `use std::cell::Cell`。
- 以为返回的引用有独立生命周期 —— 返回的 `&str` 与 `&self` 生命周期绑定。

### 验证方式

```bash
cargo run  # r1..r3 输出 Some, r4 输出 None
```

---

## 练习 3.1: 文本行迭代器 (LineIter), 零拷贝

### 结论

零拷贝迭代器的核心：持有数据引用，通过索引/切片操作返回子引用，不分配新内存。返回的引用生命周期由 `'a` 保证与原始数据一致。

### 思路

1. `LineIter<'a>` 持有 `&'a str` 引用，用 `position: usize` 追踪当前位置。
2. `next()` 方法：找到下一个 `\n` 的位置，切割当前行，更新 position。
3. 返回的 `&'a str` 是原始字符串的子切片，不复制数据。

关于生命周期：`next(&mut self) -> Option<&'a str>` 中 `'a` 是结构体的生命周期参数，表示返回的引用与输入的文本同寿命。如果写成 `'_`（匿名生命周期），编译器会尝试关联到 `&mut self`，这在很多场景下不够长。

### 参考实现

```rust
struct LineIter<'a> {
    text: &'a str,
}

impl<'a> LineIter<'a> {
    fn new(text: &'a str) -> Self {
        LineIter { text }
    }

    fn next(&mut self) -> Option<&'a str> {
        if self.text.is_empty() {
            return None;
        }

        // 找下一个换行符
        match self.text.find('\n') {
            Some(pos) => {
                let line = &self.text[..pos];
                // 跳过换行符，前进到下一行
                self.text = &self.text[pos + 1..];
                Some(line)
            }
            None => {
                // 最后一行，没有换行符
                let line = self.text;
                self.text = &self.text[self.text.len()..]; // 置空
                Some(line)
            }
        }
    }
}

fn main() {
    let text = "第一行\n第二行\n第三行";

    let mut iter = LineIter::new(text);

    assert_eq!(iter.next(), Some("第一行"));
    assert_eq!(iter.next(), Some("第二行"));
    assert_eq!(iter.next(), Some("第三行"));
    assert_eq!(iter.next(), None);

    // 原始数据仍然可用！
    println!("原始文本: {text}");

    // 测试空行和尾随换行
    let text2 = "A\n\nB\nC\n";
    let mut iter2 = LineIter::new(text2);
    assert_eq!(iter2.next(), Some("A"));
    assert_eq!(iter2.next(), Some(""));
    assert_eq!(iter2.next(), Some("B"));
    assert_eq!(iter2.next(), Some("C"));
    assert_eq!(iter2.next(), Some(""));   // 尾随换行产生空行
    assert_eq!(iter2.next(), None);

    println!("所有测试通过！");
}
```

### 为什么 `next(&mut self) -> Option<&'a str>` 用 `'a` 而非 `'_`

- `'a` 是结构体的生命周期参数，表示"返回的引用和 LineIter 持有的原始 text 数据同寿命"。
- 如果用 `'_` → `next(&mut self) -> Option<&'_ str>`，编译器会将其关联到 `&mut self` 的生命周期。这意味着返回的引用和 `&mut self` 借用绑定在一起——归还 `&mut self` 后引用就不能用了。在很多迭代场景中这不够灵活。
- `'a` 表达能力更强：即使 `LineIter` 被多次 `&mut` 借用，返回的引用仍然只要不超过原始 `text` 即可。

### 常见错误

- 试图用 `self.text.lines()` —— 练习要求自己实现查找逻辑。
- 忘记处理尾随换行符（产生额外空行）。
- 将 `position` 用作字段但忘记更新 —— 更简单的方法是用切片"消费"文本。
- 生命周期标注错误 —— `next()` 返回 `Option<&'a str>` 需要显式标注。

### 验证方式

```bash
cargo test  # 或用 cargo run 观察 assert_eq! 是否 panic
```

---

## 思考题: Python 与 Rust 的切片哲学

### 问题 1: 性能场景

**Python**: 切片 `s[0:100]` 创建一个新字符串对象，分配 100 字节的新内存，复制数据。10MB 原始字符串仍占用内存，直到 GC 回收。**时间和空间开销**: 分配 + 复制 O(n)。

**Rust**: 切片 `&s[0..100]` 创建一个胖指针（16 字节：指针 + 长度），不复制数据，不分配内存。原始 10MB 不受影响。**时间和空间开销**: 胖指针创建 O(1)，额外内存 16 字节。

**结论**: Rust 零拷贝切片在大数据场景下快数量级倍数。

### 问题 2: 安全场景

**Python**: `bytearray` 可修改，但字符串切片 `s[0:5]` 是独立副本，修改原始 `bytearray` 不影响切片。`str` 本身是不可变的（Python 中字符串不可变）。

**Rust**: 持有 `&str` 切片引用时，不能同时有 `&mut String` —— 借用检查器在编译时阻止对原始 String 的任何可变操作。这是借用规则的直接体现："可以有多个共享引用，或一个可变引用，但不能同时。"

### 问题 3: 设计哲学

| 维度 | Python（复制） | Rust（借用） |
|------|---------------|-------------|
| 简单性 | 高：切片后互不影响 | 低：需要理解借用规则 |
| 内存效率 | 差：每次切片复制 | 优：零拷贝 |
| 并发安全 | 需 GIL 或锁 | 编译期保证 |
| 适用场景 | 字符串短、切片少 | 大数据、高性能、系统编程 |

**Python 方式的优势**: 不需要理解借用规则，切片后可以任意修改原始数据而不影响切片 —— 心智负担低。

**Rust 方式的优势**: 处理大数据时零开销，适合系统编程和性能敏感场景。借用检查器保证了引用安全性。

### 问题 4: GC 的角色

**Python 的 GC** 负责追踪所有对象的生命周期。即使有多个切片（每个都是独立副本），GC 自动管理每个副本的释放。程序员不需要考虑"切片会不会比原始数据活得更长"——GC 保证每个对象只要还有引用就不会被释放。这使得"切片即复制"变得安全又简单。

**Rust 的借用检查器** 在编译期验证：任何切片引用（`&str`）的生命周期不能超过原始数据（`String`）的生命周期。这保证了运行时不需要 GC 来追踪引用关系，实现了零开销的安全抽象。代价是编译时需要提供生命周期信息，且受借用规则约束。

---

## 迁移思维练习答案

### 1. C 中通过指针传递数组的方式，在 Rust 中应该怎么改为切片？

C 的数组传递通常需要指针 + 长度两个参数（如 `void process(int* arr, size_t len)`），两者之间没有编译期绑定——调用者可能传错长度。Rust 的切片 &[T] 将指针和长度打包为一个类型安全的单元，编译器保证切片始终有效（指向有效数据）。函数签名 `fn process(data: &[i32])` 同时表达了"借用数组而非获取所有权"和"不会越界访问"（通过运行时的边界检查，在优化后常被消除）。迁移时把 `pointer + length` 的二元组统一替换为 `&[T]`。

### 2. 哪些指针关系应该改为借用（&T）？

只要不涉及所有权转移的临时访问，都应该优先使用借用。包括：函数参数只读取数据、遍历集合元素、获取结构体字段的只读访问、在多步操作之间传递同一个对象的引用。借用是零成本的（底层就是传递指针），且编译器保证借用期间数据不会被释放或同时进行可变修改——这是 C 指针完全无法提供的安全保障。

### 3. 如何用切片避免 C 中常见的数组越界和安全问题？

C 中的数组越界是未定义行为（UB），编译器不会检查，可能悄无声息地读写非法内存。Rust 中通过 &[T] 访问元素时，编译器会插入边界检查（在 debug 模式下），访问越界会触发 panic 而非 UB。更重要的是，切片的 len() 方法让长度信息永远随数据传递，不存在 C 中"谁记得数组长度"的信息丢失问题。在性能敏感场景中，可以通过迭代器完全避免索引访问，从而消除边界检查。

---

## 自检清单

- [x] 什么是引用（Reference）？`&` 运算符做什么？
- [x] 什么是解引用（Dereference）？`*` 运算符做什么？
- [x] 不可变引用 `&T` 和可变引用 `&mut T` 的区别是什么？
- [x] 借用规则的两条核心规则分别是什么？
- [x] 为什么 Rust 在编译时禁止数据竞争？
- [x] 什么是非词法生命周期（NLL）？它解决了什么问题？
- [x] 什么是悬垂引用（Dangling Reference）？Rust 如何防止它？
- [x] **`String` 和 `&str` 的区别是什么？什么时候用哪个？** `String` 拥有堆数据，`&str` 是借用视图。函数参数优先用 `&str`（更通用，支持 Deref coercion），需要所有权时用 `String`。
- [x] 切片 `&[T]` 和数组 `[T; N]` 的关系是什么？
- [x] 如何在不使用 `clone()` 的情况下修复借用冲突？

---

> "The borrow checker is not your enemy — it's your most diligent code reviewer, working 24/7 to prevent memory bugs before they reach production."
> — Rust 社区
