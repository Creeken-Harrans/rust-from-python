# 第五章练习: 所有权、移动与复制

> 练习是掌握所有权概念的唯一途径。务必**手写代码**并**亲自观察编译错误**, 只看不动手等于没学。

---

## 环境准备

```bash
# 确保在正确的目录
cd chapters/05_ownership_move_copy_clone

# 编译基础代码
cargo build

# 运行基础代码, 观察输出
cargo run
```

---

## Level 1: 基础练习 (3 题)

### 练习 1-1: 预测输出 —— Move 与 Copy

阅读以下代码, 在运行之前**写下你预测的输出和任何编译错误**。然后创建文件 `src/bin/ex1_1.rs` 并运行验证。

```rust
fn main() {
    let a = 10;
    let b = a;
    let c = b;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let s = String::from("Rust");
    let t = s;
    let u = t;
    // 猜一猜: 下面这行能编译吗?
    // println!("s = {}, t = {}, u = {}", s, t, u);

    println!("u = {}", u);
}
```

**任务**:
1. 写出你预测的输出
2. 取消注释中间那行 `println!`, 编译, 观察编译器报错
3. 修复代码使所有变量都能打印 (至少两种不同方法)

**核心要点**: Copy 类型的赋值是复制, 非 Copy 类型的赋值是移动。

---

### 练习 1-2: 函数所有权追踪

写出以下代码的输出顺序, 特别关注 `Drop` 的调用时机:

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
    // 猜一猜: println!("t1.id = {}", t1.id); 能编译吗?
    println!("main: take_it 返回了");

    let t2 = Tracker { id: 2 };
    println!("main: 创建了 t2 (不使用)");
    // t2 在这里被 drop
    println!("main: 马上结束了");
}
```

**任务**:
1. 写出 Drop 的调用顺序
2. 解释为什么是这个顺序 (提示: 声明顺序和逆序 drop)
3. 为什么 `tracker.id = {:?}` 那行被注释掉了? 如果取消注释会怎样?

**核心要点**: 
- 值在其所有者离开作用域时被 drop
- drop 顺序是声明顺序的逆序
- 移动到函数中的值在函数结束时被 drop

---

### 练习 1-3: 哪些类型是 Copy?

对于以下每种类型, 判断它是 Copy 还是非 Copy (Move):

| 类型 | Copy? | 理由 |
|------|-------|------|
| `u64` | | |
| `&str` | | |
| `String` | | |
| `Vec<i32>` | | |
| `(i32, bool)` | | |
| `(String, i32)` | | |
| `[i32; 10]` | | |
| `[String; 3]` | | |
| `Box<i32>` | | |
| `Option<i32>` | | |
| `Option<String>` | | |

**任务**:
1. 填写上表
2. 对每个判断写出理由
3. 写一个小程序验证你的答案 (每个类型声明一个变量, 赋值给另一个, 然后试着使用第一个)

---

## Level 2: 进阶练习 (2 题)

### 练习 2-1: 重构所有权设计

以下代码可以编译运行, 但所有权设计很糟糕 —— 到处是 clone:

```rust
struct Document {
    title: String,
    content: String,
}

fn print_title(doc: Document) {
    println!("标题: {}", doc.title);
    // doc 被 drop, content 被浪费了
}

fn word_count(doc: Document) -> usize {
    let count = doc.content.split_whitespace().count();
    // doc 被 drop
    count
}

fn main() {
    let doc = Document {
        title: String::from("所有权入门"),
        content: String::from("Rust 的所有权系统是它最独特的特性"),
    };

    // 问题: 每次只能调用一个函数, 因为会消耗所有权
    let title_doc = Document {
        title: doc.title.clone(),
        content: doc.content.clone(),
    };
    print_title(title_doc);

    let count_doc = Document {
        title: doc.title.clone(),
        content: doc.content.clone(),
    };
    let count = word_count(count_doc);
    println!("字数: {}", count);

    println!("原始标题: {}", doc.title);
    println!("原始内容: {}", doc.content);
}
```

**任务**:
1. 分析这段代码有哪些问题 (提示: 性能、所有权设计)
2. 在不使用 clone 的前提下, 修改函数签名使代码正常工作
3. 修改后, 解释 `print_title` 和 `word_count` 为什么不需要所有权

**核心要点**: 
- 如果你只需要读取数据, 不需要所有权
- 学会区分"需要拥有数据"和"需要访问数据"
- 尽管引用 (borrowing) 是下一章的内容, 你可以思考: 函数应该接受什么类型的参数?

---

### 练习 2-2: 编写一个文件处理模拟

要求实现以下函数和结构体, 体现正确的所有权管理:

```rust
struct LogEntry {
    timestamp: String,
    message: String,
    level: String,
    // 添加你需要的字段
}

