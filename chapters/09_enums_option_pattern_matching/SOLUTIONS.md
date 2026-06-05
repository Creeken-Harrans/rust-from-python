# 第九章练习答案 — 枚举、Option 与模式匹配

---

## 练习 1-1: 交通信号灯枚举

### 结论

枚举每个变体可以携带不同类型的数据（或纯标签）。`Display` trait 通过 `match` 模式匹配为每个变体定制输出。

### 思路

1. 定义 `TrafficLight` 枚举，三个变体各带不同数据。
2. 实现 `Display`：用 `match` 解构每个变体，`write!` 宏格式化输出。
3. `next_light`：接受引用（不获取所有权），返回新枚举值。

### 参考实现

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

fn main() {
    let lights = vec![
        TrafficLight::Red(30),
        TrafficLight::Green(45),
        TrafficLight::Yellow,
    ];

    for light in &lights {
        println!("当前: {light}");
        let next = next_light(light);
        println!("切换后: {next}\n");
    }
}
```

### 常见错误

- `Display` 实现中忘记 `use std::fmt`。
- `write!` 宏中生命周期参数写错 —— `Formatter<'_>` 是正确写法。
- `next_light` 中获取所有权导致后续不可用 —— 应该接受 `&TrafficLight`。

### 验证方式

```bash
cargo run  # 输出三种灯的当前和切换后状态
```

---

## 练习 1-2: 用 match 处理 Option —— 成绩查询

### 结论

`Option<T>` 强制调用方在编译期处理"有值"和"无值"两种情况，消除 null 引用错误。这与 Python 的 `return None` 不同 —— **Option 不是 null 的改名**: `None` 是 `Option<T>` 枚举的一个变体，与 `Some(T)` 平级，编译器强制执行穷尽性检查（exhaustiveness checking），确保你处理了缺失的情况。

### 思路

1. `get_score` 返回 `Option<u32>`，匹配名字返回 `Some` 或 `None`。
2. `grade_description` 用范围模式匹配分数段（`0..=59`, `60..=79`, `80..=100`）。
3. `describe_score` 使用 match 守卫 `if` 同时解构分数和判断等级。

### 参考实现

```rust
fn get_score(student_name: &str) -> Option<u32> {
    match student_name {
        "Alice" => Some(95),
        "Bob" => Some(72),
        "Charlie" => Some(58),
        "Dave" => None,
        _ => None,
    }
}

fn grade_description(score: u32) -> &'static str {
    match score {
        0..=59 => "不及格",
        60..=79 => "良好",
        80..=100 => "优秀",
        _ => "无效分数",
    }
}

fn describe_score(name: &str) -> String {
    match get_score(name) {
        Some(score) => {
            let grade = grade_description(score);
            format!("{name}: {score}分 ({grade})")
        }
        None => format!("{name}: 缺考"),
    }
}

fn main() {
    for name in &["Alice", "Bob", "Charlie", "Dave", "Eve"] {
        println!("{}", describe_score(name));
    }
}
```

**match 穷尽性检查（exhaustiveness）**: 如果忘记写 `None => ...` 分支，编译器会报错 `non-exhaustive patterns: None not covered`。这是 Rust 将运行时 bug（null dereference）变成编译期错误的机制 —— match 必须覆盖所有可能的模式，一个都不能少。这是 `Option<T>` 优于 null 的核心原因之一。

### 常见错误

- 写出 `Some(score) => format!(...)` 忘了 None 分支 → 编译报错，穷尽性检查起作用。
- match 守卫语法写错：是 `Some(score) if score >= 80 =>` 而非 `Some(score >= 80) =>`。
- 范围模式使用 `...`（已弃用）而非 `..=`。

### 验证方式

```bash
cargo run  # 输出: Alice: 95分 (优秀), Bob: 72分 (良好), ...
```

---

## 练习 1-3: 用 if let 简化代码

### 结论

`if let` 是 `match` 的语法糖 —— 当你只关心一种模式时，`if let` 比 `match` 更简洁。带 `else` 的 `if let` 等价于两分支 match。

### 思路

**代码 A**（只关心 Some）: `match` 的 None 分支是空操作 → 用 `if let Some(url) = config_value`。

**代码 B**（两分支）: `if let Some(data) = cached` + `else`。

**代码 C**（带守卫）: `if let Some(age) = maybe_age && age >= 18`（注意：Rust 1.83+ 用 `&&`，旧版用 `if`）。

### 参考实现

