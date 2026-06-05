# Mini KV Store — 练习指南

## 项目概述

你已阅读了 `README.md`、`src/lib.rs` 和 `src/main.rs`。这个本地键值存储采用 **library-first** 设计，核心存储逻辑在 `lib.rs` 中，REPL 交互在 `main.rs` 中。请在不直接复制粘贴的前提下完成以下练习。

---

## Level 1: 基础练习（理解现有代码）

### L1-1: 运行与观察

1. 运行 `cargo run` 进入 REPL 交互模式
2. 使用 `set key value` 添加几条记录
3. 使用 `get key` 查询记录
4. 使用 `list` 查看所有记录
5. 使用 `remove key` 删除记录
6. 使用 `save` 手动保存，用文本编辑器打开 `data.kv` 查看文件格式
7. 退出后重新运行，验证数据持久化
8. 运行 `cargo test` 观察所有测试通过

**学习点**: REPL 模式、`key|value` 文本格式、文件持久化。

### L1-2: 追踪 `get` 返回值的生命周期

在 `lib.rs` 中：

```rust
pub fn get(&self, key: &str) -> Option<&String>
```

1. 返回值 `&String` 的生命周期绑定到哪个参数？（`&self`）
2. 如果调用者在持有 `get` 返回的引用时尝试 `set`，编译器会阻止吗？为什么？
3. 如果 `get` 返回 `Option<String>`（owned），会有什么性能影响？

写一个测试验证第 2 点（编译器拒绝同时持有不可变引用和可变引用）。

**学习点**: 借用规则在 API 设计中的体现、返回引用 vs 返回 owned value。

### L1-3: 理解 `Entry` API

在 `set` 方法中：

```rust
self.data
    .entry(key)
    .and_modify(|v| *v = value.clone())
    .or_insert(value);
```

1. 这个链式调用做了几次哈希查找？（1 次还是 2 次？）
2. `and_modify` 的闭包参数 `|v|` 是什么类型？（`&mut String`）
3. 将 Entry API 版本改为传统的 `contains_key` + `insert` 模式，比较代码量和语义清晰度

**学习点**: HashMap Entry API、哈希查找次数优化。

### L1-4: 理解 `Box<dyn Error>`

在 `KvStore::open()` 中：

```rust
pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>>
```

1. `Box<dyn Error>` 是 trait object 还是泛型？（提示：`dyn` 关键字）
2. I/O 错误（`std::io::Error`）是如何自动转换为 `Box<dyn Error>` 的？
3. 如果要在 `Box<dyn Error>` 中增加自定义错误类型，需要做什么？

**学习点**: trait object、错误类型转换、`From` trait。

### L1-5: 添加 `len` 命令

为 REPL 添加 `len` 命令，返回当前存储的条目数量：

实现要点：
- 在 `KvStore` 中添加 `pub fn len(&self) -> usize` 方法
- 在 `main.rs` 的命令匹配中添加 `"len"` 分支
- 为 `len()` 编写单元测试

**学习点**: API 扩展、命令分发模式、测试先行。

---

## Level 2: 功能扩展（编写新代码）

### L2-1: 实现原子保存

当前 `save()` 直接覆盖原文件。如果在写入过程中崩溃，文件会损坏。实现原子保存：

```rust
fn save_atomic(&self) -> Result<(), Box<dyn Error>> {
    let tmp = format!("{}.tmp", self.file_path);
    // 写入临时文件
    // ...
    // 原子重命名
    std::fs::rename(&tmp, &self.file_path)?;
    Ok(())
}
```

测试要点：
- 验证写入过程中非临时文件不会被修改
- 验证 `rename` 的原子性（在 POSIX 系统上，`rename` 是原子的）

**学习点**: 原子文件写入、崩溃一致性、临时文件模式。

### L2-2: 添加过期键（TTL）

为键添加可选的过期时间：

```rust
struct ValueEntry {
    value: String,
    expires_at: Option<Instant>,
}
```

