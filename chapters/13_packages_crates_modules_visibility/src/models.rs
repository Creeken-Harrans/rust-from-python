//! 数据模型模块
//!
//! 定义任务（Task）的结构与行为，以及优先级枚举。

/// 一个待办任务。
///
/// # 字段
///
/// * `id` - 任务唯一标识
/// * `title` - 任务标题
/// * `completed` - 是否已完成
///
/// # 示例
///
/// ```
/// use packages_and_modules::models::Task;
/// let mut task = Task::new(1, "学习 Rust".to_string());
/// task.mark_done();
/// assert!(task.completed);
/// ```
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    /// 创建一个新的未完成任务。
    pub fn new(id: u32, title: String) -> Self {
        Task {
            id,
            title,
            completed: false,
        }
    }

    /// 将任务标记为已完成。
    pub fn mark_done(&mut self) {
        self.completed = true;
    }
}

/// 任务优先级。
///
/// # 变体
///
/// * `Low` - 低优先级
/// * `Medium` - 中等优先级
/// * `High` - 高优先级
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// 创建一组示例任务，用于演示和测试。
///
/// # 返回
///
/// 返回包含 4 个预定义任务的 `Vec<Task>`。
///
/// # 示例
///
/// ```
/// use packages_and_modules::models::create_sample_tasks;
/// let tasks = create_sample_tasks();
/// assert_eq!(tasks.len(), 4);
/// ```
pub fn create_sample_tasks() -> Vec<Task> {
    vec![
        Task::new(1, "学习 Rust 模块系统".to_string()),
        Task::new(2, "理解 Package 与 Crate 区别".to_string()),
        Task::new(3, "掌握 pub 可见性修饰符".to_string()),
        Task::new(4, "练习 use 路径导入".to_string()),
    ]
}
