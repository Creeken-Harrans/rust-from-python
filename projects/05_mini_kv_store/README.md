# Mini KV Store —— 本地键值存储

## 目录

1. [项目目标](#项目目标)
2. [知识点清单](#知识点清单)
3. [设计决策](#设计决策)
4. [当前设计局限](#当前设计局限)
5. [如何进一步演化](#如何进一步演化)
6. [明确声明](#明确声明)
7. [运行与测试](#运行与测试)
8. [代码详解](#代码详解)
9. [使用示例](#使用示例)
10. [扩展阅读](#扩展阅读)
11. [总结](#总结)

---

## 项目目标

**Mini KV Store** 是一个用于教学的本地键值存储，目标是通过构建一个可用的 CLI 工具来综合练习 Rust 的核心语言特性。

具体学习目标：

1. **数据结构设计**：设计 `KvStore` 结构体，封装 `HashMap<String, String>` 和文件路径。
2. **所有权与借用**：实践 `&self` vs `&mut self`、返回引用 vs 返回 owned value 的选择。
3. **集合操作**：熟练使用 `HashMap` 的 Entry API，理解其性能优势。
4. **错误处理**：使用 `Box<dyn std::error::Error>` 统一错误类型，理解 trait object 在错误处理中的应用。
5. **模块化设计**：将核心逻辑放在 `lib.rs`，CLI 界面放在 `main.rs`，实现库与二进制的分离。
6. **文件 I/O**：实现简单的持久化格式，处理文件读写中的各种边界情况。
7. **单元测试与集成测试**：编写覆盖核心逻辑的测试用例，理解 `#[cfg(test)]` 和集成测试的组织方式。
8. **CLI 交互**：构建一个 REPL（Read-Eval-Print Loop），处理用户输入、命令解析和交互式反馈。

---

## 知识点清单

### 1. 所有权（Ownership）

```rust
pub fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>>
```

- `key` 和 `value` 参数取 `String`（owned），而非 `&str`（borrowed），因为数据需要存入 `HashMap` 中长期持有。
- 函数签名明确表达了"我会取得这些数据的所有权"的语义。

**设计选择**: 为什么 `set` 取 `String` 而不是 `&str`？
- 调用者如果有 `&str`，可以通过 `.to_string()` 转换，灵活性不受影响。
- 如果取 `&str`，内部仍需 `to_string()` 来存入 HashMap，所有权转移变得隐式。
- 取 `String` 让所有权语义更明确。

### 2. 借用（Borrowing）

```rust
pub fn get(&self, key: &str) -> Option<&String>
pub fn list(&self) -> Vec<(&String, &String)>
```

- `get` 返回 `Option<&String>`：返回对内部 HashMap 的引用，零拷贝。但调用者不能同时持有此引用并修改 store（Rust 的借用规则保证了这一点）。
- `list` 返回 `Vec<(&String, &String)>`：同样是引用，零拷贝，且对引用排序不涉及数据移动。

**设计选择**: 为什么 `get` 返回 `Option<&String>` 而非 `Option<String>`？
- 避免不必要的字符串拷贝。
- 让借用检查器帮助调用者发现潜在的"迭代中修改集合"的错误。

### 3. 集合类型（Collections）

```rust
use std::collections::HashMap;

self.data: HashMap<String, String>
```

- `HashMap` 提供 O(1) 平均查找、插入、删除性能。
- 选择 `String` 作为 key 类型而非 `&str`，避免生命周期参数污染结构体定义。

**Entry API**：

```rust
self.data
    .entry(key)
    .and_modify(|v| *v = value.clone())
    .or_insert(value);
```

对比传统写法：

```rust
// 传统写法：两次哈希查找
if self.data.contains_key(&key) {
    self.data.insert(key, value);  // 第二次查找
} else {
    self.data.insert(key, value);  // 第二次查找
}

// Entry API：一次哈希查找
self.data.entry(key).or_insert(value);  // 只做一次
```

Entry API 只计算一次哈希，比 `contains_key` + `insert` 的模式更高效。

### 4. 错误处理（Error Handling）

```rust
pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>>
```

- **统一错误类型**：使用 `Box<dyn std::error::Error>` 可以容纳任何实现了 `std::error::Error` trait 的错误类型（IO 错误、解析错误等）。
- **`?` 运算符**：在返回 `Result<_, Box<dyn Error>>` 的函数中，`?` 会自动将具体错误类型转换为 `Box<dyn Error>`。
- **权衡**：`Box<dyn Error>` 方便但丢失了具体错误类型信息。生产代码中建议使用 `thiserror` 或 `anyhow` crate。

### 5. 模块系统（Module System）

```
mini_kv_store/
├── Cargo.toml
├── src/
│   ├── lib.rs      # 核心库：KvStore 结构体及实现
│   └── main.rs     # 二进制入口：REPL CLI
└── tests/
    └── integration_test.rs  # 集成测试
```

- Cargo 自动识别 `src/lib.rs` 为库 crate root，`src/main.rs` 为二进制 crate root。
- 这种结构允许其他项目将 `mini_kv_store` 作为库依赖使用。
- 集成测试放在 `tests/` 目录下，作为独立 crate 编译，只能访问公开 API。

### 6. 单元测试与集成测试

**单元测试**（在 `src/lib.rs` 中）：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::open("/tmp/test.kv").unwrap();
        store.set("key".into(), "value".into()).unwrap();
        assert_eq!(store.get("key"), Some(&"value".to_string()));
    }
}
```

- `#[cfg(test)]` 只在 `cargo test` 时编译，不影响发布二进制的大小。
- 单元测试可以访问私有函数（因为 `use super::*`），适合白盒测试。

**集成测试**（在 `tests/integration_test.rs` 中）：

```rust
use mini_kv_store::KvStore;

#[test]
fn test_full_workflow() {
    let mut store = KvStore::open("/tmp/integration.kv").unwrap();
    // 只能调用 pub API
}
```

- 集成测试作为外部用户使用 crate，只能访问公开 API。
- 适合测试完整的用户工作流和多模块协作。

### 7. CLI 交互设计

```rust
fn main() {
    let mut store = KvStore::open("data.kv").unwrap();
    loop {
        print!("kv> ");
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        // 解析命令 + 分发 + 执行
    }
}
```

- **REPL 模式**：Read（读取输入）、Eval（解析命令）、Print（输出结果）、Loop（循环）。
- **命令解析**：使用 `splitn` 限制分割次数，支持 value 中包含空格。
- **EOF 处理**：`read_line` 返回 `Ok(0)` 表示 EOF（Ctrl+D），优雅退出。
- **自动保存**：退出前自动调用 `save()`，防止数据丢失。

---

## 设计决策

### 数据存储格式

选择简单的 **行文本格式**：`key|value`

| 考量维度 | 选择 | 理由 |
|----------|------|------|
| 可读性 | 纯文本，`key|value` | 方便用文本编辑器直接查看和修改 |
| 实现复杂度 | 极低 | 一行代码解析，无需引入 serde 等依赖 |
| 性能 | 全量读写 | 教学场景数据量小，足够 |
| 局限性 | 不支持 `\|` 和换行符 | 明确记录为已知限制 |

**为什么不选 JSON？**
- JSON 需要引入 `serde` + `serde_json` 依赖。
- 对于简单的字符串键值对，`key|value` 格式更直观。
- 如果想升级格式，保留接口不变，只修改 `save()` 和 `load()` 实现即可。

### 内存存储策略

**全量加载到内存**：启动时将整个文件读入 `HashMap`，关闭时写回文件。

| 优势 | 劣势 |
|------|------|
| 实现简单 | 数据量受内存限制 |
| 读写极快（内存操作） | 崩溃可能丢失未保存数据 |
| 无需实现缓存策略 | 启动时间随文件增大而线性增长 |

### API 设计原则

1. **显式所有权**：`set` 取 `String`，明确表示数据将被持有。
2. **零拷贝读取**：`get` 返回 `Option<&String>`，避免不必要的分配。
3. **命令查询分离**（CQS）：`set`/`remove` 修改状态但不返回旧值（除非是 `remove` 返回被删值），`get`/`list` 查询但不修改。
4. **最小惊讶原则**：`open` 在文件不存在时创建空存储（而非报错），符合直觉。

---

## 当前设计局限

### 1. 并发访问

**问题**：`KvStore` 没有实现 `Send` + `Sync`，不支持多线程访问。
当前所有操作都在单一线程中执行。如果两个线程尝试同时修改 store，Rust 编译器会阻止（因为 `&mut self` 只能有一个）。

**影响**：不能用于多线程服务器场景。

### 2. 数据一致性

**问题**：`set`/`remove` 修改内存后不自动保存到磁盘。如果程序崩溃，未保存的数据会丢失。
持久化操作（`save`）与修改操作（`set`/`remove`）是分离的，不属于同一个原子操作。

**影响**：
- 用户必须显式调用 `save` 或依赖退出时的自动保存。
- 没有 Write-Ahead Log（WAL）来保证崩溃恢复。

### 3. 持久化策略

**问题**：每次 `save()` 都全量重写整个文件，当数据量大时效率极低。
- 没有增量写入机制
- 没有压缩（compaction）策略
- 保存过程中如果崩溃，文件可能损坏（先清空后写入的顺序问题）

**影响**：
- 不适合大数据量场景。
- 不适合需要频繁持久化的场景。

### 4. 无事务支持

**问题**：没有提供 `begin` / `commit` / `rollback` 事务语义。
多个 `set`/`remove` 操作之间没有原子性保证。

**影响**：无法保证一组操作的原子性，不适合需要事务语义的场景。

### 5. 数据格式的局限性

**问题**：当前 `key|value` 格式：
- key 或 value 中不能包含 `|` 字符
- key 或 value 中不能包含换行符
- 没有类型信息（所有值都是字符串）
- 没有元数据（创建时间、过期时间等）

**影响**：
- 只能存储纯文本键值对。
- 无法区分"值恰好是 `nil`"和"key 不存在"（当前用 `Option` 区分，但文件格式无法表达）。

### 6. 没有索引

**问题**：只能按 key 精确查询，不支持：
- 范围查询
- 前缀查询
- 按 value 搜索
- 排序输出（当前 `list` 在内存中排序，大文件场景效率低）

### 7. 错误恢复

**问题**：`load()` 遇到格式异常的行只打印警告并跳过，但没有提供自动修复或数据恢复机制。

---

## 如何进一步演化

### 阶段一：增强可靠性

#### WAL（Write-Ahead Log）日志

在当前实现中，`save()` 全量覆盖文件。引入 WAL 后：

```
写入流程:
1. 将操作记录追加到 WAL 文件 (append-only, 速度快)
2. 更新内存中的 HashMap
3. 定期将内存快照写入主数据文件 (checkpoint)
4. 清理旧的 WAL 记录

崩溃恢复:
1. 读取主数据文件，恢复到最近一次 checkpoint
2. 重放 (replay) WAL 中 checkpoint 之后的操作记录
```

WAL 的好处：
- 写入性能高（顺序追加 vs 随机写入）
- 崩溃后可恢复到最近一次操作
- 为事务支持打下基础

#### 原子文件写入

使用"写临时文件 + 原子重命名"策略避免文件损坏：

```rust
fn save_atomic(&self) -> Result<()> {
    let tmp = format!("{}.tmp", self.file_path);
    // 写入临时文件
    write_to(&tmp)?;
    // 原子重命名
    std::fs::rename(&tmp, &self.file_path)?;
    Ok(())
}
```

### 阶段二：支持并发

#### `Arc<RwLock<HashMap>>` 替换单线程设计

```rust
pub struct KvStore {
    data: Arc<RwLock<HashMap<String, String>>>,
    file_path: String,
}
```

- `RwLock` 允许多个读者同时访问，写者独占。
- 适合"读多写少"的场景。

#### 网络接口

在存储引擎之上增加网络层，提供远程访问：

```
Client (TCP/HTTP) ──→ Server ──→ KvStore
```

可选方案：
- **自定义 TCP 协议**：使用 `tokio` + length-prefixed framing
- **HTTP REST API**：使用 `axum` 或 `actix-web`
- **gRPC**：使用 `tonic`

### 阶段三：存储引擎升级

#### LSM-Tree 存储引擎

Log-Structured Merge-Tree 是现代 KV 存储（如 RocksDB、LevelDB）的核心数据结构：

```
写入路径:
  MemTable (内存中的有序数据结构, 如 BTreeMap)
      │
      ▼ (flush, 当 MemTable 达到阈值)
  SSTable Level 0 (磁盘上的有序不可变文件)
      │
      ▼ (compaction)
  SSTable Level 1, 2, ... (层级合并，每层容量逐级增大)

读取路径:
  MemTable → SSTable L0 → SSTable L1 → ... → 找到或返回 None
```

LSM-Tree 的优势：
- 写入极快（顺序写）
- 支持大容量数据（TB 级别）
- 天然支持范围查询
- 压缩过程中可以清理过期数据

#### 布隆过滤器（Bloom Filter）

在 SSTable 级别增加布隆过滤器，加速"key 不存在"的判断，减少不必要的磁盘读取。

### 阶段四：高级特性

1. **事务支持**：MVCC（Multi-Version Concurrency Control）实现快照隔离。
2. **过期机制**：TTL（Time-To-Live），自动清理过期 key。
3. **数据压缩**：使用 snappy / zstd 压缩磁盘上的数据块。
4. **复制与分片**：Raft 共识算法实现多副本一致性，一致性哈希实现数据分片。
5. **监控与指标**：暴露 Prometheus metrics，记录 QPS、延迟、存储大小等。

### 演化路线图

```
当前版本 (v0.1)              未来版本
──────────────────────────────────────────────
内存 HashMap                  MemTable + SSTable
全量文件读写                   WAL + Checkpoint
单线程                       Arc<RwLock> / 异步
纯文本格式                    二进制格式 (protobuf)
命令行 REPL                   网络 API (HTTP/gRPC)
无事务                        MVCC 事务
手动持久化                     自动定期刷盘
```

---

## 明确声明

> **重要：本项目的定位与限制**
>
> Mini KV Store 是一个 **教学练习项目**，旨在帮助 Rust 学习者理解所有权、借用、集合、错误处理、模块化、测试和 CLI 开发等核心概念。
>
> **本项目不是生产级数据库**，存在以下明确限制：
>
> - 不支持并发访问
> - 没有事务保证（ACID 中的任何一项）
> - 没有 WAL 日志和崩溃恢复机制
> - 持久化策略简单（全量重写），不适合大数据量场景
> - 数据格式限于纯文本，不支持二进制数据
> - 没有访问控制和认证机制
> - 没有性能优化（无缓存、无索引、无压缩）
>
> **绝对不要**在任何生产环境、重要数据或有可靠性要求的场景中使用本项目。
>
> 如需生产级 KV 存储，请考虑：
> - [RocksDB](http://rocksdb.org/) —— 嵌入式 LSM-Tree 存储引擎
> - [sled](https://github.com/spacejam/sled) —— 纯 Rust 的嵌入式数据库
> - [TiKV](https://tikv.org/) —— 分布式事务键值存储
> - [Redis](https://redis.io/) —— 内存数据结构存储
> - [etcd](https://etcd.io/) —— 分布式一致性键值存储

---

## 运行与测试

### 编译

```bash
cd /path/to/05_mini_kv_store
cargo build
```

### 运行 REPL

```bash
cargo run
```

进入交互式命令行后：

```
kv> set name Alice
OK: name = Alice
kv> set age 30
OK: age = 30
kv> get name
Alice
kv> list
共 2 条记录:
  age = 30
  name = Alice
kv> remove age
已删除: age = 30
kv> save
已保存 1 条记录到 data.kv
kv> quit
数据已自动保存到 data.kv（1 条记录）
再见！
```

### 运行测试

```bash
# 运行所有测试（单元测试 + 集成测试）
cargo test

# 仅运行单元测试
cargo test --lib

# 仅运行集成测试
cargo test --test integration_test

# 显示测试输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_set_and_get
```

### 预期测试输出

```
running 7 tests
test tests::test_empty_and_len ... ok
test tests::test_list_sorted ... ok
test tests::test_load_malformed_lines ... ok
test tests::test_open_new_store ... ok
test tests::test_remove ... ok
test tests::test_save_and_load ... ok
test tests::test_set_and_get ... ok

test result: ok. 7 passed; 0 failed
```

---

## 代码详解

### `KvStore` 结构体设计

```rust
#[derive(Debug)]
pub struct KvStore {
    data: HashMap<String, String>,
    file_path: String,
}
```

**为什么 `data` 存储 `String` 而非 `&str`？**
- `&str` 需要有生命周期标注（如 `HashMap<&'a str, &'a str>`），这会让 `KvStore` 携带生命周期参数，复杂性大增。
- `String` 拥有数据的所有权，`KvStore` 的生命周期简单清晰。
- 持久化时可以直接写入 `String`，无需额外转换。

### `open()` —— 构造函数

```rust
pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let mut store = KvStore {
        data: HashMap::new(),
        file_path: path.to_string(),
    };
    if Path::new(path).exists() {
        store.load()?;
    }
    Ok(store)
}
```

**设计要点**：
1. 先创建空的 store，再根据文件是否存在决定是否加载 —— 避免文件不存在时出错。
2. `load()` 如果失败，错误会通过 `?` 传播给调用者。
3. `file_path` 立即保存为 `String`，后续操作无需关心 path 参数的生命周期。

### `set()` —— 插入或更新

```rust
pub fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
    self.data
        .entry(key)
        .and_modify(|v| *v = value.clone())
        .or_insert(value);
    Ok(())
}
```

**Entry API 三步走**：
1. `entry(key)` —— 找到或预留 key 的位置（一次哈希）
2. `and_modify(|v| *v = value.clone())` —— 如果 key 存在，更新值
3. `or_insert(value)` —— 如果 key 不存在，插入新值

**为什么 `set` 总是返回 `Ok(())`？**
当前实现中 `set` 不会失败。返回 `Result` 是为了保持 API 一致性，并为未来的验证逻辑（如 key/value 长度限制）预留扩展空间。

### `get()` —— 查询

```rust
pub fn get(&self, key: &str) -> Option<&String> {
    self.data.get(key)
}
```

**为什么要返回 `Option<&String>`？**
- `Option` 表达 key 可能不存在。
- `&String` 是对内部数据的引用，不会复制数据。
- 调用者可以决定是否 clone：`store.get("key").cloned()` 获得 `Option<String>`。

**借用规则的实际效果**：

```rust
let mut store = KvStore::open("test.kv").unwrap();
store.set("a".into(), "1".into()).unwrap();

let val = store.get("a");  // 不可变借用 &self
// store.set("b".into(), "2".into()).unwrap();  // 编译错误！不能同时持有可变和不可变引用
println!("{}", val.unwrap());
// val 在这里离开作用域，不可变借用结束
store.set("b".into(), "2".into()).unwrap();  // 现在可以了
```

### `remove()` —— 删除

```rust
pub fn remove(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    Ok(self.data.remove(key))
}
```

- `HashMap::remove` 返回 `Option<V>`：key 存在时返回 `Some(value)`，不存在时返回 `None`。
- 返回 `Option<String>` 而非 `Option<&String>`，因为被删除的值从 HashMap 中移出，所有权需要转移给调用者。

### `list()` —— 列出所有条目

```rust
pub fn list(&self) -> Vec<(&String, &String)> {
    let mut entries: Vec<(&String, &String)> = self.data.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    entries
}
```

- 收集所有条目的引用到 `Vec` 中，零数据拷贝。
- 按 key 排序输出，方便阅读。排序操作在 `&(&String, &String)` 引用上进行，不影响原始数据。

### `save()` —— 持久化

```rust
pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&self.file_path)?;
    for (key, value) in &self.data {
        writeln!(file, "{}|{}", key, value)?;
    }
    Ok(())
}
```

- `File::create` 会截断已存在的文件（或创建新文件）。
- `writeln!` 自动在每行末尾追加换行符。
- 错误通过 `?` 传播。

**当前的原子性问题**：
`File::create` 立即清空原文件。如果在写入过程中崩溃，数据会丢失。改进方案见上文"原子文件写入"。

### `load()` —— 加载数据

```rust
pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&self.file_path).exists() {
        return Ok(());
    }
    let file = File::open(&self.file_path)?;
    let reader = BufReader::new(file);
    self.data.clear();
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        match trimmed.split_once('|') {
            Some((key, value)) => {
                self.data.insert(key.to_string(), value.to_string());
            }
            None => {
                eprintln!("警告: 第 {} 行格式不正确...", line_num + 1);
            }
        }
    }
    Ok(())
}
```

**设计要点**：
1. 使用 `BufReader` 逐行读取，内存友好（不会一次性将整个文件读入内存再解析）。
2. `split_once('|')` 按第一个 `|` 分割，这意味着 value 中可以包含 `|` 字符（这是一个灵活的设计选择）。
3. 格式异常的行只打印警告不中断加载 —— 容错设计，尽量多地恢复数据。
4. `self.data.clear()` 在加载前清空，确保 `load()` 的语义是"完全替换"而非"追加合并"。

---

## 使用示例

### 作为库使用

```rust
use mini_kv_store::KvStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = KvStore::open("config.kv")?;

    // 写入配置
    store.set("host".into(), "localhost".into())?;
    store.set("port".into(), "8080".into())?;

    // 读取配置
    if let Some(host) = store.get("host") {
        println!("服务器地址: {host}");
    }

    // 持久化
    store.save()?;

    Ok(())
}
```

### 命令行使用示例

```bash
# 基本 CRUD 操作
$ cargo run
kv> set username alice
OK: username = alice
kv> set email alice@example.com
OK: email = alice@example.com
kv> get username
alice
kv> list
共 2 条记录:
  email = alice@example.com
  username = alice
kv> remove email
已删除: email = alice@example.com
kv> save
已保存 1 条记录到 data.kv
kv> quit
数据已自动保存到 data.kv（1 条记录）
再见！

# 重新启动，验证数据持久化
$ cargo run
已打开存储文件: data.kv
从文件中加载了 1 条记录
kv> get username
alice
kv> quit
```

### 数据文件格式

运行后生成的 `data.kv` 文件示例：

```
host|localhost
port|8080
username|alice
```

纯文本格式，可直接用文本编辑器查看和修改（注意：手动编辑时不要破坏 `key|value` 格式）。

---

## 扩展阅读

### 相关 Rust crate

| Crate | 用途 | 与本项目的关系 |
|-------|------|---------------|
| `serde` + `serde_json` | 序列化/反序列化 | 可替代当前 `key\|value` 格式，支持复杂数据类型 |
| `thiserror` | 自定义错误类型 | 替代 `Box<dyn Error>`，提供更丰富的错误信息 |
| `anyhow` | 应用级错误处理 | 简化 `main` 函数中的错误传播 |
| `clap` | 命令行参数解析 | 为 CLI 增加子命令和选项（如 `kv set key value`）|
| `tokio` | 异步运行时 | 实现异步网络服务 |
| `sled` | 嵌入式数据库 | 了解生产级 Rust KV 存储的设计 |
| `tempfile` | 临时文件 | 改进 `save` 的原子性（写临时文件再重命名）|

### 推荐阅读

- 《Rust 程序设计语言》第 8 章（集合）、第 9 章（错误处理）、第 11 章（测试）
- [Rust HashMap Entry API 文档](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
- [Designing Data-Intensive Applications](https://dataintensive.net/) —— 了解分布式数据系统的设计原理
- [Bigtable: A Distributed Storage System for Structured Data](https://research.google/pubs/pub27898/) —— LSM-Tree 的工业级实现

---

## 从 Python、C、C++ 迁移时值得注意的设计差异

### 1. `Result` 贯穿全部 API，而非全局错误码

C 语言中函数通过返回值（`NULL`、`-1`）表示错误，具体原因需检查全局变量 `errno`，且 `errno` 在多次系统调用间可能被覆盖。C++ 的异常可以跨越多个调用栈层次，但异常安全的代码编写复杂（RAII、noexcept）。Python 的异常同样可以跨越栈帧，但"哪些函数可能抛出什么异常"完全依赖文档而非类型系统。Rust 的方案是：每个可能失败的操作返回 `Result<T, E>`，编译器强制调用方处理。本项目从 `KvStore::open` 到 `save`、`load`，所有 I/O 操作都返回 `Result<_, Box<dyn Error>>`，调用方用 `?` 简洁传播或用 `match` 精确处理。不存在"忘记检查 errno"或"漏接异常"的问题。

### 2. 所有权决定存储数据用 `String` 而非 `&str`

Python 中所有字符串都是对象，赋值和传参都是引用计数操作，程序员不需要思考"数据归谁所有"。C 中 `char*` 可以指向栈、堆或静态区的内存，谁负责释放是永恒的难题。Rust 迫使我们做出清晰决议：`KvStore` 的 `data: HashMap<String, String>` —— 键和值都是 `String`，表示存储层拥有数据的所有权。`set(&mut self, key: String, value: String)` 取 `String` 而非 `&str`，表示"我会接管这些数据"。同时，`get(&self, key: &str) -> Option<&String>` 返回引用，让调用者零拷贝读取但不必担心内存释放。这种设计在 C 中需要大量注释来传达意图，在 Rust 中函数签名自己就是最准确的文档。

### 3. 枚举表达状态优于布尔标志和魔数

C 中用 `int` 常量（如 `#define STATE_OK 0`）表示状态，任何 `int` 都能通过编译期检查，实际可能传入非法值。C++ 和 Python 的枚举提供了类型安全标签，但不能携带数据。Rust 的枚举是完整的"和类型"：本项目虽未直接使用复杂枚举，但其 `Option<&String>` 返回类型本身就是枚举的力量——它明确区分了"key 存在且有值"和"key 不存在"，无需使用 `None`/`NULL` 哨兵或额外布尔返回值。如果你扩展本项目来支持过期键，可以用 `enum EntryState { Active(String), Expired, Tombstone }` 替代三个布尔标志的混乱组合，编译器会确保你处理了所有可能状态。

### 4. Cargo 统一管理所有依赖并锁定版本

Python 项目的依赖管理常常分散在 `requirements.txt`、`setup.py` 和 `pyproject.toml` 之间，版本锁定需要手动维护锁止文件。C 项目依赖系统包管理器（apt、brew）或 CMake 的 `find_package`，跨平台一致性脆弱。Rust 的 Cargo 一个工具解决：`Cargo.toml` 声明语义化版本，`cargo build` 自动解析依赖树，生成 `Cargo.lock` 锁定精确版本（保证可复现构建）。本项目零外部依赖（纯标准库），但如果要升级存储格式到 JSON（引入 `serde`），只需在 `Cargo.toml` 加一行 `serde_json = "1"`，编译时自动下载——无需任何系统级安装步骤。

### 5. `BufReader` 逐行读取的显式控制

Python 的 `for line in open('file')` 自动提供缓冲和逐行迭代，这是极好的体验，但缓冲策略和内存分配对用户完全不透明。Rust 需要显式创建 `BufReader::new(File::open(...)?)` 来获得缓冲读取，略显冗长但带来了精确的控制——你知道每一步的内存分配和 I/O 策略。本项目的 `load` 方法逐行解析 `key|value` 格式，格式错误打印警告继续解析，这是"容错加载"的范例。Python 可以做到同样的事，但 Rust 额外的收获是：`File::open` 返回 `Result`，`line_result?` 传播 I/O 错误，每一步的类型都告诉你"这里可能失败，需要处理"。

### 6. 不存在 null 指针：`Option<T>` 的普遍使用

C 和 C++ 中大量使用 `NULL`/`nullptr` 表示"没有值"，但编译器不会强制你检查——忘记判空是段错误的第一大来源。Python 用 `None`，虽然有 `AttributeError` 但只能在运行时发现。Rust 没有 null：本项目 `get(&self, key: &str) -> Option<&String>` 返回 `Option`，调用方必须处理 `Some` 和 `None` 两种情况。`HashMap::remove` 同样返回 `Option<String>`。编译器强制你检查——如果你试图直接解包 `Option` 而不处理 `None`，代码无法编译。这个设计消除了 Tony Hoare 称为"价值十亿美元的错误"（null reference）在 Rust 中的存在空间。

---

## 总结

本项目通过构建一个麻雀虽小五脏俱全的本地 KV 存储，实践了 Rust 的以下核心概念：

| 概念 | 实践位置 | 关键代码 |
|------|---------|---------|
| 所有权 | `set()` 取 `String` | `self.data.entry(key).or_insert(value)` |
| 借用 | `get()` 返回 `&String` | `self.data.get(key)` |
| 集合 | `HashMap` 存储 + Entry API | `self.data.entry(key).and_modify(...)` |
| 错误处理 | `Box<dyn Error>` + `?` 传播 | `File::open(...)?` |
| 模块化 | `lib.rs` + `main.rs` 分离 | Cargo 自动识别 |
| 测试 | 单元测试 + 集成测试 | `#[cfg(test)]` + `tests/` 目录 |
| 文件 I/O | `BufReader` + `BufWriter` | `reader.lines()` + `writeln!()` |
| CLI | REPL 循环 | `stdin().read_line()` + 命令分发 |

**设计哲学**：
- **简单优先**：选择最简单的方案满足当前需求，预留扩展空间。
- **显式优于隐式**：所有权转移通过函数签名明确表达。
- **零成本抽象**：使用 Entry API 等高级抽象但不牺牲性能。
- **容错但不静默**：格式异常打印警告但不中断加载。

---

*Mini KV Store v0.1 —— 这不是生产级数据库，这是一段 Rust 学习之旅。*
*如果你需要生产级 KV 存储，请使用 RocksDB、sled、Redis 或 TiKV。*
