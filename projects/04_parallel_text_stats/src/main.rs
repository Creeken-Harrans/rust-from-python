#![allow(clippy::unnecessary_sort_by)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// ---------------------------------------------------------------------------
// 数据结构
// ---------------------------------------------------------------------------

/// 单个文本文件的统计结果
///
/// 包含文件路径、字符数、单词数、行数以及出现频率最高的前5个单词。
/// 派生 `Debug` 方便打印，派生 `Clone` 方便在线程间传递。
#[derive(Debug, Clone)]
struct FileStats {
    /// 文本来源标识（文件名或样本编号）
    path: String,
    /// 字符总数（含空格与换行符）
    chars: usize,
    /// 单词总数（以空白字符分割）
    words: usize,
    /// 行数（以 '\n' 分割）
    lines: usize,
    /// 出现频率最高的前5个单词，按频率降序排列
    top_words: Vec<(String, usize)>,
}

// ---------------------------------------------------------------------------
// 核心统计函数
// ---------------------------------------------------------------------------

/// 对一段文本内容进行统计，返回 `FileStats`
///
/// # 算法说明
/// - **字符数**: 直接取 `content.chars().count()`
/// - **单词数**: 按空白字符 split 后过滤空串
/// - **行数**: 按 '\n' split 后计数（最后一行若为空也计为一行）
/// - **高频词**: 转为小写，去除首尾标点，用 HashMap 计数，取前5
fn count_stats(path: &str, content: &str) -> FileStats {
    let chars = content.chars().count();
    let words: Vec<&str> = content.split_whitespace().collect();
    let word_count = words.len();
    let lines = content.lines().count();

    // 统计词频 —— 统一转小写并保留字母数字字符
    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in &words {
        let cleaned: String = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if !cleaned.is_empty() {
            *freq.entry(cleaned).or_insert(0) += 1;
        }
    }

    // 取出 top 5
    let mut sorted: Vec<(String, usize)> = freq.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(5);

    FileStats {
        path: path.to_string(),
        chars,
        words: word_count,
        lines,
        top_words: sorted,
    }
}

// ---------------------------------------------------------------------------
// 线程任务：负责统计一段文本并通过 channel 发回结果
// ---------------------------------------------------------------------------

/// 线程的工作函数
///
/// # 线程安全设计
/// - 该函数接收的所有数据（name, text）均为 `String`，拥有完全所有权。
/// - `move` 关键字将数据所有权转移到新线程内部，避免悬垂引用（dangling reference）。
/// - 结果通过 `mpsc::Sender` 发送回主线程 —— 这是"消息传递"模式，
///   线程间不共享内存，从根本上消除了数据竞争（data race）。
fn worker(name: String, text: String, tx: mpsc::Sender<Result<FileStats, String>>) {
    // 如果文本为空（仅含空白），视为错误
    if text.trim().is_empty() {
        let _ = tx.send(Err(format!("[{name}]: 文本为空，无法统计")));
        return;
    }
    let stats = count_stats(&name, &text);
    let _ = tx.send(Ok(stats));
}

// ---------------------------------------------------------------------------
// 方案A：消息传递（Message Passing）
//
// 每个线程计算出 FileStats 后通过 mpsc channel 发送给主线程。
// 这是 Rust 并发模型的推荐模式 —— "Do not communicate by sharing memory;
//  instead, share memory by communicating."
//
// # 线程安全设计
// - mpsc 代表 "multiple producer, single consumer"：多个线程可以向同一个
//   通道发送数据，但只有一个接收端（主线程）。
// - tx.clone() 为每个线程创建一个 sender 副本，实现多生产者。
// - drop(tx) 确保所有 sender 被丢弃后接收端能正常结束。
// ---------------------------------------------------------------------------

