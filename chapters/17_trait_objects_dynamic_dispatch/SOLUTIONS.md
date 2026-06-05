# 第17章 练习答案 — 特征对象与动态分派

> 核心概念：**trait 对象的本质是用一点点运行时代价（vtable 间接调用 + 堆分配）换取最大的灵活性（异构集合、运行时类型选择）。**

---

## 静态分派 vs 动态分派速查

| 特性 | 静态分派（泛型） | 动态分派（trait 对象） |
|------|-----------------|----------------------|
| 语法 | `fn foo<T: Notifier>(t: &T)` | `fn foo(t: &dyn Notifier)` |
| 分派时机 | 编译时（单态化） | 运行时（vtable） |
| 性能 | 零开销，可内联 | vtable 间接调用 |
| 二进制大小 | 较大（每类型一份副本） | 较小（单份代码） |
| 混合不同类型 | ❌ 不允许 | ✅ 允许 |
| 堆分配 | 不需要 | `Box<dyn>` 需要 |
| 胖指针 | 否（8 字节） | 是（16 字节：数据指针 + vtable 指针） |
| 适用场景 | 同类型集合、性能热点 | 异构集合、插件系统、运行时类型选择 |

---

## Level 1 练习

### L1-1: 添加新的通知器类型 —— WechatNotifier

#### 结论

为新类型实现 `Notifier` trait，然后通过 `Box::new(...)` 包装后加入 `Vec<Box<dyn Notifier>>`。

#### 参考实现

```rust
pub struct WechatNotifier {
    openid: String,
}

impl WechatNotifier {
    pub fn new(openid: impl Into<String>) -> Self {
        Self {
            openid: openid.into(),
        }
    }
}

impl Notifier for WechatNotifier {
    fn notify(&self, message: &str) {
        println!("📲 [WECHAT] 发送微信到 {}: {}", self.openid, message);
    }

    fn name(&self) -> &str {
        &self.openid
    }

    fn type_name(&self) -> &str {
        "WechatNotifier"
    }
}

// 在 main() 中将 WechatNotifier 加入 mixed_notifiers:
mixed_notifiers.push(Box::new(WechatNotifier::new("oABCD12345")));
```

#### 为什么这样设计

- `impl Notifier for WechatNotifier` 与其他类型的 trait 实现完全独立——体现了 Rust 的开放类型系统
- `Box::new(...)` 将具体类型擦除为 `Box<dyn Notifier>`（类型擦除）
- 调用方只通过 `Notifier` trait 的方法交互，不知道也不需要知道具体类型

#### 常见错误

- 忘记实现 `name()` 和 `type_name()` 方法
- `Box::new(WechatNotifier::new(...))` 的类型是 `Box<WechatNotifier>`，但可以被自动强制为 `Box<dyn Notifier>`（unsized coercion）

#### 验证方式

```bash
cargo run
# 应看到 📲 [WECHAT] 发送微信的日志
```

---

### L1-2: 理解胖指针的大小

#### 结论

`&dyn Notifier` 和 `Box<dyn Notifier>` 都是 **16 字节**（64 位平台），因为它们都是 "胖指针" (fat pointer)：
- 第一个 8 字节：指向实际数据的指针
- 第二个 8 字节：指向虚表 (vtable) 的指针

普通指针（如 `Box<EmailNotifier>`）只有 8 字节，因为具体类型在编译时已知，不需要 vtable。

#### 参考实现

```rust
use std::mem::size_of;

fn print_type_sizes() {
    println!("EmailNotifier 大小:       {} 字节", size_of::<EmailNotifier>());
    println!("Box<EmailNotifier> 大小:  {} 字节", size_of::<Box<EmailNotifier>>());
    println!("&dyn Notifier 大小:       {} 字节", size_of::<&dyn Notifier>());
    println!("Box<dyn Notifier> 大小:   {} 字节", size_of::<Box<dyn Notifier>>());
    println!("String 大小:              {} 字节", size_of::<String>());
}
```

