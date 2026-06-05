use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Mini KV Store —— 一个简单的本地键值数据库
///
/// 数据存储在内存的 `HashMap` 中，可持久化到磁盘文件。
/// 持久化格式为每行一条记录：`key|value`
///
/// # Examples
///
/// ```ignore
/// use mini_kv_store::KvStore;
///
/// let mut store = KvStore::open("data.kv").unwrap();
/// store.set("name".into(), "Alice".into()).unwrap();
/// assert_eq!(store.get("name"), Some(&"Alice".to_string()));
/// store.save().unwrap();
/// ```
#[derive(Debug)]
pub struct KvStore {
    /// 内存中的键值数据
    data: HashMap<String, String>,
    /// 持久化文件路径
    file_path: String,
}

impl KvStore {
    /// 打开或创建一个 KV 存储
    ///
    /// 如果指定路径的文件已经存在，则从文件中加载数据；
    /// 否则创建一个空的存储实例。
    ///
    /// # Errors
    ///
    /// 如果文件存在但无法读取，返回错误。
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = path.to_string();
        let mut store = KvStore {
            data: HashMap::new(),
            file_path,
        };

        // 如果文件存在，尝试加载已有数据
        if Path::new(path).exists() {
            store.load()?;
        }

        Ok(store)
    }

    /// 插入或更新一个键值对
    ///
    /// 使用 HashMap 的 Entry API 实现，避免不必要的查找。
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_kv_store::KvStore;
    /// let mut store = KvStore::open("/tmp/test_set.kv").unwrap();
    /// store.set("key1".into(), "value1".into()).unwrap();
    /// assert_eq!(store.get("key1"), Some(&"value1".to_string()));
    /// # std::fs::remove_file("/tmp/test_set.kv").ok();
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 Entry API：如果 key 已存在则更新 value，否则插入新条目
        // Entry API 只做一次哈希查找，比先 contains_key 再 insert 更高效
        self.data
            .entry(key)
            .and_modify(|v| *v = value.clone())
            .or_insert(value);
        Ok(())
    }

    /// 根据 key 获取 value
    ///
    /// 返回 `Option<&String>`：
    /// - `Some(&value)` 如果 key 存在
    /// - `None` 如果 key 不存在
    ///
    /// 注意：返回的是对内部数据的引用，受 `&self` 的借用生命周期约束。
    /// 调用者不能在持有此引用的同时修改 store。
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// 删除一个键值对
    ///
    /// 返回被删除的 value（如果 key 存在），否则返回 `None`。
    /// 删除操作立即生效在内存中，需要调用 `save()` 才能持久化。
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_kv_store::KvStore;
    /// let mut store = KvStore::open("/tmp/test_remove.kv").unwrap();
    /// store.set("temp".into(), "data".into()).unwrap();
    /// let removed = store.remove("temp").unwrap();
    /// assert_eq!(removed, Some("data".to_string()));
    /// assert_eq!(store.get("temp"), None);
    /// # std::fs::remove_file("/tmp/test_remove.kv").ok();
    /// ```
    pub fn remove(&mut self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(self.data.remove(key))
    }

    /// 列出所有键值对
    ///
    /// 返回按 key 排序后的键值对引用列表。
    /// 排序结果便于阅读和调试，不影响内部存储顺序（HashMap 本身无序）。
    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut entries: Vec<(&String, &String)> = self.data.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }

    /// 将当前内存中的数据持久化到文件
    ///
    /// 写入格式：每行 `key|value`。
    /// 会完全覆盖原文件内容。
    ///
    /// # 设计说明
    /// 当前使用简单的全量写入策略。对于大规模数据（百万级记录），
    /// 这种方式效率较低，可考虑：
    /// - 增量追加（append-only）
    /// - 分段写入（compaction）
    /// - 使用 WAL（Write-Ahead Log）保证崩溃安全
    ///
    /// # Errors
    ///
    /// 如果文件无法创建或写入，返回 IO 错误。
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.file_path)?;
        for (key, value) in &self.data {
            // 格式: key|value
            // 注意：此简单格式不支持 key 或 value 中包含 '|' 或换行符的情况。
            // 生产环境应使用更健壮的序列化格式（JSON、MessagePack 等）。
            writeln!(file, "{}|{}", key, value)?;
        }
        Ok(())
    }

    /// 从文件中加载数据到内存
    ///
    /// 读取每行，按第一个 `|` 分割为 key 和 value。
    /// 如果文件不存在，视为空存储（不报错）。
    ///
    /// # 格式容错
    /// - 空行会被跳过
    /// - 不含 `|` 的行会被跳过并打印警告
    /// - 文件不存在时不报错，保持空的 HashMap
    ///
    /// # Errors
    ///
    /// 如果文件存在但无法读取，返回 IO 错误。
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 如果文件不存在，直接返回（空存储是合法状态）
        if !Path::new(&self.file_path).exists() {
            return Ok(());
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        self.data.clear();

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let trimmed = line.trim();

            // 跳过空行
            if trimmed.is_empty() {
                continue;
            }

            // 按第一个 '|' 分割为 key 和 value
            match trimmed.split_once('|') {
                Some((key, value)) => {
                    self.data.insert(key.to_string(), value.to_string());
                }
                None => {
                    // 格式异常的行：打印警告但不中断加载
                    eprintln!(
                        "警告: 第 {} 行格式不正确（缺少 '|' 分隔符），已跳过: {}",
                        line_num + 1,
                        trimmed
                    );
                }
            }
        }

        Ok(())
    }

    /// 返回存储中键值对的数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 判断存储是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ---------------------------------------------------------------------------