fn run_message_passing(samples: &[(String, String)]) {
    println!("===== 方案A：消息传递（mpsc channel）=====\n");

    let (tx, rx) = mpsc::channel::<Result<FileStats, String>>();
    let sample_count = samples.len();

    let start = Instant::now();

    // 为每段文本创建一个线程
    for (name, text) in samples {
        let tx_clone = tx.clone();
        let n = name.clone();
        let t = text.clone();
        thread::spawn(move || {
            worker(n, t, tx_clone);
        });
    }
    // 丢弃原始 sender —— 否则 rx 会永远等待
    drop(tx);

    // 收集结果
    let mut total_chars = 0usize;
    let mut total_words = 0usize;
    let mut total_lines = 0usize;
    let mut errors: Vec<String> = Vec::new();
    let mut all_stats: Vec<FileStats> = Vec::new();

    for received in rx {
        match received {
            Ok(stats) => {
                println!(
                    "  [{}] 字符: {}, 单词: {}, 行数: {}",
                    stats.path, stats.chars, stats.words, stats.lines
                );
                total_chars += stats.chars;
                total_words += stats.words;
                total_lines += stats.lines;
                // 打印该文本的高频词
                let tops: Vec<String> = stats
                    .top_words
                    .iter()
                    .map(|(w, c)| format!("{w}({c})"))
                    .collect();
                println!("    高频词: {}", tops.join(", "));
                all_stats.push(stats);
            }
            Err(e) => {
                eprintln!("  [错误] {e}");
                errors.push(e);
            }
        }
    }

    let elapsed = start.elapsed();

    println!("\n--- 消息传递模式汇总 ---");
    println!("  总字符数: {total_chars}");
    println!("  总单词数: {total_words}");
    println!("  总行数:   {total_lines}");
    println!("  成功统计: {}/{} 文本", all_stats.len(), sample_count);
    if !errors.is_empty() {
        println!("  错误数量: {}", errors.len());
    }
    println!("  耗时:     {:?}", elapsed);
}

// ---------------------------------------------------------------------------
// 方案B：共享状态（Shared State）
//
// 使用 `Arc<Mutex<HashMap<String, usize>>>` 让多个线程同时更新同一个
// 全局词频表。这是"共享内存"模式，需要用 Mutex 保证互斥访问。
//
// # 线程安全设计
// - `Arc`（Atomic Reference Counting）：允许多个线程共享同一块堆内存的所有权。
//   普通的 `Rc` 不是 `Send`，无法跨线程传递；`Arc` 的引用计数操作使用
//   原子指令（atomic instructions）实现，安全且高效。
// - `Mutex`：提供内部可变性（interior mutability），保证同一时刻只有一个
//   线程能访问被保护的数据。
// - `lock()` 返回 `MutexGuard`，在其 Drop 时自动释放锁 —— RAII 保证
//   不会因忘记解锁而导致死锁。
// - 如果持有 Mutex 的线程 panic，lock() 会返回 Err（PoisonError），
//   实际项目中应根据情况决定是否 recover。
// ---------------------------------------------------------------------------

