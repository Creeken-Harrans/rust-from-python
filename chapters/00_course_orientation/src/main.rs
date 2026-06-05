/// 本课程的完整名称，用作横幅和引用。
const COURSE_NAME: &str = "从 Python 到 Rust —— 系统编程入门之旅";

/// 程序入口：打印课程导览的全部内容。
///
/// 调用各个辅助函数，依次展示：
/// - 课程横幅
/// - Rust 是什么
/// - Rust 的关键特性
/// - Rust 与 Python 的对比表格
/// - 不可变性演示
fn main() {
    print_banner();
    print_separator();

    let description = explain_what_is_rust();
    println!("{}", description);

    print_separator();

    println!("【Rust 五大核心特性】\n");
    show_key_features();

    print_separator();

    println!("【Rust vs Python 对照表】\n");
    compare_with_python();

    print_separator();

    demonstrate_immutability();

    print_separator();

    println!("🎉 课程导览结束。准备好踏上 Rust 之旅了吗？\n");
}

/// 打印课程横幅，包含课程名称和欢迎语。
fn print_banner() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║                                                  ║");
    println!("║   {}   ║", COURSE_NAME);
    println!("║                                                  ║");
    println!("║   第 00 章：课程导览 —— 认识 Rust 与系统编程     ║");
    println!("║                                                  ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();
}

/// 返回一段介绍 Rust 语言的文字。
///
/// 该说明涵盖了 Rust 的定位、设计目标以及它解决了什么问题，
/// 适合有 Python 背景的初学者快速理解 Rust 的独特价值。
fn explain_what_is_rust() -> String {
    let paragraph = format!(
        "Rust 是一门由 Mozilla Research 开发、现由 Rust 基金会维护的{}。\
        它的设计目标是提供与 C/C++ 同等级别的性能，同时通过一套独特的\
        {}（ownership system）在编译期消除内存错误、数据竞争等\
        常见 bug。与 Python 这类解释型语言不同，Rust 是{}，\
        代码在运行前会被编译为本地机器码，因此执行速度极快，\
        且不需要运行时（runtime）或垃圾回收器（Garbage Collector）。\n\n\
        对于 Python 程序员来说，Rust 最好的类比是：\
        \"像 Python 一样富有表现力的人体工程学设计，加上 C++ 级别的性能，\
        同时编译器像一位严格的导师，在你写出不安全的代码时给你清晰的错误信息。\"\n\n\
        在接下来的课程中，我们将从 Python 出发，一步步走进 Rust 的世界，\
        理解系统编程的核心概念，并学会如何在实际项目中结合两者的优势。",
        "系统编程语言（Systems Programming Language）",
        "所有权系统",
        "编译型语言（Compiled Language）"
    );

    paragraph
}

/// 依次打印 Rust 五大核心特性及其简要说明。
///
/// 每个特性都附带一句通俗的解释，帮助 Python 程序员建立直觉。
fn show_key_features() {
    let features = [
        (
            "Memory Safety（内存安全）",
            "Rust 在没有垃圾回收器的情况下，通过所有权和借用检查，\n   在编译期保证内存安全，杜绝悬垂指针、双重释放等问题。\n",
        ),
        (
            "No Garbage Collector（无垃圾回收器）",
            "Rust 不使用 GC，而是在编译时确定资源的生命周期，\n   实现了确定性的资源管理——无需 Stop-the-World 暂停。\n",
        ),
        (
            "Static Typing（静态类型）",
            "所有类型在编译期确定，编译器能捕获类型错误，\n   同时通过类型推导减少显式标注的负担。\n",
        ),
        (
            "Zero-Cost Abstractions（零成本抽象）",
            "高级抽象在编译后不产生额外的运行时开销——\n   迭代器、闭包等高级特性在 Release 模式下通常被完全优化掉。\n",
        ),
        (
            "Concurrency Safety（并发安全）",
            "Rust 的类型系统和所有权规则使得数据竞争在编译期就被阻止，\n   让写并发代码不再是\"与编译器搏斗\"而是\"编译器帮你检查\"。\n",
        ),
    ];

    for (i, (name, desc)) in features.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
        println!("     {}", desc);
    }
}

