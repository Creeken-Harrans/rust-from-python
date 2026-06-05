# 第十二章练习题答案 — 错误处理、Result 与 ? 运算符

---

## Level 1：基础练习

---

### 练习 1-1：读取文件并统计行数

**结论**

使用 `Result<usize, std::io::Error>` 将 I/O 操作的失败可能性编码进类型签名。调用方通过 `match` 穷尽处理 `Ok` 和 `Err` 两个分支，编译器强制检查错误路径——这从根本上消除了"忘记检查返回值"的 bug。`panic!` 在这里**不适用**，因为文件不存在是"可恢复错误"而非程序 bug；`Option` 也不适用，因为我们需要知道失败的原因（`io::Error` 携带 errno 等信息），而不仅仅是"有/没有"。

**思路**

1. `std::fs::read_to_string(path)` 返回 `Result<String, io::Error>`
2. 对该 `Result` 进行 `match`：`Ok(content)` 分支使用 `.lines().count()` 统计行数并包装为 `Ok(lines)`；`Err(e)` 分支将错误原样传播为 `Err(e)`
3. 在 `main()` 中对函数返回值再次 `match`，分别打印行数或错误信息

**参考实现**

```rust
use std::fs;
use std::io;

fn count_lines(path: &str) -> Result<usize, io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content.lines().count()),
        Err(e) => Err(e),
    }
}

fn main() {
    // 存在的文件
    match count_lines("data.txt") {
        Ok(n) => println!("文件 data.txt 共有 {} 行", n),
        Err(e) => println!("错误: {}", e),
    }

    // 不存在的文件
    match count_lines("/tmp/nonexistent_file_xyz.txt") {
        Ok(n) => println!("文件共有 {} 行", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 用 `unwrap()` 代替 `match` | `unwrap()` 在 `Err` 时触发 `panic!`，把可恢复错误变成了不可恢复的崩溃。原型阶段可用，生产代码不可接受 |
| 把函数签名写成 `-> usize` 然后在内部 `unwrap()` | 错误信息被吞没，调用方无法得知操作失败。这违反了 Rust 将失败编码为类型的哲学 |
| 忘记 `.lines()` 会计入文件末尾的空行 | 如果文件末尾有换行符，`.lines()` 会多计一行。可考虑用 `.lines().filter(|l| !l.is_empty()).count()` 或直接用原始字节统计 `\n` |
| 混淆 `Result` 和 `Option` | `Option` 只能表达"有/无"，丢失了"为什么失败"的信息。I/O 操作应始终使用 `Result` |

**验证方式**

```bash
# 创建测试文件
echo -e "line1\nline2\nline3" > data.txt

# 运行程序
cargo run

# 预期输出:
# 文件 data.txt 共有 3 行
# 错误: No such file or directory (os error 2)
```

---

### 练习 1-2：使用 unwrap_or 提供默认值

**结论**

本题的核心是区分三种处理 `Result` 的方式，以及它们与 `panic!`/`Option` 的关系：

- **`unwrap()`**：`Err` 时触发 `panic!`——用于"程序员的错误"，即逻辑上不可能失败的场景（如刚插入的 key 必定存在）
- **`unwrap_or(default)`**：`Err` 时用 `default` 替代——默认值计算简单且已就绪
- **`unwrap_or_else(closure)`**：`Err` 时惰性计算默认值——默认值需要运行时信息或计算昂贵
- **`expect(msg)`**：`Err` 时 `panic!` 并带自定义消息——用于"必须存在否则程序无法继续"的场景（如 API_KEY）

`Option::unwrap` 和 `Result::unwrap` 都会触发 `panic!`，但 `Option` 不携带错误信息，`Result` 携带。本题使用的环境变量场景天然适合 `Result`，因为 `std::env::var` 返回 `Result<String, VarError>`——我们能区分"变量未设置"和"变量含非 UTF-8 字符"。

**思路**

1. `get_config_value` 直接委托给 `std::env::var(key)`
2. 实验 1：`unwrap_or` 提供一个已计算好的字符串常量作为默认值
3. 实验 2：`unwrap_or_else` 的闭包只在 `Err` 时执行，适合需要进程 ID 等动态信息的场景
4. 实验 3：`expect` 在 API_KEY 不存在时 panic，给出明确的错误信息指导运维

**参考实现**

```rust
use std::env;
use std::env::VarError;

