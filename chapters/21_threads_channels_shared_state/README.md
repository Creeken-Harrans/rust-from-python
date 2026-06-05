# 第21章：线程、信道与共享状态

## Rust 并发编程核心指南

---

## 目录

1. [概述](#概述)
2. [线程（Threads）](#线程threads)
3. [消息传递（Message Passing）](#消息传递message-passing)
4. [共享状态（Shared State）](#共享状态shared-state)
5. [Send 与 Sync Trait](#send-与-sync-trait)
6. [数据竞争（Data Race）](#数据竞争data-race)
7. [死锁（Deadlock）](#死锁deadlock)
8. [对比表格](#对比表格)
9. [类型系统中的线程安全](#类型系统中的线程安全)
10. [Fearless Concurrency](#fearless-concurrency)
11. [性能：线程 vs 异步](#性能线程-vs-异步)
12. [Python 对照表](#python-对照表)
13. [核心术语速查](#核心术语速查)
14. [运行与测试](#运行与测试)
15. [进阶阅读](#进阶阅读)

---

## 概述

并发编程是现代软件开发的核心技能之一。多核 CPU 的普及使得真正并行的程序能够显著提升性能。
然而，并发编程也带来了巨大的复杂性：数据竞争、死锁、竞态条件是每个并发程序员都必须面对的难题。

Rust 的并发编程哲学与其他主流语言有根本性的不同：

- **C/C++**：程序员负责所有同步，编译器很少帮忙，错误在运行时表现为难以调试的 bug。
- **Python/Java/Go**：运行时（GIL、GC、goroutine 调度器）帮助管理并发，但数据竞争仍然是运行时的可能性。
- **Rust**：编译器在**编译期**就阻止了数据竞争。类型系统（特别是所有权模型 + Send/Sync trait）
  使得"如果代码通过编译，就不会有数据竞争"成为可能。

本章将系统介绍 Rust 并发编程的四大基石：

1. **线程**：`std::thread::spawn` 与 `join`
2. **消息传递**：`std::sync::mpsc::channel`
3. **共享状态**：`Arc<Mutex<T>>`
4. **类型级保证**：`Send` 与 `Sync` trait

---

## 线程（Threads）

### 创建线程：spawn

`std::thread::spawn` 接收一个闭包，立即在一个新的操作系统线程中执行该闭包：

```rust
use std::thread;

let handle = thread::spawn(|| {
    // 这段代码在新线程中运行
    println!("来自子线程的问候！");
});

// 主线程继续执行
println!("来自主线程的问候！");

// 等待子线程结束
handle.join().unwrap();
```

**关键点**：
- `spawn` 返回 `JoinHandle<T>`，其中 `T` 是闭包的返回值类型
- `JoinHandle` 的 `join()` 方法阻塞当前线程，等待子线程完成
- 如果子线程发生了 `panic`，`join()` 返回 `Err(Box<dyn Any>)`

### 等待线程：join

`join()` 是同步点：调用它的线程会被阻塞，直到对应的子线程执行完毕。

```rust
let handle1 = thread::spawn(|| { /* 工作 A */ });
let handle2 = thread::spawn(|| { /* 工作 B */ });

// 按顺序等待（注意：A 和 B 可能已经同时跑完了）
handle1.join().unwrap();
handle2.join().unwrap();
```

**常见的错误：忘记 join**  
如果程序在 `join` 之前就结束了（比如 `main` 函数返回），子线程会被强制终止。
这意味着子线程的工作可能没有完全完成。一定要在所有线程上调用 `join()`。

### move 闭包

这是 Rust 初学者最容易困惑的地方之一。

```rust
let message = String::from("hello");

// 错误！缺少 move 关键字
// thread::spawn(|| {
//     println!("{}", message);  // 编译错误！
// });
//
// 错误信息：closure may outlive the current function,
// but it borrows `message`, which is owned by the current function

// 正确：使用 move 把所有权移入闭包
thread::spawn(move || {
    println!("{}", message);  // 编译通过！
});
```

**为什么会这样？**

`thread::spawn` 的闭包必须满足 `'static` 生命周期约束——因为线程可能比创建它的函数活得更久。
如果闭包只是借用局部变量，当函数返回后局部变量被释放，线程中的借用就变成了悬垂指针（dangling pointer）。

`move` 关键字告诉编译器：**把闭包捕获的变量的所有权移入闭包**。这样，闭包就不再依赖原函数的作用域了。

**Copy 类型的特殊情况：**

对于 `i32`、`bool` 等实现了 `Copy` trait 的类型，即使不加 `move`，编译器也会自动复制值而非借用。
但推荐显式加上 `move`，使意图更加清晰：

```rust
let x = 42;
thread::spawn(move || {
    println!("{}", x);  // x 被复制（因为 i32 是 Copy）
});
// x 仍然可以使用！因为闭包拿到的是一份拷贝
println!("{}", x);  // 完全没问题
```

---

## 消息传递（Message Passing）

> "不要通过共享内存来通信，而是通过通信来共享内存。"
> —— Effective Go（同样适用于 Rust）

Rust 标准库提供了 **mpsc（Multiple Producer, Single Consumer）** 信道：

- **多个**线程可以通过 `Sender` 发送消息
- **一个**线程通过 `Receiver` 接收消息
- 信道保证消息的 FIFO 顺序

### 基本用法

```rust
use std::sync::mpsc;
use std::thread;

// 创建信道
let (tx, rx) = mpsc::channel();

// 启动一个生产者线程
thread::spawn(move || {
    tx.send(String::from("你好")).unwrap();
    tx.send(String::from("世界")).unwrap();
    // tx 在此被 drop，信道关闭
});

// 主线程接收消息
for msg in rx {
    println!("收到: {}", msg);
}
// 输出：
// 收到: 你好
// 收到: 世界
```

### 多生产者

`Sender` 实现了 `Clone`，可以克隆出多个发送端：

```rust
let (tx, rx) = mpsc::channel();

for i in 0..5 {
    let tx_clone = tx.clone();  // 每个线程一份
    thread::spawn(move || {
        tx_clone.send(format!("来自线程 #{}", i)).unwrap();
    });
}

// ★ 重要：drop 原始的 tx，否则信道永远不会关闭
drop(tx);

for msg in rx {
    println!("{}", msg);
}
```

**关键细节**：
- `tx` 必须在主线程中被 `drop`，因为 `rx` 的迭代器会一直等待，直到**所有** `Sender` 都被释放
- 如果忘记 `drop(tx)`，`for msg in rx` 会永远阻塞（因为还有一个 `Sender` 存在）
- 在多个 `Sender` 的场景下，这意味着原来的 `tx` 依然存在

### 发送与接收方法

| 方法 | 行为 |
|------|------|
| `tx.send(value)` | 发送值，如果接收端已关闭则返回 `Err(value)` |
| `rx.recv()` | 阻塞等待一条消息，信道关闭时返回 `Err` |
| `rx.try_recv()` | 非阻塞尝试接收，无消息时返回 `Err(TryRecvError::Empty)` |
| `rx.recv_timeout(dur)` | 阻塞等待带超时，超时返回 `Err(RecvTimeoutError::Timeout)` |

### 为什么选择消息传递？

1. **单一所有权思路的自然延伸**：每个数据只有一个所有者，通过发送转移所有权
2. **减少了锁的需要**：接收端天然串行处理消息，不需要 Mutex
3. **清晰的通信拓扑**：生产者-消费者模式天然适合很多并发问题

---

## 共享状态（Shared State）

当多个线程需要访问同一份可变数据时，消息传递变得不那么方便。这时我们需要共享状态。

### Mutex<T> —— 互斥锁

`Mutex<T>`（Mutual Exclusion，互斥）保证同一时刻只有一个线程能够访问内部数据：

```rust
use std::sync::Mutex;

let m = Mutex::new(42);

{
    let mut num = m.lock().unwrap();  // 获取锁
    *num += 1;                        // 修改数据
}  // 锁在此自动释放（MutexGuard 被 Drop）

println!("m = {:?}", m);  // m = Mutex { data: 43, .. }
```

**lock() 方法详解**：
- `lock()` 阻塞当前线程直到获取到锁
- 返回 `LockResult<MutexGuard<T>>`
- `Result` 包了一层是因为：如果持有锁的线程 `panic` 了，锁会被"毒化"（poisoned）
- `MutexGuard<T>` 实现了 `Deref` 和 `DerefMut`，可以当作 `&T`/`&mut T` 使用
- `MutexGuard` 离开作用域时自动调用 `drop()` 释放锁——**不需要手动 unlock**

**中毒（Poisoning）机制**：

```rust
let m = Mutex::new(42);

let handle = thread::spawn(move || {
    let _guard = m.lock().unwrap();
    panic!("持有锁的线程崩溃了！");
    // MutexGuard 的 Drop 会被调用，锁被释放
    // 但 Mutex 被标记为 "poisoned"
});

handle.join().unwrap_err();  // 捕获 panic

// 尝试获取毒化的锁
match m.lock() {
    Ok(guard) => println!("锁正常: {}", *guard),
    Err(poisoned) => {
        // 可以通过 into_inner() 或 get_mut() 恢复数据
        let mut guard = poisoned.into_inner();
        *guard = 0;
        println!("锁已毒化，数据已重置为 {}", *guard);
    }
}
```

### Arc<T> —— 原子引用计数

单线程中，我们使用 `Rc<T>` 实现多所有权的共享数据。但 `Rc<T>` 不是线程安全的——它的引用计数操作不是原子的。

多线程中，我们使用 `Arc<T>`（Atomic Reference Count）：

```rust
use std::sync::Arc;
use std::thread;

let shared = Arc::new(vec![1, 2, 3]);

let mut handles = vec![];
for _ in 0..3 {
    let clone = Arc::clone(&shared);  // 原子地增加引用计数
    handles.push(thread::spawn(move || {
        println!("共享数据: {:?}", *clone);
    }));
}

for h in handles {
    h.join().unwrap();
}

// Arc::try_unwrap 在引用计数为 1 时可以取出内部数据
let data = Arc::try_unwrap(shared).unwrap();
println!("取出数据: {:?}", data);
```

### Arc<Mutex<T>> —— 多线程共享可变数据的标准模式

将 Arc 和 Mutex 组合起来，就得到了多线程共享可变数据的最常用模式：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("最终计数: {}", *counter.lock().unwrap());
// 输出: 最终计数: 10000
```

**为什么需要 Arc？**
- 需要在多个线程之间共享同一份数据的所有权
- 线程的生命周期各不相同，编译器无法确定哪个线程最后使用数据
- Arc 自动管理：当最后一个线程结束时，数据自动释放

**为什么需要 Mutex？**
- 如果多个线程不加保护地同时写同一个 `i32`，会发生数据竞争
- 两个线程可能读到相同的旧值，各自加 1 后写回，导致只增加了 1 而非 2
- Mutex 保证 `读-改-写` 操作的原子性

---

## Send 与 Sync Trait

Rust 通过两个 **marker trait**（无方法的 trait，仅作为编译期标记）在类型系统中编码线程安全规则：

### Send

```rust
// std::marker::Send 的定义（概念上）
pub unsafe auto trait Send {}
```

**含义**：实现了 `Send` 的类型的值的**所有权**可以在线程之间转移。

- 几乎所有的 Rust 类型都实现了 `Send`
- 主要例外：`Rc<T>`（非原子引用计数）、裸指针
- 编译器自动为结构体和枚举推导 `Send`：当且仅当所有字段都是 `Send`

### Sync

```rust
// std::marker::Sync 的定义（概念上）
pub unsafe auto trait Sync {}
```

**含义**：实现了 `Sync` 的类型的**不可变引用**可以在线程之间安全共享。

- `T: Sync` 当且仅当 `&T: Send`
- 即：如果 `&T` 可以安全地发送到另一个线程，那么 `T` 就是 `Sync`
- 主要例外：`RefCell<T>`（运行时借用检查不是线程安全的）、`Cell<T>`

### 常见类型的 Send/Sync 状态

| 类型 | Send | Sync | 说明 |
|------|------|------|------|
| `i32`, `f64`, `bool` 等基本类型 | ✓ | ✓ | 所有基本类型都是 Send + Sync |
| `String`, `Vec<T>` | ✓ (T: Send) | ✓ (T: Sync) | 条件取决于泛型参数 |
| `&T` | ✓ (T: Sync) | ✓ (T: Sync) | 引用本身是 Send+Sync 的 |
| `&mut T` | ✓ (T: Send) | ✗ | 可变引用不是 Sync 的（独占性） |
| `Box<T>` | ✓ (T: Send) | ✓ (T: Sync) | 与内部类型一致 |
| `Rc<T>` | ✗ | ✗ | **不能跨线程！** |
| `Arc<T>` | ✓ (T: Send+Sync) | ✓ (T: Send+Sync) | Rc 的线程安全版本 |
| `RefCell<T>` | ✓ (T: Send) | ✗ | 不是 Sync（运行时借用检查） |
| `Mutex<T>` | ✓ (T: Send) | ✓ (T: Send) | RefCell 的线程安全版本 |
| `*const T`, `*mut T` | ✗ | ✗ | 裸指针没有安全保障 |

### 自动推导

```rust
struct MyStruct {
    x: i32,       // Send + Sync
    y: String,    // Send + Sync
    // z: Rc<i32>,  // 如果加入这一行，MyStruct 就不再是 Send + Sync
}
// MyStruct 自动实现 Send + Sync

// 验证
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
assert_send::<MyStruct>();
assert_sync::<MyStruct>();
```

### 实际影响

当你在代码中使用 `thread::spawn` 时，编译器会检查闭包捕获的所有类型是否满足 `Send`：

```rust
use std::rc::Rc;
use std::thread;

let rc = Rc::new(42);
// thread::spawn(move || {
//     println!("{}", rc);  // 编译错误！
// });
//
// 错误信息：
// error[E0277]: `Rc<i32>` cannot be sent between threads safely
//    = help: the trait `Send` is not implemented for `Rc<i32>`
//    = note: use `Arc<i32>` instead
```

**编译器不仅告诉你错了，还告诉你怎么改！** 这就是 Rust 的编译期线程安全保证。

---

## 数据竞争（Data Race）

### 什么是数据竞争？

数据竞争是指满足以下**三个条件**的并发访问：

1. **两个或更多**线程同时访问同一块内存
2. **至少一个**线程在写入
3. 没有任何同步机制来协调访问顺序

数据竞争是**未定义行为**（Undefined Behavior），可能导致：
- 读到被部分修改的值（tearing）
- 程序崩溃
- 安全漏洞
- 编译器做出错误优化假设

### Rust 如何防止数据竞争？

Rust 在**编译期**通过所有权系统和类型系统阻止数据竞争：

```
所有权规则                          并发推论
─────────────────────              ─────────────────────
每个值有唯一所有者      →          同一时刻只能有一个线程拥有值的所有权

不可变引用可以有多个    →          &T: Sync 时可以安全地多线程共享读

可变引用只能有一个      →          通过 Mutex 将可变引用限制在一个线程中

引用不能超过值的生命周期 →         通过 'static 约束防止悬垂指针
```

**实际效果**：在 safe Rust 中，数据竞争不可能发生。这不是"不容易发生"，而是**编译器保证了它不会发生**。

### 数据竞争 vs 竞态条件

这两个概念常常被混淆，但它们有本质区别：

| | 数据竞争 (Data Race) | 竞态条件 (Race Condition) |
|---|---|---|
| **定义** | 无同步的并发读写冲突 | 线程调度顺序导致的逻辑不确定性 |
| **内存安全影响** | 未定义行为，可能崩溃 | 仅影响程序逻辑，内存仍然安全 |
| **Rust 检测** | **编译期阻止** ✓ | 编译器无法检测 |
| **例子** | 两个线程同时 `+=` 一个 i32 | 两个线程同时向文件写入，顺序不定 |
| **解决方法** | Mutex, Atomics, Channel | 仔细设计协议、使用事务、accept 任何合法顺序 |

**记住**：Rust 消除了数据竞争，但没有（也不能）消除竞态条件。逻辑正确性仍然需要程序员的判断。

---

## 死锁（Deadlock）

### 死锁的定义

死锁是指两个或多个线程**互相等待**对方释放资源，导致所有线程**永久阻塞**的状态。

### 死锁的四个必要条件（Coffman 条件）

| 条件 | 含义 |
|------|------|
| 1. 互斥 (Mutual Exclusion) | 资源一次只能被一个线程使用 |
| 2. 持有并等待 (Hold and Wait) | 线程持有资源的同时等待其他资源 |
| 3. 不可抢占 (No Preemption) | 资源只能由持有者自愿释放 |
| 4. 循环等待 (Circular Wait) | 线程之间形成等待环 |

**四个条件缺一不可。** 要防止死锁，只需打破其中任何一个条件。

### 死锁示例

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let m1 = Arc::new(Mutex::new(0));
let m2 = Arc::new(Mutex::new(0));

let m1_a = Arc::clone(&m1);
let m2_a = Arc::clone(&m2);

// 线程 A：先锁 m1 再锁 m2
let handle_a = thread::spawn(move || {
    let _g1 = m1_a.lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(50));
    let _g2 = m2_a.lock().unwrap();  // 等待 m2
    // ... 实际代码中可能永远到不了这里
});

let m1_b = Arc::clone(&m1);
let m2_b = Arc::clone(&m2);

// 线程 B：先锁 m2 再锁 m1 ← 顺序不同！
let handle_b = thread::spawn(move || {
    let _g2 = m2_b.lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(50));
    let _g1 = m1_b.lock().unwrap();  // 等待 m1
    // ... 这里也到不了
});

// A 持有 m1 等待 m2，B 持有 m2 等待 m1
// → 死锁！
```

### 避免死锁的策略

1. **锁排序（Lock Ordering）**：始终按相同的顺序获取多个锁
2. **使用 try_lock()**：不阻塞，失败时释放已持有的锁并重试
3. **减少锁的持有时间**：尽快释放锁，缩小临界区
4. **使用更高级的同步原语**：如 channel、actor 模型
5. **避免嵌套锁**：尽量不要在持有一个锁的情况下尝试获取另一个锁

```rust
// 解决上面死锁的方法：统一锁的获取顺序
// 线程 A 和 线程 B 都先锁 m1 再锁 m2
let _g1 = m1.lock().unwrap();
let _g2 = m2.lock().unwrap();  // 安全！因为顺序一致
```

### Rust 中的死锁

- **Rust 的类型系统不能防止死锁**（死锁是运行时行为，而非类型错误）
- 但 Rust 的 `MutexGuard` RAII 机制确保：即使发生死锁，锁也不会"泄漏"——如果线程被强制终止（在 unsafe 或 FFI 场景），锁的状态可能不完整
- `std::sync::Mutex` 在 Linux 上基于 `pthread_mutex_t`，在 Windows 上基于 `SRWLOCK`
- 大多数 Rust `Mutex` 实现**不可重入**——同一线程不能重复 lock 已经持有的锁（会导致死锁或 panic）

---

## 对比表格

### 单线程 vs 多线程：共享所有权与内部可变性

| 概念 | 单线程方案 | 多线程方案 | 说明 |
|------|-----------|-----------|------|
| 单一所有权 | `let x = T;` | `let x = T;` | 基础所有权规则不变 |
| 共享所有权 | `Rc<T>` | `Arc<T>` | Arc 使用原子引用计数 |
| 内部可变性 | `RefCell<T>` | `Mutex<T>` | Mutex 使用系统级锁 |
| 读写锁 | `Cell<T>` / `RefCell<T>` | `RwLock<T>` | 多读单写模式 |
| 无锁原语 | — | `AtomicI32`, `AtomicBool` 等 | 硬件支持的原子操作 |

### 组合模式对比

| 组合 | 适用场景 | 线程数 |
|------|---------|--------|
| `Rc<RefCell<T>>` | 单线程多所有者共享可变数据 | 1 |
| `Arc<Mutex<T>>` | 多线程共享可变数据 | N |
| `Arc<RwLock<T>>` | 多线程读多写少共享数据 | N |
| `Arc<AtomicI32>` | 多线程共享简单计数器 | N |

### 典型开销对比（数量级参考）

| 操作 | 相对开销 |
|------|---------|
| 无锁读写 | 1x（基线） |
| `Rc::clone` | ~1x（非原子 inc） |
| `Arc::clone` | ~10x（原子 inc + 内存屏障） |
| `RefCell::borrow` | ~2-3x（运行时检查） |
| `Mutex::lock`（无竞争） | ~25x（系统调用或 futex） |
| `Mutex::lock`（有竞争） | ~1000x+（上下文切换） |

---

## 类型系统中的线程安全

Rust 的最大创新之一是将**线程安全编码为类型约束**，使得并发错误在编译时就被发现。

### 编译器如何检查

当你写 `thread::spawn(closure)` 时，编译器大致做以下检查：

```
1. closure: FnOnce() -> T        ✓ 闭包可以调用一次
2. closure: Send                 ✓ 闭包本身可跨线程发送
3. closure: 'static              ✓ 闭包不借用局部数据
4. T: Send                       ✓ 返回值可跨线程取回
```

任何一步不满足 → **编译错误**，而不是运行时 panic。

### 错误信息示例

```
error[E0277]: `Rc<i32>` cannot be sent between threads safely
  --> src/main.rs:10:5
   |
10 |     thread::spawn(move || {
   |     ^^^^^^^^^^^^^ `Rc<i32>` cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:10:19: 12:6]`,
           the trait `Send` is not implemented for `Rc<i32>`
   = note: required because it appears within the type `[closure@...]`
   = note: required by a bound in `spawn`
note: required by a bound in `spawn`
  --> /rustc/.../library/std/src/thread/mod.rs:691:8
   |
   |     F: Send + 'static,
   |        ^^^^ required by this bound in `spawn`
```

Rust 的错误信息告诉你：
1. **哪个类型**不满足要求（`Rc<i32>`）
2. **哪个 trait** 缺失（`Send`）
3. **调用链**是如何到达约束的（`spawn → closure → Rc<i32>`）
4. **建议**用什么替代（编译器常建议用 `Arc` 替代 `Rc`）

### 为什么这很重要？

在传统语言中，数据竞争是最难调试的 bug 之一：
- 它可能只在高负载下出现
- 它可能只在特定 CPU 架构上出现
- 调试器可能会改变时序，使 bug 消失（Heisenbug）
- 安全漏洞往往源于数据竞争（如 TOCTOU）

Rust 让这些 bug 在代码进入生产环境之前就变成了编译错误。

---

## Fearless Concurrency

"Fearless Concurrency"（无畏并发）是 Rust 社区的一个口号，意思是：

> 使用 Rust，你可以大胆地编写并发代码，因为编译器会为你兜底。

### 这个保证的范围和限度

**编译器保证**：
- 没有数据竞争（编译期保证）
- 没有悬垂引用（所有权/生命周期保证）
- 没有 use-after-free（所有权保证）

**编译器不保证**：
- 没有逻辑竞态条件（线程调度的不确定性）
- 没有死锁（运行时行为）
- 没有性能问题（锁争用、虚假共享等）
- 没有活锁（Livelock）

### "无畏"的实践意义

```rust
// 你可以在不担心数据竞争的情况下写这样的代码：
let data = Arc::new(Mutex::new(HashMap::new()));

for i in 0..100 {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut map = data.lock().unwrap();
        map.insert(i, compute_value(i));  // 完全安全！
    });
}

// 如果某处少了 Mutex 或用了 Rc 而非 Arc，
// 编译器会在你运行程序之前就告诉你。
```

### 零成本抽象

Rust 的并发原语遵循零成本抽象原则：

- **不用就不开销**：不使用 Arc 的代码不会为原子操作付出代价
- **自动化就是手动**：`MutexGuard` 的 RAII 在优化后与手写的 lock/unlock 生成相同的机器码
- **无运行时**：Rust 没有垃圾回收、没有 GIL、没有内置调度器——线程就是操作系统的线程

---

## 性能：线程 vs 异步

本章介绍了操作系统线程。Rust 还有一个重要的并发模型：**异步编程**（async/await）。

| 维度 | 操作系统线程 | 异步任务 |
|------|-------------|---------|
| **调度者** | OS 内核 | 用户态运行时（tokio 等） |
| **创建开销** | 较大（~几微秒到几十微秒） | 极小（~几十纳秒） |
| **上下文切换** | 系统调用，昂贵 | 用户态，便宜 |
| **内存占用** | 每个线程有独立栈（~2MB+） | 每个任务只需少量内存 |
| **适用场景** | CPU 密集型、少量连接 | I/O 密集型、海量连接 |
| **编程模型** | 同步式，易理解 | 异步式，有学习曲线 |

### 什么时候用线程？

- CPU 密集型任务（数学计算、图像处理、编译）
- 不需要大量并发连接（几十到几百个线程）
- 调用阻塞的系统 API（文件 I/O 等）
- 需要与 C 库交互（FFI 中的阻塞调用）

### 什么时候用异步？

- I/O 密集型任务（网络服务器、代理、数据库代理）
- 需要大量并发连接（成千上万）
- 需要精细控制任务调度
- 需要取消操作

> **下一章预告**：第22章将详细介绍 Rust 的异步编程模型——Future、async/await、以及 tokio 运行时。

---

## Python 对照表

如果你是 Python 程序员，下面这个对照表能帮助你快速理解 Rust 的并发模型：

| 概念 | Python | Rust |
|------|--------|------|
| **创建线程** | `threading.Thread(target=f).start()` | `thread::spawn(f)` |
| **等待线程** | `thread.join()` | `handle.join()` |
| **线程返回值** | 需要手动管理（Queue/dict） | `JoinHandle<T>`，`join()` 返回 `T` |
| **消息队列** | `queue.Queue` | `mpsc::channel` |
| **互斥锁** | `threading.Lock` | `std::sync::Mutex` |
| **获取锁** | `with lock:` | `let g = mutex.lock().unwrap()` |
| **释放锁** | 离开 `with` 块 | MutexGuard 离开作用域 |
| **引用计数共享** | 自动（GC 管理） | `Rc<T>`（单线程）/ `Arc<T>`（多线程） |
| **GIL** | 有（CPython） | **无** |
| **数据竞争** | 运行时报错或静默错误 | **编译期错误** |
| **线程数量建议** | 受 GIL 限制，多用于 I/O | 真正并行，CPU 密集型也有效 |

### GIL 的影响

**Python（CPython）**：
- GIL（Global Interpreter Lock）确保同一时刻只有一个线程执行 Python 字节码
- 多线程对于 CPU 密集型任务**不能加速**（甚至可能变慢）
- 多线程主要用于 I/O 密集型任务
- 要利用多核 CPU 的并行能力，需要使用 `multiprocessing` 模块

**Rust**：
- **没有 GIL**
- 多个线程可以在不同的 CPU 核心上真正并行执行
- CPU 密集型任务可以用线程直接加速
- 编译期保证内存安全，不需要运行时锁来解释器状态

---

## 核心术语速查

| 中文 | English | 说明 |
|------|---------|------|
| 线程 | Thread | 操作系统调度的最小执行单元 |
| 产生/派生 | spawn | 创建新线程 |
| 汇合/等待 | join | 等待线程完成 |
| 消息传递 | Message Passing | 通过信道在线程间发送数据 |
| 信道 | Channel | 线程间通信的管道 |
| 多生产者单消费者 | mpsc | Multiple Producer, Single Consumer |
| 互斥锁 | Mutex | Mutual Exclusion，保证互斥访问 |
| 原子引用计数 | Arc | Atomic Reference Count |
| 发送（trait） | Send | 标记类型可在线程间转移所有权 |
| 同步（trait） | Sync | 标记类型的引用可在线程间共享 |
| 死锁 | Deadlock | 线程互相等待导致的永久阻塞 |
| 数据竞争 | Data Race | 无同步的并发读写冲突 |
| 竞态条件 | Race Condition | 调度顺序导致的结果不确定性 |
| 毒化 | Poisoning | Mutex 在 panic 后的保护状态 |
| 无畏并发 | Fearless Concurrency | Rust 编译期保证无数据竞争 |
| 互斥守卫 | MutexGuard | lock() 返回的智能指针，自动释放锁 |
| 原子操作 | Atomic Operation | 不可分割的操作，硬件保证 |
| 活锁 | Livelock | 线程不断改变状态但无法推进 |

---

## 运行与测试

### 编译并运行

```bash
# 编译（debug 模式）
cargo build

# 运行
cargo run

# 使用 release 模式优化（性能更好，适合基准测试）
cargo run --release
```

### 检查编译期保证

尝试注释掉示例代码中的 `move` 关键字或把 `Arc` 换成 `Rc`，观察编译器的错误信息：

```bash
# 预期看到类似这样的错误：
# error[E0277]: `Rc<i32>` cannot be sent between threads safely
```

### 使用 Clippy 检查代码质量

```bash
cargo clippy
```

### 运行测试（如果有）

```bash
cargo test
```

---

## 进阶阅读

### 官方资源
- [The Rust Programming Language - Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [The Rustonomicon - Concurrency](https://doc.rust-lang.org/nomicon/concurrency.html)
- [std::thread 模块文档](https://doc.rust-lang.org/std/thread/)
- [std::sync 模块文档](https://doc.rust-lang.org/std/sync/)

### 深入主题（后续章节或自学）
- `AtomicI32` / `AtomicBool` 等无锁原子类型
- `RwLock<T>` 读写锁
- `Barrier` 线程屏障
- `Condvar` 条件变量
- `thread_local!` 线程局部存储
- `park` / `unpark` 线程的低级阻塞控制
- `rayon` 库：数据并行（并行迭代器）
- `crossbeam` 库：增强的信道实现和作用域线程
- `tokio` 运行时：异步 I/O（第22章）
- `async-std`：类似标准库的异步替代

### 推荐书籍
- *Rust for Rustaceans* by Jon Gjengset — 第8章深入讨论并发
- *Programming Rust* by Jim Blandy, Jason Orendorff — 第19章
- *Rust Atomics and Locks* by Mara Bos — 全面深入的低级并发

---

*本章代码位于 `src/main.rs`，可直接通过 `cargo run` 运行，观察所有概念的实时演示。*
