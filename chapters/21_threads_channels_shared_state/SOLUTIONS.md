# 练习答案 — 线程、信道与共享状态

## 迁移思维练习答案

### 1. Python 的 threading.Lock + 共享数据，在 Rust 中对应什么模式？

Python 中锁和数据是分离的——先创建 lock 对象，再手动在访问数据前后调用 acquire() 和 release()。Rust 的 Mutex<T> 将锁和数据绑定在一起：数据被包裹在 Mutex 内部，你无法不获取锁就访问数据（`mutex.lock()` 返回 MutexGuard，只有通过它才能访问内部数据）。这从类型层面消除了"忘记加锁"这一类错误。多线程共享时，使用 Arc<Mutex<T>> 组合：Arc 提供跨线程的安全共享所有权（引用计数），Mutex 提供线程间互斥的可变访问能力。

**Rust prevents data races but NOT deadlocks**——这是必须理解的关键区别。Mutex 通过类型系统强制你在访问数据前获取锁，从而消除了数据竞争（data race）。但死锁（deadlock）是逻辑层面的问题：如果两个线程按不同顺序获取多个锁，编译器不会也无法检测出潜在的死锁。Arc<Mutex<T>> 是一种组合（composition），不是魔法——它把"共享所有权"和"互斥访问"两个正交的概念组合在一起，各自解决各自的问题，但都不解决死锁。

Send/Sync 语义：Send 表示类型所有权可以安全地在线程间转移（如 i32, String, Arc<T: Send+Sync>），Sync 表示类型的不可变引用可以安全地在线程间共享（如 i32, Mutex<T: Send>）。Rc<T> 不是 Send（非原子引用计数），RefCell<T> 不是 Sync（非线程安全的运行时借用检查）。编译器在类型层面自动检查这些约束——如果你试图把 Rc<T> 传给 thread::spawn，编译器会直接拒绝编译。

### 2. Arc<Mutex<T>> 解决了什么问题，不能解决什么问题？

解决的问题：安全的跨线程共享可变数据——编译器保证你不会忘记加锁、不会在锁外访问数据、不会在 Arc 引用计数归零前释放数据。不能解决的问题：死锁（两个线程互相等待对方持有的锁，编译器不检查锁的获取顺序）、锁竞争导致的性能瓶颈（所有线程排队访问共享数据，失去并发的优势）、读写比例悬殊场景的优化（大量读、少量写时，RwLock 比 Mutex 提供更好的吞吐）。Arc<Mutex<T>> 是常见模式但不是通用答案——优先考虑消息传递（channel）避免共享状态。

### 3. 消息传递（mpsc::channel）相比共享内存有什么优势？

消息传递将"共享数据"转化为"转移数据所有权"——生产者将数据通过 channel 发送后就不再持有，消费者独占数据。这从根本上消除了数据竞争和锁竞争。符合 Rust 的所有权哲学：数据总是有明确的所有者，通过 channel 只是更换了所有者。缺点是数据拷贝可能造成开销（传递大对象时），此时共享内存方案可能更合适。原则是："不要通过共享内存来通信，而是通过通信来共享内存"。

---

## Level 1 练习

### 练习 1.1：多线程求和

**结论**：使用 `thread::spawn` + `move` 闭包 + `join` 可以轻松实现多线程并行计算，结果准确性由 Rust 的所有权系统保证。

**思路**：将数据分成两半，每半移入一个线程的闭包中计算平方和，主线程收集各线程的返回值并汇总。核心是 `move` 关键字——它把数据的**所有权**移入闭包，满足 `thread::spawn` 对 `'static` 生命周期的要求。

**参考实现**：

```rust
use std::thread;

fn main() {
    let numbers = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 分成两半
    let mid = numbers.len() / 2;
    let left = numbers[..mid].to_vec();
    let right = numbers[mid..].to_vec();

    let handle1 = thread::spawn(move || {
        left.iter().map(|x| x * x).sum::<u64>()
    });

    let handle2 = thread::spawn(move || {
        right.iter().map(|x| x * x).sum::<u64>()
    });

    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();

    let total = sum1 + sum2;
    println!("总和: {}", total);
    assert_eq!(total, 385);
}
```

