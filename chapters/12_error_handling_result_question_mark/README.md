# 第十二章：Rust 错误处理 —— Result、panic! 与 ? 运算符

> **核心术语**：Result, Ok, Err, panic!, unwrap, expect, ? operator, Error Propagation 错误传播, Custom Error 自定义错误  
> **对应 Python 概念**：exceptions, try/except, raise, finally

---

## 目录

1. [为什么 Rust 不使用异常](#1-为什么-rust-不使用异常)
2. [不可恢复错误：panic!](#2-不可恢复错误panic)
3. [可恢复错误：Result<T, E>](#3-可恢复错误resultt-e)
4. [使用 match 处理 Result](#4-使用-match-处理-result)
5. [unwrap() 与 expect()](#5-unwrap-与-expect)
6. [? 运算符](#6--运算符)
7. [错误传播模式](#7-错误传播模式)
8. [自定义错误类型](#8-自定义错误类型)
9. [From trait 与自动错误转换](#9-from-trait-与自动错误转换)
10. [main() 返回 Result](#10-main-返回-result)
11. [何时 panic 何时返回 Result](#11-何时-panic-何时返回-result)
12. [Python 对照表](#12-python-对照表)

---

## 1. 为什么 Rust 不使用异常

### 1.1 异常机制的代价

在 Python、Java、C++ 等语言中，异常（Exception / throw）是主要的错误处理手段。当一个函数遇到无法处理的情况，它"抛出"一个异常对象，调用栈向上展开（stack unwinding），直到某个 try/catch 块捕获它。

这种机制在理论上是优雅的，但在实践中存在几个问题：

| 问题 | 说明 |
|------|------|
| **隐式控制流** | 异常可以在任何函数调用点发生，阅读代码时无法从签名判断一个函数可能抛出什么异常 |
| **性能开销** | 栈展开需要维护大量的元数据（unwind tables），对二进制体积和运行时性能都有影响 |
| **资源安全** | 异常可能导致资源泄漏（Python 的 `with` / Java 的 `try-with-resources` 就是为了解决这个问题） |
| **错误被吞没** | `except Exception: pass` 是 Python 代码中常见的反模式，无声地掩盖了问题 |

### 1.2 Rust 的哲学：让错误成为类型系统的一部分

Rust 选择了一条不同的路径：**将错误编码到类型系统中**。

```rust
// Rust 中，可能失败的函数返回 Result，签名就说明了"我可能会失败"
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// 调用者 **必须** 处理这个 Result，编译器不会让你忽略它
let result = parse_number("42");  // result 的类型是 Result<i32, ParseIntError>
// 如果不处理，编译器会发出 warning: unused `Result` that must be used
```

```python
# Python 中，类似的函数签名看不出它会失败
def parse_number(s: str) -> int:
    return int(s)  # 可能抛出 ValueError —— 但从签名完全看不出来
```

**Rust 的错误处理设计目标：**

1. **显式性（Explicit）**：函数签名明确声明它可能失败
2. **强制性（Mandatory）**：编译器强制调用者处理错误
3. **零开销（Zero-cost）**：Result 的运行时表现和手写错误码一样高效
4. **可组合性（Composable）**：? 运算符让错误传播简洁而不失明确性

---

## 2. 不可恢复错误：panic!

### 2.1 什么是 panic!

`panic!` 是 Rust 处理**不可恢复错误**的机制。当程序遇到无法继续执行的状况时，它"恐慌"并终止。

```rust
fn main() {
    // 主动触发 panic
    panic!("程序遇到了无法恢复的错误！");

    // 程序永远不会执行到这里
    println!("这行不会打印");
}
```

### 2.2 常见的 panic 场景

#### 场景1：数组越界访问

```rust
let arr = [1, 2, 3];

// 编译期不会报错，但运行时会 panic
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'
let x = arr[5];
```

Rust 对数组访问会插入**运行时边界检查**（bounds check）。这是 Rust 安全保证的一部分：它不会像 C/C++ 那样默默越界访问然后导致缓冲区溢出漏洞。

#### 场景2：unwrap() 或 expect() 遇到 Err

```rust
let result: Result<i32, &str> = Err("失败了");
let value = result.unwrap();  // panic! —— "called `Result::unwrap()` on an `Err` value"
```

#### 场景3：整数溢出（debug 模式）

```rust
// debug 模式下会 panic，release 模式下会 wrap around（回绕）
let x: u8 = 255;
let y = x + 1;  // debug: panic!  release: y = 0
```

### 2.3 panic 的行为：栈展开 vs 直接终止

当 panic 发生时，Rust 默认执行**栈展开**（stack unwinding）：

1. 从 panic 发生点开始，逐帧释放栈上的资源（调用 `drop()`）
2. 释放完所有资源后终止当前线程
3. 如果是主线程，则终止进程

可以在 Cargo.toml 中配置为直接终止（abort），以减小二进制体积：

```toml
[profile.release]
panic = "abort"
```

### 2.4 查看 backtrace（调用栈回溯）

设置环境变量 `RUST_BACKTRACE=1` 可以查看 panic 发生时的完整调用栈：

```bash
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:10:5:
index out of bounds: the len is 3 but the index is 5
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_bounds_check
   3: error_handling::main
   ...
```

对于更详细的回溯（包括函数参数），可以使用 `RUST_BACKTRACE=full`。

### 2.5 catch_unwind —— 捕获 panic

Rust 允许使用 `std::panic::catch_unwind` 捕获 panic，但这很少用于正常的错误处理：

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    // 可能 panic 的代码
    let arr = [1, 2, 3];
    arr[100]
});

match result {
    Ok(_) => println!("没有 panic"),
    Err(panic_info) => {
        // panic 被捕获了，但通常这意味着程序状态可能已损坏
        println!("捕获到 panic: 不推荐在生产代码中这样做");
    }
}
```

**重要**：`catch_unwind` 主要用于测试（验证代码确实会 panic）和 FFI 边界（防止 panic 跨越语言边界）。它不应该作为常规的错误处理机制。

---

## 3. 可恢复错误：Result<T, E>

### 3.1 Result 的定义

`Result` 是 Rust 标准库中最重要的枚举类型之一：

```rust
// 标准库中的定义
pub enum Result<T, E> {
    Ok(T),   // 操作成功，包含返回值
    Err(E),  // 操作失败，包含错误信息
}
```

- `T` 是成功时返回的值的类型
- `E` 是失败时的错误类型

### 3.2 创建 Result

```rust
// 成功
let success: Result<i32, String> = Ok(42);

// 失败
let failure: Result<i32, String> = Err(String::from("计算失败"));

// 标准库函数返回 Result 的例子
use std::fs;
let content = fs::read_to_string("data.txt"); // Result<String, io::Error>
```

### 3.3 Result 的常用方法一览

| 方法 | 作用 | 签名简写 |
|------|------|----------|
| `is_ok()` | 判断是否为 Ok | `&self -> bool` |
| `is_err()` | 判断是否为 Err | `&self -> bool` |
| `unwrap()` | 提取 Ok 值，否则 panic | `self -> T` |
| `expect(msg)` | 提取 Ok 值，panic 时带消息 | `self -> T` |
| `unwrap_or(default)` | 提取 Ok 值，或使用默认值 | `self -> T` |
| `unwrap_or_else(f)` | 提取 Ok 值，或用闭包生成默认值 | `self -> T` |
| `map(f)` | 对 Ok 值应用函数 | `self -> Result<U, E>` |
| `map_err(f)` | 对 Err 值应用函数 | `self -> Result<T, F>` |
| `and_then(f)` | 链式调用（flatmap） | `self -> Result<U, E>` |
| `ok()` | 转换为 Option (Ok→Some, Err→None) | `self -> Option<T>` |
| `err()` | 转换为 Option (Err→Some, Ok→None) | `self -> Option<E>` |

### 3.4 Result 与 Option 的关系

```rust
// Option: 值可能存在，也可能不存在
// Result: 值可能有效，也可能是一个错误

// 互相转换
let opt: Option<i32> = Some(42);
let res: Result<i32, &str> = opt.ok_or("缺失值"); // Ok(42)

let res: Result<i32, &str> = Err("出错了");
let opt: Option<i32> = res.ok(); // None —— 错误信息被丢弃了
```

---

## 4. 使用 match 处理 Result

`match` 是处理 Result 最基础、最显式的方式。

### 4.1 基本用法

```rust
use std::fs::File;

let result = File::open("config.toml");

match result {
    Ok(file) => {
        println!("文件打开成功，大小: {:?} 字节", file.metadata().unwrap().len());
        // 使用 file ...
    }
    Err(error) => {
        // 可以进一步匹配具体的 I/O 错误类型
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("配置文件不存在，将使用默认配置");
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("没有权限读取配置文件");
            }
            other => {
                eprintln!("读取配置文件时发生未知错误: {}", other);
            }
        }
    }
}
```

### 4.2 嵌套匹配（"长写法"）

在没有 ? 运算符之前，Rust 程序员需要写嵌套的 match：

```rust
fn read_config() -> Result<Config, ConfigError> {
    let file = match File::open("config.toml") {
        Ok(f) => f,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let content = match read_to_string(file) {
        Ok(c) => c,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => return Err(ConfigError::ParseError(e)),
    };

    Ok(config)
}
```

这种写法虽然冗长，但**极其清晰**：每一步都显式声明了成功该做什么，失败该做什么。

---

## 5. unwrap() 与 expect()

### 5.1 定义对比

```rust
// unwrap: 提取值，失败时 panic 并打印 Debug 信息
let value = result.unwrap();

// expect: 提取值，失败时 panic 并带上自定义消息
let value = result.expect("配置文件解析失败，请检查 config.toml 格式");
```

### 5.2 何时可以使用

```
✅ 可以使用 unwrap/expect 的场景：

1. 原型开发（prototyping） —— 快速验证逻辑，暂时不想处理所有错误
2. 测试代码 —— 测试中的 assert 失败本质上也是一种 panic
3. 逻辑上不可能失败 —— 比如你先用 is_ok() 检查过了

   let result = some_function();
   if result.is_ok() {
       let value = result.unwrap(); // 安全，因为刚检查过
   }

4. 程序初始化 —— 如果配置文件/环境变量缺失，程序本来就不该启动

   let port = std::env::var("PORT").expect("必须设置 PORT 环境变量");
```

```
❌ 不应该使用 unwrap/expect 的场景：

1. 生产代码中的错误处理 —— 使用 ? 或 match
2. 库代码 —— 你不应该替调用者决定是否 panic
3. 网络请求、文件读取等外部 I/O —— 错误是常态，不是例外
```

### 5.3 expect() 优于 unwrap()

```rust
// unwrap: panic 信息没有上下文
// thread 'main' panicked at src/main.rs:42:24:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

// expect: panic 信息带着你的意图
// thread 'main' panicked at src/main.rs:42:24:
// 必须在生产环境提供 API 秘钥: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**永远优先使用 `expect()` 而不是 `unwrap()`**。当程序 crash 时，清晰的错误消息能让调试时间从小时缩短到分钟。

---

## 6. ? 运算符

### 6.1 ? 的基本语法

`?` 是 Rust 错误处理中最具标志性的语法。它放在一个 `Result` 值后面：

```rust
let content = fs::read_to_string("data.txt")?;
```

### 6.2 ? 的实际行为

? 运算符展开后等价于：

```rust
let content = match fs::read_to_string("data.txt") {
    Ok(text) => text,         // 成功：提取值，继续执行
    Err(e) => return Err(e.into()),  // 失败：立即从当前函数返回错误
};
```

关键细节：`Err(e)` 时调用的是 `e.into()`，而不是直接返回 `e`。这意味着：

- 如果函数的返回错误类型和 `e` 的类型不同，`?` 会尝试用 `From` trait 自动转换
- 这就是为什么实现 `From<io::Error> for MyError` 后，`?` 能自动把 `io::Error` 转成 `MyError`

### 6.3 ? 运算符的威力：对比

**手动 match（长写法）：**

```rust
fn run_analysis(path: &str) -> Result<(), FileStatsError> {
    let content = match read_file_content(path) {
        Ok(text) => text,
        Err(e) => return Err(FileStatsError::IoError(e)),
    };

    let numbers = match parse_numbers(&content) {
        Ok(nums) => nums,
        Err(msg) => return Err(FileStatsError::ParseError(msg)),
    };

    if numbers.is_empty() {
        return Err(FileStatsError::EmptyInput);
    }

    let stats = compute_stats(&numbers);
    println!("{}", stats);

    Ok(())
}
```

**? 运算符（短写法）：**

```rust
fn run_analysis(path: &str) -> Result<(), FileStatsError> {
    let content = read_file_content(path)?;              // 自动 .into() 转换
    let numbers = parse_numbers(&content)
        .map_err(FileStatsError::ParseError)?;           // 需要手动转换的情况

    if numbers.is_empty() {
        return Err(FileStatsError::EmptyInput);
    }

    let stats = compute_stats(&numbers);
    println!("{}", stats);

    Ok(())
}
```

代码量减少了一半，可读性提高了一倍。

### 6.4 ? 能用于哪些类型

? 运算符可以用于任何实现了 `Try` trait 的类型。目前支持：

| 类型 | 用途 |
|------|------|
| `Result<T, E>` | 标准错误处理 |
| `Option<T>` | 值可能不存在 |
| `ControlFlow` | 控制流（内部使用） |
| `Poll` | 异步轮询 |

```rust
// Option 上的 ? 用法
fn get_username(id: u32) -> Option<String> {
    let user = database.find(id)?;      // 如果 None，直接返回 None
    Some(user.name)
}
```

### 6.5 ? 运算符的限制

**? 只能在返回 `Result` 或 `Option` 的函数中使用：**

```rust
// ❌ 错误：main 返回 ()，不能使用 ?
fn main() {
    let content = fs::read_to_string("data.txt")?;  // 编译错误！
}

// ✅ 正确：main 返回 Result
fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("data.txt")?;
    Ok(())
}

// ✅ 正确：在闭包中处理
fn main() {
    let result = (|| -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("data.txt")?;
        Ok(())
    })();

    match result {
        Ok(()) => {}
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

---

## 7. 错误传播模式

### 7.1 模式一：直接传播（? 运算符）

最简单的模式 —— 让错误沿着调用栈向上冒泡：

```rust
fn step1() -> Result<String, MyError> {
    let data = fs::read_to_string("input.txt")?;  // io::Error 自动转为 MyError
    Ok(data)
}

fn step2(data: &str) -> Result<Config, MyError> {
    let config: Config = serde_json::from_str(data)?;  // serde::Error 自动转为 MyError
    Ok(config)
}

fn run() -> Result<(), MyError> {
    let data = step1()?;
    let config = step2(&data)?;
    println!("{:?}", config);
    Ok(())
}
```

### 7.2 模式二：转换后传播（map_err + ?）

当错误类型不兼容且没有 From 实现时：

```rust
fn parse_config(file: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(file)?;  // From<io::Error> 已实现

    let config: Value = serde_json::from_str(&content)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;  // 手动转换

    Ok(config.into())
}
```

### 7.3 模式三：错误降级（提供默认值）

有时候错误不需要传播，可以用默认值替代：

```rust
fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())   // 环境变量不存在，用默认值
        .parse()
        .unwrap_or(8080)                           // 解析失败，用硬编码默认值
}
```

### 7.4 模式四：收集多个错误

```rust
fn batch_process(files: &[&str]) -> Vec<Result<String, io::Error>> {
    files.iter()
        .map(|f| fs::read_to_string(f))
        .collect()
}

// 或者：只保留成功的
fn process_successful(files: &[&str]) -> Vec<String> {
    files.iter()
        .filter_map(|f| fs::read_to_string(f).ok())
        .collect()
}
```

### 7.5 模式五：anyhow / thiserror（生态系统）

在实际项目中，社区推荐使用两个 crate 简化错误处理：

```rust
// thiserror: 方便定义自定义错误类型（库代码）
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyLibError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),       // #[from] 自动生成 From 实现

    #[error("Parse error on line {line}: {msg}")]
    Parse { line: usize, msg: String },

    #[error("Empty input")]
    EmptyInput,
}

// anyhow: 方便在应用代码中处理各种错误
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("config.toml")
        .context("无法读取配置文件")?;  // context 会给错误增加描述信息

    let config: Config = toml::from_str(&content)
        .context("配置文件格式错误")?;

    Ok(())
}
```

---

## 8. 自定义错误类型

### 8.1 为什么需要自定义错误类型

```rust
// ❌ 糟糕的做法：到处用 String 做错误类型
fn step1() -> Result<Data, String> { ... }
fn step2(data: &Data) -> Result<Output, String> { ... }

// 问题：
// 1. String 不实现 Error trait，无法向上传播
// 2. 调用者无法 match 不同的错误场景
// 3. String 的分配是有成本的
```

```rust
// ✅ 好的做法：用 enum 定义领域错误
#[derive(Debug)]
enum DataPipelineError {
    IoError(std::io::Error),
    ParseError { line: usize, expected: String },
    ValidationError(String),
    TimeoutError(std::time::Duration),
}
```

### 8.2 定义一个完整的自定义错误类型

```rust
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(String),
    NotFound,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "I/O 错误: {}", e),
            MyError::Parse(msg) => write!(f, "解析错误: {}", msg),
            MyError::NotFound => write!(f, "未找到所需资源"),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),     // 保留底层 io::Error
            MyError::Parse(_) => None,
            MyError::NotFound => None,
        }
    }
}

