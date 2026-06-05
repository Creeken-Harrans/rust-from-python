# 练习答案 — 异步编程与 Tokio

## 迁移思维练习答案

### 1. Python asyncio 和 Rust async/await 的关键区别在哪里？

Python 的 async 函数调用后立即开始执行——调用 `async def foo()` 返回一个 coroutine 对象，await 它时由事件循环调度执行。Rust 的 async fn 返回 Future，是惰性的——仅调用 async fn 不做任何事，必须 .await 或被交给 runtime（如 tokio::spawn）才会执行。Rust 标准库不提供异步运行时（需引入 Tokio、async-std 等第三方库），而 Python 的 asyncio 是标准库一部分。两者都是协作式调度，但 Rust 的 Future 经过编译器的状态机转换，避免了 Python coroutine 的堆分配开销。

**Async 不等于多线程**——这是最核心的概念区分。async 是并发模型，多线程是并行模型。异步可以在单线程上实现并发（多个任务交替推进），多线程则利用多核实现真正的并行。

**Tokio 不是 Rust 标准库**——Rust 语言只定义了 `Future` trait 和 `.await` 语法，运行时（runtime）的选择留给了第三方库。Tokio 是社区最流行的异步运行时，但还有 async-std、smol 等替代品。这意味着你需要手动添加 tokio 依赖并在代码中显式启动运行时（`#[tokio::main]`）。

### 2. 什么任务适合 async，什么任务适合多线程？

I/O 密集型任务（网络请求、文件读写、数据库查询、等待外部服务）适合 async——用少量操作系统线程管理海量并发连接，在等待 I/O 时让出 CPU 给其他任务。CPU 密集型任务（数学计算、图像/视频处理、加密解密、大 JSON 解析）适合多线程或 rayon 等并行库——需要真正使用多个 CPU 核心并行计算。

**CPU密集任务不能简单塞入异步任务**——如果在 async 上下文中直接执行耗时计算（如大循环、复杂数学运算），会阻塞运行时的线程，导致该线程上的所有其他异步任务无法被 poll。正确的做法是使用 `tokio::task::spawn_blocking` 将 CPU 密集任务交给专用线程池。

混合场景的常见方案：async 作为主体框架，CPU 密集部分通过 `tokio::task::spawn_blocking` 交给专用线程池，避免阻塞异步事件循环。

### 3. 从 Python asyncio 迁移到 Rust async 需要注意什么陷阱？

第一个陷阱是"忘记 .await"——Rust 的 Future 是惰性的，如果不 .await 或 spawn，什么都不会发生，编译器通常会给出 `unused` 警告但不会报错。第二个是"阻塞异步 runtime"——在 async 上下文中调用 std::thread::sleep 或执行耗时循环会阻塞整个线程的事件循环，应该用 tokio::time::sleep 和 spawn_blocking。第三个是生命周期问题——async 函数返回的 Future 必须拥有其使用的数据或确保引用活得足够久，这在与借用交互时可能让人困惑。

---

## Level 1 练习

### L1-1：添加更多模拟工作类型

**结论**：通过参数化 `simulate_io_work` 的等待时间，可以用同一个函数模拟不同吞吐量的 I/O 操作。

**思路**：仿照 `simulate_work` 的模式，增加 `bytes` 参数来决定等待时间（模拟 I/O 吞吐量），在等待前后打印状态信息。

**参考实现**：

```rust
async fn simulate_io_work(name: &str, bytes: u64) -> String {
    let delay_ms = bytes / 10;
    println!("    [IO-{}] 开始读取 {} 字节, 预计 {} ms...", name, bytes, delay_ms);
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    println!("    [IO-{}] 读取完成！", name);
    format!("IO-{} 完成：读取了 {} 字节", name, bytes)
}
```

**在 main 中调用**：

```rust
// 顺序执行（演示用，后面会改为并发）
let r1 = simulate_io_work("文件A", 500).await;
let r2 = simulate_io_work("文件B", 1000).await;
let r3 = simulate_io_work("文件C", 2000).await;
println!("{}", r1);
println!("{}", r2);
println!("{}", r3);
```

