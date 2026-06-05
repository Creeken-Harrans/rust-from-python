# 项目参考实现说明 — Todo CLI

## 1. 需求拆分

1. 支持 `add`、`list`、`complete`、`delete` 命令
2. 数据持久化为 JSON 文件
3. 命令行参数解析（clap）
4. 错误处理：文件不存在/JSON 损坏/无效输入
5. 模块化设计：lib.rs 业务逻辑 + main.rs CLI 入口

## 2. 推荐实现顺序

1. `lib.rs`: 定义 `TodoItem` 和 `TodoList` 数据结构
2. `lib.rs`: 实现 `TodoList::new/load/save/add/list/complete/delete`
3. `lib.rs`: 单元测试覆盖所有方法
4. `main.rs`: clap 参数解析 + 调度到 lib.rs 方法
5. 集成测试：端到端测试 CLI 行为

## 3. 模块划分

```
src/
├── main.rs    # clap CLI + 调度
└── lib.rs     # TodoItem, TodoList, 所有业务逻辑
```

`lib.rs` 是关键——所有测试通过 `use todo_cli::TodoList` 访问，不依赖 CLI。

## 4. 核心数据结构

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub struct TodoList {
    items: Vec<TodoItem>,
    next_id: u32,
    file_path: String,  // 持久化路径
}
```

**为什么 `id` 是递增数字而非 UUID？**：简单、可读、适合 CLI。增量为原子操作（单线程下安全）。

## 5. 关键函数签名

```rust
impl TodoList {
    pub fn new(file_path: &str) -> Self
    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>>
    pub fn save(&self) -> Result<(), Box<dyn Error>>
    pub fn add(&mut self, title: String) -> TodoItem
    pub fn list(&self) -> &Vec<TodoItem>
    pub fn complete(&mut self, id: u32) -> Result<(), String>
    pub fn delete(&mut self, id: u32) -> Result<(), String>
}
```

## 6. 设计决策

### 为什么用 Enum 表示命令？

clap 的 `#[derive(Subcommand)]` 自动将 CLI 子命令映射为枚举变体，编译器检查穷尽性——新增命令时所有 match 都需更新。

### JSON 文件损坏处理

`load()` 中：文件不存在 → 创建空 TodoList；JSON 解析失败 → 返回 `Err` 而非静默覆盖（保护用户数据）。

### 为什么参数解析交给 clap？

clap 提供类型安全的参数解析 + `--help` 自动生成，避免手动 `env::args()` 索引踩坑。

## 7. 关键代码片段

```rust
// serde 序列化
fn save(&self) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&self.items)?;
    fs::write(&self.file_path, json)?;
    Ok(())
}

// 加载：容错初始化
fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Ok(TodoList { items: vec![], next_id: 1, file_path: file_path.to_string() });
    }
    let json = fs::read_to_string(file_path)?;
    let items: Vec<TodoItem> = serde_json::from_str(&json)?;
    let next_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    Ok(TodoList { items, next_id, file_path: file_path.to_string() })
}
```

## 8. 测试策略

- **序列化往返**：创建 TodoList → save → load → 验证内容一致
- **空文件/不存在文件**：load 应返回空列表而不报错
- **CRUD 操作**：每个方法独立测试
- **id 自增**：add 两次应得到不同 id
- **无效 id**：complete/delete 不存在的 id 应返回 `Err`

## 9. 常见失败方式

| 错误 | 原因 | 修复 |
|------|------|------|
| JSON 损坏后数据丢失 | save 直接覆盖 | 先写入临时文件 + 原子重命名 |
| id 重复 | 删除后 next_id 未更新 | 从已有数据中计算 max(id) |
| 多线程数据竞争 | 本实现无并发保护 | 明确声明单线程限制 |

## 10. 可选扩展

- 按创建时间/完成状态排序
- 支持标签/优先级
- SQLite 替代 JSON 持久化
- 多用户/多列表
- 交互式 TUI（ratatui crate）

---

*重点学习 Enum + serde + clap 的组合模式。这是 Rust CLI 工具开发的"标准配方"。*