**为什么这样设计**：
- `move` 闭包将 `left` 和 `right` 的所有权分别移入两个线程，避免了数据共享和数据竞争
- `join()` 返回 `Result<T, Box<dyn Any>>`，子线程 panic 时返回 Err
- 每个线程独立计算，互不干扰，不需要任何同步原语

**常见错误**：
1. 忘记 `move` 关键字 → 编译器报错 "closure may outlive the current function"
2. 在 `join` 后使用已被 move 的变量 → 编译错误
3. 试图让两个线程共享同一个 Vec → 需要 Arc 而非直接 move

**验证方式**：运行程序输出 `总和: 385`。手动计算：`1²+2²+...+10² = 385`。

---

### 练习 1.2：简单消息传递

**结论**：`mpsc::channel` 提供了一种"发送即转移所有权"的线程间通信方式。关键在于：必须在主线程中 `drop(原始tx)`，否则 `rx` 的迭代永远不会结束。

**思路**：创建 channel → 克隆 tx 给各线程 → drop 原始 tx → 主线程迭代 rx 接收消息。Channel 天然保证消息发送顺序（FIFO）。

**参考实现**：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    for thread_id in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for msg_num in 0..2 {
                let msg = format!("Thread {} - msg {}", thread_id, msg_num);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    // ★ 关键：drop 原始 tx，否则 rx 永远等待
    drop(tx);

    let mut count = 0;
    for msg in rx {
        println!("收到: {}", msg);
        count += 1;
    }

    println!("总共收到 {} 条消息", count);
    assert_eq!(count, 6);
}
```

**为什么这样设计**：
- `tx.clone()` 创建新的发送端——mpsc 支持多生产者（Multiple Producer）
- `drop(tx)` 是必须的：只要还有一个 Sender 存在，Receiver 就会一直等待。去掉这行代码会导致程序永远阻塞
- `for msg in rx` 遍历直到所有 Sender 被 drop，然后自动结束

**常见错误**：
1. 忘记 `drop(tx)` → 程序永远不退出（死等）
2. 在 drop 之前启动线程 → 没问题，但 drop 必须在所有 spawn 之后
3. 试图 clone rx → 不能！rx 不支持 clone（Single Consumer）

**验证方式**：程序输出 6 条消息，最后打印 "总共收到 6 条消息"。

---

### 练习 1.3：Arc<Mutex<T>> 共享计数器

**结论**：Arc<Mutex<T>> 是多线程共享可变数据的标准模式。Arc 提供线程安全的共享所有权，Mutex 提供互斥访问。**Arc<Mutex<T>> 是一种组合，不是魔法**——它把两个独立的概念组合在一起，各自解决各自的问题。

**思路**：创建 `Arc<Mutex<Vec<String>>>` → 每个线程 clone Arc → lock → push → 自动释放锁（MutexGuard drop）。

**参考实现**：

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let shared_vec = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let vec_clone = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            // lock() 获取 MutexGuard，离开作用域自动释放
            let mut v = vec_clone.lock().unwrap();
            v.push(format!("Thread {} says hello", i));
            // MutexGuard 在此 drop，锁自动释放
            drop(v);
            // 模拟 I/O 操作
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut v = shared_vec.lock().unwrap();
    v.sort(); // 因为线程执行顺序不确定，排序后输出
    println!("总共 {} 条消息:", v.len());
    for msg in v.iter() {
        println!("  {}", msg);
    }
    assert_eq!(v.len(), 5);
}
```

**为什么这样设计**：
- `Arc::clone()` 原子递增引用计数，每个线程拿到一个指向同一数据的 Arc
- `Mutex::lock()` 返回 `MutexGuard`——RAII 机制确保锁一定被释放（即使线程 panic）
- `MutexGuard` 实现了 `DerefMut`，可以直接调用 `v.push()`
- 如果线程持有锁时 panic，Mutex 被"毒化"（poisoned），后续 `lock()` 返回 `Err(PoisonError)`

**常见错误**：
1. 用 Rc 替代 Arc → 编译错误（Rc 不是 Send）
2. 用 RefCell 替代 Mutex → 编译错误（RefCell 不是 Sync）
3. 忘记释放锁（长时间持有）→ 性能问题，所有其他线程被阻塞
4. 在持锁期间执行耗时操作 → 同上

