# 参考答案

建议先独立完成练习，再阅读本文件。参考答案用于比较思路，不是用来复制的。

---

## Level 1：基础巩固

### L1-1：Vec 基本操作

#### 参考实现

```rust
fn vec_operations() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 1..=5 { v.push(i); }

    if let Some(popped) = v.pop() {
        println!("弹出的元素: {}", popped);
    }

    match v.get(2) { Some(val) => println!("索引 2: {}", val), None => () }
    match v.get(10) { Some(_) => (), None => println!("索引 10: None (越界安全处理)") }

    let v2 = vec![10, 20, 30];
    v.extend(v2);

    print!("最终 Vec: [");
    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", val);
    }
    println!("]");
    v
}
```

#### 常见错误

- 用 `v[10]` 访问越界元素导致 panic — 应用 `v.get(10)` 返回 `Option`
- `extend` 后 `v2` 仍可继续使用（`extend` 不移动原 Vec）

---

### L1-2：String 与 UTF-8 探索

#### 参考实现

```rust
fn explore_utf8(s: &str) {
    println!("字节长度: {}, 字符个数: {}", s.len(), s.chars().count());
    print!("字节 (hex): ");
    for b in s.bytes() { print!("{:02x} ", b); }
    println!();
    print!("字符: ");
    for c in s.chars() { print!("'{}' ", c); }
    println!();
    match s.chars().next() { Some(c) => println!("第一个字符: '{}'", c), None => () }
}
```

#### 关键理解

- "你好"：每个汉字 3 字节（UTF-8 中文范围），所以 `.len()` = 6，`.chars().count()` = 2
- "a🚀b"：a=1B, 🚀=4B, b=1B → `.len()` = 6，`.chars().count()` = 3
- 这就是为什么 `String` 不支持 `s[i]` 索引：在变长编码中定位第 n 个字符是 O(n) 而非 O(1)

#### 常见错误

- 认为 `s.len()` 返回字符数 — 它返回字节数
- 试图用 `s[i]` 索引 — 编译器直接拒绝

---

### L1-3：HashMap 基础 —— 学生成绩管理

#### 参考实现

```rust
fn manage_scores() {
    let mut scores = HashMap::new();
    scores.insert("张三".to_string(), 85);
    scores.insert("李四".to_string(), 92);
    scores.insert("王五".to_string(), 78);
    scores.insert("赵六".to_string(), 65);
    scores.insert("孙七".to_string(), 88);

    for name in ["张三", "王五"] {
        match scores.get(name) {
            Some(s) => println!("{}: {}分", name, s),
            None => println!("{}: 未找到", name),
        }
    }
    scores.entry("赵六".to_string()).or_insert(70);  // 已存在，不覆盖

    for (name, score) in scores.iter() { println!("{}: {}分", name, score); }

    let count = scores.values().filter(|&&s| s > 80).count();
    println!(">80分的学生数: {}", count);
}
```

#### 常见错误

- `or_insert` 会返回值的可变引用，但如果键已存在则不会更新值
- `HashMap` 不保持插入顺序；若需有序，使用 `BTreeMap` 或在打印前收集到 `Vec` 并排序

---

## Level 2：综合应用

### L2-1：文本统计分析器

#### 参考实现

```rust
fn analyze(freq: &HashMap<String, usize>) {
    let distinct = freq.len();
    let total: usize = freq.values().sum();
    let avg = if distinct > 0 { total as f64 / distinct as f64 } else { 0.0 };
    let hapax = freq.values().filter(|&&c| c == 1).count();
    let top = freq.iter().max_by_key(|(_, &c)| c);

    println!("========== 文本统计报告 ==========");
    println!("不重复单词数: {}", distinct);
    println!("总单词数: {}", total);
    println!("平均出现次数: {:.2}", avg);
    println!("仅出现一次的单词数: {}", hapax);
    if let Some((word, count)) = top {
        println!("出现次数最多的单词: \"{}\" (出现 {} 次)", word, count);
    }
    println!("==================================");
}
```

---

### L2-2：简易通讯录系统

#### 核心数据结构

```rust
#[derive(Debug, Clone)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}
```

#### 参考实现

```rust
fn add_contact(book: &mut HashMap<String, Contact>, contact: Contact) {
    book.insert(contact.name.clone(), contact);
}

fn remove_contact(book: &mut HashMap<String, Contact>, name: &str) -> Option<Contact> {
    book.remove(name)
}

fn search_contacts<'a>(book: &'a HashMap<String, Contact>, query: &str) -> Vec<&'a Contact> {
    book.iter()
        .filter(|(name, _)| name.contains(query))
        .map(|(_, contact)| contact)
        .collect()
}

fn list_contacts(book: &HashMap<String, Contact>) {
    let mut names: Vec<&String> = book.keys().collect();
    names.sort();
    for name in names {
        if let Some(c) = book.get(name) {
            println!("{}: {} ({})", c.name, c.phone, c.email);
        }
    }
}
```

#### 常见错误

- `search_contacts` 返回引用时应标注生命周期 `<'a>`
- `list_contacts` 中直接对 `HashMap` 的 key 排序而非先收集

---

## Level 3：设计思考

### L3-1：选择正确的集合类型

| 需求 | 推荐 | 理由 |
|------|------|------|
| 有序列表，频繁 push/pop 尾部 | `Vec<T>` | 尾部操作 O(1) |
| 快速按键查找 | `HashMap<K,V>` | 平均 O(1) |
| 有序键、范围查询 | `BTreeMap<K,V>` | O(log n)，有序 |
| 字符串累积拼接 | `String` (push_str) | UTF-8 保证 |
| 无需所有权的字符串视图 | `&str` | 零拷贝借用 |

### L3-2：为什么 Vec 的迭代器有三种获取方式？

| 方法 | 返回 | 使用场景 |
|------|------|---------|
| `.iter()` | `&T` 引用 | 只读遍历，保留原数据 |
| `.iter_mut()` | `&mut T` 引用 | 遍历并修改 |
| `.into_iter()` | 获取所有权 | 消费 Vec，元素被移动 |

选择取决于之后是否还需要使用原 `Vec`，以及是否需要修改元素。

---

## 思考题

### Q1：为什么 `String` 不支持索引访问？

**结论**：UTF-8 是变长编码，`s[i]` 无法在 O(1) 时间内返回用户期望的"字符"。编译器直接禁用索引，强迫程序员显式选择遍历方式。

### Q2：HashMap vs BTreeMap 如何选择？

- `HashMap`：平均 O(1)，无序。适合大多数场景。
- `BTreeMap`：O(log n)，按键有序排列。适合需要范围查询或有序遍历的场景。
- 用 `std::collections` 文档比较具体性能特征。

---

*集合类型是日常编程中最常用的标准库组件。熟练掌握 Vec/String/HashMap 的 API，能大幅减少样板代码。*
