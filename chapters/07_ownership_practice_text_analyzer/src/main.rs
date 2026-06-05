//! # 文本分析器 —— 所有权与借用综合练习
//!
//! 本程序是一个命令行文本分析工具，用于演示 Rust 所有权、借用和切片的核心概念。
//! 每个函数的设计都经过仔细考虑：参数应该用 `&str` 还是 `String`？
//! 返回值应该是借用的切片还是拥有所有权的 `String`？
//!
//! ## 设计原则
//!
//! - **优先使用 `&str` 作为参数**：如果函数只需要读取数据，使用 `&str`（不可变借用）
//!   可以让调用者保留所有权，函数只临时借用。
//! - **优先返回借用的切片**：如果返回的数据是输入的一部分，返回 `&str` 避免不必要的克隆。
//!   如果结果是从无到有合成的，返回 `String` 因为需要新的所有权。
//! - **用 `Option` 表示可能不存在的结果**：比返回空字符串更语义明确，编译器强制处理。

/// 示例文本 —— 包含英文和中文，用于展示 Unicode 处理
const SAMPLE_TEXT: &str = "Rust is a systems programming language that runs blazingly fast prevents segfaults and guarantees thread safety\n同时支持中文文本的处理和分析";

// ============================================================================
// 分析函数
// ============================================================================

/// 统计文本中的 Unicode 字符数（不是字节数）。
///
/// ## 为什么不用 `.len()`？
///
/// `&str` 的 `.len()` 返回的是**字节数**，不是字符数。
/// 中文字符在 UTF-8 中占 3 个字节，所以 `.len()` 会把一个中文汉字算作 3。
/// `.chars().count()` 遍历的是 Unicode 标量值（scalar values），每个汉字算 1 个字符，
/// 这才是人类直觉中的"字符数"。
///
/// ## 参数选择：`&str` vs `String`
///
/// ```text
/// // [好的设计] 用 &str —— 调用者保留所有权，函数只借用
/// fn count_chars(text: &str) -> usize { ... }
///
/// // [次优设计] 用 String —— 需要调用者转移所有权，不适合只读场景
/// fn count_chars(text: String) -> usize { ... }
/// ```
///
/// 这里用 `&str` 因为函数只读取数据，不需要占有它。如果用 `String`，
/// 每次调用都得 `clone()` 或者用完后无法再使用原数据。
fn count_chars(text: &str) -> usize {
    // chars() 返回 Unicode 标量值的迭代器
    // count() 消耗迭代器并返回元素数量
    // 这是 O(n) 操作，因为 UTF-8 需要解码
    text.chars().count()
}

/// 统计文本中的单词数（以空白字符分隔）。
///
/// ## 参数选择：`&str`
///
/// 同样只读场景，`&str` 是最佳选择。调用者可以把同一个 `&str`
/// 传给多个分析函数而不需要克隆。
///
/// 注意：如果想从 `String` 传参，Rust 会自动解引用（deref coercion）：
/// `&my_string` 会被自动转换为 `&str`。这就是为什么 `&str` 比 `&String`
/// 更灵活 —— 它可以同时接受 `&str` 和 `&String`。
///
/// ```text
/// // [好的设计] 用 &str —— 兼容 &str 和 &String
/// fn count_words(text: &str) -> usize { ... }
///
/// // [次优设计] 用 &String —— 只能接受 &String，不灵活
/// fn count_words(text: &String) -> usize { ... }
/// ```
fn count_words(text: &str) -> usize {
    // split_whitespace() 按空白字符分割，返回迭代器
    // 比 split(' ') 更好，因为它能处理多个空格、制表符、换行等
    text.split_whitespace().count()
}

/// 找出文本中最长的单词，返回它的借用切片（如果文本非空）。
///
/// ## 返回值选择：`Option<&str>` vs `String`
///
/// ```text
/// // [好的设计] 返回 Option<&str> —— 零成本借用，不需要分配新内存
/// fn find_longest_word(text: &str) -> Option<&str> { ... }
///
/// // [次优设计] 返回 String —— 不必要地克隆了数据
/// fn find_longest_word(text: &str) -> String { ... }
/// ```
///
/// 因为最长的单词**已经是输入文本的一部分**，返回 `&str` 直接指向原数据，
/// 不需要在堆上分配新的 `String`。这就是借用的威力 —— 安全且零开销。
///
/// 如果文本为空（没有单词），返回 `None` 而不是空字符串 `""`。
/// `Option<&str>` 让调用者必须处理"不存在"的情况，编译器会帮你检查。
fn find_longest_word(text: &str) -> Option<&str> {
    text.split_whitespace().max_by_key(|word| {
        // 使用 chars().count() 而不是 len()
        // 因为我们需要的是字符数，不是字节数
        word.chars().count()
    })
    // max_by_key 已经返回 Option<&str>，直接传递即可
}

