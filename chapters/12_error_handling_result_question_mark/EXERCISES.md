# 第十二章练习题：Rust 错误处理

> 所有练习均基于本章的 `error_handling` 包。请在项目根目录下完成。
>
> **编译命令**: `cargo build`  
> **运行命令**: `cargo run`  
> **测试命令**: `cargo test`  
> **查看 backtrace**: `RUST_BACKTRACE=1 cargo run`

---

## Level 1：基础练习（3 题）

这些练习帮助你熟悉 Result 的基本用法和 ? 运算符的语法。

### 练习 1-1：读取文件并统计行数

**目标**：练习 `Result` 的接收与 `match` 处理。

在 `main.rs` 中添加一个新函数 `count_lines(path: &str) -> Result<usize, std::io::Error>`：

- 调用 `std::fs::read_to_string(path)` 读取文件内容
- 返回文件的行数（包括空行）
- 使用 `match` 在 `main()` 中调用该函数并打印结果
- 同时用不存在的路径调用，观察错误输出

**预期输出示例**：
```
文件 data.txt 共有 12 行
错误: No such file or directory (os error 2)
```

**提示**：`String` 有一个 `.lines()` 方法返回迭代器，再调用 `.count()` 即可得到行数。

**推荐命令**：
```bash
# 运行程序查看输出
cargo run
```

---

### 练习 1-2：使用 unwrap_or 提供默认值

**目标**：区分 `unwrap()`、`unwrap_or()` 和 `unwrap_or_else()` 的适用场景。

在 `main.rs` 中添加一个函数 `get_config_value(key: &str) -> Result<String, std::env::VarError>`：

- 调用 `std::env::var(key)` 读取环境变量
- 在 `main()` 中进行以下实验：

```rust
// 实验 1: 使用 unwrap_or —— 环境变量不存在时用默认值
let db_url = get_config_value("DATABASE_URL")
    .unwrap_or("postgres://localhost:5432/mydb".to_string());

// 实验 2: 使用 unwrap_or_else —— 默认值需要计算
let log_path = get_config_value("LOG_PATH")
    .unwrap_or_else(|_| format!("/var/log/{}.log", std::process::id()));

// 实验 3: 使用 expect —— 这个变量必须存在
let api_key = get_config_value("API_KEY")
    .expect("API_KEY 环境变量必须设置");
```

- 运行程序（API_KEY 大概率不存在），观察 `expect` 触发的 panic 消息
- 注释掉实验 3 再次运行，观察前两个实验的输出

**思考**：为什么实验 3 要用 `expect` 而不是 `unwrap`？

**推荐命令**：
```bash
# 设置环境变量后运行
DATABASE_URL="mysql://localhost/test" cargo run

# 不设置环境变量，观察 expect 的 panic 信息
cargo run
```

---

### 练习 1-3：实现自定义 Error 类型

**目标**：掌握自定义错误类型的最小实现。

在 `main.rs` 中定义一个 `ConfigError` 枚举：

```rust
#[derive(Debug)]
enum ConfigError {
    NotFound(String),
    InvalidFormat(String),
}
```

要求：
1. 为 `ConfigError` 实现 `std::fmt::Display` —— 给用户友好的错误消息
2. 为 `ConfigError` 实现 `std::error::Error` —— 至少是最小实现（`{}`）
3. 编写一个函数 `parse_config_line(line: &str) -> Result<(String, String), ConfigError>`，解析格式为 `key=value` 的配置行：
   - 空行返回 `Err(ConfigError::NotFound(...))`
   - 没有 `=` 的行返回 `Err(ConfigError::InvalidFormat(...))`
   - 正常解析返回 `Ok((key, value))`
4. 在 `main()` 中用 match 调用该函数，分别处理三种情况

**预期输出示例**：
```
解析 "host=localhost": ✅ host = localhost
解析 "": ❌ 配置不存在: 空行
解析 "invalid_line": ❌ 格式错误: 缺少 '=': "invalid_line"
```

