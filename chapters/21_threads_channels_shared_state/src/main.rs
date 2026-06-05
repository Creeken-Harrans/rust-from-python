// ============================================================================
// Chapter 21: 线程、信道与共享状态 — Rust 并发编程全面演示
// ============================================================================
// 本程序系统地展示 Rust 并发编程的核心概念：
//   - 线程创建与等待 (spawn / join)
//   - move 闭包的语义与必要性
//   - 消息传递 (mpsc::channel)
//   - 共享状态 (Arc<Mutex<T>>)
//   - Send / Sync trait 体系
//   - 死锁原理与避免
// ============================================================================

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// ---------------------------------------------------------------------------
// 辅助函数：打印分隔用的节标题
// ---------------------------------------------------------------------------
fn section(title: &str) {
    println!();
    println!("{}", "═".repeat(60));
    println!("  {}", title);
    println!("{}", "═".repeat(60));
}

// ============================================================================
// 1. 线程创建与等待：spawn + join
// ============================================================================
fn demo_spawn_and_join() {
    section("1. spawn 与 join —— 创建线程并收集结果");

    println!("主线程 id: {:?}", thread::current().id());
    println!();

    // 数据：计算斐波那契数列第 n 项的独立任务
    let tasks: Vec<u32> = vec![10, 20, 30, 35, 38];

    // 启动多个线程，每个线程执行独立计算
    let handles: Vec<thread::JoinHandle<u64>> = tasks
        .into_iter()
        .map(|n| {
            thread::spawn(move || {
                let tid = thread::current().id();
                println!("  [启动] 计算 fib({}) — 线程 {:?}", n, tid);

                // 模拟耗时计算
                let result = fib(n);

                println!("  [完成] fib({}) = {} — 线程 {:?}", n, result, tid);
                result
            })
        })
        .collect();

    println!("已启动 {} 个工作线程，等待所有线程完成...", handles.len());

    // 将线程按 id 从 handles 中取出，不关心具体顺序
    // join 会阻塞当前线程直到子线程结束
    let mut results: Vec<u64> = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;

    for handle in handles {
        match handle.join() {
            Ok(value) => {
                results.push(value);
                success_count += 1;
            }
            Err(_) => {
                // 线程 panic 时会返回 Err(Any)
                eprintln!("  ⚠ 某个线程发生了 panic！");
                error_count += 1;
            }
        }
    }

    results.sort_unstable();
    println!();
    println!("全部完成 — 成功: {}, 失败: {}", success_count, error_count);
    println!("计算结果: {:?}", results);
    println!();
    println!("核心要点:");
    println!("  · thread::spawn(f) 接收一个闭包，立即在新线程中执行");
    println!("  · spawn 返回 JoinHandle<T>，其中 T 是闭包的返回值类型");
    println!("  · handle.join() 阻塞当前线程，等待子线程结束");
    println!("  · 如果子线程 panic，join() 返回 Err(Any)");
}

// 简单的递归斐波那契（故意不优化，用于演示耗时计算）
fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

