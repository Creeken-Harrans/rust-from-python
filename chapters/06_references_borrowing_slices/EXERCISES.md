# 练习：引用、借用与切片

## Exercises: References, Borrowing & Slices

---

## 使用说明

所有练习在 `chapters/06_references_borrowing_slices/` 目录下完成。

### 推荐命令

```bash
# 运行主程序，查看所有演示
cargo run

# 编译检查（不运行）
cargo check

# 格式化代码
cargo fmt

# 运行测试（如果你创建了测试）
cargo test

# 查看详细编译输出，理解借用检查器
cargo build 2>&1 | less

# 使用 Clippy 检查代码质量
cargo clippy
```

### 学习建议

1. **先运行，再理解**：运行 `cargo run` 查看所有演示的输出，对每个概念有直观印象
2. **故意犯错**：在代码中有意违反借用规则，学习阅读编译器错误信息
3. **画内存图**：对于每个练习，画一下内存布局——哪些变量拥有数据，哪些是指向数据的引用
4. **对比 Python**：如果你熟悉 Python，想想同样的操作在 Python 中会怎么做，内存会怎样分配

---

## Level 1：基础练习（3 题）

### 练习 1.1：识别引用类型

阅读以下代码，回答每个变量的类型是什么（`String`、`&String`、`&mut String`），以及它是否可以修改数据。

```rust
let mut original = String::from("Rust");

let a = &original;          // 类型？可以修改吗？
let b = &mut original;      // 类型？可以修改吗？
let c = original;           // 类型？可以修改吗？
let mut d = String::from("Python");
let e = &d;                 // 类型？可以修改吗？
```

**要求**：
1. 用注释标注每个变量的类型
2. 指出哪些行会编译失败
3. 解释为什么

**期望输出**：在答案中写清楚每个变量的类型和能否修改，以及失败的原因。

---

### 练习 1.2：修复借用错误

下面的代码无法编译。找出问题并修复它。**要求**：修复后的代码必须仍然使用引用，不能通过 clone() 或移除引用来回避问题。

```rust
// 修复此函数
fn append_world() -> String {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;       // ← 问题在这里
    r2.push_str(" world");
    let result = s;
    println!("{r1}");      // ← r1 在这里使用
    result
}

fn main() {
    let s = append_world();
    println!("结果: {s}");
}
```

**提示**：思考 `r1` 的使用是否可以移到 `r2` 创建之前？或者思考 NLL 的规则——`r1` 在哪个点之后不再需要？

**要求**：
1. 修改代码使其能编译
2. 解释你做的修改以及为什么
3. 保持程序的功能不变（输出相同）

---

### 练习 1.3：创建你的第一个切片函数

编写一个函数 `count_vowels(s: &str) -> usize`，接受一个字符串切片，统计其中元音字母（a, e, i, o, u，不区分大小写）的数量。

```rust
// 补全此函数
fn count_vowels(s: &str) -> usize {
    // 你的代码
}

fn main() {
    assert_eq!(count_vowels("Hello World!"), 3);   // e, o, o
    assert_eq!(count_vowels("Rust"), 1);            // u
    assert_eq!(count_vowels("AEIOU"), 5);           // 全部
    assert_eq!(count_vowels("xyz"), 0);             // 没有
    println!("所有测试通过！");
}
```

**要求**：
1. 参数使用 `&str`（不是 `&String`）
2. 函数内部不创建新的 `String`（不分配堆内存）
3. 使用 `for` 循环遍历 `s.chars()`
4. 正确处理大小写

**扩展思考**：为什么使用 `&str` 比 `&String` 更好？写一个 `count_vowels_string(s: &String) -> usize` 的版本，看看调用方式有什么不同。

---

## Level 2：中级练习（2 题）

### 练习 2.1：实现 split_at 的切片版本

标准库的 `split_at` 方法可以将切片在给定索引处分成两个切片。现在你需要自己实现一个函数：

```rust
/// 将字符串切片在给定的 字节索引 处分割成两个切片。
/// 如果索引不在有效的 UTF-8 字符边界上，返回 None。
fn split_at_utf8(s: &str, mid: usize) -> Option<(&str, &str)> {
    // 你的代码：
    // 1. 检查 mid 是否在有效的 UTF-8 字符边界上
    //    使用 s.is_char_boundary(mid)
    // 2. 如果有效，返回 Some((&s[..mid], &s[mid..]))
    // 3. 否则返回 None
}
```

**在 `main()` 中测试**：

```rust
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

    println!("所有测试通过！");
}
```

**要求**：
1. 函数签名正确，返回 `Option<(&str, &str)>`
2. 使用 `s.is_char_boundary(mid)` 检查 UTF-8 边界
3. 不能分配新的 `String`
4. 处理边界情况（mid == 0, mid == s.len()）

---

### 练习 2.2：实现引用计数的"最大借阅数"追踪

这个练习模拟一个简易的借阅追踪器。实现一个结构体，它可以使用不可变引用来"借阅"数据，但限制同一时间最多 3 个不可变借阅者。