fn get_config_value(key: &str) -> Result<String, VarError> {
    env::var(key)
}

fn main() {
    // 实验 1: unwrap_or -- 环境变量不存在时用默认值
    let db_url = get_config_value("DATABASE_URL")
        .unwrap_or("postgres://localhost:5432/mydb".to_string());
    println!("DATABASE_URL = {}", db_url);

    // 实验 2: unwrap_or_else -- 默认值需要运行时计算
    let log_path = get_config_value("LOG_PATH")
        .unwrap_or_else(|_| format!("/var/log/{}.log", std::process::id()));
    println!("LOG_PATH = {}", log_path);

    // 实验 3: expect -- 这个变量必须存在
    let api_key = get_config_value("API_KEY")
        .expect("API_KEY 环境变量必须设置！请在启动前 export API_KEY=<your_key>");
    println!("API_KEY = {}", api_key);
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 对所有缺失的环境变量使用 `expect` | 不是所有配置都"必须存在"。DB_URL 可以硬编码开发默认值，API_KEY 才需要用 `expect` 强力要求 |
| `unwrap_or` 的参数在调用前就被求值 | `unwrap_or(format!(...))` 中的 `format!` 无论 `Result` 是 Ok 还是 Err 都会执行。应使用 `unwrap_or_else` |
| 混淆 `Option::unwrap_or` 和 `Result::unwrap_or` | 两者 API 类似，但 `Result` 的 `unwrap_or` 返回的是 `T`（丢弃了 `E`），`Option` 的 `unwrap_or` 返回的是 `T`。语义相同但错误信息丢失层面不同 |
| 认为 `expect` 和 `unwrap` 没有区别 | `expect` 的字符串是给人看的诊断信息，在生产环境排查问题时远远优于 `unwrap` 的默认消息 "called `Result::unwrap()` on an `Err` value" |

**验证方式**

```bash
# 不设环境变量，观察 expect 触发 panic
cargo run
# 预期: panicked at 'API_KEY 环境变量必须设置！...'

# 设置部分环境变量
DATABASE_URL="mysql://localhost/test" cargo run
# 预期: DATABASE_URL = mysql://localhost/test
#        LOG_PATH = /var/log/<pid>.log
#        (panic on API_KEY)
```

---

### 练习 1-3：实现自定义 Error 类型

**结论**

自定义错误类型的核心价值在于让调用方能**按错误种类做不同处理**。如果只使用 `Box<dyn Error>` 或字符串，调用方只能"打印或传播"，无法区分"重试可行"和"重试徒劳"。`Display` 提供面向用户的消息，`Error` trait 使其可与生态兼容（如 `?` 运算符的 `From` 转换链）。

`Option` 在此场景不适用——我们需要区分"空行"和"格式错误"，这是两种不同的失败原因，而非简单的"有/无"。

**思路**

1. 定义 `ConfigError` 枚举，两个变体各携带上下文字符串
2. 实现 `Display`：`NotFound` 显示"配置不存在"，`InvalidFormat` 显示"格式错误"并附带原文
3. 实现 `Error` trait：最小实现只需 `fn source(&self) -> Option<&(dyn Error + 'static)> { None }`（Rust 2018 之后也可用 `{}` 空体，因为 `Error` 的所有方法都有默认实现）
4. `parse_config_line`：先 `trim()`，空则 `NotFound`；无 `=` 则 `InvalidFormat`；否则按 `=` 分割为 key/value

**参考实现**

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum ConfigError {
    NotFound(String),
    InvalidFormat(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(msg) => write!(f, "配置不存在: {}", msg),
            ConfigError::InvalidFormat(msg) => write!(f, "格式错误: {}", msg),
        }
    }
}

impl Error for ConfigError {}

fn parse_config_line(line: &str) -> Result<(String, String), ConfigError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::NotFound("空行".to_string()));
    }
    match trimmed.split_once('=') {
        Some((key, value)) => Ok((key.trim().to_string(), value.trim().to_string())),
        None => Err(ConfigError::InvalidFormat(format!(
            "缺少 '=': {:?}",
            trimmed
        ))),
    }
}

