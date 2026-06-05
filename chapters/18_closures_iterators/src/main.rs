#![allow(clippy::unnecessary_sort_by)]
use std::fmt;

// ============================================================================
// 数据结构
// ============================================================================

/// 文本统计信息
#[derive(Debug, PartialEq)]
struct Stats {
    word_count: usize,
    char_count: usize,
    longest_word_len: usize,
    avg_word_len: f64,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "单词数: {}, 字符数: {}, 最长单词长度: {}, 平均单词长度: {:.2}",
            self.word_count, self.char_count, self.longest_word_len, self.avg_word_len
        )
    }
}

// ============================================================================
// 命令式版本：使用传统 for 循环
// ============================================================================

fn text_stats_imperative(text: &str) -> Stats {
    let mut word_count = 0;
    let mut char_count = 0;
    let mut longest_word_len = 0;
    let mut total_word_len = 0;

    for word in text.split_whitespace() {
        word_count += 1;
        let len = word.chars().count();
        char_count += len;
        total_word_len += len;
        if len > longest_word_len {
            longest_word_len = len;
        }
    }

    let avg_word_len = if word_count > 0 {
        total_word_len as f64 / word_count as f64
    } else {
        0.0
    };

    Stats {
        word_count,
        char_count,
        longest_word_len,
        avg_word_len,
    }
}

// ============================================================================
// 迭代器版本：使用 iterator combinators
// ============================================================================

fn text_stats_iterator(text: &str) -> Stats {
    let words: Vec<&str> = text.split_whitespace().collect();

    let word_count = words.len();

    let char_count: usize = words.iter().map(|w| w.chars().count()).sum();

    let longest_word_len = words
        .iter()
        .map(|w| w.chars().count())
        .fold(0, |max, len| if len > max { len } else { max });

    let total_word_len: usize = words.iter().map(|w| w.chars().count()).sum();
    let avg_word_len = if word_count > 0 {
        total_word_len as f64 / word_count as f64
    } else {
        0.0
    };

    Stats {
        word_count,
        char_count,
        longest_word_len,
        avg_word_len,
    }
}

// ============================================================================
// 闭包演示
// ============================================================================

/// Fn 闭包：对每个单词应用转换函数（不可变借用环境）
fn apply_to_words<F>(text: &str, f: F) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    text.split_whitespace().map(f).collect()
}

/// FnMut 闭包：按谓词过滤单词（可变借用环境）
fn filter_words<F>(text: &str, mut predicate: F) -> Vec<&str>
where
    F: FnMut(&&str) -> bool,
{
    text.split_whitespace().filter(|w| predicate(w)).collect()
}

/// FnOnce 闭包：消费环境（获取所有权，只能调用一次）
fn consume_words<F>(text: &str, consumer: F)
where
    F: FnOnce(String),
{
    let all_words = text.split_whitespace().collect::<Vec<&str>>().join(" ");
    consumer(all_words);
}

// ============================================================================
// 迭代器演示
// ============================================================================

/// 演示 iter() vs iter_mut() vs into_iter()
fn demo_ownership() {
    println!("--- iter() vs iter_mut() vs into_iter() ---");

    // iter() — 不可变引用迭代
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("iter() 后原 vec 仍可用: {:?}, 结果: {:?}", v, doubled);

    // iter_mut() — 可变引用迭代
    let mut v2 = vec![10, 20, 30];
    v2.iter_mut().for_each(|x| *x += 5);
    println!("iter_mut() 修改原 vec: {:?}", v2);

    // into_iter() — 消费所有权，迭代后原 vec 不可用
    let v3 = vec!["a", "b", "c"];
    let upper: Vec<String> = v3.into_iter().map(|s| s.to_uppercase()).collect();
    println!("into_iter() 消费了原 vec, 结果: {:?}", upper);
    // println!("{:?}", v3); // 编译错误：v3 已被移动
}

/// 演示迭代器适配器
fn demo_adapters() {
    println!("--- 迭代器适配器 ---");

    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map + filter
    let even_squares: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("偶数的平方: {:?}", even_squares);

    // enumerate
    println!("带索引的迭代:");
    for (i, val) in nums.iter().enumerate() {
        println!("  nums[{}] = {}", i, val);
    }

    // take
    let first_five: Vec<i32> = nums.iter().take(5).copied().collect();
    println!("前5个: {:?}", first_five);

    // skip
    let after_five: Vec<i32> = nums.iter().skip(5).copied().collect();
    println!("跳过前5个后: {:?}", after_five);

    // chain
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let chained: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("拼接: {:?}", chained);

    // zip
    let names = ["alice", "bob", "charlie"];
    let scores = [95, 87, 92];
    println!("配对:");
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("  {}: {}", name, score);
    }
}

