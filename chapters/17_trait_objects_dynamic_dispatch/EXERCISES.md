# 第17章练习：特征对象与动态分派

## Trait Objects & Dynamic Dispatch Exercises

---

## 难度说明

- **Level 1**：基础练习，熟悉语法和基本概念
- **Level 2**：综合练习，需要结合多个概念
- **Level 3**：挑战练习，需要深入理解和创造性思考
- **思考题**：无代码，纯粹的概念理解

---

## 练习前准备

确保在正确的目录下：

```bash
cd /home/Creeken/Temp/Rust_/rust-from-python/chapters/17_trait_objects_dynamic_dispatch
cargo build
cargo test
```

所有练习题应在 `src/main.rs` 中完成，或创建新的 `.rs` 文件并在 `main.rs` 中声明模块。

---

## Level 1 练习

### L1-1: 添加新的通知器类型 —— WechatNotifier

**目标**：理解 trait 实现和 trait 对象的基础用法。

在 `src/main.rs` 中添加一个新的结构体 `WechatNotifier`，包含字段 `openid: String`，并为其实现 `Notifier` trait。然后将它添加到 main 函数的混合通知器集合中。

**要求**：
- `WechatNotifier` 有一个 `new(openid: impl Into<String>) -> Self` 构造函数
- `notify` 方法打印 `📲 [WECHAT] 发送微信到 {openid}: {message}`
- `type_name` 返回 `"WechatNotifier"`
- 在 `main` 中创建一个 `WechatNotifier` 并将其加入 `mixed_notifiers` 的 `Vec<Box<dyn Notifier>>` 中

**验收标准**：
```bash
cargo build   # 编译通过
cargo run     # 看到 WechatNotifier 的输出
cargo test    # 现有测试不受影响
```

---

### L1-2: 理解胖指针的大小

**目标**：理解 `Box<dyn Trait>` 的内存布局。

在 `main` 函数末尾添加一段代码，使用 `std::mem::size_of` 打印以下类型的大小：
1. `EmailNotifier`（具体类型）
2. `Box<EmailNotifier>`（Box 包装具体类型）
3. `&dyn Notifier`（trait 对象引用 —— 胖指针）
4. `Box<dyn Notifier>`（Box 包装 trait 对象 —— 胖指针）
5. `String`

**预期输出参考**：
```
EmailNotifier 大小: 24 字节
Box<EmailNotifier> 大小: 8 字节
&dyn Notifier 大小: 16 字节
Box<dyn Notifier> 大小: 16 字节
String 大小: 24 字节
```

**思考**：为什么 `&dyn Notifier` 是 16 字节而 `Box<EmailNotifier>` 只有 8 字节？胖指针的"胖"在哪里？

**提示**：使用 `println!("类型名 大小: {} 字节", std::mem::size_of::<类型>());`

---

### L1-3: 统计通知器类型

**目标**：练习遍历 trait 对象集合并调用方法。

编写一个函数 `count_by_type(notifiers: &[Box<dyn Notifier>])`，遍历所有通知器，统计每种类型的数量并打印统计结果。

**函数签名**：
```rust
fn count_by_type(notifiers: &[Box<dyn Notifier>]) {
    // 提示: 使用 n.type_name() 获取类型名称
    // 可以使用 HashMap<&str, usize> 来统计
}
```

在 `main` 中调用此函数验证结果。

---

## Level 2 练习

### L2-1: 可过滤的通知管理器

**目标**：结合 trait 对象和闭包，构建可过滤的通知广播系统。

扩展 `NotificationManager`，添加一个方法 `broadcast_filtered`，它接受一个过滤闭包，只向满足条件的通知器发送消息。

**要求**：
```rust
impl NotificationManager {
    /// 只向满足 predicate 条件的通知器发送消息
    pub fn broadcast_filtered<F>(&self, message: &str, predicate: F)
    where
        F: Fn(&Box<dyn Notifier>) -> bool,
    {
        // 你的实现
    }

    /// 只向指定类型的通知器发送消息
    /// 例如: manager.broadcast_by_type("EmailNotifier", "hello");
    pub fn broadcast_by_type(&self, type_name: &str, message: &str) {
        // 提示: 复用 broadcast_filtered
    }
}
```

在 `main` 中演示：
1. 只向 EmailNotifier 发送消息
2. 只向名称中包含 "team" 的通知器发送消息

