//! # cli_text_search 命令行入口
//!
//! 本文件为 `cli_text_search` 库的命令行接口。
//! 采用 library-first 设计模式，所有核心逻辑位于 `lib.rs` 中，
//! `main.rs` 仅负责：
//! 1. 收集命令行参数
//! 2. 构建搜索配置
//! 3. 调用库函数执行搜索
//! 4. 处理和展示错误信息
//!
//! ## 用法
//!
//! ```text
//! cargo run -- <查询字符串> <文件路径> [-i|--case-insensitive]
//! ```
//!
//! ## 示例
//!
//! ```text
//! cargo run -- "hello" poem.txt
//! cargo run -- "Rust" src/main.rs -i
//! ```

use std::env;
use std::process;

use cli_text_search::SearchConfig;

fn main() {
    // 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 尝试从命令行参数构建配置
    let config = SearchConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("解析命令行参数时出错: {}", err);
        eprintln!(
            "用法: {} <查询字符串> <文件路径> [-i|--case-insensitive]",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("cli_text_search")
        );
        eprintln!("示例:");
        eprintln!(
            "  {} \"hello\" poem.txt",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("cli_text_search")
        );
        eprintln!(
            "  {} \"Rust\" src/main.rs -i",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("cli_text_search")
        );
        process::exit(1);
    });

    // 显示搜索参数
    eprintln!(
        "搜索 '{}' 在文件 '{}' 中（大小写{}敏感）...",
        config.query,
        config.file_path,
        if config.case_sensitive { "" } else { "不" }
    );

    // 执行搜索
    if let Err(e) = cli_text_search::run(config) {
        eprintln!("运行出错: {}", e);
        process::exit(1);
    }
}