// 测试模块
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// 辅助函数：创建临时文件路径
    fn temp_path(name: &str) -> String {
        format!("/tmp/mini_kv_test_{name}.kv")
    }

    /// 辅助函数：清理测试文件
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_open_new_store() {
        let path = temp_path("open_new");
        cleanup(&path);
        let store = KvStore::open(&path).unwrap();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        cleanup(&path);
    }

    #[test]
    fn test_set_and_get() {
        let path = temp_path("set_get");
        cleanup(&path);
        let mut store = KvStore::open(&path).unwrap();

        store.set("hello".into(), "world".into()).unwrap();
        assert_eq!(store.get("hello"), Some(&"world".to_string()));
        assert_eq!(store.get("nonexistent"), None);

        // 测试更新
        store.set("hello".into(), "rust".into()).unwrap();
        assert_eq!(store.get("hello"), Some(&"rust".to_string()));

        cleanup(&path);
    }

    #[test]
    fn test_remove() {
        let path = temp_path("remove");
        cleanup(&path);
        let mut store = KvStore::open(&path).unwrap();

        store.set("a".into(), "1".into()).unwrap();
        store.set("b".into(), "2".into()).unwrap();

        let removed = store.remove("a").unwrap();
        assert_eq!(removed, Some("1".to_string()));
        assert_eq!(store.get("a"), None);
        assert_eq!(store.len(), 1);

        // 删除不存在的 key
        let removed = store.remove("nonexistent").unwrap();
        assert_eq!(removed, None);

        cleanup(&path);
    }

    #[test]
    fn test_list_sorted() {
        let path = temp_path("list");
        cleanup(&path);
        let mut store = KvStore::open(&path).unwrap();

        store.set("z".into(), "last".into()).unwrap();
        store.set("a".into(), "first".into()).unwrap();
        store.set("m".into(), "middle".into()).unwrap();

        let entries = store.list();
        assert_eq!(entries.len(), 3);
        // 验证按 key 排序
        assert_eq!(entries[0].0, "a");
        assert_eq!(entries[1].0, "m");
        assert_eq!(entries[2].0, "z");

        cleanup(&path);
    }

    #[test]
    fn test_save_and_load() {
        let path = temp_path("save_load");
        cleanup(&path);

        // 创建 store 并写入数据
        {
            let mut store = KvStore::open(&path).unwrap();
            store.set("name".into(), "Alice".into()).unwrap();
            store.set("age".into(), "30".into()).unwrap();
            store.save().unwrap();
        }

        // 重新打开，验证数据持久化
        {
            let store = KvStore::open(&path).unwrap();
            assert_eq!(store.len(), 2);
            assert_eq!(store.get("name"), Some(&"Alice".to_string()));
            assert_eq!(store.get("age"), Some(&"30".to_string()));
        }

        cleanup(&path);
    }

    #[test]
    fn test_load_malformed_lines() {
        let path = temp_path("malformed");
        cleanup(&path);

        // 手动创建一个包含异常格式行的文件
        let content = "valid_key|valid_value\nbad_line_no_pipe\n\nanother|good\n";
        fs::write(&path, content).unwrap();

        let store = KvStore::open(&path).unwrap();
        assert_eq!(store.len(), 2);
        assert_eq!(store.get("valid_key"), Some(&"valid_value".to_string()));
        assert_eq!(store.get("another"), Some(&"good".to_string()));
        // "bad_line_no_pipe" 应该被跳过
        assert_eq!(store.get("bad_line_no_pipe"), None);

        cleanup(&path);
    }

    #[test]
    fn test_empty_and_len() {
        let path = temp_path("empty");
        cleanup(&path);
        let mut store = KvStore::open(&path).unwrap();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);

        store.set("k".into(), "v".into()).unwrap();
        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);

        store.remove("k").unwrap();
        assert!(store.is_empty());

        cleanup(&path);
    }
}
