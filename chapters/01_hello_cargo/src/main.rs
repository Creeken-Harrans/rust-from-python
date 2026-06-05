//! # Hello Cargo — 理解 Rust 工具链与项目结构
//!
//! This crate demonstrates the standard structure of a Cargo package
//! and introduces the foundational Rust toolchain concepts.
//!
//! ## Key Concepts Covered
//! - `rustc` — the Rust compiler
//! - `cargo` — the Rust build system and package manager
//! - `rustup` — the Rust toolchain installer and version manager
//! - Standard Cargo directory layout
//! - Debug vs Release builds
//!
//! ```
//! cargo build        # debug build -> target/debug/
//! cargo build --release  # release build -> target/release/
//! cargo run          # build and run (debug)
//! ```

/// Build timestamp — hardcoded at compile time to demonstrate that
/// constants can be embedded in the binary.
const BUILD_TIME: &str = "2026-06-05T18:30:00+08:00";

/// The answer to everything, computed at compile time via a `const fn`.
/// This demonstrates that Rust can perform calculations before the
/// binary ever runs.
const THE_ANSWER: i32 = compute_at_compile_time();

// ---------------------------------------------------------------------------
// Compile-time computation
// ---------------------------------------------------------------------------

/// Computes a value entirely at compile time using a `const fn`.
///
/// `const fn` (constant function) is a function that can be evaluated
/// during compilation. The compiler runs this code and embeds the result
/// directly into the final binary — zero runtime cost.
///
/// # Examples
///
/// ```
/// let answer = compute_at_compile_time();
/// assert_eq!(answer, 91);
/// ```
pub const fn compute_at_compile_time() -> i32 {
    // A deliberately verbose constant calculation to show that
    // arithmetic, conditionals, and loops are all possible inside
    // a const fn (since Rust 1.46+).
    let base: i32 = 42;
    let doubled: i32 = base * 2; // 84
    let offset: i32 = 7;
    doubled + offset // 91
}

// ---------------------------------------------------------------------------
// Toolchain information
// ---------------------------------------------------------------------------

/// Returns an explanation of the Rust toolchain components.
///
/// The Rust toolchain consists of three key tools:
///
/// | Tool   | Role                                          |
/// |--------|-----------------------------------------------|
/// |`rustc` | The Rust compiler — turns `.rs` into binaries  |
/// |`cargo` | Build system + package manager                 |
/// |`rustup`| Toolchain installer and version manager        |
///
/// # Relationship
///
/// `rustup` installs and manages `rustc` and `cargo`.
/// `cargo` orchestrates builds by calling `rustc` under the hood.
/// `rustc` does the actual compilation work.
fn get_toolchain_info() -> String {
    let mut info = String::new();

    info.push_str("═══════════════════════════════════════════\n");
    info.push_str("  Rust 工具链 (Toolchain) 三大组件\n");
    info.push_str("═══════════════════════════════════════════\n\n");

    // rustc
    info.push_str("1. rustc — Rust 编译器 (Compiler)\n");
    info.push_str("   ┌─────────────────────────────────────\n");
    info.push_str("   │ 将 .rs 源文件编译为可执行文件或库。\n");
    info.push_str("   │ 你可以直接调用它，但日常开发中几乎\n");
    info.push_str("   │ 总是通过 cargo 间接使用。\n");
    info.push_str("   │ 类比: GCC/Clang for C/C++\n");
    info.push_str("   │ 命令示例: rustc main.rs\n");
    info.push_str("   └─────────────────────────────────────\n\n");

    // cargo
    info.push_str("2. cargo — Rust 构建系统 & 包管理器\n");
    info.push_str("   ┌─────────────────────────────────────\n");
    info.push_str("   │ 依赖管理、编译、测试、发布一站式工具。\n");
    info.push_str("   │ 读取 Cargo.toml 确定项目配置。\n");
    info.push_str("   │ 内部调用 rustc 完成实际编译。\n");
    info.push_str("   │ 类比: pip + setuptools + venv for Python\n");
    info.push_str("   │       npm for Node.js\n");
    info.push_str("   │ 命令示例: cargo build, cargo run\n");
    info.push_str("   └─────────────────────────────────────\n\n");

    // rustup
    info.push_str("3. rustup — Rust 工具链安装器\n");
    info.push_str("   ┌─────────────────────────────────────\n");
    info.push_str("   │ 安装、更新、管理多版本 Rust 工具链。\n");
    info.push_str("   │ 可以同时拥有 stable / beta / nightly。\n");
    info.push_str("   │ 类比: nvm (Node), pyenv (Python)\n");
    info.push_str("   │ 命令示例: rustup update, rustup default\n");
    info.push_str("   └─────────────────────────────────────\n\n");

    info.push_str("工作流程: rustup 安装 → cargo 编排 → rustc 编译\n");
    info.push_str("          ────────        ──────        ─────\n");
    info.push_str("          版本管理        项目管理       实际工作\n");
    info
}

