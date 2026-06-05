# 第 22 章：异步编程 —— Future、async/await 与 Tokio 入门

---

## 目录

1. [本章概述](#本章概述)
2. [同步 vs 异步：概念差异](#同步-vs-异步概念差异)
3. [并发 vs 并行](#并发-vs-并行)
4. [Rust 异步编程的核心设计](#rust-异步编程的核心设计)
5. [Future trait：惰性计算](#future-trait惰性计算)
6. [async fn：创建 Future 的语法糖](#async-fn创建-future-的语法糖)
7. [.await：驱动 Future 执行](#await驱动-future-执行)
8. [运行时 Runtime：为什么 Rust 需要它](#运行时-runtime为什么-rust-需要它)
9. [Tokio：最流行的异步运行时](#tokio最流行的异步运行时)
10. [#[tokio::main] 宏详解](#tokiomain-宏详解)
11. [tokio::spawn：并发任务](#tokiospawn并发任务)
12. [CPU 密集型 vs I/O 密集型](#cpu-密集型-vs-io-密集型)
13. [异步与多线程的关系](#异步与多线程的关系)
14. [常见陷阱与最佳实践](#常见陷阱与最佳实践)
15. [Python asyncio 对照](#python-asyncio-对照)
16. [运行本章代码](#运行本章代码)
17. [本章小结](#本章小结)

---

## 本章概述

本章介绍 Rust 中异步编程的核心概念：`Future` trait、`async`/`await` 语法以及最流行的异步运行时 **Tokio**。

**学习目标：**
- 理解同步与异步的本质区别
- 掌握 `Future` trait 的设计理念（惰性计算）
- 能用 `async fn` 和 `.await` 编写异步代码
- 理解为什么 Rust 需要独立的异步运行时
- 会使用 `tokio::join!` 和 `tokio::spawn` 实现并发
- 识别 CPU 密集和 I/O 密集型任务，知道何时使用异步

---

## 同步 vs 异步：概念差异

### 同步（Synchronous）

在同步编程模型中，代码以**顺序阻塞**的方式执行：

```
操作 A 开始 → 等待 A 完成 → 操作 B 开始 → 等待 B 完成 → 操作 C 开始
```

当一个函数发起 I/O 操作（如读取文件、发送 HTTP 请求）时，当前**线程会被阻塞**，
直到 I/O 完成才会继续执行后续代码。被阻塞的线程什么也做不了，白白浪费系统资源。

**生活类比：** 你在咖啡店排队点单，必须等前面的人点完、做完、取走咖啡，才能轮到你。
你全程站在柜台前等待，不能做其他事。

### 异步（Asynchronous）

在异步编程模型中，当一个任务需要等待时，它将**控制权交还给运行时**，
运行时可以切换到其他任务继续工作：

```
运行时调度：A 开始 → A 遇到等待 → 切换到 B → B 遇到等待 → 切换到 C → C 遇到等待 → A 完成 → ...
```

**生活类比：** 你点完咖啡后拿到一个取餐牌，然后可以坐到旁边回邮件、看书。
咖啡做好时会叫你的号，你再过去取。等待期间你没有傻站在柜台前。

### 核心区别总结

| 维度 | 同步 | 异步 |
|-----|-----|-----|
| 等待方式 | 线程阻塞，原地等待 | 交出控制权，运行时调度 |
| 资源占用 | 每个阻塞任务占用一个线程 | 成千上万个任务共享少量线程 |
| 适合场景 | CPU 密集型、简单程序 | I/O 密集型、高并发服务 |
| 编程复杂度 | 简单直观 | 需要理解 Future/async/await |
| 错误处理 | 标准 Result 传播 | 支持 `?` 操作符，基本一致 |

---

## 并发 vs 并行

这两个概念经常被混淆，但它们是不同的：

### 并发（Concurrency）

**并发是关于"结构"的** —— 程序被组织成多个可以独立推进的任务。
这些任务不一定同时执行，它们可以交错（interleave）执行。

```
时间轴：  AAAA BBBB AAAA CCCC BBBB CCCC
          ↑ 任务 A、B、C 在单核上交替执行 ↑
```

并发是**逻辑上**的同时，解决的是"如何管理多个任务"的问题。

### 并行（Parallelism）

**并行是关于"执行"的** —— 多个任务真正地**同时**运行在不同的 CPU 核心上。

```
核心 1：  AAAAAAAAAAAAAAAAAAAAAAAA
核心 2：  BBBBBBBBBBBBBBBBBBBBBBBB
核心 3：  CCCCCCCCCCCCCCCCCCCCCCCC
          ↑ 三个任务同时在不同核心上执行 ↑
```

并行是**物理上**的同时，解决的是"如何加速计算"的问题。

### 关系

- **可以并发但不并行：** 单核 CPU 上的异步程序——任务交替执行，宏观上像是同时
- **可以并行但不并发：** 单纯的多线程数值计算——每个线程独立运行，互不交互
- **可以既并发又并行：** 多核 CPU 上的 Tokio 应用——多个任务在不同核心上真正同时运行
- **可以既不并发也不并行：** 单个同步函数顺序执行

Rust 的异步模型**主要用于解决并发问题**（高效管理大量 I/O 任务），
但 Tokio 默认的多线程运行时也天然获得了并行能力。

---

## Rust 异步编程的核心设计

Rust 的异步模型有以下几个关键设计决策：

### 1. 零成本抽象

`async fn` 编译后被展开为一个状态机（state machine）结构体。
没有 GC、没有装箱（除非显式 `Box<dyn Future>`），栈变量存储在状态机内部。
每个 `.await` 点对应状态机的一个分支。

### 2. 惰性求值

`async fn` 返回的 `Future` **在未被轮询（poll）之前什么也不做**。
这与 JavaScript 的 Promise（创建即开始执行）完全不同。

### 3. 运行时分离

Rust 标准库只定义了 `Future` trait，不提供执行环境。
用户可以选择不同的运行时实现（Tokio、async-std、smol 等），
也可以自己实现。这种设计与 Python/JavaScript 内置事件循环形成对比。

### 4. 无栈协程

Rust 的异步模型基于**无栈协程**（stackless coroutine）。
每个 `Future` 的状态机嵌入在调用者的栈帧中，不需要分配独立的栈空间。
这使得创建成千上万个 Future 非常廉价。

---

## Future trait：惰性计算

`Future` 是 Rust 异步编程的基石，定义在标准库中：

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### 关键要素

- **`type Output`：** Future 完成时产出的值的类型。例如 `async fn foo() -> i32` 返回的 Future 的 `Output = i32`。
- **`poll` 方法：** 运行时调用来推进 Future 的执行。返回值有两种：
  - `Poll::Pending` —— "我还没准备好，下次再问我"
  - `Poll::Ready(value)` —— "我完成了，这是结果"
- **`Pin<&mut Self>`：** 确保 Future 在内存中的位置不变。因为自引用结构体在移动后会失效，`Pin` 保证 Future 不被移动。
- **`Context`：** 携带 `Waker`——Future 在等待外部事件时，通过 Waker 通知运行时"我可以继续了"。

### 惰性的含义

```rust
// 创建一个 Future——什么都不执行！
let future = some_async_fn();

// 只有被 poll（通常通过 .await）时，代码才开始执行
let result = future.await;  // 现在才执行
```

这种设计的一个实际后果是：如果你忘了 `.await` 一个 Future，那段代码永远不会运行。
编译器通常会给出 `#[warn(unused_must_use)]` 警告。

---

## async fn：创建 Future 的语法糖

当你写下：

```rust
async fn greet(name: &str) -> String {
    format!("你好，{}！", name)
}
```

编译器将其转换为一个实现了 `Future<Output = String>` 的结构体。大致等价于：

```rust
fn greet(name: &str) -> impl Future<Output = String> {
    async move {
        format!("你好，{}！", name)
    }
}
```

### async 代码块

除了 `async fn`，你还可以使用 `async {}` 代码块内联创建一个 Future：

```rust
let my_future = async {
    let a = fetch_data(1).await;
    let b = fetch_data(2).await;
    a + b
};
// my_future 的类型是 impl Future<Output = i32>
```

### 异步闭包

Rust 目前（edition 2021）没有原生的异步闭包语法（`async || {}` 在 edition 2024 中稳定）。
变通方案是使用同步闭包返回一个 Future：

```rust
let closure = || async {
    fetch_data(1).await
};
```

---

## .await：驱动 Future 执行

`.await` 是一个后缀操作符（postfix operator），语法为 `future.await`。

### 它的作用

1. 将当前 `Future` 注册到运行时（通过 `cx` 传递的 `Waker`）
2. 调用 `future.poll()`
3. 如果返回 `Poll::Pending`，当前函数**挂起**（suspend），控制权交还运行时
4. 如果返回 `Poll::Ready(value)`，获取值并继续执行

### 重要性质

- `.await` **只阻塞当前 Future**，不阻塞整个操作系统线程
- `.await` 只能在 `async fn` 或 `async {}` 块中使用
- `.await` 会消耗（consume）这个 Future——调用后不能再使用

### 示例

```rust
async fn example() {
    // 创建一个 Future（惰性，还没有执行）
    let work = simulate_work("任务1", 100);

    // .await 驱动它执行直到完成
    let result = work.await;
    println!("{}", result);
}
```

---

## 运行时 Runtime：为什么 Rust 需要它

### 为什么需要独立的运行时？

在 Python 和 JavaScript 中，事件循环（event loop）是语言内置的一部分：

- **Python：** `asyncio.run()` 启动事件循环，它是标准库的一部分
- **JavaScript：** 浏览器或 Node.js 提供了隐式的事件循环

Rust 选择了不同的道路：标准库只定义 `Future` trait（接口），不提供具体的执行器（executor）。
这有几个原因：

1. **嵌入式 / 无 OS 环境：** Rust 可以运行在微控制器上，那里没有标准的事件循环
2. **可定制性：** 不同场景需要不同的调度策略（单线程、多线程、优先级调度等）
3. **避免运行时开销：** 不使用的特性不需要为其付出编译/运行时成本
4. **生态竞争：** 多个运行时可以竞争，推动更好的实现

### 运行时的职责

- **调度任务：** 决定哪些 Future 可以被轮询
- **管理 I/O：** 与操作系统的异步 I/O 接口交互（epoll / kqueue / IOCP）
- **提供定时器：** `sleep`、`interval` 等
- **工作窃取：** 在多线程运行时中平衡负载

---

## Tokio：最流行的异步运行时

### 什么是 Tokio？

[Tokio](https://tokio.rs) 是 Rust 生态中**使用最广泛**的异步运行时。
它最初由 Rust 社区的核心成员创建，目前由 Tokio 团队维护，
被 AWS Lambda、Discord、Dropbox 等公司用于生产环境。

### Tokio 提供的核心能力

| 能力 | 说明 |
|-----|-----|
| **多线程工作窃取调度器** | 自动将任务分配到 CPU 核心，空闲线程从忙碌线程"窃取"任务 |
| **异步 TCP/UDP** | `tokio::net::TcpListener`、`TcpStream`、`UdpSocket` |
| **异步文件 I/O** | `tokio::fs` —— 基于 `spawn_blocking` 的异步文件操作 |
| **定时器** | `tokio::time::sleep`、`interval`、`timeout` |
| **同步原语** | `tokio::sync::Mutex`、`RwLock`、`Semaphore`、`mpsc`、`broadcast`、`watch`、`oneshot` |
| **信号处理** | `tokio::signal` —— 优雅关闭 |
| **进程管理** | `tokio::process::Command` |

### 添加到项目

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`features = ["full"]` 启用了所有特性。在生产中可以只启用需要的特性以减小编译体积：

```toml
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time", "net"] }
```

---

## #[tokio::main] 宏详解

### 基本用法

```rust
#[tokio::main]
async fn main() {
    // 这里是异步上下文
    some_async_fn().await;
}
```

### 它做了什么？

`#[tokio::main]` 是一个属性宏（proc macro），它在编译时将 `main` 函数转换为：

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // 你原来写在 main 里的代码
        some_async_fn().await;
    });
}
```

### 配置选项

```rust
// 单线程运行时（适合简单的或确定性的程序）
#[tokio::main(flavor = "current_thread")]
async fn main() { }

// 多线程运行时（默认）
#[tokio::main(flavor = "multi_thread")]
async fn main() { }

// 指定工作线程数
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() { }
```

### 手动创建运行时

你也可以不依赖宏，手动创建和管理运行时：

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("手动运行时");
    });
}
```

---

## tokio::spawn：并发任务

### 基本使用

```rust
let handle = tokio::spawn(async {
    // 这个代码会在 Tokio 运行时中并发执行
    simulate_work("独立任务", 100).await
});

// 等待任务完成并获取结果
let result = handle.await.unwrap(); // unwrap 处理 JoinError
```

### tokio::join! vs tokio::spawn

这是初学者最容易混淆的两个工具：

| 特性 | `tokio::join!` | `tokio::spawn` |
|------|---------------|---------------|
| **执行方式** | 在当前任务中同时 poll 多个 Future | 将 Future 提交为独立的顶层任务 |
| **并行性** | 所有 Future 在同一任务中，只能在 1 个线程上 | 任务可能在不同线程上并行运行 |
| **所有权** | 需要同时拥有所有 Future | 每个任务独立拥有自己的 Future |
| **取消** | 一个 panic 会导致整体 panic | 一个任务 panic 不影响其他任务 |
| **返回值** | 所有 Future 的结果的元组 | `JoinHandle<T>`，需单独 .await |
| **典型场景** | "我需要同时等待几个结果" | "启动几个独立的长期任务" |

### spawn 的 Send 约束

`tokio::spawn` 要求传入的 Future 实现 `Send` trait，
因为在多线程运行时中，任务可能被调度到任意线程执行。

```rust
// 编译错误：Rc 不是 Send
let x = std::rc::Rc::new(42);
tokio::spawn(async move {
    println!("{}", x);  // 错误！
});

// 解决方案：使用 Arc 代替 Rc
let x = std::sync::Arc::new(42);
tokio::spawn(async move {
    println!("{}", x);  // OK
});
```

---

## CPU 密集型 vs I/O 密集型

### I/O 密集型任务

**特点：** 大部分时间花在等待外部资源（网络、磁盘、数据库等），CPU 大部分时间空闲。

**适合异步：** 是的！异步的核心优势就是在等待 I/O 时可以切换到其他任务。

```rust
// 典型的 I/O 密集型：网络请求、数据库查询、文件读写
async fn handle_request() {
    let db_result = db.query("SELECT ...").await;     // 等待数据库
    let api_result = api_client.fetch_data().await;   // 等待 API
    process(db_result, api_result);
}
```

### CPU 密集型任务

**特点：** 大部分时间花在 CPU 计算上（数学运算、图像处理、加密解密等），线程一直忙碌。

**适合异步：** 不适合！CPU 密集型任务会长时间占用线程，阻塞其他异步任务的调度。

**处理方式：** 使用 `tokio::task::spawn_blocking` 将任务放到专用线程池。

```rust
// 错误：直接在 async 上下文中做 CPU 密集计算
async fn bad_example() {
    let result = expensive_calculation();  // 会阻塞当前线程！
}

// 正确：使用 spawn_blocking
async fn good_example() {
    let result = tokio::task::spawn_blocking(|| {
        expensive_calculation()
    }).await.unwrap();
}
```

`spawn_blocking` 将任务提交到一个独立的线程池（默认最多 512 个线程），
不会影响异步运行时的工作线程。

| 任务类型 | 适合的编程模型 | 在 Rust 中的实现 |
|----------|--------------|-----------------|
| 网络 I/O | 异步 | `tokio::net`、`reqwest`、`hyper` |
| 文件 I/O | 异步（通过阻塞线程池） | `tokio::fs` |
| 数据库查询 | 异步 | `sqlx`、`diesel-async` |
| 数学计算 | 同步 + 线程池 | `spawn_blocking` 或 `rayon` |
| 图像/视频处理 | 同步 + 线程池 | `spawn_blocking` 或专用库 |
| 加密/解密 | 同步 + 线程池 | `spawn_blocking` |

---

## 异步与多线程的关系

### 异步不等于多线程

这是最常见的误解。异步是一种**编程模型**，解决的是"如何组织并发任务"的问题。
多线程是一种**执行策略**，解决的是"如何使用多核 CPU"的问题。

```
异步（编程模型）
    │
    ├── 单线程运行时（flavor = "current_thread"）
    │   所有任务在一个线程上交错执行
    │   适合：嵌入式、简单的 CLI 工具、测试
    │
    └── 多线程运行时（flavor = "multi_thread"）
        任务被分配到多个工作线程并行执行
        适合：Web 服务器、高并发服务
```

### Tokio 的工作窃取模型

Tokio 的多线程运行时使用**工作窃取**（work-stealing）：

1. 每个工作线程有自己的任务队列
2. 线程优先处理自己队列中的任务
3. 当自己的队列为空时，从其他线程的队列中"窃取"任务
4. 这减少了线程间的同步开销，提高了 CPU 缓存局部性

### 竞态条件仍然存在

异步上下文中的并发访问仍然会产生竞态条件（race condition）。
如果多个任务同时修改共享状态，需要使用同步原语：

```rust
use tokio::sync::Mutex;

let shared_data = std::sync::Arc::new(Mutex::new(0));

let handle1 = tokio::spawn({
    let data = shared_data.clone();
    async move {
        let mut guard = data.lock().await;
        *guard += 1;
    }
});
```

注意：在异步上下文中应使用 `tokio::sync::Mutex` 而不是 `std::sync::Mutex`。
前者的 `lock()` 是异步的，不会阻塞线程。

---

## 常见陷阱与最佳实践

### 陷阱 1：忘记 .await

```rust
// 错误：Future 被创建但从未执行
let result = fetch_data(1);  // 返回 Future，但没 .await

// 正确
let result = fetch_data(1).await;
```

编译器通常会发出 `unused_must_use` 警告，但在某些情况下（如将 Future 传递给函数）可能不会警告。

### 陷阱 2：在异步上下文中阻塞

```rust
async fn bad_example() {
    // 严重错误：std::thread::sleep 阻塞了整个操作系统线程！
    // 在 Tokio 多线程运行时中，这会阻塞一个工作线程，
    // 让该线程上的其他异步任务饿死
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 应该使用：
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

### 陷阱 3：过长的 Future 导致编译慢

每个 `async fn` 中的 `.await` 点都会生成一个状态机枚举变体。
如果在一个函数中有过于复杂的异步逻辑，编译时间会增加。

**解决方案：** 将复杂逻辑拆分为多个小的 `async fn`。

### 陷阱 4：Send 约束

```rust
// 错误：Rc 不是 Send，不能被 spawn
let x = std::rc::Rc::new(42);
tokio::spawn(async move {
    println!("{}", *x);
});

// 正确：使用 Arc
let x = std::sync::Arc::new(42);
tokio::spawn(async move {
    println!("{}", *x);
});
```

### 陷阱 5：取消安全（Cancellation Safety）

在 tokio 中，当 `JoinHandle` 被 drop 时，关联的任务会被取消。
被取消的 Future 在顶层 `.await` 处停止执行，但**不会运行析构函数**（drop）。

```rust
async fn read_two_lines() {
    let mut buf1 = String::new();
    let mut buf2 = String::new();

    // 如果整个 Future 在第一行读取后、第二行读取前被取消，buf1 的数据会丢失
    tokio::io::stdin().read_line(&mut buf1).await.unwrap();
    tokio::io::stdin().read_line(&mut buf2).await.unwrap();
    // 这里不会有问题，但如果将两个 read_line 放在 select! 中则要注意
}
```

### 最佳实践总结

1. **I/O 操作用 async，CPU 操作用 spawn_blocking**
2. **不要在 async 代码中调用阻塞函数**
3. **使用 tokio::sync::Mutex 而非 std::sync::Mutex**（在需要跨 .await 持锁时）
4. **将大 async 函数拆分为小函数**，方便编译和测试
5. **注意 spawn 的 Send 约束**，使用 Arc 代替 Rc
6. **为长时间运行的 Future 添加 timeout**：`tokio::time::timeout(duration, future).await`
7. **使用 `#[tokio::test]` 编写异步单元测试**

---

## Python asyncio 对照

如果你有 Python 异步编程的经验，这个对照表会帮助你快速理解 Rust 的异步模型：

| 概念 | Python (asyncio) | Rust (Tokio) |
|------|-----------------|-------------|
| **声明异步函数** | `async def foo():` | `async fn foo()` |
| **等待** | `await foo()` (前缀) | `foo().await` (后缀) |
| **创建任务** | `asyncio.create_task(coro())` | `tokio::spawn(future)` |
| **并发等待** | `asyncio.gather(*tasks)` | `tokio::join!(f1, f2, f3)` |
| **事件循环/运行时** | `asyncio.run()` (内置) | `#[tokio::main]` 或 `Runtime::block_on()` |
| **立即执行 vs 惰性** | coroutine 惰性，task 非惰性 | Future 始终惰性 |
| **睡眠** | `await asyncio.sleep(1)` | `tokio::time::sleep(Duration::from_secs(1)).await` |
| **超时** | `asyncio.wait_for(task, timeout)` | `tokio::time::timeout(duration, future).await` |
| **异步互斥锁** | `asyncio.Lock()` | `tokio::sync::Mutex::new()` |
| **通道** | `asyncio.Queue()` | `tokio::sync::mpsc::channel()` |
| **多线程运行** | 默认单线程，用 `run_in_executor` | 默认多线程（work-stealing） |
| **标准库生态** | `aiohttp`, `aiomysql` (第三方) | `reqwest`, `sqlx`, `tokio` 生态 |

### 关键差异：惰性执行

```python
# Python：协程惰性，但 create_task 后立即执行
import asyncio

async def work():
    print("工作中")
    await asyncio.sleep(1)

async def main():
    coro = work()            # 协程对象，惰性
    task = asyncio.create_task(work())  # 立即调度执行！
    await task
```

```rust
// Rust：Future 始终惰性，spawn 才开始执行
async fn work() {
    println!("工作中");
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    let future = work();       // Future，惰性，什么都不执行
    let handle = tokio::spawn(work()); // 提交到运行时，惰性——还需要 .await 驱动
    handle.await.unwrap();              // 现在才真正执行
}
```

### 关键差异：后缀 .await

```python
# Python：前缀 await
result = await fetch_data()
```

```rust
// Rust：后缀 .await（允许链式调用）
let result = fetch_data().await;
// 链式调用示例：
let result = client.get("https://api.example.com")
    .send().await?
    .json::<MyType>().await?;
```

---

## 运行本章代码

### 前提条件

- Rust 工具链（`rustc`、`cargo`）
- 网络连接（用于下载 Tokio 依赖）

### 编译与运行

```bash
# 进入本章目录
cd chapters/22_async_await_tokio_intro

# 编译（首次会下载依赖，需要一些时间）
cargo build

# 运行
cargo run

# 或者一步到位
cargo run
```

### 预期输出

程序会依次运行 7 个示例，展示：
1. 基本的 async/await 使用
2. 异步上下文中的错误处理
3. 顺序执行与并发执行的时间对比（并发明显更快）
4. tokio::spawn 的使用
5. CPU 密集型任务的处理
6. Future 惰性的演示
7. 综合示例：并发获取多个数据源

### 调整日志级别

```bash
RUST_LOG=info cargo run
# 或
RUST_LOG=tokio=debug cargo run
```

---

## 本章小结

### 核心概念回顾

1. **Future trait** 是 Rust 异步编程的基石——它是一个惰性的、可轮询的状态机
2. **async fn** 是创建 Future 的语法糖，编译器将其转换为状态机
3. **.await** 驱动 Future 执行，在等待时**不阻塞线程**
4. **运行时**负责轮询 Future、管理 I/O 和调度任务
5. **Tokio** 是 Rust 生态最流行的异步运行时，提供多线程工作窃取调度
6. **tokio::join!** 并发等待多个 Future，**tokio::spawn** 提交独立任务

### 心理模型

把异步编程理解为**合作式多任务**（cooperative multitasking）：

- 每个 `.await` 点都是一个"让出点"（yield point）
- 任务在 I/O 等待时主动让出 CPU
- 运行时调度其他就绪的任务
- 没有抢占（preemption），除非使用 `tokio::task::yield_now()`

### 下一步

掌握了本章的基础概念后，你可以继续学习：

- Tokio 的 select! 宏：同时等待多个 Future，处理先完成的那个
- 异步流（Stream）：类似于同步的 Iterator，但每个元素是异步获取的
- Tower / Hyper / Axum：基于 Tokio 的 Web 服务框架
- SeaORM / SQLx：异步数据库操作
- 优雅关闭（graceful shutdown）：如何安全地停止异步服务

---

**异步编程是 Rust 中最具挑战性但也最强大的特性之一。理解 Future 的惰性本质和运行时的角色，是掌握 Rust 异步编程的关键。**