// ============================================================================
// 2. move 闭包 —— 为什么线程需要 move
// ============================================================================
fn demo_move_closure() {
    section("2. move 闭包 —— 把数据的所有权移入线程");

    // ----- 场景 A：值类型（Copy）-----
    let x: i32 = 42;
    // i32 是 Copy 类型，即使不加 move，闭包也会自动复制 x
    let handle_a = thread::spawn(move || {
        println!("  [场景A] x = {} (i32 是 Copy 类型)", x);
    });
    handle_a.join().unwrap();

    // ----- 场景 B：非 Copy 类型（String）—— 必须用 move -----
    let message = String::from("你好，Rust 并发！");

    // 如果忘记 move，编译器会报错：
    //
    //   error[E0373]: closure may outlive the current function,
    //   but it borrows `message`, which is owned by the current function
    //
    // 因为 thread::spawn 要求闭包是 'static 的 —— 线程可能比当前函数活得更久，
    // 闭包不能借用局部变量（借用的引用可能在变量被释放后变成悬垂指针）。
    //
    // 正确的写法：加上 move 关键字，把数据所有权移入闭包：
    let handle_b = thread::spawn(move || {
        println!("  [场景B] 线程获得了所有权: {}", message);
        // message 的所有权在此，函数结束后 message 被 drop
    });
    handle_b.join().unwrap();
    // 注意：此时 message 已经不可用，因为所有权已被移走

    // ----- 场景 C：Vec 在多个线程间的所有权问题 -----
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  [场景C] 原始数据: {:?}", numbers);

    // 移动整个 Vec 到一个线程
    let handle_c = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("  [场景C] 线程内求和: {}", sum);
        // numbers 在此被释放
    });
    handle_c.join().unwrap();

    // numbers 已不可用

    // ----- 场景 D：如果需要在多个线程共享数据？-----
    // 不能用 move 把同一个 String 移给多个线程（所有权只能有一个）。
    // 也不能用裸引用（生命周期问题）。
    // 解决方案：Arc（下一节介绍）或 channel（下一节介绍）。
    println!();
    println!("核心要点:");
    println!("  · thread::spawn 的闭包签名是 FnOnce() + Send + 'static");
    println!("  · 'static 意味着闭包不能借用任何非 'static 的数据");
    println!("  · move 把捕获变量的所有权移入闭包，满足 'static 要求");
    println!("  · Copy 类型（i32, bool 等）不加 move 也会被复制，但推荐显式写上");
}

// ============================================================================
// 3. 消息传递：mpsc::channel —— 多生产者，单消费者
// ============================================================================
fn demo_channels() {
    section("3. 消息传递 —— mpsc::channel（多生产者，单消费者）");

    // mpsc = Multiple Producer, Single Consumer
    // tx: Sender（发送端），可克隆
    // rx: Receiver（接收端），不可克隆
    let (tx, rx) = mpsc::channel::<String>();

    println!("信道已创建。将启动 5 个生产者线程...");

    // 启动 5 个生产者线程
    let producer_count = 5;

    for i in 0..producer_count {
        // 克隆 tx 给每个线程一份
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for msg_num in 0..3 {
                // 模拟一些工作
                thread::sleep(Duration::from_millis(10 * i as u64 + 5 * msg_num as u64));

                let message = format!("生产者 #{}, 消息 #{}", i, msg_num);
                if let Err(e) = tx_clone.send(message) {
                    eprintln!("  ⚠ 生产者 #{} 发送失败: {}（接收端已关闭）", i, e);
                    break;
                }
            }
            println!("  [生产者 #{}] 所有消息已发送完毕，Sender 将被 drop", i);
            // tx_clone 在此离开作用域，被 drop
        });
    }

    // ★ 关键：必须 drop 原始的 tx，否则 rx 永远不会收到 "信道关闭" 的信号
    //    因为只要还有一个 Sender 存在，信道就保持打开状态。
    drop(tx);

    println!("原始 tx 已 drop，开始接收所有消息...");
    println!();

    // 通过迭代 rx 收集所有消息
    // 当所有 Sender 都被 drop 后，迭代自动结束
    let mut all_messages: Vec<String> = Vec::new();
    for msg in rx {
        println!("  收到: {}", msg);
        all_messages.push(msg);
    }

    println!();
    println!("接收完毕！共收到 {} 条消息。", all_messages.len());
    println!();
    println!("核心要点:");
    println!("  · mpsc = Multiple Producer, Single Consumer");
    println!("  · tx.clone() 克隆发送端给多个生产者");
    println!("  · tx.send(msg) 发送消息（如果接收端已关闭则返回 Err）");
    println!("  · rx.recv() 阻塞接收一条消息，或 rx.iter() 迭代到信道关闭");
    println!("  · 必须 drop 所有 Sender，Receiver 才知道信道已关闭");
    println!("  · Channel 天然保证消息的顺序是发送顺序（FIFO）");
}