```rust
fn main() {
    // 代码 A —— 只关心一种情况
    let config_value: Option<String> = Some("rust-lang.org".to_string());
    if let Some(url) = config_value {
        println!("连接到 {url}");
    }

    // 代码 B —— if let + else
    let cached: Option<String> = None;
    if let Some(data) = cached {
        println!("缓存命中: {data}");
    } else {
        println!("缓存未命中，从数据库加载...");
    }

    // 代码 C —— 带守卫条件 (Rust 2024 / 1.83+, 使用 &&)
    let maybe_age: Option<u8> = Some(17);
    if let Some(age) = maybe_age && age >= 18 {
        println!("成年人: {age}岁");
    } else if let Some(age) = maybe_age {
        println!("未成年人: {age}岁");
    } else {
        println!("年龄未知");
    }

    // 代码 C 等效写法 (旧版 Rust, 使用 match 守卫风格)
    let maybe_age2: Option<u8> = Some(17);
    match maybe_age2 {
        Some(age) if age >= 18 => println!("成年人: {age}岁"),
        Some(age) => println!("未成年人: {age}岁"),
        None => println!("年龄未知"),
    }
}
```

### 常见错误

- 在 `if let` 中尝试解构而忘了模式语法：`if let age = maybe_age`（错误，应为 `if let Some(age) = maybe_age`）。
- 守卫条件写法错误（Rust 版本差异）：旧版 `if let Some(age) = maybe_age && age >= 18` 需要用 match 替代。
- 忘记 `if let` 的 `else` 可以与多个 `else if let` 链式使用。

### 验证方式

```bash
cargo run  # 验证输出
```

---

## 练习 2-1: 事件处理系统 —— 嵌套模式匹配

### 结论

match 的模式可以嵌套 —— 在匹配变体的同时解构内部字段。`matches!` 宏用于返回 bool，适合在条件判断中快速检查模式。

### 思路

1. `handle_event` 对每个 Event 变体用 match，内部再按字段条件处理。
2. KeyPress + ctrl + 'q' 的特殊情况：需要在模式中解构同时用 if 守卫（`if key == 'q'`），然后额外 println!。
3. `process_events` 遍历并用 `if let` 过滤结果。
4. `demo_nested_match` 展示 `matches!` 宏和 match 守卫。

### 参考实现

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

fn handle_event(event: &Event) -> EventOutcome {
    match event {
        Event::MouseClick { x, y } => {
            if *x >= 0 && *x <= 800 && *y >= 0 && *y <= 800 {
                EventOutcome::Handled(format!("点击在屏幕内({x}, {y})"))
            } else {
                EventOutcome::Handled(format!("点击超出边界({x}, {y})"))
            }
        }
        Event::KeyPress { key, ctrl } => {
            if *ctrl {
                match key {
                    'c' => EventOutcome::Handled("复制".to_string()),
                    'v' => EventOutcome::Handled("粘贴".to_string()),
                    'q' => {
                        println!("Quit shortcut detected!");
                        EventOutcome::Handled("退出程序".to_string())
                    }
                    _ => EventOutcome::Handled(format!("按键: {key}")),
                }
            } else {
                EventOutcome::Handled(format!("按键: {key}"))
            }
        }
        Event::WindowResize { width, height } => {
            if *width < 100 || *height < 100 {
                EventOutcome::Handled("窗口太小".to_string())
            } else {
                EventOutcome::Handled(format!("窗口调整为 {width}x{height}"))
            }
        }
        Event::Quit => EventOutcome::Handled("程序退出".to_string()),
    }
}

fn process_events(events: &[Event]) {
    for event in events {
        let outcome = handle_event(event);
        if let EventOutcome::Handled(msg) = outcome {
            println!("  {msg}");
        }
    }
}

fn demo_nested_match() {
    let events = vec![
        Event::KeyPress { key: 'c', ctrl: true },
        Event::MouseClick { x: 0, y: 0 },
        Event::KeyPress { key: 'x', ctrl: false },
    ];

    for event in &events {
        // matches! 宏 —— 判断是否匹配某模式
        if matches!(event, Event::KeyPress { ctrl: true, .. }) {
            println!("检测到 Ctrl+按键 组合");
        }

        // match 守卫：匹配 MouseClick 且坐标为原点
        match event {
            Event::MouseClick { x: 0, y: 0 } => {
                println!("鼠标点击在原点！");
            }
            _ => {}
        }
    }
}

