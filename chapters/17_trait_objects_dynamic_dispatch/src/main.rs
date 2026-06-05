#![allow(clippy::vec_init_then_push, clippy::empty_line_after_doc_comments, rustdoc::invalid_html_tags)]
// ============================================================================
// 特征对象与动态分派 (Trait Objects & Dynamic Dispatch)
// ============================================================================
// 本程序展示 trait 对象的完整用法:
// 1. 定义 trait
// 2. 实现 trait 的多种类型
// 3. 通过 Box<dyn Trait> 实现运行时多态
// 4. 对比静态分派 (泛型) 与动态分派 (trait 对象)
// 5. 工厂函数返回 trait 对象
// 6. 对象安全 (Object Safety) 规则演示
// ============================================================================

// ---------------------------------------------------------------------------
// Trait 定义: Notifier
// ---------------------------------------------------------------------------
/// 通知器 trait —— 任何可以发送通知的类型都需要实现此接口。
///
/// # 对象安全 (Object Safety)
///
/// 此 trait 是"对象安全"的，因为:
/// - 所有方法都有 `&self` 或 `&mut self` 作为接收者
/// - 没有关联类型 (associated types) 在方法返回值中使用
/// - 没有泛型方法 (generic methods)
/// - 没有返回 `Self` 的方法 (除了作为 trait 对象本身)
/// - 所有方法参数和返回值都是具体类型
///
/// 满足以上条件的 trait 可以用作 trait 对象 `dyn Notifier`。
pub trait Notifier {
    /// 发送通知消息
    fn notify(&self, message: &str);

    /// 返回通知器的名称/标识
    fn name(&self) -> &str;

    /// 返回通知器的详细描述
    fn describe(&self) -> String {
        // trait 可以提供默认实现 —— 这仍然是对象安全的
        format!("{} (type: {})", self.name(), self.type_name())
    }

    /// 返回具体类型的名称 (用于演示)
    fn type_name(&self) -> &str;
}

// ---------------------------------------------------------------------------
// Struct 1: EmailNotifier
// ---------------------------------------------------------------------------
pub struct EmailNotifier {
    pub address: String,
}

impl EmailNotifier {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("📧 [EMAIL] 发送邮件到 {}: {}", self.address, message);
    }

    fn name(&self) -> &str {
        &self.address
    }

    fn type_name(&self) -> &str {
        "EmailNotifier"
    }
}

// ---------------------------------------------------------------------------
// Struct 2: SmsNotifier
// ---------------------------------------------------------------------------
pub struct SmsNotifier {
    pub phone: String,
}

impl SmsNotifier {
    pub fn new(phone: impl Into<String>) -> Self {
        Self {
            phone: phone.into(),
        }
    }
}

impl Notifier for SmsNotifier {
    fn notify(&self, message: &str) {
        println!("📱 [SMS]  发送短信到 {}: {}", self.phone, message);
    }

    fn name(&self) -> &str {
        &self.phone
    }

    fn type_name(&self) -> &str {
        "SmsNotifier"
    }
}

// ---------------------------------------------------------------------------
// Struct 3: SlackNotifier
// ---------------------------------------------------------------------------
pub struct SlackNotifier {
    pub channel: String,
    pub webhook: String,
}

impl SlackNotifier {
    pub fn new(channel: impl Into<String>, webhook: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            webhook: webhook.into(),
        }
    }
}

impl Notifier for SlackNotifier {
    fn notify(&self, message: &str) {
        println!(
            "💬 [SLACK] 发送到频道 #{} (webhook: {}): {}",
            self.channel, self.webhook, message
        );
    }

    fn name(&self) -> &str {
        &self.channel
    }

    fn type_name(&self) -> &str {
        "SlackNotifier"
    }
}

