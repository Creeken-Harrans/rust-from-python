# 练习：第 22 章 —— 异步编程入门

---

## 难度说明

- **Level 1** 基础巩固 —— 修改/扩充本章已有代码
- **Level 2** 独立应用 —— 需要组合多个概念
- **Level 3** 综合挑战 —— 需要设计架构、处理边界情况
- **思考题** 无需写代码，加深概念理解

---

## Level 1：基础巩固

### L1-1：添加更多模拟工作类型

**任务：** 在 `main.rs` 中添加一个 `async fn simulate_io_work(name: &str, bytes: u64) -> String` 函数。

**要求：**
- 函数接收 `bytes` 参数，用 `bytes / 10` 作为等待毫秒数（模拟 I/O 吞吐量）
- 在等待前后分别打印消息，格式如 `[IO-{name}] 开始读取 {bytes} 字节...`
- 返回格式为 `"IO-{name} 完成：读取了 {bytes} 字节"`
- 在 `main` 函数中调用它，分别用 500、1000、2000 字节测试

**预期效果：**
```
[IO-文件A] 开始读取 500 字节...
[IO-文件A] 读取完成！
IO-文件A 完成：读取了 500 字节
```

**提示：** 参考 `simulate_work` 函数的实现模式。

---

### L1-2：使用 tokio::join! 并发执行新增的任务

**任务：** 使用 `tokio::join!` 同时运行 3 个 `simulate_io_work` 实例。

**要求：**
- 创建 3 个 Future（文件A 500字节、文件B 1000字节、文件C 2000字节）
- 使用 `tokio::join!` 并发等待它们全部完成
- 使用 `Instant` 测量并发总耗时
- 对比：如果顺序执行这 3 个任务，理论耗时是多少？并发实际耗时是多少？

**预期效果：** 并发耗时远小于顺序耗时之和（约为最慢任务的时间）。

**思考：** 为什么并发的总时间约等于最慢任务的时间，而不是所有任务时间之和？

---

### L1-3：对比 spawn 和 join!

**任务：** 编写一个函数，分别用 `tokio::spawn` 和 `tokio::join!` 执行相同的 3 个任务，比较结果。

**要求：**
- 创建 3 个任务，每个耗时分别为 80ms、120ms、100ms
- 先用 `tokio::join!` 执行并测量时间
- 再用 `tokio::spawn` 执行并测量时间
- 打印两种方式的耗时对比
- 解释两种方式在什么场景下各自更合适

**思考：** 在单线程运行时（`flavor = "current_thread"`）中，两者行为有何不同？

---

## Level 2：独立应用

### L2-1：模拟并发的 URL 健康检查器

**任务：** 编写一个健康检查器，并发检查多个 URL 的响应时间。

**要求：**
- 创建 `async fn check_url(url: &str, expected_delay_ms: u64) -> Result<(String, u64), String>`
  - `expected_delay_ms` 模拟该 URL 的正常响应时间
  - 函数"检查"URL（用 sleep 模拟网络请求），耗时 = `expected_delay_ms`
  - 返回 `(url, actual_delay_ms)` 表示检查成功
  - 如果 `expected_delay_ms > 500`，视为"超时"，返回 Err
- 创建 `async fn run_health_checks(urls: Vec<(&str, u64)>) -> Vec<Result<(String, u64), String>>`
  - 接收一个 URL 和预期延迟的列表
  - 使用 `tokio::spawn` 并发检查所有 URL
  - 用 `JoinSet`（提示：`tokio::task::JoinSet`）或手动收集 `JoinHandle`
  - 按完成顺序输出结果（先完成的先报告）
- 在 `main` 中测试：至少 5 个 URL，其中 2 个"超时"

**预期效果：**
```
开始检查 5 个服务...
[OK] https://api.example.com - 延迟: 120ms
[OK] https://cdn.example.com - 延迟: 80ms
[TIMEOUT] https://slow-service.example.com - 延迟 800ms 超过阈值
[OK] https://db.example.com - 延迟: 300ms
[TIMEOUT] https://legacy.example.com - 延迟 600ms 超过阈值
健康检查完成，总耗时: 310ms
```

**提示：** `tokio::task::JoinSet` 可以方便地管理多个 spawn 出来的任务。如果没有，可以手动循环 `JoinHandle` 的 `.await`。

---

### L2-2：带超时的数据获取

**任务：** 编写一个数据获取函数，如果某个数据源响应太慢，就放弃它继续处理其他数据。