**推荐命令**：
```bash
cargo run
```

---

## Level 2：综合练习（2 题）

这些练习综合使用 ? 运算符、错误转换和文件 I/O。

### 练习 2-1：安全的数字文件处理器

**目标**：综合运用本章所有错误处理技巧，构建一个完整的文件处理管道。

修改 `main.rs`，在已有的 `run_analysis` 函数基础上，增强功能：

**任务**：
1. 让程序接受命令行参数作为文件路径（使用 `std::env::args()`）
   - 如果没有提供参数，使用默认的 `/tmp/rust_error_demo_numbers.txt`
   - 使用 `nth(1)` 从 `args()` 迭代器中获取第一个参数
2. 为 `parse_numbers` 函数增加对浮点数的支持：
   - 尝试先解析为 `i32`，失败则尝试 `f64`，再失败才报错
   - 如果解析到浮点数，四舍五入为 `i32` 并打印一条警告
3. 添加一个新函数 `verify_file_exists(path: &str) -> Result<(), FileStatsError>`，在读取文件前检查文件是否存在：
   - 使用 `std::path::Path::new(path).exists()` 检查
   - 如果文件不存在，返回自定义的 `FileStatsError` 变体
   - 在 `run_analysis()` 中使用 `?` 调用它
4. 打印更丰富的统计信息：
   - 所有数字排序后的输出（前 5 个 + 后 5 个）
   - 正数个数 vs 负数个数
   - 零的个数

**要求**：
- 必须使用 `?` 运算符进行错误传播（不允许全部用 match）
- 至少在一处使用 `.map_err()` 转换错误类型
- 在创建示例文件时使用 `?` 而不是 `unwrap()`

**推荐命令**：
```bash
# 使用默认文件运行
cargo run

# 使用自定义文件运行
cargo run -- /path/to/your/numbers.txt

# 使用不存在的文件测试错误处理
cargo run -- /tmp/nonexistent.txt
```

---

### 练习 2-2：Result 的链式操作

**目标**：理解 `and_then`、`map`、`map_err`、`or_else` 等组合子的用法。

在 `main.rs` 中添加一个函数 `process_pipeline(input: &str) -> Result<i32, String>`，用**链式调用**（不允许使用 ? 运算符）完成以下管道：

```
输入字符串 → 去除首尾空白 → 检查是否为空 → 解析为 i32 → 检查范围 [1, 100] → 返回
```

要求每一步都用 `Result` 的组合子方法实现（`and_then`、`map` 等）：

```rust
// 链式调用框架（你需要填充）：
fn process_pipeline(input: &str) -> Result<i32, String> {
    let trimmed = input.trim().to_string();

    // Step 1: 用 Ok(trimmed) 开始管道
    // Step 2: 用 and_then 检查是否为空
    // Step 3: 用 and_then 解析为 i32
    // Step 4: 用 and_then 检查范围 [1, 100]
    // 返回最终结果
}
```

**测试用例**（在 main 中调用并验证）：

| 输入 | 预期输出 |
|------|----------|
| `"42"` | `Ok(42)` |
| `" 99 "` | `Ok(99)` |
| `""` | `Err("输入为空")` |
| `"abc"` | `Err("不是有效数字: abc")` |
| `"0"` | `Err("数字 0 不在范围 [1, 100] 内")` |
| `"200"` | `Err("数字 200 不在范围 [1, 100] 内")` |

**思考**：链式调用和 ? 运算符分别适合什么场景？用 ? 重写这个函数对比代码行数。

**推荐命令**：
```bash
cargo run
cargo test
```

---

## Level 3：高级挑战（1 题）

### 练习 3-1：构建 Result 迭代器适配器

**目标**：深入理解 Rust 的迭代器与 Result 的集成，学习 `collect::<Result<Vec<_>, _>>()` 模式。

**背景**：Rust 的迭代器有一个强大的特性：当你对 `Result` 元素的迭代器调用 `collect()` 时，它会短路处理——遇到第一个 `Err` 就停止收集并返回该错误。