fn main() {
    let test_cases = vec![
        ("host=localhost", "正常键值对"),
        ("", "空行"),
        ("invalid_line", "缺少等号"),
    ];

    for (input, _desc) in test_cases {
        match parse_config_line(input) {
            Ok((key, value)) => println!("解析 {:?}: ✅ {} = {}", input, key, value),
            Err(e) => println!("解析 {:?}: ❌ {}", input, e),
        }
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 用 `String` 作为错误类型 | `Result<T, String>` 丢失了错误类型的区分能力，match 时只能比对字符串内容 |
| 忘记实现 `Error` trait | `ConfigError` 无法与 `Box<dyn Error>` 和 `?` 的自动转换链兼容 |
| `Display` 消息太技术化 | 应面向用户（运维人员），而非面向开发者（如不要输出 Rust 调试格式） |
| 用 `split('=')` 而不是 `split_once('=')` | 如果值中包含 `=`（如 `url=http://example.com?a=1`），`split` 会错误地分割成多段。`split_once` 只在第一个 `=` 处分割 |
| 忘记对 key/value 做 `trim()` | `" host = localhost "` 解析出的 key 会是 `" host "` 而非 `"host"` |

**验证方式**

```bash
cargo run
# 预期输出:
# 解析 "host=localhost": ✅ host = localhost
# 解析 "": ❌ 配置不存在: 空行
# 解析 "invalid_line": ❌ 格式错误: 缺少 '=': "invalid_line"
```

---

## Level 2：综合练习

---

### 练习 2-1：安全的数字文件处理器

**结论**

本练习综合运用 `?` 运算符、`map_err`、命令行参数解析和多种错误变体。核心设计原则：
- `?` 用于错误传播——遇到错误立即从当前函数返回，减少样板代码
- `.map_err()` 用于错误类型转换——将底层错误（如 `ParseIntError`）转换为领域错误（`FileStatsError`）
- `panic!` 仅用于不可恢复的场景（如断言逻辑矛盾），不应出现在 I/O 或解析路径中
- `Option` 出现在 `nth(1)` 的返回值——命令行参数可能不存在，用 `unwrap_or` 提供默认值

**思路**

1. 用 `std::env::args().nth(1)` 获取第一个命令行参数，`unwrap_or` 提供默认路径
2. 扩展 `FileStatsError` 枚举，添加 `FileNotFound` 变体
3. `verify_file_exists` 用 `Path::new(path).exists()` 检查，不存在则返回 `Err`
4. `parse_numbers` 尝试先解析 `i32`，失败则尝试 `f64` 并四舍五入
5. 增强统计输出：排序后的前 5 / 后 5、正数/负数/零计数

**参考实现**

```rust
use std::env;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use std::fmt;

#[derive(Debug)]
enum FileStatsError {
    IoError(io::Error),
    ParseError(String),
    FileNotFound(String),
}

impl fmt::Display for FileStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileStatsError::IoError(e) => write!(f, "IO 错误: {}", e),
            FileStatsError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            FileStatsError::FileNotFound(path) => write!(f, "文件不存在: {}", path),
        }
    }
}

impl std::error::Error for FileStatsError {}

impl From<io::Error> for FileStatsError {
    fn from(e: io::Error) -> Self {
        FileStatsError::IoError(e)
    }
}

impl From<ParseIntError> for FileStatsError {
    fn from(_e: ParseIntError) -> Self {
        // 注意：ParseIntError 不包含原始字符串，所以这里信息有限
        // 通常在 parse_numbers 中捕获并附加上下文
        FileStatsError::ParseError("数字解析失败".to_string())
    }
}

fn verify_file_exists(path: &str) -> Result<(), FileStatsError> {
    if !Path::new(path).exists() {
        Err(FileStatsError::FileNotFound(path.to_string()))
    } else {
        Ok(())
    }
}

fn parse_numbers(content: &str) -> Result<Vec<i32>, FileStatsError> {
    let mut numbers = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // 先尝试 i32
        match line.parse::<i32>() {
            Ok(n) => numbers.push(n),
            Err(_) => {
                // 尝试 f64 并四舍五入
                match line.parse::<f64>() {
                    Ok(f) => {
                        let rounded = f.round() as i32;
                        println!("⚠ 警告: 第 {} 行 \"{}\" 是浮点数，四舍五入为 {}", i + 1, line, rounded);
                        numbers.push(rounded);
                    }
                    Err(_) => {
                        return Err(FileStatsError::ParseError(format!(
                            "第 {} 行不是有效数字: \"{}\"",
                            i + 1,
                            line
                        )));
                    }
                }
            }
        }
    }
    Ok(numbers)
}

fn print_stats(numbers: &[i32]) {
    println!("共 {} 个数字", numbers.len());
    let mut sorted = numbers.to_vec();
    sorted.sort();

    // 前 5 个
    print!("前 5 个: ");
    for n in sorted.iter().take(5) {
        print!("{} ", n);
    }
    println!();

    // 后 5 个
    print!("后 5 个: ");
    let len = sorted.len();
    let start = if len > 5 { len - 5 } else { 0 };
    for n in sorted.iter().skip(start) {
        print!("{} ", n);
    }
    println!();

    // 正数/负数/零
    let positive = numbers.iter().filter(|&&n| n > 0).count();
    let negative = numbers.iter().filter(|&&n| n < 0).count();
    let zeros = numbers.iter().filter(|&&n| n == 0).count();
    println!("正数: {} | 负数: {} | 零: {}", positive, negative, zeros);
}

fn run_analysis(file_path: &str) -> Result<(), FileStatsError> {
    // 使用 ? 传播错误
    verify_file_exists(file_path)?;
    let content = fs::read_to_string(file_path)?; // io::Error 通过 From 自动转换为 FileStatsError
    let numbers = parse_numbers(&content)?;
    print_stats(&numbers);
    Ok(())
}

fn main() {
    let default_path = "/tmp/rust_error_demo_numbers.txt";
    let file_path = env::args().nth(1).unwrap_or(default_path.to_string());

    // 如果使用默认路径，先创建示例文件
    if file_path == default_path {
        let sample = "10\n-5\n0\n42\n3.7\n100\n-20\n7\n15\n-99\n";
        fs::write(&file_path, sample).expect("无法创建示例文件");
        println!("已创建示例文件: {}", file_path);
    }

    match run_analysis(&file_path) {
        Ok(()) => println!("✅ 分析完成"),
        Err(e) => println!("❌ {}", e),
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 全用 `match` 而不用 `?` | `?` 正是为错误传播设计的，全用 `match` 会产生大量嵌套样板代码 |
| 忘记 `map_err` 转换错误类型 | 当函数内部有多种错误类型时，需要用 `.map_err()` 或 `From` trait 统一类型 |
| 在创建示例文件时用 `unwrap()` 而非 `?` | 题目要求使用 `?`。`main` 中 `fs::write(...).expect(...)` 可接受，但函数内部的 I/O 应使用 `?` |
| `parse_numbers` 中忽略浮点数 | 题目明确要求支持浮点数并四舍五入 + 警告 |
| 统计信息遗漏零 | 零既不是正数也不是负数，需要单独计数 |

**验证方式**

```bash
# 使用默认文件
cargo run

# 使用自定义文件
echo -e "1\n2\n3.14\n-10\n0" > /tmp/my_numbers.txt
cargo run -- /tmp/my_numbers.txt

# 使用不存在的文件测试错误
cargo run -- /tmp/nonexistent.txt
```

---

### 练习 2-2：Result 的链式操作

**结论**

链式组合子（`and_then`、`map`、`map_err`、`or_else`）和 `?` 运算符都能实现错误传播，但适用场景不同：
- **链式组合子**：适合数据变换管道，每一步是对上一步结果的纯函数转换，不需要提前返回到调用方
- **`?` 运算符**：适合在函数体内混合"有副作用的操作"和"错误传播"，更接近命令式编程风格
- 本题对比两者：链式版本需要更多函数式编程思维，`?` 版本行数更少且更直观

`Option` 在此场景也可实现类似管道（如 `Option::and_then`），但缺乏错误信息。用 `Result` 可以携带具体的失败原因。

**思路**

1. 从 `Ok(trimmed)` 开始管道
2. `and_then` 检查非空
3. `and_then` 尝试解析 i32，用 `map_err` 将 `ParseIntError` 转换为字符串错误
4. `and_then` 检查范围 `[1, 100]`

用 `?` 重写时，每个步骤直接返回 `Err`，代码结构更扁平。

**参考实现**

```rust
fn process_pipeline(input: &str) -> Result<i32, String> {
    let trimmed = input.trim().to_string();

    Ok(trimmed)
        .and_then(|s| {
            if s.is_empty() {
                Err("输入为空".to_string())
            } else {
                Ok(s)
            }
        })
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| format!("不是有效数字: {}", s))
        })
        .and_then(|n| {
            if n >= 1 && n <= 100 {
                Ok(n)
            } else {
                Err(format!("数字 {} 不在范围 [1, 100] 内", n))
            }
        })
}