/// 打印一张 Rust 与 Python 的多维度对照表。
///
/// 表格从语言类型、内存管理、类型系统、性能、学习曲线等维度
/// 展示两种语言的差异与各自优势。
fn compare_with_python() {
    println!(
        "  ┌──────────────────────┬────────────────────────────┬────────────────────────────┐"
    );
    println!(
        "  │ 维度                 │ Rust                       │ Python                     │"
    );
    println!(
        "  ├──────────────────────┼────────────────────────────┼────────────────────────────┤"
    );
    println!(
        "  │ 语言分类             │ 编译型 + 系统编程语言      │ 解释型 + 脚本/通用语言     │"
    );
    println!(
        "  │ 内存管理             │ 所有权系统（编译期）       │ 引用计数 + GC              │"
    );
    println!(
        "  │ 类型系统             │ 静态强类型 + 类型推导      │ 动态强类型（duck typing）  │"
    );
    println!("  │ 运行时开销           │ 极小（无 GC，无虚拟机）   │ 有 CPython 解释器开销     │");
    println!("  │ 执行速度             │ 接近 C/C++                 │ 通常慢 10–100 倍          │");
    println!(
        "  │ 并发模型             │ 编译期保证无数据竞争       │ GIL 限制 + asyncio         │"
    );
    println!(
        "  │ 学习曲线             │ 较陡（所有权概念需要适应） │ 较平缓（语法直观）         │"
    );
    println!(
        "  │ 典型场景             │ 操作系统、嵌入式、WebAssembly│ 数据分析、Web 后端、AI/ML │"
    );
    println!(
        "  │ 包管理               │ Cargo + crates.io           │ pip + PyPI                 │"
    );
    println!(
        "  │ 错误处理             │ Result / Option（编译期）  │ 异常（运行时）             │"
    );
    println!(
        "  └──────────────────────┴────────────────────────────┴────────────────────────────┘"
    );

    println!();
    println!("  简而言之：Rust 追求「极致性能 + 编译期安全」，Python 追求「开发效率 + 灵活性」。");
    println!("  两者并非对立——后续章节你会看到它们在项目中如何互补。");
}

/// 通过 let 绑定演示 Rust 的不可变性（immutability）。
///
/// Rust 中默认所有变量都是不可变的（immutable），
/// 需要使用 `mut` 关键字显式声明可变性。
/// 这与 Python 中默认可变的语义形成鲜明对比。
fn demonstrate_immutability() {
    println!("【不可变性（Immutability）演示】\n");

    // 不可变绑定 —— 默认行为
    let message: &str = "这条消息是不可变的";
    println!("  let message = \"{}\";", message);
    println!("  此变量默认不可变。尝试修改 message 会导致编译错误。\n");

    // 可变绑定 —— 需要显式标注 mut
    let mut counter: i32 = 0;
    println!(
        "  let mut counter = {};  // 使用 mut 关键字声明可变",
        counter
    );

    for _ in 0..3 {
        counter += 1;
        println!("  counter += 1  →  counter = {}", counter);
    }

    println!();
    println!("  对比 Python：");
    println!("    Python:  x = 5; x = 10         # ✅ 默认允许重新绑定");
    println!("    Rust:    let x = 5; x = 10;    # ❌ 编译错误");
    println!("    Rust:    let mut x = 5; x = 10; # ✅ 需要 mut\n");

    // 展示 shadowing（变量遮盖）
    let value: i32 = 42;
    println!("  let value = {};", value);
    let value: &str = "现在 value 变成了字符串！";
    println!(
        "  let value = \"{}\";  ← 变量遮盖（shadowing），类型可以改变\n",
        value
    );

    println!("  变量遮盖（Shadowing）允许用同名变量覆盖旧值，");
    println!("  甚至改变类型。这与 mut 不同——shadowing 创建了全新绑定。");
}

/// 打印一条分隔线，用于美化输出排版。
fn print_separator() {
    println!("{}", "─".repeat(54));
    println!();
}