```rust
/// 一个简单的借阅追踪器——模拟 Rust 借用检查器的规则
struct BookShelf {
    content: String,
    // 提示：你可能需要额外的字段来追踪借阅状态
}

impl BookShelf {
    /// 创建一个新的书架，包含给定内容
    fn new(content: &str) -> Self {
        todo!()
    }

    /// 尝试获取一个不可变引用。
    /// 如果当前借阅数 >= MAX_READERS (3)，返回 None
    /// 否则返回 Some(&str) 并增加借阅计数
    fn try_borrow(&self) -> Option<&str> {
        // 注意：这个方法需要改变 self 的内部状态（借阅计数）
        // 但签名是 &self —— 思考如何实现
        todo!()
    }

    // 提示：考虑哪些类型可以帮助你在 &self 下修改内部状态
    // std::cell 中的类型可能有用...
}

const MAX_READERS: usize = 3;

fn main() {
    let shelf = BookShelf::new("The Rust Book");

    let r1 = shelf.try_borrow();
    let r2 = shelf.try_borrow();
    let r3 = shelf.try_borrow();
    let r4 = shelf.try_borrow(); // 应该返回 None！

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    println!("r3: {:?}", r3);
    println!("r4: {:?}", r4); // None
}
```

**提示**：你需要用到 `std::cell::Cell` 或 `std::cell::RefCell` 来实现"内部可变性"（Interior Mutability），这些会在后面的章节中详细介绍。本题是预览。

**要求**：
1. 实现 `BookShelf` 结构体和两个方法
2. 使用 `Cell<usize>` 来追踪借阅计数
3. 前 3 次借用返回 `Some(&str)`，第 4 次返回 `None`

---

## Level 3：高级练习（1 题）

### 练习 3.1：实现一个简单的文本行迭代器（零拷贝）

实现一个结构体 `LineIter`，它持有一个字符串切片的引用，并可以逐行迭代，**不分配任何新内存**。

```rust
/// 行迭代器：对 &str 的逐行借用视图
/// 每次调用 next() 返回下一行的切片引用
struct LineIter<'a> {
    text: &'a str,
    // 你需要字段来追踪当前迭代位置
}

impl<'a> LineIter<'a> {
    /// 从给定的文本创建一个新的 LineIter
    fn new(text: &'a str) -> Self {
        todo!()
    }

    /// 返回下一行（不包括换行符），如果没有更多行则返回 None
    fn next(&mut self) -> Option<&'a str> {
        todo!()
    }
}

fn main() {
    let text = "第一行\n第二行\n第三行";

    let mut iter = LineIter::new(text);

    assert_eq!(iter.next(), Some("第一行"));
    assert_eq!(iter.next(), Some("第二行"));
    assert_eq!(iter.next(), Some("第三行"));
    assert_eq!(iter.next(), None);

    // 验证：原始数据仍然可以被访问（借用未影响所有权）
    println!("原始文本: {text}");  // text 仍然可用！

    // 测试空行和尾随换行
    let text2 = "A\n\nB\nC\n";
    let mut iter2 = LineIter::new(text2);
    assert_eq!(iter2.next(), Some("A"));
    assert_eq!(iter2.next(), Some(""));
    assert_eq!(iter2.next(), Some("B"));
    assert_eq!(iter2.next(), Some("C"));
    assert_eq!(iter2.next(), Some(""));
    assert_eq!(iter2.next(), None);

    println!("所有测试通过！");
}
```

**要求**：
1. `LineIter` 持有 `&'a str` 引用，不复制数据
2. 正确处理 `\n` 换行符（仅考虑 `\n`，不需要处理 `\r\n`）
3. 正确处理空行（连续两个 `\n`）
4. 正确处理尾随换行符（文本以 `\n` 结尾时有额外的空行）
5. 原始 `text` 变量在迭代后仍然可用
6. 不能使用标准库的 `.lines()` 方法——自己实现查找 `\n` 的逻辑（可以参考 `first_word` 的思路）

**提示**：
- `text.find('\n')` 可以找到下一个换行符的位置
- 使用 `&self.text[..pos]` 来创建行切片
- 使用 `self.text = &self.text[pos+1..]` 来前进到下一行

**思考**：
- `next(&mut self) -> Option<&'a str>` 的 `&mut self` 和返回的 `&'a str` 有什么关系？
- 为什么返回的引用有生命周期 `'a` 而不是 `'_`？
- 这个迭代器的设计如何体现"零拷贝"？

---

## 思考题：Python 与 Rust 的切片哲学

### 思考题：切片——复制还是借用？

在 Python 中，字符串切片 `s[0:5]` 会创建一个新的字符串对象（复制数据）。在 Rust 中，`&s[0..5]` 只是创建一个不可变的借用视图（不复制数据）。

**问题**：

1. **性能场景**：假设你有一个 10MB 的字符串。在 Python 和 Rust 中分别取前 100 个字符的切片。内存使用有什么不同？哪个更快？为什么？

2. **安全场景**：在 Python 中，如果你持有一个字符串的切片，原始字符串可以被修改吗（如果是可变的如 bytearray）？它的切片会受影响吗？在 Rust 中，如果你持有一个 `&str` 切片，原始的 `String` 可以被修改吗？为什么？

