#![allow(rustdoc::invalid_html_tags)]
// ============================================================================
// 第 22 章：异步编程 —— Future、async/await 与 Tokio 入门
// ============================================================================
//
// 核心概念：
//   Future  trait  —— 一个惰性（lazy）的状态机，代表未来会完成的计算
//   async fn       —— 编译器将函数体转换为实现了 Future trait 的状态机
//   .await         —— 驱动 Future 执行直到完成（在此处等待结果）
//   Runtime 运行时 —— 负责轮询（poll）Future 并推进其状态的执行器
//
//   重要：async fn 返回的 Future 是惰性的。
//   仅仅调用 async fn 并不会执行任何代码——必须 .await 或交给 runtime 才会执行。
//
// Tokio 是 Rust 生态最流行的异步运行时，提供了：
//   - 多线程工作窃取（work-stealing）调度器
//   - 异步 I/O（网络、文件）
//   - 定时器（tokio::time::sleep / interval）
//   - 同步原语（Mutex、RwLock、Semaphore、channel 等）
// ============================================================================

use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// 示例 1：基础的 async 函数与 .await
// ---------------------------------------------------------------------------

/// 模拟一个需要等待的工作任务。
///
/// 注意：这个函数的签名是 `async fn`，它返回一个 Future<Output = String>。
/// 函数体中使用 .await 来等待异步操作完成。
///
/// `tokio::time::sleep` 是 Tokio 提供的异步睡眠——它不会阻塞线程，
/// 而是将控制权交还给运行时，让运行时可以调度其他任务。
async fn simulate_work(name: &str, duration_ms: u64) -> String {
    println!("    [{}] 开始工作，预计耗时 {} ms...", name, duration_ms);
    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    println!("    [{}] 工作完成！", name);
    format!("{} 的结果", name)
}

/// 无需等待的任务——不包含 .await 的 async fn。
/// 编译器会发出警告，因为这样的函数没必要是 async 的。
/// 这里刻意保留 async 来展示：即使声明为 async，内部没有 .await 时
/// Future 会立即完成（同步返回）。
#[allow(dead_code)]
async fn immediate_result() -> String {
    "立即完成".to_string()
}

// ---------------------------------------------------------------------------
// 示例 2：异步数据获取与错误处理
// ---------------------------------------------------------------------------

/// 模拟异步获取数据（如 HTTP 请求、数据库查询）。
/// 返回 Result 类型，展示如何在异步上下文中传播错误。
async fn fetch_data(id: u32) -> Result<String, String> {
    match id {
        0 => Err("无效的 ID: 0".to_string()),
        1 => {
            // 模拟快请求（50 ms）
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(format!("数据块 #{}: 用户信息", id))
        }
        2 => {
            // 模拟慢请求（200 ms）
            tokio::time::sleep(Duration::from_millis(200)).await;
            Ok(format!("数据块 #{}: 订单列表", id))
        }
        3 => {
            // 模拟超慢请求（300 ms）
            tokio::time::sleep(Duration::from_millis(300)).await;
            Ok(format!("数据块 #{}: 统计报表", id))
        }
        _ => {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(format!("数据块 #{}: 默认数据", id))
        }
    }
}

/// 使用 ? 操作符在 async 上下文中传播错误。
/// async fn 结合 Result 非常自然——与同步代码的写法几乎一致。
async fn fetch_with_error_handling() -> Result<Vec<String>, String> {
    // `?` 在这里会将 Err 从 async fn 中提前返回
    let data1 = fetch_data(1).await?;
    let data2 = fetch_data(2).await?;
    // 注意：fetch_data(0) 会返回 Err，如果取消注释下面这行，函数会在此处返回 Err
    // let data_err = fetch_data(0).await?;

    Ok(vec![data1, data2])
}

// ---------------------------------------------------------------------------
// 示例 3：顺序执行 vs 并发执行
// ---------------------------------------------------------------------------

/// 顺序执行：每个任务在前一个完成后才开始。
/// 总耗时 = 所有任务耗时之和。
async fn sequential_execution() {
    println!("\n--- 顺序执行（Sequential） ---");
    let start = Instant::now();

    // 每次 .await 都阻塞当前 Future 的推进，直到该子 Future 完成
    let r1 = simulate_work("任务A", 100).await;
    let r2 = simulate_work("任务B", 150).await;
    let r3 = simulate_work("任务C", 100).await;

    let elapsed = start.elapsed();
    println!("    结果: [{}, {}, {}]", r1, r2, r3);
    println!(
        "    顺序执行总耗时: {:?}（约等于 100+150+100 = 350ms）",
        elapsed
    );
}