**预期输出**（64 位平台）：
```
EmailNotifier 大小:       24 字节
Box<EmailNotifier> 大小:  8 字节
&dyn Notifier 大小:       16 字节
Box<dyn Notifier> 大小:   16 字节
String 大小:              24 字节
```

#### 为什么胖指针是 16 字节

```
&dyn Notifier 的内存布局:
┌──────────────────────────────┬──────────────────────────────┐
│  数据指针 (8 bytes)          │  vtable 指针 (8 bytes)       │
│  指向 EmailNotifier 的地址   │  指向 Notifier vtable        │
│  或 SmsNotifier 的地址       │  (包含 notify/name/... 等    │
│                              │   函数指针)                  │
└──────────────────────────────┴──────────────────────────────┘
```

vtable 内容（概念性）：
```
Notifier vtable for EmailNotifier:
  notify    → <EmailNotifier as Notifier>::notify
  name      → <EmailNotifier as Notifier>::name
  describe  → <EmailNotifier as Notifier>::describe
  type_name → <EmailNotifier as Notifier>::type_name
  drop      → <EmailNotifier as std::ops::Drop>::drop  (如果是 Box<dyn>)
  size      → size_of::<EmailNotifier>()
  align     → align_of::<EmailNotifier>()
```

#### 常见错误

- 混淆 `Box<EmailNotifier>`（8 字节，不带 vtable）和 `Box<dyn Notifier>`（16 字节，带 vtable）
- 误以为 `&dyn Notifier` 包含被引用数据的大小——vtable 中存有 size/align 信息

#### 验证方式

```bash
cargo run | grep "大小"
```

---

### L1-3: 统计通知器类型

#### 结论

遍历 trait 对象集合，调用 `type_name()` 获取类型标识，使用 `HashMap` 统计。

#### 参考实现

```rust
use std::collections::HashMap;

fn count_by_type(notifiers: &[Box<dyn Notifier>]) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for n in notifiers {
        *counts.entry(n.type_name()).or_insert(0) += 1;
    }

    println!("\n📊 通知器类型统计:");
    for (type_name, count) in &counts {
        println!("  {}: {} 个", type_name, count);
    }
}

// 在 main() 中调用:
count_by_type(&mixed_notifiers);
```

#### 为什么这样设计

- `type_name()` 返回 `&str`，不需要堆分配
- `HashMap` 的 `entry().or_insert()` 模式简洁高效
- 直接遍历 `&[Box<dyn Notifier>]`，通过动态分派调用 `type_name()`

#### 常见错误

- 使用 `HashMap<String, usize>` 而非 `HashMap<&str, usize>`（不必要的堆分配）
- 忘记 `*counts.entry(...)` 的解引用

#### 验证方式

```bash
cargo run
# 应看到类型统计输出
```

---

## Level 2 练习

### L2-1: 可过滤的通知管理器

#### 结论

`broadcast_filtered` 接受一个闭包 `F: Fn(&Box<dyn Notifier>) -> bool` 作为过滤条件。`broadcast_by_type` 可以直接委托给 `broadcast_filtered`。

#### 参考实现

```rust
impl NotificationManager {
    pub fn broadcast_filtered<F>(&self, message: &str, predicate: F)
    where
        F: Fn(&Box<dyn Notifier>) -> bool,
    {
        println!("\n🔊 过滤广播: \"{}\"", message);
        println!("{}", "─".repeat(50));
        for n in &self.notifiers {
            if predicate(n) {
                n.notify(message);
            }
        }
    }

    pub fn broadcast_by_type(&self, type_name: &str, message: &str) {
        self.broadcast_filtered(message, |n| n.type_name() == type_name);
    }
}

// 在 main() 中演示:
manager.broadcast_by_type("EmailNotifier", "仅邮件通知");
manager.broadcast_filtered("团队消息", |n| n.name().contains("team"));
```