3. **设计哲学**：Python 选择"切片即复制"，Rust 选择"切片即借用"。这两种设计分别适合什么样的使用场景？各有什么优缺点？

4. **GC 的角色**：Python 的垃圾回收器如何使得"切片即借用"变得不必要？Rust 的借用检查器如何使得"切片即借用"变得安全？

**参考答案要点**（尝试自己回答后再看）：

<details>
<summary>点击展开参考要点</summary>

1. **性能**：Python 会分配 100 字节的新字符串，原始 10MB 仍然存在但可能被 GC。Rust 的 `&str` 只是一个胖指针（16 字节：指针 + 长度），根本不复制数据。Rust 版本快得多，内存占用极小。

2. **安全**：Python 的 `bytearray` 是可变的，但字符串切片是独立的副本，修改原始 `bytearray` 不影响切片。Rust 中，如果你持有 `&str` 切片引用，就不能同时有 `&mut String`——编译器会阻止修改原始 String。这就是借用规则保护了切片。

3. **设计哲学**：Python 方式更简单、更灵活（没有借用检查器的约束），但有内存和时间开销。Rust 方式追求零成本抽象，用编译时的复杂性换取运行时的极致性能。

4. **GC 的角色**：Python 的 GC 保证了即使有多个切片（副本），每个对象都有独立的生命周期管理，不需要使用者关心。Rust 没有 GC，所以必须通过借用检查器在编译时确保切片引用不会比原始数据活得更长。

</details>

---

## 参考答案与提示

### 练习 1.1 答案

```
a: &String 类型，不可变借用，不能修改
b: &mut String 类型，可变借用，可以修改（但会与 a 冲突导致编译失败）
    因为 a 和 b 在同作用域同时存在，违反了"不能同时有 & 和 &mut"
c: String 类型，拥有所有权，声明时没有 mut 所以不能修改
d: String 类型，拥有所有权，有 mut 因此可以修改
e: &String 类型，不可变借用，不能修改
```

### 练习 1.2 提示

将 `r1` 的使用移到 `r2` 创建之前，或者将 `r2` 的作用域用 `{}` 限制：

```rust
fn append_world() -> String {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{r1}");          // r1 在这里最后使用
    // r1 不再使用，NLL 认为它"结束"了
    let r2 = &mut s;           // ✅ 现在可以！
    r2.push_str(" world");
    let result = s;
    result
}
```

### 练习 1.3 答案

```rust
fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => {}
        }
    }
    count
}
```

---

## 自检清单

在进入下一章之前，确认你能回答以下问题：

- [ ] 什么是引用（Reference）？`&` 运算符做什么？
- [ ] 什么是解引用（Dereference）？`*` 运算符做什么？
- [ ] 不可变引用 `&T` 和可变引用 `&mut T` 的区别是什么？
- [ ] 借用规则的两条核心规则分别是什么？
- [ ] 为什么 Rust 在编译时禁止数据竞争？
- [ ] 什么是非词法生命周期（NLL）？它解决了什么问题？
- [ ] 什么是悬垂引用（Dangling Reference）？Rust 如何防止它？
- [ ] `String` 和 `&str` 的区别是什么？什么时候用哪个？
- [ ] 切片 `&[T]` 和数组 `[T; N]` 的关系是什么？
- [ ] 如何在不使用 `clone()` 的情况下修复借用冲突？

---

> "The borrow checker is not your enemy — it's your most diligent code reviewer, working 24/7 to prevent memory bugs before they reach production."
> — Rust 社区

---

## 迁移思维练习

> 以下问题帮助你思考 C 中的指针传递模式如何重新建模为 Rust 的引用与切片。

### 问题 1：C 中通过指针传递数组的方式，在 Rust 中应该怎么改为切片？

在 C 中，你习惯用 `void process(int *data, size_t len)` 来传递数组——指针和长度是两个独立参数，编译器不会检查它们的对应关系。Rust 的切片 `&[i32]` 把指针和长度打包在一起。这种打包带来了什么安全保障？如果一段 C 代码中指针和长度来自不同变量、不同来源，翻译到 Rust 时你会发现什么设计问题？

**提示**：切片是一个"胖指针"（pointer + length），它保证了指针和长度的不可分割性，消除了传递错误的长度值的可能性。

### 问题 2：哪些指针关系应该改为借用（&T）？

C/C++ 中充斥着各种指针：函数参数用 `const int*` 表示"我只读不改"、用 `int*` 表示"我可能改"、用 `void*` 表示"任意数据"、存储指向其他对象内部数据的指针作为成员变量……请分类思考：哪些指针关系对应 Rust 的 `&T`（共享不可变借用），哪些对应 `&mut T`（独占可变借用），哪些对应 `Box<T>`（拥有所有权的堆分配）？C 的 `const` 指针和 Rust 的 `&T` 在"不可变"这个概念上有什么本质区别？

**提示**：C 的 `const` 可以被 cast 掉，Rust 的 `&T` 由编译器在类型层面强制不可变——任何绕过 `&T` 修改数据的 unsafe 代码都需要开发者显式标记。