**要求：**
- 创建 5 个模拟数据源，每个有不同的"延迟"（50ms 到 1000ms）
- 使用 `tokio::spawn` 并发发起所有请求
- 对每个请求使用 `tokio::time::timeout(Duration::from_millis(300), handle)` 设置 300ms 超时
- 收集成功的结果和超时的结果，分别打印
- 计算总耗时和"兜底数据"的使用率（超时的用兜底数据替代）

**预期效果：**
```
数据源 #1 (50ms):  ✓ 获取成功
数据源 #2 (200ms): ✓ 获取成功
数据源 #3 (150ms): ✓ 获取成功
数据源 #4 (800ms): ✗ 超时，使用兜底数据
数据源 #5 (1000ms): ✗ 超时，使用兜底数据
成功: 3/5, 超时: 2/5, 兜底数据使用率: 40%
总耗时: 308ms
```

**核心 API：**
```rust
match tokio::time::timeout(Duration::from_millis(300), handle).await {
    Ok(Ok(data)) => { /* 成功 */ }
    Ok(Err(_)) => { /* 任务内部错误 */ }
    Err(_) => { /* 超时 */ }
}
```

---

## Level 3：综合挑战

### L3-1：迷你异步任务调度器

**任务：** 设计并实现一个"迷你爬虫"，并发抓取一个由多个级别组成的树状数据。

**背景：**
你有一个两级数据依赖：先获取用户列表（Level 1），然后根据每个用户 ID 获取该用户的订单列表（Level 2）。

**要求：**

1. **Level 1 —— 获取用户列表：**
   - `async fn fetch_user_list() -> Vec<u32>`
   - 模拟耗时 200ms，返回 5 个用户 ID（1, 2, 3, 4, 5）

2. **Level 2 —— 获取每个用户的订单：**
   - `async fn fetch_orders_for_user(user_id: u32) -> Result<Vec<String>, String>`
   - 不同用户有不同耗时：用户 1 (100ms)、2 (300ms)、3 (50ms)、4 (400ms)、5 (150ms)
   - 用户 4 有 30% 概率返回错误（模拟间歇性故障）
   - 返回该用户的订单描述列表

3. **编排逻辑：**
   - `async fn crawl_all_users() -> HashMap<u32, Result<Vec<String>, String>>`
   - 先获取用户列表
   - **并发地**为每个用户获取订单（用 `tokio::spawn` 或 `JoinSet`）
   - 收集所有结果到 `HashMap<u32, Result<Vec<String>, String>>`
   - 记录每个用户请求的耗时
   - 打印汇总统计：成功多少个、失败多少个、总耗时

4. **重试机制：**
   - 如果用户 4 的请求失败，**自动重试**（最多 2 次重试，每次重试前等待 50ms）
   - 使用 loop 实现重试逻辑

5. **统计输出：**
   ```
   获取用户列表...完成 (200ms)
   并发获取 5 个用户的订单...
     用户 1: ✓ (100ms, 3 个订单)
     用户 3: ✓ (50ms, 1 个订单)
     用户 5: ✓ (150ms, 2 个订单)
     用户 2: ✓ (300ms, 5 个订单)
     用户 4: ✗ 第1次失败，重试...
     用户 4: ✓ 重试成功 (350ms, 4 个订单)
   汇总: 成功 5/5
   总耗时: 550ms
   ```

**提示：**
- Level 1 必须先完成才能开始 Level 2
- Level 2 的 5 个请求应该**并发**执行
- 使用 `tokio::time::Instant` 记录每个用户请求的耗时
- 错误用 `rand::random` 或简单的计数方式模拟

**进阶（选做）：**
- 限制最大并发数（如同时最多发起 3 个 Level 2 请求）
- 使用 `tokio::sync::Semaphore` 实现

---

## 思考题

### Q1：Future 的惰性设计

Rust 的 `Future` 在被 `.await` 或 `poll` 之前不会执行任何代码。
这与 JavaScript 的 `Promise`（创建即开始执行）形成鲜明对比。

**请思考并回答：**
- 惰性设计带来了哪些好处？给出至少 2 个。
- 惰性设计带来了哪些不便？给出至少 1 个。
- 如果 Rust 采用"创建即执行"的模型（如 JavaScript Promise），会对 `tokio::join!` 的实现造成什么影响？

---

### Q2：异步与并行的关系

有人说："异步就是单线程上的并发"；也有人说："异步可以充分利用多核 CPU 的并行能力"。

**请回答：**
- 这两种说法各在什么场景下是正确的？
- Tokio 默认的多线程运行时如何实现了"异步 + 并行"的结合？
- 在什么情况下你希望使用单线程运行时而非多线程运行时？