// 用 ? 重写的对比版本
fn process_pipeline_with_question_mark(input: &str) -> Result<i32, String> {
    let s = input.trim().to_string();
    if s.is_empty() {
        return Err("输入为空".to_string());
    }
    let n: i32 = s.parse().map_err(|_| format!("不是有效数字: {}", s))?;
    if n < 1 || n > 100 {
        return Err(format!("数字 {} 不在范围 [1, 100] 内", n));
    }
    Ok(n)
}

fn main() {
    let test_cases = vec!["42", " 99 ", "", "abc", "0", "200"];
    for input in test_cases {
        let result = process_pipeline(input);
        println!("process_pipeline({:?}) = {:?}", input, result);
    }

    println!("\n--- 用 ? 运算符版本对比 ---");
    for input in test_cases {
        let result = process_pipeline_with_question_mark(input);
        println!("? 版本({:?}) = {:?}", input, result);
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 链式调用中混用 `?` | 题目要求**不**使用 `?`。应全部使用 `and_then`、`map`、`map_err` 等组合子 |
| `and_then` 与 `map` 混淆 | `map` 对 `Ok` 值做转换并保持 `Ok`（不引入新错误），`and_then` 可以返回新的 `Result`（可能变成 `Err`） |
| `or_else` 与 `map_err` 混淆 | `or_else` 处理 `Err` 并可能将其恢复为 `Ok`（如重试逻辑），`map_err` 仅转换 `Err` 的类型 |
| 检测空字符串时忘记 trim | 输入 `"   "` 应该被视为空，需要在 trim 后再检查 |

**验证方式**

```bash
cargo run
# 预期输出:
# process_pipeline("42") = Ok(42)
# process_pipeline(" 99 ") = Ok(99)
# process_pipeline("") = Err("输入为空")
# process_pipeline("abc") = Err("不是有效数字: abc")
# process_pipeline("0") = Err("数字 0 不在范围 [1, 100] 内")
# process_pipeline("200") = Err("数字 200 不在范围 [1, 100] 内")
```

---

## Level 3：高级挑战

---

### 练习 3-1：构建 Result 迭代器适配器

**结论**

`collect::<Result<Vec<T>, E>>()` 是 Rust 迭代器与错误处理集成的强大模式：迭代器遇到第一个 `Err` 时短路停止收集，直接返回该错误。这实现了"要么全部成功要么整体失败"的语义，无需手动循环和提前返回。

`From` trait 让 `?` 运算符能在不同的错误类型间自动转换——编译器在传播错误时查找 `From<E> for YourError` 实现并自动调用。

`Option` 的 `collect` 模式类似但语义不同：`Option<Vec<T>>` 表示"可能缺少整个集合"，而 `Result<Vec<T>, E>` 表示"要么有整个集合，要么处理过程中出了错误"。

**思路**

1. 为 `FileStatsError` 添加 `JsonError(String)` 变体
2. 实现 `From<ParseIntError> for FileStatsError`，让 `?` 自动转换
3. `batch_parse_to_json`：读取文件 -> 每行 parse -> `collect::<Result<Vec<i32>, _>>()` -> 手动拼 JSON
4. 在 `main()` 中测试正常/坏行/不存在三种场景

**参考实现**

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
enum FileStatsError {
    IoError(io::Error),
    ParseError(String),
    FileNotFound(String),
    JsonError(String),
}

impl fmt::Display for FileStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileStatsError::IoError(e) => write!(f, "IO 错误: {}", e),
            FileStatsError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            FileStatsError::FileNotFound(path) => write!(f, "文件不存在: {}", path),
            FileStatsError::JsonError(msg) => write!(f, "JSON 错误: {}", msg),
        }
    }
}