// ============================================================================
// 静态分派 (泛型) 尝试 —— 无法编译！
// ============================================================================
// 以下函数看起来应该可以工作，但实际上无法通过编译。
// 原因: `impl Trait` 在参数位置是一种"静态多态" —— 编译器会为
// 每个被调用的具体类型生成一份独立的函数副本 (单态化/monomorphization)。
// 但这里 `&[impl Notifier]` 试图创建一个切片，其中每个元素必须
// 是**相同类型**。你无法将 `EmailNotifier` 和 `SmsNotifier`
// 放在同一个 `&[impl Notifier]` 切片中，因为编译器要求切片元素
// 的类型一致。
//
// 要混合不同类型，你需要 trait 对象 `dyn Notifier`。
// ============================================================================

// 取消注释下面这行会导致编译错误:
// fn send_all_static(notifiers: &[impl Notifier]) {
//     for n in notifiers {
//         n.notify("静态分派消息");
//     }
// }
//
// 编译错误类似于:
//   error[E0277]: the size for values of type `dyn Notifier`
//   cannot be known at compilation time
//
// 或者当你尝试传入混合类型时:
//   error[E0308]: mismatched types
//   expected `EmailNotifier`, found `SmsNotifier`

// ============================================================================
// 动态分派 (Trait 对象) —— 正常工作！
// ============================================================================
/// 使用 trait 对象实现真正的运行时多态。
///
/// `Box<dyn Notifier>` 是一个"胖指针" (fat pointer):
/// - 一个指针指向堆上的实际数据 (EmailNotifier / SmsNotifier / ...)
/// - 一个指针指向虚表 (vtable)，其中包含所有 trait 方法的函数指针
///
/// 调用 `n.notify(...)` 时，Rust 在运行时通过 vtable 查找正确的
/// 函数地址并调用它 —— 这就是"动态分派"。
pub fn send_all_dynamic(notifiers: &[Box<dyn Notifier>]) {
    println!("=== 发送消息给 {} 个通知器 (动态分派) ===", notifiers.len());
    for (i, n) in notifiers.iter().enumerate() {
        print!("  [{}/{}] ", i + 1, notifiers.len());
        n.notify("系统通知: 服务器需要维护，预计停机 30 分钟。");
    }
    println!("=== 全部发送完成 ===\n");
}

/// 使用内联 `dyn Notifier` 引用的版本 —— 对已有变量的引用。
/// `&dyn Notifier` 也是一个胖指针，但不需要堆分配。
pub fn send_all_refs(notifiers: &[&dyn Notifier]) {
    println!("=== 发送消息给 {} 个通知器 (引用方式) ===", notifiers.len());
    for n in notifiers {
        n.notify("提醒: 团队周会将在 10 分钟后开始。");
    }
    println!("=== 全部发送完成 ===\n");
}

// ============================================================================
// 工厂函数 —— 根据字符串创建不同类型的通知器
// ============================================================================
/// 工厂函数 (Factory Pattern): 根据 `kind` 参数返回不同类型的通知器。
///
/// 返回类型 `Box<dyn Notifier>` 是 trait 对象:
/// - 调用者只知道返回了一个"实现了 Notifier 的东西"
/// - 具体类型在运行时决定
/// - 这类似于 Python 中的工厂模式返回一个抽象基类实例
pub fn create_notifier(kind: &str) -> Box<dyn Notifier> {
    match kind {
        "email" => Box::new(EmailNotifier::new("admin@example.com")),
        "sms" => Box::new(SmsNotifier::new("+86-138-0000-0000")),
        "slack" => Box::new(SlackNotifier::new(
            "ops-alerts",
            "https://hooks.slack.com/services/xxx",
        )),
        _ => panic!("未知的通知器类型: '{}'", kind),
    }
}

/// 更灵活的工厂函数 —— 传入配置参数
pub fn create_notifier_with_config(kind: &str, config: &str) -> Result<Box<dyn Notifier>, String> {
    match kind {
        "email" => Ok(Box::new(EmailNotifier::new(config))),
        "sms" => Ok(Box::new(SmsNotifier::new(config))),
        "slack" => {
            let parts: Vec<&str> = config.split(',').collect();
            if parts.len() != 2 {
                return Err("Slack 需要格式: 'channel,webhook'".into());
            }
            Ok(Box::new(SlackNotifier::new(
                parts[0].trim(),
                parts[1].trim(),
            )))
        }
        _ => Err(format!("未知的通知器类型: '{}'", kind)),
    }
}

