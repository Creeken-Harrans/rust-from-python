# todo_cli — 命令行待办事项管理器

基于 Rust 的命令行待办事项管理工具，支持 JSON 持久化存储和完整的 CRUD 操作。

---

## 目录

1. [项目目标](#1-项目目标)
2. [需求分析](#2-需求分析)
3. [知识点清单](#3-知识点清单)
4. [目录结构](#4-目录结构)
5. [快速开始](#5-快速开始)
6. [设计决策](#6-设计决策)
7. [运行示例](#7-运行示例)
8. [测试方法](#8-测试方法)
9. [代码讲解按模块](#9-代码讲解按模块)
10. [扩展方向](#10-扩展方向)
11. [当前设计局限](#11-当前设计局限)

---

## 1. 项目目标

设计并实现一个 Rust 语言的命令行待办事项管理器（Todo CLI），作为学习 Rust 语言核心概念和生态系统工具的实践项目。项目明确的教学目标包括：

- **掌握 Struct 与 impl**：定义数据结构并为其实现方法。
- **理解 Enum 与模式匹配**：使用枚举表示 CLI 子命令，并通过 `match` 分发。
- **熟练使用 Result 错误处理**：培养不依赖异常、显式处理错误的 Rust 思维。
- **模块化设计**：将项目拆分为 `lib.rs`（核心逻辑）和 `main.rs`（CLI 界面）两层。
- **第三方依赖管理**：通过 Cargo 引入 `serde`、`serde_json`、`clap` 等生态包。
- **序列化与反序列化**：使用 JSON 格式持久化数据。
- **CLI 设计模式**：理解 CLI 工具的标准交互模型。
- **单元测试与集成测试**：建立完善的多层次测试体系。

从实际用户需求角度，该项目解决以下问题：

- 用户希望用命令行快速记录待办事项，无需离开终端。
- 数据需要持久化，关闭程序后不丢失。
- 支持标记完成、删除、按状态筛选等基本操作。
- 存储格式可读、可手动编辑（JSON 文件）。
- 界面简洁直观，提供友好的错误提示。

---

## 2. 需求分析

### 2.1 功能需求

| 功能       | 命令               | 描述                                   |
| ---------- | ------------------ | -------------------------------------- |
| 添加待办   | `todo add <标题>`  | 添加一个新待办事项，自动分配唯一 ID    |
| 列出全部   | `todo list`        | 显示所有待办事项及其完成状态           |
| 标记完成   | `todo complete ID` | 将指定 ID 的条目标记为已完成           |
| 删除条目   | `todo delete ID`   | 删除指定 ID 的条目                     |
| 列出未完成 | `todo pending`     | 筛选并显示所有未完成的待办事项         |
| 列出已完成 | `todo done`        | 筛选并显示所有已完成的待办事项         |

### 2.2 非功能需求

- **数据持久化**：程序退出后数据不丢失，以 JSON 格式存储于文件中。
- **错误处理**：对无效输入（如不存在的 ID）给出清晰的错误提示，不 panic。
- **文件容错**：首次运行或文件缺失时自动创建空列表，不报错。
- **可配置性**：允许用户通过 `-f` 参数指定存储文件路径。
- **自文档化**：通过 `--help` 显示完整的命令使用说明。
- **代码质量**：所有公开 API 均有文档注释，库与二进制分离。

### 2.3 用户故事

1. 作为用户，我可以用 `todo add "学习 Rust"` 记录一个新任务。
2. 作为用户，我可以用 `todo list` 查看所有任务。
3. 作为用户，我可以用 `todo complete 3` 标记第 3 个任务为已完成。
4. 作为用户，我可以用 `todo pending` 只看还没完成的任务。
5. 作为用户，我可以用 `todo done` 查看已完成的任务列表。
6. 作为用户，我可以用 `todo delete 2` 删除一个任务。
7. 作为用户，在输入错误命令时系统会给出清晰的提示。
8. 作为用户，关闭程序后再打开，之前的数据依然存在。

---

## 3. 知识点清单

以下列出本项目实践的核心 Rust 知识点，并标注在哪个文件/位置体现。

| 知识点             | 章节映射                      | 在本项目中的体现                                         |
| ------------------ | ----------------------------- | -------------------------------------------------------- |
| **Struct（结构体）**   | Rust 结构体定义与实例化       | `TodoItem`、`TodoList` 两个结构体，见 `src/lib.rs`      |
| **Enum（枚举）**       | Rust 枚举与模式匹配           | `Commands` 枚举定义 CLI 子命令，见 `src/main.rs`       |
| **impl 块**           | 为结构体实现方法              | `impl TodoList` 包含 9 个方法，见 `src/lib.rs`         |
| **Result<T, E>**      | Rust 的错误处理核心类型       | 所有 I/O 和解析操作返回 `Result`，不 panic              |
| **Option<T>**         | 可空值的类型安全表示          | `Vec.iter().find()` 返回 `Option`                       |
| **Vec<T> / 集合**    | 动态数组与迭代器              | `TodoList.items: Vec<TodoItem>`                         |
| **模块化（mod）**     | Rust 的 crate 和模块系统      | `lib.rs` 提供库，`main.rs` 使用库，清晰的职责分离       |
| **第三方依赖（crate）** | Cargo.toml 依赖声明         | `serde`、`serde_json`、`clap`                           |
| **Derive 宏**         | `#[derive(...)]` 自动实现 trait | `Serialize`、`Deserialize`、`Debug`、`Clone`、`Parser` |
| **序列化/反序列化**   | serde 框架                    | `serde_json::to_string_pretty` / `serde_json::from_str` |
| **CLI 设计**          | clap derive API               | `#[derive(Parser)]` 和 `#[derive(Subcommand)]`          |
| **错误处理**          | `Box<dyn Error>`、模式匹配   | `TodoList::load()` 区分文件不存在和解析错误             |
| **文件 I/O**          | `std::fs`                     | `fs::read_to_string`、`fs::write`                       |
| **单元测试**          | `#[cfg(test)]` + `#[test]`   | `src/lib.rs` 底部 9 个测试                              |
| **集成测试**          | `tests/` 目录                | `tests/integration_test.rs` 6 个测试                    |
| **文档注释**          | `///` 文档注释                | 所有 `pub` 函数均有 `///` 注释                           |
| **生命周期（引用）**   | 借用与引用                    | `list_pending(&self) -> Vec<&TodoItem>`                 |
| **字符串处理**        | `String` vs `&str`           | 函数签名中精确选择 `String` 还是 `&str`                |
| **命令行参数解析**    | std::env 或 clap              | `clap` derive 模式自动生成解析逻辑                      |

---

## 4. 目录结构

```
03_todo_cli/
├── Cargo.toml              # 项目元数据和依赖声明
├── README.md               # 本文件
├── src/
│   ├── lib.rs              # 核心库：数据结构 + 业务逻辑
│   └── main.rs             # 二进制入口：CLI 子命令解析与分发
└── tests/
    └── integration_test.rs # 集成测试：端到端验证
```

### 各文件职责说明

#### Cargo.toml

项目的清单文件，声明：

- 包名 `todo_cli`、版本 `0.1.0`、使用 Rust 2021 edition
- 依赖项：`serde`（序列化框架）、`serde_json`（JSON 格式支持）、`clap`（命令行参数解析）

#### src/lib.rs（核心库）

这是项目的核心，包含所有业务逻辑：

- **`TodoItem` 结构体**：表示单条待办事项，含 `id`、`title`、`completed` 三个字段。
- **`TodoList` 结构体**：管理待办列表，含 `items`（条目集合）、`next_id`（自增主键）、`file_path`（存储路径）。
- **`impl TodoList`**：提供 `new`、`load`、`save`、`add`、`list`、`complete`、`delete`、`list_pending`、`list_completed` 共 9 个方法。
- **单元测试**：9 个测试覆盖新建、添加、完成、删除、筛选、持久化等场景。

#### src/main.rs（CLI 入口）

面向最终用户的命令行界面：

- 使用 `clap` 的 derive API 定义 `Cli` 主结构和 `Commands` 枚举。
- 6 个子命令：`add`、`list`、`complete`、`delete`、`pending`、`done`。
- `-f` / `--file` 选项允许指定自定义存储文件。
- `print_item` 辅助函数以友好的格式打印条目。
- 所有错误通过 `eprintln!` 输出到 stderr，并返回非零退出码。

#### tests/integration_test.rs（集成测试）

端到端测试，验证库函数的整体行为：

- 完整生命周期测试（添加 → 完成 → 删除）
- 列表过滤测试（pending / done 分类）
- 持久化往返测试（保存 → 重新加载 → 逐字段验证）
- 空文件和缺失文件处理测试
- 大批量操作测试（50 条记录的增删改）
- 边界情况测试（重复完成、无效 ID、空标题）

---

## 5. 快速开始

### 5.1 前置条件

- Rust 工具链（rustc >= 1.70, cargo >= 1.70）
- 网络连接（用于下载依赖包）

### 5.2 编译项目

```bash
cd /home/Creeken/Temp/Rust_/rust-from-python/projects/03_todo_cli
cargo build --release
```

首次编译会自动下载并编译 `serde`、`serde_json`、`clap` 及其传递依赖。

### 5.3 运行

```bash
# 查看帮助
cargo run -- --help

# 添加待办事项
cargo run -- add "学习 Rust 所有权"
cargo run -- add "写单元测试"
cargo run -- add "阅读 The Book 第10章"

# 列出所有
cargo run -- list

# 标记完成
cargo run -- complete 2

# 查看未完成的
cargo run -- pending

# 删除
cargo run -- delete 1

# 指定自定义存储文件
cargo run -- -f my_tasks.json add "自定义文件任务"
```

### 5.4 直接使用编译后的二进制文件

```bash
# 编译后安装到 ~/.cargo/bin/
cargo install --path .

# 然后可以直接使用
todo add "全局命令测试"
todo list
```

---

## 6. 设计决策

### 6.1 为什么选择 library-first 设计

本项目采用 **库优先（library-first）** 的架构模式，将核心逻辑放在 `lib.rs` 中，
CLI 界面放在 `main.rs` 中。这种设计的优势：

1. **可复用性**：`TodoList` 可以被其他程序（GUI、Web 服务、脚本）直接引用，不需要依赖 CLI。
2. **可测试性**：核心逻辑不绑定 CLI，可以在单元测试中直接调用，集成测试也可以通过 `tests/` 目录测试。
3. **职责分离**：库只关心"待办事项的数据结构和管理"，CLI 只关心"如何把用户输入转成库调用"。
4. **符合 Rust 惯例**：Rust 社区强烈推荐将核心逻辑作为库暴露，二进制作为薄壳。
5. **编译效率**：修改 `main.rs` 不会导致依赖方重新编译库部分。

### 6.2 为什么用 Result 而不是 panic

本项目中所有可能失败的操作都返回 `Result<T, E>`，没有任何 `panic!` 或 `unwrap()`（测试除外）。原因：

1. **用户体验**：CLI 工具遇到错误应该打印友好信息并退出，而不是打印 Rust 调用栈恐慌信息。
2. **Rust 哲学**：`panic!` 对应不可恢复的错误（如数组越界、逻辑 bug），`Result` 对应可恢复的错误（如文件不存在、ID 未找到）。
3. **控制流的显式性**：`Result` 强制调用方处理错误，编译器会检查所有可能的错误路径。
4. **组合性**：`Result` 可以使用 `?` 运算符传播错误，代码简洁且语义清晰。
5. **测试友好**：测试中可以精确断言错误信息内容（如 `unwrap_err()` 并检查 `err.contains(...)`）。

具体来说，`TodoList::load()` 在文件不存在时返回空的 `TodoList` 而非错误，这是一个设计选择：
- 首次运行不应被视为错误。
- 文件内容损坏才应报错。

### 6.3 文件存储格式选择 JSON

选择 JSON 作为持久化格式的原因：

1. **人类可读**：用户可以直接用文本编辑器打开 `todos.json` 查看和手动修改数据。
2. **生态系统支持**：Rust 有成熟的 `serde_json` 库，只需一行 `#[derive(Serialize, Deserialize)]` 即可获得完整的序列化支持。
3. **格式自描述**：JSON 的键值对结构对应 Rust 的结构体字段，一一映射，不需要额外的格式说明文档。
4. **跨语言互通**：如果需要用 Python、JavaScript 等语言处理数据，JSON 是最通用的选择。
5. **美观输出**：`serde_json::to_string_pretty` 生成带缩进的 JSON，便于人工阅读。

JSON 文件的示例内容：

```json
{
  "items": [
    {
      "id": 1,
      "title": "学习 Rust 所有权",
      "completed": false
    },
    {
      "id": 2,
      "title": "写单元测试",
      "completed": true
    }
  ],
  "next_id": 3,
  "file_path": "todos.json"
}
```

### 6.4 错误处理策略

本项目采用分层错误处理：

| 层级       | 策略                                                           |
| ---------- | -------------------------------------------------------------- |
| 库层       | 返回 `Result<T, E>`，使用 `Box<dyn Error>` 统一不同错误类型    |
| CLI 层     | 将 `Result` 的 `Err` 转换为用户友好的 stderr 消息              |
| 特殊处理   | 文件不存在不视为错误（返回空列表），区分"缺失"和"损坏"         |
| 测试层     | 使用 `unwrap()` 简化测试代码，失败时 panic 表示测试失败        |

具体实现要点：

- `TodoList::load()` 使用模式匹配区分 `ErrorKind::NotFound` 和其他 I/O 错误。
- `TodoList::complete()` 和 `TodoList::delete()` 返回 `Result<(), String>`，
  将业务逻辑错误（ID 不存在、重复完成）以字符串形式返回。
- `main.rs` 中，`load` 错误立即退出（exit 1），因为无法继续；
  但 `save` 错误仅打印警告（因为数据仍在内存中）。

---

## 7. 运行示例

### 7.1 基础使用流程

```text
$ todo add "买牛奶"
✅ 已添加待办事项:
  [ ] #1    买牛奶

$ todo add "学习 Rust" "写代码"
✅ 已添加待办事项:
  [ ] #2    学习 Rust 写代码

$ todo list
📋 全部待办事项 (共 2 条):
──────────────────────────────────────────────────
  [ ] #1    买牛奶
  [ ] #2    学习 Rust 写代码
──────────────────────────────────────────────────
统计: 2 条未完成, 0 条已完成

$ todo complete 1
✅ 条目 #1 已标记为完成

$ todo list
📋 全部待办事项 (共 2 条):
──────────────────────────────────────────────────
  [✅] #1    买牛奶
  [ ] #2    学习 Rust 写代码
──────────────────────────────────────────────────
统计: 1 条未完成, 1 条已完成

$ todo pending
📝 未完成的待办事项 (共 1 条):
──────────────────────────────────────────────────
  [ ] #2    学习 Rust 写代码
──────────────────────────────────────────────────

$ todo done
✅ 已完成的待办事项 (共 1 条):
──────────────────────────────────────────────────
  [✅] #1    买牛奶
──────────────────────────────────────────────────

$ todo delete 1
🗑️  条目 #1 已删除

$ todo list
📋 全部待办事项 (共 1 条):
──────────────────────────────────────────────────
  [ ] #2    学习 Rust 写代码
──────────────────────────────────────────────────
统计: 1 条未完成, 0 条已完成
```

### 7.2 错误处理示例

```text
$ todo complete 999
错误: 未找到 ID 为 999 的待办事项

$ todo complete 1
错误: 条目 #1 已经是完成状态

$ todo add
错误: 请提供待办事项的标题
用法: todo add <标题>

$ todo --help
命令行待办事项管理器

Usage: todo [OPTIONS] <COMMAND>

Commands:
  add       添加一个新的待办事项
  list      列出所有待办事项（默认按 ID 排序）
  complete  将指定 ID 的条目标记为已完成
  delete    删除指定 ID 的条目
  pending   列出所有未完成的待办事项
  done      列出所有已完成的待办事项

Options:
  -f, --file <FILE>  指定 JSON 存储文件的路径（默认为 todos.json）
  -h, --help         Print help
  -V, --version      Print version
```

### 7.3 自定义存储文件

```text
$ todo -f work.json add "完成季度报告"
✅ 已添加待办事项:
  [ ] #1    完成季度报告

$ todo -f personal.json add "预约牙医"
✅ 已添加待办事项:
  [ ] #1    预约牙医

$ todo -f work.json list
📋 全部待办事项 (共 1 条):
──────────────────────────────────────────────────
  [ ] #1    完成季度报告
──────────────────────────────────────────────────
统计: 1 条未完成, 0 条已完成

$ todo -f personal.json list
📋 全部待办事项 (共 1 条):
──────────────────────────────────────────────────
  [ ] #1    预约牙医
──────────────────────────────────────────────────
统计: 1 条未完成, 0 条已完成
```

---

## 8. 测试方法

### 8.1 运行所有测试

```bash
cargo test
```

预期输出包含以下测试结果：

```
running 9 tests
test tests::test_new_list_is_empty ... ok
test tests::test_add_increments_id ... ok
test tests::test_complete_toggle ... ok
test tests::test_complete_nonexistent ... ok
test tests::test_delete_removes_item ... ok
test tests::test_delete_nonexistent ... ok
test tests::test_list_pending_and_completed ... ok
test tests::test_save_and_load ... ok
test tests::test_load_nonexistent_file ... ok

running 6 tests
test test_full_lifecycle ... ok
test test_list_filtering ... ok
test test_persistence_roundtrip ... ok
test test_empty_and_missing_files ... ok
test test_bulk_operations ... ok
test test_edge_cases ... ok
```

### 8.2 运行特定测试

```bash
# 仅运行库的单元测试
cargo test --lib

# 仅运行集成测试
cargo test --test integration_test

# 按名称筛选
cargo test test_save_and_load
cargo test test_full_lifecycle

# 显示测试输出（println! 等）
cargo test -- --nocapture
```

### 8.3 测试覆盖的场景

| 测试名称                        | 所在文件                       | 覆盖场景                           |
| ------------------------------- | ------------------------------ | ---------------------------------- |
| `test_new_list_is_empty`        | `src/lib.rs`（单元测试）       | 空列表初始化                       |
| `test_add_increments_id`        | `src/lib.rs`（单元测试）       | ID 自增                            |
| `test_complete_toggle`          | `src/lib.rs`（单元测试）       | 完成/重复完成                      |
| `test_complete_nonexistent`     | `src/lib.rs`（单元测试）       | 完成不存在的 ID                    |
| `test_delete_removes_item`      | `src/lib.rs`（单元测试）       | 删除条目                           |
| `test_delete_nonexistent`       | `src/lib.rs`（单元测试）       | 删除不存在的 ID                    |
| `test_list_pending_and_completed` | `src/lib.rs`（单元测试）     | 筛选功能                           |
| `test_save_and_load`            | `src/lib.rs`（单元测试）       | 持久化往返                         |
| `test_load_nonexistent_file`    | `src/lib.rs`（单元测试）       | 缺失文件容错                       |
| `test_full_lifecycle`           | `tests/integration_test.rs`    | 完整生命周期                       |
| `test_list_filtering`           | `tests/integration_test.rs`    | 筛选准确性                         |
| `test_persistence_roundtrip`    | `tests/integration_test.rs`    | 持久化完整性                       |
| `test_empty_and_missing_files`  | `tests/integration_test.rs`    | 空文件和缺失文件                   |
| `test_bulk_operations`          | `tests/integration_test.rs`    | 50 条批量操作                      |
| `test_edge_cases`               | `tests/integration_test.rs`    | 边界条件（重复、无效 ID、空标题） |

---

## 9. 代码讲解按模块

### 9.1 数据模型（`src/lib.rs` 顶部）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
```

`TodoItem` 是单条记录的载体：
- `id: u32`：全局唯一的数字标识，由 `TodoList` 的 `next_id` 字段管理，永不重复使用。
- `title: String`：任务的文字描述。选择 `String` 而非 `&str` 是因为条目拥有自己的数据。
- `completed: bool`：标记是否完成，默认 `false`。
- `#[derive(Serialize, Deserialize)]`：由 `serde` 自动生成序列化和反序列化代码，无需手写。
- `#[derive(Debug, Clone)]`：方便调试打印和复制。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub next_id: u32,
    pub file_path: String,
}
```

`TodoList` 是数据管理器：
- `items: Vec<TodoItem>`：使用堆分配的动态数组存储条目。
- `next_id: u32`：自增计数器，每次 `add` 后递增。
- `file_path: String`：存储路径，序列化到 JSON 中方便重建时保持。

### 9.2 构造函数与持久化（`src/lib.rs` `impl TodoList`）

**`fn new(file_path: &str) -> Self`**

创建空列表，`next_id` 从 1 开始。`file_path` 参数是 `&str`（借用），内部转为 `String`（拥有），因为列表需要持有该值。

**`fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>>`**

加载逻辑的精妙之处在于错误分类：

```rust
match fs::read_to_string(file_path) {
    Ok(content) => serde_json::from_str(&content).map_err(|e| e.into()),
    Err(e) if e.kind() == ErrorKind::NotFound => Ok(TodoList::new(file_path)),
    Err(e) => Err(Box::new(e)),
}
```

- 文件存在且内容合法 → 反序列化为 `TodoList`
- 文件不存在 → 返回空列表（非错误）
- 文件存在但无法读取 → 返回 I/O 错误
- `Box<dyn Error>` 可以包装任意类型的错误，是库代码的常见返回类型

**`fn save(&self) -> Result<(), Box<dyn std::error::Error>>`**

使用 `to_string_pretty` 生成格式化 JSON，`fs::write` 一次性写入。如果需要对大文件做增量写入，可以改用 `BufWriter`。

### 9.3 CRUD 操作

**`fn add(&mut self, title: String) -> TodoItem`**

- 接收 `String` 而非 `&str`，语义上明确列表会取得标题的所有权。
- 创建 `TodoItem`，`next_id` 递增后推入 `items`。
- 返回创建项的克隆副本，方便 CLI 层展示。

**`fn complete(&mut self, id: u32) -> Result<(), String>`**

- 使用 `iter_mut().find()` 找到目标条目。
- 已完成的条目不能再次完成，返回描述性错误。
- `Result<(), String>` 表示操作成功无返回值，失败时有错误描述。

**`fn delete(&mut self, id: u32) -> Result<(), String>`**

- 使用 `iter().position()` 找到索引然后 `remove(index)`，这是按索引删除的标准模式。
- `Vec::remove` 的 O(n) 复杂度对典型使用量级（< 1000 条）完全可接受。

### 9.4 筛选方法

**`fn list_pending(&self) -> Vec<&TodoItem>`**

返回未完成条目的引用集合。注意返回值的生命周期绑定到 `&self`——返回的引用在列表被修改前有效。

**`fn list_completed(&self) -> Vec<&TodoItem>`**

对称地返回已完成条目的引用。

两个筛选方法都不修改数据，只提供视图（view）。这样设计使得 CLI 层可以灵活地以不同方式展示同一份数据。

### 9.5 CLI 层（`src/main.rs`）

**命令定义**

```rust
#[derive(Subcommand)]
enum Commands {
    Add { title: Vec<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Pending,
    Done,
}
```

`clap` 的 derive 模式从枚举变体自动推导子命令名、参数和帮助文本：
- `Add { title: Vec<String> }` 中 `Vec<String>` 允许 `todo add a b c` → 合并为 `"a b c"`。
- 无字段的变体（`List`、`Pending`、`Done`）为无参数子命令。

**命令分发**

每个 `Commands` 变体对应 `match` 的一个分支，调用 `TodoList` 的相应方法。这种模式清晰且易于扩展——添加新命令只需增加枚举变体和匹配分支。

**错误处理**

```rust
let result = match &cli.command { ... };
if let Err(msg) = result {
    eprintln!("错误: {}", msg);
    process::exit(1);
}
```

统一在匹配后处理错误，将库层的 `Err(String)` 转换为 stderr 输出和非零退出码。

### 9.6 集成测试（`tests/integration_test.rs`）

集成测试在 `tests/` 目录下，每个文件被编译为独立的 crate，只能访问 `todo_cli` 库的公开 API。这模拟了外部消费者的视角：

- 测试不依赖 `main.rs` 中的 CLI 代码。
- 测试中的每个函数都是独立的，Cargo 并行运行它们。
- 使用 `cleanup()` 辅助函数确保测试文件不残留。

---

## 10. 扩展方向

### 10.1 优先级

为每个条目增加 `priority: enum Priority { High, Medium, Low }` 字段，支持按优先级排序和筛选。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Priority { High, Medium, Low }

// 在 TodoItem 中添加
pub priority: Priority,
```

新增命令：`todo priority ID High`

### 10.2 截止日期

添加 `due_date: Option<NaiveDate>` 字段（使用 `chrono` crate），支持：

- 按截止日期排序
- 列出逾期任务：`todo overdue`
- 设置提醒（通过桌面通知）

### 10.3 标签/分类

添加 `tags: Vec<String>` 字段，支持：

- 按标签筛选：`todo list --tag rust`
- 多标签组合：`todo list --tag rust --tag cli`
- 标签统计：`todo tags`

### 10.4 同步到云端

将 JSON 文件同步到远程服务：

- **Git 同步**：在每次 `save` 后自动 `git commit && git push`（适合个人使用）。
- **WebDAV / S3 同步**：上传 JSON 到远程存储。
- **自建服务**：实现 REST API 服务端，CLI 调用 API。
- **SQLite 数据库**：替换 JSON 存储为 SQLite（使用 `rusqlite` crate），支持并发访问。

### 10.5 交互式界面

使用 `dialoguer` 或 `inquire` crate 实现：

- 交互式选择要完成/删除的条目（上下箭头 + 回车）
- 编辑条目标题
- TUI（Terminal User Interface）全屏待办面板

### 10.6 多用户支持

- 每个用户独立的 JSON 文件（`todos_{username}.json`）
- 用户认证（`whoami` crate 获取系统用户名）
- 共享列表的协作模式

### 10.7 导出功能

- 导出为 Markdown 清单
- 导出为 CSV
- 导出为 HTML 报告（含完成率统计图表）

### 10.8 国际化（i18n）

使用 `rust-i18n` 或 `fluent` crate 支持多语言，默认英文和中文。

### 10.9 配置系统

通过 `~/.config/todo_cli/config.toml` 配置文件管理：

- 默认存储路径
- 默认排序方式
- 颜色主题（彩色终端输出）
- 别名（如 `t` 替代 `todo`）

---

## 11. 当前设计局限

### 11.1 并发访问问题

由于数据存储在单一 JSON 文件中，多个进程同时读写会导致数据丢失。场景：

- 打开了两个终端，都运行 `todo add ...`。
- 两个进程分别读取文件、在内存中添加条目、写回文件。
- 后写回的进程覆盖先写回的进程的修改。

**解决方法**（超出当前项目范围）：
- 使用文件锁（如 `fs2` crate 提供的 `flock`）。
- 切换到 SQLite 数据库（自带事务和并发控制）。

### 11.2 数据一致性

JSON 文件没有校验和或版本号，如果用户手动编辑文件引入格式错误，程序可能丢失数据。

**解决方法**：在保存前计算并存储校验和，加载时验证；或使用具有模式验证的格式（如 protobuf 配合严格的 schema）。

### 11.3 不是数据库

JSON 文件不是数据库，缺乏：

- **索引**：查找条目需要 O(n) 遍历（当前代码中的 `iter().find()`）。
- **事务**：没有原子性的多步操作。
- **查询语言**：无法做复杂筛选（如"找出标题包含 'Rust' 且未完成的高优先级条目"）。
- **增量更新**：每次 save 都重写整个文件。

**影响范围**：
- 当前设计适合 < 1000 条条目的个人使用。
- 超过这个规模，建议迁移到 SQLite 或 PostgreSQL。

### 11.4 不可变的 ID 策略

`next_id` 只增不减，删除条目后 ID 不会复用。这导致：

- 长期使用的列表会出现 ID 空洞（如只看到 #1、#5、#99），不够美观。
- 但这是正确的设计选择——复用的 ID 引用可能指向不同历史条目。

### 11.5 缺乏撤销功能

没有"撤销上次操作"或操作历史记录。可能的实现：

- 维护操作日志（append-only 的 `audit.log` 文件）。
- 在修改前自动创建备份（`todos.json.bak`）。

### 11.6 命令行中的多词标题

虽然 `todo add a b c` 会合并为 `"a b c"`，但与 shell 的交互可能出现问题：

- `todo add "包含'引号'的标题"` 在某些 shell 中可能被错误解释。
- 特殊字符（`$`、`!`、`` ` ``）需要在 shell 中正确转义。

### 11.7 无数据迁移机制

如果未来 `TodoItem` 结构体增加字段（如 `priority`），旧版 JSON 文件无法自动兼容：

- `serde` 会因缺少字段而报错（除非使用 `#[serde(default)]`）。
- 需要设计版本化存储格式或数据迁移脚本。

### 11.8 文件路径安全性

`file_path` 直接由用户输入控制，未做路径安全检查：
- `-f /etc/passwd` 可能造成意外的系统文件覆盖（虽然 `serde_json` 写入的格式不会产生有效的系统配置）。
- 建议限制文件扩展名或使用安全的默认目录。

---

## 从 Python、C、C++ 迁移时值得注意的设计差异

### 1. 用 Enum 而非字符串常量表示任务状态

Python 中常用字符串常量 `"pending"`、`"done"` 或布尔值 `is_completed` 表示状态。C 中用 `#define` 或整型枚举常量。Rust 的枚举是"代数数据类型"，每个变体可以携带数据，且编译器强制穷尽匹配。本项目虽用 `bool` 表示 `completed` 字段（因为状态简单），但 `Commands` 枚举展示了更重要的用法：六个子命令各为一个变体，变体可携带参数（`Add { title: Vec<String> }`）。`match` 分发时，编译器确保你覆盖了所有命令。Python 中用 `if/elif` 链分发命令，忘记处理某个命令是运行时 bug；在 Rust 中这是编译错误。

### 2. 依赖管理通过 Cargo 而非 pip install

Python 项目用 `pip install` 或 `requirements.txt` 管理依赖，依赖安装到全局或虚拟环境，版本锁定需要 `pip freeze` 生成锁止文件。Rust 的 Cargo 在项目级别声明依赖（`Cargo.toml`），自动解析版本冲突，生成 `Cargo.lock` 锁定精确版本。本项目的 `serde`、`serde_json`、`clap` 三个依赖，一行配置即可，`cargo build` 自动下载并编译所有传递依赖。关键优势：依赖是项目级别的（非全局），同一份 `Cargo.lock` 在任何机器上编译出相同的二进制——这在 Python 生态中需要虚拟环境加锁文件配合才能近似做到。

### 3. 业务逻辑放 `lib.rs` 以提升可测试性

Python 项目中常把所有逻辑（数据处理 + CLI 交互）混在一个脚本里，测试 CLI 工具需要模拟 `sys.argv` 或使用 `subprocess`。Rust 社区约定将核心逻辑放在 `lib.rs` 作为库暴露，`main.rs` 只做参数解析和调用。本项目遵循此模式：`TodoList` 及其所有 CRUD 方法在 `lib.rs` 中定义，`main.rs` 仅负责 `clap` 解析和命令分发。好处是测试可以直接 `use todo_cli::TodoList`，不经过 CLI 层，测试更稳定、更快速，且能访问所有的公开 API —— 这在纯 Python CLI 脚本中需要额外重构才能实现。

### 4. `clap` 的编译期 CLI 验证 vs argparse 的运行时解析

Python 的 `argparse` 在运行时解析参数，选项名称拼写错误或参数类型不匹配只有执行到那个路径才会发现。Rust 的 `clap` derive 模式通过 `#[derive(Parser)]` 和 `#[derive(Subcommand)]` 宏在编译期生成解析代码，子命令名、参数类型都在编译期验证。本项目中 `Commands` 枚举定义了六个子命令，`Complete { id: u32 }` 中的 `u32` 类型确保 ID 参数必须在编译期就是可解析为无符号整数的——这比运行时 `int()` 转换健壮得多。更重要的是，`--help` 文档也是自动从结构体定义生成的，不可能出现文档与代码不同步的问题。

### 5. `serde` 的零模板序列化 vs 手动字典构建

Python 中序列化对象到 JSON 通常需要手动构建字典列表，或使用 `json.dumps(obj.__dict__)`（有安全隐患）。C/C++ 中更复杂，需要手写解析器或引入 protobuf 等重量级框架。Rust 的 `serde` + `#[derive(Serialize, Deserialize)]` 两个 derive 宏即可让 `TodoItem` 和 `TodoList` 获得完整的 JSON 序列化/反序列化能力，支持嵌套结构，编译期保证正确性。它不需要运行时反射（Python 的基础），序列化/反序列化代码在编译期生成，性能接近手写。

---

## 附录 A：依赖版本说明

| Crate       | 版本 | 用途                           |
| ----------- | ---- | ------------------------------ |
| `serde`     | 1.x  | 序列化/反序列化框架，derive 宏 |
| `serde_json` | 1.x | JSON 格式的序列化实现          |
| `clap`      | 4.x  | 命令行参数解析，derive 模式    |

## 附录 B：编译产物大小

在 release 模式下，典型编译产物大小约 1-2 MB（取决于是否启用 LTO 和 strip）。

```bash
cargo build --release
ls -lh target/release/todo_cli
```

如需进一步减小体积：

```toml
[profile.release]
opt-level = "z"     # 优化体积
lto = true          # 链接时优化
strip = true        # 去除符号表
codegen-units = 1   # 更好的优化但更慢的编译
```

## 附录 C：学习资源

- [The Rust Book](https://doc.rust-lang.org/book/) — Rust 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 通过示例学习
- [serde 文档](https://serde.rs/) — 序列化框架
- [clap 文档](https://docs.rs/clap/) — CLI 参数解析
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — Rust API 设计指南

---

*本项目作为 Rust 学习路径中的 CLI 实践项目，涵盖了从数据结构设计到 CLI 交互的完整流程。建议配合 [Rust 程序设计语言](https://doc.rust-lang.org/book/) 第 7-12 章阅读。*