**验证方式**：Vec 中恰好有 5 条消息。

---

## Level 2 练习

### 练习 2.1：并行搜索

**结论**：将数据分块后交给多个线程并行搜索，每个线程通过 mpsc channel 回传结果。这种"消息传递"模式避免了共享状态，更符合 Rust 的并发哲学。

**思路**：生成数据 → 分 4 块 → 每块一个线程 → 每个线程搜到的结果通过 channel 发回 → 主线程收集。

**参考实现**：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 生成 100 个元素，查找包含 "42" 的
    let data: Vec<String> = (0..100)
        .map(|i| format!("item_{}", i))
        .collect();
    let target = "42";

    let (tx, rx) = mpsc::channel();

    let chunk_size = (data.len() + 3) / 4; // 向上取整
    let chunks: Vec<Vec<String>> = (0..4)
        .map(|i| {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(data.len());
            data[start..end].to_vec()
        })
        .collect();

    // 需要存储原始偏移量
    for (chunk_idx, chunk) in chunks.into_iter().enumerate() {
        let tx_clone = tx.clone();
        let offset = chunk_idx * chunk_size;
        let target = target.to_string();
        thread::spawn(move || {
            for (j, item) in chunk.iter().enumerate() {
                if item.contains(&target) {
                    let global_idx = offset + j;
                    tx_clone.send((global_idx, item.clone())).unwrap();
                }
            }
        });
    }

    drop(tx);

    let mut results = vec![];
    for result in rx {
        results.push(result);
    }

    for (idx, item) in &results {
        println!("找到: data[{}] = {}", idx, item);
    }

    // 只应找到包含 "42" 的那一项（item_42，索引 42）
    assert!(!results.is_empty());
    for (idx, item) in &results {
        assert!(item.contains("42"));
        println!("验证通过: 索引 {} 包含 '{}'", idx, target);
    }
}
```

**为什么这样设计**：
- 使用 mpsc channel 而非 Arc<Mutex<Vec>>——因为题目要求使用 channel，这体现了"消息传递"模式
- 分块大小用向上取整避免遗漏元素
- 需要传递全局索引（offset + local_idx），因为每个线程只知道自己在块内的位置
- `tx.clone()` 在每个线程外部完成，保证所有线程都拿到自己的 Sender

**常见错误**：
1. 在 `drop(tx)` 之前就迭代 `rx` → 主线程阻塞
2. 忘记传递偏移量 → 索引错误（输出的是块内索引而非全局索引）
3. 使用 Arc<Mutex<Vec>>（虽然也能工作，但不符合题目要求）

**验证方式**：输出恰好 1 条匹配（item_42），索引为 42。

---

### 练习 2.2：线程安全的缓存

**结论**：通过封装 Arc<Mutex<HashMap<K, V>>> 可以构建出线程安全的缓存数据结构。关键在于设计安全的 API（insert/get/len）并正确处理 Clone。

**思路**：创建 ThreadSafeCache 结构体，内部持有一个 Arc<Mutex<HashMap>>。实现 Clone trait 通过 Arc::clone 共享底层数据。

**参考实现**：

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct ThreadSafeCache {
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl ThreadSafeCache {
    fn new() -> Self {
        ThreadSafeCache {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn insert(&self, key: String, value: String) {
        let mut map = self.inner.lock().unwrap();
        map.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        let map = self.inner.lock().unwrap();
        map.get(key).cloned()
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
}

fn main() {
    let cache = ThreadSafeCache::new();
    let mut handles = vec![];

    for i in 0..10 {
        let cache_clone = cache.clone();
        let handle = thread::spawn(move || {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            cache_clone.insert(key, value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("缓存条目数: {}", cache.len());
    assert_eq!(cache.len(), 10);

    // 验证数据完整性
    for i in 0..10 {
        let key = format!("key_{}", i);
        let expected = format!("value_{}", i);
        let actual = cache.get(&key).unwrap();
        println!("  {} = {}", key, actual);
        assert_eq!(actual, expected);
    }
    println!("数据完整性验证通过！");
}
```

