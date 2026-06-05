use mini_kv_store::KvStore;
use std::io::{self, BufRead, Write};

/// 命令帮助文本
const HELP: &str = r"
可用命令:
  set <key> <value>    设置键值对
  get <key>            获取键对应的值
  remove <key>         删除键值对
  list                 列出所有键值对
  save                 持久化到磁盘
  help                 显示此帮助
  quit / exit          退出（自动保存）
";

fn main() {
    // 打开或创建 KV 存储
    let store_path = "data.kv";
    let mut store = match KvStore::open(store_path) {
        Ok(s) => {
            println!("已打开存储文件: {store_path}");
            if !s.is_empty() {
                println!("从文件中加载了 {} 条记录", s.len());
            } else {
                println!("存储为空，等待输入数据");
            }
            s
        }
        Err(e) => {
            eprintln!("无法打开存储文件: {e}");
            std::process::exit(1);
        }
    };

    println!("{HELP}");
    println!("输入命令 (输入 quit 或按 Ctrl+D 退出):\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // 打印提示符
        print!("kv> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\n收到 EOF，正在退出...");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("读取输入失败: {e}");
                break;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // 解析命令 —— 按空白字符分割，最多取3部分
        // 格式: <command> <arg1> <arg2...>
        let mut parts = line.splitn(3, char::is_whitespace);
        let command = parts.next().unwrap_or("").to_lowercase();
        let arg1 = parts.next().unwrap_or("").to_string();
        let arg2 = parts.next().unwrap_or("").to_string();

        match command.as_str() {
            "set" => {
                if arg1.is_empty() || arg2.is_empty() {
                    println!("用法: set <key> <value>");
                    continue;
                }
                match store.set(arg1.clone(), arg2.clone()) {
                    Ok(()) => println!("OK: {} = {}", arg1, arg2),
                    Err(e) => eprintln!("错误: {e}"),
                }
            }

            "get" => {
                if arg1.is_empty() {
                    println!("用法: get <key>");
                    continue;
                }
                match store.get(&arg1) {
                    Some(value) => println!("{}", value),
                    None => println!("(nil) — key '{}' 不存在", arg1),
                }
            }

            "remove" => {
                if arg1.is_empty() {
                    println!("用法: remove <key>");
                    continue;
                }
                match store.remove(&arg1) {
                    Ok(Some(value)) => println!("已删除: {} = {}", arg1, value),
                    Ok(None) => println!("(nil) — key '{}' 不存在", arg1),
                    Err(e) => eprintln!("错误: {e}"),
                }
            }

            "list" => {
                let entries = store.list();
                if entries.is_empty() {
                    println!("(空)");
                } else {
                    println!("共 {} 条记录:", entries.len());
                    for (key, value) in &entries {
                        println!("  {} = {}", key, value);
                    }
                }
            }

            "save" => match store.save() {
                Ok(()) => println!("已保存 {} 条记录到 {}", store.len(), store_path),
                Err(e) => eprintln!("保存失败: {e}"),
            },

            "help" => {
                println!("{HELP}");
            }

            "quit" | "exit" => {
                println!("正在退出...");
                break;
            }

            _ => {
                println!("未知命令: '{}'。输入 help 查看可用命令。", command);
            }
        }
    }

    // 退出前自动保存
    match store.save() {
        Ok(()) => println!("数据已自动保存到 {store_path}（{} 条记录）", store.len()),
        Err(e) => eprintln!("自动保存失败: {e}"),
    }

    println!("再见！");
}
