# 并行文本统计器 (Parallel Text Stats)

## 目录

1. [项目目标](#项目目标)
2. [知识点清单](#知识点清单)
3. [架构设计](#架构设计)
4. [线程安全设计](#线程安全设计)
5. [运行与测试](#运行与测试)
6. [代码详解](#代码详解)
7. [运行结果示例](#运行结果示例)
8. [扩展方向](#扩展方向)
9. [常见问题](#常见问题)
10. [总结](#总结)

---

## 项目目标

本项目的核心目标是通过一个 **并行文本统计器** 综合练习 Rust 并发编程的核心知识点。具体而言：

- **多线程处理**：将多段文本分配给不同的线程并行统计，充分发挥多核 CPU 的性能优势。
- **两种并发模型对比**：同时演示"消息传递"（message passing）和"共享状态"（shared state）两种 Rust 并发模型，让学习者理解各自的适用场景与 trade-off。
- **错误处理**：在线程中处理可能的错误（如空文本），并通过 channel 将错误信息传递回主线程。
- **性能对比**：提供单线程与多线程的执行时间对比，直观感受并行带来的加速效果。

这不是一个"生产级"的文本分析工具，而是一个 **教学练习项目**，旨在巩固 Rust 的以下并发知识：
`std::thread`、`std::sync::mpsc`、`Arc`、`Mutex`、`move` 闭包、`Send` / `Sync` trait。

---

## 知识点清单

### 1. `std::thread::spawn` —— 线程创建与管理

```rust
thread::spawn(move || {
    // 线程执行的代码
});
```

- **`move` 关键字**：强制闭包获取所用变量的所有权。这在多线程场景中至关重要 —— 如果闭包只是借用变量，主线程可能在子线程结束前就释放了该变量，造成悬垂引用（dangling reference）。Rust 编译器在缺少 `move` 时会报错，强制开发者显式处理所有权问题。
- **`JoinHandle`**：`spawn` 返回 `JoinHandle<T>`，调用 `join()` 等待线程结束并获取返回值（或捕获 panic）。

### 2. `std::sync::mpsc` —— 多生产者单消费者通道

```rust
let (tx, rx) = mpsc::channel();
let tx_clone = tx.clone();    // 多生产者
thread::spawn(move || { tx_clone.send(data).unwrap(); });
drop(tx);                      // 必须丢弃原始 sender
for received in rx { /* ... */ }
```

- **mpsc** = Multiple Producer, Single Consumer。
- `tx.clone()` 创建多个 sender，每个线程持有一个。
- `drop(tx)` 是 **必须的**：如果原始 sender 没有被 drop，接收端 `rx` 将永远阻塞等待，因为 channel 不知道是否还会有新消息。
- `rx` 实现了 `Iterator` trait，可以直接用 `for` 循环消费所有消息。

### 3. `Arc<T>` —— 原子引用计数

```rust
let shared = Arc::new(MyData::new());
let clone1 = Arc::clone(&shared);
thread::spawn(move || { /* 使用 clone1 */ });
```

- `Rc<T>` 使用非原子操作维护引用计数，性能更好，但不是 `Send`（不能跨线程）。
- `Arc<T>` 使用 **原子操作**（atomic operations）维护引用计数，保证多线程环境下的安全性，因此实现了 `Send` 和 `Sync`。
- `Arc::clone()` 只增加引用计数，不深拷贝数据 —— 多个线程共享同一块堆内存。

### 4. `Mutex<T>` —— 互斥锁

```rust
let mutex = Mutex::new(data);
let mut guard = mutex.lock().unwrap();  // 获取锁
*guard = new_data;                       // 修改被保护的数据
// guard 在此离开作用域，自动释放锁
```

- **内部可变性**（interior mutability）：`Mutex<T>` 允许通过不可变引用修改内部数据。
- **RAII 解锁**：`lock()` 返回 `MutexGuard<T>`，在其 `Drop` 时自动释放锁。不会出现"忘记解锁"的问题。
- **PoisonError**：如果持有锁的线程 panic，Mutex 会被"毒化"（poisoned），后续 `lock()` 返回 `Err(PoisonError)`。

### 5. `Send` 与 `Sync` trait

- **`Send`**：拥有该类型所有权的值可以安全地转移到另一个线程。大多数 Rust 类型都是 `Send`，但 `Rc<T>` 不是。
- **`Sync`**：该类型的不可变引用可以安全地在多个线程间共享。`Mutex<T>` 是 `Sync`（当 T 是 `Send` 时），`RefCell<T>` 不是。
- **编译期检查**：这两个 trait 是 **自动派生**（auto-trait）的，编译器在编译时检查 —— 如果你尝试在线程间传递非线程安全的类型，会得到编译错误而非运行时的数据竞争。

### 6. 所有权转移与 `move` 语义

Rust 的并发模型建立在所有权系统之上：

| 场景 | 模式 | 所有权去向 |
|------|------|-----------|
| `thread::spawn(move \|\| ...)` | 转移给线程 | 数据离开当前作用域，进入新线程 |
| `channel.send(data)` | 通过 channel 转移 | 数据从发送线程转移到接收线程 |
| `Arc::clone(&arc)` | 共享所有权 | 引用计数+1，多线程共享同一数据 |

**核心原则**：要么共享不可变引用（通过 `Arc`），要么独占可变引用（通过 `Mutex` 或 `channel` 转移所有权）。不存在"多个线程同时持有可变引用"的情况。

### 7. 错误聚合

在线程环境中，单个线程的错误不应导致整个程序崩溃。本项目演示了如何：
- 在线程中捕获错误并通过 `Result` 发送回主线程。
- 主线程收集所有错误并统一报告。
- 区分"成功的结果"和"失败的错误"，分别聚合。

---

## 架构设计

### 整体架构图

```
┌─────────────────────────────────────────────────────┐
│                     main()                          │
│                                                     │
│  1. 准备样本数据(samples: Vec<(String, String)>)     │
│  2. 调用 run_single_threaded()  ← 单线程对照         │
│  3. 调用 run_message_passing() ← 方案A               │
│  4. 调用 run_shared_state()    ← 方案B               │
│  5. 打印线程安全要点总结                              │
└─────────────────────────────────────────────────────┘
          │                        │
          ▼                        ▼
┌─────────────────┐    ┌──────────────────────┐
│  方案A: 消息传递  │    │  方案B: 共享状态       │
│                 │    │                      │
│  main           │    │  main                │
│   │             │    │   │                  │
│   ├─ spawn(t1)  │    │   ├─ Arc::clone()    │
│   ├─ spawn(t2)  │    │   ├─ spawn(t1)       │
│   ├─ spawn(t3)  │    │   ├─ spawn(t2)       │
│   └─ rx 收集     │    │   ├─ join_all()      │
│                 │    │   └─ 读取共享HashMap   │
│  tx──→ rx       │    │                      │
│  (channel)      │    │  Arc<Mutex<HashMap>>  │
└─────────────────┘    └──────────────────────┘
```

### 数据流

**方案A（消息传递）数据流**：

```
sample text ──→ worker() ──→ FileStats ──→ tx.send() ──→ channel ──→ rx.recv() ──→ main 聚合
                                  │
                                  └── 错误路径: Err(msg) ──→ channel ──→ main 错误收集
```

**方案B（共享状态）数据流**：

```
sample text ──→ thread ──→ 本地词频统计 ──→ Mutex::lock() ──→ 更新全局 HashMap ──→ 释放锁
                 （并行）       （局部）         （串行化点）        （共享数据）        （自动）
```

### 关键设计决策

| 决策点 | 选择 | 理由 |
|--------|------|------|
| 并发模型A | mpsc channel | Rust 推荐的"通过通信共享内存"模式 |
| 并发模型B | Arc + Mutex | 演示共享内存模式，展示内部可变性 |
| 错误传递 | `Result<FileStats, String>` | 统一通过 channel 传递成功和失败 |
| 数据所有权 | `String`（非 `&str`）| 避免生命周期问题，线程需要 `'static` 数据 |
| 词频统计 | HashMap + Entry API | 高效的单次哈希查找 |
| 高频词选取 | sort + truncate | 简单直观，数据量小无需堆排序 |

---

## 线程安全设计

### 为什么 Rust 的并发是"无畏的"（Fearless Concurrency）

Rust 通过 **类型系统** 在编译期防止数据竞争（data race），而非依赖运行时检测：

1. **所有权规则**：同一时刻，一个值只能有一个所有者。这天然防止了多个线程同时修改同一数据。
2. **借用规则**：同一时刻，要么有多个不可变引用（`&T`），要么有一个可变引用（`&mut T`）。这防止了"读写冲突"。
3. **`Send` / `Sync` trait**：编译器自动检查跨线程传递的类型是否安全。

### 本项目的线程安全实践

```rust
// 1. move 闭包 —— 数据所有权跟随线程
thread::spawn(move || {
    // name 和 text 的所有权已移入此闭包
    // 主线程不能再访问它们 —— 编译器阻止
});

// 2. Arc::clone —— 共享不可变引用（引用计数）
let freq = Arc::clone(&global_freq);
// freq 和 global_freq 指向同一块堆内存
// Arc 的引用计数操作是原子的

// 3. Mutex::lock —— 互斥访问可变数据
let mut map = freq.lock().unwrap();
// 只有一个线程能成功 lock，其他线程在此阻塞
map.entry(word).or_insert(0) += 1;
// MutexGuard drop 时自动释放锁

// 4. channel —— 所有权通过 channel 转移
tx.send(stats).unwrap();
// stats 的所有权从工作线程转移到主线程
// 发送后工作线程不能再访问 stats
```

### 潜在的死锁风险分析

在本项目中 **不存在死锁风险**，原因如下：
- 方案A 使用 channel，不涉及锁。
- 方案B 中每个线程只获取 **一个** Mutex，且锁的持有时间很短（仅在更新 HashMap 时），不存在"持有锁A等锁B"的循环等待。
- `MutexGuard` 实现了 RAII，不会出现忘记释放锁的情况。

---

## 运行与测试

### 编译

```bash
cd /path/to/04_parallel_text_stats
cargo build
```

### 运行

```bash
cargo run
```

### Release 模式（查看真实性能）

```bash
cargo run --release
```

**注意**：在 debug 模式下，线程创建和同步的开销可能超过并行带来的收益。Release 模式启用优化后，才能观察到明显的并行加速效果（尤其在文本量大或数量多时）。

### 预期输出

程序会依次执行三个阶段并输出结果：
1. 单线程对照（输出每个样本的统计 + 耗时）
2. 方案A：消息传递（输出每个样本的统计 + 汇总 + 耗时）
3. 方案B：共享状态（输出全局 Top-10 高频词 + 耗时）
4. 线程安全要点总结

---

## 代码详解

### `FileStats` 结构体

```rust
#[derive(Debug, Clone)]
struct FileStats {
    path: String,
    chars: usize,
    words: usize,
    lines: usize,
    top_words: Vec<(String, usize)>,
}
```

- 派生 `Clone` 是为了在线程间传递（channel 需要所有权的转移）。
- 派生 `Debug` 是为了方便打印调试信息。
- `path` 字段标识文本来源，在多文件场景中非常有用。

### `count_stats()` 函数

这是整个程序的核心统计逻辑：

1. **字符数**：`content.chars().count()` —— 统计 Unicode 字符数（而非字节数）。
2. **单词数**：`content.split_whitespace().count()` —— 按任意空白字符分割。
3. **行数**：`content.lines().count()` —— 按换行符分割。
4. **高频词**：
   - 转小写：`word.to_lowercase()` 确保 "Rust" 和 "rust" 统计为同一词。
   - 去标点：`.filter(|c| c.is_alphanumeric())` 过滤掉标点符号。
   - HashMap 计数：使用 Entry API 的 `or_insert(0)` 模式。
   - 排序取 top-5：`sort_by` 按频率降序排列，`truncate(5)` 取前5个。

### `worker()` —— 线程工作函数

```rust
fn worker(name: String, text: String, tx: mpsc::Sender<Result<FileStats, String>>) {
    if text.trim().is_empty() {
        let _ = tx.send(Err(format!("[{name}]: 文本为空")));
        return;
    }
    let stats = count_stats(&name, &text);
    let _ = tx.send(Ok(stats));
}
```

- 接收 `String` 而非 `&str`：因为线程需要 `'static` 生命周期的数据，`&str` 可能在线程运行时被释放。
- 发送 `Result`：将错误处理推迟到主线程统一处理。

### 方案A：消息传递详解

```rust
fn run_message_passing(samples: &[(String, String)]) {
    let (tx, rx) = mpsc::channel();
    for (name, text) in samples {
        let tx_clone = tx.clone();        // 每个线程一个 sender
        thread::spawn(move || worker(name.clone(), text.clone(), tx_clone));
    }
    drop(tx);                              // 关闭原始 sender
    for received in rx { /* 聚合结果 */ }
}
```

关键要点：
- `tx.clone()` 创建多个 sender —— 这就是 "Multiple Producer" 的含义。
- `drop(tx)` 是 **必须的**。如果忘记，`rx` 会永远阻塞，因为 channel 不知道是否还会有新消息。
- `rx` 的 `for` 循环会在所有 sender 被 drop 且 channel 为空时自动结束。

### 方案B：共享状态详解

```rust
fn run_shared_state(samples: &[(String, String)]) {
    let global_freq = Arc::new(Mutex::new(HashMap::new()));
    for (name, text) in samples {
        let freq = Arc::clone(&global_freq);
        thread::spawn(move || {
            let mut map = freq.lock().unwrap();
            // 更新共享 HashMap
        });
    }
    // join 所有线程后读取结果
}
```

关键要点：
- `Arc::clone()` 只增加引用计数，数据本身只有一份。
- `Mutex::lock()` 是串行化点 —— 同一时刻只有一个线程能进入临界区。
- 临界区内的代码应尽可能简短，以减少锁竞争。

---

## 运行结果示例

下面是一个典型的运行输出（实际数值因系统而异）：

```
并行文本统计器 (Parallel Text Stats)

===== 单线程对照 =====

  [sample_01_english] 字符: 412, 单词: 67, 行数: 1
  [sample_02_mixed] 字符: 364, 单词: 59, 行数: 1
  ...
  单线程耗时: 1.2ms

===== 方案A：消息传递（mpsc channel）=====

  [sample_01_english] 字符: 412, 单词: 67, 行数: 1
    高频词: rust(4), and(4), a(3), to(3), system(2)
  ...
  耗时: 2.8ms

===== 方案B：共享状态（Arc<Mutex<HashMap>>）=====

  线程完成 sample_01_english
  线程完成 sample_02_mixed
  ...
  全局高频词 Top-10（共享状态聚合）:
    rust: 9
    and: 8
    the: 7
    ...
  耗时: 1.5ms
```

**关于耗时**：
- 在文本量较小的场景下，线程创建的开销可能超过并行带来的收益，导致多线程比单线程更慢。这是正常的。
- 随着文本量增大或文件数量增多，多线程的优势会逐渐显现。
- 使用 `--release` 模式编译可以更准确地反映真实性能。

---

## 扩展方向

### 1. 可配置的线程池

当前实现为每个文本创建一个线程。当文本数量很大时，应考虑使用线程池限制并发线程数：

```rust
// 可使用 rayon 库或 threadpool crate
use rayon::prelude::*;
samples.par_iter().map(|(name, text)| count_stats(name, text)).collect()
```

### 2. 从真实文件读取

将硬编码的文本样本替换为从命令行参数指定的文件中读取：

```rust
let paths: Vec<String> = std::env::args().skip(1).collect();
let samples: Vec<_> = paths.iter().map(|p| {
    (p.clone(), std::fs::read_to_string(p).unwrap())
}).collect();
```

### 3. 进度条指示

使用 `indicatif` crate 为长时间运行的任务添加进度条，提升用户体验。

### 4. 更丰富的统计指标

- 句子数统计
- 平均单词长度
- 词频分布直方图
- TF-IDF 计算
- 文本相似度比较（余弦相似度）

### 5. 异步版本

使用 `tokio` 或 `async-std` 将多线程替换为异步任务，适用于 I/O 密集型场景（如读取大量文件）。

### 6. 错误恢复策略

当前设计遇到空文本仅报告错误。可扩展为：
- 重试机制
- 部分结果聚合（成功的结果仍然输出）
- 超时处理（某个线程卡住时其他线程不受影响）

---

## 常见问题

### Q: 为什么多线程反而比单线程慢？

A: 线程创建、上下文切换、锁竞争都有一定开销。当数据量很小时，这些开销可能超过并行带来的收益。这是完全正常的。增大数据量或增加文本样本数量，就能观察到多线程的加速效果。

### Q: `drop(tx)` 是必须的吗？

A: 是的。如果原始 sender `tx` 没有被 drop，接收端 `rx` 的迭代器永远不会结束（因为它不知道是否还会有新消息），导致主线程永久阻塞。

### Q: 为什么不直接用 `Rc<RefCell<HashMap>>` 替代 `Arc<Mutex<HashMap>>`？

A: `Rc` 和 `RefCell` 都不是 `Send`，无法在线程间传递。`Rc` 的引用计数不是原子的，在多线程中会导致数据竞争。`Arc` 使用原子操作保证引用计数的正确性。

### Q: 如果线程 panic 了怎么办？

A: `thread::spawn` 返回的 `JoinHandle` 的 `join()` 方法会返回 `Err`，包含 panic 的信息。本项目中，方案A 如果线程 panic，channel 的 sender 会被 drop，主线程能正常结束；方案B 中 panic 会毒化 Mutex，后续 `lock()` 会返回 `Err`。

---

## 从 Python、C、C++ 迁移时值得注意的设计差异

### 1. 线程生成时的显式所有权转移

Python 的 `threading.Thread(target=fn, args=(data,))` 中，`data` 的传递依赖运行时引用计数和 GIL 保护。C 的 `pthread_create` 通过 `void*` 传参，类型信息丢失，且需要手动管理内存生命周期。Rust 的 `thread::spawn(move || ...)` 中，`move` 关键字将闭包中变量的所有权显式移入新线程。本项目中每条样本数据（`name: String`、`text: String`）通过 `move` 移入工作线程，编译器保证主线程此后不再使用这些数据——这从根本上杜绝了"主线程释放了数据而子线程还在使用"的悬垂引用问题。C 中这是最头疼的 bug 来源之一，Python 中由 GIL 掩盖但降低了并发度。

### 2. `Arc<Mutex<T>>` 替代 GIL 保护共享可变状态

Python 的 GIL 使得多线程中访问共享字典（如 `global_dict[key] += 1`）在字节码层面看似安全（实际上仍有竞态机会）。C++ 的 `std::mutex` 和 `std::lock_guard` 是手动模式，可能忘记加锁或死锁。Rust 将互斥锁与数据绑定：`Mutex<HashMap<String, usize>>` 意味着"要访问这个 HashMap，必须先获取锁"。本项目的方案 B 中，`Arc::clone` 共享数据，`freq.lock().unwrap()` 获取守卫——守卫离开作用域时自动释放锁（RAII）。更重要的是，编译器不会让你绕过 Mutex 直接访问内部数据，这在 C++ 中仅靠代码审查纪律保证。

### 3. 通道提供类型安全的线程间消息传递

Python 的 `queue.Queue` 可以放入任意类型，读取后需要 `isinstance` 检查。C 中通过 `pipe` 或共享内存通信，需要手动约定二进制协议。Rust 的 `mpsc::channel::<Result<FileStats, String>>()` 在编译时就确定了通道中传输的类型。本项目的 worker 线程发送 `Result<FileStats, String>`，主线程接收后直接 `match`——不涉及类型转换、不涉及协议解析。发送后所有权转移给通道，worker 不能再访问 `stats`，避免了"发送后误改数据"的问题。Go 程序员会对这种类型安全的 channel 感到熟悉，但 Rust 额外保证了发送端析构后接收端的 for 循环会自动终止（因为 `drop(tx)` 触发关闭）。

### 4. 编译器在编译期防止数据竞争

这是 Rust 并发模型最根本的差异。Python 依赖 GIL 防止解释器层面的数据竞争，但逻辑竞争仍然存在，且在释放 GIL 的 C 扩展中无效。C/C++ 依赖 sanitizer（如 ThreadSanitizer）在运行时检测数据竞争，成本高且依赖充分测试覆盖。Rust 的 `Send` 和 `Sync` trait 在编译期验证：任何跨线程传递的类型必须实现 `Send`，任何跨线程共享引用的类型必须实现 `Sync`。如果你尝试将 `Rc<T>`（非 `Send`）传入线程，编译器会直接报错并精确指出问题所在。本项目中没有一处显式使用 `Send`/`Sync` 标注，但编译器在后台自动检查了每个 `thread::spawn` 和 `channel.send` 的类型安全性。

### 5. 单线程与多线程对照的性能测量

Python 的多线程受 GIL 限制，CPU 密集型任务往往不如单线程，性能分析常用 `time.perf_counter()`。C 中用 `clock_gettime` 等系统调用。Rust 的 `std::time::Instant` 提供单调时钟，配合 `--release` 编译可获得真实性能数据。本项目同时实现了单线程版本和两种多线程方案（消息传递、共享状态），可以直接对比三种模式的耗时。在 release 模式下，你可以观察到数据量超过一定阈值后多线程带来的真实加速——这与 Python 中多线程的"虚假加速"形成鲜明对比。

---

## 总结

本项目通过一个实际的并行文本统计任务，系统地演示了 Rust 并发编程的核心概念：

| 概念 | 在本项目中的应用 |
|------|-----------------|
| `thread::spawn` | 为每个文本创建独立线程 |
| `mpsc::channel` | 方案A：线程通过 channel 发回统计结果 |
| `Arc<T>` | 方案B：多线程共享全局词频 HashMap |
| `Mutex<T>` | 方案B：保护共享 HashMap 的互斥访问 |
| `move` 闭包 | 将数据所有权转移到线程 |
| `Send` / `Sync` | 编译器自动检查线程安全性 |
| 错误处理 | 通过 `Result` 在线程间传递错误 |

**核心理念**：
- **消息传递**："不要通过共享内存来通信，而要通过通信来共享内存。"（Do not communicate by sharing memory; instead, share memory by communicating.）
- **共享状态**：当确实需要共享内存时，Rust 提供了安全的抽象（`Arc` + `Mutex`），并确保在编译期消除数据竞争。

两个方案各有适用场景：消息传递更适合"任务分发-结果收集"模式，共享状态更适合"多个线程协作维护同一数据结构"的场景。理解何时使用哪种模式，是掌握 Rust 并发编程的关键。

---

*本项目为 Rust 并发编程教学练习，代码量约 260 行，覆盖了线程、通道、原子引用计数、互斥锁等核心并发原语。*
