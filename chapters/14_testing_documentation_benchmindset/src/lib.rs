//! # testing_and_docs
//!
//! 一个用于演示 Rust 测试和文档的小型工具库。
//!
//! # Examples
//!
//! ```
//! use testing_and_docs;
//!
//! let result = testing_and_docs::add(2, 3);
//! assert_eq!(result, 5);
//! ```

/// 返回两个整数的和。
///
/// # Examples
///
/// ```
/// let result = testing_and_docs::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 返回两个整数的乘积。
/// 包含一个会 panic 的边界检查作为文档示例。
///
/// # Examples
///
/// ```
/// let result = testing_and_docs::multiply(3, 4);
/// assert_eq!(result, 12);
/// ```
///
/// # Panics
///
/// 当任一参数为负数时 panic（仅用于教学演示）
///
/// ```should_panic
/// testing_and_docs::multiply(-1, 5);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    if a < 0 || b < 0 {
        panic!("negative numbers are not allowed: a={}, b={}", a, b);
    }
    a * b
}

/// 统计文本中的单词数量。
///
/// 单词以空白字符分隔。连续的空白字符被视为一个分隔符，
/// 前导和尾随空白会被忽略。
///
/// # Examples
///
/// ```
/// let count = testing_and_docs::word_count("Hello world");
/// assert_eq!(count, 2);
///
/// let empty = testing_and_docs::word_count("");
/// assert_eq!(empty, 0);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// 检查字符串是否为回文。
///
/// 回文是指正读反读都一样的字符串。此函数区分大小写。
///
/// # Examples
///
/// ```
/// assert!(testing_and_docs::is_palindrome("radar"));
/// assert!(!testing_and_docs::is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let reversed: String = reverse_string(s);
    s == reversed
}

// 私有辅助函数 —— 反转字符串。
// 通过公共 API 间接测试，但也可以在单元测试中直接测试。
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(100, 200), 300);
        assert_eq!(add(-5, -3), -8);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(5, 0), 0);
        assert_eq!(multiply(1, 1), 1);
        assert_eq!(multiply(7, 8), 56);
    }

    #[test]
    #[should_panic(expected = "negative")]
    fn test_multiply_panics_on_negative() {
        multiply(-1, 5);
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
        assert_eq!(word_count("Hello"), 1);
        assert_eq!(word_count("Hello world"), 2);
        assert_eq!(word_count("Hello  world"), 2); // 多个空格
        assert_eq!(word_count("The quick brown fox"), 4);
        assert_eq!(word_count("  leading spaces"), 2);
        assert_eq!(word_count("trailing spaces  "), 2);
        assert_eq!(word_count("  both  sides  "), 2);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("radar"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("")); // 空字符串视为回文
        assert!(is_palindrome("a")); // 单字符是回文
        assert!(!is_palindrome("Radar")); // 区分大小写
        assert!(is_palindrome("level"));
        assert!(is_palindrome("civic"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    fn test_reverse_private() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
        assert_eq!(reverse_string("radar"), "radar");
        assert_eq!(reverse_string("rust"), "tsur");
    }

    // 返回 Result 类型的测试 —— 失败时不会 panic，而是返回 Err
    #[test]
    fn test_word_count_as_result() -> Result<(), String> {
        if word_count("one two three") != 3 {
            return Err(String::from(
                "word_count should return 3 for 'one two three'",
            ));
        }
        if word_count("") != 0 {
            return Err(String::from("word_count should return 0 for empty string"));
        }
        if word_count("single") != 1 {
            return Err(String::from("word_count should return 1 for 'single'"));
        }
        Ok(())
    }
}