// ============================================================================
// 静态分派与动态分派对比演示
// ============================================================================

/// 静态分派: 使用泛型，所有通知器必须是同一类型
/// 优点: 零开销抽象，编译器内联优化
/// 缺点: 不能混合不同类型
pub fn send_all_static_single_type<T: Notifier>(notifiers: &[T]) {
    println!(
        "=== 静态分派 (泛型, 单一类型) 共 {} 个通知器 ===",
        notifiers.len()
    );
    for n in notifiers {
        n.notify("静态分派: 批量通知");
    }
    println!();
}

fn print_static_vs_dynamic_table() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  特性              │ 静态分派 (泛型)      │ 动态分派 (trait对象)");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  语法              │ fn foo<T: Notifier>  │ fn foo(n: &dyn Notifier)");
    println!("  分派时机          │ 编译时               │ 运行时");
    println!("  性能开销          │ 零开销 (内联可能)    │ vtable 间接调用");
    println!("  二进制大小        │ 较大 (每类型一份)    │ 较小 (单份代码)");
    println!("  混合不同类型      │ ❌ 不允许            │ ✅ 允许");
    println!("  单态化            │ ✅ 是                │ ❌ 否");
    println!("  堆分配            │ 不需要               │ Box<dyn> 需要");
    println!("  缓存局部性        │ 好 (连续内存)        │ 差 (堆上散布)");
    println!("  编译时类型检查    │ ✅ 完整              │ ⚠️ 部分丢失");
    println!("  impl Trait 参数   │ 简洁语法             │ 不适用");
    println!("  适用场景          │ 同类型集合           │ 异构集合");
    println!("  Python 类比       │ 泛型函数             │ 鸭子类型/ABC");
    println!("═══════════════════════════════════════════════════════════════");
}

// ============================================================================
// 对象安全 (Object Safety) 演示
// ============================================================================

/// 一个对象安全的 trait — 可以用作 `dyn Trait`:
///
/// ✅ 所有方法接收者: &self, &mut self, self: Box<Self>, self: Rc<Self> 等
/// ✅ 没有泛型方法
/// ✅ 没有返回 Self (除非是接收者)
/// ✅ 没有关联常量要求
///
/// 一个**不**对象安全的 trait 示例:
///
/// ```ignore
/// trait NotObjectSafe {
///     // ❌ 泛型方法 —— 编译器无法为每种 T 在 vtable 中创建条目
///     fn generic_method<T: Display>(&self, val: T);
///
///     // ❌ 返回 Self —— 编译时无法知道 Self 的大小
///     fn clone_self(&self) -> Self;
///
///     // ❌ 关联类型在方法中使用 —— 编译器无法确定类型
///     fn get_item(&self) -> Self::Item;
/// }
/// ```
///
/// 解决方案: 将这些方法移到不同的 trait 中，或使用 `where Self: Sized` 排除它们。

fn demonstrate_object_safety() {
    // Notifier trait 是对象安全的 —— 所以可以这样做:
    let _obj_safe: Box<dyn Notifier> = Box::new(EmailNotifier::new("test@test.com"));
    // ✅ 编译通过
    println!("✅ Notifier trait 是对象安全的，可以创建 Box<dyn Notifier>\n");

    // 引用方式也可以:
    let email = EmailNotifier::new("ref@test.com");
    let _ref_obj_safe: &dyn Notifier = &email;
    // ✅ 编译通过
    println!("✅ 也可以创建 &dyn Notifier 引用\n");
}

// ============================================================================
// 实际应用场景: 通知管理器
// ============================================================================