// ---------------------------------------------------------------------------
// Package structure explanation
// ---------------------------------------------------------------------------

/// Prints a visual representation of the standard Cargo package directory
/// layout to stdout.
///
/// Every Cargo project follows a conventional structure. Understanding
/// this layout helps you navigate any Rust codebase consistently.
///
/// # Standard Layout
///
/// ```text
/// my_project/
/// ├── Cargo.toml          ← 项目清单 (manifest)
/// ├── Cargo.lock          ← 锁定依赖版本 (自动生成)
/// ├── src/
/// │   ├── main.rs         ← 二进制箱入口 (binary crate root)
/// │   └── lib.rs           ← 库箱入口 (library crate root, 可选)
/// ├── tests/              ← 集成测试 (integration tests)
/// ├── examples/           ← 示例程序
/// ├── benches/            ← 性能基准测试
/// └── target/             ← 编译输出 (自动生成, 不提交到 git)
///     ├── debug/          ← cargo build 的输出
///     └── release/        ← cargo build --release 的输出
/// ```
fn explain_package_structure() {
    println!();
    println!("═══════════════════════════════════════════");
    println!("  标准 Cargo Package 目录结构");
    println!("═══════════════════════════════════════════");
    println!();
    println!("  hello_cargo/                  ← Package 根目录");
    println!("  ├── Cargo.toml                ← 项目清单 (manifest)");
    println!("  │                               [package] 定义元数据");
    println!("  │                               [dependencies] 声明依赖");
    println!("  ├── Cargo.lock                ← 锁定依赖精确版本 (自动生成)");
    println!("  ├── src/                      ← 源代码目录");
    println!("  │   ├── main.rs               ← 二进制箱入口 (binary crate)");
    println!("  │   └── lib.rs                ← 库箱入口 (library crate, 可选)");
    println!("  ├── tests/                    ← 集成测试 (integration tests)");
    println!("  │   └── integration_test.rs");
    println!("  ├── examples/                 ← 示例程序");
    println!("  ├── benches/                  ← 性能基准测试 (benchmarks)");
    println!("  └── target/                   ← 编译输出 (自动生成)");
    println!("      ├── debug/                ← cargo build (调试构建)");
    println!("      │   ├── hello_cargo       ← 可执行文件");
    println!("      │   └── deps/             ← 依赖编译中间产物");
    println!("      └── release/              ← cargo build --release (发布构建)");
    println!("          └── hello_cargo       ← 优化后的可执行文件");
    println!();
    println!("  💡 Cargo.toml 是你唯一必须手动编写的配置文件。");
    println!("     其他文件要么是源代码，要么是自动生成的。");
    println!();
    println!("  📦 Package (包) vs Crate (箱):");
    println!("     - Package 是 Cargo.toml 定义的完整项目（可以包含多个 crate）");
    println!("     - Crate 是 rustc 的编译单元（一个 lib.rs 或 main.rs）");
    println!("     - 一个 Package 可以包含 0-1 个 library crate + 任意个 binary crate");
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// Program entry point.
///
/// This `main` function demonstrates:
/// - Standard output (`println!`)
/// - Error output (`eprintln!`)
/// - Compile-time constants
/// - Runtime function calls
/// - The fact that the program was built with `cargo`
fn main() {
    // Log message to stderr — this is how Rust programs emit diagnostics
    // without mixing them into stdout (important for piping).
    eprintln!("[LOG]  程序启动，构建时间: {BUILD_TIME}");
    eprintln!("[LOG]  输出到 stderr 的日志不会干扰 stdout 的数据流");

    // --- Greeting ---
    println!("╔══════════════════════════════════════╗");
    println!("║     Hello, Rust!                     ║");
    println!("║     欢迎来到 Rust 工具链的世界       ║");
    println!("║     🚀 Built with cargo 🚀            ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("这个程序由 Cargo 构建。");
    println!("如果你看到了这行输出，说明 cargo build && cargo run 成功了！");
    println!();

    // --- Build info ---
    println!("═══════════════════════════════════════════");
    println!("  构建信息 (Build Info)");
    println!("═══════════════════════════════════════════");
    println!("  构建时间 (BUILD_TIME):     {BUILD_TIME}");
    println!("  编译期计算结果:             {THE_ANSWER}");
    println!("  (42 * 2 + 7 = {THE_ANSWER} — 这个值在编译期就已确定)");
    println!();

    // --- Toolchain info ---
    let info = get_toolchain_info();
    println!("{info}");

    // --- Package structure ---
    explain_package_structure();

    // --- Cargo commands reference ---
    println!("═══════════════════════════════════════════");
    println!("  常用 Cargo 命令速查");
    println!("═══════════════════════════════════════════");
    println!();
    println!("  命令                      作用");
    println!("  ──────────────────────────────────────");
    println!("  cargo check               检查代码能否编译 (不生成二进制, 速度快)");
    println!("  cargo build               调试构建 → target/debug/");
    println!("  cargo run                 构建并运行 (调试模式)");
    println!("  cargo build --release     发布构建 → target/release/ (带优化)");
    println!("  cargo clean               清理 target/ 目录");
    println!("  cargo fmt                 代码格式化 (rustfmt)");
    println!("  cargo clippy              静态检查 & 代码建议 (clippy)");
    println!("  cargo test                运行测试");
    println!("  cargo doc --open          生成并打开文档");
    println!();

    // --- eprintln vs println ---
    println!("═══════════════════════════════════════════");
    println!("  stdout vs stderr 演示");
    println!("═══════════════════════════════════════════");
    println!("  stdout  ← 你看到的大部分内容 (println!)");
    eprintln!("  stderr  ← 这一行 (eprintln!)");
    println!("  试试: cargo run 2>/dev/null  # stderr 将被丢弃");
    println!("  试试: cargo run 1>/dev/null  # stdout 将被丢弃");
    println!();

    // --- Compile-time computation demo ---
    println!("═══════════════════════════════════════════");
    println!("  编译期计算 (Const Evaluation)");
    println!("═══════════════════════════════════════════");
    println!("  const fn compute_at_compile_time() -> i32 {{");
    println!("      let base = 42;");
    println!("      let doubled = base * 2;  // 84");
    println!("      let offset = 7;");
    println!("      doubled + offset         // 91");
    println!("  }}");
    println!("  结果: {THE_ANSWER}");
    println!("  这段代码在编译时执行，运行时零开销！");
    println!();

    println!("═══════════════════════════════════════════");
    println!("  恭喜！第一章完成 🎉");
    println!("  你已经理解了 Rust 的工具链和项目结构。");
    println!("═══════════════════════════════════════════");
}