impl std::error::Error for FileStatsError {}

impl From<io::Error> for FileStatsError {
    fn from(e: io::Error) -> Self {
        FileStatsError::IoError(e)
    }
}

impl From<ParseIntError> for FileStatsError {
    fn from(e: ParseIntError) -> Self {
        FileStatsError::ParseError(format!("数字解析失败: {}", e))
    }
}

fn batch_parse_to_json(input_path: &str, output_path: &str) -> Result<(), FileStatsError> {
    // 检查文件存在
    if !Path::new(input_path).exists() {
        return Err(FileStatsError::FileNotFound(input_path.to_string()));
    }

    let content = fs::read_to_string(input_path)?;

    // 核心：使用 collect::<Result<Vec<_>, _>>() 短路收集
    let numbers: Vec<i32> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            line.parse::<i32>()
                .map_err(|_| FileStatsError::ParseError(format!(
                    "第 {} 行不是有效数字: \"{}\"",
                    i + 1,
                    line
                )))
        })
        .collect::<Result<Vec<i32>, FileStatsError>>()?; // 短路：第一个 Err 就返回

    // 手动构造 JSON
    let json = format!(
        "[{}]",
        numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    fs::write(output_path, &json)?;
    println!(
        "✅ JSON 已写入 {}，包含 {} 个数字",
        output_path,
        numbers.len()
    );
    Ok(())
}