**为什么这样设计**：
- `bytes / 10` 作为等待毫秒数：500字节=50ms, 1000字节=100ms, 2000字节=200ms
- 前后打印状态信息便于观察执行流程
- 返回格式化字符串，符合 Rust 的表达式导向风格

**常见错误**：
1. 忘记 `.await` → Future 没有执行
2. 参数类型用 `u32` 在除法时可能溢出
3. 将 `bytes` 和 `duration_ms` 混淆

**验证方式**：运行程序，能看到三条带有不同字节数和时间的输出。

---

### L1-2：使用 tokio::join! 并发执行新增的任务

**结论**：`tokio::join!` 让多个 Future 在当前任务中**并发推进**，总耗时约等于最慢任务的时间，而非所有任务时间之和。这体现了异步的核心价值——在"等待"期间切换执行其他任务。

**思路**：创建 3 个 Future → 用 `tokio::join!` 并发等待 → 用 `Instant` 测量耗时 → 对比顺序执行理论耗时。

**参考实现**：

```rust
async fn demo_join_concurrent() {
    println!("\n--- tokio::join! 并发 I/O 演示 ---");
    let start = Instant::now();

    let fut_a = simulate_io_work("文件A", 500);   // 50ms
    let fut_b = simulate_io_work("文件B", 1000);  // 100ms
    let fut_c = simulate_io_work("文件C", 2000);  // 200ms

    // 三个 Future 并发推进
    let (r1, r2, r3) = tokio::join!(fut_a, fut_b, fut_c);

    let elapsed = start.elapsed();
    println!("结果:");
    println!("  {}", r1);
    println!("  {}", r2);
    println!("  {}", r3);
    println!("并发总耗时: {:?}", elapsed);
    println!("顺序执行理论耗时: {:?} (50+100+200=350ms)",
        Duration::from_millis(350));
    println!(
        "节省时间: {:?}",
        Duration::from_millis(350).saturating_sub(elapsed)
    );
}
```

**为什么这样设计**：
- `tokio::join!` 不是并行执行，而是在同一个任务中轮询多个 Future——当 fut_a 在 sleep 时，运行时会 poll fut_b、fut_c
- **Future/Runtime/Executor 模型**：Future 是惰性的状态机，Runtime（Tokio）负责 poll Future，Executor 决定何时 poll 哪个 Future。当 Future 返回 `Poll::Pending` 时，Runtime 把它放入等待队列；当等待条件满足时（如 timer 到期），Runtime 重新 poll 它
- 并发耗时约 200ms（最慢任务），而非 350ms（顺序之和）

**常见错误**：
1. 认为 `tokio::join!` 使用了多线程 → 它只是并发轮询，不涉及多线程
2. 忘记创建变量持有 Future → `simulate_io_work(...)` 的返回值如果不绑定到变量，它就是一个临时 Future，在 `tokio::join!` 中可能有不预期的行为

**验证方式**：并发总耗时约 200ms（约为最慢任务的时间），远小于 350ms。

---

### L1-3：对比 spawn 和 join!

**结论**：`tokio::join!` 和 `tokio::spawn` 都能实现并发，但机制不同。`join!` 在当前任务中并发 poll 多个 Future；`spawn` 将 Future 提交为独立任务，在多线程运行时中可能在不同线程上并行执行。

**思路**：分别用两种方式执行相同的 3 个任务，用 Instant 计时对比。

**参考实现**：