/// 通知管理器 —— 管理一组通知器的集合
pub struct NotificationManager {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifiers: Vec::new(),
        }
    }

    /// 注册一个新的通知器
    pub fn register(&mut self, notifier: Box<dyn Notifier>) {
        println!(
            "➕ 注册通知器: {} ({})",
            notifier.name(),
            notifier.type_name()
        );
        self.notifiers.push(notifier);
    }

    /// 广播消息给所有注册的通知器
    pub fn broadcast(&self, message: &str) {
        println!("\n🔊 广播消息: \"{}\"", message);
        println!("{}", "─".repeat(50));
        send_all_dynamic(&self.notifiers);
    }

    /// 列出所有已注册的通知器
    pub fn list(&self) {
        println!("\n📋 已注册的通知器列表:");
        for (i, n) in self.notifiers.iter().enumerate() {
            println!("  {}. {}", i + 1, n.describe());
        }
    }

    /// 获取注册的通知器数量
    pub fn count(&self) -> usize {
        self.notifiers.len()
    }
}

// ============================================================================
// main
// ============================================================================
fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  特征对象与动态分派 — 通知系统演示              ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ---- 创建不同类型的通知器 ----
    let email_n = EmailNotifier::new("alice@company.com");
    let sms_n = SmsNotifier::new("+86-139-1234-5678");
    let slack_n = SlackNotifier::new(
        "engineering",
        "https://hooks.slack.com/services/T00/B00/xxxx",
    );

    // ---- 演示 1: 直接调用 (静态分派) ----
    println!("--- 演示 1: 直接调用各通知器 ---");
    email_n.notify("你的 PR 已被审核通过。");
    sms_n.notify("验证码: 884921，5 分钟内有效。");
    slack_n.notify("部署完成: v2.3.1 已发布到生产环境。");
    println!();

    // ---- 演示 2: 同类型集合 —— 静态分派 ----
    println!("--- 演示 2: 静态分派 (同类型) ---");
    let email_list = vec![
        EmailNotifier::new("bob@company.com"),
        EmailNotifier::new("carol@company.com"),
        EmailNotifier::new("dave@company.com"),
    ];
    send_all_static_single_type(&email_list);

    // ---- 演示 3: 异构集合 —— 动态分派 (trait 对象) ----
    println!("--- 演示 3: 动态分派 (trait 对象, 混合类型) ---");

    // 关键: Vec<Box<dyn Notifier>> 可以存储**不同类型**的通知器
    let mut mixed_notifiers: Vec<Box<dyn Notifier>> = Vec::new();

    // 每种类型都需要 Box::new() 进行堆分配
    // Box<dyn Notifier> 是一个"胖指针": (数据指针, vtable 指针)
    mixed_notifiers.push(Box::new(EmailNotifier::new("alice@company.com")));
    mixed_notifiers.push(Box::new(SmsNotifier::new("+86-139-1234-5678")));
    mixed_notifiers.push(Box::new(SlackNotifier::new(
        "engineering",
        "https://hooks.slack.com/services/xxx",
    )));

    // 动态分派: 运行时通过 vtable 决定调用哪个 notify 实现
    send_all_dynamic(&mixed_notifiers);

    // ---- 演示 4: 使用 &dyn Notifier 引用 ----
    println!("--- 演示 4: &dyn Notifier 引用 (无堆分配) ---");
    let email2 = EmailNotifier::new("eve@company.com");
    let sms2 = SmsNotifier::new("+86-136-0000-1111");

    // &dyn Notifier 是引用到已存在的栈上变量，不需要额外堆分配
    let refs: Vec<&dyn Notifier> = vec![&email2, &sms2];
    send_all_refs(&refs);

    // ---- 演示 5: 工厂函数 ----
    println!("--- 演示 5: 工厂函数 (运行时决定具体类型) ---");
    let n1 = create_notifier("email");
    let n2 = create_notifier("sms");
    let n3 = create_notifier("slack");

    let factory_notifiers: Vec<Box<dyn Notifier>> = vec![n1, n2, n3];
    for n in &factory_notifiers {
        n.notify("工厂创建的通知器发出的消息。");
    }
    println!();

    // ---- 演示 6: 带配置的工厂函数 ----
    println!("--- 演示 6: 工厂函数 (带配置参数) ---");
    match create_notifier_with_config("email", "ops@company.com") {
        Ok(n) => n.notify("磁盘使用率达到 85%，请关注。"),
        Err(e) => eprintln!("错误: {}", e),
    }
    match create_notifier_with_config("sms", "+86-137-0000-9999") {
        Ok(n) => n.notify("服务器重启完成。"),
        Err(e) => eprintln!("错误: {}", e),
    }
    match create_notifier_with_config("slack", "ops-alerts,https://hooks.slack.com/yyy") {
        Ok(n) => n.notify("⚠️ CPU 使用率超过 90%！"),
        Err(e) => eprintln!("错误: {}", e),
    }
    println!();

    // ---- 演示 7: 通知管理器 (实际应用模式) ----
    println!("--- 演示 7: NotificationManager (实际应用) ---");
    let mut manager = NotificationManager::new();
    manager.register(Box::new(EmailNotifier::new("team@company.com")));
    manager.register(Box::new(SmsNotifier::new("+86-138-0000-1111")));
    manager.register(Box::new(SlackNotifier::new(
        "alerts",
        "https://hooks.slack.com/services/alerts",
    )));
    manager.list();
    manager.broadcast("🔴 紧急: 数据库连接池耗尽，正在自动扩容...");
    println!("通知器总数: {}", manager.count());
    println!();

    // ---- 演示 8: 对比表 ----
    println!("--- 演示 8: 静态分派 vs 动态分派 对比 ---");
    print_static_vs_dynamic_table();
    println!();

    // ---- 演示 9: 对象安全 ----
    println!("--- 演示 9: 对象安全 (Object Safety) 验证 ---");
    demonstrate_object_safety();

    println!("╔══════════════════════════════════════════════════╗");
    println!("║  程序运行完毕！                                 ║");
    println!("╚══════════════════════════════════════════════════╝");
}