// ============================================================================
// 4. 共享状态：Arc<Mutex<T>> —— 多线程共享可变数据
// ============================================================================
fn demo_arc_mutex() {
    section("4. 共享状态 —— Arc<Mutex<T>>");

    // ------------------------------------------------------------------
    // 为什么需要 Arc？
    //   Rc<T> 使用非原子引用计数，只适合单线程。
    //   Arc<T>（Atomic Reference Count）使用原子操作，线程安全。
    //
    // 为什么需要 Mutex？
    //   多个线程同时修改同一个值会导致数据竞争（data race）。
    //   Mutex<T> 提供互斥访问：同一时刻只有一个线程能访问数据。
    // ------------------------------------------------------------------

    // 创建共享计数器：Arc 包装 Mutex 包装 i32
    let counter = Arc::new(Mutex::new(0));

    println!("初始计数器值: {}", *counter.lock().unwrap());
    println!("将启动 10 个线程，每个线程递增计数器 1000 次...");

    let thread_count = 10;
    let increments_per_thread = 1000;
    let mut handles = Vec::new();

    for i in 0..thread_count {
        // Arc::clone 增加引用计数（原子操作），克隆出另一个指向同一数据的 Arc
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                // lock() 返回 LockResult<MutexGuard<T>>
                // MutexGuard 实现了 Deref 和 DerefMut，可以像普通引用一样使用
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // MutexGuard 在此离开作用域，自动释放锁（Drop）
            }
            println!("  [线程 #{}] 完成 {} 次递增", i, increments_per_thread);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = *counter.lock().unwrap();
    let expected = thread_count * increments_per_thread;

    println!();
    println!("最终计数器值: {}", final_value);
    println!("期望值:        {}", expected);
    println!(
        "结果: {}",
        if final_value == expected {
            "✓ 正确！没有数据竞争。"
        } else {
            "✗ 错误！存在数据竞争。"
        }
    );

    // 检查是否只有最后一个 Arc 引用
    // Arc::try_unwrap 只有在引用计数为 1 时才能成功
    match Arc::try_unwrap(counter) {
        Ok(mutex) => {
            let inner = mutex.into_inner().unwrap();
            println!("成功解包 Arc — 计数器最终值: {}", inner);
        }
        Err(_) => {
            println!("Arc 仍有多个引用（这不应该发生）");
        }
    }

    println!();
    println!("核心要点:");
    println!("  · Arc<T> = 原子引用计数，允许多个线程共享同一数据的所有权");
    println!("  · Mutex<T> = 互斥锁，保证同一时刻只有一个线程访问 T");
    println!("  · lock() 返回 MutexGuard，它像一个智能指针（Deref/DerefMut）");
    println!("  · MutexGuard 离开作用域时自动释放锁（RAII，无需手动 unlock）");
    println!("  · 如果线程在持有锁时 panic，Mutex 会被 \"毒化\"（poisoned）");
    println!("  · lock() 在 poisoned 状态下返回 Err，防止使用可能不一致的数据");
}