**验收标准**：
```bash
cargo build    # 编译通过
cargo test     # 添加对应测试
```

---

### L2-2: 通知器优先级系统

**目标**：理解 trait 对象的默认方法和扩展 trait 的用法。

为 `Notifier` trait 添加一个新方法 `priority(&self) -> u8`，默认返回 `5`（中等优先级）。然后修改 `send_all_dynamic` 函数，使其按优先级从高到低排序后再发送通知。

**要求**：
1. 在 `Notifier` trait 中添加 `fn priority(&self) -> u8 { 5 }`（带默认实现）
2. 为 `EmailNotifier` 覆写 `priority` 返回 `3`（低优先级）
3. 为 `SmsNotifier` 覆写 `priority` 返回 `8`（高优先级）
4. 为 `SlackNotifier` 覆写 `priority` 返回 `6`（中高优先级）
5. 编写 `send_all_by_priority(notifiers: &mut [Box<dyn Notifier>])` 函数
6. 在 `main` 中演示优先级排序效果

**提示**：使用 `notifiers.sort_by_key(|n| std::cmp::Reverse(n.priority()));`

**思考**：为什么 `priority` 可以有默认实现？这和对象安全的哪条规则相关？

---

## Level 3 练习

### L3-1: 自定义 trait 对象注册表

**目标**：构建一个完全动态的插件注册系统，结合 trait 对象、工厂模式、以及运行时配置。

设计并实现一个 `NotificationRegistry`，满足以下需求：

**设计要求**：
1. 支持通过字符串名称注册和创建通知器
2. 注册表本身不知道具体类型，完全通过 `Box<dyn Fn(&str) -> Box<dyn Notifier>>` 工厂函数工作
3. 支持从配置文件（用字符串模拟）批量创建通知器
4. 提供一个 `list_available_types()` 方法列出所有可用的通知器类型

**类型定义**：
```rust
type NotifierFactory = Box<dyn Fn(&str) -> Box<dyn Notifier>>;

pub struct NotificationRegistry {
    factories: HashMap<String, NotifierFactory>,
    // 你可以添加更多字段
}
```

**需要实现的方法**：
```rust
impl NotificationRegistry {
    /// 创建空的注册表
    pub fn new() -> Self;

    /// 注册一个通知器类型及其工厂函数
    pub fn register<F>(&mut self, type_name: &str, factory: F)
    where
        F: Fn(&str) -> Box<dyn Notifier> + 'static;

    /// 根据类型和配置创建一个通知器
    pub fn create(&self, type_name: &str, config: &str)
        -> Result<Box<dyn Notifier>, String>;

    /// 从配置字符串批量创建
    /// 格式: "type1:config1;type2:config2"
    pub fn create_batch(&self, config_str: &str)
        -> Result<Vec<Box<dyn Notifier>>, String>;

    /// 列出所有可用类型
    pub fn list_available_types(&self) -> Vec<&str>;
}
```

在 `main` 中演示：
1. 创建一个注册表
2. 注册 EmailNotifier、SmsNotifier、SlackNotifier 的工厂函数
3. 从配置字符串 `"email:alice@b.com;sms:+86-138;slack:ops,https://hook/xxx"` 批量创建
4. 广播消息到所有创建的通知器

**验收标准**：
```bash
cargo build    # 编译通过
cargo test     # 添加完整测试
cargo clippy   # 无警告
```

**提示**：
- 工厂闭包需要捕获类型信息但不需要捕获实例数据
- 使用 `'static` 生命周期限制工厂函数
- 考虑使用 `std::collections::HashMap`

---

## 思考题

### T1: 什么时候不应该使用 trait 对象？

阅读以下场景，判断应该使用 **enum**、**泛型（静态分派）** 还是 **trait 对象（动态分派）**，并解释你的理由。

**场景 A**：一个网络协议解析器，已知有 HTTP、WebSocket、gRPC 三种协议。你需要根据协议类型做不同的解析处理，并保证所有协议类型都被处理到。

**场景 B**：一个数学库的向量运算函数，需要对 `f32`、`f64`、`i32` 等数值类型提供统一的 `dot_product` 方法。性能至关重要。

**场景 C**：一个 IDE 的语言服务器，需要支持任意编程语言的语法高亮和自动补全。语言支持通过插件添加，第三方可以开发新语言插件。