```rust
async fn compare_join_and_spawn() {
    println!("\n--- join! vs spawn 对比 ---");

    // 方式一: tokio::join!
    println!("方式一: tokio::join!");
    let start = Instant::now();
    let (r1, r2, r3) = tokio::join!(
        simulate_work("join-A", 80),
        simulate_work("join-B", 120),
        simulate_work("join-C", 100),
    );
    let join_elapsed = start.elapsed();
    println!("  join! 耗时: {:?}", join_elapsed);

    // 方式二: tokio::spawn
    println!("\n方式二: tokio::spawn");
    let start = Instant::now();
    let h1 = tokio::spawn(simulate_work("spawn-A", 80));
    let h2 = tokio::spawn(simulate_work("spawn-B", 120));
    let h3 = tokio::spawn(simulate_work("spawn-C", 100));
    let (s1, s2, s3) = (h1.await.unwrap(), h2.await.unwrap(), h3.await.unwrap());
    let spawn_elapsed = start.elapsed();
    println!("  spawn 耗时: {:?}", spawn_elapsed);

    println!("\n对比:");
    println!("  join!  总耗时: {:?}", join_elapsed);
    println!("  spawn 总耗时: {:?}", spawn_elapsed);
    println!("\n何时使用 join!:");
    println!("  - 需要同时等待固定数量的 Future");
    println!("  - 不需要独立的任务生命周期管理");
    println!("  - 在当前任务上下文中并发");
    println!("\n何时使用 spawn:");
    println!("  - 需要在多线程运行时中并行执行");
    println!("  - 需要独立的任务生命周期（可以取消、超时）");
    println!("  - 任务数量不确定（在循环中动态创建）");
}
```

**为什么这样设计**：
- `join!` 在当前任务中并发 poll——所有 Future 共享同一个任务的执行上下文。适合"我知道要等哪些 Future"的场景
- `spawn` 提交独立任务到运行时——每个任务有自己的执行上下文，可以被调度到不同线程。适合"动态创建任务"的场景
- **在单线程运行时（`current_thread`）中**：`join!` 的 Future 在当前线程交替执行；`spawn` 的任务也在同一线程调度，但由于 `spawn` 返回 `JoinHandle` 需要单独 `.await`，如果顺序 await 多个 handle 而没有使用 `join!`，可能出现"第一个未完成就不 poll 第二个"的情况

**常见错误**：
1. 单线程运行时中，顺序 await JoinHandle（不是用 join!）→ 并发度降低
2. 混淆 `join!` 和 `spawn` 的执行模型，以为 `join!` 也利用多线程

**验证方式**：两种方式耗时都接近 120ms（最慢任务）。

---

## Level 2 练习

### L2-1：模拟并发的 URL 健康检查器

**结论**：使用 `tokio::spawn` + `JoinSet`（或手动收集 JoinHandle）可以构建一个简单的健康检查器。JoinSet 的 `join_next()` 按完成顺序返回结果，非常适合"哪个先完成就先报告"的场景。

**思路**：为每个 URL 创建 spawn 任务 → 用 JoinSet 管理 → 按完成顺序输出结果 → 统计成功/超时。

**参考实现**：

```rust
use tokio::task::JoinSet;

async fn check_url(url: &str, expected_delay_ms: u64) -> Result<(String, u64), String> {
    // 实际项目中这里是真实的 HTTP 请求
    tokio::time::sleep(Duration::from_millis(expected_delay_ms)).await;

    if expected_delay_ms > 500 {
        Err(format!("{} - 延迟 {}ms 超过阈值", url, expected_delay_ms))
    } else {
        Ok((url.to_string(), expected_delay_ms))
    }
}

async fn run_health_checks(urls: Vec<(&str, u64)>) -> Vec<Result<(String, u64), String>> {
    println!("开始检查 {} 个服务...", urls.len());
    let start = Instant::now();
    let mut results = Vec::new();
    let mut set = JoinSet::new();

    for (url, delay) in urls {
        let url = url.to_string();
        set.spawn(async move { check_url(&url, delay).await });
    }

    // 按完成顺序处理
    while let Some(result) = set.join_next().await {
        match result.unwrap() {
            Ok((url, delay)) => {
                println!("[OK] {} - 延迟: {}ms", url, delay);
                results.push(Ok((url, delay)));
            }
            Err(msg) => {
                println!("[TIMEOUT] {}", msg);
                results.push(Err(msg));
            }
        }
    }

    let elapsed = start.elapsed();
    println!("健康检查完成，总耗时: {:?}", elapsed);
    results
}

// 在 main 中调用:
// let urls = vec![
//     ("https://api.example.com", 120),
//     ("https://cdn.example.com", 80),
//     ("https://slow-service.example.com", 800),
//     ("https://db.example.com", 300),
//     ("https://legacy.example.com", 600),
// ];
// let results = run_health_checks(urls).await;
```

