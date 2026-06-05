# 练习：模式与解构

> 核心术语：Pattern 模式, Destructuring 解构, Match Guard 匹配守卫, @ Binding, Refutable Pattern 可反驳模式, Irrefutable Pattern 不可反驳模式

## 推荐编译运行命令

```bash
# 编译和运行
cargo run

# 仅检查编译（不运行）
cargo check

# 运行测试
cargo test

# 查看编译器展开的宏和模式（需要 nightly）
# rustc +nightly -Zunpretty=expanded src/main.rs
```

建议每完成一道练习后运行 `cargo run` 或 `cargo test` 验证正确性。

---

## Level 1: 基础模式匹配

### 练习 1.1 — 成绩评定系统

在 `src/main.rs` 同级目录下创建 `exercises.rs`，实现一个成绩评定函数。

**要求：**

```rust
/// 成绩评定系统
///
/// # 参数
/// - score: 0-100 之间的整数
///
/// # 返回
/// - "优秀" 当分数 >= 90
/// - "良好" 当分数 >= 75 且 < 90
/// - "及格" 当分数 >= 60 且 < 75
/// - "不及格" 当分数 < 60
///
/// # 约束
/// - 必须使用 match 配合**范围模式** (..=) 实现
/// - 至少使用一个**匹配守卫** (if guard)
fn grade(score: u32) -> &'static str {
    match score {
        // 使用范围模式
        90..=100 => "优秀",
        // 使用匹配守卫: 对特定分数给出鼓励
        n if n >= 89 => "优秀（差一分满分！）",
        // TODO: 补充 75..=89 和 60..=74 分支
        // TODO: 补充 < 60 分支
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_levels() {
        assert_eq!(grade(100), "优秀");
        assert_eq!(grade(90), "优秀");
        assert_eq!(grade(89), "优秀（差一分满分！）");
        assert_eq!(grade(80), "良好");
        assert_eq!(grade(75), "良好");
        assert_eq!(grade(70), "及格");
        assert_eq!(grade(60), "及格");
        assert_eq!(grade(59), "不及格");
        assert_eq!(grade(0), "不及格");
    }
}
```

---

### 练习 1.2 — 解构 HTTP 响应

定义一个 `HttpResponse` 枚举，然后实现一个消息格式化函数。

```rust
/// HTTP 响应枚举
#[derive(Debug)]
enum HttpResponse {
    Success { status: u16, body: String },
    Redirect { status: u16, location: String },
    ClientError { status: u16, message: String },
    ServerError { status: u16, message: String },
}

/// 将 HTTP 响应格式化为人类可读的消息
///
/// # 要求
/// - 使用结构体解构提取字段
/// - 使用**或模式**将 ClientError 和 ServerError 合并处理
/// - 对 Redirect 使用匹配守卫: 仅当 status 为 301 或 302 时
///   打印特殊信息，其他 3xx 打印通用重定向信息
fn format_response(response: &HttpResponse) -> String {
    match response {
        HttpResponse::Success { status, body } => {
            format!("成功 ({status}): {body}")
        }
        // TODO: Redirect — 使用或模式 + 匹配守卫区分 301/302 和其他
        // TODO: ClientError | ServerError — 使用或模式合并
    }
}
```

---

### 练习 1.3 — while let 栈操作

使用 `while let` 实现一个简单计算器，基于栈操作模拟 RPN（逆波兰表示法）计算。

```rust
/// 计算器操作
#[derive(Debug)]
enum Op {
    Push(i32),
    Add,
    Sub,
    Mul,
    Print,
}

/// RPN 计算器
///
/// 使用 while let 从 ops 切片依次取出操作并执行。
/// - Push(n): 将 n 压入栈
/// - Add: 弹出两个数，相加后压回
/// - Sub: 弹出两个数，相减（后弹出的减先弹出的）后压回
/// - Mul: 弹出两个数，相乘后压回
/// - Print: 打印栈顶元素
///
/// # 要求
/// - 使用 while let Some(op) = iter.next() 模式循环
/// - 在 Add/Sub/Mul 分支中使用 if let 检查栈是否有足够元素
/// - 返回最终栈顶元素（如果有的话）
fn calculate(ops: &[Op]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut iter = ops.iter();

    while let Some(op) = iter.next() {
        match op {
            Op::Push(n) => {
                stack.push(*n);
            }
            Op::Add => {
                // TODO: 使用 if let 解构，弹出两个数
                // if let (Some(a), Some(b)) = (stack.pop(), stack.pop())
            }
            Op::Sub => {
                // TODO
            }
            Op::Mul => {
                // TODO
            }
            Op::Print => {
                if let Some(top) = stack.last() {
                    println!("栈顶: {top}");
                }
            }
        }
    }

    stack.pop()
}

#[test]
fn test_calculator() {
    // 计算: 5 3 + 2 * = (5+3)*2 = 16
    let ops = vec![
        Op::Push(5),
        Op::Push(3),
        Op::Add,
        Op::Push(2),
        Op::Mul,
        Op::Print,
    ];
    assert_eq!(calculate(&ops), Some(16));

    // 计算: 10 4 - = 6
    let ops = vec![Op::Push(10), Op::Push(4), Op::Sub];
    assert_eq!(calculate(&ops), Some(6));
}
```

