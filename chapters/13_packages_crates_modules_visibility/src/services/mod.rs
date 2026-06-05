//! 服务模块
//!
//! 包含任务相关的业务逻辑：
//! - `parser`：解析输入的公开模块
//! - `validator`：验证输入的私有模块（演示可见性控制）

// 公开子模块：外部可以通过 `services::parser` 访问
pub mod parser;

// 私有子模块：外部无法直接访问 `services::validator`
// 这演示了 Rust 默认私有的特性
mod validator;

// 重导出（re-export）：将 parser 中的 parse_task_line 提升到 services 层级
// 外部使用者可以直接写 `use packages_and_modules::services::parse_task_line;`
pub use parser::parse_task_line;