**为什么这样设计**：
- `JoinSet` 是 Tokio 提供的任务集合管理器——`join_next()` 返回的是**下一个完成**的任务（而非插入顺序）。这天然支持"先到先报告"
- 每个 URL 的检查逻辑封装在独立的 async fn 中，职责清晰
- 通过 `expected_delay_ms > 500` 模拟超时判断

**常见错误**：
1. 用 Vec<JoinHandle> 并顺序 await → 失去了"先完成先报告"的能力
2. 忘记将 url 转为 owned String → 生命周期问题

**验证方式**：输出中低延迟的 URL 先报告，超时的后报告，总耗时约 310ms（最慢任务 800ms 但其中两个超时了）。

---

### L2-2：带超时的数据获取

**结论**：`tokio::time::timeout` 可以在异步上下文中为任何 Future 设置超时限制。超时的任务被取消，成功的任务正常收集。这是构建健壮的分布式系统的基础模式。

**思路**：并发发起所有请求 → 每个请求包一个 timeout → 超时的用兜底数据 → 统计成功率。

**参考实现**：

```rust
async fn fetch_with_timeout() {
    println!("\n--- 带超时的数据获取 ---");
    let start = Instant::now();

    // 模拟数据源的延迟
    let delays = vec![50, 200, 150, 800, 1000];
    let mut handles = Vec::new();

    for (i, delay_ms) in delays.iter().enumerate() {
        let i = i + 1;
        let delay = *delay_ms;
        let handle = tokio::spawn(async move {
            // 模拟数据获取
            tokio::time::sleep(Duration::from_millis(delay)).await;
            (i, format!("数据源 #{} 的数据 (耗时{}ms)", i, delay))
        });
        handles.push((i, delay, handle));
    }

    let mut success = 0;
    let mut timeout_count = 0;
    let timeout_duration = Duration::from_millis(300);

    for (i, delay, handle) in handles {
        match tokio::time::timeout(timeout_duration, handle).await {
            Ok(Ok((id, data))) => {
                println!("数据源 #{} ({}ms): ✓ 获取成功", id, delay);
                success += 1;
            }
            Ok(Err(e)) => {
                println!("数据源 #{} ({}ms): ✗ 任务错误", i, delay);
            }
            Err(_elapsed) => {
                println!("数据源 #{} ({}ms): ✗ 超时，使用兜底数据", i, delay);
                timeout_count += 1;
            }
        }
    }

    let elapsed = start.elapsed();
    let total = success + timeout_count;
    println!("成功: {}/{}, 超时: {}/{}, 兜底数据使用率: {}%",
        success, total, timeout_count, total,
        (timeout_count as f64 / total as f64 * 100.0) as u32
    );
    println!("总耗时: {:?}", elapsed);
}
```

**为什么这样设计**：
- `tokio::time::timeout(duration, future).await` 返回：
  - `Ok(future_output)` —— Future 在超时前完成
  - `Err(Elapsed)` —— 超时，Future 被取消
- 三层 Result 解包：`timeout` 的 Result → `JoinHandle` 的 Result → 任务自身的 Result
- 超时的任务被自动取消（drop），不会继续占用资源

**常见错误**：
1. 设置过短的超时 → 正常请求也被取消
2. 忘记处理 `JoinError`（`handle.await.unwrap()` panic 时）→ 丢失错误信息
3. 超时后的 Future 仍在运行 → Tokio 的 timeout 会取消 Future，但如果有 side effect 需要额外处理

**验证方式**：3/5 成功，2/5 超时，兜底数据使用率 40%。

---

## Level 3 练习

### L3-1：迷你异步任务调度器（两级数据爬取）