/// 并发执行：使用 tokio::join! 同时等待多个 Future。
/// 所有任务并发地推进，总耗时 ≈ 最慢的那个任务。
async fn concurrent_execution() {
    println!("\n--- 并发执行（Concurrent，使用 tokio::join!） ---");
    let start = Instant::now();

    // 创建 Future（此时还不会执行！Future 是惰性的）
    let fut1 = simulate_work("任务A", 100);
    let fut2 = simulate_work("任务B", 150);
    let fut3 = simulate_work("任务C", 100);

    // tokio::join! 会同时轮询所有 Future，并发推进
    let (r1, r2, r3) = tokio::join!(fut1, fut2, fut3);

    let elapsed = start.elapsed();
    println!("    结果: [{}, {}, {}]", r1, r2, r3);
    println!(
        "    并发执行总耗时: {:?}（约等于最慢的任务 = 150ms）",
        elapsed
    );
}

// ---------------------------------------------------------------------------
// 示例 4：tokio::spawn 创建并发任务
// ---------------------------------------------------------------------------

/// 使用 tokio::spawn 将 Future 提交到 Tokio 运行时，在独立的任务中执行。
///
/// spawn 返回 JoinHandle<T>，它本身也是一个 Future，
/// .await 它会等待任务完成并获取结果。
///
/// 与 tokio::join! 的区别：
///   - tokio::join! 在当前任务中同时 poll 多个 Future
///   - tokio::spawn 将 Future 提交为独立任务，可能在不同线程上执行
async fn spawn_concurrent_tasks() {
    println!("\n--- tokio::spawn 并发任务 ---");
    let start = Instant::now();

    // spawn 返回 JoinHandle，任务是立即提交到运行时的
    let handle1 = tokio::spawn(simulate_work("Spawn-A", 120));
    let handle2 = tokio::spawn(simulate_work("Spawn-B", 130));
    let handle3 = tokio::spawn(simulate_work("Spawn-C", 110));

    // 等待所有任务完成并收集结果
    // unwrap() 处理 JoinError（任务 panic 时会返回 Err）
    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();
    let r3 = handle3.await.unwrap();

    let elapsed = start.elapsed();
    println!("    结果: [{}, {}, {}]", r1, r2, r3);
    println!(
        "    spawn 并发总耗时: {:?}（约等于最慢任务 = 130ms）",
        elapsed
    );
}

// ---------------------------------------------------------------------------
// 示例 5：CPU 密集型任务与 spawn_blocking
// ---------------------------------------------------------------------------

/// 模拟一个 CPU 密集型计算（如密码哈希、图像处理）。
/// 在异步上下文中，CPU 密集型任务不能直接运行，因为它会阻塞线程，
/// 导致其他异步任务无法被调度。
async fn cpu_bound_example() {
    println!("\n--- CPU 密集型任务处理 ---");

    // 错误做法：直接在 async 上下文中做 CPU 密集计算（会阻塞整个线程）
    // let result = heavy_computation(); // 这会阻塞，导致其他任务饿死

    // 正确做法：使用 tokio::task::spawn_blocking 将 CPU 任务放到专用线程池
    let result = tokio::task::spawn_blocking(|| {
        // 这里可以安全地做 CPU 密集计算、同步阻塞操作等
        println!("    [CPU任务] 正在执行密集计算...");
        std::thread::sleep(Duration::from_millis(200)); // 模拟计算
        let sum: u64 = (0..10_000_000).sum();
        println!("    [CPU任务] 计算完成，sum = {}", sum);
        sum
    })
    .await
    .unwrap();

    println!("    CPU 密集任务结果: {}", result);
}

// ---------------------------------------------------------------------------
// 示例 6：Future 的惰性——不 .await 就不会执行
// ---------------------------------------------------------------------------

async fn demonstrate_laziness() {
    println!("\n--- Future 的惰性演示 ---");

    // 创建一个 Future，但没有 .await 它
    let _unused_future = simulate_work("不会被执行的Future", 1000);

    println!("    创建了一个 Future 但没有 .await 它");
    println!("    注意：上面没有出现 '[不会被执行的Future] 开始工作' 的消息！");
    println!("    因为 async fn 返回的 Future 是惰性的——未被 .await / poll，");
    println!("    其中的代码永远不会执行。");

    // 短暂等待，证明即使时间流逝也不会执行
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("    即使等待了 10ms，那个 Future 依然没有被执行。");

    // 现在显式 .await
    println!("    现在显式调用 .await：");
    let result = simulate_work("现在才执行", 50).await;
    println!("    结果: {}", result);
}

