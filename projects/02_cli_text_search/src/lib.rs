//! # cli_text_search
//!
//! 一个简化的命令行文本搜索工具，模拟 `grep` 的核心功能。
//!
//! 本库采用 library-first 设计理念：将核心搜索逻辑封装在库中，
//! 命令行入口 (`main.rs`) 仅负责参数解析和结果展示。
//!
//! ## 主要功能
//!
//! - 大小写敏感的文本搜索
//! - 大小写不敏感的文本搜索
//! - 命令行参数解析
//! - 环境变量配置支持
//!
//! ## 快速开始
//!
//! ```rust
//! use cli_text_search::{SearchConfig, run};
//!
//! let args = vec![
//!     "program".to_string(),
//!     "hello".to_string(),
//!     "test.txt".to_string(),
//! ];
//! let config = SearchConfig::new(&args).unwrap();
//! // run(config).unwrap();
//! ```

use std::env;
use std::error::Error;
use std::fs;

/// 搜索配置，封装一次搜索操作所需的所有参数。
///
/// 包含三个核心字段：
/// - `query`: 要搜索的文本模式
/// - `file_path`: 被搜索的目标文件路径
/// - `case_sensitive`: 是否区分英文字母的大小写
///
/// # 示例
///
/// ```
/// let config = cli_text_search::SearchConfig {
///     query: "Rust".to_string(),
///     file_path: "example.txt".to_string(),
///     case_sensitive: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// 搜索查询字符串
    pub query: String,
    /// 目标文件的路径
    pub file_path: String,
    /// 是否大小写敏感（`true` 表示区分大小写）
    pub case_sensitive: bool,
}