**结论**：通过组合 `tokio::spawn` + `JoinSet` + 重试逻辑，可以构建一个完整的异步工作流：先获取用户列表（Level 1），再并发为每个用户获取订单（Level 2），失败自动重试。

**思路**：分两级执行——Level 1 必须完成后才能开始 Level 2。Level 2 的多个请求并发执行。使用 loop 实现重试逻辑。

**参考实现**：

```rust
use std::collections::HashMap;
use tokio::task::JoinSet;

/// Level 1: 获取用户列表
async fn fetch_user_list() -> Vec<u32> {
    println!("获取用户列表...");
    tokio::time::sleep(Duration::from_millis(200)).await;
    println!("获取用户列表...完成 (200ms)");
    vec![1, 2, 3, 4, 5]
}

/// Level 2: 获取单个用户的订单（带失败概率）
async fn fetch_orders_for_user(user_id: u32) -> Result<Vec<String>, String> {
    let delay = match user_id {
        1 => 100,
        2 => 300,
        3 => 50,
        4 => 400,
        5 => 150,
        _ => 80,
    };

    tokio::time::sleep(Duration::from_millis(delay)).await;

    // 用户 4 有 30% 概率失败（用简单计数模拟）
    if user_id == 4 {
        // 模拟：使用一个静态计数器
        static mut COUNTER: u32 = 0;
        // 注意：这里用 unsafe 仅用于模拟，实际应使用 AtomicU32
        // 简化起见，随机判断
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        if nanos % 10 < 3 {
            return Err(format!("用户 {} 的订单获取失败（间歇性错误）", user_id));
        }
    }

    let count = delay / 30 + 1;
    let orders: Vec<String> = (1..=count)
        .map(|i| format!("订单#{}-{}", user_id, i))
        .collect();
    Ok(orders)
}

/// 编排：两级爬取 + 重试
async fn crawl_all_users() -> HashMap<u32, Result<Vec<String>, String>> {
    let start = Instant::now();

    // Level 1
    let users = fetch_user_list().await;

    // Level 2: 并发获取订单
    println!("并发获取 {} 个用户的订单...", users.len());
    let mut set = JoinSet::new();
    let mut results = HashMap::new();

    // 使用 Semaphore 限制最大并发数为 3（选做）
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(3));

    for &user_id in &users {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        set.spawn(async move {
            let start_time = tokio::time::Instant::now();

            // 重试逻辑：最多 2 次重试
            let mut attempts = 0;
            let order_result = loop {
                match fetch_orders_for_user(user_id).await {
                    Ok(orders) => break Ok(orders),
                    Err(e) if attempts < 2 => {
                        attempts += 1;
                        println!("  用户 {}: ✗ 第{}次失败（{}），重试...", user_id, attempts, e);
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    Err(e) => break Err(e),
                }
            };

            let elapsed = start_time.elapsed();
            drop(permit); // 释放信号量
            (user_id, order_result, elapsed)
        });
    }

    // 收集结果
    while let Some(result) = set.join_next().await {
        let (user_id, order_result, elapsed) = result.unwrap();
        match &order_result {
            Ok(orders) => {
                println!("  用户 {}: ✓ ({}ms, {} 个订单)", user_id,
                    elapsed.as_millis(), orders.len());
            }
            Err(e) => {
                println!("  用户 {}: ✗ {} ({}ms)", user_id, e, elapsed.as_millis());
            }
        }
        results.insert(user_id, order_result);
    }

    let total_elapsed = start.elapsed();
    let success_count = results.values().filter(|r| r.is_ok()).count();
    println!("\n汇总: 成功 {}/{}", success_count, users.len());
    println!("总耗时: {}ms", total_elapsed.as_millis());

    results
}
```

**为什么这样设计**：
- **两阶段编排**：Level 1 必须先完成才能开始 Level 2——这模拟了真实的依赖关系（需要先知道有哪些用户）
- **重试逻辑**：`loop { match ... }` 模式，成功时 break Ok，失败时判断重试次数。重试前 sleep 50ms 避免立即重试相同的错误
- **Semaphore 限制并发**：`tokio::sync::Semaphore::new(3)` 同时最多 3 个 Level 2 请求——避免对下游服务造成过大压力
- **每个请求独立计时**：用 `tokio::time::Instant` 记录每个用户请求的耗时

