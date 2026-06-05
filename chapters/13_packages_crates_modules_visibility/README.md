# Rust 包、箱与模块系统

## 目录

1. [概述](#概述)
2. [核心概念](#核心概念)
3. [Package（包）详解](#package包详解)
4. [Crate（箱）详解](#crate箱详解)
5. [Module（模块）详解](#module模块详解)
6. [文件系统与模块系统的关系](#文件系统与模块系统的关系)
7. [可见性（Visibility）](#可见性visibility)
8. [路径系统（Paths）](#路径系统paths)
9. [use 关键字详解](#use-关键字详解)
10. [重导出（Re-export）](#重导出re-export)
11. [本项目的文件结构解读](#本项目的文件结构解读)
12. [Python 对照](#python-对照)
13. [常见误区](#常见误区)
14. [总结](#总结)

---

## 概述

Rust 的代码组织系统由三个核心概念组成：**Package（包）**、**Crate（箱）** 和 **Module（模块）**。这是 Rust 初学者最容易混淆的概念集群之一，尤其对于来自 Python 等语言的开发者来说，Rust 的模块系统与文件系统之间的关系与直觉大相径庭。

本文档以本项目 `packages_and_modules` 为实例，彻底剖析这三者的定义、关系与实践要点。

---

## 核心概念

### 一句话区分

| 概念 | 一句话定义 | 类比 |
|------|-----------|------|
| **Package** | 一个 Cargo.toml 描述的项目 | Python 的 `pyproject.toml` 项目 |
| **Crate** | 编译器一次处理的编译单元（一棵模块树） | Python 的一个顶层包 |
| **Module** | `mod` 声明的代码组织单元（子树或叶子） | Python 的一个 `.py` 文件 |

### 层级关系

```
Package (Cargo.toml)
 ├── Library Crate (src/lib.rs)          ← 最多 1 个
 │    ├── Module: models (src/models.rs)
 │    │    ├── struct Task
 │    │    ├── enum Priority
 │    │    └── fn create_sample_tasks()
 │    └── Module: services (src/services/mod.rs)
 │         ├── Module: parser (src/services/parser.rs)
 │         │    ├── pub fn parse_task_line()
 │         │    └── fn trim_input()        ← 私有
 │         └── Module: validator (src/services/validator.rs)  ← 模块私有！
 │              ├── pub(super) fn validate_title()
 │              └── pub(crate) fn sanitize()
 │
 └── Binary Crate (src/main.rs)          ← 可以有多个
      └── Module: app_utils              ← binary crate 自己的模块
```

---

## Package（包）详解

### 定义

Package 是 Cargo 项目的顶层抽象。一个 Package 由一个 `Cargo.toml` 文件定义。

### 一个 Package 包含什么

| 组成部分 | 数量限制 | 说明 |
|----------|---------|------|
| Library Crate | 最多 1 个 | 入口是 `src/lib.rs` |
| Binary Crate | 0 到多个 | 入口是 `src/main.rs` 或 `src/bin/*.rs` |
| Example Crate | 0 到多个 | 入口是 `examples/*.rs` |
| Test Crate | 0 到多个 | 入口是 `tests/*.rs` |
| Bench Crate | 0 到多个 | 入口是 `benches/*.rs` |

### 关键规则

1. **一个 Package 最多只能有一个 library crate**。如果你定义了 `src/lib.rs`，你的 Package 就有了一个 library crate。
2. **一个 Package 可以有任意数量的 binary crate**。默认的 binary crate 入口是 `src/main.rs`，额外的可以放在 `src/bin/` 目录下。
3. **Library crate 和 binary crate 可以同时存在**。这是非常常见的模式：library crate 提供核心逻辑，binary crate 提供命令行接口。本项目就是这种模式。
4. **Package 的名字由 `Cargo.toml` 中的 `[package].name` 决定**。对于外部使用者来说，library crate 就以这个名字被引用。

### 本项目示例

```toml
# Cargo.toml
[package]
name = "packages_and_modules"
version = "0.1.0"
edition = "2024"
```

- 外部 crate 通过 `packages_and_modules::models::Task` 引用我们的类型。
- Binary crate 内部也通过 `packages_and_modules::models::Task` 引用 library crate 的类型（binary crate 和 library crate 是不同的 crate！）。

---

## Crate（箱）详解

### 定义

Crate 是 Rust 编译器一次处理的最小代码单元。每个 Crate 都有一个 **crate root**（根文件），编译器从该文件开始构建一棵模块树。

### Crate 的类型

| 类型 | Crate Root | 编译产物 |
|------|-----------|---------|
| Library Crate | `src/lib.rs` | `.rlib`（库文件） |
| Binary Crate | `src/main.rs` | 可执行文件 |
| Binary Crate | `src/bin/other.rs` | 可执行文件（名为 `other`） |

### Crate Root 的作用

Crate Root 是编译器构建模块树的起点。它负责声明（`mod`）顶级模块。

```rust
// src/lib.rs — library crate root
pub mod models;   // 声明 models 模块
pub mod services; // 声明 services 模块
```

### Crate 名字

- **Library crate** 的名字就是 Package 的名字（`[package].name`）。
- **Binary crate** 的名字默认是 Package 的名字（如果只有 `src/main.rs`），或者是 `src/bin/` 下的文件名。
- 在代码中引用当前 crate 的根，使用 `crate::` 前缀。

### 重要理解

Binary crate 和 library crate 虽然属于同一个 Package，但它们是**不同的 crate**。这意味着：

- Binary crate 不能直接访问 library crate 中使用 `pub(crate)` 修饰的项（`pub(crate)` 限定只在**同一个 crate** 内可见）。
- Binary crate 通过 library crate 的名字来引用库中的公开 API：
  ```rust
  use packages_and_modules::models::Task; // 引用 library crate
  ```
  而不是：
  ```rust
  // use crate::models::Task; // 错误！binary crate 中没有 models
  ```

---

## Module（模块）详解

### 定义

Module 是 Rust 中组织代码、控制作用域和可见性的基本单元。它由 `mod` 关键字声明。

### 模块声明 vs 模块定义

Rust 中模块的声明和定义有两种方式：

#### 方式一：内联模块

```rust
mod my_module {
    pub fn hello() {
        println!("Hello!");
    }
}
```

#### 方式二：文件模块（本项目使用的方式）

```rust
// 在 lib.rs 中声明
pub mod models; // Rust 会查找 src/models.rs 或 src/models/mod.rs
```

Rust 编译器按以下顺序查找模块对应的文件：
1. `<module_name>.rs`（例如 `src/models.rs`）
2. `<module_name>/mod.rs`（例如 `src/services/mod.rs`）

### 模块树结构

模块形成一棵树。根是 crate root 文件。

```
crate (lib.rs)
├── models (models.rs)      ← 声明的子模块
└── services (services/mod.rs)  ← 声明的子模块
    ├── parser (services/parser.rs)  ← services 声明的子模块
    └── validator (services/validator.rs)  ← services 声明的子模块
```

### 模块默认私有

一个模块默认是**私有的**。这意味着：

- 在 `services/mod.rs` 中写 `mod validator;`（无 `pub`），则 `validator` 对外部不可见。
- 在 `services/mod.rs` 中写 `pub mod parser;`，则 `parser` 对外部可见。

函数也是如此：默认私有，需要显式添加 `pub`。

---

## 文件系统与模块系统的关系

**这是 Rust 与 Python 最大的不同之处，也是初学者最常犯错的地方。**

### Rust：显式声明模块

```
Python 模式（错误的理解）:
  src/
  ├── models.py     ← 自动是一个模块
  └── services.py   ← 自动是一个模块

Rust 模式（正确的理解）:
  src/
  ├── lib.rs        ← 必须在此声明: pub mod models;
  ├── models.rs     ← 仅为 mod models; 提供代码
  └── services/
      ├── mod.rs    ← 必须在此声明: pub mod parser; mod validator;
      ├── parser.rs
      └── validator.rs
```

**关键区别**：
- Python 中，每个 `.py` 文件**自动成为一个模块**。
- Rust 中，`.rs` 文件**不会自动成为模块**。必须有一个父模块通过 `mod` 关键字显式声明它。

### 具体规则

| 场景 | 你需要做什么 |
|------|------------|
| 新建 `src/foo.rs` | 在 `src/lib.rs`（或父模块）中添加 `mod foo;` 或 `pub mod foo;` |
| 新建 `src/foo/bar.rs` | 在 `src/foo/mod.rs` 中添加 `mod bar;` 或 `pub mod bar;` |
| 新建 `src/bin/tool.rs` | 自动被 Cargo 识别为 binary crate，无需声明 |

### 常见的编译错误

```rust
// error[E0583]: file not found for module `foo`
// 原因：你创建了 foo.rs 但没有在任何地方用 mod foo; 声明它
```

```rust
// error[E0432]: unresolved import `crate::foo`
// 原因：mod foo; 存在但 foo.rs 中没有定义被导入的项
```

---

## 可见性（Visibility）

Rust 的可见性修饰符精确控制一个项（item）可以从哪里被访问。

### 可见性修饰符一览

| 修饰符 | 可见范围 | 说明 |
|--------|---------|------|
| 无修饰符（默认） | 当前模块 | 与 `pub(self)` 等价 |
| `pub` | 所有地方 | 完全公开 |
| `pub(crate)` | 当前 crate 内 | 外部 crate 不可见 |
| `pub(super)` | 父模块 | 父模块及其所有子模块 |
| `pub(self)` | 当前模块 | 显式等同于默认 |
| `pub(in path)` | 指定路径 | 对指定路径祖先可见 |

### 本项目中的实例

```rust
// validator.rs

// pub(super): 仅对父模块 services 及其子模块可见
// parser 可以调用，但外部 crate 不能
pub(super) fn validate_title(title: &str) -> bool { ... }

// pub(crate): 对整个 library crate 可见
// library crate 内的任何模块都可以调用，但 binary crate 不能
pub(crate) fn sanitize(input: &str) -> String { ... }
```

```rust
// services/mod.rs

// 私有模块声明，外部无法访问 services::validator
mod validator;

// 公开模块声明，外部可以访问 services::parser
pub mod parser;
```

```rust
// parser.rs

// 私有函数，仅 parser 模块内部可用
fn trim_input(s: &str) -> String { ... }
```

### 可见性验证

| 尝试访问 | 结果 |
|---------|------|
| `packages_and_modules::models::Task` | ✅ 公开 |
| `packages_and_modules::services::parse_task_line` | ✅ 通过 re-export 公开 |
| `packages_and_modules::services::validator::sanitize` | ❌ 模块私有 + pub(crate) |
| `packages_and_modules::services::parser::trim_input` | ❌ 函数私有 |
| `crate::app_utils::print_module_info`（在 main.rs 中） | ✅ 同一 crate |

---

## 路径系统（Paths）

Rust 中有两种路径形式：**绝对路径** 和 **相对路径**。

### 绝对路径

以 `crate::` 或外部 crate 名字开头。

| 路径 | 含义 |
|------|------|
| `crate::models::Task` | 从当前 crate 根开始的绝对路径 |
| `packages_and_modules::models::Task` | 从 library crate 根开始的绝对路径 |
| `std::collections::HashMap` | 标准库路径 |

### 相对路径

以 `self::` 或 `super::` 开头。

| 路径 | 含义 |
|------|------|
| `self::foo` | 当前模块内的 `foo` |
| `super::foo` | 父模块中的 `foo` |
| `super::validator::validate_title` | 父模块中 `validator` 子模块的 `validate_title` |

### 本项目实例

在 `parser.rs` 中：
```rust
// 绝对路径：从 library crate 根开始
use crate::models::Priority;

// 相对路径：引用父模块（services）中的兄弟模块 validator
if super::validator::validate_title(&title) {
    ...
}
```

在 `main.rs` 中：
```rust
// 绝对路径：从 binary crate 根开始
crate::app_utils::print_module_info();

// 绝对路径：从 library crate 根开始
packages_and_modules::models::create_sample_tasks();
```

---

## use 关键字详解

### use 的本质

`use` 创建一个**路径别名**（或称为路径的快捷方式）。`use` 不导入任何代码，它只是让一个路径在作用域中可用。

### use 的常见形式

```rust
// 1. 导入单个项
use packages_and_modules::models::Task;

// 2. 导入多个项（花括号）
use packages_and_modules::models::{Task, Priority, create_sample_tasks};

// 3. 别名导入
use packages_and_modules::models::Priority as TaskPriority;

// 4. 导入模块本身
use packages_and_modules::services; // 然后通过 services::parse_task_line 使用

// 5. Glob 导入（通配符，谨慎使用）
use packages_and_modules::models::*;

// 6. 导入自身（self）
use std::io::{self, Read}; // io 模块和 Read trait

// 7. 嵌套路径
use std::collections::{HashMap, HashSet};
```

### use 与 mod 的区别

这是另一个常见的混淆点。

| 关键字 | 作用 | 类比 Python |
|--------|------|------------|
| `mod` | **声明**一个模块的存在，使其成为模块树的一部分 | 无直接对应（不是 import） |
| `use` | **创建**路径别名，将名称带入作用域 | `import` |

**记忆口诀**：`mod` 创建模块，`use` 创建快捷方式。

### 具体例子

```rust
// lib.rs

// mod 声明：告诉编译器"models 模块存在，代码在 models.rs 中"
pub mod models;

// 然后 use 才可以把内容引入作用域
use crate::models::Task; // 现在可以直接写 Task 而不写完整路径
```

---

## 重导出（Re-export）

### 定义

`pub use` 语句将一个项重新导出，使其出现在新的位置。

### 本项目的例子

```rust
// services/mod.rs
pub use parser::parse_task_line;
```

这样，外部使用者可以：
```rust
// 直接使用
use packages_and_modules::services::parse_task_line;

// 而不是
use packages_and_modules::services::parser::parse_task_line;
```

### 重导出的意义

1. **简化 API**：将深层路径提升到更浅的位置。
2. **隐藏内部结构**：外部不需要知道 `parser` 子模块的存在。
3. **灵活重组**：内部可以随意调整模块结构而不破坏外部 API。

---

## 本项目的文件结构解读

### 逐个文件说明

| 文件 | 角色 | 说明 |
|------|------|------|
| `Cargo.toml` | Package 配置 | 定义包名、版本、edition |
| `src/lib.rs` | Library Crate Root | 声明公开模块，写库级文档 |
| `src/models.rs` | 数据模型模块 | `Task` 结构体、`Priority` 枚举 |
| `src/services/mod.rs` | 服务模块入口 | 声明子模块，重导出 API |
| `src/services/parser.rs` | 解析模块（公开） | `parse_task_line` 公开函数 |
| `src/services/validator.rs` | 验证模块（私有） | 演示 `pub(super)` 和 `pub(crate)` |
| `src/main.rs` | Binary Crate Root | 可执行程序，演示路径系统 |
| `tests/integration_test.rs` | 集成测试 | 测试公开 API，验证可见性边界 |

### 编译产物

```bash
# 编译 library crate（cargo build）
target/debug/libpackages_and_modules.rlib

# 编译 binary crate（cargo build）
target/debug/packages_and_modules

# 运行 binary crate
cargo run
# 等效于: cargo build && ./target/debug/packages_and_modules

# 运行所有测试
cargo test
```

### Cargo 命令速查

| 命令 | 作用 |
|------|------|
| `cargo build` | 编译所有 crate |
| `cargo build --lib` | 仅编译 library crate |
| `cargo build --bin <name>` | 仅编译指定的 binary crate |
| `cargo run` | 编译并运行默认 binary crate |
| `cargo test` | 运行所有测试（包括单元测试和集成测试） |
| `cargo test --test integration_test` | 仅运行指定集成测试文件 |
| `cargo doc --open` | 生成并打开文档 |

---

## Python 对照

### 文件即模块 vs 显式声明模块

**Python**：
```python
# my_project/
#   models.py          ← 自动是一个模块
#   services/
#       __init__.py    ← 标识为包
#       parser.py      ← 自动是一个模块

# 使用
from my_project.models import Task
from my_project.services.parser import parse_task_line
```

**Rust**：
```rust
// my_project/
//   Cargo.toml         ← Package 定义
//   src/
//       lib.rs         ← 必须: pub mod models; pub mod services;
//       models.rs      ← 仅当 mod models; 声明后才成为模块
//       services/
//           mod.rs     ← 必须: pub mod parser; mod validator;
//           parser.rs  ← 仅当 mod parser; 声明后才成为模块
//           validator.rs

// 使用
use my_project::models::Task;
use my_project::services::parse_task_line;
```

### import vs use/mod

| Python | Rust |
|--------|------|
| `import` 导入模块 | `mod` 声明模块 + `use` 创建路径别名 |
| `from x import y` | `use x::y` |
| `from x import y as z` | `use x::y as z` |
| `from x import *` | `use x::*` |
| `__init__.py` | `mod.rs` / 同名的 `.rs` 文件 |
| `__all__` | `pub use` 控制重导出 |
| 默认公开 | 默认私有 |
| 无 `pub(crate)` 对应 | `pub(crate)` 限定 crate 内可见 |

### 可见性对比

| Python | Rust |
|--------|------|
| 公开（默认） | `pub` |
| `_name`（约定私有） | 无修饰符（编译器强制私有） |
| `__name`（名称改写） | 无直接对应 |
| 无 | `pub(crate)` |
| 无 | `pub(super)` |
| 无 | `pub(in path)` |

### 关键思维转变

对于 Python 开发者来说，最重要的思维转变是：

1. **Python 的文件系统即模块系统**。创建一个 `.py` 文件就创建了一个模块。
   **Rust 的模块系统独立于文件系统**。创建一个 `.rs` 文件后，必须用 `mod` 将其声明为模块。

2. **Python 默认公开**。所有顶层名称默认可以被外部访问。
   **Rust 默认私有**。必须显式添加 `pub`。

3. **Python 的 `__init__.py` 使目录成为包**。
   **Rust 使用 `mod.rs` 或同名的 `.rs` 文件来定义模块入口**。

---

## 常见误区

### 误区 1：认为 `src/foo.rs` 自动是一个模块

**错误心态**："我创建了 `src/utils.rs`，为什么不能 `use crate::utils::helper`？"

**正确理解**：你必须在 `src/lib.rs` 或 `src/main.rs` 中添加 `mod utils;` 或 `pub mod utils;`，然后编译器才会把 `utils.rs` 纳入模块树。

### 误区 2：混淆 `mod` 和 `use`

**错误代码**：
```rust
// 在 lib.rs 中
use models::Task; // 错误！models 模块还没有被声明
```

**正确代码**：
```rust
// 在 lib.rs 中
pub mod models;          // 先声明模块
use crate::models::Task; // 再引入名称
```

### 误区 3：认为 `pub(crate)` 对同 Package 的 binary crate 也可见

`pub(crate)` 的 `crate` 指的是**同一个编译单元**。Binary crate 和 library crate 是**不同的 crate**。

```rust
// library crate (lib.rs → validator.rs)
pub(crate) fn sanitize(input: &str) -> String { ... }

// binary crate (main.rs)
// 无法调用 sanitize！因为 binary crate 和 library crate 是不同的 crate
// packages_and_modules::services::validator::sanitize("x"); // 编译错误
```

### 误区 4：认为 `crate::` 在任何文件中都指向同一个根

`crate::` 在 library crate 的文件中指向 library crate 的根（`lib.rs`）。
`crate::` 在 binary crate 的文件中指向 binary crate 的根（`main.rs`）。

在 `parser.rs`（属于 library crate）中：
```rust
crate::models::Task // 正确 — 指向 library crate 的 models
```

在 `main.rs`（属于 binary crate）中：
```rust
crate::models::Task // 错误 — binary crate 中不存在 models
packages_and_modules::models::Task // 正确 — 显式引用 library crate
```

### 误区 5：认为文件路径和模块路径必须一致

文件路径和模块路径是**两个独立的概念**，尽管 Cargo 的默认约定使它们看起来一致。通过 `#[path]` 属性可以覆盖文件路径：

```rust
#[path = "some/other/path.rs"]
mod my_module;
```

但这是高级用法，大多数情况下遵循约定即可。

### 误区 6：滥用 glob 导入

```rust
use some_crate::*; // 不推荐：污染作用域，破坏可读性
```

除了一些特定场景（如 prelude 模式、测试模块），应尽量避免 glob 导入。

---

## 总结

### 知识速查表

| 你想做什么 | 关键字/做法 |
|-----------|-----------|
| 声明一个模块 | `mod name;` 或 `pub mod name;` |
| 让外部访问模块 | `pub mod name;` |
| 让外部访问函数/类型 | `pub fn` / `pub struct` / `pub enum` |
| 限制在 crate 内可见 | `pub(crate)` |
| 限制在父模块可见 | `pub(super)` |
| 简化路径 | `use path::to::item;` |
| 简化路径并重命名 | `use path::to::item as NewName;` |
| 让深层 API 在更浅位置可用 | `pub use path::to::item;` |
| 引用当前 crate 根 | `crate::` |
| 引用父模块 | `super::` |
| 引用当前模块 | `self::` |
| 引用外部 crate | `crate_name::` |
| 引用标准库 | `std::` |

### 学习建议

1. **从本项目入手**：先运行 `cargo run` 和 `cargo test`，观察输出。
2. **阅读源码**：逐文件阅读，理解每个 `pub`、`mod`、`use` 的作用。
3. **动手修改**：尝试将某个 `pub` 改为默认私有，观察编译错误。
4. **对照 Python**：如果你来自 Python，时刻提醒自己 Rust 的模块需要显式声明。
5. **画模块树**：对于复杂项目，先画出模块树结构，再写代码。

### 下一步

完成本章后，建议继续学习：
- Cargo Workspace（多 Package 管理）
- Feature flags（条件编译）
- `cfg` 属性（平台条件编译）
- 发布到 crates.io