**为什么这样设计**：
- `ThreadSafeCache` 包装了 `Arc<Mutex<HashMap>>`，对外暴露安全的 API
- `#[derive(Clone)]` 或手动 `impl Clone` 通过 `Arc::clone()` 实现——每个线程获得一个缓存"句柄"，但底层是同一份数据
- `get` 方法用 `.cloned()` 返回 String 而非 &str——锁在函数返回时释放，不能返回对锁内数据的引用
- 所有方法签名都是 `&self`（不可变借用），但内部通过 Mutex 实现内部可变性

**常见错误**：
1. `get` 试图返回 `&str` → 生命周期问题（锁释放后引用失效）
2. 忘记 `impl Clone` → 无法在多个线程间共享缓存
3. 在 `insert` 时忘记 `lock()` → 编译错误（不能直接访问 Mutex 内部数据）

**验证方式**：最终缓存中有 10 个条目，每个 key 都能读到正确的 value。

---

## Level 3 练习

### 练习 3.1：并行归并排序

**结论**：归并排序天然适合并行化——分治策略可以将左右两半交给不同线程处理。但对于小数组，线程创建的开销会超过并行收益。

**思路**：基准情况（小数组）用标准库 sort；大于阈值时递归拆分，左半交给新线程，右半当前线程处理，最后合并。

**参考实现**：

```rust
use std::thread;
use std::time::Instant;
use rand::Rng;

/// 合并两个已排序的切片
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

/// 并行归并排序
fn parallel_merge_sort<T: Ord + Send + Clone + 'static>(
    arr: &mut [T],
    threshold: usize,
) {
    if arr.len() <= threshold {
        arr.sort();
        return;
    }

    let mid = arr.len() / 2;
    let (left_slice, right_slice) = arr.split_at_mut(mid);

    // 左半交给新线程
    let mut left_copy = left_slice.to_vec();
    let handle = thread::spawn(move || {
        parallel_merge_sort(&mut left_copy, threshold);
        left_copy
    });

    // 右半在當前線程處理
    let mut right_copy = right_slice.to_vec();
    parallel_merge_sort(&mut right_copy, threshold);

    // 等待左半完成
    let left_sorted = handle.join().unwrap();

    // 合并回原始数组
    let merged = merge(&left_sorted, &right_copy);
    arr.copy_from_slice(&merged);
}

fn main() {
    let mut rng = rand::thread_rng();
    let size = 1_000_000;
    let threshold = 1000;

    // 生成随机数据
    let mut data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1_000_000)).collect();
    let mut data_parallel = data.clone();

    // 标准库排序计时
    let start = Instant::now();
    data.sort();
    let std_duration = start.elapsed();
    println!("标准库 sort(): {:?}", std_duration);

    // 并行排序计时
    let start = Instant::now();
    parallel_merge_sort(&mut data_parallel, threshold);
    let parallel_duration = start.elapsed();
    println!("并行归并排序: {:?}", parallel_duration);

    // 验证正确性
    assert_eq!(data, data_parallel);
    println!("结果一致！");

    println!(
        "加速比: {:.2}x",
        std_duration.as_secs_f64() / parallel_duration.as_secs_f64()
    );
}
```

**为什么这样设计**：
- **阈值设计**：对于长度 <= threshold 的子数组使用标准库 sort，避免为小数组创建线程（线程创建开销 > 计算收益）
- `split_at_mut` 将一个可变切片分成两个互不重叠的可变切片，Rust 的所有权系统保证这两个切片不会互相干扰
- 实际排序在副本上进行（`to_vec()`），避免直接操作可变切片带来的借用冲突
- `merge` 是标准的两路归并，时间复杂度 O(n)

**阈值选择**：通常 1000-10000 比较合理。太小会导致过多线程，太大则并行度不足。需要根据数据大小和 CPU 核心数做权衡。

**扩展思考**：单次递归只创建 1 个线程（2 路并行），在 16 核机器上只能利用约 2-3 个核。更好的方案：在递归的顶层 N 层创建线程（而非每层都创建），或使用线程池。（实际项目中应使用 rayon 库，它针对此问题做了大量优化。）

