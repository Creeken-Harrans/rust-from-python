// ---------------------------------------------------------------------------
// Mini KV Store —— 集成测试
//
// 集成测试作为外部用户使用 crate，只能访问公开 API。
// 放在 tests/ 目录下，Cargo 会将每个文件编译为独立的测试 crate。
// ---------------------------------------------------------------------------

use mini_kv_store::KvStore;
use std::fs;

/// 辅助函数：生成唯一的临时文件路径，避免测试间文件冲突
fn temp_path(label: &str) -> String {
    let pid = std::process::id();
    format!("/tmp/mini_kv_integration_{label}_{pid}.kv")
}

/// 辅助函数：清理测试文件
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

// ---------------------------------------------------------------------------
// 基本 CRUD 工作流测试
// ---------------------------------------------------------------------------

#[test]
fn test_full_crud_workflow() {
    let path = temp_path("crud");
    cleanup(&path);

    let mut store = KvStore::open(&path).unwrap();
    assert!(store.is_empty());

    // Create
    store.set("name".into(), "Alice".into()).unwrap();
    store.set("city".into(), "Beijing".into()).unwrap();
    store.set("language".into(), "Rust".into()).unwrap();
    assert_eq!(store.len(), 3);

    // Read
    assert_eq!(store.get("name"), Some(&"Alice".to_string()));
    assert_eq!(store.get("city"), Some(&"Beijing".to_string()));
    assert_eq!(store.get("language"), Some(&"Rust".to_string()));
    assert_eq!(store.get("nonexistent"), None);

    // Update
    store.set("city".into(), "Shanghai".into()).unwrap();
    assert_eq!(store.get("city"), Some(&"Shanghai".to_string()));
    assert_eq!(store.len(), 3); // 长度不变（更新而非新增）

    // Delete
    let removed = store.remove("language").unwrap();
    assert_eq!(removed, Some("Rust".to_string()));
    assert_eq!(store.get("language"), None);
    assert_eq!(store.len(), 2);

    // Delete non-existent key
    let removed = store.remove("nonexistent").unwrap();
    assert_eq!(removed, None);
    assert_eq!(store.len(), 2);

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 持久化：save 后 reopen 验证数据完整性
// ---------------------------------------------------------------------------

#[test]
fn test_persistence_across_sessions() {
    let path = temp_path("persist");
    cleanup(&path);

    // 第一次会话：写入数据并保存
    {
        let mut store = KvStore::open(&path).unwrap();
        store.set("k1".into(), "v1".into()).unwrap();
        store.set("k2".into(), "v2".into()).unwrap();
        store.set("k3".into(), "v3".into()).unwrap();
        store.save().unwrap();
    }

    // 第二次会话：重新打开，验证数据是否正确加载
    {
        let store = KvStore::open(&path).unwrap();
        assert_eq!(store.len(), 3);
        assert_eq!(store.get("k1"), Some(&"v1".to_string()));
        assert_eq!(store.get("k2"), Some(&"v2".to_string()));
        assert_eq!(store.get("k3"), Some(&"v3".to_string()));
    }

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 数据隔离：第二次 open 看到的是磁盘数据，不是上一次会话的内存数据
// ---------------------------------------------------------------------------

#[test]
fn test_unsaved_data_not_persisted() {
    let path = temp_path("unsaved");
    cleanup(&path);

    // 写入并保存一条记录
    {
        let mut store = KvStore::open(&path).unwrap();
        store.set("saved_key".into(), "saved_value".into()).unwrap();
        store.save().unwrap();
    }

    // 第二次打开，添加但不保存
    {
        let mut store = KvStore::open(&path).unwrap();
        store
            .set("unsaved_key".into(), "lost_value".into())
            .unwrap();
        assert_eq!(store.len(), 2);
        // 不调用 save()，直接退出
    }

    // 第三次打开：只有 saved_key 存在，unsaved_key 丢失了
    {
        let store = KvStore::open(&path).unwrap();
        assert_eq!(store.len(), 1);
        assert_eq!(store.get("saved_key"), Some(&"saved_value".to_string()));
        assert_eq!(store.get("unsaved_key"), None);
    }

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 空存储：文件不存在时 open 应正常创建
// ---------------------------------------------------------------------------

#[test]
fn test_open_nonexistent_file() {
    let path = temp_path("nonexistent");
    cleanup(&path); // 确保文件不存在

    let store = KvStore::open(&path).unwrap();
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);

    // list 空存储
    let entries = store.list();
    assert!(entries.is_empty());

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 空文件：存在但内容为空的文件应正确处理
// ---------------------------------------------------------------------------

#[test]
fn test_open_empty_file() {
    let path = temp_path("empty_file");
    cleanup(&path);

    // 创建空文件
    fs::write(&path, "").unwrap();

    let store = KvStore::open(&path).unwrap();
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 格式异常文件：包含格式不正确行的文件应部分加载
// ---------------------------------------------------------------------------

#[test]
fn test_load_file_with_malformed_lines() {
    let path = temp_path("malformed");
    cleanup(&path);

    // 手动创建包含混合内容的数据文件
    let content = "\
good_key|good_value
bad_line_without_pipe

another_good|another_value
   trailing_spaces_line
mixed|value|with|pipes
";
    fs::write(&path, content).unwrap();

    let store = KvStore::open(&path).unwrap();
    // 正确格式的行应被加载
    assert_eq!(store.get("good_key"), Some(&"good_value".to_string()));
    assert_eq!(
        store.get("another_good"),
        Some(&"another_value".to_string())
    );
    // value 中包含 | 的行：split_once 只按第一个 | 分割
    assert_eq!(store.get("mixed"), Some(&"value|with|pipes".to_string()));
    // 空行和格式错误行被跳过
    assert_eq!(store.get("bad_line_without_pipe"), None);

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// list 排序验证
// ---------------------------------------------------------------------------

#[test]
fn test_list_returns_sorted_by_key() {
    let path = temp_path("list_sorted");
    cleanup(&path);

    let mut store = KvStore::open(&path).unwrap();
    store.set("zebra".into(), "z".into()).unwrap();
    store.set("apple".into(), "a".into()).unwrap();
    store.set("monkey".into(), "m".into()).unwrap();
    store.set("1numeric".into(), "n".into()).unwrap();
    store.set("Apple_caps".into(), "ac".into()).unwrap();

    let entries = store.list();
    assert_eq!(entries.len(), 5);

    // 按 key 字符串排序：数字 < 大写字母 < 小写字母（ASCII 顺序）
    let keys: Vec<&String> = entries.iter().map(|(k, _)| *k).collect();
    assert!(
        keys.windows(2).all(|w| w[0] <= w[1]),
        "keys 未按升序排列: {keys:?}"
    );

    // 验证具体排序
    assert_eq!(keys[0], "1numeric");
    assert_eq!(keys[1], "Apple_caps");
    assert_eq!(keys[2], "apple");
    assert_eq!(keys[3], "monkey");
    assert_eq!(keys[4], "zebra");

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// value 包含空格：验证 set 和 get 对含空格值的处理
// ---------------------------------------------------------------------------

#[test]
fn test_values_with_spaces() {
    let path = temp_path("spaces");
    cleanup(&path);

    let mut store = KvStore::open(&path).unwrap();
    store.set("greeting".into(), "hello world".into()).unwrap();
    store
        .set("quote".into(), "to be or not to be".into())
        .unwrap();

    assert_eq!(store.get("greeting"), Some(&"hello world".to_string()));
    assert_eq!(store.get("quote"), Some(&"to be or not to be".to_string()));

    // 持久化后再加载，验证含空格的值不丢失
    store.save().unwrap();
    let store2 = KvStore::open(&path).unwrap();
    assert_eq!(store2.get("greeting"), Some(&"hello world".to_string()));
    assert_eq!(store2.get("quote"), Some(&"to be or not to be".to_string()));

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 覆盖写后持久化：验证更新后的值正确保存
// ---------------------------------------------------------------------------

#[test]
fn test_update_and_persist() {
    let path = temp_path("update_persist");
    cleanup(&path);

    // 写入初始值
    {
        let mut store = KvStore::open(&path).unwrap();
        store.set("version".into(), "1".into()).unwrap();
        store.save().unwrap();
    }

    // 更新值
    {
        let mut store = KvStore::open(&path).unwrap();
        assert_eq!(store.get("version"), Some(&"1".to_string()));
        store.set("version".into(), "2".into()).unwrap();
        store.save().unwrap();
    }

    // 验证持久化的是更新后的值
    {
        let store = KvStore::open(&path).unwrap();
        assert_eq!(store.get("version"), Some(&"2".to_string()));
    }

    cleanup(&path);
}

// ---------------------------------------------------------------------------
// 大批量数据测试：验证基本功能在百级数据下的稳定性
// ---------------------------------------------------------------------------

#[test]
fn test_bulk_operations() {
    let path = temp_path("bulk");
    cleanup(&path);

    let mut store = KvStore::open(&path).unwrap();
    let count = 500;

    // 批量插入
    for i in 0..count {
        store
            .set(format!("key_{i:04}"), format!("value_{i}"))
            .unwrap();
    }
    assert_eq!(store.len(), count);

    // 随机抽查
    assert_eq!(store.get("key_0000"), Some(&"value_0".to_string()));
    assert_eq!(store.get("key_0100"), Some(&"value_100".to_string()));
    assert_eq!(store.get("key_0499"), Some(&"value_499".to_string()));

    // list 验证数量
    let entries = store.list();
    assert_eq!(entries.len(), count);

    // 持久化
    store.save().unwrap();

    // 重新加载验证
    let store2 = KvStore::open(&path).unwrap();
    assert_eq!(store2.len(), count);
    assert_eq!(store2.get("key_0000"), Some(&"value_0".to_string()));
    assert_eq!(store2.get("key_0499"), Some(&"value_499".to_string()));

    cleanup(&path);
}