实现要点：
- `set` 命令支持 `set key value EX 3600`（3600 秒后过期）
- `get` 时检查是否过期，过期则返回 `None` 并删除
- `list` 不显示已过期的键（或标注"已过期"）
- 过期键在文件中如何表示？需要在 `load` 时处理吗？

**学习点**: 时间处理、惰性删除、存储格式演化。

### L2-3: 实现键名验证

当前 `set` 接收任意字符串作为 key。添加验证规则：

- key 不能为空
- key 不能包含 `|` 字符（与分隔符冲突）
- key 长度不能超过 256 字符
- value 长度不能超过 1MB

```rust
pub fn set(&mut self, key: String, value: String) -> Result<(), KvError>
```

将返回类型从 `Result<(), Box<dyn Error>>` 改为 `Result<(), KvError>`，需要先定义 `KvError` 枚举。

**学习点**: 自定义错误类型、输入验证、API 演化。

### L2-4: 实现 `count` 方法

统计 value 中出现特定子串的频率：

```
kv> count Rust
包含 "Rust" 的值有 3 个
```

实现要点：
- 在 `KvStore` 中添加 `pub fn count_values_containing(&self, sub: &str) -> usize`
- 遍历所有 value，统计包含子串的数量
- 使用 `filter` + `count` 迭代器组合

**学习点**: 迭代器链、字符串搜索、聚合查询。

---

## Level 3: 设计思维（架构与扩展）

### L3-1: 日志结构存储引擎

当前使用全量重写。设计一个日志结构存储：

```
写入流程：
  1. 追加操作记录到 WAL 文件
  2. 更新内存 HashMap
  3. 定期 checkpoint（全量快照）
  4. 清理旧 WAL

崩溃恢复：
  1. 读取最近的 checkpoint
  2. 重放 checkpoint 之后的 WAL 记录
```

- WAL 记录的格式设计
- checkpoint 的触发条件（时间间隔？操作数？）
- 如何确保 WAL 写入的顺序性？

**不要求完整实现**，设计数据结构和关键函数签名。

**学习点**: WAL 日志、checkpoint、崩溃恢复。

### L3-2: 并发读写设计

设计一个支持多线程并发读写的 `ConcurrentKvStore`：

```rust
pub struct ConcurrentKvStore {
    data: Arc<RwLock<HashMap<String, String>>>,
    file_path: String,
}
```

- 为什么用 `RwLock` 而不是 `Mutex`？
- 写操作（set/remove）和持久化（save）如何协调？
- 多个读者和一个写者并发时，行为如何？

设计 API 签名并标注并发语义。

**学习点**: `RwLock`、读写锁、并发 API 设计。

### L3-3: 网络服务层

设计一个基于 TCP 的网络接口，使 KvStore 可以远程访问：

```
协议：简单文本协议
  SET key value\n → OK\n 或 ERR <msg>\n
  GET key\n → value\n 或 NIL\n
  DEL key\n → OK\n 或 NIL\n
```

- 使用 `std::net::TcpListener` 还是 `tokio::net::TcpListener`？
- 每个连接一个线程还是异步处理？
- 如何在多个连接间共享 KvStore？

**不要求完整实现**，设计架构和关键代码片段。

**学习点**: 网络协议设计、连接处理模型、异步 vs 多线程。

---

## 检查清单

完成上述练习后，你应该能够：

- [ ] 理解 REPL 交互模式的设计和实现
- [ ] 掌握 `HashMap` 的 Entry API 和所有权语义
- [ ] 区分 `&self` vs `&mut self` 的方法调用约束
- [ ] 使用 `Box<dyn Error>` 统一错误类型
- [ ] 实现原子文件写入（临时文件 + rename）
- [ ] 设计自定义错误类型枚举
- [ ] 理解 WAL 日志和崩溃恢复的基本原理
- [ ] 评估 `RwLock` 在读写混合场景下的适用性
- [ ] 设计简单的文本网络协议