/// 返回文本中的第一个单词作为借用切片。
///
/// ## 返回值分析
///
/// `Option<&str>` 再次展示：
/// - `Option` 表示"可能有，也可能没有"（空文本）
/// - `&str` 表示"我只借用，不拥有"
/// - 生命周期由 Rust 自动推断：返回的引用生命周期与输入 `text` 相同
///
/// 这意味着返回的切片在 `text` 被释放之前都有效 —— Rust 编译器
/// 在编译时就能保证这一点，不会出现悬垂指针。
fn first_word(text: &str) -> Option<&str> {
    // split_whitespace().next() 返回 Option<&str>
    // 这正是我们需要的 —— 第一个单词的引用
    text.split_whitespace().next()
}

/// 检查文本中是否包含指定关键词（区分大小写）。
///
/// ## 为什么两个参数都用 `&str`？
///
/// 两个参数都是只读的，都应该用借用。如果某个参数用了 `String`，
/// 调用者在不必要时失去了所有权。
///
/// ```text
/// // [好的设计] 两个参数都是 &str
/// fn contains_keyword(text: &str, keyword: &str) -> bool { ... }
///
/// // [次优设计] 第一个参数用 String —— 调用者必须转移所有权
/// fn contains_keyword(text: String, keyword: &str) -> bool { ... }
/// ```
///
/// 一个好的经验法则：**函数需要拥有数据时才用 `String`，否则用 `&str`**。
fn contains_keyword(text: &str, keyword: &str) -> bool {
    text.contains(keyword)
}

/// 统计每个单词出现的频率，返回按频率降序排列的 `Vec<(&str, usize)>`。
///
/// ## 返回值选择：`Vec<(&str, usize)>` vs `Vec<(String, usize)>`
///
/// ```text
/// // [好的设计] Vec<(&str, usize)> —— 单词引用指向原文本，零额外分配
/// fn count_word_frequency(text: &str) -> Vec<(&str, usize)> { ... }
///
/// // [次优设计] Vec<(String, usize)> —— 每个单词都克隆到新的 String
/// fn count_word_frequency(text: &str) -> Vec<(String, usize)> { ... }
/// ```
///
/// 这体现了 Rust 的一个核心优势：我们可以安全地让返回值引用输入数据。
/// 在很多其他语言中，你需要担心原数据被修改或释放导致的问题；
/// Rust 的借用检查器保证这一切是安全的。
///
/// ## 为什么返回 `Vec` 而不是切片 `&[(&str, usize)]`？
///
/// 因为频率统计结果是**新生成的数据结构**（排序后的 Vec），不属于输入数据。
/// 我们必须 return 一个有所有权的 `Vec` —— 如果返回 `&[(...)]`，
/// 它引用的是函数内部的局部变量，函数返回后局部变量被释放，引用就悬垂了。
fn count_word_frequency(text: &str) -> Vec<(&str, usize)> {
    use std::collections::HashMap;

    let mut freq: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        // entry API：如果存在就修改，不存在就插入默认值 0 然后修改
        *freq.entry(word).or_insert(0) += 1;
    }

    // 将 HashMap 转换为 Vec 以便排序
    let mut result: Vec<(&str, usize)> = freq.into_iter().collect();

    // 按频率降序排序
    // sort_by 需要可变借用 &mut self，所以 result 需要是 mut
    result.sort_by(|a, b| {
        // 先按频率降序
        b.1.cmp(&a.1)
            // 频率相同时按字母顺序升序
            .then(a.0.cmp(b.0))
    });

    result
}