/// 演示迭代器消费器
fn demo_consumers() {
    println!("--- 迭代器消费器 ---");

    let nums = [1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // collect
    let collected: Vec<i32> = nums.to_vec();
    println!("collect: {:?}", collected);

    // fold
    let sum_by_fold = nums.iter().sum::<i32>();
    println!("fold 求和: {}", sum_by_fold);

    // count
    let count = nums.iter().count();
    println!("count: {}", count);

    // sum
    let sum: i32 = nums.iter().sum();
    println!("sum: {}", sum);

    // for_each
    print!("for_each: ");
    nums.iter().for_each(|x| print!("{} ", x));
    println!();

    // find
    let first_even = nums.iter().find(|&&x| x % 2 == 0);
    println!("find 第一个偶数: {:?}", first_even);

    // any / all
    let has_ten = nums.contains(&10);
    let all_positive = nums.iter().all(|&x| x > 0);
    let all_even = nums.iter().all(|&x| x % 2 == 0);
    println!(
        "any(==10): {}, all(>0): {}, all(even): {}",
        has_ten, all_positive, all_even
    );

    // nth
    let third = nums.get(2);
    println!("nth(2): {:?}", third);
}

/// 惰性求值演示
fn demo_laziness() {
    println!("--- 惰性求值 ---");

    let nums = [1, 2, 3, 4, 5];

    // 这个链条在 collect 之前不会执行任何 map 调用
    println!("迭代器链已建立, 但尚未消费任何元素");
    // 下面这行不会触发任何 map 调用：
    let lazy_chain = nums.iter().map(|&x| {
        print!("[map] ");
        x * 2
    });

    // 现在消费迭代器
    print!("collect 开始消费: ");
    let result: Vec<i32> = lazy_chain.collect();
    println!();
    println!("collect 之后 result = {:?}", result);

    // 再次演示：take 会限制实际执行次数
    let mut call_count2 = 0;
    let first_three: Vec<i32> = nums
        .iter()
        .map(|&x| {
            call_count2 += 1;
            x * 3
        })
        .take(3)
        .collect();
    println!(
        "take(3) 后 call_count = {} (只调用了3次而非5次), result = {:?}",
        call_count2, first_three
    );
}

// ============================================================================
// 闭包环境捕获演示
// ============================================================================

fn demo_closure_capture() {
    println!("--- 闭包环境捕获 ---");

    // 1. 不可变借用（Fn）
    let prefix = "Item: ";
    let label = |s: &str| format!("{}{}", prefix, s);
    println!("不可变借用: {}", label("apple"));
    println!("prefix 仍可用: {}", prefix); // prefix 只是被不可变借用

    // 2. 可变借用（FnMut）
    let mut counter = 0;
    let mut count_and_return = |x: i32| {
        counter += 1;
        x * 2
    };
    println!("可变借用调用1: {}", count_and_return(5));
    println!("可变借用调用2: {}", count_and_return(10));
    println!("counter 最终值: {}", counter);

    // 3. 移动捕获（FnOnce / move）
    let owned_string = String::from("这条数据会被消费");
    let consumer = |msg: String| {
        println!("消费消息: {} (原始数据: {})", msg, owned_string);
    };
    // consumer 是 FnOnce，因为 owned_string 被移动到闭包中
    // 如果 String 未实现 Copy，移动后 owned_string 不可再用
    consumer(String::from("你好"));
    // println!("{}", owned_string); // 编译错误：owned_string 已被移动

    // 4. move 关键字：显式移动捕获
    let greeting = String::from("Hello");
    let move_closure = move |name: &str| {
        // greeting 被移动到闭包中
        format!("{}, {}!", greeting, name)
    };
    println!("move 闭包: {}", move_closure("World"));
    // println!("{}", greeting); // 编译错误：greeting 已被 move 到闭包中
}

// ============================================================================
// 命令式 vs 迭代器风格对比
// ============================================================================

fn demo_style_comparison() {
    println!("--- 风格对比 ---");

    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];

    // 命令式：找出长度 > 5 的单词并转为大写
    let mut long_upper_imperative = Vec::new();
    for w in &words {
        if w.len() > 5 {
            long_upper_imperative.push(w.to_uppercase());
        }
    }
    println!("命令式: {:?}", long_upper_imperative);

    // 迭代器风格：同样逻辑
    let long_upper_iterator: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 5)
        .map(|w| w.to_uppercase())
        .collect();
    println!("迭代器: {:?}", long_upper_iterator);

    println!("结果一致: {}", long_upper_imperative == long_upper_iterator);

    println!();
    println!("=== 命令式 vs 迭代器风格分析 ===");
    println!("命令式优点:");
    println!("  - 逻辑流程直观，按步骤阅读");
    println!("  - 容易插入调试打印或早期返回");
    println!("  - 对复杂状态管理更灵活");
    println!();
    println!("迭代器优点:");
    println!("  - 声明式，意图清晰（filter then map then collect）");
    println!("  - 链式调用，减少临时变量");
    println!("  - 零成本抽象：编译后通常生成与手写循环相同的机器码");
    println!("  - 惰性求值：中间结果不分配额外内存");
    println!();
    println!("选择建议：简单转换用迭代器，复杂逻辑用循环；");
    println!("两者并非对立，同一个函数中可以混用。");
}