**常见错误**：
1. 忘记 acquire semaphore → 所有请求同时发出（没有并发限制）
2. 重试逻辑中不 sleep → 可能导致紧循环重试
3. JoinSet 中忘记 drop permit → Semaphore 资源泄漏

**验证方式**：输出显示 5 个用户的结果，用户 4 可能有一次失败后重试成功的记录。

---

## 思考题

### Q1：Future 的惰性设计

**好处**：
1. **零开销的组合**：创建 `Future` 只是创建了一个状态机（栈分配），不发生任何 I/O。`tokio::join!(f1, f2)` 可以安全地同时构建多个 Future，而不担心副作用提前发生
2. **显式的取消语义**：如果不 `.await` 一个 Future，它就不会执行——取消一个未开始的异步操作不需要任何清理
3. **与 Rust 的所有权模型完美契合**：Future 是值，可以像普通值一样被移动、借用、返回，不需要 JavaScript Promise 那样的"始终悬着"的状态

**不便**：
1. 容易忘记 `.await`——创建 Future 但不用它，编译器可能只给一个 warning（`unused_must_use` 不是默认 deny 的）

**如果 Rust 采用"创建即执行"模型**：`tokio::join!` 将无法实现——因为当我们创建 `fut1` 时它已经开始执行了，等到 `join!` 那一刻，`fut2` 还没创建。`join!` 要求所有 Future 在同一时间点被 poll，这就需要惰性设计。

### Q2：异步与并行的关系

**"异步就是单线程上的并发"**——这种说法在单线程运行时（`current_thread`）中是正确的。一个线程通过协作式调度交替执行多个 Future，在任何时刻只有一个 Future 在执行。

**"异步可以充分利用多核 CPU"**——这种说法在多线程运行时（`multi_thread`）中是正确的。Tokio 默认使用多线程工作窃取调度器，多个 Future 可以被调度到不同线程上并行执行。

**Tokio 多线程运行时的"异步+并行"**：Tokio 默认启动 N 个工作线程（N=CPU 核心数），每个线程有自己的任务队列。当一个线程空闲时，它会从其他线程"窃取"任务。这是"窃取式调度"——结合了 async 的高效 I/O 处理和多线程的并行计算能力。

**使用单线程运行时的场景**：
- 嵌入式系统（内存受限）
- 不需要并行的简单服务
- 需要确定性执行顺序的场景（单线程保证了执行顺序）
- 测试和调试

### Q3：取消安全

**为什么危险**：`transfer_money` 在步骤 1（扣款）和步骤 2（入账）之间如果被取消，钱被扣了但没有入账——钱**凭空消失**了。这是取消不安全的典型例子。

**如何保证原子性**：
1. 使用数据库事务：将两个操作包裹在事务中，要么全部完成，要么全部回滚
2. 使用两阶段提交模式：先预留资金，确认后再正式转账
3. 使用幂等操作 + 补偿事务：每个操作可以安全重试，失败时有对应的回滚操作
4. 避免使用 tokio::spawn 的 JoinHandle 被 drop 时的隐式取消——使用 `JoinSet` 或显式的 CancellationToken

```rust
async fn transfer_money_safe(from: u32, to: u32, amount: u64) -> Result<(), DbError> {
    db.begin_transaction().await?;
    db.debit(from, amount).await?;
    db.credit(to, amount).await?;
    db.commit_transaction().await?;
    Ok(())
    // 如果任何一步失败，整个事务回滚——包括之前的操作
}
```

---

*完成所有练习后，你应该已经掌握了 Rust 异步编程的核心概念：Future 的惰性本质、tokio::join!/spawn 的区别、timeout 模式、重试逻辑、取消安全。记住：Async 不等于多线程；Tokio 不是 Rust 标准库；CPU密集任务不能简单塞入异步任务。*
