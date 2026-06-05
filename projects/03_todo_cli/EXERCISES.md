# todo_cli — 练习指南

## 项目概述

你已阅读了 `README.md`、`src/lib.rs` 和 `src/main.rs`。这个 CLI 待办事项管理器采用 **library-first** 设计，核心业务逻辑在 `lib.rs` 中，CLI 解析在 `main.rs` 中。请在不直接复制粘贴的前提下完成以下练习。

---

## Level 1: 基础练习（理解现有代码）

### L1-1: 运行与观察

1. 运行 `cargo run -- --help` 查看所有子命令
2. 运行 `cargo run -- add "学习 Rust"` 添加一条待办
3. 运行 `cargo run -- list` 查看所有待办
4. 运行 `cargo run -- complete 1` 标记完成
5. 运行 `cargo run -- -f my_test.json add "自定义文件"` 测试自定义存储路径
6. 关闭程序后重新运行 `cargo run -- list`，验证数据持久化
7. 运行 `cargo test` 观察所有测试通过

**学习点**: CLI 子命令模式、JSON 持久化、数据跨会话保留。

### L1-2: 追踪 `TodoItem` 的所有权

在 `lib.rs` 中，找出以下场景的所有权流转：

- `TodoList::add(&mut self, title: String)` — `title` 的所有权去了哪里？
- `TodoList::get(&self, key: u32) -> Option<&TodoItem>` — 为什么返回引用而不是 owned value？
- `TodoList::delete(&mut self, id: u32) -> Result<TodoItem, String>` — 被删除的 `TodoItem` 所有权转移给了谁？

回答：如果 `get` 返回 `Option<TodoItem>`（owned），会有什么问题？

**学习点**: 所有权在 CRUD 操作中的体现、引用 vs 所有权的 API 设计选择。

### L1-3: 理解 `clap` derive 模式

在 `main.rs` 中：

1. 找到 `#[derive(Parser)]` 和 `#[derive(Subcommand)]`
2. 解释 `Commands` 枚举中每个变体的参数如何映射到命令行参数
3. 为什么 `Add { title: Vec<String> }` 使用 `Vec<String>` 而不是 `String`？
4. 尝试在 `Commands` 中临时添加一个错误的变体定义（如重复的 `List`），观察编译错误

**学习点**: clap derive 宏、枚举变体携带数据、命令行参数解析。

### L1-4: 理解错误处理策略

在 `lib.rs` 中，`TodoList::load()` 和 `TodoList::save()` 返回 `Result<_, Box<dyn Error>>`：

1. `Box<dyn Error>` 可以容纳哪些类型的错误？它为什么能统一 I/O 错误和 JSON 解析错误？
2. `load()` 中文件不存在时返回空列表而不是错误——为什么这是合理的设计？
3. 如果文件存在但内容不是合法 JSON，程序如何响应？
4. 将 `main.rs` 中某处 `unwrap()` 改为 `expect("...")`，使错误信息更有意义

**学习点**: trait object 在错误处理中的应用、`?` 运算符、`Box<dyn Error>`。

### L1-5: 添加 `--sort` 排序选项

为 `list` 子命令添加排序选项：

```
cargo run -- list --sort id     # 按 ID 排序（默认）
cargo run -- list --sort title  # 按标题排序
```

实现要点：
- `Commands::List` 变体需添加 `sort: Option<String>` 字段
- 在 `main.rs` 的分发逻辑中根据排序选项调用不同的排序方法
- 排序逻辑应在 `lib.rs` 还是 `main.rs` 中？

**学习点**: clap 可选参数、排序实现位置的设计权衡。

---

## Level 2: 功能扩展（编写新代码）

### L2-1: 实现 `undo` 命令

添加撤销最近一次操作的命令：

```
cargo run -- undo
```

实现要点：
- 需要在 `TodoList` 中维护操作历史（哪些操作需要记录？add / complete / delete）
- 设计 `enum Action { Add(TodoItem), Complete(u32), Delete(TodoItem) }` 来表示操作
- `undo` 如何恢复被删除的条目？如何回滚"标记完成"？

**学习点**: 操作历史设计、枚举携带数据、撤销模式。

