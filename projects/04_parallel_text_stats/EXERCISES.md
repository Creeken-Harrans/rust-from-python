# 并行文本统计器 (Parallel Text Stats) — 练习指南

## 项目概述

你已阅读了 `README.md` 和 `src/main.rs`。这个项目演示了两种并发模型：**消息传递**（mpsc channel）和**共享状态**（Arc<Mutex<HashMap>>）。请在不直接复制粘贴的前提下完成以下练习。

---

## Level 1: 基础练习（理解现有代码）

### L1-1: 运行与观察

1. 运行 `cargo run` 观察单线程、消息传递、共享状态三种模式输出
2. 运行 `cargo run --release` 比较性能差异
3. 修改样本数据（增大文本量或增加样本数），重新运行观察多线程加速效果

**学习点**: debug vs release 性能差异、并行加速的阈值效应。

### L1-2: 追踪 `move` 闭包的所有权

在消息传递方案中：

```rust
thread::spawn(move || {
    worker(name.clone(), text.clone(), tx_clone);
});
```

1. 为什么需要 `move` 关键字？移除它后编译错误是什么？
2. `name.clone()` 和 `text.clone()` 为什么需要 clone？
3. `tx_clone` 的所有权转移给了谁？发送端被 drop 的时机是什么？

**学习点**: `move` 闭包、所有权跨线程转移、clone 在并发中的角色。

### L1-3: 理解 `drop(tx)` 的必要性

在消息传递方案中注释掉 `drop(tx);`：

1. 程序行为发生了什么变化？（提示：程序是否会挂起？）
2. 为什么 `for received in rx` 不会自动终止？
3. mpsc channel 如何判断"所有发送端都已关闭"？

恢复 `drop(tx)` 并验证程序正常退出。

**学习点**: mpsc channel 关闭语义、sender 引用计数。

### L1-4: 分析共享状态方案的锁竞争

在共享状态方案（`Arc<Mutex<HashMap>>`）中：

1. 临界区（critical section）是哪些代码？
2. 如果临界区内执行了耗时的字符串格式化操作，会对性能产生什么影响？
3. 如何缩小临界区范围？

在代码中用注释标注临界区边界。

**学习点**: 锁粒度、临界区优化、MutexGuard RAII。

### L1-5: 比较两种方案的输出

1. 消息传递方案和共享状态方案的最终词频统计结果是否一致？为什么（不）？
2. 哪种方案更容易保证结果的正确性？
3. 如果样本数据量极大（GB 级别），哪种方案更适合？为什么？

**学习点**: 并发正确性、方案适用场景分析。

---

## Level 2: 功能扩展（编写新代码）

### L2-1: 添加 `Rayon` 并行版本

引入 `rayon` crate，用并行迭代器替代手动线程管理：

```rust
use rayon::prelude::*;

fn run_rayon(samples: &[(String, String)]) -> HashMap<String, usize> {
    samples
        .par_iter()
        .map(|(name, text)| count_stats(name, text))
        .reduce(HashMap::new, |mut acc, stats| {
            merge_freq(&mut acc, &stats.word_freq);
            acc
        })
}
```

1. `rayon` 版本与手动线程版本在代码量上的差异
2. Rayon 如何处理线程池大小和工作分配？
3. 比较 `par_iter()` 和 `iter()` 的性能

**学习点**: rayon 并行迭代器、声明式并发、reduce 模式。

### L2-2: 实现超时控制

为每个线程添加超时机制——如果某个样本的处理时间超过 2 秒，放弃该样本：

实现要点：
- `thread::spawn` 没有原生超时支持
- 可以使用 `mpsc::channel` + `recv_timeout` 模式
- 也可以在 worker 内部用 `Instant::now()` 自检
- 超时后如何处理？发送错误？静默跳过？

**学习点**: 超时处理、`recv_timeout`、优雅降级。

### L2-3: 添加可视化进度条

使用 `indicatif` crate 为多线程处理添加进度条：

```
处理中 [████████████░░░░░░░░] 6/8 (75%) 耗时: 1.2s
```

实现要点：
- `ProgressBar::new(total as u64)` 创建进度条
- 在 worker 完成后通过 channel 通知进度更新
- 需要单独的进度收集线程还是主线程轮询？

**学习点**: 进度可视化、多线程进度通信。

### L2-4: 实现线程池版本

将"每样本一线程"改为固定大小的线程池：

- 实现 `fn run_threadpool(samples: &[(String, String)], pool_size: usize) -> WordFreq`
- 使用 `std::sync::mpsc` 配合工作队列模式
- 比较线程池与 naive 多线程在样本数=1000 时的性能差异

**学习点**: 线程池模式、工作队列、资源控制。

---

## Level 3: 设计思维（架构与扩展）

### L3-1: 错误分类聚合

当前实现在错误发生时仅打印警告。设计一个错误分类聚合系统：

```rust
struct ProcessingResult {
    successes: Vec<FileStats>,
    empty_files: Vec<String>,      // 空文件（不算错误）
    read_errors: Vec<(String, io::Error)>,
    non_utf8_files: Vec<String>,
}
```

- 如何在线程间收集这些分类信息？
- 是否需要为错误单独建立一个 channel？
- 结束时输出汇总报告：成功 N 个，跳过 M 个，错误 K 个

**学习点**: 错误分类、聚合报告、多个 channel 的使用。

### L3-2: 流式并行处理

当前实现将所有文件内容一次性读入内存。设计一个流式处理方案：

- 使用 `BufReader` 逐行读取文件
- 多个文件同时进行流式读取和统计
- 使用 channel 流式传输部分统计结果到聚合线程
- 内存占用如何变化？

**不要求完整实现**，设计架构图和数据流即可。

**学习点**: 流式处理、内存效率、部分聚合。

### L3-3: 分布式版本设计

设计将文本统计分布到多台机器上的方案：

- 使用何种通信协议？（HTTP? gRPC? TCP?）
- 如何分割任务？（按文件列表、按文件大小、按行号范围？）
- 如何处理节点故障？（重试? 重新分配?）
- 结果如何合并？（在协调节点上汇总？分层次聚合？）

给出架构图和关键数据结构定义。

**学习点**: 分布式系统设计、任务调度、容错。

---

## 检查清单

完成上述练习后，你应该能够：

- [ ] 理解 `move` 闭包在 `thread::spawn` 中的必要性
- [ ] 掌握 mpsc channel 的发送/接收/关闭语义
- [ ] 使用 `Arc<Mutex<T>>` 实现共享可变状态
- [ ] 区分消息传递和共享状态两种并发模型
- [ ] 理解 `Send` 和 `Sync` trait 的编译期保证
- [ ] 使用 `rayon` 简化并行处理
- [ ] 设计线程池和工作队列
- [ ] 评估并发方案在不同场景下的适用性
