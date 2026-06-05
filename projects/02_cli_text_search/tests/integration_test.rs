//! # 集成测试
//!
//! 本文件包含对 `cli_text_search` 库核心函数的集成测试。
//! 集成测试位于 `tests/` 目录中，作为独立 crate 编译，
//! 只能访问库的公开 API（`pub` 函数），模拟外部用户的使用方式。
//!
//! 这种设计确保：
//! - 测试覆盖了公开接口的正确性
//! - 不会依赖库的内部实现细节
//! - 测试代码与库代码完全隔离

use cli_text_search::{SearchConfig, search, search_case_insensitive};

// ============================================================================
// 测试用示例文本
// ============================================================================

/// 返回用于测试的多行示例文本
fn sample_poem() -> &'static str {
    "I'm nobody! Who are you?\n\
     Are you nobody, too?\n\
     Then there's a pair of us - don't tell!\n\
     They'd banish us, you know.\n\
     \n\
     How dreary to be somebody!\n\
     How public, like a frog\n\
     To tell your name the livelong day\n\
     To an admiring bog!"
}

/// 返回包含中英文混合的测试文本
fn sample_bilingual() -> &'static str {
    "Rust 是一门系统编程语言\n\
     rust 是 Rust 的小写形式\n\
     RUST 是全大写形式\n\
     学习 Rust 非常有趣\n\
     Java 和 Python 也很流行\n\
     这行完全不相关"
}

// ============================================================================
// 测试 search 函数
// ============================================================================

/// 测试 `search` 函数的基本大小写敏感搜索。
///
/// 验证在给定的英文诗歌中搜索特定单词时：
/// - 能够找到所有大小写精确匹配的行
/// - 不会匹配大小写不同的行
#[test]
fn test_search_basic_case_sensitive() {
    let contents = sample_poem();

    // 搜索 "you"（小写）—— 注意 "your" 也包含 "you" 作为子串
    let result = search("you", contents, true);
    assert_eq!(result.len(), 4);
    assert!(result.contains(&"I'm nobody! Who are you?"));
    assert!(result.contains(&"Are you nobody, too?"));
    assert!(result.contains(&"They'd banish us, you know."));
    assert!(result.contains(&"To tell your name the livelong day"));

    // 搜索 "You"（大写开头）—— 诗中没有大写的 "You" 开头
    let result = search("You", contents, true);
    assert_eq!(result.len(), 0, "不应该匹配到任何以大写 Y 开头的 'You'");

    // 搜索 "How"（大写开头）
    let result = search("How", contents, true);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"How dreary to be somebody!"));
    assert!(result.contains(&"How public, like a frog"));
}

/// 测试中文内容的搜索。
///
/// 验证对包含中文字符的文本进行搜索时的行为。
#[test]
fn test_search_with_chinese() {
    let contents = sample_bilingual();

    // 搜索中文
    let result = search("系统编程", contents, true);
    assert_eq!(result.len(), 1);
    assert!(result[0].contains("系统编程语言"));

    // 搜索混合内容
    let result = search("Rust", contents, true);
    // "Rust 是一门系统编程语言" — 匹配（首字母大写）
    // "rust 是 Rust 的小写形式" — 匹配（第二个 "Rust" 是大写）
    // "学习 Rust 非常有趣" — 匹配
    assert_eq!(result.len(), 3);
}

/// 测试 `search` 函数通过 `case_sensitive: false` 标志的搜索。
///
/// 验证当 `case_sensitive` 设置为 `false` 时，搜索行为
/// 与 `search_case_insensitive` 一致。
#[test]
fn test_search_with_case_insensitive_flag() {
    let contents = sample_bilingual();

    let result = search("rust", contents, false);
    assert_eq!(result.len(), 4, "应该匹配 Rust/rust/RUST/rUsT 所有变体");
    assert!(result.contains(&"Rust 是一门系统编程语言"));
    assert!(result.contains(&"rust 是 Rust 的小写形式"));
    assert!(result.contains(&"RUST 是全大写形式"));
    assert!(result.contains(&"学习 Rust 非常有趣"));
}

// ============================================================================
// 测试 search_case_insensitive 函数
// ============================================================================

/// 测试大小写不敏感搜索：验证能匹配所有大小写变体。
#[test]
fn test_search_case_insensitive_all_variants() {
    let contents = "Rust\nrust\nRUST\nrUsT\nRuSt\nJava";

    let result = search_case_insensitive("rust", contents);
    assert_eq!(result.len(), 5, "大小写不敏感搜索应匹配所有 5 种大小写变体");
}

