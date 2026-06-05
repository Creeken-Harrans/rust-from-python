# 项目参考实现说明 — 并行文本统计

## 1. 需求拆分

1. 多线程读取多个文本文件
2. 每线程统计各自的单词频率
3. 汇总所有结果（词频合并）
4. 输出 Top-N 高频词
5. 正确处理错误（部分文件失败不影响其他文件）

## 2. 推荐实现顺序

1. 单线程版本：词频统计逻辑
2. 引入多线程：`thread::spawn` + `move` closure
3. 结果收集：`Arc<Mutex<HashMap>>` 或 channel
4. 错误聚合：`thread::spawn` 返回 `Result`
5. 优化：批量处理 + 减小锁粒度

## 3. 模块划分

```
src/
└── main.rs    # 所有逻辑（本项目规模适中，未拆 lib.rs）
    ├── 参数解析
    ├── 工作线程 (thread::spawn)
    ├── 结果聚合 (Arc<Mutex<HashMap>> 或 mpsc)
    ├── 输出格式化
    └── 错误处理
```

## 4. 核心数据结构

```rust
// 词频表：单词 -> 出现次数
type WordFreq = HashMap<String, usize>;

// 线程结果：成功返回词频，失败返回错误
type ThreadResult = Result<WordFreq, Box<dyn Error + Send>>;

// 聚合状态：共享的可变词频合并
type SharedFreq = Arc<Mutex<WordFreq>>;
```

## 5. 关键函数签名

```rust
fn count_words(content: &str) -> WordFreq            // 单文件词频统计
fn process_file(path: &str) -> ThreadResult          // 读取+统计单个文件
fn merge_freq(target: &mut WordFreq, source: &WordFreq)  // 合并两个词频表
fn top_n(freq: &WordFreq, n: usize) -> Vec<(&String, &usize)>  // Top-N
```

## 6. 设计决策

### Channel 还是 Arc<Mutex<>>？

| 方案 | 优势 | 劣势 |
|------|------|------|
| `mpsc::channel` | 所有权清晰，无锁 | 需要在收集端合并 |
| `Arc<Mutex<HashMap>>` | 共享写入，实时合并 | 锁粒度控制不当影响性能 |

本项目推荐 **channel + 收集端合并**：
- 每个线程通过 `tx.send(result)` 发送 `WordFreq`
- 主线程 `rx.iter()` 收集所有结果
- 最后一次性 `merge_freq`（无锁竞争）

### 为什么 `thread::spawn` 闭包需要 `move`？

文件内容 `String` 可能较大，通过 `move` 将所有权转移给线程，避免主线程持有不必要的引用。

### 错误聚合策略

部分文件失败不应导致整体失败。每个线程返回 `ThreadResult`，收集端过滤错误并报告。

## 7. 关键代码片段

```rust
let (tx, rx) = mpsc::channel::<ThreadResult>();

for path in file_paths {
    let tx = tx.clone();
    thread::spawn(move || {
        let result = process_file(&path);
        tx.send(result).unwrap(); // 发送失败意味着接收端已关闭
    });
}
drop(tx); // 关闭发送端 → rx.iter() 在所有线程结束后自然终止

let mut merged = HashMap::new();
for result in rx {
    match result {
        Ok(freq) => merge_freq(&mut merged, &freq),
        Err(e) => eprintln!("警告: {}", e),
    }
}
```

## 8. 测试策略

- 创建多个临时文件（不同内容）
- 验证词频合并正确
- 验证部分文件缺失不导致整体失败
- 验证空文件的词频表为空
- 手动验证线程数 ≤ CPU 核心数时性能提升

## 9. 常见失败方式

| 错误 | 原因 | 修复 |
|------|------|------|
| `tx.send` panic | 接收端提前 drop | 确保 `rx` 在 `tx.clone()` 之前创建 |
| 死锁 | `Mutex` 嵌套 lock | 同一线程不重复 lock 同一个 Mutex |
| 非 UTF-8 文件 panic | `read_to_string` | 用 `read` + `from_utf8_lossy` |
| 性能反降 | 锁竞争过重 | 改用 channel 一次性发送结果 |

## 10. 可选扩展

- 使用 `rayon` crate 简化并行处理
- 支持正则表达式分词
- 忽略停用词（stop words）
- 多语言分词（Unicode 边界）
- 结果可视化输出

---

*重点学习：move 闭包的所有权转移、channel vs 共享状态的选择、错误聚合。Rust 的并发不会"自动快"，需要根据数据结构选择合适的并发模式。*