// ---------------------------------------------------------------------------
// 示例 7：综合示例 —— 模拟并发数据获取
// ---------------------------------------------------------------------------

async fn process_all() {
    println!("\n========================================");
    println!("  综合示例：并发获取多个数据源");
    println!("========================================");
    let start = Instant::now();

    // 同时发起 5 个数据请求
    let handles: Vec<_> = (1..=5).map(|id| tokio::spawn(fetch_data(id))).collect();

    // 等待所有请求完成
    let mut results = Vec::new();
    for handle in handles {
        match handle.await.unwrap() {
            Ok(data) => {
                println!("    ✓ 获取成功: {}", data);
                results.push(data);
            }
            Err(e) => println!("    ✗ 获取失败: {}", e),
        }
    }

    let elapsed = start.elapsed();
    println!("\n    并发获取 5 个数据源总耗时: {:?}", elapsed);
    println!("    如果顺序执行，需要: 50+200+300+100+100 = 750ms");
    println!("    并发执行只需约: 300ms（最慢的那个请求）");
}

// ============================================================================
// 程序入口
// ============================================================================

/// #[tokio::main] 是一个属性宏（attribute macro），它将 main 函数转换为
/// Tokio 运行时的入口点。
///
/// 展开后大致等价于：
/// ```ignore
/// fn main() {
///     tokio::runtime::Runtime::new()
///         .unwrap()
///         .block_on(async { /* 原来的 main 函数体 */ });
/// }
/// ```
///
/// 也可以使用 `#[tokio::main(flavor = "multi_thread")]` 显式指定多线程运行时。
/// 默认是 multi_thread，工作线程数等于 CPU 核心数。
#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  第 22 章：Rust 异步编程入门                ║");
    println!("║  Future | async/await | Tokio                ║");
    println!("╚══════════════════════════════════════════════╝");

    let total_start = Instant::now();

    // ---- 示例 1：基础 async/await ----
    println!("\n========== 示例 1：基础 async/await ==========");
    let result = simulate_work("基础任务", 80).await;
    println!("  返回结果: {}", result);

    // ---- 示例 2：错误处理 ----
    println!("\n========== 示例 2：异步上下文中的错误处理 ==========");
    match fetch_with_error_handling().await {
        Ok(data) => println!("  获取到的数据: {:?}", data),
        Err(e) => println!("  错误: {}", e),
    }

    // ---- 示例 3：顺序 vs 并发 ----
    println!("\n========== 示例 3：顺序执行 vs 并发执行 ==========");
    sequential_execution().await;
    concurrent_execution().await;

    // ---- 示例 4：tokio::spawn ----
    println!("\n========== 示例 4：tokio::spawn ==========");
    spawn_concurrent_tasks().await;

    // ---- 示例 5：CPU 密集型任务 ----
    println!("\n========== 示例 5：CPU 密集型任务 ==========");
    cpu_bound_example().await;

    // ---- 示例 6：惰性 ----
    println!("\n========== 示例 6：Future 的惰性 ==========");
    demonstrate_laziness().await;

    // ---- 示例 7：综合示例 ----
    println!("\n========== 示例 7：综合示例 ==========");
    process_all().await;

    // ---- 总结 ----
    println!("\n========================================");
    println!("  所有示例执行完毕");
    println!("  总耗时: {:?}", total_start.elapsed());
    println!("========================================");

    println!("\n关键要点回顾：");
    println!("  1. async fn 返回 Future，是惰性的——不 .await 就不执行");
    println!("  2. .await 将控制权交还给运行时，不阻塞操作系统线程");
    println!("  3. tokio::join! 在当前任务中并发等待多个 Future");
    println!("  4. tokio::spawn 将 Future 提交为独立任务，可能并行执行");
    println!("  5. CPU 密集任务用 spawn_blocking，避免阻塞异步运行时");
    println!("  6. async 适合 I/O 密集型（网络请求、文件读写），而非 CPU 密集型");
    println!("  7. 异步 != 多线程，但 Tokio 默认使用多线程工作窃取调度器");
}