// From trait 实现：让 ? 自动转换 io::Error
impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}
```

### 8.3 错误设计的指导原则

1. **使用 enum 而不是结构体** —— 错误通常是"非此即彼"的
2. **保留底层错误** —— 通过 `source()` 暴露，方便调试
3. **实现 Display** —— 给用户友好的错误信息
4. **实现 From** —— 让 ? 运算符能够自动转换
5. **保持扁平** —— 不要嵌套太深的错误层级

---

## 9. From trait 与自动错误转换

### 9.1 From trait 的作用

`From<T>` trait 定义了如何从类型 `T` 转换为类型 `Self`：

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}
```

在错误处理中的关键用法：当你为自定义错误类型实现了 `From<io::Error>`，? 运算符就会自动调用它。

### 9.2 转换流水线

```rust
// 假设我们有这些类型：
//   io::Error          (标准库)
//   serde_json::Error   (第三方 crate)
//   MyError             (自定义)

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self { MyError::Io(e) }
}

impl From<serde_json::Error> for MyError {
    fn from(e: serde_json::Error) -> Self { MyError::Parse(e.to_string()) }
}

// 现在 ? 可以自动处理两种外部错误：
fn process() -> Result<(), MyError> {
    let content = fs::read_to_string("data.json")?;      // io::Error -> MyError
    let data: Value = serde_json::from_str(&content)?;   // serde_json::Error -> MyError
    Ok(())
}
```

