# 第 9 章练习题：枚举、Option 与模式匹配

> 完成这些练习以巩固你对 Rust 枚举、Option 和模式匹配的理解。
> 所有代码使用 Rust 2024 版，可在 `src/main.rs` 旁新建文件或在 `main()` 中添加测试代码。

---

## 目录

- [Level 1 基础练习（3 题）](#level-1-基础练习)
- [Level 2 进阶练习（2 题）](#level-2-进阶练习)
- [Level 3 挑战练习（1 题）](#level-3-挑战练习)
- [思考题](#思考题)
- [推荐命令](#推荐命令)

---

## Level 1 基础练习

### 练习 1-1：定义交通信号灯枚举

**目标**：掌握枚举定义和 Display trait 实现。

**要求**：
1. 定义一个枚举 `TrafficLight`，包含三个变体：
   - `Red` —— 携带一个 `u32`（红灯剩余秒数）
   - `Yellow` —— 纯标签，不携带数据
   - `Green` —— 携带一个 `u32`（绿灯剩余秒数）
2. 为 `TrafficLight` 实现 `std::fmt::Display` trait
   - `Red(30)` 输出：`"红灯，剩余 30 秒"`
   - `Yellow` 输出：`"黄灯，请减速"`
   - `Green(45)` 输出：`"绿灯，剩余 45 秒"`
3. 编写 `fn next_light(light: &TrafficLight) -> TrafficLight` 函数：
   - `Red` → `Green(60)`
   - `Green(_)` → `Yellow`
   - `Yellow` → `Red(30)`
4. 在 `main()` 中测试：创建三种灯，打印它们，然后调用 `next_light` 并再次打印

**预期输出示例**：
```
当前: 红灯，剩余 30 秒
切换后: 绿灯，剩余 60 秒

当前: 绿灯，剩余 45 秒
切换后: 黄灯，请减速

当前: 黄灯，请减速
切换后: 红灯，剩余 30 秒
```

**提示**：`Display` trait 需要 `use std::fmt;`，然后在 `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result` 中使用 `write!(f, "...")`。

<details>
<summary>点击查看参考代码框架</summary>

```rust
use std::fmt;

#[derive(Debug)]
enum TrafficLight {
    Red(u32),
    Yellow,
    Green(u32),
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrafficLight::Red(secs) => write!(f, "红灯，剩余 {secs} 秒"),
            TrafficLight::Yellow => write!(f, "黄灯，请减速"),
            TrafficLight::Green(secs) => write!(f, "绿灯，剩余 {secs} 秒"),
        }
    }
}

fn next_light(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red(_) => TrafficLight::Green(60),
        TrafficLight::Green(_) => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red(30),
    }
}
```

</details>

---

### 练习 1-2：用 match 处理 Option —— 成绩查询

**目标**：熟练使用 `match` 处理 `Option<T>`。

**要求**：
1. 编写函数 `fn get_score(student_name: &str) -> Option<u32>`：
   - `"Alice"` → `Some(95)`
   - `"Bob"` → `Some(72)`
   - `"Charlie"` → `Some(58)`（不及格）
   - `"Dave"` → `None`（缺考）
   - 其他名字 → `None`
2. 编写函数 `fn grade_description(score: u32) -> &'static str`：
   - `0..=59` → `"不及格"`
   - `60..=79` → `"良好"`
   - `80..=100` → `"优秀"`
   - 其他 → `"无效分数"`
3. 编写函数 `fn describe_score(name: &str) -> String`：
   - 用 `match` 同时匹配 `get_score(name)` 和成绩等级
   - 存在成绩 → `"{name}: {score}分 ({grade})"`
   - 缺考 → `"{name}: 缺考"`
4. 在 `main()` 中测试 Alice、Bob、Charlie、Dave 和 Eve

**预期输出示例**：
```
Alice: 95分 (优秀)
Bob: 72分 (良好)
Charlie: 58分 (不及格)
Dave: 缺考
Eve: 缺考
```

**提示**：可以在 `match` 的守卫中使用 `if`：
```rust
match get_score(name) {
    Some(score) if score >= 80 => format!("{name}: {score}分 (优秀)"),
    // ...
}
```

---

### 练习 1-3：用 if let 简化代码

**目标**：理解 `if let` 与 `match` 的等价关系。

**要求**：
文件中有如下使用 `match` 的代码，请将它们改写为 `if let` 形式：

**代码 A** —— 只关心一种情况：
```rust
let config_value: Option<String> = Some("rust-lang.org".to_string());
match config_value {
    Some(url) => println!("连接到 {url}"),
    None => (),
}
```

**代码 B** —— 同时使用 if let 和 else：
```rust
let cached: Option<String> = None;
match cached {
    Some(data) => println!("缓存命中: {data}"),
    None => println!("缓存未命中，从数据库加载..."),
}
```

**代码 C** —— 带守卫条件：
```rust
let maybe_age: Option<u8> = Some(17);
match maybe_age {
    Some(age) if age >= 18 => println!("成年人: {age}岁"),
    Some(age) => println!("未成年人: {age}岁"),
    None => println!("年龄未知"),
}
```

**改写后**：将改写后的代码放入 `main()` 函数中运行验证。

---

## Level 2 进阶练习

### 练习 2-1：事件处理系统 —— 嵌套模式匹配

**目标**：掌握嵌套模式匹配和穷尽性检查。

**要求**：

1. 定义以下类型：
```rust
#[derive(Debug)]
enum Event {
    MouseClick { x: i32, y: i32 },
    KeyPress { key: char, ctrl: bool },
    WindowResize { width: u32, height: u32 },
    Quit,
}

#[derive(Debug)]
enum EventOutcome {
    Handled(String),
    Ignored,
}
```

2. 实现函数 `fn handle_event(event: &Event) -> EventOutcome`：

   - **MouseClick { x, y }**：
     - 如果 x 和 y 都在 0..=800 范围内 → `Handled("点击在屏幕内({x}, {y})".to_string())`
     - 否则 → `Handled("点击超出边界({x}, {y})".to_string())`

   - **KeyPress { key, ctrl }**：
     - 如果 `ctrl` 为 `true` 且 `key` 为 `'c'`  → `Handled("复制".to_string())`
     - 如果 `ctrl` 为 `true` 且 `key` 为 `'v'` → `Handled("粘贴".to_string())`
     - 如果 `ctrl` 为 `true` 且 `key` 为 `'q'` → 返回 `Handled("退出程序".to_string())`（但要额外打印 `"Quit shortcut detected!"`）
     - 其他按键 → `Handled(format!("按键: {key}"))`

   - **WindowResize { width, height }**：
     - 如果 `width < 100 || height < 100` → `Handled("窗口太小".to_string())`
     - 否则 → `Handled(format!("窗口调整为 {width}x{height}"))`

   - **Quit** → `Handled("程序退出".to_string())`

3. 实现函数 `fn process_events(events: &[Event])`：
   - 遍历所有事件
   - 用 `match` 处理每个事件，只打印 `Handled(msg)` 的结果
   - 用 `if let` 过滤掉 `Ignored` 的结果（本题所有事件都返回 Handled，所以 Ignored 不会出现）

4. 编写 `fn demo_nested_match()` 函数，使用**嵌套模式匹配**同时检查 `Event` 变体和内部字段：
   - 使用 `matches!` 宏判断事件是否为 `KeyPress` 且带有 `ctrl`
   - 使用 `match` 守卫：当事件为 `MouseClick` 且坐标为原点 `(0, 0)` 时，打印特殊消息

5. 在 `main()` 中调用 `process_events` 处理一组测试事件，然后调用 `demo_nested_match`

**提示**：注意 `match` 中模式覆盖的顺序——把更具体的模式放在前面。

<details>
<summary>点击查看测试事件数据</summary>

```rust
let events = vec![
    Event::MouseClick { x: 150, y: 300 },
    Event::KeyPress { key: 'c', ctrl: true },
    Event::KeyPress { key: 'v', ctrl: false },
    Event::WindowResize { width: 1920, height: 1080 },
    Event::MouseClick { x: 900, y: 100 },  // 超出边界（假设屏幕 800x600）
    Event::KeyPress { key: 'q', ctrl: true },
    Event::WindowResize { width: 50, height: 80 },  // 太小
    Event::Quit,
];
```

</details>

---

### 练习 2-2：用 let else 重构“箭头代码”

**目标**：使用 `let else` 消除深层嵌套，体会扁平化代码的优势。

**要求**：

下面是"读取配置 → 连接数据库 → 查询用户 → 发送通知"的模拟代码。它用 `match` 实现了类似"回调地狱"的深层嵌套（箭头代码）。

**重构前代码**（箭头式嵌套，请运行它看看效果，然后重构）：

```rust
#[derive(Debug)]
struct Config {
    db_url: Option<String>,
    api_key: Option<String>,
}

#[derive(Debug)]
struct DbConnection {
    url: String,
}

#[derive(Debug)]
struct User {
    name: String,
    notification_email: Option<String>,
}

fn load_config() -> Option<Config> {
    Some(Config {
        db_url: Some("postgres://localhost/mydb".to_string()),
        api_key: Some("sk-abc123".to_string()),
    })
}

fn connect_db(url: &str) -> Option<DbConnection> {
    if url.is_empty() { None } else {
        Some(DbConnection { url: url.to_string() })
    }
}

fn find_user(conn: &DbConnection, name: &str) -> Option<User> {
    Some(User {
        name: name.to_string(),
        notification_email: Some(format!("{name}@example.com")),
    })
}

fn send_notification(user: &User, api_key: &str) -> Option<String> {
    match &user.notification_email {
        Some(email) => Some(format!("已向 {email} 发送通知（密钥: {api_key}）")),
        None => None,
    }
}

// 这是需要你重构的函数 —— 它有很多嵌套
fn notify_user_old(username: &str) -> Option<String> {
    match load_config() {
        Some(config) => {
            match config.db_url {
                Some(ref db_url) => {
                    match connect_db(db_url) {
                        Some(conn) => {
                            match find_user(&conn, username) {
                                Some(user) => {
                                    match config.api_key {
                                        Some(ref api_key) => {
                                            send_notification(&user, api_key)
                                        }
                                        None => None,
                                    }
                                }
                                None => None,
                            }
                        }
                        None => None,
                    }
                }
                None => None,
            }
        }
        None => None,
    }
}
```

**任务**：用 `let else` 语法将 `notify_user_old` 重构为扁平化的版本 `notify_user_new`。

**重构目标代码风格**：
```rust
fn notify_user_new(username: &str) -> Option<String> {
    let Some(config) = load_config() else { return None; };
    let Some(ref db_url) = config.db_url else { return None; };
    // ... 继续扁平化
}
```

**额外挑战**：用 `?` 操作符再做一版 `notify_user_try`（提示：`?` 在 `Option` 上的行为是 `None` 时提前返回 `None`）。

**对比**：在 `main()` 中分别调用三个版本，打印结果，感受代码可读性的差异。

---

## Level 3 挑战练习

### 练习 3-1：小型命令解析器

**目标**：综合运用枚举、模式匹配、`Option`、`Result`，构建一个内部 DSL。

**要求**：

1. **定义命令枚举**：
```rust
#[derive(Debug, PartialEq)]
enum Command {
    Help,
    List { filter: Option<String> },
    Create { name: String, email: Option<String> },
    Delete { name: String, force: bool },
    Unknown(String),
}
```

2. **实现命令解析器** `fn parse_command(input: &str) -> Option<Command>`：
   - 输入字符串按空白字符分割
   - 解析规则：
     | 输入示例 | 解析结果 |
     |---------|---------|
     | `"help"` | `Some(Command::Help)` |
     | `"list"` | `Some(Command::List { filter: None })` |
     | `"list active"` | `Some(Command::List { filter: Some("active") })` |
     | `"create alice"` | `Some(Command::Create { name: "alice", email: None })` |
     | `"create alice a@b.com"` | `Some(Command::Create { name: "alice", email: Some("a@b.com") })` |
     | `"delete bob"` | `Some(Command::Delete { name: "bob", force: false })` |
     | `"delete bob -f"` | `Some(Command::Delete { name: "bob", force: true })` |
     | `"unknown_cmd"` | `Some(Command::Unknown("unknown_cmd"))` |
     | 空字符串 | `None` |

3. **实现命令执行器** `fn execute_command(cmd: &Command)`：
   - 用 `match` 处理每个命令
   - `Help` → 打印所有可用命令的帮助信息
   - `List { filter }` → 打印列表（如果 filter 存在则打印过滤条件）
   - `Create { name, email }` → 打印创建信息（用 `if let` 判断是否有 email）
   - `Delete { name, force }` → 如果 `force` 为 false，打印 `"确认删除 {name}? 使用 -f 强制删除"`；如果为 true，打印 `"{name} 已被强制删除"`
   - `Unknown(cmd)` → 打印 `"未知命令: {cmd}，输入 help 查看帮助"`
   - 使用 `let else` 确保解析成功

4. **实现交互循环**（可选加分项）：
```rust
fn repl() {
    println!("命令解析器已启动，输入命令 (Ctrl+D 退出):");
    for line in std::io::stdin().lines() {
        let Ok(line) = line else { break; };
        let line = line.trim().to_string();
        if line.is_empty() { continue; }

        let Some(cmd) = parse_command(&line) else {
            println!("无法解析输入");
            continue;
        };

        if matches!(&cmd, Command::Unknown(_)) {
            println!("警告: 无法识别的命令");
        }

        execute_command(&cmd);
        println!(); // 空行分隔
    }
}
```

5. **编写 5 个单元测试**（用 `#[cfg(test)]` 模块）：
   - 测试 `parse_command("help")` 返回 `Command::Help`
   - 测试 `parse_command("delete bob -f")` 返回 `Command::Delete { name: "bob", force: true }`
   - 测试 `parse_command("unknown")` 返回 `Command::Unknown("unknown")`
   - 测试 `parse_command("")` 返回 `None`
   - 测试 `parse_command("list active")` 返回 `Command::List { filter: Some("active") }`

**提示**：解析时可以使用 `Vec<String>` 的 `split_whitespace()` 和 `match` 切片模式：
```rust
let parts: Vec<&str> = input.split_whitespace().collect();
match parts.as_slice() {
    ["help"] => Some(Command::Help),
    ["list"] => Some(Command::List { filter: None }),
    ["list", filter] => Some(Command::List { filter: Some(filter.to_string()) }),
    // ... 继续
    [] | _ => None,
}
```

---

## 思考题

### 为什么 Rust 选择 Option\<T\> 而不是 null？

Tony Hoare 称 null 引用是他职业生涯中最大的错误（"the billion-dollar mistake"）。Rust 的设计者选择不在语言中包含 null，而是用 `Option<T>` 枚举来表达"值可能缺失"的语义。

请结合本章内容，从以下角度思考并写下你的理解：

1. **类型安全性**：`Option<T>` 和 `T` 是不同的类型，这带来了什么好处？和 Python 的 `str | None` 类型提示有什么本质区别？

2. **编译器角色**：Rust 编译器在模式匹配中扮演什么角色？穷尽性检查是如何把运行时 bug 变成编译期错误的？

3. **设计取舍**：Rust 选择了"没有 null"，但代价是什么？你觉得这个取舍值得吗？在什么场景下你会特别感激这个设计，在什么场景下你会觉得不方便？

4. **对比其他语言**：TypeScript 的 `strictNullChecks`、Kotlin 的可空类型（`String?`）、Swift 的 Optional、Rust 的 `Option<T>`——它们解决的是同一个问题。查阅资料后，比较它们的设计异同。

*提示：没有标准答案。请结合你的编程经验，用自己的话分析。认真思考这道题比完成所有编程练习更有价值。*

---

## 推荐命令

### 编译与运行

```bash
# 在项目目录下编译并运行
cd chapters/09_enums_option_pattern_matching
cargo run

# 检查代码是否编译通过（不运行）
cargo check

# 以 release 模式编译（优化后的版本）
cargo build --release
```

### 测试

```bash
# 运行单元测试（练习 3-1 的测试模块）
cargo test

# 显示测试输出（包括 println! 的内容）
cargo test -- --nocapture

# 只运行特定名称的测试
cargo test test_parse_help
```

### 代码质量

```bash
# Clippy linter — Rust 官方静态分析工具
cargo clippy

# 如果提示 clippy 未安装，先安装：
rustup component add clippy

# 自动修复 clippy 建议的问题
cargo clippy --fix

# 格式化代码
cargo fmt
```

### 文档

```bash
# 生成并打开本地文档（包含你写的文档注释）
cargo doc --open
```

### 学习技巧

1. **先编译，再阅读**：跑通 `cargo run` 看输出，理解数据流，然后再逐行看代码
2. **故意引入错误**：尝试删除一个 `match` 分支、把 `Some` 赋值给 `String`、在 `let else` 中不加 `return`，看看编译器报什么错——这是最快的学习方式
3. **对照 Python 实现**：为每个练习写一份 Python 版本，体会 Rust 的编译期检查 vs Python 的运行时检查
4. **练习 3-1 是综合题**：它涵盖了本章所有知识点，如果做完这题并且单元测试通过，说明你真正理解了枚举和模式匹配