// ============================================================================
// 5. Send 与 Sync —— Rust 的线程安全类型体系
// ============================================================================
fn demo_send_sync() {
    section("5. Send 与 Sync —— 编译期线程安全保证");

    println!(
        r#"
Rust 通过两个 marker trait 在编译期保证线程安全：

┌─────────┬──────────────────────────────────────────────────────────┐
│ Trait   │ 含义                                                     │
├─────────┼──────────────────────────────────────────────────────────┤
│ Send    │ 该类型的值的所有权可以在线程之间传递。                    │
│         │ 例如：i32, String, Vec<T: Send>, Arc<T: Send>            │
│         │ 反例：Rc<T> 不是 Send（非原子引用计数）                   │
├─────────┼──────────────────────────────────────────────────────────┤
│ Sync    │ 该类型的不可变引用可以在线程之间安全共享。                │
│         │ T: Sync 当且仅当 &T: Send                                │
│         │ 例如：i32, &str, Mutex<T: Send>                          │
│         │ 反例：RefCell<T> 不是 Sync（运行时借用检查，非线程安全）  │
└─────────┴──────────────────────────────────────────────────────────┘

自动推导规则：
  · 大多数基本类型（i32, bool, String 等）都是 Send + Sync
  · 如果类型的所有字段都是 Send，则该类型自动 Send
  · 如果类型的所有字段都是 Sync，则该类型自动 Sync
  · 裸指针 *const T 和 *mut T 既不是 Send 也不是 Sync（需 unsafe）

常见的 Send/Sync 状态：

  i32, f64, bool          → Send ✓  Sync ✓
  String, Vec<T>           → Send ✓ (T: Send)  Sync ✓ (T: Sync)
  &T                       → Send ✓ (T: Sync)  Sync ✓ (T: Sync)
  Rc<T>                    → Send ✗  Sync ✗  （需用 Arc<T> 代替）
  RefCell<T>               → Send ✓ (T: Send)  Sync ✗  （需用 Mutex<T> 代替）
  Mutex<T>                 → Send ✓ (T: Send)  Sync ✓ (T: Send)
  Arc<T>                   → Send ✓ (T: Send + Sync)  Sync ✓ (T: Send + Sync)

为什么 Rc<T> 不是 Send + Sync？
  · Rc 使用非原子的引用计数操作（效率高，但不安全跨线程）
  · 如果两个线程同时修改引用计数，会导致计数错误和 double-free
  · Arc<T> 使用原子操作（稍慢，但线程安全）

为什么 RefCell<T> 不是 Sync？
  · RefCell 在运行时检查借用规则（borrow/borrow_mut）
  · 它的借用计数器不是原子的，多线程共享会导致数据竞争
  · Mutex<T> 通过锁机制保证互斥，是线程安全的替代方案
"#
    );

    println!();
    println!("实际验证（编译会通过这些）：");
    println!("  · 尝试把 Rc<T> 传给 thread::spawn → 编译错误");
    println!("  · 尝试把 &RefCell<T> 共享给多线程 → 编译错误");
    println!("  · 使用 Arc<T> 和 Mutex<T> 替代 → 编译通过 ✓");
}

