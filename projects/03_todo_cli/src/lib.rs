//! # todo_cli
//!
//! 一个简单的命令行待办事项管理器核心库。
//!
//! 该库提供了待办事项的数据结构定义、持久化存储（JSON 格式）
//! 以及基本的 CRUD 操作。命令行界面通过 `clap` 在 `main.rs`
//! 中实现，该库专注于业务逻辑。
//!
//! ## 示例
//!
//! ```rust,no_run
//! use todo_cli::TodoList;
//!
//! let mut list = TodoList::new("todos.json");
//! list.add("学习 Rust".to_string());
//! list.add("写代码".to_string());
//! list.complete(1).unwrap();
//! list.save().unwrap();
//! ```

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

/// 单个待办事项条目。
///
/// 每个条目拥有一个唯一 ID、标题和完成状态。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    /// 条目的唯一标识符
    pub id: u32,
    /// 待办事项的标题/内容
    pub title: String,
    /// 是否已完成
    pub completed: bool,
}

/// 待办事项列表管理器。
///
/// 管理一组 `TodoItem`，支持增删改查和 JSON 文件持久化。
/// `next_id` 用于自增主键，`file_path` 记录持久化文件路径。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    /// 所有待办事项的集合
    pub items: Vec<TodoItem>,
    /// 下一个可用的自增 ID
    pub next_id: u32,
    /// JSON 存储文件的路径
    pub file_path: String,
}

