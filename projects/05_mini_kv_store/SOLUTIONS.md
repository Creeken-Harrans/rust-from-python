# 项目参考实现说明 — Mini KV Store

## 1. 需求拆分

1. 内存键值存储（`HashMap<String, String>`）
2. 支持 `set`、`get`、`remove`、`list` 操作
3. 持久化到文件（启动时加载，写入后保存）
4. CLI 交互入口
5. 错误处理：文件不存在/损坏/权限问题

## 2. 推荐实现顺序

1. `lib.rs`: 定义 `KvStore` 结构体 + `HashMap` 内部存储
2. `lib.rs`: 实现 CRUD 方法
3. `lib.rs`: 实现 `load()` + `save()` 持久化
4. `lib.rs`: 单元测试（内存操作 + 持久化往返）
5. `main.rs`: CLI 解析 + 交互循环
6. 集成测试

## 3. 模块划分

```
src/
├── main.rs    # CLI 入口
├── lib.rs     # KvStore 核心逻辑
└── error.rs   # (可选) 自定义错误类型
```

## 4. 核心数据结构

```rust
pub struct KvStore {
    store: HashMap<String, String>,
    file_path: String,
}

// 文件格式：每行 "key|value"
const SEPARATOR: &str = "|";
```

## 5. 关键函数签名

```rust
impl KvStore {
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>>
    pub fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>>
    pub fn get(&self, key: &str) -> Option<&String>
    pub fn remove(&mut self, key: &str) -> Result<Option<String>, Box<dyn Error>>
    pub fn list(&self) -> Vec<(&String, &String)>
    pub fn save(&self) -> Result<(), Box<dyn Error>>
    pub fn load(&mut self) -> Result<(), Box<dyn Error>>
}
```

## 6. 设计决策

### 为什么返回 `&String` 而非 `String`？

`get()` 返回引用避免复制。调用者若需要拥有所有权的副本，显式 `clone()`。

### 持久化策略：全量重写

每次 `set`/`remove` 后全量序列化整个 HashMap。简单正确，不适合大数据量。大场景可改为：
- 追加日志（WAL）
- 定期 compaction
- `fsync` 保证崩溃一致性

### 为什么用 `|` 分隔符而非 JSON/TOML？

简单文件格式，逐行解析快速，无需引入第三方依赖。缺点：key 或 value 不能包含 `|` 字符。

### 文件损坏/格式错误处理

`load()` 遇到格式错误的行时，记录警告并跳过，而非 panic 或丢弃所有数据——这是"容错加载"策略。

## 7. 关键代码片段

```rust
fn load(&mut self) -> Result<(), Box<dyn Error>> {
    if !Path::new(&self.file_path).exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&self.file_path)?;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        match line.split_once(SEPARATOR) {
            Some((key, value)) => {
                self.store.insert(key.to_string(), value.to_string());
            }
            None => eprintln!("警告: 跳过格式错误行: {}", line),
        }
    }
    Ok(())
}

fn save(&self) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    for (key, value) in &self.store {
        content.push_str(&format!("{}|{}\n", key, value));
    }
    // 原子写入：先写临时文件，再重命名
    let tmp = format!("{}.tmp", self.file_path);
    fs::write(&tmp, &content)?;
    fs::rename(&tmp, &self.file_path)?;
    Ok(())
}
```

## 8. 测试策略

- **CRUD 测试**：set → get → remove → list 全流程
- **持久化往返**：set → save → load (新实例) → get
- **空文件**：open 不存在的文件应返回空 store
- **格式错误**：含 `|||` 或空行的文件应能正确加载有效部分
- **文件权限**：(仅 Linux) 测试只读文件时的错误处理

## 9. 常见失败方式

| 错误 | 原因 | 修复 |
|------|------|------|
| `get` 返回了过时数据 | save 失败但静默 | 检查 save 返回值 |
| key 或 value 包含 `|` | 分隔符冲突 | 改用 JSON/TOML 或转义 |
| 并发写入数据损坏 | 无锁保护 | 声明单线程限制 |
| `save` 中途崩溃导致文件损坏 | 直接覆写原文件 | 使用临时文件 + 原子重命名 |

## 10. 可选扩展

- 日志结构存储（WAL → SST）
- 支持 TTL（过期时间）
- 网络 API（TCP/UDP 服务）
- 并发控制（RwLock）
- 内存淘汰策略（LRU）

## 11. 明确声明：非生产级数据库

本项目的持久化策略（全量重写）、单线程模型、简单文件格式都无法满足生产级数据库的要求。它用于学习 Rust 的文件 I/O、HashMap 操作和错误处理模式。

---

*学习重点：HashMap 作为内存存储、文件持久化的安全写入模式、容错加载策略。明确理解"教学项目"与"生产系统"的边界。*