---

### Q3：取消安全

考虑以下代码：

```rust
async fn transfer_money(from: u32, to: u32, amount: u64) {
    db.debit(from, amount).await;   // 步骤1：扣款
    db.credit(to, amount).await;    // 步骤2：入账
}
```

如果 `transfer_money` 在步骤 1 后、步骤 2 前被取消（例如 `JoinHandle` 被 drop），
会发生什么？这是一个"取消安全"的反面例子。

**请回答：**
- 为什么这种情况很危险？
- 你会如何修改代码来保证操作的原子性？

---

## 推荐命令

```bash
# 运行本章示例代码
cd chapters/22_async_await_tokio_intro
cargo run

# 运行并查看 release 模式的性能（异步并发优势更明显）
cargo run --release

# 编译检查（不运行）
cargo check

# 查看文档
cargo doc --open

# 运行 clippy 静态检查
cargo clippy

# 格式化代码
cargo fmt

# 查看 Tokio 运行时内部日志
RUST_LOG=tokio=trace cargo run 2>&1 | head -50

# 运行测试（如果有）
cargo test
```

---

## 参考答案提示

### L1-2 提示
并发耗时约为 2000/10 = 200ms（最慢任务），而不是 500/10 + 1000/10 + 2000/10 = 350ms。
因为 3 个 sleep 在运行时中被同时计时，最慢的那个决定了总时间。

### L1-3 提示
在单线程运行时中，`tokio::join!` 的多个 Future 会被同一个线程轮流 poll；
`tokio::spawn` 的任务也是单线程调度，但由于 `spawn` 返回 `JoinHandle` 需要单独 `.await`，
顺序 `.await` 多个 handle 可能会导致"第一个未完成就不 poll 第二个"的情况。

### L2-1 提示
`tokio::task::JoinSet` 的 `join_next()` 方法返回下一个完成的任务，天然支持"先完成先处理"。

### L3-1 提示
- 重试逻辑的核心结构：
  ```rust
  let mut attempts = 0;
  loop {
      match fetch_orders_for_user(user_id).await {
          Ok(orders) => break Ok(orders),
          Err(e) if attempts < 2 => {
              attempts += 1;
              tokio::time::sleep(Duration::from_millis(50)).await;
              continue;
          }
          Err(e) => break Err(e),
      }
  }
  ```
- Semaphore 限制并发：
  ```rust
  let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
  let permit = semaphore.clone().acquire_owned().await.unwrap();
  // ... do work ...
  drop(permit); // 释放信号量
  ```

---

**练习的目的是加深对 Rust 异步模型的理解。如果遇到困难，回顾 README 中的概念讲解，尤其是 Future 的惰性本质和运行时的工作方式。**

---

## 迁移思维练习

> 以下问题帮助你思考 Python asyncio 的异步模型如何重新理解 Rust 的 async/await。

### 问题 1：Python asyncio 和 Rust async/await 的关键区别在哪里？

Python 的 async/await 构建在一个内置的事件循环（event loop）之上——`asyncio.run()` 启动一个全局的事件循环，所有协程都在其中调度。Rust 的 async/await 则把运行时的选择留给了库（Tokio、async-std、smol 等），语言本身只定义了 `Future` trait 和 `.await` 语法。这种"语言不绑定运行时"的设计有什么优势？它又带来了什么代价（比如你需要手动选择 Runtime）？另外，Python 的协程在创建后就开始执行，Rust 的 Future 在被 `.await` 之前是惰性的——这种差异在实际编码中会怎样影响你的代码结构？

**提示**：Rust 把 async 视为一种"零成本抽象"——编译器将 async fn 编译为状态机，运行时只负责 poll 和 wake，不参与代码生成。

### 问题 2：什么任务适合 async，什么任务适合多线程？

你已经学过了第 21 章的多线程和本章的 async——两者都可以实现并发。对于一个需要同时处理 1000+ 个网络连接的服务器，为什么 async 模型比"每个连接一个线程"更高效？反过来，对于一个 CPU 密集型任务（如视频编码或科学计算），为什么应该使用 `spawn_blocking` 或将任务交给线程池，而不是在 async 上下文中直接计算？如果 async 任务中不小心执行了一个长时间的同步操作，会对整个运行时产生什么影响？

**提示**：async 适用于 I/O 密集场景——大部分时间在"等待"；CPU 密集任务会阻塞运行时线程，阻碍其他 async 任务被 poll，应该通过 `spawn_blocking` 分离到线程池中执行。
