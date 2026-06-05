//! # 猜数字游戏 (Guessing Game)
//!
//! 一个经典的命令行猜数字游戏，作为 Rust 基础知识的综合练习项目。
//!
//! ## 涉及的关键概念
//!
//! - **extern crate**: 外部 crate 的引入 (`rand`)
//! - **所有权 (Ownership)**: `String` 从 `stdin` 读取时的所有权转移和借用
//! - **Result 处理**: `read_line` 和 `parse` 返回的 `Result` 类型处理
//! - **match 模式匹配**: 处理 `Ordering`、`Result`、范围等多种模式
//! - **loop 循环**: 无限循环直到猜对，`break` 和 `continue` 控制流
//! - **变量可变性**: `mut` 关键字的正确使用
//! - **引用与借用**: `&str` vs `String`，避免不必要的所有权转移

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/// 生成一个 1 到 100（含）之间的随机秘密数字。
///
/// # 实现说明
///
/// 使用 `rand::thread_rng()` 获取线程本地的随机数生成器，
/// 然后通过 `gen_range(1..=100)` 生成 1 到 100（含）之间的随机整数。
/// `1..=100` 使用了 Rust 的 `RangeInclusive` 语法，`=` 表示包含上界。
///
/// # 备选方案
///
/// 如果 `rand` crate 不可用，可以回退到基于系统时间的伪随机数生成：
/// ```ignore
/// use std::time::{SystemTime, UNIX_EPOCH};
/// let seed = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .unwrap()
///     .as_nanos();
/// (seed % 100 + 1) as u32
/// ```
///
/// # 返回值
///
/// 返回 1 到 100 之间的一个 `u32` 随机数。
fn generate_secret() -> u32 {
    // rand::thread_rng() 返回线程本地的随机数生成器
    // gen_range(1..=100) 使用 RangeInclusive 语法，包含上界 100
    rand::thread_rng().gen_range(1..=100)
}

/// 从标准输入读取一行文本。
///
/// 此函数展示了 Rust 中 **所有权 (Ownership)** 的核心概念：
/// 函数内部创建的 `String` 的所有权将被转移给调用者。
///
/// # 所有权详解
///
/// 1. `let mut input = String::new();` — 在堆上分配一个空的 `String`，
///    `input` 变量拥有这块内存的所有权。
/// 2. `io::stdin().read_line(&mut input)` — 以可变引用的方式借用 `input`，
///    将用户输入追加到字符串中。`read_line` 不获取所有权，只修改已有数据。
/// 3. `Ok(input)` — 将所有权转移给调用者。调用 `read_input()` 的代码
///    现在拥有这个 `String`，负责在不再需要时释放它。
///
/// # 返回值
///
/// * `Ok(String)` — 成功读取一行（包含末尾的换行符 `\n`）
/// * `Err(io::Error)` — 读取失败，包括：
///   - `UnexpectedEof`: 用户按下 Ctrl+D (EOF) 终止输入
///   - 其他 I/O 错误
fn read_input() -> io::Result<String> {
    // 打印提示符。使用 print! 而非 println! 让输入在同一行。
    // flush 确保缓冲区被刷新到终端，否则在换行前可能看不到提示符。
    print!("请输入你的猜测 (1-100): ");
    io::stdout().flush()?;

    // 在堆上分配 String — input 拥有其所有权
    let mut input = String::new();

    // read_line 以可变引用 &mut input 借用，追加数据到已有字符串
    // 返回值是 io::Result<usize>，其中 usize 是读取的字节数
    match io::stdin().read_line(&mut input) {
        Ok(0) => {
            // 读取了 0 字节 — 这表示 EOF (Ctrl+D)
            // 构造自定义错误并返回
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "检测到 EOF (Ctrl+D)，游戏结束",
            ))
        }
        Ok(_bytes_read) => {
            // 读取成功，_bytes_read 包含换行符在内的字节数
            // 将 input 的所有权转移给调用者
            Ok(input)
        }
        Err(e) => {
            // I/O 错误，直接向上传播
            Err(e)
        }
    }
}

