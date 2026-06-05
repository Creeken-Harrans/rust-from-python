# 第六章：引用、借用与切片

## References, Borrowing & Slices

---

## 目录

1. [问题引入：为什么需要引用和借用？](#1-问题引入为什么需要引用和借用)
2. [Python 视角：我们从 Python 怎么做的](#2-python-视角我们从-python-怎么做的)
3. [Rust 的设计：引用与借用的核心理念](#3-rust-的设计引用与借用的核心理念)
4. [借用规则详解](#4-借用规则详解)
5. [代码示例与实际运行](#5-代码示例与实际运行)
6. [常见错误与编译器报错](#6-常见错误与编译器报错)
7. [编译器如何检查借用规则](#7-编译器如何检查借用规则)
8. [如何修复借用相关错误](#8-如何修复借用相关错误)
9. [边界与注意事项](#9-边界与注意事项)
10. [本章总结](#10-本章总结)

---

## 1. 问题引入：为什么需要引用和借用？

在上一章中，我们学习了 Rust 的所有权（Ownership）系统。所有权规则非常强大，但也带来了一个实际问题：

**如果每次将数据传给函数都要转移所有权，那么：**

- 我们如何在不失去所有权的情况下，多次使用同一个数据？
- 如何让函数"查看"数据而不"夺走"它？
- 如何在修改数据的同时，保持代码的安全性和可预测性？

考虑这个场景：你有一本珍贵的书。你想让朋友们也看看这本书，但你不想把书送给他们——你只想**借**给他们。等他们看完后，书还是你的。

这就是 Rust 中**借用（Borrowing）**的概念。

```rust
let s = String::from("珍贵的书");

// 把书"借"给函数看——不转移所有权
print_content(&s);    // &s 创建了一个引用

// 书还是我的，我可以继续看
println!("{s}");      // 仍然有效！
```

如果没有借用机制，上面的代码就不得不写成：

```rust
let s = String::from("珍贵的书");
print_content(s);     // 所有权被转移！
// println!("{s}");   // 编译错误！s 已经无效
```

**核心问题**：如何在不需要所有权的情况下访问数据？

---

## 2. Python 视角：我们从 Python 怎么做的

### Python 中的"引用"

在 Python 中，一切皆是对象，所有变量本质上都是对对象的引用：

```python
s = "hello"          # s 指向字符串对象 "hello"
t = s                 # t 也指向同一个对象（没有拷贝）
print(s)              # "hello"
print(t)              # "hello"
# Python 使用引用计数——当没有任何变量指向对象时，对象被回收
```

Python 的做法：
- 所有变量本质上都是"引用"
- 不需要考虑所有权——垃圾回收器（GC）会自动处理
- 可以随意地让多个变量指向同一个对象
- 修改对象时，所有指向该对象的变量都会看到变化（对于可变对象）

### Python 切片与 Rust 切片的本质区别

这是一个非常重要的对比：

```python
# Python 切片：创建新对象（分配新内存）
s = "Hello World"
slice = s[0:5]       # 创建一个全新的字符串 "Hello"
# slice 是独立的对象，有自己的内存空间
# 修改 s 不会影响 slice（字符串不可变）
```

```rust
// Rust 切片：借用视图（不分配新内存）
let s = String::from("Hello World");
let slice: &str = &s[0..5];  // 不创建新字符串，只是"视图"
// slice 指向 s 内部数据的第 0 到第 5 个字节
// 零成本抽象！没有内存分配，没有数据拷贝
```

**关键区别**：
| 特性 | Python | Rust |
|------|--------|------|
| 切片机制 | 创建新对象（分配内存） | 借用视图（零分配） |
| 所有权 | 不适用（GC 管理） | 编译时强制检查 |
| 并发安全 | 运行时（GIL） | 编译时（借用检查器） |
| 悬垂引用 | 不可能（GC 保证） | 编译时禁止 |

---

## 3. Rust 的设计：引用与借用的核心理念

### 3.1 引用（Reference, `&`）

**引用**是 Rust 中一种不获取所有权的访问数据的方式。它像一个**安全的指针**：

```rust
let s = String::from("hello");
let r = &s;  // r 是 s 的引用，类型是 &String
```

引用有两个关键特性：
1. **不拥有数据**：创建引用不转移所有权
2. **可以解引用**：使用 `*` 运算符获取引用指向的值

```
内存布局：
  s (String)                r (&String)
  ┌──────────┐              ┌──────────┐
  │ ptr     ─┼──→ "hello"  ←┼── ptr    │
  │ len: 5  │              │          │
  │ cap: 5  │              └──────────┘
  └──────────┘
  r 只是存储了 s 的地址，不拥有数据
```

### 3.2 解引用（Dereference, `*`）

`*` 运算符从引用中获取所指向的实际值：

```rust
let x = 42;
let rx = &x;     // rx 是 &i32 类型
assert_eq!(*rx, 42);  // *rx 获取引用指向的值

// 对于 Copy 类型（如 i32），解引用会复制值
let y = *rx;     // y 是 i32，值是 42
// x、rx、y 都可以独立使用
```

Rust 在很多地方会自动解引用（deref coercion），例如 `println!("{}", rx)` 会自动调用 `*rx`。

### 3.3 借用（Borrowing）的概念

**借用**就是创建引用的行为。像借书：

- 你（所有者）仍然拥有书的所有权
- 别人（借阅者）可以阅读（不可变引用）
- 别人（借阅者）也可以做笔记（可变引用）——但你只能借给一个人做笔记
- 书最终还是要还给你（引用有生命周期，不能比所有者活得长）

```
借书类比：
┌────────────────────────────────────────────────┐
│  你拥有一本书（所有权）                         │
│  ├─ 借给 A 阅读  (& 不可变借用)                 │
│  ├─ 借给 B 阅读  (& 不可变借用)  ← 多人同时读 OK │
│  └─ 借给 C 修改  (&mut 可变借用) ← 修改期间不能借 │
│                   给别人读，也不能再借给别人改     │
│  → 书始终是你的，只是暂时借出                    │
└────────────────────────────────────────────────┘
```

---

## 4. 借用规则详解

Rust 的借用检查器（Borrow Checker）强制执行两条核心规则：

### 规则 1：在任意给定时间，你可以拥有 **其中之一** 而非两者

- **任意数量的不可变引用** `&T`
- **恰好一个可变引用** `&mut T`

```rust
// ✅ 合法：多个不可变引用
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &s;
println!("{r1} {r2} {r3} {s}"); // 全部可用

// ❌ 非法：在已有不可变引用时创建可变引用
let mut s = String::from("hello");
let r1 = &s;         // 不可变引用
let rm = &mut s;     // 编译错误！不能同时有 & 和 &mut
println!("{r1}");

// ✅ 合法：只有一个可变引用
let mut s = String::from("hello");
let rm = &mut s;     // 唯一可变引用
rm.push_str(" world");
println!("{rm}");

// ❌ 非法：两个可变引用
let mut s = String::from("hello");
let rm1 = &mut s;
let rm2 = &mut s;    // 编译错误！同作用域内不能有两个 &mut
```

### 规则 2：引用必须始终有效

Rust 编译器保证**引用永远不会指向已释放的内存**（即没有悬垂引用）：

```rust
// ❌ 悬垂引用——编译器拒绝编译
fn dangling() -> &String {
    let s = String::from("hello");
    &s  // 编译错误：s 在函数结束时会被释放
}       // 返回的引用将指向已释放的内存！

// ✅ 正确：返回 String 本身（转移所有权）
fn not_dangling() -> String {
    let s = String::from("hello");
    s   // 返回所有权，调用者获得数据
}
```

### 4.1 为什么需要这些规则？——数据竞争（Data Race）

**数据竞争**是指两个或多个操作同时访问同一数据，并且至少有一个是写操作，同时没有任何同步机制。数据竞争会导致：
- 读到不完整或损坏的数据
- 程序行为不可预测（未定义行为，UB）
- 极难调试（可能偶尔出现，难以复现）

用借书的类比来理解：

```
场景 1：读-写冲突
  A 正在读书      (不可变引用，读)
  C 同时在涂改书   (可变引用，写)
  → A 可能读到被涂改一半的内容 ← 数据竞争！

场景 2：写-写冲突
  C 在书的第5页写笔记   (可变引用)
  D 也在书的第5页写笔记  (另一个可变引用)
  → 第5页的内容变成谁写的？ ← 数据竞争！

场景 3：多读（安全！）
  A 在读书
  B 也在读书
  → 互不干扰 ← 完全安全！
```

Rust 在**编译时**就通过借用规则消除了所有数据竞争的可能——不需要运行时开销，不需要垃圾回收器。

### 4.2 非词法生命周期（Non-Lexical Lifetimes, NLL）

在 Rust 2018 之前，引用的生命周期由词法作用域（大括号 `{}`）决定。如果一个可变引用在一个代码块中定义，即使你不再使用它，它也会在整个代码块中保持"活跃"。

NLL 改变了这一点：编译器现在追踪引用的**实际使用点**。引用在最后一次使用之后就"失效"了：

```rust
// NLL 之前（Rust 2015）——编译错误
// NLL 之后（Rust 2018+）——编译成功！
let mut data = String::from("hello");

let rm = &mut data;
rm.push_str(" world");
// rm 在这里最后一次使用——NLL 认为它"结束"了

let r1 = &data;  // ✅ 现在可以创建不可变引用！
let r2 = &data;  // ✅ 多个不可变引用！
println!("{r1} {r2}");
```

没有 NLL 时，你需要用额外的大括号来限制引用的作用域；有了 NLL，代码更自然、更简洁。

---

## 5. 代码示例与实际运行

本章配套的 `src/main.rs` 包含完整的可运行示例。运行方式：

```bash
cd chapters/06_references_borrowing_slices
cargo run
```

主要演示函数：

| 函数名 | 演示内容 |
|--------|----------|
| `demonstrate_references()` | `&` 创建引用，`*` 解引用，地址对比 |
| `demonstrate_immutable_borrows()` | 多个 `&T` 共存，所有权不丢失 |
| `demonstrate_mutable_borrow()` | `&mut T` 独占修改权限 |
| `demonstrate_nll()` | NLL：可变引用结束后立即创建不可变引用 |
| `demonstrate_slices()` | 字符串切片 `&str` 的各种创建方式 |
| `demonstrate_array_slices()` | 数组切片 `&[T]`，可变切片修改原数组 |
| `demonstrate_lifetime_preview()` | 生命周期标注 `'a` 预览 |
| `first_word()` | 经典：找到第一个单词并返回切片 |
| `longest_word()` | 生命周期标注：返回最长单词的引用 |
| `sum_slice()` | 接受 `&[i32]` 切片，返回元素和 |

### 5.1 引用的内存模型

```rust
let s = String::from("hello");
let r = &s;

// s 的地址 和 r 指向的地址 是同一个
// 因为 r 存储了 s 所在栈位置的地址
```

引用的本质是一个指针，但它带有编译时的所有权和生命周期信息。在运行时，引用就是一个普通的指针——零成本抽象。

### 5.2 String 与 &str 的关系

这是理解 Rust 字符串的关键：

```
String:                      &str:
┌──────────────┐             ┌──────────────┐
│ ptr   ───────┼──→ "hello"  ←── ptr        │
│ len: 5       │   (堆上)    │ len: 5       │
│ cap: 5       │             └──────────────┘
└──────────────┘              "胖指针"：地址 + 长度
 拥有数据，在堆上分配            不拥有数据，只是"视图"
```

- `String`：拥有所有权的、可变长的、堆分配的字符串
- `&str`：借用的、不可变的、固定长度的字符串"视图"
- `&String`：对 `String` 的引用（通常用 `&str` 代替，因为可以自动转换）

```rust
// 函数参数：优先使用 &str 而非 &String
fn greet(name: &str) {           // ✅ 更好：可以接受 String 和 &str
    println!("Hello, {name}!");
}
fn greet_less_good(name: &String) { // ⚠️ 只能用 &String 调用
    println!("Hello, {name}!");
}
```

### 5.3 Array 与 &[T] 的关系

数组和数组切片的关系，完全对应 String 和 &str 的关系：

```rust
let arr = [1, 2, 3, 4, 5];  // [i32; 5] —— 拥有所有权，固定长度
let slice: &[i32] = &arr[1..4]; // &[i32] —— 借用视图，指向 arr 的一部分
// slice 是"胖指针"：{ 指针: arr[1]的地址, 长度: 3 }
```

---

## 6. 常见错误与编译器报错

### 错误 1：同时有可变和不可变引用

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // 错误！
println!("{r1}");
```

编译器报错：

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let r1 = &s;
  |              -- immutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
5 |     println!("{r1}");
  |               ---- immutable borrow later used here
```

### 错误 2：两个可变引用

```rust
let mut s = String::from("hello");
let rm1 = &mut s;
let rm2 = &mut s;  // 错误！
rm1.push_str("a");
```

编译器报错：

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:4:15
  |
3 |     let rm1 = &mut s;
  |               ------ first mutable borrow occurs here
4 |     let rm2 = &mut s;
  |               ^^^^^^ second mutable borrow occurs here
5 |     rm1.push_str("a");
  |     --- first borrow later used here
```

### 错误 3：悬垂引用（Dangling Reference）

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s  // s 在这里被释放！
}       // 返回的引用指向已释放的内存
```

编译器报错：

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:20
  |
1 | fn dangling() -> &String {
  |                  ^ expected named lifetime parameter
  ...
```

或者更直接的：

```
error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:3:5
  |
3 |     &s
  |     ^^ returns a reference to data owned by the current function
```

### 错误 4：在 &mut 仍然活跃时使用原变量

```rust
let mut s = String::from("hello");
let rm = &mut s;
println!("{s}");  // 错误！s 已经被可变借用
rm.push_str(" world");
```

编译器报错：

```
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
```

---

## 7. 编译器如何检查借用规则

Rust 编译器包含一个**借用检查器（Borrow Checker）**，它是编译器中最核心的部分之一。

### 7.1 借用检查器的工作原理

借用检查器在编译时追踪每一个引用的"生命周期"：

1. **每个引用被赋予一个"借用"**：记录引用的类型（`&` 或 `&mut`）、来源、和使用位置
2. **检查冲突**：确保在任何一个代码点，两个规则都被遵守
3. **推断生命周期**：自动推断引用之间的生命周期关系（大多数情况下不需要手动标注）
4. **验证生命周期**：确保引用不会比被引用的数据活得更长

### 7.2 借用检查的时机

借用检查发生在编译时，对运行时性能**零影响**。这被称为**零成本抽象**（Zero-Cost Abstraction）：

- 生成的机器码中，引用就是普通指针
- 没有引用计数、没有垃圾回收、没有运行时借用检查
- 编译器验证通过后，生成的代码和手写的 C 代码一样高效

### 7.3 NLL 如何改变了借用检查

NLL（Non-Lexical Lifetimes，非词法生命周期）是 MIR-based borrow check（基于 MIR 的借用检查）的一部分。变化在于：

- **之前**：引用的生命周期 = 从创建到所在作用域结束
- **之后**：引用的生命周期 = 从创建到最后一次使用

这使得借用检查器更加精确，允许更多正确的代码通过编译。

---

## 8. 如何修复借用相关错误

### 修复策略 1：限制作用域

使用大括号限制可变引用的作用范围：

```rust
let mut s = String::from("hello");
{
    let rm = &mut s;
    rm.push_str(" world");
} // rm 在这里离开作用域
let r = &s;  // ✅ 现在可以创建不可变引用
```

### 修复策略 2：提前使用不可变引用

确保在创建可变引用之前，不可变引用已经"使用完毕"：

```rust
let mut s = String::from("hello");
let r = &s;
println!("{r}");  // r 的最后一次使用
// r 不再使用，NLL 认为它已结束
let rm = &mut s;  // ✅
rm.push_str(" world");
```

### 修复策略 3：Clone 数据

如果你确实需要独立的数据副本：

```rust
let s = String::from("hello");
let r1 = &s;
let s2 = s.clone();  // 克隆数据，创建独立副本
let r2 = &mut s2;    // ✅ r2 指向的是 s2，不影响 s
```

### 修复策略 4：使用 Copy 类型

对于 Copy 类型（如整数、布尔），引用规则更宽松：

```rust
let x = 42;
let r1 = &x;
let r2 = &x;    // ✅ 两个不可变引用，没问题
let y = *r1;    // ✅ Copy 类型：解引用只是复制值
// x, r1, r2, y 都可以独立使用
```

### 修复策略 5：重新设计代码结构

有时借用错误暗示了代码结构问题。重构以缩短引用的生命周期：

```rust
// 不好：引用跨度太长
fn bad() {
    let mut data = vec![1, 2, 3];
    let r = &mut data;
    // ... 很多其他代码 ...
    r.push(4);
}

// 好：将借用限制在最小的作用域
fn good() {
    let mut data = vec![1, 2, 3];
    add_four(&mut data);
    // data 的所有权回来后可以做其他事
}

fn add_four(v: &mut Vec<i32>) {
    v.push(4);
}
```

---

## 9. 边界与注意事项

### 9.1 借用的传播

当你传递一个引用时，引用不能比它借用的原始数据活得更长：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 调用时，返回引用的生命周期取两个参数中较短的那个
let s1 = String::from("short");
let result;
{
    let s2 = String::from("longer string");
    result = longest(&s1, &s2);
    // result 的生命周期 = min(s1, s2) = s2 的生命周期
    println!("{result}");  // ✅ s1 和 s2 都存活
}
// s2 已释放
// println!("{result}");  // ❌ result 引用了已释放的 s2
```

### 9.2 可变引用的"传染性"

如果你有一个可变引用，你不能通过它创建其他引用（包括不可变引用），除非你先"结束"对它的使用：

```rust
let mut data = vec![1, 2, 3];
let rm = &mut data;
// 不能同时做这些：
// let r = &*rm;       // 错误
// let r2 = &rm[0];    // 错误（除非 NLL 已结束 rm）
```

### 9.3 切片索引与 UTF-8

`&str` 的索引操作是按**字节**而非字符：

```rust
let s = "你好世界";     // 每个中文字符占 3 个字节（UTF-8）
let slice = &s[0..3];  // "你"（1 个字符，3 个字节）
// let bad = &s[0..2]; // ❌ 运行时 panic！在字符中间切断了 UTF-8！
```

Rust 要求切片边界必须在 UTF-8 字符边界上，否则会 panic。这是内存安全的一部分——不会产生无效的 UTF-8 字符串。

### 9.4 &str 与 String 的互操作

```rust
// &str -> String：需要明确分配
let s1: &str = "hello";
let s2: String = s1.to_string();  // 分配新内存
let s3: String = String::from(s1); // 同上
let s4: String = s1.to_owned();    // 同上

// String -> &str：自动转换（deref coercion）
let s5 = String::from("world");
fn take_str(s: &str) { }
take_str(&s5);   // &String 自动转换为 &str
take_str(&s5[..]); // 显式获取完整切片
```

### 9.5 引用与模式匹配

模式匹配中解引用：

```rust
let x = Some(42);
match &x {
    Some(val) => println!("{val}"),  // val 自动解引用
    None => println!("None"),
}

// 等价于
match &x {
    &Some(ref val) => println!("{val}"),  // 显式 ref 模式
    &None => println!("None"),
}
```

### 9.6 ref 关键字

在模式中，`ref` 创建一个引用而非移动值：

```rust
let s = Some(String::from("hello"));
match s {
    Some(ref inner) => println!("{inner}"), // inner 是 &String，不移动
    None => {}
}
// s 仍然可用，因为它没有被移动
println!("{s:?}");
```

---

## 10. 本章总结

### 核心概念回顾

| 概念 | 符号 | 含义 |
|------|------|------|
| 引用 (Reference) | `&T` | 不拥有所有权的数据访问方式 |
| 可变引用 (Mutable Reference) | `&mut T` | 独占的、可修改的数据访问方式 |
| 解引用 (Dereference) | `*r` | 从引用获取所指向的值 |
| 借用 (Borrowing) | `&` 创建 | 创建引用的行为 |
| 切片 (Slice) | `&[T]`, `&str` | 对连续数据的借用视图 |
| 悬垂引用 (Dangling Reference) | — | 指向已释放内存的引用（Rust 禁止） |
| NLL (非词法生命周期) | — | 基于实际使用而非作用域的生命周期判定 |

### 借用规则（黄金法则）

```
┌─────────────────────────────────────────────────────┐
│                                                       │
│   规则 1：多个 &T 或 一个 &mut T，不能同时拥有         │
│   规则 2：引用必须始终有效（禁止悬垂引用）              │
│                                                       │
│   这些规则在编译时强制执行，零运行时开销                 │
│   它们从根源上消除数据竞争（Data Race）                 │
│                                                       │
└─────────────────────────────────────────────────────┘
```

### Python 与 Rust 对比总结

| 方面 | Python | Rust |
|------|--------|------|
| 内存管理 | 引用计数 + GC | 所有权 + 借用（编译时） |
| 切片 | 创建新对象（分配内存） | 借用视图（零分配） |
| 并发安全 | GIL + 运行时 | 借用检查器（编译时） |
| 悬垂引用 | 不可能（GC 处理） | 编译时阻止 |
| 性能开销 | GC 暂停 | 无运行时开销 |
| 学习曲线 | 简单 | 需要理解所有权和借用 |

### 下一步

在下一章中，我们将学习 Rust 的**生命周期（Lifetimes）**——编译器如何确保引用始终有效，以及何时需要显式标注生命周期参数 `'a`。

本章的 `longest_word<'a>` 函数只是生命周期的预览。完整的生命周章将深入讨论：
- 生命周期省略规则（Lifetime Elision）
- 结构体中的生命周期
- 静态生命周期 `'static`
- 生命周期与泛型的交互

### 推荐练习

1. 运行 `cargo run` 查看所有演示的输出
2. 逐个取消注释 `main.rs` 中的错误示例，观察编译器的错误信息
3. 尝试修改代码，故意违反借用规则，学习阅读编译器报错
4. 完成 `EXERCISES.md` 中的练习

---

## 核心术语表

| 英文 | 中文 | 说明 |
|------|------|------|
| Reference | 引用 | 使用 `&` 创建，不获取所有权 |
| Borrowing | 借用 | 创建引用的行为 |
| Dereference | 解引用 | 使用 `*` 获取引用指向的值 |
| Mutable Reference | 可变引用 | `&mut T`，独占修改权限 |
| Immutable Reference | 不可变引用 | `&T`，只读访问 |
| Slice | 切片 | `&str` / `&[T]`，借用视图 |
| Dangling Reference | 悬垂引用 | 指向已释放内存的引用 |
| Non-Lexical Lifetimes | 非词法生命周期 | 基于使用点的生命周期判定 |
| String Slice | 字符串切片 | `&str`，字符串的借用视图 |
| Array Slice | 数组切片 | `&[T]`，数组的借用视图 |
| Borrow Checker | 借用检查器 | 编译时验证借用规则的组件 |
| Data Race | 数据竞争 | 并发的读写冲突（Rust 编译时防止） |
| Fat Pointer | 胖指针 | 切片引用：包含地址和长度的双字指针 |

---

> "Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector."
> — The Rust Book

> "与其让你的代码在凌晨三点崩溃于生产环境，不如让编译器在下午三点拒绝编译它。"
> — Rust 社区谚语