// TODO: 实现以下函数

// 1. 创建一个新的 LogEntry (获取所有传入字符串的所有权)
fn create_log(timestamp: String, message: String, level: String) -> LogEntry {
    // 你的代码
}

// 2. 格式化输出日志 (不需要所有权, 但本章可能只能传所有权然后返回)
//    思考: 为什么这个函数可能需要返回 LogEntry?
fn format_log(entry: LogEntry) -> (String, LogEntry) {
    // 返回格式化字符串和 LogEntry 本身
}

// 3. 批量处理日志
fn process_logs(entries: Vec<LogEntry>) -> Vec<LogEntry> {
    // 遍历并处理每个日志条目
    // 注意 Vec 的所有权问题
}
```

**任务**:
1. 实现以上函数, 编译通过
2. 解释在 `process_logs` 中你如何处理 Vec 的所有权 (是用 for 循环? 是 clone? 还是什么?)
3. 思考: 为什么 `format_log` 的设计 (拿走 LogEntry 再返回) 在实际代码中非常笨拙? Rust 的什么特性可以解决这个问题?

**核心要点**:
- 函数获取所有权后, 如果要继续使用需要返回它
- 这种"拿走再还回来"的模式很常见, 但也很繁琐 —— 这自然引出了下一章的"借用"
- 理解所有权转移的两面性: 既是保证安全的机制, 也是需要灵活处理的约束

---

## Level 3: 挑战题 (1 题)

### 练习 3-1: 实现一个简单的 String 池 —— 代码改造

下面的代码是一个简单的字符串池 (String Pool), 用于管理一系列字符串的所有权。但是当前实现有设计缺陷:

```rust
// 保存到 src/bin/ex3_1.rs, 使用 cargo run --bin ex3_1 运行

struct StringPool {
    // 存储所有字符串, 每个字符串存储在堆上
    strings: Vec<String>,
}

impl StringPool {
    fn new() -> Self {
        StringPool { strings: Vec::new() }
    }

    // 向池中添加一个字符串
    fn add(&mut self, s: String) {
        // 先把字符串转成大写再存储
        let processed = s.to_uppercase();
        self.strings.push(processed);
        // BUG/TODO: s 的所有权去哪了? processed 是谁的?
    }

    // 获取池中所有字符串 (将所有权移出)
    fn take_all(&mut self) -> Vec<String> {
        // 问题: 我们需要把所有字符串移出, 但 self.strings 应该保留空 Vec
        // 当前的 std::mem::take 或 std::mem::replace 可以实现
        // 但你先试着用你目前掌握的知识处理
        let result = self.strings.clone(); // 用 clone 是不对的, 请改掉
        self.strings.clear();
        result
    }

    // 获取池中的条目数
    fn len(&self) -> usize {
        self.strings.len()
    }
}

