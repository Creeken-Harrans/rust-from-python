# 猜数字游戏 (Guessing Game)

## Rust 基础综合练习项目

---

## 目录

1. [项目目标](#1-项目目标)
2. [需求分析](#2-需求分析)
3. [涉及的知识点](#3-涉及的知识点)
4. [目录结构](#4-目录结构)
5. [快速开始](#5-快速开始)
6. [预期输出示例](#6-预期输出示例)
7. [设计决策](#7-设计决策)
8. [代码讲解](#8-代码讲解)
9. [测试方法](#9-测试方法)
10. [扩展方向](#10-扩展方向)
11. [常见问题](#11-常见问题)

---

## 1. 项目目标

本项目旨在通过实现一个经典的命令行猜数字游戏，综合练习 Rust 编程语言的基础知识。游戏规则简单：程序随机生成一个 1 到 100 之间的秘密数字，玩家通过不断猜测来找到这个数字。每次猜测后，程序会提示"太大了"或"太小了"，直到玩家猜对为止。

### 具体学习目标

- 掌握 Rust 项目的创建和依赖管理（Cargo）
- 理解外部 crate 的引入和使用（`rand`）
- 深入理解 Rust 的所有权（Ownership）、借用（Borrowing）和引用（References）机制
- 熟练使用 `Result` 和 `Option` 枚举进行错误处理
- 掌握 `match` 模式匹配的各种用法
- 理解 `loop`、`while`、`for` 循环的区别和适用场景
- 学习标准输入输出处理（`std::io`）
- 编写带有文档注释（`///`）的 Rust 函数
- 了解集成测试的基本写法

---

## 2. 需求分析

### 功能需求

| 编号 | 需求描述                                    | 优先级 |
|------|---------------------------------------------|--------|
| F1   | 生成 1-100 之间的随机秘密数字                | P0     |
| F2   | 循环接收用户输入                             | P0     |
| F3   | 将输入解析为整数                             | P0     |
| F4   | 比较猜测与答案，给出"太大"/"太小"/"正确"提示   | P0     |
| F5   | 跟踪并显示尝试次数                           | P1     |
| F6   | 处理非法输入（非数字、空输入、超出范围）      | P1     |
| F7   | 处理 EOF (Ctrl+D) 优雅退出                  | P2     |
| F8   | 根据尝试次数给予评价                         | P2     |
| F9   | 调试模式下显示答案                           | P3     |
| F10  | 正确时打印漂亮的胜利信息                      | P3     |

### 非功能需求

- 代码使用 Rust 2024 edition
- 所有公开函数必须有 `///` 文档注释
- 核心逻辑封装在独立函数中，`main` 函数保持简洁
- 错误处理不能导致 panic（除不可恢复的系统错误外）
- 代码至少 120 行，组织结构清晰

### 边界情况

- 用户输入空行
- 用户输入非数字字符（如 "abc"、"12.5"、"1e5"）
- 用户输入超出 1-100 范围的数字
- 用户输入非常大的数字（超过 u32::MAX）
- 用户输入前后有空格（如 " 42 "）
- 用户按下 Ctrl+D 或 Ctrl+C
- 标准输入被重定向或管道输入

---

## 3. 涉及的知识点

### 3.1 项目与依赖管理（对应 Cargo 章节）

- `Cargo.toml` 的 `[package]` 和 `[dependencies]` 配置
- `cargo run` 运行项目
- `cargo test` 运行测试
- `cargo build` 编译项目
- edition = "2024" 的含义

### 3.2 外部 crate（对应 extern crate / use 章节）

- `rand = "0.8"` 的语义化版本号（SemVer）
- `use rand::Rng;` 将 trait 引入作用域
- Cargo 自动下载和解析依赖
- Cargo.lock 锁定依赖版本

### 3.3 所有权系统（对应 Ownership 章节）

- **所有权规则**：每个值只有一个所有者；所有者离开作用域时值被释放
- **移动语义**：`String` 从 `read_input()` 返回时所有权转移给调用者
- **借用**：`&mut input` 可变借用传递给 `read_line`
- **引用**：`&str` 不可变引用，不获取所有权
- **Copy 类型**：`u32` 等基本类型自动复制，不涉及所有权转移

### 3.4 Result 与错误处理（对应 Error Handling 章节）

- `io::Result<T>` 是 `Result<T, io::Error>` 的别名
- `parse::<u32>()` 返回 `Result<u32, ParseIntError>`
- `?` 运算符传播错误
- `match` 解构 `Ok` 和 `Err` 变体
- 自定义错误消息（`Err(String)` 模式）

### 3.5 match 模式匹配（对应 Match 章节）

- 匹配枚举变体：`Ok(val)` / `Err(e)`
- 匹配 `Ordering::{Less, Equal, Greater}`
- 范围模式：`2..=5`（包含上界）
- 字面量模式：`1`
- 通配符模式：`_`
- 穷尽性检查：编译器强制覆盖所有可能情况

### 3.6 循环与控制流（对应 Control Flow 章节）

- `loop { ... }` 无限循环
- `break` 退出循环
- `continue` 跳过当前迭代
- `return` 退出函数
- `if` 条件判断
- `loop` vs `while` 的设计选择

### 3.7 标准输入输出（对应 std::io 章节）

- `std::io::stdin()` 获取标准输入句柄
- `read_line(&mut String)` 读取一行
- `print!()` vs `println!()` 的区别
- `stdout().flush()` 刷新输出缓冲区
- `io::ErrorKind` 判断错误类型

### 3.8 注释与文档（对应 Comments 章节）

- `///` 文档注释（支持 Markdown）
- `//!` 模块级文档注释
- `//` 普通行注释
- `cargo doc --open` 生成 HTML 文档

### 3.9 测试（对应 Testing 章节）

- `#[cfg(test)]` 条件编译
- `#[test]` 属性标记测试函数
- `assert!` / `assert_eq!` / `assert_ne!` 断言宏
- 单元测试 vs 集成测试
- `tests/` 目录存放集成测试

### 3.10 条件编译（对应 Attributes 章节）

- `#[cfg(debug_assertions)]` 仅在调试模式下编译
- 用于开发调试，不影响 release 构建

---

## 4. 目录结构

```
01_guessing_game/
├── Cargo.toml              # 项目配置文件（包名、版本、依赖）
├── Cargo.lock              # 依赖锁定文件（自动生成）
├── README.md               # 项目文档（本文件）
├── src/
│   └── main.rs             # 主程序源代码（含所有函数）
└── tests/
    └── integration_test.rs # 集成测试
```

### 文件说明

| 文件                     | 作用                                              |
|--------------------------|---------------------------------------------------|
| `Cargo.toml`             | 定义包元数据和依赖，是 Cargo 的入口配置文件           |
| `src/main.rs`            | 程序入口，包含所有业务逻辑函数和文档注释              |
| `tests/integration_test.rs` | 集成测试，从外部测试 `parse_guess` 等功能          |

---

## 5. 快速开始

### 前置条件

- Rust 工具链 1.80+（本项目使用 2024 edition）
- 网络连接（用于下载 `rand` 依赖）

### 运行游戏

```bash
# 进入项目目录
cd projects/01_guessing_game

# 直接运行
cargo run

# 或显式指定包名（在 workspace 中需要）
cargo run -p guessing_game

# Release 模式运行（优化后性能更好）
cargo run --release
```

### 编译（不运行）

```bash
cargo build              # Debug 模式
cargo build --release    # Release 模式
```

### 查看文档

```bash
cargo doc --open         # 生成并打开 API 文档
```

### 运行测试

```bash
cargo test               # 运行所有测试
cargo test -- --nocapture  # 显示测试中的 println 输出
```

### 检查代码（不编译生成二进制）

```bash
cargo check              # 快速检查代码能否编译
```

---

## 6. 预期输出示例

### 6.1 正常游戏流程

```
[调试] 秘密数字是: 42

╔══════════════════════════════════════╗
║          🎯 猜数字游戏 🎯           ║
╚══════════════════════════════════════╝

游戏规则：
  1. 我已经想好了一个 1 到 100 之间的秘密数字
  2. 你需要猜出这个数字
  3. 每次猜测后我会告诉你是太大还是太小
  4. 继续猜直到猜对为止！

💡 提示：按 Ctrl+D (EOF) 可以随时退出游戏

请输入你的猜测 (1-100): 50
📈 太大了！往下猜。

请输入你的猜测 (1-100): 25
📉 太小了！往上猜。

请输入你的猜测 (1-100): 37
📉 太小了！往上猜。

请输入你的猜测 (1-100): 42

╔══════════════════════════════════════╗
║          🎉 恭喜你猜对了！ 🎉        ║
╚══════════════════════════════════════╝

秘密数字: 42
总尝试次数: 4

👏 非常厉害！你很快找到了答案。
```

### 6.2 非法输入处理

```
请输入你的猜测 (1-100): abc
❌ 'abc' 不是一个有效的数字，请输入 1-100 之间的整数
请确保输入的是 1 到 100 之间的整数。

请输入你的猜测 (1-100):
❌ 输入为空，请输入一个数字
请确保输入的是 1 到 100 之间的整数。

请输入你的猜测 (1-100): 150
❌ 数字超出范围！请输入 1 到 100 之间的整数。

请输入你的猜测 (1-100): -5
❌ '-5' 不是一个有效的数字，请输入 1-100 之间的整数
请确保输入的是 1 到 100 之间的整数。
```

### 6.3 EOF 退出

```
请输入你的猜测 (1-100): ^D
👋 检测到 EOF (Ctrl+D)，游戏结束。
秘密数字是: 42
你进行了 0 次尝试。
```

### 6.4 一次猜中

```
请输入你的猜测 (1-100): 42

╔══════════════════════════════════════╗
║          🎉 恭喜你猜对了！ 🎉        ║
╚══════════════════════════════════════╝

秘密数字: 42
总尝试次数: 1

🌟 一次就中！你是天才！理论最优解！
```

---

## 7. 设计决策

### 7.1 为什么使用 `loop` 而不是 `while`？

这是最常见的问题之一。选择 `loop` 的理由：

**1. 语义精确性**

```rust
// while 方式 — 需要一个"永远为真"的条件
let mut playing = true;
while playing {
    // ...
    playing = false; // 间接控制退出
}

// loop 方式 — 直接表达"无限循环，直到 break"
loop {
    // ...
    break; // 直接控制退出
}
```

`loop` 表达的是"我要无限循环，在合适的时候退出"。
`while condition` 表达的是"只要条件满足就循环"。
猜数字游戏中没有一个自然的循环条件，循环是否继续取决于猜测结果，
这是一个多分支决策的结果，而不是一个简单的布尔条件。

**2. 没有伪造的循环变量**

使用 `while` 需要一个 `playing: bool` 变量来跟踪状态，
增加了不必要的状态管理。`loop` 不需要任何外部状态，
所有的退出逻辑都在 `break` 和 `return` 中。

**3. 编译器的理解**

Rust 编译器知道 `loop` 至少会执行一次（循环体至少运行一次），
而 `while` 可能一次都不执行。在某些场景下，
这个信息能帮助编译器做出更好的优化。

**4. 表达式能力（进阶）**

```rust
let result = loop {
    // ...
    break 42; // loop 可以作为表达式返回值
};
```

虽然本游戏没有使用这个特性，但 `loop` 可以用 `break value` 返回值，
这使得它在某些场景下非常强大。

**5. Rust 社区惯例**

在 Rust 社区中，当循环的退出条件来自循环体内部的复杂逻辑时，
`loop { ... break; }` 是惯用写法。Rust 标准库和官方文档中大量使用这种模式。

### 7.2 为什么使用 `match` 而不是 `if-else`？

**1. 穷尽性检查（Exhaustiveness Check）**

这是 `match` 最大的优势。当匹配枚举（如 `Ordering`、`Result`）时，
编译器会强制你处理所有可能的变体：

```rust
// 这段代码会编译失败！
match ordering {
    Ordering::Less => { /* ... */ }
    Ordering::Greater => { /* ... */ }
    // 错误: missing match arm: `Equal` not covered
}
```

在 `if-else` 链中，遗漏分支是运行时 bug；在 `match` 中，它是编译错误。
这种"将错误从运行时提前到编译时"的理念是 Rust 设计的核心哲学之一。

**2. 与枚举类型的天然配合**

Rust 的枚举是"和类型"（sum type），
`match` 是消费和类型的标准方式。两者的配合天衣无缝：

```rust
match result {
    Ok(value) => { /* 使用 value */ }
    Err(error) => { /* 使用 error */ }
}
```

`if let` 和 `while let` 是 `match` 的语法糖，
用于只需要处理一个变体的场景。

**3. 代码可读性**

对于三路或多路分支，`match` 的结构比 `if-else if-else` 更清晰：

```rust
// match — 结构清晰
match guess.cmp(&secret) {
    Ordering::Less => println!("太小"),
    Ordering::Greater => println!("太大"),
    Ordering::Equal => println!("正确"),
}

// if-else — 需要理解多个条件
if guess < secret {
    println!("太小");
} else if guess > secret {
    println!("太大");
} else {
    println!("正确");
}
```

### 7.3 所有权在输入处理中的体现

这是理解 Rust 所有权系统的最佳示例之一。完整的输入处理流程：

```
1. 创建 String
   let mut input = String::new();
   → input 拥有堆上的字符串缓冲区

2. 借用给 read_line
   io::stdin().read_line(&mut input)
   → &mut input 是可变引用（借用）
   → read_line 临时使用但不拥有 input
   → 借用在 read_line 返回后结束

3. 所有权转移
   Ok(input) — 从 read_input 返回
   → input 的所有权从函数转移给调用者

4. 借用给 parse_guess
   parse_guess(&input)
   → &input 是不可变引用
   → parse_guess 只读取，不修改
   → input 的所有权仍然属于调用者

5. 自动释放
   当 input 离开作用域时，
   堆上的字符串缓冲区被自动释放
   → 不需要手动 free/delete！
```

**关键点**：整个过程没有垃圾回收，也没有手动内存管理。
编译器在编译时就确定了所有权的转移和释放时机，
在运行时零成本。

**反面案例**（如果 Rust 没有所有权系统）：
```c
// C 语言风格 — 容易出错
char *input = malloc(BUFFER_SIZE);  // 分配
read_line(input);                    // 使用
// 谁负责释放？调用者？被调用者？
// 忘了 free 会内存泄漏
// 提前 free 会导致 use-after-free
free(input);
```

### 7.4 为什么参数使用 `&str` 而不是 `String`？

`parse_guess` 函数接受 `&str` 而不是 `String`。理解这个设计：

**内存视角**：

```
String:   [ ptr | len | cap ] → 堆: [ '4', '2', '\n', ... ]
&str:     [ ptr | len ]       → 堆: [ '4', '2', '\n', ... ] (同一块内存)

&str 不拥有数据，只是一个"视图"
```

**设计优势**：

| 方面         | `&str` 参数                        | `String` 参数                        |
|-------------|-----------------------------------|-------------------------------------|
| 所有权       | 借用，不获取所有权                   | 获取所有权，调用者不能再使用           |
| 灵活性       | 接受 `&String`、`&str`、字符串字面量  | 只能用 `String`，需要 `.clone()`     |
| 性能         | 零拷贝（栈上拷贝指针+长度）           | 堆分配 + 数据拷贝                      |
| 内存         | 不分配新内存                        | 分配新的堆内存                        |
| 推荐         | 只读访问时的首选                     | 需要拥有和修改数据时使用               |

**Rust 设计原则**：接受 `&str`，返回 `String` 或 `&str`
（取决于返回的数据是否需要拥有所有权）。

---

## 8. 代码讲解

### 8.1 模块级文档 (`//!`)

```rust
//! # 猜数字游戏 (Guessing Game)
//!
//! 一个经典的命令行猜数字游戏...
```

`//!` 注释是模块级文档注释，描述整个 crate 或模块的用途。
与 `///`（函数/结构体文档）不同，`//!` 写在模块的顶部。
运行 `cargo doc --open` 后，这段内容会出现在 crate 首页。

### 8.2 外部 crate 引入

```rust
use rand::Rng;          // trait — 需要引入才能调用 .gen_range()
use std::cmp::Ordering; // 枚举 — Less, Equal, Greater
use std::io::{self, Write}; // 模块自身 + Write trait (用于 flush)
```

`use` 语句将外部路径引入当前作用域：
- `rand::Rng` 是一个 **trait**，需要 use 才能调用它的方法
- `{self, Write}` 是嵌套导入语法 — `io` 模块本身 + `io::Write` trait

### 8.3 `generate_secret()` — 随机数生成

```rust
fn generate_secret() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
```

逐层解释：
1. `rand::thread_rng()` — 获取线程本地的 CSPRNG（密码学安全伪随机数生成器）
2. `.gen_range(1..=100)` — 调用 `Rng` trait 的方法，生成 1 到 100（含）的随机数
3. `1..=100` — `RangeInclusive<u32>` 语法，`=` 表示包含上界

**备选方案（无 rand 时）**：
```rust
fn generate_secret() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (seed % 100 + 1) as u32
}
```

### 8.4 `read_input()` — 读取与所有权

```rust
fn read_input() -> io::Result<String> {
    print!("请输入你的猜测 (1-100): ");
    io::stdout().flush()?;        // [?运算符] 传播错误

    let mut input = String::new(); // [堆分配] input 拥有内存
    match io::stdin().read_line(&mut input) { // [可变借用]
        Ok(0) => Err(io::Error::new(...)),    // [EOF处理]
        Ok(_) => Ok(input),                   // [所有权转移]
        Err(e) => Err(e),                     // [错误传播]
    }
}
```

**所有权流转图**：

```
String::new() 创建
    │
    ▼
read_line(&mut input) — 借用 input，追加数据
    │
    ▼
Ok(input) — 返回，所有权转移给调用者
    │
    ▼
调用者拥有 String，离开作用域时自动 Drop
```

### 8.5 `parse_guess()` — 解析与借用

```rust
fn parse_guess(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();       // [&str切片] 零拷贝
    if trimmed.is_empty() {
        return Err("输入为空...".to_string());
    }
    match trimmed.parse::<u32>() {    // [turbofish语法]
        Ok(num) => Ok(num),
        Err(_) => Err(format!("...")),
    }
}
```

**关键点**：
- `&str` 参数 — 不获取所有权，只借用
- `trim()` — 返回原始字符串的切片引用，不分配新内存
- `parse::<u32>()` — turbofish 语法指定泛型参数
- `format!()` — 返回 `String`，这是堆分配

### 8.6 `game_loop()` — 主循环

这是程序的核心，展示了完整的错误处理流程：

```
loop {
    attempts += 1;

    let input = match read_input() {
        Ok(s) => s,           // 继续
        Err(EOF) => return,   // 退出
        Err(e) => return,     // 退出
    };

    let guess = match parse_guess(&input) {
        Ok(n) => n,           // 继续
        Err(msg) => continue, // 重新输入
    };

    if guess out of range { continue; }

    match check_guess(guess, secret) {
        Less => continue,     // 重新输入
        Greater => continue,  // 重新输入
        Equal => break,       // 胜利！
    }
}
```

每个步骤的错误处理策略不同：

| 错误         | 策略       | 理由                              |
|-------------|-----------|-----------------------------------|
| EOF         | `return`  | 用户主动退出，结束程序               |
| 解析失败     | `continue` | 用户输入错误，给重新输入的机会        |
| 超出范围     | `continue` | 同样是输入错误，需要重新输入          |
| I/O 其他错误 | `return`  | 不可恢复的错误，终止程序              |

### 8.7 `main()` — 入口点

```rust
fn main() {
    game_loop();
}
```

`main` 保持极简。这是良好的工程实践：
- **单一职责**：`main` 负责启动，`game_loop` 负责逻辑
- **可测试性**：可以单独测试 `game_loop` 的逻辑（虽然需要 mock stdin）
- **可扩展性**：未来可以添加命令行参数解析、配置加载等，而不影响游戏逻辑

---

## 9. 测试方法

### 9.1 单元测试

在 `src/main.rs` 文件末尾添加单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_guess_valid() {
        assert_eq!(parse_guess("42").unwrap(), 42);
        assert_eq!(parse_guess("  100  ").unwrap(), 100);
        assert_eq!(parse_guess("1").unwrap(), 1);
    }

    #[test]
    fn test_parse_guess_invalid() {
        assert!(parse_guess("abc").is_err());
        assert!(parse_guess("").is_err());
        assert!(parse_guess("12.5").is_err());
    }

    #[test]
    fn test_check_guess() {
        use std::cmp::Ordering;
        assert_eq!(check_guess(50, 50), Ordering::Equal);
        assert_eq!(check_guess(30, 50), Ordering::Less);
        assert_eq!(check_guess(70, 50), Ordering::Greater);
    }
}
```

### 9.2 集成测试

在 `tests/integration_test.rs` 中（见该文件），测试 `parse_guess` 函数。

### 9.3 手动测试清单

运行 `cargo run` 后执行以下测试：

| 测试场景               | 操作                            | 预期结果                  |
|-----------------------|---------------------------------|--------------------------|
| 正常猜数               | 输入合法数字                      | 给出大小提示               |
| 猜对                   | 输入正确答案                      | 显示胜利信息并退出          |
| 非数字输入              | 输入 "abc"                      | 显示错误提示，继续循环       |
| 空输入                 | 直接按回车                        | 显示"输入为空"提示          |
| 带空格输入              | 输入 " 42 "                     | 正常解析并比较              |
| 超出范围（上界）         | 输入 101                        | 显示"超出范围"提示          |
| 超出范围（下界）         | 输入 0                          | 显示"超出范围"提示          |
| 负号输入                | 输入 -5                         | 显示"不是有效数字"提示      |
| EOF                   | 按 Ctrl+D                       | 揭示答案并退出              |
| 多次尝试               | 连续猜测直到猜对                   | 正确显示尝试次数            |

### 9.4 运行测试命令

```bash
# 运行所有测试
cargo test

# 显示测试输出（包括 println）
cargo test -- --nocapture

# 运行特定测试
cargo test test_parse_guess

# 运行集成测试
cargo test --test integration_test
```

---

## 10. 扩展方向

以下是可以在掌握基础后继续实现的扩展功能，难度分级供参考。

### 10.1 初级扩展（1-2 小时）

**难度选择**
- 让用户选择难度等级：简单 (1-50)、中等 (1-100)、困难 (1-200)
- 根据难度调整随机数范围和提示信息

**重玩功能**
- 猜对后询问"是否再来一局？(y/n)"
- 支持多轮游戏，累计统计总成绩

**提示范围**
- 显示当前的上下界：如 "当前范围: 1-50"（根据历史猜测动态缩小）
- 实现二分查找策略的可视化

### 10.2 中级扩展（3-5 小时）

**排行榜系统**
- 使用文件存储历史最好成绩（最少尝试次数）
- 按难度级别分别排名
- 支持查看排行榜命令（如输入 "rank"）

**保存/加载游戏**
- 将当前游戏状态序列化到文件
- 下次启动时询问是否恢复上次游戏
- 使用 `serde` crate 进行序列化

**计时功能**
- 记录每局游戏耗时
- 显示平均猜测时间
- 排行榜中加入时间维度

**提示系统**
- 猜错 5 次后提供提示（如："这个数是偶数"、"这个数能被 3 整除"）
- 实现冷却时间，避免连续滥用提示

### 10.3 高级扩展（1-2 天）

**多人对战模式**
- 交替回合制：两个玩家轮流猜，先猜对的获胜
- 服务器/客户端架构（使用 TCP）
- 使用 `tokio` 实现异步网络通信

**AI 对手模式**
- 实现一个使用二分查找策略的 AI 对手
- 对比人类和 AI 的猜测效率
- 扩展到更复杂的搜索算法

**图形界面（GUI）**
- 使用 `egui` 或 `iced` crate 创建图形界面
- 实时显示猜测历史和范围变化
- 添加动画效果

**统计与分析**
- 记录所有历史游戏数据到 SQLite 数据库
- 生成统计报告：平均尝试次数、猜测分布图
- 分析用户的猜测策略

### 10.4 架构改进

**模块化重构**
```
src/
├── main.rs          # 入口点
├── game.rs          # 游戏核心逻辑
├── input.rs         # 输入处理
├── output.rs        # 输出格式化
├── random.rs        # 随机数生成
├── stats.rs         # 统计功能
└── config.rs        # 配置管理
```

**配置系统**
- 使用 `toml` 或 `serde` 读取配置文件
- 支持环境变量覆盖配置
- 命令行参数解析（使用 `clap` crate）

**国际化（i18n）**
- 使用 `rust-i18n` 或 `fluent` crate
- 支持中英文切换
- 动态加载翻译文件

---

## 11. 常见问题

### Q1: 运行 `cargo run` 时显示 "could not find `rand`"

A: 确保 `Cargo.toml` 中有 `rand = "0.8"` 依赖声明，并且网络连接正常。
首次运行 Cargo 会自动下载依赖。

### Q2: 为什么 `parse_guess` 接受 `&str` 而不是 `String`？

A: 参见 [设计决策 7.4](#74-为什么参数使用-str-而不是-string)。
简单说：不获取所有权（更灵活），零拷贝（更高效）。

### Q3: `continue` 和 `break` 的区别？

A:
- `continue` — 跳过当前循环迭代的剩余部分，开始下一次迭代
- `break` — 完全退出当前循环

### Q4: `?` 运算符的作用？

A: `?` 是错误传播运算符。`expr?` 等价于：
```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
}
```

### Q5: 为什么 `trim()` 返回 `&str` 而不是 `String`？

A: `trim()` 返回的是原始字符串的"切片引用"，指向原始数据中不含空白字符的部分。
这是一种零拷贝优化 — 不需要分配新的堆内存。

### Q6: `parse::<u32>()` 中的 `::<u32>` 是什么？

A: 这是 turbofish 语法，用于在调用泛型函数时显式指定类型参数。
因为 `parse` 可以解析为多种类型（`u32`、`i32`、`f64` 等），
需要告诉编译器你想要哪种类型。也可以用类型标注：`let n: u32 = trimmed.parse()?;`

### Q7: 如何在 release 模式下隐藏调试输出？

A: `#[cfg(debug_assertions)]` 属性确保代码只在 debug 模式下编译。
运行 `cargo run --release` 时，`[调试] 秘密数字是: ...` 不会显示。

---

## 从 Python、C、C++ 迁移时值得注意的设计差异

### 1. 用 `match` 处理 `Result` 而非 `try/except`

Python 和 C++ 使用异常机制（`try/except`、`try/catch`）处理错误。Rust 没有异常，而是通过 `Result<T, E>` 枚举将错误作为返回值显式传递。在本项目中，`read_line`、`parse` 等操作都返回 `Result`，调用方用 `match` 穷尽匹配 `Ok` 和 `Err` 两个分支。与异常不同，编译器会强制检查你是否处理了错误路径——忘记处理 `Err` 分支是编译错误，而不是运行时才发现。C 语言用返回码（如 `-1`、`NULL`）表示错误，但返回值与正常数据混在同一通道里，容易遗漏检查。Rust 的 `Result` 在类型层面区分了成功和失败，`?` 运算符又提供了类似异常的便捷传播能力，兼顾了安全性和表达力。

### 2. 随机数生成器显式传入（无全局状态）

Python 的 `random.randint()` 依赖模块级全局状态，C 的 `rand()` 依赖进程级全局种子。这些隐式状态在多线程环境中会引发问题，也使得测试难以复现。Rust 的 `rand::thread_rng()` 虽然也是便捷入口，但它返回的是一个明确的随机数生成器实例。本项目将生成器封装在 `generate_secret()` 函数内，调用者不感知全局状态。如果你需要可复现的测试，可以显式创建带固定种子的 `StdRng` 实例并传入——这种"依赖注入"模式在 Rust 中很自然，因为所有权系统让传入和持有实例变得安全可控。

### 3. 用 `loop` 而非 `while True`

Python 中无限循环写 `while True:`，C 中写 `while(1)`。Rust 提供了专门的 `loop` 关键字。这不是语法糖——`loop` 向编译器明确表达了"这是无限循环，退出点由内部 break 决定"的意图。更重要的是，`loop` 可以作为表达式，通过 `break value` 返回一个值。本项目虽未使用这个特性，但它体现了 Rust 将控制流视为表达式而非语句的设计理念——这与 C 的"语句-表达式"二分法截然不同。

### 4. 类型标注在函数签名中的核心地位

Python 的类型注解是可选的后置检查（需配合 mypy 等工具），C 的类型声明在调用处可能因隐式转换而失去保护。Rust 中，类型是编译期强制检查的硬约束。本项目的 `parse_guess(input: &str) -> Result<u32, String>` 不仅声明了输入和输出类型，还通过 `&str` vs `String` 区分了"借用"和"拥有"。函数签名本身就是一份精确的契约——你能一眼看出该函数获取所有权还是仅借用，是否会失败。这在 Python/C 中要么不存在，要么依赖注释和惯例。

### 5. 编译期保证穷尽性：`match` 的威力超乎想象

C 的 `switch` 可以遗漏 `default`，Python 的 `if/elif/else` 链可以漏掉分支——这些在编译或运行时不产生任何警告。Rust 的 `match` 在匹配枚举（如 `Ordering::Less/Equal/Greater`、`Result::Ok/Err`）时，编译器会检查你是否穷尽了所有变体。遗漏一个变体是编译错误。这意味着当你将来为枚举新增变体时，编译器会立刻告诉你所有需要更新 `match` 的地方。这就是"将错误从运行时提前到编译时"的核心理念——本项目中多处 `match` 都受益于此。

---

## 项目信息

- **Rust Edition**: 2024
- **最低 Rust 版本**: 1.85.0（2024 edition 要求）
- **依赖**: rand 0.8
- **代码行数**: src/main.rs 约 250+ 行（含注释和文档）
- **创建日期**: 2026-06

---

*本项目是 Rust 学习路线中阶段 A 的综合练习，旨在通过一个完整的项目巩固变量、类型、控制流、所有权、错误处理等核心概念。*