/// 将用户输入的字符串解析为 `u32` 数字。
///
/// # 参数
///
/// * `input` — 用户输入的字符串**切片引用** (`&str`)
///
/// # 为什么使用 `&str` 而非 `String`？
///
/// 使用不可变引用 `&str` 而不是 `String` 体现了 Rust 借用的核心理念：
/// 1. **不获取所有权**：调用者仍拥有原始 `String`，可以继续使用
/// 2. **零成本抽象**：`&str` 只是指针+长度，不涉及堆分配
/// 3. **灵活性**：可以接受 `String` 的引用、字符串字面量等
///
/// # 返回值
///
/// * `Ok(u32)` — 解析成功
/// * `Err(String)` — 解析失败，包含描述性错误信息
///
/// # 错误处理
///
/// 使用 `match` 匹配 `parse` 返回的 `Result`，配合 `trim()` 处理空白字符。
/// `parse::<u32>()` 中的 `::<u32>` 是 turbofish 语法，指定泛型参数类型。
fn parse_guess(input: &str) -> Result<u32, String> {
    // trim() 去除首尾空白，返回 &str 切片（不分配新内存）
    let trimmed = input.trim();

    // 检查空输入
    if trimmed.is_empty() {
        return Err("输入为空，请输入一个数字".to_string());
    }

    // parse::<u32>() — turbofish 语法明确指定要解析的目标类型
    // 返回 Result<u32, std::num::ParseIntError>
    match trimmed.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!(
            "'{}' 不是一个有效的数字，请输入 1-100 之间的整数",
            trimmed
        )),
    }
}

/// 将玩家的猜测与秘密数字进行比较。
///
/// 使用标准库的 `std::cmp::Ordering` 枚举进行三路比较。
///
/// # 为什么使用 `match` + `Ordering` 而非 `if-else`？
///
/// 1. **穷尽性检查**：编译器强制处理 `Less`、`Equal`、`Greater` 三种情况，
///    如果遗漏任何一个变体，编译就会失败。这消除了运行时遗漏分支的 bug。
/// 2. **语义清晰**：三路比较的含义直接映射到三个枚举变体
/// 3. **模式匹配强大**：可以嵌入守卫条件、解构等高级模式
///
/// # 参数
///
/// * `guess` — 用户猜测的数字
/// * `secret` — 秘密数字
///
/// # 返回值
///
/// * `Ordering::Less` — 猜小了 (guess < secret)
/// * `Ordering::Equal` — 猜对了 (guess == secret)
/// * `Ordering::Greater` — 猜大了 (guess > secret)
fn check_guess(guess: u32, secret: u32) -> Ordering {
    // cmp 方法是 Ord trait 的一部分，u32 实现了 Ord
    // 比较两个值并返回 Ordering 枚举
    guess.cmp(&secret)
}