fn main() {
    let events = vec![
        Event::MouseClick { x: 150, y: 300 },
        Event::KeyPress { key: 'c', ctrl: true },
        Event::KeyPress { key: 'v', ctrl: false },
        Event::WindowResize { width: 1920, height: 1080 },
        Event::MouseClick { x: 900, y: 100 },
        Event::KeyPress { key: 'q', ctrl: true },
        Event::WindowResize { width: 50, height: 80 },
        Event::Quit,
    ];

    println!("=== 事件处理 ===");
    process_events(&events);

    println!("\n=== 嵌套模式演示 ===");
    demo_nested_match();
}
```

### 常见错误

- 在 match 守卫中忘记解构引用：`event` 是 `&Event`，`x`/`y` 是 `&i32`，需要 `*x` 或模式中 `x: &0`。
- `matches!` 宏中模式写错 —— 注意嵌套字段的写法。
- match 分支顺序错误 —— 更具体的模式应放在前面。

### 验证方式

```bash
cargo run  # 输出完整事件处理结果和嵌套模式演示
```

---

## 练习 2-2: 用 let else 重构"箭头代码"

### 结论

`let else` 将"提前返回失败"逻辑扁平化，消除深层嵌套的 `match`。`?` 操作符在 `Option` 上等价于 `None` 时提前返回。

### 思路

1. 每个 `match { Some(x) => ..., None => return None }` 可替换为 `let Some(x) = expr else { return None };`。
2. `?` 操作符更进一步：`let x = expr?` 自动传播 `None`。
3. 三种写法等价，但可读性差异巨大：嵌套 match → let else → ? 操作符。

### 参考实现

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

// 原版: 箭头式嵌套
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
                                        Some(ref api_key) => send_notification(&user, api_key),
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

// let else 重构版
fn notify_user_new(username: &str) -> Option<String> {
    let Some(config) = load_config() else { return None; };
    let Some(ref db_url) = config.db_url else { return None; };
    let Some(conn) = connect_db(db_url) else { return None; };
    let Some(user) = find_user(&conn, username) else { return None; };
    let Some(ref api_key) = config.api_key else { return None; };
    send_notification(&user, api_key)
}

// ? 操作符版 (Option 支持 ? 从 Rust 1.22+)
fn notify_user_try(username: &str) -> Option<String> {
    let config = load_config()?;
    let db_url = config.db_url.as_ref()?;
    let conn = connect_db(db_url)?;
    let user = find_user(&conn, username)?;
    let api_key = config.api_key.as_ref()?;
    send_notification(&user, api_key)
}

fn main() {
    println!("旧版: {:?}", notify_user_old("alice"));
    println!("新版: {:?}", notify_user_new("alice"));
    println!("?版 : {:?}", notify_user_try("alice"));
}
```

### 常见错误

- `let else` 中忘记 `return` 或 `break` —— 编译器报错 `else` 分支必须是发散型表达式（diverging）。
- `?` 操作符在 `Option` 上和 `Result` 上行为不同 —— `Option` 的 `?` 传播 `None`。
- `ref` 的用法：`Some(ref db_url)` 用于在 `config` 被借用时获取 `config.db_url` 的引用。

### 验证方式

```bash
cargo run  # 三版本输出一致
```

---

## 练习 3-1: 小型命令解析器

### 结论

综合运用 enum、match、`Option`、`Result`、`matches!`、`let else`、切片模式。模式匹配可以优雅地实现命令行 DSL 解析器。

### 参考实现