impl TodoList {
    /// 创建一个新的空待办事项列表。
    ///
    /// # 参数
    ///
    /// * `file_path` - JSON 持久化文件的路径。该文件不需要预先存在，
    ///   调用 `save()` 时会自动创建。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use todo_cli::TodoList;
    /// let list = TodoList::new("my_todos.json");
    /// assert!(list.items.is_empty());
    /// assert_eq!(list.next_id, 1);
    /// ```
    pub fn new(file_path: &str) -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
        }
    }

    /// 从 JSON 文件加载待办事项列表。
    ///
    /// 如果文件不存在，则返回一个空的 `TodoList`（不会报错）。
    /// 如果文件存在但格式无效，则返回错误。
    ///
    /// # 错误
    ///
    /// 当 JSON 文件存在但解析失败时返回错误。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use todo_cli::TodoList;
    /// let list = TodoList::load("todos.json").expect("无法加载待办事项");
    /// println!("加载了 {} 个条目", list.items.len());
    /// ```
    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let list: TodoList = serde_json::from_str(&content)?;
                Ok(list)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // 文件不存在，返回空列表
                Ok(TodoList::new(file_path))
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    /// 将当前待办事项列表保存到 JSON 文件。
    ///
    /// 使用格式化的 JSON（带缩进），方便用户直接查看和编辑。
    /// 如果目标目录不存在则会返回错误；但文件本身会被自动创建。
    ///
    /// # 错误
    ///
    /// 在序列化失败或文件写入失败时返回错误。
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// 添加一个新的待办事项。
    ///
    /// 自动分配一个唯一 ID（`next_id` 自增），返回新创建的条目副本。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use todo_cli::TodoList;
    /// let mut list = TodoList::new("t.json");
    /// let item = list.add("买牛奶".to_string());
    /// assert_eq!(item.id, 1);
    /// assert_eq!(item.title, "买牛奶");
    /// assert!(!item.completed);
    /// assert_eq!(list.next_id, 2);
    /// ```
    pub fn add(&mut self, title: String) -> TodoItem {
        let item = TodoItem {
            id: self.next_id,
            title,
            completed: false,
        };
        self.next_id += 1;
        let clone = item.clone();
        self.items.push(item);
        clone
    }

    /// 返回所有待办事项的不可变引用。
    pub fn list(&self) -> &Vec<TodoItem> {
        &self.items
    }

    /// 将指定 ID 的条目标记为已完成。
    ///
    /// # 错误
    ///
    /// 若未找到该 ID 则返回错误字符串。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use todo_cli::TodoList;
    /// let mut list = TodoList::new("t.json");
    /// list.add("测试".to_string());
    /// list.complete(1).unwrap();
    /// assert!(list.items[0].completed);
    /// ```
    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.items.iter_mut().find(|item| item.id == id) {
            Some(item) => {
                if item.completed {
                    return Err(format!("条目 #{} 已经是完成状态", id));
                }
                item.completed = true;
                Ok(())
            }
            None => Err(format!("未找到 ID 为 {} 的待办事项", id)),
        }
    }

    /// 删除指定 ID 的条目。
    ///
    /// # 错误
    ///
    /// 若未找到该 ID 则返回错误字符串。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use todo_cli::TodoList;
    /// let mut list = TodoList::new("t.json");
    /// list.add("测试".to_string());
    /// assert_eq!(list.items.len(), 1);
    /// list.delete(1).unwrap();
    /// assert!(list.items.is_empty());
    /// ```
    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        let pos = self.items.iter().position(|item| item.id == id);
        match pos {
            Some(index) => {
                self.items.remove(index);
                Ok(())
            }
            None => Err(format!("未找到 ID 为 {} 的待办事项", id)),
        }
    }

    /// 返回所有未完成的待办事项引用。
    pub fn list_pending(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    /// 返回所有已完成的待办事项引用。
    pub fn list_completed(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.completed).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试文件路径，测试后自动清理
    const TEST_FILE: &str = "__test_todo_lib.json";

    fn cleanup() {
        let _ = std::fs::remove_file(TEST_FILE);
    }

    #[test]
    fn test_new_list_is_empty() {
        let list = TodoList::new(TEST_FILE);
        assert!(list.items.is_empty());
        assert_eq!(list.next_id, 1);
        assert_eq!(list.file_path, TEST_FILE);
    }

    #[test]
    fn test_add_increments_id() {
        cleanup();
        let mut list = TodoList::new(TEST_FILE);
        let a = list.add("A".to_string());
        let b = list.add("B".to_string());
        let c = list.add("C".to_string());
        assert_eq!(a.id, 1);
        assert_eq!(b.id, 2);
        assert_eq!(c.id, 3);
        assert_eq!(list.items.len(), 3);
        cleanup();
    }

    #[test]
    fn test_complete_toggle() {
        let mut list = TodoList::new(TEST_FILE);
        list.add("测试完成".to_string());
        assert!(!list.items[0].completed);
        list.complete(1).unwrap();
        assert!(list.items[0].completed);
        // 重复完成应报错
        let err = list.complete(1).unwrap_err();
        assert!(err.contains("已经"));
    }

    #[test]
    fn test_complete_nonexistent() {
        let mut list = TodoList::new(TEST_FILE);
        let err = list.complete(999).unwrap_err();
        assert!(err.contains("未找到"));
    }

    #[test]
    fn test_delete_removes_item() {
        let mut list = TodoList::new(TEST_FILE);
        list.add("待删除".to_string());
        list.add("保留".to_string());
        assert_eq!(list.items.len(), 2);
        list.delete(1).unwrap();
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].id, 2);
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut list = TodoList::new(TEST_FILE);
        let err = list.delete(999).unwrap_err();
        assert!(err.contains("未找到"));
    }

    #[test]
    fn test_list_pending_and_completed() {
        let mut list = TodoList::new(TEST_FILE);
        list.add("未完成1".to_string());
        list.add("未完成2".to_string());
        list.add("已完成".to_string());
        list.complete(3).unwrap();

        let pending = list.list_pending();
        let completed = list.list_completed();
        assert_eq!(pending.len(), 2);
        assert_eq!(completed.len(), 1);
        assert_eq!(pending[0].title, "未完成1");
        assert_eq!(completed[0].title, "已完成");
    }

    #[test]
    fn test_save_and_load() {
        cleanup();
        let mut list = TodoList::new(TEST_FILE);
        list.add("持久化条目1".to_string());
        list.add("持久化条目2".to_string());
        list.complete(1).unwrap();
        list.save().unwrap();

        // 重新加载
        let loaded = TodoList::load(TEST_FILE).unwrap();
        assert_eq!(loaded.items.len(), 2);
        assert_eq!(loaded.items[0].title, "持久化条目1");
        assert!(loaded.items[0].completed);
        assert_eq!(loaded.items[1].title, "持久化条目2");
        assert!(!loaded.items[1].completed);
        assert_eq!(loaded.next_id, 3);
        cleanup();
    }

    #[test]
    fn test_load_nonexistent_file() {
        let loaded = TodoList::load("__this_file_does_not_exist__.json").unwrap();
        assert!(loaded.items.is_empty());
        assert_eq!(loaded.next_id, 1);
    }
}