// ============================================================================
// 6. 死锁：原理与避免
// ============================================================================
fn demo_deadlock_warning() {
    section("6. 死锁 —— 原理与避免");

    println!(
        r#"
死锁（Deadlock）是指两个或多个线程互相等待对方释放资源，导致所有线程永远阻塞。

┌─────────────────────────────────────────────────────────────────────┐
│                         死锁的四个必要条件                           │
├─────────────────────────────────────────────────────────────────────┤
│ 1. 互斥（Mutual Exclusion）     ：资源一次只能被一个线程使用         │
│ 2. 持有并等待（Hold and Wait）  ：线程持有资源时等待其他资源         │
│ 3. 不可抢占（No Preemption）    ：资源只能由持有者自愿释放           │
│ 4. 循环等待（Circular Wait）    ：线程间形成环形等待链               │
└─────────────────────────────────────────────────────────────────────┘

【死锁示例（伪代码，本程序不会执行）】

    线程 A:                   线程 B:
    ────────                 ────────
    lock(mutex_1);           lock(mutex_2);
    // 持有 mutex_1          // 持有 mutex_2
    lock(mutex_2); ← 等待B   lock(mutex_1); ← 等待A
    // 这里永远到不了！       // 这里也永远到不了！
    unlock(mutex_2);         unlock(mutex_1);
    unlock(mutex_1);         unlock(mutex_2);

    A 持有 mutex_1 等待 mutex_2，B 持有 mutex_2 等待 mutex_1。
    两个线程互相等待，永远不会继续执行。

【本程序的安全演示】

    下面演示一个不会死锁的场景：两个线程按相同顺序获取锁。
"#
    );

    // ---- 安全的多锁获取：始终按相同顺序加锁 ----
    let resource_a = Arc::new(Mutex::new(0));
    let resource_b = Arc::new(Mutex::new(0));

    let a1 = Arc::clone(&resource_a);
    let b1 = Arc::clone(&resource_b);

    let handle_a = thread::spawn(move || {
        for _ in 0..100 {
            // ★ 关键：始终先锁 a 再锁 b
            let mut a = a1.lock().unwrap();
            thread::sleep(Duration::from_micros(10)); // 模拟工作
            let mut b = b1.lock().unwrap();

            *a += 1;
            *b += 1;
        }
        println!("  [线程A] 完成，按 (a, b) 顺序加锁，无死锁");
    });

    let a2 = Arc::clone(&resource_a);
    let b2 = Arc::clone(&resource_b);

    let handle_b = thread::spawn(move || {
        for _ in 0..100 {
            // ★ 关键：同样先锁 a 再锁 b（与线程 A 相同顺序）
            let mut a = a2.lock().unwrap();
            thread::sleep(Duration::from_micros(10)); // 模拟工作
            let mut b = b2.lock().unwrap();

            *a += 2;
            *b += 2;
        }
        println!("  [线程B] 完成，按 (a, b) 顺序加锁，无死锁");
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!(
        "最终状态: a = {}, b = {}",
        *resource_a.lock().unwrap(),
        *resource_b.lock().unwrap()
    );

    println!();
    println!("核心要点:");
    println!("  · 死锁需要四个条件同时满足才能发生");
    println!("  · 避免死锁的最简单方法：始终按相同顺序获取多个锁");
    println!("  · Rust 的类型系统无法在编译期检测死锁（这是运行时行为）");
    println!("  · 但 Rust 的 MutexGuard RAII 机制确保锁一定会被释放（无泄漏）");
    println!("  · 考虑使用 try_lock() 代替 lock() 实现超时/重试策略");
    println!("  · 对于复杂的锁依赖，可以使用 std::sync::RwLock（读写锁）");
}

// ============================================================================
// 7. 对比表格：单线程 vs 多线程 所有权与内部可变性
// ============================================================================
fn print_comparison_table() {
    section("7. 对比表格 —— 内部可变性与共享所有权");

    println!(
        r#"
┌─────────────────────┬───────────────────────────┬──────────────────────────────┐
│         类型         │       适用场景             │           说明               │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  单线程所有权             │  基础 Rust 所有权规则         │
│  单一所有权          │  let x = T;              │  一个值只有一个所有者         │
│                      │  let y = x;  // 移动      │  编译期检查，无运行时开销     │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  单线程共享所有权          │  非原子引用计数              │
│   Rc<T>             │  多个所有者共享同一堆数据   │  clone() 增加计数            │
│                      │  不能跨线程使用！          │  Drop 时减少计数并释放       │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  多线程共享所有权          │  原子引用计数                │
│   Arc<T>            │  等价于线程安全的 Rc<T>    │  clone() 使用原子操作        │
│                      │  用于不可变数据共享        │  有轻微性能开销              │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  单线程内部可变性          │  运行时借用检查              │
│   RefCell<T>        │  在不可变引用下修改数据     │  borrow()/borrow_mut()       │
│                      │  违反借用规则会 panic！    │  非线程安全（!Sync）         │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  多线程互斥访问            │  操作系统级别的锁            │
│   Mutex<T>          │  同一时刻只有一个线程访问    │  lock() 获取 MutexGuard      │
│                      │  等价于线程安全的 RefCell  │  RAII 自动释放               │
├─────────────────────┼───────────────────────────┼──────────────────────────────┤
│                      │  多线程读写锁              │  读多写少场景优化            │
│   RwLock<T>         │  多个读者/单个写者          │  read() 共享锁               │
│                      │  比 Mutex 更细粒度         │  write() 独占锁              │
└─────────────────────┴───────────────────────────┴──────────────────────────────┘

组合使用示例：

  Rc<RefCell<T>>      →  单线程中，多所有者共享可变数据
  Arc<Mutex<T>>       →  多线程中，多所有者共享可变数据 ← 最常用！
  Arc<RwLock<T>>      →  多线程中，读多写少的可变共享

Python 对照：

  Python                         Rust
  ───────                        ────
  threading.Thread               std::thread::spawn
  queue.Queue                    std::sync::mpsc::channel
  threading.Lock                 std::sync::Mutex
  (无需 Rc/Arc, GC 自动管理)     Rc<T> / Arc<T> (显式引用计数)
  (GIL 保护 Python 内部状态)     (编译期保证，无运行时 GIL 开销)
"#
    );
}

// ============================================================================
// 8. 恐惧-free 并发：Rust 的并发保证
// ============================================================================
fn print_fearless_concurrency() {
    section("8. Fearless Concurrency —— 恐惧-free 并发");

    println!(
        r#"
"Fearless Concurrency"（无畏并发）是 Rust 社区的核心口号之一。

它的含义是：Rust 的类型系统和所有权模型让程序员能够放心地编写并发代码，
因为编译器会在编译期捕捉几乎所有的并发错误。

Rust 保证：

  1. 数据竞争（Data Race）零容忍
     编译器在类型层面阻止数据竞争 —— 代码要么通过编译，要么就不会有数据竞争。

  2. 数据竞争 ≠ 竞态条件（Race Condition）
     竞态条件是更高层的逻辑错误（比如两个线程同时读写一个文件导致顺序不确定），
     Rust 不能完全防止竞态条件，但它消除了更底层、更危险的数据竞争。

  3. 线程安全由类型系统强制执行
     Send / Sync trait 自动推导，错误使用会导致编译错误而非运行时崩溃。

  4. 没有 GIL（全局解释器锁）
     不像 Python 的 CPython 解释器有 GIL 限制真正的并行，Rust 线程可以在
     多核 CPU 上真正并行执行。

  5. 零成本抽象
     Arc、Mutex 等抽象在不需要时不会引入额外开销。
     MutexGuard 的 RAII 机制在编译器优化后与手写锁操作等效。

数据竞争 vs 竞态条件：

  数据竞争 (Data Race):  Rust 编译期阻止
    · 两个或多个线程同时访问同一内存
    · 至少一个线程在写入
    · 没有同步机制
    → Rust: 编译错误！不可能通过编译。

  竞态条件 (Race Condition): Rust 不阻止（逻辑层面）
    · 两个线程操作顺序不确定，导致结果不同
    · 例如：一个线程读文件，另一个线程正在写同一个文件
    → 需要程序员在逻辑层面保证正确性。
"#
    );
}

// ============================================================================
// main —— 编排所有演示
// ============================================================================
fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Chapter 21: Rust 并发编程                                  ║");
    println!("║  线程 · 信道 · 共享状态 · Send/Sync · 死锁                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    demo_spawn_and_join();
    demo_move_closure();
    demo_channels();
    demo_arc_mutex();
    demo_send_sync();
    demo_deadlock_warning();
    print_comparison_table();
    print_fearless_concurrency();

    section("总结");
    println!("Rust 并发编程的核心工具:");
    println!("  · thread::spawn    — 创建新线程");
    println!("  · mpsc::channel    — 线程间消息传递");
    println!("  · Arc<T>           — 多线程共享所有权");
    println!("  · Mutex<T>         — 互斥访问可变数据");
    println!("  · Send / Sync      — 编译期线程安全保证");
    println!();
    println!("Python 程序员需要适应的最大变化:");
    println!("  1. 没有 GIL 的保护 → 需要显式同步（Mutex, Channel）");
    println!("  2. 没有 GC → 需要显式管理共享所有权（Arc）");
    println!("  3. 编译器会阻止数据竞争 → 许多 Python 中的运行时 bug 在 Rust 中直接变编译错误");
    println!();
}