fn main() {
    let mut pool = StringPool::new();

    pool.add(String::from("hello"));
    pool.add(String::from("rust"));
    pool.add(String::from("ownership"));

    println!("池中有 {} 个字符串", pool.len());

    // 取出所有字符串, 处理后销毁池
    let strings = pool.take_all();
    println!("取出了 {} 个字符串:", strings.len());
    for s in &strings {
        println!("  - {}", s);
    }

    // 池现在应该是空的
    // 因为有 &mut self, pool 还能用吗? 
    // println!("池中还有 {} 个", pool.len());
}
```

**任务**:
1. **找出并修复代码中的所有权问题**: 跟踪每个 `String` 的所有权从创建到销毁的完整流程
2. **改写 `take_all` 方法**: 将 `self.strings` 中的数据所有权真正移出 (不能用 `.clone()`)。提示: `std::mem::take()` 可以将值移出并留下默认值
3. **解释所有权流转**: 用注释标注代码中每个 String 的所有者变化
4. **添加 Drop 支持**: 为 `StringPool` 实现 `Drop`, 打印被释放的字符串数量和内容
5. **回答**: 如果 `pool.add()` 获取所有权后, 原始字符串就不能再用了。这在什么场景下是合理的? 什么场景下不合理? 你会如何改进 API?

**运行命令**:
```bash
# 创建练习文件
# 写入到 src/bin/ex3_1.rs
cargo run --bin ex3_1
```

**核心要点**:
- 所有权不只是"能不能编译"的问题, 更是 API 设计的问题
- `std::mem::take()` 和 `std::mem::replace()` 是处理所有权的重要工具
- 一个良好的 Rust API 会清楚地表明哪些函数获取所有权 (消耗值), 哪些只是借用

---

## 思考题

### 为什么 Rust 不让所有类型都默认实现 Copy?

如果 Rust 让 `String` 也实现 `Copy`, 那么我们写代码时就可以像 Python 一样:

```rust
let s1 = String::from("hello");
let s2 = s1;  // 如果 String 是 Copy, 两边都能用
println!("{}, {}", s1, s2);
```

请从以下角度分析为什么 Rust 刻意不让 `String` 实现 `Copy`:

1. **性能**: 如果 String 是 Copy, 每次赋值都会复制整个堆上的字符串内容。会发生什么?
2. **安全性**: 如果 String 是 Copy, 两个"独立"的变量实际上拥有不同的堆内存吗? 如果不是, 有什么问题?
3. **语义清晰性**: `Copy` 意味着"赋值即是简单位复制"。String 的位复制 (浅拷贝) 会导致什么问题? (提示: double free)
4. **设计哲学**: Rust 为什么选择"默认 move, 显式 clone"而不是"默认 copy, 手动 free"?

请将你的回答写成 200-400 字的段落。

---

## 练习提交检查清单

在完成所有练习后, 确认以下每一条:

- [ ] 练习 1-1: 能解释 Copy 和 Move 的行为差异
- [ ] 练习 1-2: 能正确预测 Drop 的调用顺序
- [ ] 练习 1-3: 能准确判断 10 种类型的 Copy 属性
- [ ] 练习 2-1: 能重构代码消除不必要的 clone
- [ ] 练习 2-2: 实现了 LogEntry 相关函数, 理解所有权"拿走再还"模式
- [ ] 练习 3-1: 修复了 StringPool, 理解了所有权在 API 设计中的角色
- [ ] 思考题: 写出了 200-400 字的分析

---

## 推荐执行命令

```bash
# 编译并运行主示例程序
cargo run

# 运行练习 1-1 (需要先创建文件)
# 将练习 1-1 的代码保存到 src/bin/ex1_1.rs
cargo run --bin ex1_1

# 运行练习 3-1 (需要先创建文件)
cargo run --bin ex3_1

# 查看编译器对某个错误更详细的解释
cargo build 2>&1 | head -40

# 如果你用 VS Code + rust-analyzer, 可以在代码中看到:
# - 类型提示 (哪些是 Copy)
# - 所有权转移的实时标注
# - 内联的错误提示
```

---

## 常见问题 FAQ

**Q: 我的代码到处都是 move error, 是不是我不适合学 Rust?**

A: 不是。这是每个 Rust 学习者都会经历的阶段, 包括 Rust 核心团队的成员。所有权系统是全新的心智模型, 需要时间适应。坚持手写代码, 每遇到一个 move error 就停下来理解为什么, 一两周后就会形成"所有权直觉"。

**Q: 什么时候用 clone, 什么时候用引用?**

A: 简单的判断标准:
- 如果你只需要**读取**数据 → 用引用 (borrow, `&T`)
- 如果你需要**独立修改**一份数据 → 用 clone (获取所有权)
- 如果你被编译器逼着加 `.clone()` → 停下来, 思考是不是你的所有权设计有问题, 而不是盲目加 clone

**Q: Copy 和 Clone 可以同时实现吗?**

A: 可以, 但 Clone 是 Copy 的"supertrait" (父 trait)。也就是说:
- `Copy: Clone` —— 所有 Copy 类型都实现了 Clone
- 如果一个类型实现了 Copy, 它必须也实现 Clone
- 你可以为类型实现 Clone 而不实现 Copy (如 String)
- 你不能实现 Copy 而不实现 Clone

**Q: 我的结构体有 `i32` 和 `String`, 能不能让它实现 Copy?**

A: 不能。因为 `String` 不是 Copy, 结构体就不能自动推导 Copy。一个类型要实现 Copy, 它的**所有字段**都必须实现 Copy。但你可以手动实现 `Clone`, 在需要对结构体做独立副本时显式调用 `.clone()`。

---

*练习是掌握 Rust 所有权的唯一捷径。每一个 move error 都是你通往 Rust 思维模式的阶梯。*
