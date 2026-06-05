//! 集成测试 - 测试 Todo CLI 的端到端行为。
//!
//! 这些测试直接使用 `todo_cli` 库函数，模拟 CLI 层的操作流程。

use std::fs;
use todo_cli::TodoList;

const TEST_FILE: &str = "__test_integration.json";

/// 测试结束后清理文件
fn cleanup() {
    let _ = fs::remove_file(TEST_FILE);
}

/// 辅助函数：创建预填充的列表
fn seeded_list() -> TodoList {
    let mut list = TodoList::new(TEST_FILE);
    list.add("集成测试条目 A".to_string());
    list.add("集成测试条目 B".to_string());
    list.add("集成测试条目 C".to_string());
    list
}

// ─── 测试 1: 添加、完成、删除的完整生命周期 ───

#[test]
fn test_full_lifecycle() {
    cleanup();
    let mut list = TodoList::new(TEST_FILE);

    // 添加
    let item = list.add("完整生命周期测试".to_string());
    assert_eq!(item.id, 1);
    assert_eq!(list.items.len(), 1);

    // 完成
    list.complete(1).unwrap();
    assert!(list.items[0].completed);

    // 删除
    list.delete(1).unwrap();
    assert!(list.items.is_empty());

    cleanup();
}

// ─── 测试 2: 列表过滤（pending / done） ───

#[test]
fn test_list_filtering() {
    let mut list = seeded_list();

    // 完成条目 1 和 3
    list.complete(1).unwrap();
    list.complete(3).unwrap();

    let pending = list.list_pending();
    let done = list.list_completed();

    assert_eq!(pending.len(), 1, "应剩 1 条未完成");
    assert_eq!(done.len(), 2, "应有 2 条已完成");
    assert_eq!(pending[0].id, 2, "未完成的应该是条目 #2");
    assert_eq!(done[0].id, 1, "已完成的第一个应该是 #1");
    assert_eq!(done[1].id, 3, "已完成的第二个应该是 #3");
}

// ─── 测试 3: 持久化：保存并重新加载 ───

#[test]
fn test_persistence_roundtrip() {
    cleanup();
    let mut original = TodoList::new(TEST_FILE);

    // 添加多种状态的数据
    original.add("持久化-未完成".to_string());
    let done_item = original.add("持久化-已完成".to_string());
    original.complete(done_item.id).unwrap();
    original.add("持久化-另一个未完成".to_string());

    // 保存
    original.save().expect("保存应成功");
    assert!(fs::metadata(TEST_FILE).is_ok(), "文件应存在");

    // 加载
    let loaded = TodoList::load(TEST_FILE).expect("加载应成功");

    // 验证所有字段
    assert_eq!(loaded.items.len(), 3, "加载后条目数量应一致");
    assert_eq!(loaded.next_id, original.next_id, "next_id 应一致");
    assert_eq!(loaded.file_path, original.file_path, "file_path 应一致");

    // 逐条验证
    assert_eq!(loaded.items[0].id, 1);
    assert_eq!(loaded.items[0].title, "持久化-未完成");
    assert!(!loaded.items[0].completed);

    assert_eq!(loaded.items[1].id, 2);
    assert_eq!(loaded.items[1].title, "持久化-已完成");
    assert!(loaded.items[1].completed);

    assert_eq!(loaded.items[2].id, 3);
    assert_eq!(loaded.items[2].title, "持久化-另一个未完成");
    assert!(!loaded.items[2].completed);

    cleanup();
}

// ─── 测试 4: 对空文件和缺失文件的处理 ───

#[test]
fn test_empty_and_missing_files() {
    // 加载不存在的文件应返回空列表
    let loaded = TodoList::load("__nonexistent_file_12345.json").unwrap();
    assert!(loaded.items.is_empty());
    assert_eq!(loaded.next_id, 1);
    assert_eq!(loaded.file_path, "__nonexistent_file_12345.json");

    // 空文件（创建后加载）
    let empty_file = "__test_empty.json";
    fs::write(empty_file, "").unwrap();
    let result = TodoList::load(empty_file);
    // 空内容不是有效 JSON，应返回错误
    assert!(result.is_err());
    fs::remove_file(empty_file).unwrap();
}

// ─── 测试 5: 大批量操作 ───

#[test]
fn test_bulk_operations() {
    let mut list = TodoList::new(TEST_FILE);

    // 批量添加
    for i in 1..=50 {
        let item = list.add(format!("大批量条目 #{}", i));
        assert_eq!(item.id, i);
    }
    assert_eq!(list.items.len(), 50);
    assert_eq!(list.next_id, 51);

    // 批量完成偶数 ID
    for id in (2..=50).step_by(2) {
        list.complete(id).unwrap();
    }
    assert_eq!(list.list_pending().len(), 25);
    assert_eq!(list.list_completed().len(), 25);

    // 批量删除前 10 条
    for id in 1..=10 {
        list.delete(id).unwrap();
    }
    assert_eq!(list.items.len(), 40);
}

// ─── 测试 6: 重复完成和无效操作 ───

#[test]
fn test_edge_cases() {
    let mut list = seeded_list();

    // 重复完成同一个条目
    list.complete(1).unwrap();
    let err = list.complete(1).unwrap_err();
    assert!(err.contains("已经"), "重复完成应提示已完成: {}", err);

    // 完成不存在的 ID
    let err = list.complete(999).unwrap_err();
    assert!(err.contains("未找到"), "不存在的 ID: {}", err);

    // 删除不存在的 ID
    let err = list.delete(999).unwrap_err();
    assert!(err.contains("未找到"), "删除不存在的 ID: {}", err);

    // 空标题
    let item = list.add("".to_string());
    assert_eq!(item.title, "");
    assert_eq!(item.id, 4);
}