**提示：** `stack.pop()` 返回 `Option<i32>`，可以配合 `if let` 使用。注意 `Sub` 操作中弹出的顺序——先弹出的是右操作数，后弹出的是左操作数。

---

## Level 2: 嵌套解构与所有权

### 练习 2.1 — 配置文件解析器

设计一个嵌套的配置结构体，然后实现解析和合并逻辑。

```rust
use std::collections::HashMap;

/// 应用程序配置
#[derive(Debug, Clone)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    features: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    tls: Option<TlsConfig>,
}

#[derive(Debug, Clone)]
struct TlsConfig {
    cert_path: String,
    key_path: String,
}

#[derive(Debug, Clone)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    timeout_secs: u32,
}

/// 验证配置的合法性
///
/// # 要求
/// - 使用**嵌套结构体解构**提取所有字段
/// - 使用 @ 绑定，将 ServerConfig 绑定为变量 server
/// - 使用匹配守卫: 如果 tls 存在且 port != 443，打印警告
/// - 使用 .. 忽略不需要的字段
fn validate_config(config: &AppConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    // 使用 @ 绑定和嵌套解构
    // TODO: 实现嵌套解构
    // let AppConfig {
    //     server: server @ ServerConfig { host, port, tls, .. },
    //     database: DatabaseConfig { url, max_connections, .. },
    //     ..
    // } = config;

    // 如果启用了 TLS 但端口不是 443, 给出警告
    // 使用匹配守卫: if let Some(tls_config) = tls && *port != 443

    // 如果 max_connections == 0, 给出警告
    // 如果 host 为空, 给出警告

    warnings
}
```

---

### 练习 2.2 — 带所有权的 tree walker

实现一个简单的树结构，然后编写使用 ref 模式的遍历函数。

```rust
/// 树节点
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Node>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, children: Vec::new() }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

/// 查找树中第一个满足条件的值
///
/// # 要求
/// - 使用 **ref 模式** 在递归中借用节点（不获取所有权）
/// - 在整个函数中不能使用 .clone()
/// - 使用 if let 进行模式匹配
fn find_first<F>(node: &Node, predicate: &F) -> Option<&Node>
where
    F: Fn(i32) -> bool,
{
    // 检查当前节点
    if predicate(node.value) {
        return Some(node);
    }

    // TODO: 遍历子节点
    // 使用 for child in &node.children 循环
    // 递归调用 find_first(child, predicate)
    // 使用 if let 检查结果

    None
}

#[test]
fn test_find_first() {
    let mut root = Node::new(1);
    let mut child1 = Node::new(2);
    child1.add_child(Node::new(5));
    child1.add_child(Node::new(6));
    root.add_child(child1);
    root.add_child(Node::new(3));
    root.add_child(Node::new(4));

    let result = find_first(&root, &|v| v > 4);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 5);
}
```

**提示：** 因为函数签名接受 `&Node` 并返回 `Option<&Node>`，你不需要 `ref` 模式也能实现——但思考一下：如果接受的是 `Option<Node>` 而不是 `Option<&Node>`，该怎么做？这就是 `ref` 发挥作用的地方。

---

## Level 3: 综合练习 — 事件处理框架

### 练习 3.1 — 事件分发器

实现一个完整的事件处理系统，将所有学到的模式特性综合运用。