/// 测试大小写不敏感搜索：英文诗歌中的搜索。
#[test]
fn test_search_case_insensitive_poem() {
    let contents = sample_poem();

    // 搜索 "nobody" 应该匹配 Nobody 和 nobody
    let result = search_case_insensitive("nobody", contents);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"I'm nobody! Who are you?"));
    assert!(result.contains(&"Are you nobody, too?"));

    // 搜索 "THEY"（全大写）应该匹配 "They'd"
    let result = search_case_insensitive("THEY", contents);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&"They'd banish us, you know."));
}

/// 测试大小写不敏感搜索：部分匹配（子串包含在一个单词中）。
#[test]
fn test_search_case_insensitive_substring() {
    let contents = "programming\nPROGRAMMER\nProgram\nJava";

    let result = search_case_insensitive("program", contents);
    assert_eq!(result.len(), 3, "program 应作为子串匹配所有变体");
}

// ============================================================================
// 测试空查询（防御性编程）
// ============================================================================

/// 测试空查询：验证防御性编程——空查询不返回任何结果。
///
/// 在常规字符串操作中，`"hello".contains("")` 返回 `true`，
/// 这将导致空查询匹配所有行。我们通过在函数中添加显式检查来避免这种行为。
#[test]
fn test_empty_query_returns_empty() {
    let contents = "line one\nline two\nline three";

    let result_sensitive = search("", contents, true);
    assert!(
        result_sensitive.is_empty(),
        "空查询在大小写敏感模式下应返回空结果"
    );

    let result_insensitive = search_case_insensitive("", contents);
    assert!(
        result_insensitive.is_empty(),
        "空查询在大小写不敏感模式下应返回空结果"
    );
}

/// 测试空查询在非空文本中的行为——多行验证。
#[test]
fn test_empty_query_multi_line() {
    let contents = "\n\n\n"; // 三个空行
    let result = search("", contents, true);
    assert!(result.is_empty(), "即使内容为空行，空查询也不应匹配");
}

// ============================================================================
// 测试无匹配结果
// ============================================================================

/// 测试无匹配：当查询字符串不存在于任何行中时，返回空向量。
#[test]
fn test_no_matches_returns_empty() {
    let contents = "apple\nbanana\ncherry\ndate";

    let result = search("xyz_not_found_xyz", contents, true);
    assert!(result.is_empty());
}

/// 测试无匹配：英文诗歌中搜索不存在的词。
#[test]
fn test_no_matches_in_poem() {
    let contents = sample_poem();

    let result = search("dinosaur", contents, true);
    assert!(result.is_empty());
}

/// 测试无匹配：搜索几乎匹配但不完全相同的字符串。
#[test]
fn test_near_miss_no_match() {
    let contents = "hello world\nhelloo\nhell";

    // "helloo" 包含 "hello" 作为子串，但反过来不成立
    let result = search("hellooo", contents, true);
    assert!(result.is_empty());

    // 精确搜索 "hell" 应匹配所有三行（"hello"、"helloo"、"hell" 都包含 "hell"）
    let result = search("hell", contents, true);
    assert_eq!(result.len(), 3);
}

// ============================================================================
// 测试 SearchConfig
// ============================================================================

/// 测试 SearchConfig::new 的正确参数解析。
#[test]
fn test_config_new_valid() {
    let args = vec![
        "program".to_string(),
        "hello".to_string(),
        "test.txt".to_string(),
    ];
    let config = SearchConfig::new(&args).expect("应成功解析参数");
    assert_eq!(config.query, "hello");
    assert_eq!(config.file_path, "test.txt");
    assert!(config.case_sensitive);
}

/// 测试 SearchConfig::new 参数不足的情况。
#[test]
fn test_config_new_insufficient_args() {
    let args = vec!["program".to_string()];
    let result = SearchConfig::new(&args);
    assert!(result.is_err());
}

/// 测试 SearchConfig::new 带有 -i 标志。
#[test]
fn test_config_new_case_insensitive() {
    let args = vec![
        "program".to_string(),
        "rust".to_string(),
        "notes.txt".to_string(),
        "-i".to_string(),
    ];
    let config = SearchConfig::new(&args).expect("应成功解析参数");
    assert!(!config.case_sensitive);

    // 长格式也应支持
    let args = vec![
        "program".to_string(),
        "rust".to_string(),
        "notes.txt".to_string(),
        "--case-insensitive".to_string(),
    ];
    let config = SearchConfig::new(&args).expect("应成功解析参数");
    assert!(!config.case_sensitive);
}

/// 测试 SearchConfig 的 Debug 派生。
#[test]
fn test_config_debug() {
    let config = SearchConfig {
        query: "test".to_string(),
        file_path: "file.txt".to_string(),
        case_sensitive: true,
    };
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("file.txt"));
}
