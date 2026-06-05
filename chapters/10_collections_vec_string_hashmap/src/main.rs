#![allow(
    clippy::approx_constant,
    clippy::unnecessary_sort_by,
    clippy::vec_init_then_push
)]
use std::collections::HashMap;

/// 将文本分词：转为小写，去除标点符号，返回拥有的 Vec<String>
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .collect()
}

/// 统计每个单词的出现次数
fn count_frequencies(words: &[String]) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in words {
        // Entry API: 如果存在则返回可变引用，否则插入默认值 0
        *freq.entry(word.clone()).or_insert(0) += 1;
    }
    freq
}

/// 返回出现次数最多的前 n 个单词（按次数降序排序）
fn get_top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = freq.iter().map(|(k, v)| (k.clone(), *v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(n);
    entries
}

/// 演示 HashMap::get —— 返回 Option<&usize>
fn search_word<'a>(freq: &'a HashMap<String, usize>, word: &str) -> Option<&'a usize> {
    freq.get(word)
}

// ---------------------------------------------------------------------------
//  str  vs  String   vs  &str   vs  &String
// ---------------------------------------------------------------------------
//  str        : 原始字符串类型，大小未知，通常以引用 &str 形式使用
//  String     : 拥有所有权的、堆上分配的、可增长的 UTF-8 字符串
//  &String    : 对 String 的引用，自动强制解引用为 &str（Deref Trait）
//  &str       : 字符串切片 — 对 UTF-8 字节序列的不可变引用
//
//  关系：
//    String --& --> &str        (String 实现了 Deref<Target = str>)
//    &String --Deref--> &str    (自动强制解引用)
//    str 是 !Sized，所以永远不能直接拥有 str；必须用 Box<str> 或 &str
// ---------------------------------------------------------------------------