### L2-2: 实现按关键词搜索

添加 `search` 子命令，按关键词搜索待办事项：

```
cargo run -- search "Rust"
  #1 [ ] 学习 Rust 所有权
  #3 [✅] 阅读 Rust Book 第10章
```

实现要点：
- 在 `TodoList` 中添加 `fn search(&self, keyword: &str) -> Vec<&TodoItem>`
- 搜索应不区分大小写
- 匹配标题中包含关键词的所有条目
- 如何让搜索支持多个关键词？（AND 逻辑 vs OR 逻辑）

**学习点**: 字符串搜索、大小写处理、迭代器 filter 链。

### L2-3: 添加优先级字段

为 `TodoItem` 添加优先级：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Priority { High, Medium, Low }

// TodoItem 中添加
pub priority: Priority,
```

实现要点：
- `add` 命令需要支持可选优先级参数：`todo add "标题" --priority high`
- `list` 需要支持按优先级排序
- JSON 文件中的旧数据（没有 priority 字段）如何处理？（提示：`#[serde(default)]`）

**学习点**: serde 默认值、向后兼容、枚举序列化。

### L2-4: 实现批量导入

添加 `import` 命令，从文件批量导入待办事项：

```
cargo run -- import tasks.txt
```

文件格式（每行一个标题）：
```
学习 Rust
写单元测试
阅读 The Book
```

实现要点：
- 逐行读取文件，每行作为一个待办标题
- 跳过空行和注释行（`#` 开头）
- 返回导入数量统计
- 文件不存在或格式错误时给出清晰提示

**学习点**: 文件逐行读取、批量操作、错误聚合。

---

## Level 3: 设计思维（架构与扩展）

### L3-1: 存储后端抽象

当前实现将数据存储为 JSON 文件。设计一个存储后端 trait，使 `TodoList` 可以切换到不同的存储：

```rust
trait Storage {
    fn load(&self, path: &str) -> Result<TodoListData, Box<dyn Error>>;
    fn save(&self, path: &str, data: &TodoListData) -> Result<(), Box<dyn Error>>;
}

struct JsonStorage;
struct CsvStorage;
struct SqliteStorage { conn: Connection }
```

- 如何修改 `TodoList` 以接受 `Box<dyn Storage>`？
- 是否需要引入泛型参数 `<S: Storage>`？
- 比较 trait object 方案与泛型方案的优劣

**学习点**: trait object vs 泛型、策略模式、存储层抽象。

### L3-2: 并发安全设计

当前 `TodoList` 不支持并发访问（`&mut self` 需要独占引用）。设计一个支持多线程的版本：

- 哪些方法需要修改签名？
- `Arc<Mutex<TodoList>>` 是最简方案吗？在什么场景下 `RwLock` 更好？
- 多线程环境下 JSON 文件的读写如何保证一致性？

**不要求完整实现**，设计关键数据结构和方法签名即可。

**学习点**: `Arc`、`Mutex`、`RwLock`、并发文件访问。

### L3-3: 网络同步协议

设计一个简单的网络同步协议，使两个设备上的 `todo_cli` 可以合并数据：

- 如何检测冲突？（两台设备修改了同一条待办）
- 如何解决冲突？（last-write-wins? 用户手动选择？）
- 同步需要哪些元数据？（时间戳、设备 ID、版本号）

给出消息格式设计和同步流程图。

**学习点**: 分布式系统基础、冲突检测与解决、CRDT 概念。

---

## 检查清单

完成上述练习后，你应该能够：

- [ ] 理解 `lib.rs` 与 `main.rs` 的 library-first 职责分离
- [ ] 使用 `serde` + `serde_json` 进行 JSON 序列化/反序列化
- [ ] 通过 `clap` derive 模式构建 CLI 子命令
- [ ] 使用 `Box<dyn Error>` 统一不同错误类型
- [ ] 理解所有权在 CRUD 操作中的体现（&self vs &mut self）
- [ ] 为结构体添加新字段并保持向后兼容
- [ ] 设计存储后端抽象（trait object 模式）
- [ ] 评估并发方案（Mutex vs RwLock）的适用场景
