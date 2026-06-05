# 第17章：特征对象与动态分派

## Trait Objects & Dynamic Dispatch

---

## 目录

1. [什么是特征对象 (Trait Object)](#什么是特征对象-trait-object)
2. [dyn Trait 语法](#dyn-trait-语法)
3. [动态分派：vtable 虚表原理](#动态分派vtable-虚表原理)
4. [静态分派 vs 动态分派 对比](#静态分派-vs-动态分派-对比)
5. [Box\<dyn Trait\> —— 最常用的形式](#boxdyn-trait--最常用的形式)
6. [对象安全 (Object Safety)](#对象安全-object-safety)
7. [何时使用：enum vs 泛型 vs trait 对象](#何时使用enum-vs-泛型-vs-trait-对象)
8. [真实世界的应用场景](#真实世界的应用场景)
9. [性能考量](#性能考量)
10. [常见错误](#常见错误)
11. [与 Python 的对照](#与-python-的对照)
12. [运行本示例](#运行本示例)

---

## 什么是特征对象 (Trait Object)

**特征对象 (Trait Object)** 是 Rust 实现**运行时多态** (runtime polymorphism) 的机制。它允许你在运行时处理实现了某个 trait 的不同类型，而不需要在编译时知道具体类型。

### 核心概念

在 Rust 中，类型的大小 (size) 在编译时必须可知。但是，当我们想要存储"任何实现了 Notifier trait 的类型"时，这些具体类型的大小各不相同 (EmailNotifier、SmsNotifier、SlackNotifier 可能大小不同)。这就是 trait 对象要解决的问题。

```rust
// ❌ 不能编译：dyn Notifier 的大小在编译时未知
// let notifiers: Vec<dyn Notifier> = vec![];  // 错误！

// ✅ 使用指针：Box<dyn Notifier> — 指针的大小是已知的
let notifiers: Vec<Box<dyn Notifier>> = vec![
    Box::new(EmailNotifier::new("a@b.com")),
    Box::new(SmsNotifier::new("+86-138")),
];
```

### 为什么叫"特征对象"

- **特征 (Trait)**：定义了行为接口
- **对象 (Object)**：强调这是面向对象编程 (OOP) 中"对象"的概念 —— 你只知道它实现了某个接口，不关心具体类型

这类似于 Java 的 `interface`、C++ 的抽象基类 (通过虚函数)、Python 的 ABC (抽象基类)。

---

## dyn Trait 语法

`dyn` 关键字是 Rust 用于声明 trait 对象的语法。它显式地标记"这里正在进行动态分派"。

### 基本形式

| 形式 | 说明 | 所有权 |
|------|------|--------|
| `Box<dyn Trait>` | 堆上分配的 trait 对象，拥有所有权 | 拥有 |
| `&dyn Trait` | trait 对象的引用，不拥有所有权 | 借用 |
| `&mut dyn Trait` | trait 对象的可变引用 | 可变借用 |
| `Rc<dyn Trait>` | 引用计数的 trait 对象 | 共享拥有 |
| `Arc<dyn Trait>` | 原子引用计数的 trait 对象 | 线程安全共享 |

```rust
// Box<dyn Trait> — 堆分配，拥有所有权
let boxed: Box<dyn Notifier> = Box::new(EmailNotifier::new("a@b.com"));

// &dyn Trait — 引用栈上的值，不需要堆分配
let email = EmailNotifier::new("a@b.com");
let ref_trait: &dyn Notifier = &email;

// Arc<dyn Trait> — 多线程共享
use std::sync::Arc;
let shared: Arc<dyn Notifier> = Arc::new(EmailNotifier::new("a@b.com"));
```

### 历史变迁

- **Rust 2015**: 写 `Box<Notifier>` 即可 (没有 `dyn` 关键字)
- **Rust 2018+**: `dyn` 关键字成为必须，使动态分派更加显式
- **Rust 2021+**: 编译器会警告缺少 `dyn` 关键字的用法

```rust
// 旧风格 (Rust 2015) — 已废弃
// let n: Box<Notifier> = ...;

// 新风格 (Rust 2018+) — 推荐
let n: Box<dyn Notifier> = Box::new(EmailNotifier::new("a@b.com"));
```

---

## 动态分派：vtable 虚表原理

### 什么是动态分派

**动态分派 (Dynamic Dispatch)** 是指在运行时 (而非编译时) 决定调用哪个函数实现的过程。

### 胖指针 (Fat Pointer)

`Box<dyn Notifier>` 或 `&dyn Notifier` 是一个**胖指针**，包含两部分：

```
┌──────────────────────────────────────────────────────┐
│                  Box<dyn Notifier>                    │
├──────────────────────┬───────────────────────────────┤
│   数据指针 (8字节)    │     vtable 指针 (8字节)        │
│   指向堆上的实际数据   │    指向虚表                    │
├──────────────────────┼───────────────────────────────┤
│  EmailNotifier {     │  ┌─────────────────────────┐  │
│    address: String   │  │ vtable for EmailNotifier│  │
│  }                   │  │  notify → 0x1234        │  │
│                      │  │  name   → 0x5678        │  │
│                      │  │  type_name → 0x9ABC     │  │
│                      │  │  describe → 0xDEF0      │  │
│                      │  │  drop    → 0x1111       │  │
│                      │  │  size    → 24           │  │
│                      │  │  align   → 8            │  │
│                      │  └─────────────────────────┘  │
└──────────────────────┴───────────────────────────────┘
```

### vtable 的工作原理

1. 当编译器看到 `impl Notifier for EmailNotifier` 时，它会生成一个**虚表 (vtable)**
2. 虚表是一个包含所有 trait 方法函数指针的结构体（以及 `drop`、`size`、`align` 等元数据）
3. 当你创建 `Box::new(email_notifier)` 时，Box 中存储了两个指针：
   - 一个指向堆上的 `EmailNotifier` 数据
   - 一个指向 `EmailNotifier` 的 vtable
4. 当你调用 `n.notify(...)` 时：
   - 从 n 中获取 vtable 指针
   - 在 vtable 中找到 `notify` 条目
   - 跳转到该地址执行

```rust
// 运行时发生的事情 (简化版):
// n: Box<dyn Notifier> = { data_ptr, vtable_ptr }
// n.notify(msg)
//   1. 从 n.vtable_ptr 获取 notify 函数指针
//   2. 将 n.data_ptr 作为 &self 传入
//   3. 调用该函数
```

### 为什么叫"动态分派"

因为具体调用哪个函数**不是在编译时决定的**，而是**在运行时通过查表确定的**。编译器生成代码时只知道"这里需要一个实现了 Notifier 的东西"，具体的函数地址在运行时从 vtable 中查找。

---

## 静态分派 vs 动态分派 对比

### 概念对比表

```
══════════════════════════════════════════════════════════════════════════
  特性                │ 静态分派 (泛型/单态化)    │ 动态分派 (trait 对象)
══════════════════════════════════════════════════════════════════════════
  英文术语            │ Static Dispatch           │ Dynamic Dispatch
  ────────────────────┼───────────────────────────┼──────────────────────
  语法                │ fn foo<T: Trait>(t: T)    │ fn foo(t: &dyn Trait)
                      │ fn foo(t: impl Trait)     │ fn foo(t: Box<dyn Trait>)
  ────────────────────┼───────────────────────────┼──────────────────────
  分派时机            │ 编译时 (compile-time)      │ 运行时 (runtime)
  ────────────────────┼───────────────────────────┼──────────────────────
  实现机制            │ 单态化 (monomorphization)  │ vtable 查表
                      │ 编译器为每个具体类型        │ 胖指针 = 数据指针
                      │ 生成独立函数副本            │       + vtable 指针
  ────────────────────┼───────────────────────────┼──────────────────────
  运行时开销          │ 零开销 (zero-cost)         │ vtable 间接调用 (~1-2ns)
                      │ 可被内联优化                │ 无法内联 (编译器不知道
                      │                             │ 具体类型)
  ────────────────────┼───────────────────────────┼──────────────────────
  二进制大小          │ 较大                       │ 较小
                      │ (每具体类型一份代码)        │ (只有一份代码 + vtable)
  ────────────────────┼───────────────────────────┼──────────────────────
  编译时间            │ 较长 (更多代码生成)         │ 较短
  ────────────────────┼───────────────────────────┼──────────────────────
  堆分配              │ 不需要                     │ Box<dyn> 需要
                      │ &dyn 也不需要               │
  ────────────────────┼───────────────────────────┼──────────────────────
  混合不同类型        │ ❌ 不允许                   │ ✅ 允许
  ────────────────────┼───────────────────────────┼──────────────────────
  集合中存储          │ Vec<ConcreteType>          │ Vec<Box<dyn Trait>>
  不同实现            │ 类型必须一致                │ 类型可以不同
  ────────────────────┼───────────────────────────┼──────────────────────
  编译时类型检查      │ ✅ 完整检查                │ ⚠️ 只知道 trait 接口
  ────────────────────┼───────────────────────────┼──────────────────────
  trait 方法约束      │ 无限制                     │ 必须对象安全
                      │ 可以有泛型方法              │ 不能有泛型方法
                      │ 可以返回 Self               │ 不能返回 Self
  ────────────────────┼───────────────────────────┼──────────────────────
  Python 类比         │ @overload / 泛型            │ 鸭子类型 / ABC
  ────────────────────┼───────────────────────────┼──────────────────────
  最佳使用场景        │ - 函数库 (必须高性能)       │ - 插件系统
                      │ - 同类型批量操作            │ - 异构集合
                      │ - 不需要类型擦除时           │ - GUI 组件树
                      │                             │ - 依赖注入容器
══════════════════════════════════════════════════════════════════════════
```

### 代码示例对比

```rust
// ========== 静态分派 ==========
// 编译时：编译器知道 T = EmailNotifier，生成专门的 send_static::<EmailNotifier> 版本
fn send_static<T: Notifier>(n: &T, msg: &str) {
    n.notify(msg); // 直接调用 EmailNotifier::notify，可以被内联
}

// 使用时
let email = EmailNotifier::new("a@b.com");
send_static(&email, "hello"); // 编译时确定调用 EmailNotifier::notify

// ========== 动态分派 ==========
// 编译时：编译器只知道这是一个 "实现了 Notifier 的东西"
fn send_dynamic(n: &dyn Notifier, msg: &str) {
    n.notify(msg); // 通过 vtable 查找函数地址，然后调用
}

// 使用时
let email = EmailNotifier::new("a@b.com");
let sms = SmsNotifier::new("+86-138");
send_dynamic(&email, "hello"); // 运行时查 EmailNotifier 的 vtable
send_dynamic(&sms, "hello");   // 运行时查 SmsNotifier 的 vtable
```

---

## Box\<dyn Trait\> —— 最常用的形式

`Box<dyn Trait>` 是最常用的 trait 对象形式，原因如下：

### 为什么需要 Box

Rust 中所有变量在编译时必须有已知大小 (Sized)。`dyn Trait` 本身是 ?Sized (大小未知)，所以不能直接存放在栈上。`Box<T>` 在堆上分配内存，而 `Box` 本身是一个固定大小的指针 (8 字节或 16 字节作为胖指针)。

```rust
// ❌ 无法编译：dyn Notifier 大小未知
// let notifier: dyn Notifier = EmailNotifier::new("...");

// ❌ 也无法编译：Vec 要求元素大小已知
// let v: Vec<dyn Notifier> = vec![];

// ✅ Box<dyn Trait>：Box 的大小已知 (胖指针)
let v: Vec<Box<dyn Notifier>> = vec![
    Box::new(EmailNotifier::new("a@b.com")),
    Box::new(SmsNotifier::new("+86-138")),
];

// ✅ &dyn Trait：引用的大小已知 (胖指针)
let email = EmailNotifier::new("a@b.com");
let refs: Vec<&dyn Notifier> = vec![&email];
```

### 所有权语义

```rust
// Box<dyn Trait> 是拥有所有权的
fn create_and_return() -> Box<dyn Notifier> {
    Box::new(EmailNotifier::new("a@b.com")) // 所有权传递给调用者
}

// &dyn Trait 是借用的
fn use_temporarily(n: &dyn Notifier) {
    n.notify("临时使用");
} // n 的借用在这里结束，原始值仍然存在

// 引用的生命周期限制
let email = EmailNotifier::new("a@b.com");
let ref_n: &dyn Notifier = &email;
// ref_n 不能比 email 活得更长
```

### 与 Pin 组合

当需要自引用结构时，可以使用 `Pin<Box<dyn Trait>>`：

```rust
use std::pin::Pin;

// Pin<Box<dyn Future<Output = String>>>
let future: Pin<Box<dyn Future<Output = String>>> =
    Box::pin(async { "hello".to_string() });
```

---

## 对象安全 (Object Safety)

**对象安全 (Object Safety)** 是指一个 trait 能够被用作 trait 对象 (`dyn Trait`) 的条件。

### 对象安全的规则

一个 trait 是对象安全的，当且仅当：

1. **所有方法不能是泛型的** — trait 方法不能有自己的类型参数
2. **方法不能返回 `Self`** — 除非 `Self` 是接收者类型
3. **方法接收者必须是** `self`、`&self`、`&mut self`、`Box<Self>`、`Rc<Self>`、`Arc<Self>` 或 `Pin<P>` 其中 P 是以上类型
4. **不能有关联常量** (trait 中定义常量和类型需要通过 Sized 限制)
5. **`where Self: Sized` 可以用来排除不对象安全的方法**

### 对象安全的 trait 示例

```rust
// ✅ 对象安全
trait Notifier {
    fn notify(&self, msg: &str);           // ✅ &self
    fn name(&self) -> &str;                // ✅ &self, 返回具体类型
    fn describe(&self) -> String { ... }   // ✅ 默认实现
}
```

### 不对象安全的 trait 示例

```rust
trait NotObjectSafe {
    // ❌ 泛型方法
    fn process<T: Display>(&self, val: T);

    // ❌ 返回 Self
    fn clone_self(&self) -> Self;

    // ❌ 返回关联类型
    fn get_item(&self) -> Self::Item;
}
```

### 解决方案：使用 `where Self: Sized`

可以使用 `where Self: Sized` 来排除不对象安全的方法，使 trait 恢复对象安全：

```rust
trait PartiallyObjectSafe {
    fn method1(&self) -> String; // ✅ 对象安全

    // ❌ 不对象安全，但通过 Sized 限制排除
    fn method2(&self) -> Self where Self: Sized;

    fn generic_method<T: Display>(&self, t: T) where Self: Sized;
}

// 现在可以创建 trait 对象了
fn use_as_object(s: &dyn PartiallyObjectSafe) {
    s.method1(); // ✅ 可以调用
    // s.method2(); // ❌ 不能调用 (要求 Self: Sized)
    // s.generic_method(42); // ❌ 不能调用 (要求 Self: Sized)
}
```

### 为什么需要这些规则？

考虑 vtable 的生成：

- **泛型方法**：如果方法有类型参数 `fn foo<T>(&self, t: T)`，vtable 中需要为每种 T 生成一个条目 —— 这在编译时无法穷举，所以不允许。
- **返回 Self**：如果方法返回 `Self`，vtable 的返回类型在编译时无法确定大小。
- **关联常量**：如果 trait 有 `const N: usize`，不同实现可能有不同大小，vtable 无法处理。

---

## 何时使用：enum vs 泛型 vs trait 对象

这是 Rust 中常见的设计决策。以下决策表帮助你选择：

```
══════════════════════════════════════════════════════════════════════════
  场景                                │ 推荐方案       │ 原因
══════════════════════════════════════════════════════════════════════════
  你知道所有可能的变体                │ enum           │ 编译时穷举，模式匹配
  且在编译时固定                      │                │ 安全，无需堆分配
  ────────────────────────────────────┼────────────────┼──────────────────
  需要在运行时注册新类型              │ trait 对象     │ enum 无法扩展
  (插件系统、用户自定义类型)          │                │ trait 对象可动态添加
  ────────────────────────────────────┼────────────────┼──────────────────
  同类型的高性能批量操作              │ 泛型/静态分派   │ 零开销，可内联
  (如数值计算、序列化)                │                │ 更好的缓存局部性
  ────────────────────────────────────┼────────────────┼──────────────────
  异构集合 (需要把不同实现放一起)     │ trait 对象     │ 泛型要求集合元素
  │ 例如：Vec<不同的Notifier>         │                │ 类型一致
  ────────────────────────────────────┼────────────────┼──────────────────
  只有少数几个变体                    │ enum           │ 简单、无堆分配
  (如 2-5 个)                         │                │ 模式匹配完备性检查
  ────────────────────────────────────┼────────────────┼──────────────────
  有许多变体，而且持续增长            │ trait 对象     │ enum 越大越难维护
  ────────────────────────────────────┼────────────────┼──────────────────
  需要依赖注入 / 测试 mock            │ trait 对象     │ 可以在测试中替换实现
  ────────────────────────────────────┼────────────────┼──────────────────
  每个变体的数据差异很大              │ trait 对象     │ enum 每个变体都需要
  │                                    │                │ 携带所有可能的数据
  ────────────────────────────────────┼────────────────┼──────────────────
  编译时已知所有类型且数量少          │ enum or 泛型   │ 结合 match 获得
  │                                    │                │ 编译时穷举检查
  ────────────────────────────────────┼────────────────┼──────────────────
  库代码需要被下游扩展                │ trait 对象     │ 下游 crate 可实现
  │                                    │                │ trait 并注入
══════════════════════════════════════════════════════════════════════════
```

### 三种方案对比示例

```rust
// ========== 方案 1: enum ==========
enum Notification {
    Email { address: String },
    Sms { phone: String },
    Slack { channel: String, webhook: String },
}

impl Notification {
    fn send(&self, msg: &str) {
        match self {
            Notification::Email { address } => println!("Email to {}", address),
            Notification::Sms { phone } => println!("SMS to {}", phone),
            Notification::Slack { channel, .. } => println!("Slack to #{}", channel),
        }
    }
}
// 优点: 无堆分配，编译时穷举检查
// 缺点: 无法在外部 crate 添加新变体，match 随变体增多样式变长

// ========== 方案 2: 泛型/静态分派 ==========
fn send_all<T: Notifier>(notifiers: &[T], msg: &str) {
    for n in notifiers {
        n.notify(msg);
    }
}
// 优点: 零开销，可内联优化
// 缺点: 集合中只能有一种类型

// ========== 方案 3: trait 对象 ==========
fn send_all_dyn(notifiers: &[Box<dyn Notifier>], msg: &str) {
    for n in notifiers {
        n.notify(msg);
    }
}
// 优点: 混合不同类型，运行时扩展
// 缺点: 堆分配，vtable 间接调用
```

---

## 真实世界的应用场景

### 1. 日志框架

```rust
trait Logger {
    fn log(&self, level: LogLevel, message: &str);
}

struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>, // 可以同时输出到文件、控制台、网络
}
```

### 2. GUI 组件树

```rust
trait Widget {
    fn render(&self, ctx: &mut RenderContext);
    fn handle_event(&mut self, event: &Event);
    fn children(&self) -> &[Box<dyn Widget>];
}
```

### 3. 插件系统

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn on_load(&self);
    fn on_unload(&self);
    fn handle_request(&self, req: &Request) -> Response;
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}
```

### 4. Actix-web / Axum 中间件

```rust
// 每个中间件都是一个实现了特定 trait 的类型
// 框架使用 Vec<Box<dyn Middleware>> 存储中间件链
```

### 5. 测试中的 Mock 对象

```rust
trait Database {
    fn query(&self, sql: &str) -> Vec<Row>;
}

struct RealDb { /* ... */ }
struct MockDb { /* ... */ }

fn test_service() {
    let db: Box<dyn Database> = Box::new(MockDb::new());
    // 注入 mock 进行测试
}
```

### 6. 事件处理系统

```rust
trait EventHandler {
    fn handle(&self, event: &Event);
    fn event_type(&self) -> EventType;
}

struct EventBus {
    handlers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
}
```

---

## 性能考量

### 动态分派的性能开销

1. **vtable 间接调用** (~1-2 nanoseconds)：从 vtable 加载函数指针，然后通过指针调用。CPU 的分支预测器通常能很好地处理这种情况，但如果 vtable 条目频繁变化，可能会产生 pipeline stall。

2. **无法内联 (inlining)**：编译器不知道具体类型的 `notify` 实现是什么，因此无法将函数体内联到调用点。对于小函数（如 getter/setter），这可能是一个显著的损失。

3. **堆分配**：`Box<dyn Trait>` 需要堆分配，具有一次性的分配开销和潜在的缓存不友好访问模式。

4. **缓存局部性差**：异构集合中，不同元素散布在堆的不同位置，遍历时可能产生更多的 cache miss。

### 什么时候性能差异不重要

- I/O 密集型操作（网络请求、文件读写）—— I/O 延迟远大于 vtable 调用的开销
- 人机交互频率（GUI 事件、Web 请求）
- 不频繁调用的代码路径

### 什么时候应该避免

- 高性能数值计算（使用泛型/静态分派）
- 紧密循环中的 type-erased 操作
- 实时系统或对延迟极度敏感的场景

### 实测对比建议

使用 `cargo bench` 或 `criterion` 框架在你的具体场景中对比两种方案的性能。在绝大多数应用中，动态分派的开销可以忽略不计。

---

## 常见错误

### 错误 1: trait 不对象安全

```rust
trait MyTrait {
    fn clone_me(&self) -> Self; // ❌ 返回 Self，trait 不对象安全
}

// let x: Box<dyn MyTrait> = ...; // 编译错误！
```

**错误信息**：
```
error[E0038]: the trait `MyTrait` cannot be made into an object
  --> note: for a trait to be "object safe" it needs to allow building
            a vtable ...
  = note: method `clone_me` has a `Self` return type
```

**解决方法**：移除返回 Self 的方法，或使用 `where Self: Sized` 限制。

### 错误 2: 泛型方法在 trait 对象中

```rust
trait Serializer {
    fn serialize<T: Serialize>(&self, value: &T) -> String; // ❌ 泛型方法
}

// let s: Box<dyn Serializer> = ...; // 编译错误！
```

**解决方法**：将泛型参数移到 trait 级别，或使用 `where Self: Sized`。

### 错误 3: 忘记使用 dyn 关键字

```rust
// Rust 2015 风格 (已废弃)
// let v: Vec<Box<Notifier>> = vec![]; // 警告

// Rust 2018+ 正确风格
let v: Vec<Box<dyn Notifier>> = vec![]; // ✅
```

### 错误 4: Sized trait 限制

```rust
fn process(notifiers: &[Box<dyn Notifier>]) { // ✅
    // ...
}

// 如果你定义:
fn process_generic<T: Notifier + ?Sized>(n: &T) { // 也接受 dyn Notifier
    // ...
}
```

### 错误 5: 试图在 trait 对象上调用需要 Sized 的方法

```rust
trait Foo {
    fn method_a(&self);
    fn method_b(&self) -> Self where Self: Sized; // 需要 Sized
}

fn use_dyn(f: &dyn Foo) {
    f.method_a(); // ✅
    // f.method_b(); // ❌ 编译错误: method_b 要求 Self: Sized
}
```

### 错误 6: trait 对象的大小

```rust
use std::mem;

let email = EmailNotifier::new("a@b.com");
let trait_obj: &dyn Notifier = &email;

// 胖指针 = 数据指针 (8字节) + vtable 指针 (8字节) = 16字节
println!("&dyn Notifier 大小: {} 字节", mem::size_of::<&dyn Notifier>());
// 输出: 16
```

---

## 与 Python 的对照

对于从 Python 转过来的 Rust 学习者，以下是概念对照：

| Python 概念 | Rust 等价 | 说明 |
|-------------|-----------|------|
| 鸭子类型 | `dyn Trait` | 只要实现了方法就可以，不关心具体类型 |
| `abc.ABC` / `@abstractmethod` | `trait` | 定义接口契约 |
| `isinstance(obj, SomeABC)` | 编译时 trait bound | Rust 在编译时检查，Python 在运行时检查 |
| `Protocol` (typing.Protocol) | `trait` | 结构化类型，无需显式继承 |
| 函数的参数注解 `def foo(x: Shape)` | `fn foo(x: &dyn Shape)` | Python 只是注解提示，Rust 是强制检查 |
| 工厂函数返回基类 | 工厂函数返回 `Box<dyn Trait>` | 都支持运行时不同类型 |

### Python 示例对照

```python
# Python: 鸭子类型 / 抽象基类
from abc import ABC, abstractmethod

class Notifier(ABC):
    @abstractmethod
    def notify(self, message: str) -> None: ...
    @abstractmethod
    def name(self) -> str: ...

class EmailNotifier(Notifier):
    def __init__(self, address: str):
        self.address = address
    def notify(self, message: str):
        print(f"[EMAIL] {self.address}: {message}")
    def name(self) -> str:
        return self.address

# Python 中可以直接混合不同类型
notifiers: list[Notifier] = [
    EmailNotifier("a@b.com"),
    SmsNotifier("+86-138"),
]

for n in notifiers:
    n.notify("hello")  # 鸭子类型，运行时查找方法
```

```rust
// Rust: trait 对象 —— 类似但需要显式声明
trait Notifier {
    fn notify(&self, message: &str);
    fn name(&self) -> &str;
}

// 关键区别：Rust 需要 Box<dyn Notifier> 而不是 Notifier
let notifiers: Vec<Box<dyn Notifier>> = vec![
    Box::new(EmailNotifier::new("a@b.com")),
    Box::new(SmsNotifier::new("+86-138")),
];

for n in &notifiers {
    n.notify("hello");  // 动态分派，通过 vtable
}
```

### 关键区别

- Python 的鸭子类型没有编译时保证 —— 如果对象没有 `notify` 方法，会在运行时抛 `AttributeError`
- Rust 的 trait 系统在编译时保证所有实现了 trait 的类型都有对应的方法
- Python 不需要显式的 `Box` —— 所有对象都是堆分配的"引用语义"
- Rust 需要显式选择值语义 vs 引用语义，trait 对象需要指针包装

---

## 运行本示例

### 编译并运行

```bash
cd /home/Creeken/Temp/Rust_/rust-from-python/chapters/17_trait_objects_dynamic_dispatch
cargo run
```

### 运行测试

```bash
cargo test
```

### 预期输出

程序将依次演示：
1. 直接静态调用各通知器
2. 同类型集合的静态分派
3. 异构集合的动态分派
4. `&dyn Notifier` 引用方式
5. 工厂函数返回 trait 对象
6. 带配置的工厂函数
7. NotificationManager 实际应用
8. 静态 vs 动态分派对比表
9. 对象安全验证

### 编译检查

```bash
cargo check     # 快速检查编译
cargo clippy    # lint 检查
cargo fmt       # 格式化代码
```

---

## 小结

- **Trait 对象** (`dyn Trait`) 是 Rust 实现运行时多态的机制
- **动态分派** 通过 vtable (虚表) 在运行时查找函数地址
- **静态分派** 通过单态化在编译时确定函数调用，零开销但代码膨胀
- **`Box<dyn Trait>`** 是最常用的 trait 对象形式 (堆分配 + 所有权)
- **对象安全** 限制了哪些 trait 可以用作 trait 对象
- **选择指南**：enum 用于固定变体，泛型用于同类型高性能操作，trait 对象用于异构集合和运行时扩展

---

## 延伸阅读

- [Rust Reference: Trait objects](https://doc.rust-lang.org/reference/types/trait-object.html)
- [Rust Book: Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Rust Nomicon: Object Safety](https://doc.rust-lang.org/nomicon/object-safety.html)
- [rfcs: Object Safety reform](https://rust-lang.github.io/rfcs/2552-object-safety.html)