#### 为什么这样设计

- 闭包参数类型是 `&Box<dyn Notifier>`：对 `Box<dyn Notifier>` 的引用，而不是 `&dyn Notifier`
- `broadcast_by_type` 通过闭包复用 `broadcast_filtered`——DRY 原则
- 闭包捕获了 `type_name` 参数（不可变借用），编译器推导为 `Fn` trait

#### 常见错误

- 闭包参数类型写 `&dyn Notifier` 而不是 `&Box<dyn Notifier>`（因为集合存储的是 `Box<dyn Notifier>`）
- 忘记 `move` 关键字（如果闭包需要捕获所有权）

#### 验证方式

```bash
cargo test
cargo run
```

---

### L2-2: 通知器优先级系统

#### 结论

在 trait 中添加默认方法不影响对象安全。对象安全的规则是：方法接收者是 `self`、`&self`、`&mut self`、`Box<Self>` 等，且没有泛型参数的方法都是对象安全的。`priority()` 有 `&self` 接收者、返回具体类型 `u8`、有默认实现——完全对象安全。

#### 参考实现

```rust
pub trait Notifier {
    fn notify(&self, message: &str);
    fn name(&self) -> &str;
    fn describe(&self) -> String {
        format!("{} (type: {})", self.name(), self.type_name())
    }
    fn type_name(&self) -> &str;

    /// 优先级: 1-10，数字越大优先级越高，默认 5
    fn priority(&self) -> u8 {
        5
    }
}

// 各类型覆写:
impl Notifier for EmailNotifier {
    fn priority(&self) -> u8 { 3 }  // 低优先级
    // ... 其余方法不变
}

impl Notifier for SmsNotifier {
    fn priority(&self) -> u8 { 8 }  // 高优先级
    // ...
}

impl Notifier for SlackNotifier {
    fn priority(&self) -> u8 { 6 }  // 中高优先级
    // ...
}

// 按优先级发送:
fn send_all_by_priority(notifiers: &mut [Box<dyn Notifier>]) {
    notifiers.sort_by_key(|n| std::cmp::Reverse(n.priority()));
    for n in notifiers {
        n.notify("按优先级排序的通知");
    }
}
```

#### 为什么 `priority()` 可以有默认实现且不影响对象安全？

对象安全的核心要求是：编译器能够在 vtable 中为每个 trait 方法创建一个条目。默认方法是**可以被放入 vtable 的**（如果没有被覆写，vtable 条目指向默认实现）。真正导致对象不安全的是：
- 泛型方法（编译器无法为无限种 `T` 创建 vtable 条目）
- 返回 `Self` 的方法（编译器在编译 trait 对象时不知道具体类型大小）
- 没有 `self` 接收者的关联函数（无法通过 vtable 调用）

`priority()` 不符合上述任何一条，所以完全对象安全。

#### 常见错误

- 排序时忘记 `std::cmp::Reverse`（默认是升序）
- `sort_by_key` 需要 `notifiers` 是 `&mut` 引用

#### 验证方式

```bash
cargo run
# 应看到按优先级排序的输出
```

---

## Level 3 练习

### L3-1: 自定义 trait 对象注册表

#### 结论

`NotificationRegistry` 是完全动态的工厂系统。工厂函数类型 `Box<dyn Fn(&str) -> Box<dyn Notifier>>` 需要 `'static` 生命周期，因为注册表可能比工厂闭包的创建者活得更久。

#### 参考实现

