//! 二进制 crate 入口
//!
//! 本文件是一个独立的 binary crate root。
//! 它与 `lib.rs`（library crate root）属于同一个 **Package**，
//! 但编译为不同的 **Crate**。
//!
//! # 路径系统演示
//!
//! 本文件演示以下路径与导入方式：
//! - 绝对路径：`crate::`（引用 binary crate 自身）
//! - 库 crate 路径：`packages_and_modules::`（引用同包的 library crate）
//! - 相对路径：`super::`、`self::`
//! - `use` 语句的各种形式
//! - `pub`、`pub(crate)`、`pub(super)` 可见性差异
//! - 重导出（re-export）的效果

// ─── 使用库 crate 的类型 ───

// 方式 1：逐个导入
use packages_and_modules::models::create_sample_tasks;

// 方式 2：别名导入（use as）——演示将 Priority 重命名为 TaskPriority
use packages_and_modules::models::Priority as TaskPriority;

// 方式 3：通过重导出路径导入（parse_task_line 被 re-export 到 services 层级）
use packages_and_modules::services::parse_task_line;

// ─── 定义 binary crate 自身的模块 ───

/// binary crate 内部的模块，演示 `self::` 和 `super::` 相对路径。
mod app_utils {
    /// 打印模块来源信息。
    pub fn print_module_info() {
        println!("[app_utils::print_module_info] 这个函数在 binary crate 的 app_utils 模块中");
    }

    /// 返回一个问候字符串。
    pub fn greeting(name: &str) -> String {
        // self:: 引用当前模块内的函数
        let base = self::base_message();
        format!("{base}，{name}！")
    }

    fn base_message() -> String {
        "欢迎来到 Rust 模块系统演示".to_string()
    }
}

/// 演示各类路径与可见性。
fn demonstrate_paths() {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║        路径与可见性演示                    ║");
    println!("╚══════════════════════════════════════════╝\n");

    // ─── 绝对路径（crate::）───
    // crate:: 指向当前 binary crate 的根
    println!("--- 绝对路径（crate::）---");
    crate::app_utils::print_module_info();

    // ─── 库 crate 绝对路径 ───
    // 库 crate 的名字由 Cargo.toml 中的 [package].name 决定
    println!("\n--- 库 crate 绝对路径 ---");
    let tasks = packages_and_modules::models::create_sample_tasks();
    println!("通过完整路径创建了 {} 个任务", tasks.len());

    // ─── use 别名 ───
    println!("\n--- use 别名（use ... as ...）---");
    let priority = TaskPriority::High;
    println!("TaskPriority::High = {priority:?}（实际上是 models::Priority）");

    // ─── 重导出效果 ───
    // parse_task_line 原本在 services::parser，但被 pub use 提升到了 services
    println!("\n--- 重导出（re-export）---");
    if let Some((title, p)) = parse_task_line("[High] 理解模块系统") {
        println!("重导出的 parse_task_line 返回: 标题={title}, 优先级={p:?}");
    }

    // ─── 相对路径演示 ───
    println!("\n--- 相对路径 ---");
    let msg = crate::app_utils::greeting("Rust 学习者");
    println!("{msg}");

    // ─── 可见性边界 ───
    println!("\n--- 可见性演示 ---");
    println!("公开（pub）: Task, Priority, create_sample_tasks — 外部可访问");
    println!("crate 内公开（pub(crate)）: validator::sanitize — 外部 crate 不可访问");
    println!("父模块公开（pub(super)）: validator::validate_title — 仅父模块可见");
    println!("私有（默认）: trim_input — 仅当前模块可见");

    // 演示：可以直接调用 pub(crate) 的函数吗？
    // 下面这行如果取消注释，会编译失败（binary crate 和 library crate 是不同的 crate）：
    // packages_and_modules::services::validator::sanitize("test");
    // 但 parser 内部可以调用，因为它们在同一个 library crate 内

    // 演示：validator 模块本身也是私有的
    // 下面这行也会编译失败：
    // use packages_and_modules::services::validator; // ERROR: module `validator` is private

    println!("\n尝试访问私有模块/函数将导致编译错误（已注释在源码中）");
}

/// 演示任务系统的基本使用。
fn demonstrate_tasks() {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║        任务系统演示                        ║");
    println!("╚══════════════════════════════════════════╝\n");

    // 创建示例任务
    let mut tasks = create_sample_tasks();

    println!("初始任务列表（全部未完成）：");
    for task in &tasks {
        println!(
            "  [{}] {} - {}",
            task.id,
            task.title,
            if task.completed { "✓" } else { "✗" }
        );
    }

    // 将前两个任务标记为完成
    tasks[0].mark_done();
    tasks[1].mark_done();

    println!("\n标记部分完成后：");
    for task in &tasks {
        println!(
            "  [{}] {} - {}",
            task.id,
            task.title,
            if task.completed { "✓" } else { "✗" }
        );
    }

    // 解析输入
    println!("\n解析任务输入：");
    let inputs = vec![
        "[High] 提交代码审查",
        "[Medium] 编写单元测试",
        "[Low] 更新文档",
        "无效输入（无优先级标记）",
        "", // 空输入
    ];

    for input in &inputs {
        match parse_task_line(input) {
            Some((title, priority)) => {
                println!("  ✓ 解析成功: \"{title}\" — 优先级: {priority:?}");
            }
            None => {
                println!("  ✗ 解析失败: \"{input}\"（无有效优先级标记）");
            }
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║  Rust 包、箱与模块系统 - 交互式演示       ║");
    println!("║  Package: packages_and_modules            ║");
    println!("║  Library crate: src/lib.rs                ║");
    println!("║  Binary crate:  src/main.rs               ║");
    println!("╚══════════════════════════════════════════╝");

    demonstrate_paths();
    demonstrate_tasks();

    println!("\n══════════════════════════════════════════");
    println!("演示完毕！cargo run 成功运行。");
    println!("使用 cargo test 运行所有测试。");
    println!("══════════════════════════════════════════\n");
}