impl SearchConfig {
    /// 从命令行参数构建 `SearchConfig`。
    ///
    /// 解析传入的参数切片，提取查询字符串、文件路径和大小写设置。
    /// 支持 `-i` 或 `--case-insensitive` 标志来关闭大小写敏感。
    ///
    /// # 参数
    ///
    /// * `args` - 命令行参数切片，`args[0]` 通常为程序名。
    ///   `args[1]` 为查询字符串，`args[2]` 为文件路径。
    ///   可选的 `args[3]` 可以是 `-i` 或 `--case-insensitive` 来启用大小写不敏感模式。
    ///
    /// # 错误
    ///
    /// 当参数数量不足时，返回描述性的错误字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// let args = vec![
    ///     "program".to_string(),
    ///     "hello".to_string(),
    ///     "test.txt".to_string(),
    /// ];
    /// let config = cli_text_search::SearchConfig::new(&args).unwrap();
    /// assert_eq!(config.query, "hello");
    /// assert_eq!(config.file_path, "test.txt");
    /// assert!(config.case_sensitive);
    /// ```
    ///
    /// 使用大小写不敏感标志:
    ///
    /// ```
    /// let args = vec![
    ///     "program".to_string(),
    ///     "rust".to_string(),
    ///     "notes.txt".to_string(),
    ///     "-i".to_string(),
    /// ];
    /// let config = cli_text_search::SearchConfig::new(&args).unwrap();
    /// assert!(!config.case_sensitive);
    /// ```
    pub fn new(args: &[String]) -> Result<SearchConfig, String> {
        if args.len() < 3 {
            return Err(format!(
                "参数不足：需要至少 2 个参数（查询字符串 和 文件路径），但只收到了 {} 个。\n\
                 用法: {} <查询字符串> <文件路径> [-i|--case-insensitive]",
                args.len().saturating_sub(1),
                args.first().map(|s| s.as_str()).unwrap_or("program")
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // 检查是否有 -i 或 --case-insensitive 标志
        let case_sensitive = if args.len() > 3 {
            let flag = &args[3];
            !(flag == "-i" || flag == "--case-insensitive")
        } else {
            true
        };

        Ok(SearchConfig {
            query,
            file_path,
            case_sensitive,
        })
    }

    /// 从环境变量构建 `SearchConfig`。
    ///
    /// 读取以下环境变量：
    /// - `SEARCH_QUERY`: 搜索查询字符串（必需）
    /// - `SEARCH_FILE`: 目标文件路径（必需）
    /// - `CASE_INSENSITIVE`: 如果设置为 `"1"`、`"true"`、`"yes"` 或 `"on"`，
    ///   则关闭大小写敏感（可选，默认为大小写敏感）
    ///
    /// # 错误
    ///
    /// 当必需的环境变量未设置时，返回错误字符串。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// // 设置环境变量后调用:
    /// // $ export SEARCH_QUERY="error"
    /// // $ export SEARCH_FILE="app.log"
    /// // $ export CASE_INSENSITIVE=1
    /// let config = cli_text_search::SearchConfig::from_env().unwrap();
    /// assert_eq!(config.query, "error");
    /// assert!(!config.case_sensitive);
    /// ```
    pub fn from_env() -> Result<SearchConfig, String> {
        let query = env::var("SEARCH_QUERY")
            .map_err(|_| "环境变量 SEARCH_QUERY 未设置。请设置要搜索的查询字符串。".to_string())?;

        let file_path = env::var("SEARCH_FILE")
            .map_err(|_| "环境变量 SEARCH_FILE 未设置。请设置目标文件路径。".to_string())?;

        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(val) => {
                let val_lower = val.to_lowercase();
                !(val_lower == "1"
                    || val_lower == "true"
                    || val_lower == "yes"
                    || val_lower == "on")
            }
            Err(_) => true, // 环境变量未设置，默认大小写敏感
        };

        Ok(SearchConfig {
            query,
            file_path,
            case_sensitive,
        })
    }
}

/// 执行搜索的完整流程：读取文件、执行搜索、输出结果。
///
/// 此函数编排了整个搜索流程：
/// 1. 读取指定文件的全部内容
/// 2. 根据 `config.case_sensitive` 选择搜索策略
/// 3. 将匹配的行逐行打印到标准输出
///
/// # 参数
///
/// * `config` - 搜索配置，包含查询字符串、文件路径和大小写设置
///
/// # 错误
///
/// 当文件读取失败时，通过 `?` 操作符向上传播 `std::io::Error`。
/// 返回类型使用 `Box<dyn std::error::Error>` 以支持多种错误类型。
///
/// # 示例
///
/// ```no_run
/// let config = cli_text_search::SearchConfig {
///     query: "fn".to_string(),
///     file_path: "src/main.rs".to_string(),
///     case_sensitive: true,
/// };
/// if let Err(e) = cli_text_search::run(config) {
///     eprintln!("运行出错: {}", e);
/// }
/// ```
pub fn run(config: SearchConfig) -> Result<(), Box<dyn Error>> {
    // 读取文件内容，? 操作符在出错时自动将 io::Error 转换为 Box<dyn Error>
    let contents = fs::read_to_string(&config.file_path).map_err(|e| {
        Box::<dyn Error>::from(format!("无法读取文件 '{}': {}", config.file_path, e))
    })?;

    // 根据配置选择搜索策略
    let results = if config.case_sensitive {
        search(&config.query, &contents, true)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // 输出搜索结果
    if results.is_empty() {
        println!("未找到包含 '{}' 的行。", config.query);
    } else {
        println!(
            "在文件 '{}' 中找到 {} 行包含 '{}' 的结果:\n",
            config.file_path,
            results.len(),
            config.query
        );
        for (i, line) in results.iter().enumerate() {
            println!("  {}: {}", i + 1, line);
        }
    }

    Ok(())
}

/// 在文本内容中搜索包含查询字符串的行（核心搜索逻辑）。
///
/// 遍历 `contents` 的每一行，返回所有包含 `query` 的行的引用。
/// 返回的 `Vec<&str>` 中的每个元素都直接引用 `contents` 中的原始行数据，
/// 不进行任何内存拷贝，因此返回值与输入参数 `contents` 共享同一生命周期 `'a`。
///
/// 如果 `case_sensitive` 为 `false`，则委托给 [`search_case_insensitive`] 函数。
///
/// # 参数
///
/// * `query` - 要搜索的查询字符串。**空查询将直接返回空结果**（防御性设计，
///   避免匹配所有行）。
/// * `contents` - 被搜索的文本内容，通常来自文件读取。
/// * `case_sensitive` - `true` 时执行大小写敏感的精确匹配；
///   `false` 时委托给 [`search_case_insensitive`]。
///
/// # 返回值
///
/// `Vec<&'a str>` — 匹配行的引用集合。生命周期 `'a` 与 `contents` 参数绑定，
/// 确保返回的引用不会超过被引用数据的存活范围。这是 Rust 借用检查器的核心保障。
///
/// # 示例
///
/// 大小写敏感搜索：
///
/// ```
/// let contents = "Rust 编程语言\nrust 是系统编程语言\n学习 Rust\nJava";
/// let result = cli_text_search::search("Rust", contents, true);
/// assert_eq!(result, vec!["Rust 编程语言", "学习 Rust"]);
/// // 注意："rust 是系统编程语言" 因为首字母小写而未匹配
/// ```
///
/// 大小写不敏感搜索（通过 flag）：
///
/// ```
/// let contents = "Rust\nrust\nRUST\nJava";
/// let result = cli_text_search::search("rust", contents, false);
/// assert_eq!(result.len(), 3);
/// ```
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    // 防御性编程：空查询不应匹配所有行
    if query.is_empty() {
        return Vec::new();
    }

    if case_sensitive {
        // 大小写敏感：使用标准的字符串匹配
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    } else {
        // 委托给大小写不敏感的搜索函数
        search_case_insensitive(query, contents)
    }
}

/// 大小写不敏感的文本搜索。
///
/// 将 `query` 和 `contents` 的每一行都转换为小写后再进行比较，
/// 因此搜索 "rust" 可以匹配 "Rust"、"RUST"、"rust" 等任何大小写组合。
///
/// 注意：`query.to_lowercase()` 和 `line.to_lowercase()` 都会创建新的 `String`，
/// 但返回的引用仍然指向原始的 `contents` 数据，不会拷贝匹配行本身。
/// 这体现了 Rust 的所有权模型：在函数内部创建临时数据（小写版本），
/// 但返回引用指向外部数据（原始内容）。
///
/// # 参数
///
/// * `query` - 要搜索的查询字符串。空查询返回空结果。
/// * `contents` - 被搜索的文本内容，生命周期为 `'a`。
///
/// # 返回值
///
/// `Vec<&'a str>` — 匹配行的引用集合。
///
/// # 示例
///
/// ```
/// let contents = "Hello World\nHELLO Rust\nhello everyone\nGoodbye";
/// let result = cli_text_search::search_case_insensitive("hello", contents);
/// assert_eq!(result, vec!["Hello World", "HELLO Rust", "hello everyone"]);
/// // 三行都匹配：Hello → hello, HELLO → hello, hello → hello
/// ```
///
/// 验证大小写不敏感:
///
/// ```
/// let contents = "Rust\nrust\nRUST\nrUsT";
/// let result = cli_text_search::search_case_insensitive("rust", contents);
/// assert_eq!(result.len(), 4);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 防御性编程
    if query.is_empty() {
        return Vec::new();
    }

    // 将查询字符串转为小写一次（而非在每一行重复转换），提高效率
    let query_lower = query.to_lowercase();

    contents
        .lines()
        .filter(|line| {
            // 对每一行创建小写副本进行比较
            // to_lowercase() 返回 String，contains 检查子串匹配
            line.to_lowercase().contains(&query_lower)
        })
        .collect()
}

// ============================================================================
// 单元测试（放置在库文件中是 Rust 的惯用做法）
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构造函数：参数不足
    #[test]
    fn test_new_not_enough_args() {
        let args = vec!["program".to_string()];
        let result = SearchConfig::new(&args);
        assert!(result.is_err());
    }