/// 打印游戏说明和欢迎信息。
///
/// 将 UI 文本集中在一个函数中，便于修改和维护。
/// 如果将来需要国际化 (i18n)，可以集中替换这些字符串。
fn print_welcome() {
    println!("╔══════════════════════════════════════╗");
    println!("║          🎯 猜数字游戏 🎯           ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("游戏规则：");
    println!("  1. 我已经想好了一个 1 到 100 之间的秘密数字");
    println!("  2. 你需要猜出这个数字");
    println!("  3. 每次猜测后我会告诉你是太大还是太小");
    println!("  4. 继续猜直到猜对为止！");
    println!();
    println!("💡 提示：按 Ctrl+D (EOF) 可以随时退出游戏");
    println!();
}

/// 根据尝试次数给出性能评价。
///
/// # 参数
///
/// * `attempts` — 猜对时总共尝试的次数
///
/// # 评价标准
///
/// | 尝试次数 | 评价                         |
/// |---------|------------------------------|
/// | 1       | 一次就中！你是天才！           |
/// | 2-3     | 非常厉害！                    |
/// | 4-7     | 不错的表现！                  |
/// | 8+      | 继续加油！                    |
fn print_performance(attempts: u32) {
    println!();
    match attempts {
        1 => println!("🌟 一次就中！你是天才！理论最优解！"),
        2..=3 => println!("👏 非常厉害！你很快找到了答案。"),
        4..=7 => println!("👍 不错的表现！在合理范围内。"),
        8..=12 => println!("😊 还可以，继续练习会更好。"),
        _ => println!("💪 继续加油！试试二分查找策略？"),
    }
    println!();
}

/// 游戏主循环。
///
/// 这是整个游戏的核心控制逻辑，管理从开始到结束的完整流程。
///
/// # 流程
///
/// 1. 生成 1-100 随机秘密数字
/// 2. 打印欢迎信息
/// 3. 进入 `loop` 主循环：
///    a. 尝试次数 +1
///    b. 读取用户输入（处理 EOF）
///    c. 解析输入为数字（处理非法输入）
///    d. 验证数字范围
///    e. 比较猜测与秘密数字
///    f. 猜对时打印结果并退出
///
/// # 设计决策：为什么使用 `loop` 而不是 `while`？
///
/// - **语义精确**：`loop` 表示"无限循环直到显式退出"，不需要伪造条件变量
/// - **编译器理解**：编译器知道 `loop` 至少执行一次，可以更好地进行某些优化
/// - **表达式能力**：`loop` 可以通过 `break value;` 返回值，作为表达式使用
///   （虽然本游戏没有利用这一点）
/// - **Rust 惯用写法**：当循环条件不是简单的布尔判断时，`loop { ... break; }`
///   比 `while some_flag { ... }` 更加惯用
///
/// # 错误处理策略
///
/// | 错误类型                | 处理方式                     |
/// |------------------------|-----------------------------|
/// | 解析失败（非数字输入）    | 打印错误，`continue` 继续循环 |
/// | 数字超出 1-100 范围     | 打印提示，`continue` 继续循环 |
/// | EOF (Ctrl+D)           | 揭示答案，`return` 退出函数   |
/// | I/O 错误               | 打印错误，`return` 退出函数   |
fn game_loop() {
    // 生成秘密数字 — generate_secret 返回 u32，所有权简单（Copy 类型）
    let secret = generate_secret();

    // 调试模式下显示答案，方便测试
    #[cfg(debug_assertions)]
    println!("[调试] 秘密数字是: {}\n", secret);

    print_welcome();

    // attempts 跟踪尝试次数，初始为 0
    let mut attempts: u32 = 0;

    // loop 表达"持续循环直到显式 break"
    loop {
        attempts += 1;

        // ---------- 步骤 1: 读取输入 ----------
        // read_input() 返回 io::Result<String>
        // 使用 match 处理 Result — Rust 的错误处理核心模式
        let input = match read_input() {
            Ok(s) => s, // 成功：s 是 String，所有权转移给 input
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    // 用户按了 Ctrl+D，优雅退出
                    println!("\n👋 检测到 EOF (Ctrl+D)，游戏结束。");
                    println!("秘密数字是: {}", secret);
                    println!("你进行了 {} 次尝试。", attempts.saturating_sub(1));
                } else {
                    eprintln!("❌ 读取输入时出错: {}", e);
                }
                return; // 退出 game_loop 函数
            }
        };

        // ---------- 步骤 2: 解析输入 ----------
        // parse_guess 接受 &str（字符串切片引用），不获取 String 的所有权
        // 这意味着 input 变量在此之后仍然可用（虽然本例中不再需要）
        let guess: u32 = match parse_guess(&input) {
            Ok(num) => num,
            Err(msg) => {
                println!("❌ {}", msg);
                println!("请确保输入的是 1 到 100 之间的整数。");
                continue; // 跳过本次循环，重新开始
            }
        };

        // ---------- 步骤 3: 验证范围 ----------
        if !(1..=100).contains(&guess) {
            println!("❌ 数字超出范围！请输入 1 到 100 之间的整数。");
            continue;
        }

        // ---------- 步骤 4: 比较猜测 ----------
        // check_guess 返回 Ordering 枚举
        // match 必须覆盖所有三个变体，编译器会检查
        match check_guess(guess, secret) {
            Ordering::Less => {
                // 提示偏小，给出搜索方向建议
                let hint = if guess < 10 {
                    "试试大一点的数"
                } else {
                    "往上猜"
                };
                println!("📉 太小了！{}。", hint);
            }
            Ordering::Greater => {
                let hint = if guess > 90 {
                    "试试小一点的数"
                } else {
                    "往下猜"
                };
                println!("📈 太大了！{}。", hint);
            }
            Ordering::Equal => {
                // 猜对了！打印结果并退出循环
                println!();
                println!("╔══════════════════════════════════════╗");
                println!("║          🎉 恭喜你猜对了！ 🎉        ║");
                println!("╚══════════════════════════════════════╝");
                println!();
                println!("秘密数字: {}", secret);
                println!("总尝试次数: {}", attempts);
                print_performance(attempts);
                break; // 退出 loop 循环
            }
        }
    }
}

/// 程序的入口点。
///
/// `main` 函数保持简洁，只负责调用顶层函数。
/// 这种设计遵循单一职责原则：`main` 负责启动，`game_loop` 负责游戏逻辑。
///
/// `main` 不返回 `Result` 是因为所有错误都在 `game_loop` 内部处理了，
/// 用户不会看到 Rust 的 panic 信息。
fn main() {
    game_loop();
}