// ============================================================================
// 实用函数组合示例
// ============================================================================

/// 统计每个单词出现次数
fn word_frequency(text: &str) -> Vec<(&str, usize)> {
    let mut freq: Vec<(&str, usize)> = text
        .split_whitespace()
        .fold(std::collections::HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect();
    // 按频率降序排序
    freq.sort_by_key(|b| std::cmp::Reverse(b.1));
    freq
}

/// 找出所有包含指定子串的单词
fn find_words_containing<'a>(text: &'a str, substr: &str) -> Vec<&'a str> {
    text.split_whitespace()
        .filter(|w| w.contains(substr))
        .collect()
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║    闭包与迭代器 — Rust 的函数式编程工具        ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let sample =
        "Rust is a modern systems programming language focusing on safety speed and concurrency";

    // ========================
    // 第1部分：文本统计对比
    // ========================
    println!("{}", "═".repeat(56));
    println!("  第1部分：命令式 vs 迭代器 — 文本统计");
    println!("{}", "═".repeat(56));
    println!();
    println!("样本: \"{}\"", sample);
    println!();

    let stats_imp = text_stats_imperative(sample);
    let stats_iter = text_stats_iterator(sample);

    println!("命令式结果:      {}", stats_imp);
    println!("迭代器结果:      {}", stats_iter);
    println!("结果一致: {}", stats_imp == stats_iter);
    println!();

    // ========================
    // 第2部分：闭包演示
    // ========================
    println!("{}", "═".repeat(56));
    println!("  第2部分：闭包（Closures）");
    println!("{}", "═".repeat(56));
    println!();

    demo_closure_capture();
    println!();

    // Fn 闭包 — apply_to_words
    let upper = apply_to_words(sample, |w| w.to_uppercase());
    println!("Fn 闭包 (to_uppercase): {:?}", upper);

    // FnMut 闭包 — filter_words（带计数器）
    let mut call_count = 0;
    let short_words = filter_words(sample, |w: &&str| {
        call_count += 1;
        w.len() <= 3
    });
    println!(
        "FnMut 闭包 (len<=3, 调用了{}次): {:?}",
        call_count, short_words
    );

    // FnOnce 闭包 — consume_words
    let log_target = String::from("LOG");
    consume_words(sample, move |all| {
        println!("FnOnce 闭包 (目标={}): 所有单词 = \"{}\"", log_target, all);
    });
    println!();

    // ========================
    // 第3部分：迭代器演示
    // ========================
    println!("{}", "═".repeat(56));
    println!("  第3部分：迭代器（Iterators）");
    println!("{}", "═".repeat(56));
    println!();

    demo_ownership();
    println!();

    demo_adapters();
    println!();

    demo_consumers();
    println!();

    demo_laziness();
    println!();

    demo_style_comparison();
    println!();

    // ========================
    // 第4部分：实际应用
    // ========================
    println!("{}", "═".repeat(56));
    println!("  第4部分：实际应用示例");
    println!("{}", "═".repeat(56));
    println!();

    let text = "the quick brown fox jumps over the lazy dog the fox was quick";

    println!("词频统计:");
    for (word, count) in word_frequency(text) {
        println!("  {}: {}", word, count);
    }
    println!();

    let containing = find_words_containing(text, "o");
    println!("包含 'o' 的单词: {:?}", containing);
    println!();

    println!("══ 演示完成 ══");
}