```rust
#[derive(Debug, PartialEq)]
enum Command {
    Help,
    List { filter: Option<String> },
    Create { name: String, email: Option<String> },
    Delete { name: String, force: bool },
    Unknown(String),
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        [] => None,
        ["help"] => Some(Command::Help),
        ["list"] => Some(Command::List { filter: None }),
        ["list", filter] => Some(Command::List {
            filter: Some(filter.to_string()),
        }),
        ["create", name] => Some(Command::Create {
            name: name.to_string(),
            email: None,
        }),
        ["create", name, email] => Some(Command::Create {
            name: name.to_string(),
            email: Some(email.to_string()),
        }),
        ["delete", name] => Some(Command::Delete {
            name: name.to_string(),
            force: false,
        }),
        ["delete", name, "-f"] => Some(Command::Delete {
            name: name.to_string(),
            force: true,
        }),
        [cmd, ..] => Some(Command::Unknown(cmd.to_string())),
    }
}

fn execute_command(cmd: &Command) {
    match cmd {
        Command::Help => {
            println!("可用命令:");
            println!("  help                 - 显示帮助");
            println!("  list [filter]        - 列出条目");
            println!("  create <name> [email] - 创建用户");
            println!("  delete <name> [-f]   - 删除用户");
        }
        Command::List { filter } => {
            if let Some(f) = filter {
                println!("列出条目 (过滤: {f})");
            } else {
                println!("列出所有条目");
            }
        }
        Command::Create { name, email } => {
            if let Some(e) = email {
                println!("创建用户: {name} (邮箱: {e})");
            } else {
                println!("创建用户: {name} (无邮箱)");
            }
        }
        Command::Delete { name, force } => {
            if *force {
                println!("{name} 已被强制删除");
            } else {
                println!("确认删除 {name}? 使用 -f 强制删除");
            }
        }
        Command::Unknown(cmd) => {
            println!("未知命令: {cmd}，输入 help 查看帮助");
        }
    }
}

// 可选: REPL 交互循环
// fn repl() { ... }

fn main() {
    let inputs = vec![
        "help",
        "list",
        "list active",
        "create alice",
        "create alice a@b.com",
        "delete bob",
        "delete bob -f",
        "unknown_cmd",
        "",
    ];

    for input in &inputs {
        println!("> {input}");
        let Some(cmd) = parse_command(input) else {
            println!("无法解析输入");
            continue;
        };

        if matches!(&cmd, Command::Unknown(_)) {
            println!("警告: 无法识别的命令");
        }

        execute_command(&cmd);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help() {
        assert_eq!(parse_command("help"), Some(Command::Help));
    }

    #[test]
    fn test_parse_delete_force() {
        assert_eq!(
            parse_command("delete bob -f"),
            Some(Command::Delete { name: "bob".to_string(), force: true })
        );
    }

    #[test]
    fn test_parse_unknown() {
        assert_eq!(
            parse_command("unknown"),
            Some(Command::Unknown("unknown".to_string()))
        );
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse_command(""), None);
    }

    #[test]
    fn test_parse_list_filter() {
        assert_eq!(
            parse_command("list active"),
            Some(Command::List { filter: Some("active".to_string()) })
        );
    }
}
```

**match 穷尽性说明**: 在 `parse_command` 中，`match parts.as_slice()` 的每个分支都对应一种命令模式。编译器会检查是否覆盖了所有可能的切片模式（如 `["list"]` 和 `["list", filter]` 是不同的分支）。同样在 `execute_command` 中，`match cmd` 必须覆盖所有 5 个 `Command` 变体 —— 少一个就编译报错。这种穷尽性检查保证了：当你新增一个 `Command` 变体时，编译器会提醒你必须在所有 match 处添加处理。

### 常见错误

- 切片匹配时遗漏变体组合（如忘记 `["delete", name]` 只有两个元素的情况）。
- `match parts.as_slice()` 的 `[]` 分支放错位置 —— 应在最前面或最后面，但不能被其他模式遮蔽。
- 测试中 `assert_eq!` 需要用 `PartialEq`，记得 `#[derive(Debug, PartialEq)]`。
- `Unknown` 变体用 `[cmd, ..]` 覆盖所有未匹配模式，注意它的位置可能遮蔽更具体的模式。

### 验证方式

```bash
cargo run      # 运行所有命令解析和执行
cargo test     # 5 个单元测试通过
cargo clippy   # 无 lint 警告
```

---

## 思考题: 为什么 Rust 选择 Option\<T\> 而不是 null?

### 结论

`Option<T>` 不是 null 的简单改名 —— 它通过**类型系统**和**穷尽性检查**从根本上消除了 null 引用的运行时错误。

### 回答（多角度分析）

**1. 类型安全性**: `Option<T>` 和 `T` 是**不同的类型** —— 你不能把 `Option<String>` 当作 `String` 使用。这带来的好处是：类型签名本身就表达了"这个值可能缺失"的语义。Python 的 `str | None` 只是类型提示，运行时仍然是同一个类型，IDE 可以警告但解释器不强制。Rust 的 `Option<T>` 是类型系统的一等公民 —— 编译器强制你在使用前解包。这不是更友好的 lint，而是不可绕过的规则。

**2. 编译器角色 —— match 穷尽性检查**: 编译器在处理 `match option_value { ... }` 时会检查是否覆盖了 `Some(T)` 和 `None` 两个变体。忘记处理 `None` 会直接导致**编译错误**而非运行时 panic。这相当于把 Tony Hoare 的"十亿美元错误"从运行时搬到了编译期。穷尽性检查不仅适用于 `Option`，所有 `enum` 都受此保护 —— 每当你新增一个变体，编译器会在所有 `match` 处提醒你处理新情况。

**3. 设计取舍**: Rust 选择"没有 null"的代价是：
- 学习曲线更陡（需要理解 `match`、`if let`、`unwrap`、`?` 等工具）
- 代码量更多（需要显式处理 `None` 情况）
- API 设计时需要决定是返回 `T` 还是 `Option<T>`

