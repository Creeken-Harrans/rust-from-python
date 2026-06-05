//! 包、箱与模块 — Rust 的代码组织系统
//!
//! 本库演示 Rust 中 **Package（包）**、**Crate（箱）** 与 **Module（模块）** 三者的关系。
//!
//! # 核心概念速览
//!
//! - **Package**：一个 Cargo.toml 描述的项目。包含 0 或 1 个 library crate，
//!   以及任意数量的 binary crate。
//! - **Crate**：编译器一次处理的代码单元。每个 `.rs` 文件在 crate root 处开始，
//!   形成一棵模块树。
//! - **Module**：`mod` 关键字声明的代码组织单元，控制作用域与可见性。
//!
//! # 本库结构
//!
//! ```text
//! src/
//! ├── lib.rs          ← library crate root（本文件）
//! ├── main.rs         ← binary crate root（独立可执行文件）
//! ├── models.rs       ← 数据模型模块
//! └── services/
//!     ├── mod.rs       ← 服务模块入口
//!     ├── parser.rs    ← 解析功能（公开）
//!     └── validator.rs ← 验证功能（部分私有）
//! ```
//!
//! # 可见性速查
//!
//! | 修饰符 | 可见范围 |
//! |--------|----------|
//! | `pub` | 所有地方 |
//! | `pub(crate)` | 当前 crate 内 |
//! | `pub(super)` | 父模块 |
//! | `pub(self)` / 无修饰符 | 当前模块 |
//! | `pub(in path)` | 指定路径 |
//!
//! # 使用示例
//!
//! ```
//! use packages_and_modules::models::{Task, Priority, create_sample_tasks};
//! use packages_and_modules::services::parse_task_line;
//!
//! let tasks = create_sample_tasks();
//! assert_eq!(tasks.len(), 4);
//!
//! if let Some((title, priority)) = parse_task_line("[High] 学习 Rust 模块系统") {
//!     println!("解析成功: {title} ({priority:?})");
//! }
//! ```

// 公开模块：外部使用者可以看到 models 和 services
pub mod models;
pub mod services;
