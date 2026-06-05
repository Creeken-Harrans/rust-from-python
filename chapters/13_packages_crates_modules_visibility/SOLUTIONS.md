# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：`use` 导入练习

**结论**：`use` 是路径别名，不是文本包含。`mod` 声明模块层级，`use` 缩短路径。

```rust
// 绝对路径 vs 相对路径
use std::collections::HashMap;          // 绝对路径
use crate::models::User;                // crate 根开始
use super::sibling_module;              // 父模块中的兄弟
use self::child_module;                 // 当前模块的子模块
```

与 C `#include` 的区别：`#include` 是预处理器文本粘贴，`use` 是编译器层面的符号导入，不影响编译单元。

与 Python `import` 的区别：Python `import` 加载并执行整个模块，Rust `use` 只是符号可见性。

---

### 1-2：模块可见性

```rust
pub fn public_api() {}       // 任何地方可见
pub(crate) fn crate_api() {} // 整个 crate 内可见
pub(super) fn parent_api() {}// 仅父模块可见
fn private_fn() {}           // 仅当前模块（默认私有）
```

Rust 默认私有（与 Python/C++ 不同），这鼓励明确设计公共 API 边界。

---

### 1-3：多文件模块

```
src/
├── main.rs       // crate root
├── lib.rs        // library root
├── models/
│   ├── mod.rs    // pub mod models 的入口
│   ├── user.rs
│   └── post.rs
└── services/
    ├── mod.rs
    └── api.rs
```

与 Python 的差异：Rust 需要显式 `mod` 声明，文件名和模块名不强制关联（虽然惯例一致）。

---

## Level 2：组合应用

### 2-1：库 crate 设计

```rust
// lib.rs
pub mod models;
pub mod services;

// models/mod.rs
pub mod user;
pub use user::User;

// models/user.rs
pub struct User {
    pub name: String,
    email: String,  // 私有字段，通过方法访问
}
impl User {
    pub fn email(&self) -> &str { &self.email }
}
```

#### 设计理由

- `pub use user::User`（re-export）：外部直接用 `crate::User`，不需知道内部文件结构
- 私有字段 + getter：封装内部表示，允许未来改变字段类型而不影响消费者
- 与 Python `__init__.py` 的 `from .user import User` 类似

---

### 2-2：Workspace 成员

答案在 ch26 详细讨论。简言之：Workspace 成员是独立的 Package，共享 `Cargo.lock` 和 `target/`，但编译隔离。

---

## Level 3：设计思考

### 3-1：Package → Crate → Module 关系

| 层级 | 概念 | 物理表现 | 类比 |
|------|------|---------|------|
| Package | 一个 Cargo 项目 | `Cargo.toml` | npm package / Python package |
| Crate | 编译单元 | lib.rs 或 main.rs | shared library / executable |
| Module | 代码组织 | `mod` 声明 + 文件 | Python module / C++ namespace |

一个 Package 可以包含多个 Crate（最多 1 个 lib + 多个 bin）。

### 3-2：可见性默认私有的工程价值

- 公共 API 必须显式声明，减少了"不小心暴露内部细节"的代价
- 重构时只影响 `pub` 的接口，模块内部变化不会破坏下游
- 阅读代码时，`pub` 是有意义的语义信号

---

## 迁移思维练习

### Python 模块与 Rust 模块

| 概念 | Python | Rust |
|------|--------|------|
| 模块声明 | 文件系统隐式 | `mod` 显式声明 |
| 导入 | `import x` (加载+执行) | `use x` (符号别名) |
| 可见性 | 默认公开 (`_` 前缀约定) | 默认私有 (`pub` 显式) |
| 重导出 | `__init__.py` `from .x import Y` | `pub use x::Y` |
| 循环引用 | 可能（需小心） | 编译期拒绝（`mod` 树无环） |

**迁移提示**：Rust 的模块系统初学比 Python 多些仪式感，但换来的是编译器保证的无循环依赖和明确的 API 边界。

---

*模块系统是工程的骨架。花时间理解 Package/Crate/Module 的关系，后续大型项目的组织会清晰很多。*