fn main() {
    // 场景 1: 正常输入
    let numbers_file = "/tmp/numbers_for_json.txt";
    fs::write(numbers_file, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n")
        .expect("无法创建测试文件");
    match batch_parse_to_json(numbers_file, "/tmp/output.json") {
        Ok(()) => {}
        Err(e) => println!("❌ {}", e),
    }

    // 场景 2: 有一行不是数字（验证短路收集）
    let bad_file = "/tmp/numbers_bad.txt";
    fs::write(bad_file, "1\n2\nabc\n4\n5\n")
        .expect("无法创建测试文件");
    match batch_parse_to_json(bad_file, "/tmp/output_bad.json") {
        Ok(()) => {}
        Err(e) => println!("❌ {}", e),
    }

    // 场景 3: 文件不存在
    match batch_parse_to_json("/tmp/nonexistent_file.txt", "/tmp/output_missing.json") {
        Ok(()) => {}
        Err(e) => println!("❌ {}", e),
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 手动写 `for` 循环收集，未用 `collect` 模式 | 题目要求使用 `collect::<Result<Vec<_>, _>>()` 模式。手动循环也能实现但冗长且容易出错 |
| `collect` 的 `map` 中返回了 `Result` 但 `collect()` 没标注类型 | 必须显式标注 `collect::<Result<Vec<i32>, FileStatsError>>()` 让编译器知道要短路收集 |
| 忘记实现 `From<ParseIntError>` | 没有 `From` 实现时，`line.parse::<i32>()?` 无法自动转换错误类型，需要手动 `.map_err()` |
| JSON 手动拼接时忘记处理空 Vec | 空数组应该输出 `[]`，如果 `numbers.is_empty()` 直接 `fs::write(output_path, "[]")` 更安全 |
| `ParseIntError` 不包含原始字符串 | `From<ParseIntError>` 实现中无法附加"第几行"的上下文信息。应在 `map_err` 中捕获并附加上下文 |

**验证方式**

```bash
cargo run

# 检查输出文件
cat /tmp/output.json
# 预期: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# 确认短路行为
# 预期输出中有: ❌ 解析失败: 第 3 行不是有效数字: "abc"
```

---

## 思考题

---

### 为什么 Rust 社区说 "unwrap() is a code smell"？

**1. 类型安全视角：`unwrap()` 绕过了 Rust 的类型系统检查**

Rust 的类型系统将"可能失败"编码为类型签名中的 `Result<T, E>`。这强迫调用方看到 `Result` 就必须处理两种可能——编译器强制执行穷尽性匹配。`unwrap()` 在 `Err` 时触发 `panic!`，实际上是把编译期的类型安全恢复为运行时的崩溃风险。它绕过了 Rust "让错误处理显式化"的设计哲学，把本应在代码中处理的错误路径交给了程序崩溃。

`panic!` 的本质是"不可恢复错误"——程序遇到了无法继续的状态（如数组越界、逻辑矛盾）。但 I/O 失败、网络超时、解析失败是"可恢复错误"——调用方可能选择重试、降级或报告用户。用 `unwrap()` 处理可恢复错误，等于是把"暂时打不开文件"当成了"程序逻辑 bug"，混淆了不可恢复与可恢复的边界。

**2. 代码演进视角：从原型到生产的代价**

原型阶段用 `unwrap()` 写出的代码要用于生产时，需要系统性替换所有 `unwrap()` 为正确的错误处理：
- 逐一审查每个 `unwrap()` 的调用点：这个错误是否真的"不可能发生"？
- 如果是可恢复错误：替换为 `match` 或 `?` 传播
- 如果是"确实不可能"：至少换成 `expect("why this is infallible")` 留下文档
- 如果是库代码：函数签名需要从 `-> T` 改为 `-> Result<T, E>`，这是一个破坏性 API 变更

这种从"快速原型"到"正确实现"的重构代价往往被低估。更好的实践是从一开始就返回 `Result`，只在原型中暂时统一用 `?` 传播到 `main()`，而非在各个层级 `unwrap()`。

**3. 与 Python 的对比**

Python 的 `try: value = dict[key] except KeyError: ...` 类似于 Rust 的 `match result { Ok(v) => ..., Err(_) => ... }` 模式——两者都显式处理了"key 不存在"的错误路径。`dict.get(key, default)` 类似 Rust 的 `unwrap_or(default)`——提供一个值来替代缺失情况。

Python 开发者容易犯的错误：
- **异常吞噬**：`except: pass` 或 `except Exception: pass` 静默吞掉所有异常，导致 bug 难以追踪。这在 Rust 中没有直接等价物，因为 `Result` 不能被"忽略"（编译器至少会警告 `#[must_use]`）
- **异常范围过宽**：`except Exception` 捕获了包括 `KeyboardInterrupt`、`SystemExit` 以外的几乎所有异常，可能掩盖了不相关的 bug
- **忘记可能的异常**：Python 的函数签名不声明可能抛出的异常，调用方只能靠文档或源码了解。Rust 的 `Result<T, E>` 将错误类型明确编码在签名中

**4. 实例分析**

原代码：
```rust
fn load_user_data() -> Vec<User> {
    let file = std::fs::File::open("users.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let users: Vec<User> = serde_json::from_reader(reader).unwrap();
    users
}
```

问题分析：
- **文件被删除**：`unwrap()` 触发 panic，程序崩溃。在 Web 服务器中这意味着整个进程终止，所有正在处理的请求全部失败
- **JSON 损坏**：同上，panic 不可恢复，且不会给用户返回任何友好的错误信息
- **每次请求调用**：每次 panic 都是致命的。如果文件只在启动时加载一次，panic 影响有限；但如果每次请求都 panic，服务完全不可用

改进版本：
```rust
fn load_user_data() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("users.json")?;
    let reader = std::io::BufReader::new(file);
    let users: Vec<User> = serde_json::from_reader(reader)?;
    Ok(users)
}
// 调用方可以决定：失败时用缓存数据、返回错误页面、还是重试
```

---

## 迁移思维练习答案

### 1. C 中用返回 -1 表示错误的函数，改为 Rust 应该返回什么？

返回 Result<T, E>，其中 E 是自定义的错误类型（通常是 enum）。C 的 -1 丢失了"为什么失败"的信息——是文件不存在、权限不足还是磁盘满了？调用者只能靠 errno 推测，且 errno 可能在中间调用中被覆盖。Rust 的 Result 将失败原因编码进类型签名（如 `Result<File, io::Error>`），? 运算符自动传播错误而保留类型信息，编译器还会警告未处理 Result 值，彻底消除"忘记检查返回值"这一类 bug。

### 2. Python 的 try/except 逻辑如何用 Result<T, E> + ? 重新表达？

Python 的 try/except 在控制流上不透明——调用者从函数签名无法知道可能抛出什么异常，只能靠文档或读源码。Rust 用 Result 将错误变成普通值：成功时 Ok(T)，失败时 Err(E)，两者都是值而非"被抛出的异常对象"。`?` 运算符的作用类似 `try: ... except: raise`——如果值是 Err 就立即从当前函数返回该错误，但区别在于它只传播错误而不捕获，错误类型必须兼容或可转换（通过 From trait），整个传播链是类型安全的。

### 3. 什么时候应该自己定义错误 enum，什么时候用 Box<dyn Error>？

当调用者需要根据错误类型做不同处理时（如区分"重试"和"报错给用户"），应定义自定义错误 enum，每个变体对应一种可区分的错误场景。当错误只需向上传播并最终打印/记日志时，Box<dyn Error> 或 anyhow crate 提供了便捷的"万能错误类型"。自定义 enum 的优势是类型安全（match 时编译器保证穷尽性），代价是样板代码；Box<dyn Error> 的优势是简洁，代价是丢失类型区分能力。