**任务**：

1. 创建一个函数 `batch_parse_to_json(input_path: &str, output_path: &str) -> Result<(), FileStatsError>`：
   - 读取输入文件（每行一个数字）
   - 解析所有数字
   - 将数字数组序列化为 JSON 并写入输出文件

2. 关键的实现要求：
   - 在解析阶段使用迭代器的 `collect::<Result<Vec<i32>, _>>()` 模式
   - 对于 JSON 序列化，使用硬编码的字符串拼接（不引入 serde 依赖）
   - 手动构造 JSON 格式：`[1, 2, 3, ...]`

3. 为 `FileStatsError` 增加一个新的变体 `JsonError(String)`：
   - 更新 `Display` 和 `Error` 的实现
   - 更新所有 match 分支

4. 实现 `From<std::num::ParseIntError>` 用于 `FileStatsError`：
   - 这样 ? 运算符处理 `parse::<i32>()` 时就能自动转换

5. 在 `main()` 中添加调用，测试以下场景：
   - 输入文件正常
   - 输入文件有一行不是数字（验证短路收集）
   - 输入文件不存在

**预期输出示例**：
```
✅ JSON 已写入 /tmp/output.json，包含 10 个数字
❌ 解析失败: 第 3 行不是有效数字: "abc"
❌ 输入文件不存在: /tmp/nonexistent.txt
```

**推荐命令**：
```bash
# 运行完整程序测试所有场景
cargo run

# 在 release 模式下验证性能
cargo build --release
./target/release/error_handling

# 检查输出文件内容
cat /tmp/output.json 2>/dev/null || echo "文件不存在（预期之内）"
```

---

## 思考题

### 为什么 Rust 社区说 "unwrap() is a code smell"？

请结合本章内容，从以下角度阐述你的理解（300 字以上）：

1. **类型安全视角**：`unwrap()` 绕过了 Rust 的类型系统检查，它与 Rust "让错误处理显式化"的设计哲学如何冲突？
2. **代码演进视角**：假设你在一开始用 `unwrap()` 写原型代码，后来要将这段代码用于生产环境，你需要做什么改造？这种改造的代价有多大？
3. **与 Python 的对比**：Python 代码中常见 `try: value = dict[key] except KeyError: ...`，这是否类似于 Rust 的 `unwrap_or`？Python 开发者更容易犯什么错误处理方面的错误？
4. **实例分析**：下面这段 Rust 代码存在什么问题？如何改进？

```rust
fn load_user_data() -> Vec<User> {
    let file = std::fs::File::open("users.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let users: Vec<User> = serde_json::from_reader(reader).unwrap();
    users
}
```

**提示**：考虑以下场景：
- 文件被其他进程删除
- 文件存在但 JSON 格式损坏
- 这个函数被用在 Web 服务器中，每次请求都会调用它

---

## 练习完成度自检清单

| 练习 | 完成了？ | 掌握的关键概念 |
|------|----------|----------------|
| 1-1 | ☐ | match, Result 基本模式 |
| 1-2 | ☐ | unwrap, unwrap_or, unwrap_or_else, expect |
| 1-3 | ☐ | 自定义 Error, Display, Error trait |
| 2-1 | ☐ | ?, map_err, 错误传播, 命令行参数 |
| 2-2 | ☐ | and_then, map, 链式组合子 |
| 3-1 | ☐ | collect pattern, From trait, 短路迭代 |
| 思考题 | ☐ | unwrap 的适用场景与局限 |

---

## 推荐学习资源

- [Rust Book 第九章: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example: Error handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [The Error Handling Project Group Blog](https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html)
- [Rust API: std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)
- [thiserror crate](https://crates.io/crates/thiserror) — 简化自定义错误定义
- [anyhow crate](https://crates.io/crates/anyhow) — 灵活的应用级错误处理
- [The Error Design Patterns in Rust](https://github.com/rust-lang/project-error-handling)