/// 生成文本的摘要报告，返回拥有所有权的 `String`。
///
/// ## 为什么这里返回 `String` 而不是 `&str`？
///
/// 因为摘要内容是**从多个分析结果拼接而成**的，不是输入文本的一部分。
/// 我们正在创建全新的数据，必须用 `String` 来拥有和返回这些数据。
///
/// 如果用 `&str` 返回，这个引用指向谁？指向局部构造的 `String`？
/// 那函数返回后 `String` 被 drop，引用就悬垂了 —— Rust 编译器会报错。
///
/// ```text
/// // [好的设计] 返回 String —— 合成的新数据需要所有权
/// fn summarize_text(text: &str) -> String { ... }
///
/// // [错误设计] 返回 &str —— 引用了局部变量，编译不过
/// fn summarize_text(text: &str) -> &str {
///     let result = format!(...);  // result 是局部变量
///     &result  // 编译错误：result 活不到函数外面
/// }
/// ```
///
/// ## 参数用了 `&str`
///
/// 即使函数返回 `String`，参数仍然用 `&str`。因为函数需要读取输入数据
/// 但不需要拥有它。读取用借用，返回新数据用所有权 —— 各取所需。
fn summarize_text(text: &str) -> String {
    let char_count = count_chars(text);
    let word_count = count_words(text);
    let longest = find_longest_word(text);
    let first = first_word(text);

    // format! 宏返回一个新的 String（拥有所有权）
    // 这是 "从借用数据合成拥有数据" 的典型模式
    let mut summary = format!(
        "╔══════════════════ 文本分析报告 ══════════════════╗\n\
         ║  字符总数（Unicode）: {:>5}                       ║\n\
         ║  单词总数           : {:>5}                       ║\n",
        char_count, word_count
    );

    // 处理 Option —— 模式匹配是 Rust 的核心特性
    match longest {
        Some(w) => {
            summary.push_str(&format!(
                "║  最长单词           : {w} ({} 字符)        ║\n",
                w.chars().count()
            ));
        }
        None => {
            summary.push_str("║  最长单词           : (无)                       ║\n");
        }
    }

    match first {
        Some(w) => {
            summary.push_str(&format!(
                "║  第一个单词         : {w}                           ║\n"
            ));
        }
        None => {
            summary.push_str("║  第一个单词         : (无)                       ║\n");
        }
    }

    summary.push_str("╚══════════════════════════════════════════════════╝\n");
    summary
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║           文本分析器 —— 所有权与借用实战                ║");
    println!("║  本程序展示 Rust 所有权系统中 &str、String、Option      ║");
    println!("║  等类型在实际编程中的选择依据和使用模式                 ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ---- 使用字符串字面量（&str 类型）----
    // SAMPLE_TEXT 是 &str，存储在二进制文件的只读数据段中
    // 所有函数都通过借用访问它，没有发生任何复制或所有权转移
    let text = SAMPLE_TEXT;

    println!("📝 分析文本：\n\"{}\"\n", text);

    // ================================================================
    // 1. 字符统计
    // ================================================================
    println!("═══════════════ 1. 字符统计 ═══════════════");

    let char_count = count_chars(text);
    let byte_count = text.len(); // 字节数，用于对比

    println!("  Unicode 字符数（chars().count()）: {}", char_count);
    println!("  字节数（.len()）                : {}", byte_count);
    println!("  差异说明：中文字符在 UTF-8 中占 3 个字节，");
    println!("  所以字节数 > 字符数。.chars() 按 Unicode 标量值计数。\n");

    // ================================================================
    // 2. 单词统计
    // ================================================================
    println!("═══════════════ 2. 单词统计 ═══════════════");

    let word_count = count_words(text);
    println!("  单词总数: {}\n", word_count);

    // ================================================================
    // 3. 最长单词
    // ================================================================
    println!("═══════════════ 3. 最长单词 ═══════════════");

    // find_longest_word 返回 Option<&str>，必须用 match 或 if let 处理
    match find_longest_word(text) {
        Some(longest) => {
            println!("  最长单词: \"{}\"", longest);
            // longest 仍然引用着 text —— 不需要再分配内存
            println!("  类型是 &str —— 它是 text 的一个借用切片");
            println!("  生命周期：只要 text 有效，longest 就有效\n");
        }
        None => println!("  文本为空，没有单词\n"),
    }

    // ================================================================
    // 4. 第一个单词
    // ================================================================
    println!("═══════════════ 4. 第一个单词 ═══════════════");

    // if let 是 match 的语法糖，适合只关心一种情况
    if let Some(first) = first_word(text) {
        println!("  第一个单词: \"{}\"", first);
        // 验证：用 == 比较 &str 和字符串字面量
        // Rust 的 == 比较的是内容，不是地址
        assert_eq!(first, "Rust");
        println!("  验证通过：确实是 \"Rust\"\n");
    }

    // ================================================================
    // 5. 关键词搜索
    // ================================================================
    println!("═══════════════ 5. 关键词搜索 ═══════════════");

    // contains_keyword 接受两个 &str —— 都是借用，没有所有权转移
    let keywords = ["Rust", "Python", "guarantees", "中文"];

    for kw in &keywords {
        let found = contains_keyword(text, kw);
        println!("  包含 \"{}\"? {}", kw, if found { "是 ✓" } else { "否 ✗" });
    }
    println!();

    // ================================================================
    // 6. 词频统计
    // ================================================================
    println!("═══════════════ 6. 词频统计 ═══════════════");

    let freq = count_word_frequency(text);
    println!("  单词频率（降序）:");
    println!("  {:<20} 次数", "单词");
    println!("  {:-<30}", "");
    for (word, count) in &freq {
        // freq 是 Vec<(&str, usize)>，遍历时得到 &(&str, usize)
        // 可以用模式解构直接拿到 word（&&str）和 count（&usize）
        println!("  {:<20} {}", word, count);
    }
    println!();

    // ================================================================
    // 7. 文本摘要 —— 返回拥有所有权的 String
    // ================================================================
    println!("═══════════════ 7. 文本摘要 ═══════════════");

    // summarize_text 返回 String —— 所有权从函数转移到 summary 变量
    // text 仍然有效（它只是被借用了），summary 是全新的数据
    let summary = summarize_text(text);
    println!("{}", summary);

    // 验证所有权：
    // - text 仍然可以访问（只是被借用了）
    println!(
        "  验证：text 仍然可访问，第一个字符是 '{}'",
        text.chars().next().unwrap()
    );
    // - summary 拥有新数据的所有权
    println!("  验证：summary 长度 = {} 字节\n", summary.len());

    // ================================================================
    // 额外演示：所有权和借用的区别
    // ================================================================
    println!("═══════════════ 8. 所有权与借用演示 ═══════════════");

    demonstration();

    // ================================================================
    // 额外演示：次优设计的影响
    // ================================================================
    println!("═══════════════ 9. 次优设计的影响 ═══════════════");

    suboptimal_demonstration();
}

/// 演示所有权和借用的关键区别。
///
/// 这段代码展示了为什么 `&str` 参数比 `String` 参数更灵活：
/// - 借用的数据可以被多次使用
/// - 拥有所有权的数据在传递后可能被消耗
fn demonstration() {
    println!("  场景 A：用 &str 参数（推荐）");

    let message = String::from("hello world rust");
    // count_words 接受 &str，message 被不可变借用
    let wc1 = count_words(&message); // message 仍然有效
    let wc2 = count_words(&message); // 可以再次借用
    println!("    单词数（第1次）: {}", wc1);
    println!("    单词数（第2次）: {}", wc2);
    println!("    message 仍然可用: \"{}\"", message);
    println!("    → &str 参数 = 借用，原数据不受影响\n");

    println!("  场景 B：如果函数用 String 参数会怎样？");
    // 假设有一个接收 String 的函数（我们不会真这么写）
    fn hypothetical_bad_design(text: String) -> usize {
        text.split_whitespace().count()
    }

    let message2 = String::from("hello world rust");
    let wc3 = hypothetical_bad_design(message2);
    // message2 的所有权已经转移进函数，这里不能再使用
    // println!("{}", message2);  // 编译错误：value borrowed after move
    println!("    单词数: {}", wc3);
    println!("    message2 的所有权已转移，无法再访问");
    println!("    → String 参数 = 转移所有权，不适用于只读场景\n");
}

/// 演示次优设计选择的实际影响。
///
/// 展示如果返回值选择不当（例如返回 String 而不是 &str），
/// 会造成不必要的内存分配和克隆。
fn suboptimal_demonstration() {
    let text = "a short text example";

    // [好的设计] 返回 Option<&str> —— 零分配
    let longest_ref = find_longest_word(text);
    println!("  好的设计 (Option<&str>):");
    println!("    - 没有堆分配");
    println!("    - 返回的引用直接指向原数据");
    if let Some(w) = longest_ref {
        println!("    - 最长单词: \"{}\"\n", w);
    }

    // [次优设计] 如果返回 String —— 每次调用都要克隆
    fn suboptimal_find_longest(text: &str) -> Option<String> {
        text.split_whitespace()
            .max_by_key(|w| w.chars().count())
            .map(|s| s.to_string()) // 不必要的堆分配！
    }

    println!("  次优设计 (Option<String>):");
    println!("    - 需要 .to_string() 在堆上分配新内存");
    println!("    - 克隆了原本就存在的数据");
    let longest_owned = suboptimal_find_longest(text);
    if let Some(w) = longest_owned {
        println!("    - 最长单词: \"{}\"", w);
        println!("    - 但这是克隆的副本，不是原文\n");
    }

    // 总结
    println!("  总结：当返回值是输入数据的一部分时，");
    println!("  优先返回借用 (&str, &[T])，避免不必要的克隆。");
    println!("  当返回值是新合成的数据时，返回拥有所有权的类型 (String, Vec<T>)。");
}