### 9.3 什么时候用 From，什么时候用 map_err

```rust
// 用 From: 当转换是"全局性的"，多次出现，逻辑固定
// 用 map_err: 当转换需要上下文（比如行号、文件名），或是一次性的
fn parse_line(line: &str, line_num: usize) -> Result<i32, MyError> {
    line.trim().parse::<i32>()
        .map_err(|e| MyError::ParseError(format!("第 {} 行: {}", line_num, e)))
        // 这里不能用 From，因为需要 line_num 上下文
}
```

---

## 10. main() 返回 Result

### 10.1 从 Rust 2018 版开始，main 可以返回 Result

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("data.txt")?;
    println!("{}", content);
    Ok(())
}

// 如果 main 返回 Err，程序会：
// 1. 打印 Error 的 Debug 信息
// 2. 以非零退出码终止
```

`Box<dyn Error>` 是一个能装任何错误类型的"万能容器"，适合在 main 中使用，因为 main 通常不需要区分具体的错误类型。

### 10.2 更好的 main 错误处理

```rust
fn main() {
    // 方式1: 在 main 内部匹配
    if let Err(e) = run() {
        eprintln!("应用程序错误: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    // 所有逻辑
    Ok(())
}
```

把逻辑放在 `run()` 中，`main()` 只负责错误报告，是一种好的模式：

- `run()` 可以返回具体的错误类型
- `main()` 统一处理错误展示和退出码
- 便于测试 —— 可以直接测试 `run()` 函数

---

## 11. 何时 panic 何时返回 Result

这是 Rust 编程中最需要判断力的问题之一。

### 11.1 应该 panic 的场景

```rust
// 1. 内部不变量被破坏 —— 意味着代码有 bug
let index = calculate_index();
let value = vec[index];  // 如果 index 真的可能越界，代码逻辑就有错误

// 2. 程序初始化失败 —— 无法恢复
let config = load_config().expect("配置文件必须在启动时提供");

// 3. 测试代码
#[test]
fn test_parser() {
    let result = parse("input").unwrap();  // 测试中 unwrap 是合理的
    assert_eq!(result, expected);
}

// 4. 对外部输入的防御 —— 当输入违反约定时
fn set_port(port: u16) {
    if port == 0 {
        panic!("端口号不能为 0");  // 这是调用者的错误
    }
}
```

### 11.2 应该返回 Result 的场景

```rust
// 1. I/O 操作 —— 文件、网络等，错误是正常情况
fn read_config(path: &str) -> Result<Config, io::Error>;

// 2. 用户输入解析 —— 输入可能格式错误
fn parse_query(input: &str) -> Result<Query, ParseError>;

// 3. API 调用 —— 远端可能不可达
fn fetch_data(url: &str) -> Result<Data, HttpError>;

// 4. 库代码 —— 永远不要让库代码 panic，把决定权交给调用者
```

### 11.3 判定原则

> **一个好用的经验法则：如果错误可能由"外部因素"导致（文件不存在、网络断连、用户输入非法），返回 Result。如果错误只能在"代码有 bug"时发生（内部不变量被破坏、错误的 API 使用），可以 panic。**

---

## 12. Python 对照表

### 12.1 核心概念对照

| Rust | Python | 说明 |
|------|--------|------|
| `Result<T, E>` | `try/except` 机制 | 可恢复错误的处理框架 |
| `Ok(value)` | `return value` | 正常返回 |
| `Err(e)` | `raise Exception(...)` | 返回/抛出错误 |
| `?` 运算符 | `raise`（在 except 中） | 传播错误给调用者 |
| `panic!` | `sys.exit()` / 未捕获异常 | 不可恢复错误 |
| `match result {}` | `try: ... except X: ...` | 模式匹配处理错误 |
| `unwrap()` | 无直接对应（类似不写 try） | 粗暴提取值 |
| `expect(msg)` | `assert condition, msg` | 带消息的断言式提取 |
| `unwrap_or(x)` | `result or x` / `dict.get(key, x)` | 提供默认值 |
| `map_err(f)` | `except X as e: raise Y from e` | 错误类型转换 |
| `From<E>` trait | 隐式异常链（Python 3 的 `from`） | 自动类型转换 |
| 自定义错误 enum | 自定义 Exception 子类 | 领域错误定义 |
| `main() -> Result<()>` | `if __name__ == "__main__": try: ... except:` | 应用入口错误处理 |

### 12.2 代码对比

#### Rust 的错误处理（显式）

```rust
use std::fs;
use std::io;

fn read_numbers(path: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;  // io::Error 自动传播

    let numbers: Vec<i32> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse::<i32>())     // parse::<i32> 返回 Result
        .collect::<Result<Vec<_>, _>>()?;     // 收集所有结果，有错就传播

    if numbers.is_empty() {
        return Err("文件中没有有效的数字".into());
    }

    Ok(numbers)
}

fn main() {
    match read_numbers("data.txt") {
        Ok(nums) => println!("读取到 {} 个数字: {:?}", nums.len(), nums),
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

#### Python 的异常处理（隐式）

```python
def read_numbers(path: str) -> list[int]:
    """读取文件中的数字列表。

    可能抛出: FileNotFoundError, ValueError, PermissionError 等
    —— 但从函数签名完全看不出来。
    """
    try:
        with open(path) as f:
            content = f.read()
    except FileNotFoundError:
        raise  # 或记录日志后重新抛出

    numbers = []
    for line in content.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            numbers.append(int(line))
        except ValueError:
            raise ValueError(f"第 {i} 行不是有效的数字: {line!r}")

    if not numbers:
        raise ValueError("文件中没有有效的数字")

    return numbers


if __name__ == "__main__":
    try:
        nums = read_numbers("data.txt")
        print(f"读取到 {len(nums)} 个数字: {nums}")
    except Exception as e:
        print(f"错误: {e}")
```

### 12.3 关键区别总结

```
+--------------------------------------+--------------------------------------+
|           Rust (Result)              |         Python (Exceptions)          |
+--------------------------------------+--------------------------------------+
| 错误是类型系统的一部分                | 错误是运行时控制流的一部分            |
| 编译器强制处理 Result                 | 可以忽略异常直到运行时崩溃            |
| 函数签名声明可能失败                  | 函数签名看不出可能抛出什么            |
| ? 运算符显式标注每个传播点            | raise 可以发生在任何地方，没有标记    |
| 零运行时开销（编译为分支判断）        | 异常抛出有栈展开开销                  |
| 没有 try/finally，用 RAII + Drop     | try/finally 和 context manager       |
| 自定义错误 = enum + Display + Error   | 自定义异常 = class MyException        |
| Box<dyn Error> 万能容器               | except Exception 万能捕获             |
+--------------------------------------+--------------------------------------+
```

### 12.4 从 Python 迁移到 Rust 的思维转变

**Python 程序员的习惯：**

```python
# 习惯1: "先跑起来，出错再说"
result = do_something()  # 可能会抛异常，但我不写 try

# 习惯2: "万能捕获"
try:
    data = complex_operation()
except Exception:
    pass  # 吞掉所有错误

# 习惯3: "异常即控制流"
try:
    value = dict[key]
except KeyError:
    value = default
```

**Rust 程序员的做法：**

```rust
// 习惯1: "先想清楚失败怎么办"
let result: Result<Data, Error> = do_something(); // 编译器逼我处理它

// 习惯2: "精确匹配错误"
match complex_operation() {
    Ok(data) => use_data(data),
    Err(MyError::NotFound) => create_default(),
    Err(MyError::PermissionDenied) => log_and_exit(),
    Err(e) => return Err(e), // 未知错误向上传播
}

// 习惯3: "用 Option / Result 的方法"
let value = map.get(&key).ok_or(MissingKey)?;
// 或直接用 unwrap_or
let value = map.get(&key).unwrap_or(&default);
```

---

## 本章总结

Rust 的错误处理哲学可以用三句话概括：

1. **不可恢复的**用 `panic!` —— 程序无法继续，栈展开，释放资源，终止
2. **可恢复的**用 `Result<T, E>` —— 编译器强制处理，没有"意外"的错误
3. **传播错误**用 `?` —— 简洁而不失可见性，每个 `?` 都是一个潜在的返回点

记住：**Rust 没有异常，但它的错误处理比任何异常机制都更安全、更清晰。** 因为每一个可能的失败都在类型系统中有迹可循，编译器确保你不会漏掉任何一个。

---

## 运行本程序

```bash
# 编译
cargo build

# 运行
cargo run

# 查看 panic backtrace
RUST_BACKTRACE=1 cargo run

# 发布模式构建
cargo build --release
```

## 参考资料

- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Rust API Docs - std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)
- [The thiserror crate](https://docs.rs/thiserror)
- [The anyhow crate](https://docs.rs/anyhow)