fn main() {
    // ======================== 1. Vec<T> 演示 ========================
    println!("============ Vec<T> 演示 ============");

    // Vec::new() 创建空向量
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("Vec::new + push: {:?}", v);

    // vec! 宏
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("vec! 宏: {:?}", v2);

    // pop：弹出最后一个元素
    let popped = v2.pop();
    println!("pop 后: {:?}, 弹出的值: {:?}", v2, popped);

    // 索引（越界会 panic）
    println!("v2[0] = {}", v2[0]);
    // v2[100] —— 会导致 panic，编译通过但运行时崩溃

    // get：安全访问，返回 Option<&T>
    match v2.first() {
        Some(val) => println!("v2.get(0) = {val}"),
        None => println!("v2.get(0) → None"),
    }
    match v2.get(100) {
        Some(val) => println!("v2.get(100) = {val}"),
        None => println!("v2.get(100) → None（安全，不会 panic）"),
    }

    // 迭代：iter() 不可变借用
    print!("iter() 不可变借用: ");
    for val in v2.iter() {
        print!("{val} ");
    }
    println!();

    // iter_mut() 可变借用
    print!("iter_mut() 可变借用（每个 * 2）: ");
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("{v2:?}");

    // into_iter() 消耗所有权
    let v3 = vec![100, 200, 300];
    let sum: i32 = v3.into_iter().sum();
    println!("into_iter() 消耗所有权，总和: {sum}");
    // v3 在此之后不能再被使用 —— 所有权已转移

    println!();

    // ======================== 2. String 演示 ========================
    println!("============ String 演示 ============");

    // String::new() + push_str
    let mut s = String::new();
    s.push_str("Hello");
    s.push_str(", ");
    s.push_str("世界");
    println!("String 拼接: {s}");

    // format! 宏：不获取任何参数的所有权
    let name = String::from("Rust");
    let greeting = format!("你好，{}！", name);
    println!("format! 宏: {greeting}");
    println!("name 仍然可用: {name}"); // name 的所有权未转移

    // String 从 &str 创建
    let greeting_str: &str = "Hello, Rust!";
    let owned: String = String::from(greeting_str);
    println!("String::from(&str): {owned}");

    // UTF-8 演示：为什么不能直接用索引
    let chinese = String::from("你好世界"); // 每个中文字符 3 字节
    println!("\n字符串: \"{chinese}\"");
    println!("  字节长度 (len): {}", chinese.len()); // 12，不是 4
    // chinese[0] —— 编译错误！String 不支持按索引访问
    // 因为 UTF-8 中一个字符可以是 1~4 字节，O(1) 索引不能保证有效字符边界

    // 使用 .chars() 获取 Unicode 标量值
    print!("  .chars(): ");
    for c in chinese.chars() {
        print!("{c} ");
    }
    println!();

    // 使用 .bytes() 获取原始字节
    print!("  .bytes(): ");
    for b in chinese.bytes() {
        print!("{b:#x} ");
    }
    println!();

    // 获取第 n 个字符的正确方式
    let third_char = chinese.chars().nth(2);
    println!("  第 3 个字符: {:?}", third_char);

    // String vs &str 关系演示
    let owned_str: String = String::from("owned");
    let _borrowed: &str = &owned_str; // String → &str（Deref）
    let ref_string: &String = &owned_str; // &String
    let also_borrowed: &str = ref_string; // &String → &str（自动解引用）
    println!("\nString → &str → &String → &str: {also_borrowed}");

    // 切片
    let hello = &owned_str[0..5];
    println!("切片: {hello}");

    println!();

    // ======================== 3. HashMap 演示 ========================
    println!("============ HashMap 演示 ============");

    // 创建 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 82);
    scores.insert(String::from("Charlie"), 88);

    println!("HashMap 内容:");
    for (name, score) in scores.iter() {
        println!("  {name}: {score}");
    }

    // get：返回 Option<&V>
    let name_to_find = String::from("Alice");
    match scores.get(&name_to_find) {
        Some(score) => println!("Alice 的分数: {score}"),
        None => println!("未找到 Alice"),
    }
    // 查询不存在的键
    match scores.get("David") {
        Some(score) => println!("David: {score}"),
        None => println!("David 不在 HashMap 中"),
    }

    // Entry API: entry().or_insert()
    println!("\nEntry API 演示:");
    // 只有不存在时才插入
    scores.entry(String::from("Alice")).or_insert(100); // Alice 已存在，不会覆盖
    scores.entry(String::from("David")).or_insert(75); // David 不存在，插入 75
    println!("  插入后: {scores:?}");

    // entry().or_insert_with() —— 惰性计算默认值
    scores.entry(String::from("Eve")).or_insert_with(|| {
        println!("   正在计算 Eve 的默认分数…");
        60
    });
    println!("  惰性插入后: {scores:?}");

    // 三种迭代器对比
    println!("\n三种迭代器对比:");
    // iter(): 不可变借用，原 HashMap 仍可用
    print!("  iter(): ");
    for (k, v) in scores.iter() {
        print!("{k}:{v} ");
    }
    println!();

    // iter_mut(): 可变借用
    for (_, v) in scores.iter_mut() {
        *v += 5; // 每人加 5 分
    }
    println!("  iter_mut() 加 5 分后: {scores:?}");

    // into_iter(): 消耗所有权
    let consumed: HashMap<String, usize> = [("X".into(), 1), ("Y".into(), 2)].into_iter().collect();
    println!("  into_iter() 消耗后的新 HashMap: {consumed:?}");

    println!();

    // ======================== 4. 所有权演示 ========================
    println!("============ 所有权与集合 ============");

    // HashMap 中存储 String（Owned） vs &str（Borrowed）
    // 存储 String：HashMap 拥有数据，生命周期独立
    {
        let mut map: HashMap<String, i32> = HashMap::new();
        let owned_key = String::from("key1");
        map.insert(owned_key.clone(), 10); // 克隆所有权进 map
        map.insert(String::from("key2"), 20);
        println!("  Owned HashMap: {map:?}");
        println!("  owned_key 仍可使用: {owned_key}"); // 因为 clone 了
    }

    // 存储 &str：HashMap 不拥有数据，受生命周期约束
    // 以下代码无法编译（演示目的）：
    // let mut map: HashMap<&str, i32> = HashMap::new();
    // {
    //     let temp = String::from("short-lived");
    //     map.insert(&temp, 10);  // temp 生命周期不够长
    // }
    // println!("{map:?}"); // temp 已释放，map 持有悬垂引用！

    println!("  结论: 优先在 HashMap 中使用 String（Owned）");
    println!("       仅在数据生命周期明确长于 HashMap 时使用 &str");

    println!();

    // ======================== 5. 文本分析演练 ========================
    println!("============ 文本分析演练 ============");

    let text = "\
        Rust is a systems programming language that runs blazingly fast, \
        prevents segfaults, and guarantees thread safety. Rust is \
        memory-safe without using a garbage collector. Rust achieves \
        memory safety through its ownership system. Ownership is Rust's \
        most unique feature. Rust's ownership system enables memory \
        safety guarantees without needing a garbage collector. The \
        borrow checker enforces ownership rules at compile time.\
    ";

    // 分词
    let tokens = tokenize(text);
    println!("分词结果 ({} 个单词):", tokens.len());
    println!("  {tokens:?}");

    // Vec 操作演示
    let mut demo_vec = tokens.clone();
    println!("\nVec 操作:");
    println!("  len: {}", demo_vec.len());
    println!("  is_empty: {}", demo_vec.is_empty());
    println!("  first: {:?}", demo_vec.first());
    println!("  last: {:?}", demo_vec.last());
    demo_vec.truncate(10);
    println!("  truncate(10): {demo_vec:?}");
    demo_vec.clear();
    println!("  clear() 后: {:?} (len = {})", demo_vec, demo_vec.len());

    // 统计频率
    let frequencies = count_frequencies(&tokens);
    println!("\n词频统计 (按插入顺序):");
    for (word, count) in frequencies.iter() {
        println!("  {word}: {count}");
    }

    // Top N
    let top5 = get_top_n(&frequencies, 5);
    println!("\n出现次数最多的 5 个单词:");
    for (i, (word, count)) in top5.iter().enumerate() {
        println!("  {}. {word}: {count}", i + 1);
    }

    // 搜索单词
    let search_terms = ["rust", "ownership", "python"];
    println!("\n单词搜索:");
    for term in search_terms {
        match search_word(&frequencies, term) {
            Some(count) => println!("  \"{term}\" 出现了 {count} 次"),
            None => println!("  \"{term}\" 未找到"),
        }
    }

    // HashMap 的更多方法
    println!("\nHashMap 更多方法:");
    println!("  len: {}", frequencies.len());
    println!("  is_empty: {}", frequencies.is_empty());
    println!(
        "  contains_key(\"rust\"): {}",
        frequencies.contains_key("rust")
    );
    println!(
        "  contains_key(\"python\"): {}",
        frequencies.contains_key("python")
    );

    // String 的更多方法
    let mut demo_string = String::from("  Rust Collections  ");
    println!("\nString 更多方法:");
    println!("  原始: \"{demo_string}\"");
    println!("  trim(): \"{}\"", demo_string.trim());
    println!("  to_uppercase(): \"{}\"", demo_string.to_uppercase());
    println!("  to_lowercase(): \"{}\"", demo_string.to_lowercase());
    println!(
        "  replace: \"{}\"",
        demo_string.replace("Collections", "集合")
    );
    demo_string.push('!');
    println!("  push('!'): \"{demo_string}\"");
    println!("  contains: {}", demo_string.contains("Rust"));
    println!(
        "  split_whitespace: {:?}",
        demo_string.split_whitespace().collect::<Vec<&str>>()
    );

    println!("\n============ 演示完毕 ============");
}