// ============================================================================
// 测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_notifier() {
        let email = EmailNotifier::new("test@test.com");
        assert_eq!(email.name(), "test@test.com");
        assert_eq!(email.type_name(), "EmailNotifier");
    }

    #[test]
    fn test_trait_object_creation() {
        let n: Box<dyn Notifier> = Box::new(SmsNotifier::new("+86-000"));
        assert_eq!(n.name(), "+86-000");
        assert_eq!(n.type_name(), "SmsNotifier");
    }

    #[test]
    fn test_factory() {
        let n = create_notifier("email");
        assert_eq!(n.type_name(), "EmailNotifier");

        let n = create_notifier("sms");
        assert_eq!(n.type_name(), "SmsNotifier");
    }

    #[test]
    fn test_factory_with_config() {
        let n = create_notifier_with_config("email", "hello@world.com").unwrap();
        assert_eq!(n.name(), "hello@world.com");
    }

    #[test]
    fn test_manager_register_and_count() {
        let mut mgr = NotificationManager::new();
        assert_eq!(mgr.count(), 0);
        mgr.register(Box::new(EmailNotifier::new("a@b.com")));
        assert_eq!(mgr.count(), 1);
        mgr.register(Box::new(SmsNotifier::new("+86-111")));
        assert_eq!(mgr.count(), 2);
    }

    #[test]
    fn test_mixed_trait_objects() {
        let notifiers: Vec<Box<dyn Notifier>> = vec![
            Box::new(EmailNotifier::new("e@e.com")),
            Box::new(SmsNotifier::new("+86-123")),
        ];
        assert_eq!(notifiers.len(), 2);
        // 验证它们确实是不同类型
        assert_eq!(notifiers[0].type_name(), "EmailNotifier");
        assert_eq!(notifiers[1].type_name(), "SmsNotifier");
    }

    #[test]
    fn test_describe_default_impl() {
        let email = EmailNotifier::new("x@y.com");
        let desc = email.describe();
        assert!(desc.contains("x@y.com"));
        assert!(desc.contains("EmailNotifier"));
    }
}
