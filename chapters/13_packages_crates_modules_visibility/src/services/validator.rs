//! 输入验证模块（私有）
//!
//! 此模块在 `services/mod.rs` 中以 `mod validator;`（无 `pub`）声明，
//! 因此对外部 crate 不可见。它演示了 Rust 的默认私有可见性。

/// 验证任务标题的有效性。
///
/// `pub(super)` 表示此函数仅对父模块（`services`）及其子模块可见。
/// 也就是说，`parser` 可以通过 `super::validator::validate_title` 调用它，
/// 但 `crate::services::validator::validate_title` 对外部不可用。
///
/// 验证规则：标题不能为空，且长度在 1~200 字符之间。
pub(super) fn validate_title(title: &str) -> bool {
    !title.is_empty() && title.len() <= 200
}

/// 清理输入字符串。
///
/// `pub(crate)` 表示此函数对当前 crate 内的所有模块可见，
/// 但外部 crate 无法访问。
///
/// 清理规则：
/// - 去除首尾空白
/// - 将连续多个空格压缩为单个空格
///
/// `#[allow(dead_code)]` 抑制未使用警告——此函数作为可见性演示而存在。
#[allow(dead_code)]
pub(crate) fn sanitize(input: &str) -> String {
    let trimmed = input.trim();
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_title_valid() {
        assert!(validate_title("学习 Rust"));
    }

    #[test]
    fn test_validate_title_empty() {
        assert!(!validate_title(""));
    }

    #[test]
    fn test_validate_title_too_long() {
        let long_title = "Rust".repeat(100); // 400 字符
        assert!(!validate_title(&long_title));
    }

    #[test]
    fn test_sanitize_removes_extra_spaces() {
        let result = sanitize("  学习    Rust   模块  ");
        assert_eq!(result, "学习 Rust 模块");
    }

    #[test]
    fn test_sanitize_empty() {
        let result = sanitize("   ");
        assert_eq!(result, "");
    }
}
