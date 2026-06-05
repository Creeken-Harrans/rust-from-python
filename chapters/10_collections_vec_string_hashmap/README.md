# 第十章：集合类型 — Vec、String 与 HashMap

Rust 标准库提供了几种最常用的集合类型（Collections），用于在堆上存储多个值。与 Python 的内置容器不同，Rust 的集合类型与所有权系统深度绑定，理解它们之间的关系是掌握 Rust 的关键一步。

## 目录

1. [Vec`<T>` —— 动态数组](#1-vect--动态数组)
2. [String —— 可增长的 UTF-8 字符串](#2-string--可增长的-utf-8-字符串)
3. [HashMap`<K, V>` —— 键值对映射](#3-hashmapk-v--键值对映射)
4. [三种迭代器：iter / iter_mut / into_iter](#4-三种迭代器iter--iter_mut--into_iter)
5. [所有权与集合类型](#5-所有权与集合类型)
6. [Python 对比速查](#6-python-对比速查)
7. [常见错误与修复](#7-常见错误与修复)

---

## 1. Vec`<T>` —— 动态数组

### 1.1 什么是 Vec

`Vec<T>` 是 Rust 中最常用的集合类型之一，表示**在堆上分配的、可动态增长的同类型元素序列**。它在内存中连续存储，类似于 Python 的 `list`，但有严格的类型约束——一个 `Vec<T>` 只能存储类型为 `T` 的元素。

### 1.2 创建 Vec

```rust
// 方式一：Vec::new() + push
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// 方式二：vec! 宏（最常用）
let v = vec![1, 2, 3, 4, 5];

// 方式三：从迭代器收集
let v: Vec<_> = (0..10).collect();

// 方式四：vec![val; n] —— 重复值
let zeros = vec![0; 100];  // 100 个 0
```

### 1.3 访问元素

Rust 提供两种方式访问 Vec 中的元素：

| 方式 | 语法 | 越界行为 | 返回类型 |
|------|------|----------|----------|
| 索引 | `v[i]` | **panic（运行时崩溃）** | `T`（直接值拷贝或移动） |
| `get()` | `v.get(i)` | 返回 `None` | `Option<&T>` |

```rust
let v = vec![10, 20, 30];

// 索引：越界会 panic
let first = v[0];           // 10
// let bad = v[100];        // 运行时 panic!

// get：安全访问
match v.get(0) {
    Some(val) => println!("{val}"),
    None => println!("索引越界"),
}
match v.get(100) {
    Some(_) => {},
    None => println!("100 越界，但不会崩溃"),
}
```

**为什么 Rust 同时提供两种方式？** 索引方式假定你确定索引有效——如果错了就 crash，适合调试阶段快速暴露 bug。`get()` 返回 `Option`，适合处理用户输入等不可信索引的场景。这与 Python 不同——Python 的 `list[i]` 在越界时抛出 `IndexError`，而 Rust 直接 panic。

### 1.4 push 与 pop

```rust
let mut v = vec![1, 2, 3];
v.push(4);            // 在末尾添加 → [1, 2, 3, 4]
let last = v.pop();   // 弹出末尾 → Some(4)，v 变成 [1, 2, 3]
let empty: Option<i32> = vec![].pop();  // 空 Vec 的 pop 返回 None
```

`push` 在容量不足时会自动重新分配更大的堆内存（通常翻倍增长）。`pop` 返回 `Option<T>`。

### 1.5 其他常用方法

```rust
let mut v = vec![1, 2, 3, 4, 5];

v.len();           // 长度: 5
v.is_empty();      // false
v.first();         // Some(&1)
v.last();          // Some(&5)
v.contains(&3);    // true
v.truncate(3);     // 截断到 3 个元素 → [1, 2, 3]
v.clear();         // 清空 → []
v.insert(0, 99);   // 在索引 0 插入 99 → [99, 1, 2, 3]
v.remove(0);       // 移除索引 0 → 99
v.sort();          // 排序
v.reverse();       // 反转
```

### 1.6 所有权与 Vec

当 Vec 被销毁时，其所有元素也会被销毁。如果把元素**移动**进 Vec，则原变量不再可用：

```rust
let s = String::from("hello");
let mut v = Vec::new();
v.push(s);          // s 的所有权移入 v
// println!("{s}"); // 编译错误！s 已被移动
```

如果需要对同一个值同时存在于 Vec 和外部，有两种选择：
- **克隆（Clone）**：`v.push(s.clone());` —— 复制一份
- **引用**：使用 `Vec<&T>` —— 但需要确保引用的生命周期足够长（见第五节）

---

## 2. String —— 可增长的 UTF-8 字符串

### 2.1 String 的本质

`String` 是 Rust 标准库提供的**可增长、可变、拥有所有权的 UTF-8 编码字符串**。它在底层是一个 `Vec<u8>` 的封装，保证其中存储的字节始终是有效的 UTF-8。

与 Python 的 `str` 不同，Python 的 `str` 是不可变的，而 Rust 的 `String` 是可变的。

### 2.2 创建 String

```rust
// 方式一：String::new()
let mut s = String::new();

// 方式二：String::from(&str)
let s = String::from("hello");

// 方式三：to_string() / to_owned()
let s = "hello".to_string();
let s = "hello".to_owned();

// 方式四：format! 宏
let name = "Rust";
let s = format!("Hello, {name}!");  // 不获取参数所有权

// 方式五：从字符迭代器收集
let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();
```

### 2.3 追加与拼接

```rust
let mut s = String::new();
s.push_str("Hello");     // 追加 &str
s.push('!');             // 追加单个 char
s += " World";           // += 运算符（会获取左值所有权再赋值回去）

// format! 不获取参数所有权
let a = String::from("Hello");
let b = String::from("World");
let combined = format!("{a} {b}");  // a 和 b 仍然可用
// 对比：a + &b 会消耗 a 的所有权！
```

### 2.4 UTF-8 特性 —— 为什么 String 不支持索引

这是 Rust 初学者最容易困惑的地方之一。在 Python 中：

```python
s = "你好世界"
print(s[0])   # '你'  —— 直观
print(len(s)) # 4    —— 看起来是 4 个字符
```

但在 Rust 中：

```rust
let s = String::from("你好世界");
println!("{}", s.len());   // 12  —— 字节长度，不是字符数！
// let c = s[0];           // 编译错误！String 不支持索引
```

**原因**：Rust 的 `String` 底层是 `Vec<u8>`，存储的是 UTF-8 编码的字节序列。"你好世界" 中每个汉字占 3 字节（UTF-8 中文范围），共 4 个汉字 × 3 = 12 字节。

如果允许 `s[0]`，你期望得到什么？一个字节 `0xE4`？显然不是用户想要的。获取第一个完整的 Unicode 标量值需要检查 1-4 个字节，这不是 O(1) 操作。Rust 为了保护你不出错，**直接禁止了对 String 的索引操作**。

### 2.5 正确处理 String 中的字符

```rust
let s = String::from("你好世界");

// 获取 Unicode 标量值（chars）
for c in s.chars() {
    println!("{c}");  // '你' '好' '世' '界'
}
println!("字符数: {}", s.chars().count());  // 4

// 获取第 n 个字符（O(n)）
let third = s.chars().nth(2);  // Some('世')

// 获取原始字节（bytes）
for b in s.bytes() {
    println!("{b:#x}");  // 0xe4, 0xbd, 0xa0, ...
}

// 字节切片（必须在 UTF-8 字符边界上！）
let hello = &s[0..3];  // "你" —— 恰好是完整字符的字节范围
// let bad = &s[0..2]; // 运行时 panic！切片不在字符边界
```

### 2.6 str / &str / String / &String 关系图

```
         ┌──────────────────────────────────────────┐
         │                  str                      │
         │  (原始字符串类型，!Sized，不能直接持有)      │
         └───────┬──────────────────┬────────────────┘
                 │                  │
            &str (引用)        Box<str> (堆上拥有)
                 ▲
                 │ Deref
         ┌───────┴────────┐
         │     String      │
         │  (堆上分配，可变，│
         │   拥有所有权，    │
         │  实现了 Deref    │
         │  Target = str)  │
         └───────┬────────┘
                 │ & (引用)
                 ▼
         ┌───────────────┐
         │   &String      │───── 自动解引用 (Deref coercion) ────►  &str
         │ (对 String 的引用，│
         │  函数参数中几乎   │
         │  总是应该用 &str   │
         │  而不是 &String) │
         └────────────────┘
```

**关键结论**：
- `str` 是动态大小类型（DST），永远不能直接拥有，只能通过 `&str` 引用
- `String` 是可增长的、拥有所有权的字符串，可以修改
- 函数接受字符串参数时，**优先使用 `&str`**，因为它可以同时接受 `&String` 和 `&str`
- `&String` 会被自动强制解引用为 `&str`（Deref coercion）

### 2.7 常用方法

```rust
let s = String::from("  Hello World  ");
s.len();                    // 字节长度
s.is_empty();               // false
s.trim();                   // "Hello World" —— 返回 &str
s.to_lowercase();           // "  hello world  "
s.to_uppercase();           // "  HELLO WORLD  "
s.replace("World", "Rust"); // "  Hello Rust  "
s.contains("Hello");        // true
s.starts_with("  He");      // true
s.ends_with("  ");          // true
s.split_whitespace();       // 迭代器
s.chars().count();          // 字符个数（Unicode 标量值计数）
```

---

## 3. HashMap`<K, V>` —— 键值对映射

### 3.1 什么是 HashMap

`HashMap<K, V>` 存储键值对，通过哈希函数将键映射到值。与 Python 的 `dict` 类似，但 Rust 的 HashMap 要求所有键类型相同、所有值类型相同。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 95);
scores.insert(String::from("Bob"), 82);
```

### 3.2 创建与插入

```rust
// 方式一：new + insert
let mut map = HashMap::new();
map.insert("key1", 10);
map.insert("key2", 20);

// 方式二：从迭代器收集
let pairs = vec![("a", 1), ("b", 2)];
let map: HashMap<_, _> = pairs.into_iter().collect();
```

### 3.3 查询 —— get 方法

`HashMap::get(&key)` 返回 `Option<&V>`。注意参数是键的**引用**——HashMap 只需要借用键来查找：

```rust
let mut map = HashMap::new();
map.insert(String::from("Alice"), 95);

let name = String::from("Alice");
match map.get(&name) {          // 传 &name，不是 name
    Some(score) => println!("{score}"),
    None => println!("未找到"),
}
// name 仍然可用，因为只借用了它
```

### 3.4 Entry API —— 最强大的 HashMap 功能

`Entry API` 允许你在**一次查找中**处理"键存在"和"键不存在"两种情况，避免两次哈希计算：

```rust
let mut map = HashMap::new();
map.insert("a", 1);

// or_insert：不存在时插入默认值，返回 &mut V
let v = map.entry("a").or_insert(0);   // v = &mut 1（已存在，不覆盖）
*v += 10;                              // "a" 现在为 11

let v = map.entry("b").or_insert(0);   // 插入 0，v = &mut 0
*v += 5;                               // "b" 现在为 5

// or_insert_with：不存在时惰性计算默认值
map.entry("c").or_insert_with(|| expensive_computation());

// and_modify：存在时修改
map.entry("a")
   .and_modify(|v| *v += 1)  // 存在则 +1
   .or_insert(0);            // 不存在则插入 0
```

**Entry API 的变体**：

| 方法 | 行为 |
|------|------|
| `entry(key).or_insert(val)` | 不存在时插入 `val` |
| `entry(key).or_insert_with(f)` | 不存在时调用 `f()` 生成默认值 |
| `entry(key).or_insert_with_key(f)` | 同上，`f` 接收键引用 |
| `entry(key).and_modify(f)` | 存在时调用 `f(&mut V)` |
| `entry(key).or_default()` | 不存在时插入 `Default::default()` |

**为什么 Entry API 重要？** 在没有 Entry API 的情况下，你需要先 `contains_key` 再 `get_mut` 或 `insert`，这需要两次哈希查找。Entry API 合并为一次查找，同时借用检查器可以验证你的代码不会同时持有不可变借用和可变借用。

### 3.5 迭代 HashMap

```rust
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);

// iter(): 不可变借用，返回 (&K, &V)
for (key, value) in map.iter() {
    println!("{key}: {value}");
}

// iter_mut(): 可变借用，返回 (&K, &mut V)
for (_, value) in map.iter_mut() {
    *value += 10;
}

// into_iter(): 消耗所有权，返回 (K, V)
for (key, value) in map.into_iter() {
    println!("{key}: {value}");
}
// 此后 map 不可用
```

### 3.6 其他常用方法

```rust
map.len();                      // 键值对数量
map.is_empty();                 // 是否为空
map.contains_key("a");          // 是否包含键
map.remove("a");                // 移除键，返回 Option<V>
map.remove_entry("a");          // 移除键，返回 Option<(K, V)>
map.keys();                     // 所有键的迭代器
map.values();                   // 所有值的迭代器
map.values_mut();               // 所有值的可变引用迭代器
```

---

## 4. 三种迭代器：iter / iter_mut / into_iter

这是 Rust 集合通用的三种迭代模式：

| 迭代器 | 所有权影响 | 返回元素 | 原集合是否可用 | 典型用途 |
|--------|-----------|---------|---------------|---------|
| `.iter()` | 不可变借用 | `&T` / `(&K, &V)` | 是 | 只读遍历 |
| `.iter_mut()` | 可变借用 | `&mut T` / `(&K, &mut V)` | 是（可修改元素） | 就地修改 |
| `.into_iter()` | **消耗所有权** | `T` / `(K, V)` | **否** | 转移所有权、消费集合 |

```rust
// iter() —— 只读
let v = vec![1, 2, 3];
for x in v.iter() { println!("{x}"); }
println!("{v:?}");  // v 仍然可用

// iter_mut() —— 可变修改
let mut v = vec![1, 2, 3];
for x in v.iter_mut() { *x *= 2; }
println!("{v:?}");  // [2, 4, 6]，v 仍可用

// into_iter() —— 消耗集合
let v = vec![1, 2, 3];
let sum: i32 = v.into_iter().sum();
// println!("{v:?}");  // 编译错误！
```

**Python 对比**：Python 的 `for x in list` 等价于 Rust 的 `.iter()`——都是借用。但 Python 没有所有权概念，所以不存在 `.into_iter()` 的语义。Rust 的 `.into_iter()` 是唯一能够将元素的所有权转移出去的迭代方式。

---

## 5. 所有权与集合类型

### 5.1 核心原则

当向集合中插入元素时，集合**获取元素的所有权**：

```rust
let s = String::from("hello");
let mut v = Vec::new();
v.push(s);          // s 的所有权移入 v
// println!("{s}"); // 编译错误！
```

### 5.2 Owned vs Borrowed in Collections

这是 Rust 设计中最重要的权衡之一：

| 选择 | 优点 | 缺点 |
|------|------|------|
| `HashMap<String, V>` (Owned) | 生命周期独立，使用方便 | 需要克隆，有额外内存开销 |
| `HashMap<&str, V>` (Borrowed) | 零拷贝，高效 | 受生命周期约束，容易写出悬垂引用 |

**建议**：除非你对生命周期非常熟悉且性能分析显示此处是关键路径，否则**始终在 HashMap 的键类型中使用 String 而非 &str**。

```rust
// 推荐：使用 owned 类型
let mut map: HashMap<String, i32> = HashMap::new();
map.insert(String::from("key"), 10);

// 谨慎：使用 borrowed 类型（容易出现生命周期问题）
// 以下代码无法编译：
// let mut map: HashMap<&str, i32> = HashMap::new();
// {
//     let temp = format!("key_{}", 42);
//     map.insert(&temp, 10);  // temp 生命周期不够长！
// }  // temp 在此被释放
// println!("{map:?}");  // 悬垂引用！
```

### 5.3 Vec 与所有权

```rust
let mut v: Vec<String> = Vec::new();
{
    let s = String::from("hello");
    v.push(s);           // s 的所有权移入 v
}  // s 离开作用域，但它的值已经安全地保存在 v 中了

// 从 Vec 中移出元素
let first = v.remove(0);  // first: String，所有权移出
// v 中该位置被移除，后续元素向前移动

let second = v.swap_remove(0);  // 更快，但不保持顺序
```

---

## 6. Python 对比速查

| 场景 | Python | Rust |
|------|--------|------|
| 动态数组 | `list = [1, 2, 3]` | `let v = vec![1, 2, 3];` |
| 末尾添加 | `list.append(4)` | `v.push(4);` |
| 末尾弹出 | `list.pop()` | `v.pop()` 返回 `Option<T>` |
| 越界访问 | 抛出 `IndexError` | `v[i]` panic; `v.get(i)` 返回 `None` |
| 不可变字符串 | `str` (默认不可变) | `&str` (借用，不可变) |
| 可变字符串 | `list` of chars / `io.StringIO` | `String` (可变，拥有所有权) |
| 字符串拼接 | `f"{a} {b}"` | `format!("{a} {b}")` |
| 字符串索引 | `s[0]` — O(1) | `s[0]` — **编译错误！** |
| 遍历字符 | `for c in s:` | `for c in s.chars()` |
| 字典/映射 | `d = {"a": 1}` | `let mut m = HashMap::new(); m.insert("a", 1);` |
| 查询键 | `d["a"]` → 可能 `KeyError` | `m["a"]` — **编译错误！** 用 `m.get("a")` 返回 `Option` |
| 默认值 | `d.get("a", 0)` | `m.get("a").copied().unwrap_or(0)` |
| 遍历键值对 | `for k, v in d.items():` | `for (k, v) in m.iter()` |
| 迭代器消耗 | `sum(list)` — list 仍可用 | `v.into_iter().sum()` — v 不可用 |
| 空容器 | `[]`, `{}`, `""` | `Vec::new()`, `HashMap::new()`, `String::new()` |

### 关键差异总结

1. **Python 的 `list`** 可以存储任意类型（动态类型），`Vec<T>` 只能存储单一类型 `T`（静态类型）。
2. **Python 的 `str`** 是不可变的，但可以按索引访问字符；Rust 的 `String` 是可变但**不支持索引访问**。
3. **Python 的 `dict`** 查询不存在的键抛出 `KeyError`；Rust 的 `HashMap::get` 返回 `Option`，强制你处理缺失情况。
4. **Rust 的所有权**意味着将元素放入集合就是转移所有权；Python 中引用语义意味着 "放入的是引用"。

---

## 7. 常见错误与修复

### 错误 1：对 String 使用索引

```rust
// ❌ 错误
let s = String::from("hello");
let c = s[0];  // 编译错误：`String` cannot be indexed by `{integer}`
```

**修复**：
```rust
// ✅ 正确：使用 chars() 迭代器
let first_char: Option<char> = s.chars().next();

// ✅ 正确：获取字节切片（需确保在字符边界上）
let slice: &str = &s[0..1];  // ASCII 字符占 1 字节，安全
```

### 错误 2：读取后修改 —— 借用冲突

```rust
// ❌ 错误
let mut v = vec![1, 2, 3];
let first = &v[0];     // 不可变借用
v.push(4);             // 可变借用 —— 冲突！
println!("{first}");   // 使用不可变借用
```

**修复**：
```rust
// ✅ 方案一：先复制，再修改
let mut v = vec![1, 2, 3];
let first = v[0];      // i32 实现了 Copy，直接复制值
v.push(4);
println!("{first}");

// ✅ 方案二：调整顺序
let mut v = vec![1, 2, 3];
v.push(4);
let first = &v[0];
println!("{first}");
```

### 错误 3：HashMap 中持有临时字符串的引用

```rust
// ❌ 错误
let mut map: HashMap<&str, i32> = HashMap::new();
{
    let key = String::from("temp");
    map.insert(&key, 10);
}  // key 被释放，map 中的 &str 成为悬垂引用
```

**修复**：
```rust
// ✅ 正确：使用 String 作为键
let mut map: HashMap<String, i32> = HashMap::new();
{
    let key = String::from("temp");
    map.insert(key, 10);  // 所有权转移
}  // key 被移动，值安全保存在 map 中
```

### 错误 4：从空 Vec pop

```rust
// ❌ 不处理 None 的情况
let mut v: Vec<i32> = Vec::new();
let x = v.pop().unwrap();  // panic! 因为 pop 返回 None
```

**修复**：
```rust
// ✅ 正确
let mut v: Vec<i32> = Vec::new();
match v.pop() {
    Some(x) => println!("{x}"),
    None => println!("Vec 为空"),
}
```

### 错误 5：试图在 for 循环中修改 HashMap

```rust
// ❌ 错误
let mut map = HashMap::new();
map.insert("a", 1);
for (k, v) in map.iter() {
    map.insert("b", 2);  // 编译错误：不能同时持有不可变借用和可变借用
}
```

**修复**：
```rust
// ✅ 先收集需要修改的数据
let mut map = HashMap::new();
map.insert("a", 1);
let mut updates: Vec<String> = Vec::new();
for (k, v) in map.iter() {
    updates.push(format!("{k}_new"));
}
for key in updates {
    map.insert(key, 0);
}
```

### 错误 6：混淆 iter() 和 into_iter()

```rust
// ❌ 错误
let v = vec![1, 2, 3];
for x in v.into_iter() {
    // ...
}
println!("{v:?}");  // 编译错误！v 已被 consumed
```

**修复**：
```rust
// ✅ 如果后续还需要 v，使用 iter()
let v = vec![1, 2, 3];
for x in v.iter() {
    // ...
}
println!("{v:?}");  // v 仍然可用
```

---

## 总结

- **Vec** 用于有序同类型元素序列，索引访问 O(1)，越界索引 panic，用 `get()` 安全访问
- **String** 是可变 UTF-8 字符串，不支持索引（因 UTF-8 字符非定长），用 `.chars()` 遍历字符
- **HashMap** 用于键值对映射，`get()` 返回 `Option<&V>`，Entry API 是最强大的单次查找+插入模式
- **iter / iter_mut / into_iter** 分别对应不可变借用、可变借用、消耗所有权三种迭代方式
- **所有权**是 Rust 集合类型设计的核心——优先使用 Owned 类型作为键和元素，引用类型仅用于生命周期有保障的场景
- 与 Python 最大的不同：Rust 的类型安全是编译期保证的，所有边界情况通过 `Option`、`Result` 等类型显式处理

下一步建议：动手修改 `src/main.rs` 中的示例，尝试添加自己的文本，观察不同操作的输出结果。