```rust
use std::collections::HashMap;

type NotifierFactory = Box<dyn Fn(&str) -> Box<dyn Notifier>>;

pub struct NotificationRegistry {
    factories: HashMap<String, NotifierFactory>,
}

impl NotificationRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, type_name: &str, factory: F)
    where
        F: Fn(&str) -> Box<dyn Notifier> + 'static,
    {
        self.factories
            .insert(type_name.to_string(), Box::new(factory));
    }

    pub fn create(&self, type_name: &str, config: &str)
        -> Result<Box<dyn Notifier>, String>
    {
        self.factories
            .get(type_name)
            .ok_or_else(|| format!("未知的通知器类型: '{}'", type_name))
            .map(|factory| factory(config))
    }

    pub fn create_batch(&self, config_str: &str)
        -> Result<Vec<Box<dyn Notifier>>, String>
    {
        let mut results = Vec::new();
        for part in config_str.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let (type_name, config) = part
                .split_once(':')
                .ok_or_else(|| format!("配置格式错误: '{}'", part))?;
            results.push(self.create(type_name.trim(), config.trim())?);
        }
        Ok(results)
    }

    pub fn list_available_types(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
}

// 在 main() 中使用:
let mut registry = NotificationRegistry::new();

// 注册工厂
registry.register("email", |config: &str| {
    Box::new(EmailNotifier::new(config))
});
registry.register("sms", |config: &str| {
    Box::new(SmsNotifier::new(config))
});
registry.register("slack", |config: &str| {
    let parts: Vec<&str> = config.split(',').collect();
    Box::new(SlackNotifier::new(parts[0].trim(), parts[1].trim()))
});

println!("可用类型: {:?}", registry.list_available_types());

// 批量创建
let notifiers = registry
    .create_batch("email:alice@b.com;sms:+86-138;slack:ops,https://hook/xxx")
    .unwrap();

for n in &notifiers {
    n.notify("来自注册表的批量通知");
}
```

#### 为什么需要 `'static` 生命周期约束？

```rust
F: Fn(&str) -> Box<dyn Notifier> + 'static
```

工厂函数被存储在 `NotificationRegistry` 中，其生命周期需要覆盖注册表的整个生命期。如果工厂函数捕获了某个局部变量的引用，那个引用可能在使用工厂前就失效了。`'static` 约束确保了工厂函数不借任何会提前失效的数据（它可以拥有自己的数据）。

如果工厂函数确实需要捕获数据，可以让它获取所有权而不是引用：
```rust
// 可行：捕获 String（拥有所有权），满足 'static
let prefix = String::from("PREFIX_");
registry.register("email", move |config: &str| {
    Box::new(EmailNotifier::new(format!("{}{}", prefix, config)))
});
```

#### 常见错误

- 忘记 `'static` 约束，编译器报错 "borrowed data escapes"
- `create_batch` 中 `split_once` 返回 `Option`，需要用 `ok_or_else` 转换
- `list_available_types` 返回 `Vec<&str>` 时 key 是 `String`，需要 `as_str()`

#### 验证方式

```bash
cargo build
cargo test
cargo clippy
```

---

## 思考题 T1：什么时候不应该使用 trait 对象？

### 场景 A：网络协议解析器（HTTP、WebSocket、gRPC 三种固定协议）

**推荐方案：enum**

理由：
1. 协议类型**封闭固定**（只有 3 种），不需要扩展点
2. enum 可以**穷尽匹配**：`match protocol { Http => ... , WebSocket => ... , gRPC => ... }`，编译器确保处理了所有类型
3. 不需要堆分配，性能更好
4. enum 可以放在栈上、嵌入其他结构体，内存局部性更好

如果未来需要第三方扩展新协议，可以改为 trait 对象方案。

### 场景 B：数学库的向量运算（f32、f64、i32 等数值类型）

**推荐方案：泛型（静态分派）**

理由：
1. 性能至关重要——数学运算在热点路径上，动态分派的 vtable 开销不可接受
2. 编译器可以将泛型代码内联，利用 SIMD 指令优化
3. 这些类型在编译时就已知，不需要运行时多态
4. 单态化虽然增大二进制，但对数值运算库来说是可接受的

**不会**选择 trait 对象，即使有 100 种数值类型也不会——性能损失乘以调用频率是巨大的。

