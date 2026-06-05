//! # todo_cli - 命令行待办事项管理器
//!
//! 使用 `clap` 提供子命令界面来管理待办事项。
//!
//! ## 用法
//!
//! ```text
//! # 添加待办事项
//! cargo run -- add "学习 Rust"
//! cargo run -- add "写单元测试"
//!
//! # 列出所有待办事项
//! cargo run -- list
//!
//! # 完成一个待办事项
//! cargo run -- complete 1
//!
//! # 删除一个待办事项
//! cargo run -- delete 2
//!
//! # 列出未完成的待办事项
//! cargo run -- pending
//!
//! # 列出已完成的待办事项
//! cargo run -- done
//! ```

use clap::{Parser, Subcommand};
use std::process;

use todo_cli::TodoList;

/// 默认的持久化文件名
const DEFAULT_FILE: &str = "todos.json";

/// 命令行待办事项管理器
///
/// 使用 JSON 文件存储数据，支持增删改查操作。
#[derive(Parser)]
#[command(name = "todo", version, about = "命令行待办事项管理器", long_about = None)]
struct Cli {
    /// 指定 JSON 存储文件的路径（默认为 todos.json）
    #[arg(short, long, default_value = DEFAULT_FILE)]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加一个新的待办事项
    Add {
        /// 待办事项的标题内容
        title: Vec<String>,
    },
    /// 列出所有待办事项（默认按 ID 排序）
    List,
    /// 将指定 ID 的条目标记为已完成
    Complete {
        /// 要完成的条目 ID
        id: u32,
    },
    /// 删除指定 ID 的条目
    Delete {
        /// 要删除的条目 ID
        id: u32,
    },
    /// 列出所有未完成的待办事项
    Pending,
    /// 列出所有已完成的待办事项
    Done,
}

fn main() {
    let cli = Cli::parse();

    // 加载或创建待办列表
    let mut list = match TodoList::load(&cli.file) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("错误: 无法加载文件 \"{}\": {}", cli.file, e);
            process::exit(1);
        }
    };

    // 分发子命令
    let result = match &cli.command {
        Commands::Add { title } => {
            if title.is_empty() {
                eprintln!("错误: 请提供待办事项的标题");
                eprintln!("用法: todo add <标题>");
                process::exit(1);
            }
            let combined = title.join(" ");
            let item = list.add(combined);
            println!("✅ 已添加待办事项:");
            print_item(&item);
            // 添加后自动保存
            if let Err(e) = list.save() {
                eprintln!("警告: 保存失败: {}", e);
            }
            Ok(())
        }
        Commands::List => {
            if list.items.is_empty() {
                println!("📋 暂无待办事项。使用 `todo add <标题>` 来添加一个。");
            } else {
                println!("📋 全部待办事项 (共 {} 条):", list.items.len());
                println!("{}", "─".repeat(50));
                for item in list.list() {
                    print_item(item);
                }
                println!("{}", "─".repeat(50));
                let pending_count = list.list_pending().len();
                let done_count = list.list_completed().len();
                println!("统计: {} 条未完成, {} 条已完成", pending_count, done_count);
            }
            Ok(())
        }
        Commands::Complete { id } => match list.complete(*id) {
            Ok(()) => {
                println!("✅ 条目 #{} 已标记为完成", id);
                if let Err(e) = list.save() {
                    eprintln!("警告: 保存失败: {}", e);
                }
                Ok(())
            }
            Err(msg) => Err(msg),
        },
        Commands::Delete { id } => match list.delete(*id) {
            Ok(()) => {
                println!("🗑️  条目 #{} 已删除", id);
                if let Err(e) = list.save() {
                    eprintln!("警告: 保存失败: {}", e);
                }
                Ok(())
            }
            Err(msg) => Err(msg),
        },
        Commands::Pending => {
            let pending = list.list_pending();
            if pending.is_empty() {
                println!("🎉 所有待办事项都已完成！");
            } else {
                println!("📝 未完成的待办事项 (共 {} 条):", pending.len());
                println!("{}", "─".repeat(50));
                for item in &pending {
                    print_item(item);
                }
                println!("{}", "─".repeat(50));
            }
            Ok(())
        }
        Commands::Done => {
            let completed = list.list_completed();
            if completed.is_empty() {
                println!("📭 暂无已完成的待办事项。");
            } else {
                println!("✅ 已完成的待办事项 (共 {} 条):", completed.len());
                println!("{}", "─".repeat(50));
                for item in &completed {
                    print_item(item);
                }
                println!("{}", "─".repeat(50));
            }
            Ok(())
        }
    };

    // 处理命令执行结果
    if let Err(msg) = result {
        eprintln!("错误: {}", msg);
        process::exit(1);
    }
}

/// 以友好的格式打印单个待办事项条目。
fn print_item(item: &todo_cli::TodoItem) {
    let status = if item.completed { "[✅]" } else { "[ ]" };
    println!("  {} #{:<4} {}", status, item.id, item.title);
}
