//! 集成测试
//!
//! 集成测试位于 `tests/` 目录下，每个 `.rs` 文件被编译为独立的 crate。
//! 因此，集成测试只能访问库的公开 API（`pub` 接口），
//! 无法访问 `pub(crate)` 或私有的函数/模块。
//!
//! 这与单元测试不同：单元测试写在 `#[cfg(test)] mod tests { ... }` 中，
//! 可以访问父模块的私有成员。

use packages_and_modules::models::*;
use packages_and_modules::services::*;

// ============================================================================
// 测试 parse_task_line 函数（通过 re-export 路径导入）
// ============================================================================

#[test]
fn test_parse_task_line_high_priority() {
    let result = parse_task_line("[High] 完成 Rust 作业");
    assert!(result.is_some(), "应该成功解析高优先级任务");

    let (title, priority) = result.unwrap();
    assert_eq!(title, "完成 Rust 作业");
    assert_eq!(priority, Priority::High);
}

#[test]
fn test_parse_task_line_medium_priority() {
    let result = parse_task_line("[Medium] 代码审查");
    assert!(result.is_some(), "应该成功解析中等优先级任务");

    let (title, priority) = result.unwrap();
    assert_eq!(title, "代码审查");
    assert_eq!(priority, Priority::Medium);
}

#[test]
fn test_parse_task_line_low_priority() {
    let result = parse_task_line("[Low] 整理文件");
    assert!(result.is_some(), "应该成功解析低优先级任务");

    let (title, priority) = result.unwrap();
    assert_eq!(title, "整理文件");
    assert_eq!(priority, Priority::Low);
}

#[test]
fn test_parse_task_line_no_prefix() {
    let result = parse_task_line("没有优先级标记的任务");
    assert!(result.is_none(), "无标记的输入应返回 None");
}

#[test]
fn test_parse_task_line_empty_string() {
    let result = parse_task_line("");
    assert!(result.is_none(), "空字符串应返回 None");
}

#[test]
fn test_parse_task_line_whitespace_only() {
    let result = parse_task_line("     ");
    assert!(result.is_none(), "纯空白字符串应返回 None");
}

#[test]
fn test_parse_task_line_with_extra_spaces() {
    let result = parse_task_line("  [Medium]   有额外空格的标题  ");
    assert!(result.is_some(), "带空格的输入应该被正确处理");

    let (title, priority) = result.unwrap();
    assert_eq!(title, "有额外空格的标题");
    assert_eq!(priority, Priority::Medium);
}

#[test]
fn test_parse_task_line_wrong_case_prefix() {
    // 前缀必须是精确匹配 [High] 而非 [high] 或 [HIGH]
    let result = parse_task_line("[high] 小写前缀");
    assert!(result.is_none(), "小写前缀应返回 None");
}

#[test]
fn test_parse_task_line_empty_title() {
    // 优先级标记后没有有效标题（标题验证会失败）
    let result = parse_task_line("[Low] ");
    assert!(result.is_none(), "空标题应返回 None（validator 会拒绝）");
}

// ============================================================================
// 测试 Task 创建与状态更新
// ============================================================================

#[test]
fn test_task_creation() {
    let task = Task::new(42, "编写测试".to_string());

    assert_eq!(task.id, 42);
    assert_eq!(task.title, "编写测试");
    assert!(!task.completed, "新任务默认未完成");
}

#[test]
fn test_task_mark_done() {
    let mut task = Task::new(1, "学习 Rust".to_string());

    assert!(!task.completed, "标记前应为未完成");
    task.mark_done();
    assert!(task.completed, "标记后应为已完成");
}

#[test]
fn test_task_mark_done_idempotent() {
    let mut task = Task::new(1, "学习 Rust".to_string());

    task.mark_done();
    task.mark_done(); // 第二次调用不应 panic
    assert!(task.completed);
}

// ============================================================================
// 测试 create_sample_tasks
// ============================================================================

#[test]
fn test_create_sample_tasks_count() {
    let tasks = create_sample_tasks();
    assert_eq!(tasks.len(), 4, "示例任务应有 4 个");
}

#[test]
fn test_create_sample_tasks_all_uncompleted() {
    let tasks = create_sample_tasks();
    for task in &tasks {
        assert!(!task.completed, "示例任务默认全部未完成");
    }
}

#[test]
fn test_create_sample_tasks_unique_ids() {
    let tasks = create_sample_tasks();
    let mut ids: Vec<u32> = tasks.iter().map(|t| t.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), tasks.len(), "所有任务 ID 应唯一");
}

// ============================================================================
// 测试 Task 的 Debug 和 Clone trait
// ============================================================================

#[test]
fn test_task_debug_format() {
    let task = Task::new(1, "测试".to_string());
    let debug_str = format!("{task:?}");
    assert!(debug_str.contains("Task"));
    assert!(debug_str.contains("测试"));
}

#[test]
fn test_task_clone() {
    let original = Task::new(10, "原始任务".to_string());
    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);
    assert_eq!(original.title, cloned.title);
    assert_eq!(original.completed, cloned.completed);
}

#[test]
fn test_priority_equality() {
    assert_eq!(Priority::Low, Priority::Low);
    assert_eq!(Priority::Medium, Priority::Medium);
    assert_eq!(Priority::High, Priority::High);
    assert_ne!(Priority::Low, Priority::High);
}

// ============================================================================
// 验证可见性边界：集成测试无法访问私有 API
// ============================================================================

// 以下代码如果取消注释，会导致编译错误，因为：
// - `validator` 模块是私有的（mod validator, 没有 pub）
// - `trim_input` 是 parser 模块的私有函数
// - 集成测试是外部 crate，只能访问 pub 接口

// ❌ 编译错误: module `validator` is private
// use packages_and_modules::services::validator;

// ❌ 编译错误: function `trim_input` is private
// use packages_and_modules::services::parser::trim_input;

// ❌ 编译错误: function `sanitize` is `pub(crate)`, 集成测试是外部 crate
// use packages_and_modules::services::validator::sanitize;

#[test]
fn test_visibility_demonstrated() {
    // 这个测试仅用来文档化可见性规则
    // 它验证我们可以访问公开的 API
    let tasks = create_sample_tasks();
    assert!(!tasks.is_empty());

    let result = parse_task_line("[High] 测试");
    assert!(result.is_some());

    // 证明公开 API 可以正常使用即可
    println!("集成测试只能访问 pub 接口，这验证了 Rust 的可见性规则工作正常。");
}