### 场景 C：IDE 语言服务器（支持任意编程语言插件）

**推荐方案：trait 对象（动态分派）**

理由：
1. 语言类型的集合**开放无界**——第三方可以开发新语言插件
2. IDE 框架不知道未来有哪些语言，但知道它们的接口（`highlight()`、`complete()` 等）
3. enum 无法涵盖未知的扩展类型
4. 用户交互频率低（毫秒级），vtable 开销可忽略

这正是 IntelliJ、VS Code 等 IDE 的插件架构模式。

### 场景 D：嵌入式设备传感器（3 种固定传感器，256KB RAM）

**推荐方案：enum**

理由：
1. 传感器类型固定为 3 种——enum 胜任
2. **内存有限**（256KB）——trait 对象需要堆分配（Box），在嵌入式环境中是奢侈的
3. 堆分配在嵌入式设备上可能根本不可用（no_std 环境）
4. 穷尽匹配保证所有传感器类型都被处理

### 场景 E：Web 框架中间件系统

**推荐方案：trait 对象（动态分派）**

理由：
1. 中间件类型**开放**——用户可以编写自定义中间件
2. 框架需要按顺序执行不同类型的中间件（异构集合）
3. Web 请求的延迟主要由 IO 决定，vtable 开销微不足道
4. 中间件链是典型的 "责任链模式"，trait 对象是 Rust 中实现此模式的惯用方式

---

## 对象安全 (Object Safety) 规则总结

一个 trait 是对象安全的（可以作为 `dyn Trait` 使用），当且仅当：

1. **所有方法的接收者是 `self`、`&self`、`&mut self`、`Box<Self>`、`Rc<Self>`、`Arc<Self>` 等**
   - `fn foo() -> Self` 不行（没有 self 接收者）

2. **方法不包含泛型参数**
   - `fn generic<T>(&self, x: T)` 不行
   - 但 `fn concrete(&self, x: i32)` 可以

3. **方法不返回 `Self`（除了作为接收者）**
   - `fn clone(&self) -> Self` 不行（`Self` 的大小在编译 trait 对象时未知）
   - 但 `fn name(&self) -> &str` 可以

4. **Trait 不包含关联常量（在方法签名中使用时）**
   - Rust 2021 起 trait 可以有 `const` 关联项，但不影响对象安全

**解决方法**：
- 添加 `where Self: Sized` 排除不对象安全的方法：`fn clone_self(&self) -> Self where Self: Sized;`
- 这些方法在 trait 对象上不可用，但在具体类型上仍可用

---

## 学习检查清单答案

- `dyn Trait` 和 `impl Trait` 的区别：`dyn Trait` 是运行时多态（类型擦除），`impl Trait` 是编译期多态（单态化）
- 胖指针多出：vtable 指针（8 字节），包含所有 trait 方法的函数指针和数据布局信息（size/align/drop）
- `Box<dyn Trait>` 需要堆分配：因为 trait 对象大小在编译期不固定（不同类型有不同大小），必须在堆上分配
- 对象安全 3 条规则：见上文
- 泛型方法不能在 trait 对象中使用：编译器无法为每种可能的 `T` 创建 vtable 条目
- 选择 enum 而非 trait 对象：类型集合封闭固定、需要穷尽匹配、内存受限环境
- 动态分派开销：vtable 间接调用（可能 cache miss）、无法内联、堆分配
- `Self: Sized` 限制：被排除的方法不能在 trait 对象上调用，但可以在具体类型上
- `&dyn Trait` vs `Box<dyn Trait>`：`&dyn` 借用已有变量（无堆分配），`Box<dyn>` 拥有数据（堆分配 + 自动 Drop）
- Python 鸭子类型 vs Rust trait 对象：Python 在运行时查找方法（可能 AttributeError），Rust trait 对象通过 vtable 保证方法存在（编译期验证）