    /// 测试构造函数：基本参数解析
    #[test]
    fn test_new_basic() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        let config = SearchConfig::new(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
        assert!(config.case_sensitive);
    }

    /// 测试构造函数：大小写不敏感标志 -i
    #[test]
    fn test_new_case_insensitive_short() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
            "-i".to_string(),
        ];
        let config = SearchConfig::new(&args).unwrap();
        assert!(!config.case_sensitive);
    }

    /// 测试构造函数：大小写不敏感标志 --case-insensitive
    #[test]
    fn test_new_case_insensitive_long() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
            "--case-insensitive".to_string(),
        ];
        let config = SearchConfig::new(&args).unwrap();
        assert!(!config.case_sensitive);
    }

    /// 测试 search 函数：基本大小写敏感搜索
    #[test]
    fn test_search_case_sensitive() {
        let contents = "\
Rust 是一门系统编程语言
rust 是 Rust 的小写形式
学习 Rust 很有趣";
        let result = search("Rust", contents, true);
        assert_eq!(result.len(), 3);
        assert!(result[0].contains("Rust"));
        assert!(result[1].contains("Rust"));
        assert!(result[2].contains("Rust"));
        // 三行都包含大写的 "Rust"：
        // - 行1: "Rust 是一门系统编程语言" — 开头就是 "Rust"
        // - 行2: "rust 是 Rust 的小写形式" — 包含第二个 "Rust"（大写）
        // - 行3: "学习 Rust 很有趣" — 包含 "Rust"
    }

    /// 测试 search_case_insensitive 函数
    #[test]
    fn test_search_case_insensitive_variants() {
        let contents = "Rust\nrust\nRUST\nrUsT\nJava";
        let result = search_case_insensitive("rust", contents);
        assert_eq!(result.len(), 4);
    }

    /// 测试空查询
    #[test]
    fn test_empty_query() {
        let contents = "line1\nline2\nline3";
        let result = search("", contents, true);
        assert!(result.is_empty());
        let result_ci = search_case_insensitive("", contents);
        assert!(result_ci.is_empty());
    }

    /// 测试无匹配结果
    #[test]
    fn test_no_matches() {
        let contents = "apple\nbanana\ncherry";
        let result = search("zebra", contents, true);
        assert!(result.is_empty());
    }
}