fn run_shared_state(samples: &[(String, String)]) {
    println!("\n===== 方案B：共享状态（Arc<Mutex<HashMap>>）=====\n");

    let global_freq: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = Vec::new();
    let start = Instant::now();

    for (name, text) in samples {
        let freq = Arc::clone(&global_freq);
        let t = text.clone();
        let n = name.clone();

        let handle = thread::spawn(move || {
            // 本地先预处理单词
            let words: Vec<String> = t
                .split_whitespace()
                .map(|w| {
                    w.to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>()
                })
                .filter(|w| !w.is_empty())
                .collect();

            // 获取全局互斥锁并更新全局 HashMap
            let mut map = freq.lock().unwrap();
            for word in words {
                *map.entry(word).or_insert(0) += 1;
            }
            // MutexGuard 在此自动 drop，释放锁

            println!("  [线程完成] {n}");
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for h in handles {
        h.join().unwrap();
    }

    let elapsed = start.elapsed();

    // 从全局 HashMap 中取出 top-10 高频词
    let map = global_freq.lock().unwrap();
    let mut sorted: Vec<(&String, &usize)> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    let top_n: Vec<&(&String, &usize)> = sorted.iter().take(10).collect();

    println!("\n--- 全局高频词 Top-10（共享状态聚合）---");
    for (word, count) in top_n {
        println!("  {}: {}", word, count);
    }
    println!("  不同单词总数: {}", map.len());
    println!("  耗时: {:?}", elapsed);
}

// ---------------------------------------------------------------------------
// 单线程对照：用于对比并行加速效果
// ---------------------------------------------------------------------------

fn run_single_threaded(samples: &[(String, String)]) {
    println!("===== 单线程对照 =====\n");
    let start = Instant::now();

    for (name, text) in samples {
        let stats = count_stats(name, text);
        println!(
            "  [{}] 字符: {}, 单词: {}, 行数: {}",
            stats.path, stats.chars, stats.words, stats.lines
        );
    }

    let elapsed = start.elapsed();
    println!("\n  单线程耗时: {:?}\n", elapsed);
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("并行文本统计器 (Parallel Text Stats)\n");

    // ---------- 准备样本数据 ----------
    let samples: Vec<(String, String)> = vec![
        (
            "sample_01_english".into(),
            "Rust is a systems programming language that runs blazingly fast, \
             prevents segfaults, and guarantees thread safety. Rust's rich type \
             system and ownership model guarantee memory-safety and thread-safety \
             — enabling you to eliminate many classes of bugs at compile-time. \
             Rust has great documentation, a friendly compiler with useful error \
             messages, and top-notch tooling — an integrated package manager and \
             build tool, smart multi-editor support with auto-completion and type \
             inspections, an auto-formatter, and more."
                .into(),
        ),
        (
            "sample_02_mixed".into(),
            "Concurrency in Rust is fearless. The type system catches data races \
             at compile time. You can use threads, channels, mutexes, and atomics. \
             Rust's ownership rules ensure that shared data is either immutable or \
             accessed exclusively. Fearless concurrency allows you to write code \
             that's free of subtle bugs and easy to refactor. Message passing and \
             shared state are both first-class patterns in the language."
                .into(),
        ),
        (
            "sample_03_repeat".into(),
            "thread thread channel channel mutex mutex arc arc arc \
             rust rust rust rust concurrency concurrency concurrency \
             safety safety safety safety safety \
             memory memory memory \
             ownership ownership ownership ownership ownership \
             type type type type type type \
             compile compile compile \
             parallel parallel parallel parallel parallel parallel parallel"
                .into(),
        ),
        (
            "sample_04_empty".into(),
            // 空文本（仅含空白） —— 用于测试错误处理路径
            "   ".into(),
        ),
        (
            "sample_05_longer".into(),
            "The Rust programming language helps you write faster more reliable \
             software. High-level ergonomics and low-level control are often at \
             odds in programming language design; Rust challenges that conflict. \
             Through balancing powerful technical capacity and a great developer \
             experience, Rust gives you the option to control low-level details \
             such as memory layout and thread management without all the hassle \
             traditionally associated with such control. Rust is for students, \
             for companies, for open source developers, for everyone who values \
             speed and stability in their software. The language is designed to \
             be reliable and efficient, especially in systems programming where \
             those properties are paramount. Memory safety without garbage \
             collection, concurrency without data races, abstraction without \
             overhead — these are the core principles that guide Rust's design."
                .into(),
        ),
    ];

    // ---------- 单线程 ----------
    run_single_threaded(&samples);

    // ---------- 多线程：消息传递 ----------
    run_message_passing(&samples);

    // ---------- 多线程：共享状态 ----------
    run_shared_state(&samples);

    // ---------- 总结 ----------
    println!("\n========================================");
    println!("线程安全设计要点总结:");
    println!("1. 使用 `move` 闭包将数据所有权转移到线程中，避免悬垂引用。");
    println!("2. mpsc channel 实现了\"消息传递\"模式：线程间通过发送数据通信，");
    println!("   不需要共享内存，避免数据竞争（data race）。");
    println!("3. Arc<Mutex<T>> 实现了\"共享状态\"模式：多个线程安全地共享数据，");
    println!("   Arc 保证引用计数的原子性，Mutex 保证互斥访问。");
    println!("4. MutexGuard 实现 RAII：离开作用域自动释放锁，防止忘记释放。");
    println!("5. Rust 的类型系统在编译期检查 Send / Sync trait，");
    println!("   如果错误地在线程间共享非线程安全的数据，编译器会直接报错。");
    println!("6. 通过 join() 等待所有线程完成，确保主线程不会提前退出。");
    println!("========================================");
}
