//! 任务解析模块
//!
//! 负责将文本行解析为 `(title, Priority)` 元组。

use crate::models::Priority;

/// 解析一行任务输入。
///
/// 支持的格式：
/// - `[High] 任务标题` → `Some(("任务标题", Priority::High))`
/// - `[Medium] 任务标题` → `Some(("任务标题", Priority::Medium))`
/// - `[Low] 任务标题` → `Some(("任务标题", Priority::Low))`
/// - 无优先级标记 → `None`
///
/// # 示例
///
/// ```
/// use packages_and_modules::services::parse_task_line;
/// use packages_and_modules::models::Priority;
///
/// let result = parse_task_line("[High] 学习模块系统");
/// assert!(result.is_some());
/// let (title, priority) = result.unwrap();
/// assert_eq!(title, "学习模块系统");
/// assert_eq!(priority, Priority::High);
/// ```
pub fn parse_task_line(line: &str) -> Option<(String, Priority)> {
    // 调用同模块内的私有辅助函数
    let trimmed = trim_input(line);

    if trimmed.is_empty() {
        return None;
    }

    // 尝试提取优先级标记
    if let Some(rest) = trimmed.strip_prefix("[High]") {
        let title = trim_input(rest);
        // 使用 super:: 引用父模块（services）中的私有模块 validator
        // 虽然 validator 对外部私有，但 parser 作为 services 的子模块可以访问
        if super::validator::validate_title(&title) {
            Some((title, Priority::High))
        } else {
            None
        }
    } else if let Some(rest) = trimmed.strip_prefix("[Medium]") {
        let title = trim_input(rest);
        if super::validator::validate_title(&title) {
            Some((title, Priority::Medium))
        } else {
            None
        }
    } else if let Some(rest) = trimmed.strip_prefix("[Low]") {
        let title = trim_input(rest);
        if super::validator::validate_title(&title) {
            Some((title, Priority::Low))
        } else {
            None
        }
    } else {
        // 无优先级标记的行直接返回 None
        None
    }
}

/// 去除字符串前后空白（私有辅助函数）。
///
/// 此函数仅在当前模块内可见，外部无法调用。
fn trim_input(s: &str) -> String {
    s.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    #[test]
    fn test_parse_high_priority() {
        let result = parse_task_line("[High] 完成作业");
        assert!(result.is_some());
        let (title, priority) = result.unwrap();
        assert_eq!(title, "完成作业");
        assert_eq!(priority, Priority::High);
    }

    #[test]
    fn test_parse_low_priority() {
        let result = parse_task_line("[Low] 整理桌面");
        assert!(result.is_some());
        let (title, priority) = result.unwrap();
        assert_eq!(title, "整理桌面");
        assert_eq!(priority, Priority::Low);
    }

    #[test]
    fn test_parse_no_priority() {
        let result = parse_task_line("没有优先级的任务");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_empty_line() {
        let result = parse_task_line("   ");
        assert!(result.is_none());
    }

    #[test]
    fn test_trim_input() {
        // trim_input 是私有的，但我们可以通过 parse_task_line 间接测试
        let result = parse_task_line("  [High]  带空格标题  ");
        assert!(result.is_some());
        let (title, _) = result.unwrap();
        assert_eq!(title, "带空格标题");
    }
}