**常见错误**：
1. 直接对 `split_at_mut` 返回的切片 spawn 线程 → 借用检查器报错（需要将数据移入线程）
2. 阈值设得太低 → 创建过多线程，性能反而下降
3. 忘记 `Send` trait bound → 线程边界需要 Send
4. 递归没有终止条件 → 栈溢出

**验证方式**：并行排序结果与标准库 `sort()` 完全一致。在多核机器上，100万元素应该有 1.5x-3x 加速比。

---

## 思考题

### 1. GIL 的利弊

Python 的 GIL 通过"同一时刻只有一个线程执行 Python 字节码"来避免底层数据竞争，代价是 CPU 密集型任务无法利用多核。如果在 Rust 中加入 GIL：
- Rust 的类型系统优势将被完全浪费——编译器已经能在编译期保证数据竞争不存在，再加运行时锁是双重开销
- Rust 选择了**编译期保证**的 trade-off：用编译器复杂度换取零运行时开销的线程安全
- 这种选择意味着程序员需要付出"学习类型系统"的代价，但获得的是"无 GIL + 编译期检查"的双重收益

### 2. 编译期 vs 运行时

Python 中"不用担心数据竞争"是因为 GIL 在运行时保护了你——但这也意味着你的 CPU 密集型代码永远不会真正并行。Rust 中"编译器帮忙检查"意味着：如果你写错了，代码根本不能编译。这是两种本质不同的体验：
- Python：写起来快，但 bug 在运行时暴露（可能在生产环境）
- Rust：写起来慢（需要和编译器"战斗"），但一旦通过编译，数据竞争的概率几乎为 0

Rust 的模式更能帮助写出正确的并发程序——因为编译期的错误比运行时的 bug 好处理得多。一个 Rust 编译错误花 5 分钟修复，一个生产环境的数据竞争 bug 可能需要花 5 天追踪。

### 3. Rust 的"困难"是否值得

半天改 Rc→Arc、RefCell→Mutex 是值得的：
- 如果 Python 程序中有一个"偶尔出现"的数据竞争（比如多线程操作同一个 dict 而不加锁），追踪和修复可能需要数天甚至数周
- Rust 的这半天是"一次性的学习成本"——理解了 Send/Sync 之后，未来的并发代码编写速度不会比 Python 慢多少
- 更重要的是：Rust 代码编译通过后，你获得了**确定的正确性**，而不是"可能正确"的侥幸

### 4. 并发模型的哲学

"不要通过共享内存来通信，而是通过通信来共享内存"（Do not communicate by sharing memory; instead, share memory by communicating）——这是 Go 和 Rust 社区共享的并发哲学。

这句话的含义：与其让多个线程"共享"一块内存、通过加锁来协调访问（复杂、容易出错），不如让数据通过 channel 从一个线程"传递"到另一个线程。在任意时刻，只有一个线程拥有数据的所有权。所有权转移 = 通信完成。

与 Python 的线程模型的不同：
- Python：依赖 GIL 保护共享数据，多线程共享同一个命名空间
- Rust：通过 channel 在编译期强制执行"同一时刻只有一个所有者"的原则

### 5. 如果 Rust 有 GIL

如果 Rust 有 GIL，Send/Sync trait 仍然有意义，但作用范围会变化：
- **Send 仍然有意义**：GIL 只保护 Python 字节码层面的数据安全，不保护"将数据从一个线程转移到另一个线程"这个语义。在 Rust 中，Send 不仅表示"数据可以跨线程"，更是"该类型的所有权可以从线程 A 移动到线程 B"的语义声明
- **Sync 的价值会减弱但不会消失**：如果有 GIL，RefCell 就不再需要 Sync 的限制（因为同一时刻只有一个线程执行）。但 GIL 不能解决所有并发问题——比如 channel 的实现仍然需要原子操作
- 实际上，Rust 去掉 GIL 正是其并发模型的精髓：**用零成本抽象取代运行时开销**

---

*完成所有练习后，你应该已经掌握了 Rust 并发编程的核心概念：spawn/join、move 闭包、mpsc channel、Arc<Mutex<T>>、Send/Sync trait 体系。记住：Rust prevents data races but NOT deadlocks。*