但收获是：
- 生产中完全消除了 `NullPointerException` / `AttributeError: 'NoneType'` 这类 bug
- 代码自文档化 —— 函数签名明确表达"可能失败"
- 强制开发者思考边界情况

**特别感激的场景**: 处理外部数据（用户输入、网络响应、文件解析）—— 这些天然包含缺失值，`Option` 迫使你优雅处理而非崩溃。

**不方便的场景**: 快速原型开发、一次性脚本 —— `.unwrap()` 的噪音较多。但 Rust 提供了 `?`、`unwrap_or`、`unwrap_or_default` 等工具减轻负担。

**4. 对比其他语言**:

| 语言 | 机制 | 强制检查? | 可绕开? |
|------|------|-----------|---------|
| Rust `Option<T>` | 枚举类型 | 编译期穷尽性检查 | 否（`.unwrap()` 在运行时 panic） |
| Swift Optional | 枚举 + 语法糖 | 编译期（解包） | 可强制解包 `!` |
| Kotlin `String?` | 类型系统级别可空 | 编译期 | 可用 `!!` 绕过 |
| TypeScript strictNullChecks | 联合类型 `T \| null` | 编译期（开启后） | 可关闭 strictNullChecks |
| Python `str \| None` | 类型提示 | 仅 lint 层面 | 是（运行时不检查） |

核心区别在于：Rust 和 Swift 的可空/可选类型是**运行时的独立类型**，而 TypeScript 的可空标记在编译到 JS 后会完全消失。Kotlin 的 `!!` 和 Swift 的 `!` 允许开发者强行绕过，Rust 的 `.unwrap()` 在语义上等价 —— 但 Rust 社区文化强烈倾向于安全的解包方式。

- **Option 不是 null 的简单改名**: null 是"所有引用类型都暗中允许的一个特殊值"；`Option<T>` 是"显式的两个可能性"。前者是隐式的、在类型系统之外的；后者是显式的、类型系统强制处理的。这是本质区别，不是命名上的差异。

---

## 迁移思维练习答案

### 1. C 中用多个 boolean 字段表达状态的代码，如何改为 Rust Enum？

将分散的 boolean 字段替换为一个 Enum，每个变体代表一个有效状态并携带该状态特有的数据。例如，将 `is_connected: bool, is_connecting: bool, peer: char*, attempt: int` 改为 `enum ConnectionState { Disconnected, Connecting { attempt: u32 }, Connected { peer: String } }`。这样做的好处是：编译器确保你永远不会在 Disconnected 状态下错误地读取 peer 字段——peer 只在 Connected 变体中存在，必须通过模式匹配解构才能访问。状态和数据之间的约束从"程序员必须记住的规矩"变成了"编译器强制执行的类型规则"。

### 2. Python 中返回 None 表示"没找到"的模式，如何改为 Option<T>？

Python 函数返回 None 时，调用者可能忘记检查而导致运行时 AttributeError（如 `result.do_something()` 报错 `'NoneType' object has no attribute 'do_something'`）。Rust 的 Option<T> 是独立的类型，与 T 类型不兼容——你不能把 Option<T> 当作 T 使用。编译器强制调用者通过 match、if let、unwrap 等方式显式处理 None 情况。这是从"运行时的君子协定"到"编译期强制验证"的根本转变。

### 3. 为什么 Rust 的 Result 和 Option 不需要 Python 那样的异常处理？

Result 和 Option 都是普通类型，不是异常机制。它们通过类型签名在函数接口层面就表达了"可能失败"或"可能为空"的语义，调用者的代码中处理这些情况是常规的控制流（match/if let），不需要 try/except 那样的特殊语法结构。这种方式让错误处理路径和正常路径在同一层控制流中可见，不像异常可以跨越多层调用栈——这提高了代码的可读性和可预测性。

---

## 练习提交检查清单

- [x] 练习 1-1: 定义了 TrafficLight enum + Display + next_light
- [x] 练习 1-2: match 处理 Option 成绩查询，穷尽性检查说明
- [x] 练习 1-3: if let 改写三种 match 模式
- [x] 练习 2-1: 事件处理系统，嵌套模式匹配 + matches! 宏
- [x] 练习 2-2: let else 重构箭头代码 + ? 操作符版本
- [x] 练习 3-1: 命令解析器 + 5 个单元测试
- [x] 思考题: Option vs null 多角度分析（含 Option 不是 null 改名和 match 穷尽性检查）

---

*枚举和模式匹配是 Rust 表达力的核心。掌握它们，你就能用类型系统精确描述问题域，让编译器帮你写"不会出错的代码"。*