```rust
use std::collections::HashMap;

/// 事件类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EventType {
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
    WindowResize,
    WindowClose,
}

/// 事件数据（不同的变体携带不同的数据）
#[derive(Debug, Clone)]
enum EventData {
    Mouse { x: i32, y: i32, button: u8 },
    Keyboard { key: char, modifiers: Modifiers },
    Resize { width: u32, height: u32 },
    Close,
}

#[derive(Debug, Clone)]
struct Modifiers {
    ctrl: bool,
    alt: bool,
    shift: bool,
}

/// 完整的事件
#[derive(Debug, Clone)]
struct AppEvent {
    event_type: EventType,
    data: EventData,
    timestamp: u64,
}

/// 事件处理器 trait
trait EventHandler {
    fn handle(&self, event: &AppEvent) -> bool;
    fn name(&self) -> &str;
}

/// 事件分发器
struct EventDispatcher {
    handlers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
}

impl EventDispatcher {
    fn new() -> Self {
        EventDispatcher {
            handlers: HashMap::new(),
        }
    }

    fn register(&mut self, event_type: EventType, handler: Box<dyn EventHandler>) {
        self.handlers.entry(event_type).or_default().push(handler);
    }

    /// 分发事件给对应类型的处理器
    ///
    /// # 实现要求
    ///
    /// 1. 使用 match 匹配 event_type（穷尽所有变体）
    /// 2. 对 MouseMove 事件使用**匹配守卫**: 仅当坐标在 (0..=1920, 0..=1080) 范围内才处理
    /// 3. 对 Keyboard 事件使用**嵌套解构**: 提取 key 和 modifiers
    /// 4. 使用**或模式**: MouseDown 和 MouseUp 共享同一日志格式
    /// 5. 使用 **@ 绑定**: 绑定整个 AppEvent 用于日志
    /// 6. 对 WindowClose 事件, 在处理完成后调用所有处理器的清理逻辑
    ///
    /// 返回被触发处理的数量
    fn dispatch(&self, event: &AppEvent) -> usize {
        // TODO: 实现完整的事件分发逻辑
        0
    }
}

#[cfg(test)]
mod dispatch_tests {
    use super::*;

    struct LogHandler { name: String }

    impl EventHandler for LogHandler {
        fn handle(&self, event: &AppEvent) -> bool {
            println!("[{}] 处理事件: {:?}", self.name, event.event_type);
            true
        }
        fn name(&self) -> &str { &self.name }
    }

    #[test]
    fn test_dispatch_basic() {
        let mut dispatcher = EventDispatcher::new();
        dispatcher.register(
            EventType::MouseDown,
            Box::new(LogHandler { name: "mouse_logger".into() }),
        );

        let event = AppEvent {
            event_type: EventType::MouseDown,
            data: EventData::Mouse { x: 100, y: 200, button: 1 },
            timestamp: 1000,
        };

        let count = dispatcher.dispatch(&event);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_mouse_move_bounds_check() {
        let mut dispatcher = EventDispatcher::new();
        dispatcher.register(
            EventType::MouseMove,
            Box::new(LogHandler { name: "move_logger".into() }),
        );

        // 超出屏幕范围, 不应被处理（守卫过滤）
        let event = AppEvent {
            event_type: EventType::MouseMove,
            data: EventData::Mouse { x: -10, y: 500, button: 0 },
            timestamp: 2000,
        };
        assert_eq!(dispatcher.dispatch(&event), 0);

        // 在屏幕范围内，应被处理
        let event = AppEvent {
            event_type: EventType::MouseMove,
            data: EventData::Mouse { x: 500, y: 500, button: 0 },
            timestamp: 2000,
        };
        assert_eq!(dispatcher.dispatch(&event), 1);
    }

    #[test]
    fn test_keyboard_modifiers() {
        let mut dispatcher = EventDispatcher::new();
        dispatcher.register(
            EventType::KeyDown,
            Box::new(LogHandler { name: "key_logger".into() }),
        );

        let event = AppEvent {
            event_type: EventType::KeyDown,
            data: EventData::Keyboard {
                key: 's',
                modifiers: Modifiers { ctrl: true, alt: false, shift: false },
            },
            timestamp: 3000,
        };

        assert_eq!(dispatcher.dispatch(&event), 1);
    }
}
```

---

## 思考题

### 为什么 Rust 编译器要求 `let` 使用不可反驳模式？

Rust 的设计者为什么决定 `let` 语句只能接受不可反驳模式（irrefutable pattern），而把可反驳模式限制在 `match`、`if let`、`while let` 中？如果允许 `let Some(x) = maybe_value` 在匹配失败时自动 panic，会带来什么问题？

请从以下角度分析：

1. **类型安全**: 如果 `let` 允许可反驳模式，类型系统会受到什么影响？
2. **错误处理哲学**: Rust 倾向于显式错误处理（Result）还是隐式 panic？
3. **代码可读性**: 在大型项目中，`let` 的可反驳模式会如何影响代码审查？
4. **与其他语言对比**: Swift 的 `guard let`、Kotlin 的 `?.let` 如何处理这个问题？Rust 的方案有何优劣？

---

## 练习完成检查清单

在提交前，确认以下所有项：

- [ ] 练习 1.1: `grade()` 使用范围模式和匹配守卫，所有测试通过
- [ ] 练习 1.2: `format_response()` 使用结构体解构、或模式、匹配守卫
- [ ] 练习 1.3: 计算器使用 `while let` 和 `if let`，测试通过
- [ ] 练习 2.1: `validate_config()` 使用嵌套解构、@ 绑定、匹配守卫
- [ ] 练习 2.2: `find_first()` 正确处理所有权，不使用 `.clone()`
- [ ] 练习 3.1: `dispatch()` 综合使用了所有要求的模式特性，测试通过
- [ ] 思考题: 写下了自己的理解（至少 100 字）
- [ ] 所有代码通过 `cargo check` 无错误
- [ ] 所有测试通过 `cargo test` 无失败

---

祝编码愉快！