**场景 D**：一个嵌入式设备的传感器读取系统，内存有限（256KB RAM），需要处理温度传感器、湿度传感器、气压传感器。传感器类型固定为 3 种。

**场景 E**：一个 Web 框架的中间件系统，用户可以编写自定义中间件，框架按顺序执行它们。

**请为每个场景回答**：
1. 推荐方案（enum / 泛型 / trait 对象）
2. 为什么？（至少 2 条理由）
3. 如果有替代方案，什么情况下你会改变选择？

---

## 推荐执行命令

```bash
# 1. 进入项目目录
cd /home/Creeken/Temp/Rust_/rust-from-python/chapters/17_trait_objects_dynamic_dispatch

# 2. 编译检查（快速验证语法）
cargo check

# 3. 编译并运行
cargo run

# 4. 运行测试
cargo test

# 5. 运行测试并显示输出
cargo test -- --nocapture

# 6. Clippy lint 检查
cargo clippy

# 7. 格式化代码
cargo fmt

# 8. 查看文档
cargo doc --open

# 9. 查看编译后二进制大小（对比静态分派的代码膨胀）
cargo build --release
ls -lh target/release/trait_objects

# 10. 查看胖指针大小（在代码中添加后运行）
cargo run | grep "大小"
```

---

## 参考答案要点（自己先完成再对照）

### L1-1 要点
- `impl Notifier for WechatNotifier` 需要实现 `notify`、`name`、`type_name` 三个方法
- 通过 `Box::new(WechatNotifier::new(...))` 加入 `Vec<Box<dyn Notifier>>`

### L1-2 要点
- 胖指针 = 数据指针(8B) + vtable 指针(8B) = 16B
- 普通指针只有数据指针(8B)，但丢失了运行时类型信息
- `Box<EmailNotifier>` 只是单指针是因为具体类型在编译时已确定

### L1-3 要点
- 遍历 `notifiers.iter()`，调用 `n.type_name()` 获取类型名
- 使用 `HashMap<&str, usize>` 或 `HashMap<String, usize>` 统计

### L2-1 要点
- 闭包参数类型：`F: Fn(&Box<dyn Notifier>) -> bool`
- `broadcast_by_type` 可以委托给 `broadcast_filtered`

### L2-2 要点
- `priority` 是默认方法，不影响对象安全
- 排序使用 `sort_by_key` 或 `sort_by`

### L3-1 要点
- 工厂函数是 `Box<dyn Fn(&str) -> Box<dyn Notifier>>`
- 需要 `'static` 生命周期，因为注册表可能比工厂函数来源活得更久
- 配置字符串解析需要注意错误处理

---

## 扩展挑战（可选）

如果你完成了所有练习，可以尝试以下扩展：

1. **添加错误恢复**：修改 `NotificationManager::broadcast`，如果一个通知器发送失败，继续执行后面的通知器而不是中止。

2. **实现并行广播**：使用 `std::thread::spawn` 或 `rayon` 并行向所有通知器发送消息。

3. **实现重试机制**：为通知器添加 `fn retry_count(&self) -> u32` 方法，失败时自动重试。

4. **实现 `std::fmt::Display` for `dyn Notifier`**：让 trait 对象可以直接被格式化打印。
   - 提示：需要为 `Notifier` 添加 `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result` 方法

5. **比较 enum vs trait 对象性能**：实现同样的通知系统分别用 enum 和 trait 对象，用 `criterion` 或 `std::time::Instant` 做基准测试。

---

## 学习检查清单

完成练习后，你应该能够回答以下问题：

- [ ] `dyn Trait` 和 `impl Trait` 的区别是什么？
- [ ] 什么是"胖指针"？它比普通指针多出了什么信息？
- [ ] 为什么 `Box<dyn Trait>` 需要堆分配？
- [ ] 什么情况下 trait 是"对象安全"的？列出至少 3 条规则。
- [ ] 泛型方法为什么不能在 trait 对象中使用？
- [ ] 什么时候选择 enum 而不是 trait 对象？
- [ ] 动态分派的性能开销主要在哪些方面？
- [ ] 如何在 trait 对象上调用需要 `Self: Sized` 的方法？
- [ ] `&dyn Trait` 和 `Box<dyn Trait>` 的所有权区别是什么？
- [ ] Python 的鸭子类型和 Rust 的 trait 对象有什么异同？

---

*祝学习愉快！记住：trait 对象的本质是用一点点运行时代价换取最大的灵活性。*
