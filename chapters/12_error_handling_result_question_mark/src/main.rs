#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unnecessary_literal_unwrap,
    rustdoc::invalid_html_tags
)]
// ============================================================================
// 第十二章：错误处理 —— Result、panic! 与 ? 运算符
// ============================================================================
// 本程序演示 Rust 错误处理的核心机制：
//   - Result<T, E> 用于可恢复错误
//   - panic! 用于不可恢复错误
//   - ? 运算符简化错误传播
//   - 自定义错误类型与 From trait 自动转换
// ============================================================================

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

// ----------------------------------------------------------------------------
// 1. 自定义统计结果结构体
// ----------------------------------------------------------------------------

/// 文件数值的统计结果
#[derive(Debug)]
struct StatsResult {
    /// 有效数字的总数
    count: usize,
    /// 所有数字的和
    sum: i64,
    /// 平均值（可能为 None，当 count == 0 时）
    average: Option<f64>,
    /// 最小值
    min: i32,
    /// 最大值
    max: i32,
}

impl fmt::Display for StatsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "统计结果:\n\
             ┌──────────────┬────────────┐\n\
             │ 总数量       │ {:>10} │\n\
             │ 总和         │ {:>10} │\n\
             │ 平均值       │ {:>10} │\n\
             │ 最小值       │ {:>10} │\n\
             │ 最大值       │ {:>10} │\n\
             └──────────────┴────────────┘",
            self.count,
            self.sum,
            self.average
                .map_or("N/A".to_string(), |v| format!("{:.2}", v)),
            self.min,
            self.max,
        )
    }
}

// ----------------------------------------------------------------------------
// 2. 自定义错误类型
//    Rust 鼓励使用 enum 来定义领域相关的错误类型。
// ----------------------------------------------------------------------------

/// 文件统计过程中可能出现的所有错误
#[derive(Debug)]
enum FileStatsError {
    /// I/O 错误 —— 包装标准库的 std::io::Error
    IoError(io::Error),
    /// 解析错误 —— 某一行不是有效的数字
    ParseError(String),
    /// 空输入 —— 文件中没有任何有效数据
    EmptyInput,
}

// --- 2a. 实现 Display trait（用户友好的错误信息） ---
impl fmt::Display for FileStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileStatsError::IoError(e) => write!(f, "I/O 错误: {}", e),
            FileStatsError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            FileStatsError::EmptyInput => write!(f, "输入为空: 文件中没有有效数据"),
        }
    }
}

// --- 2b. 实现 Error trait（最小实现，使它能被向上传播） ---
// Rust 2024 中 Error trait 的 source() 方法已有默认实现，只需 impl Error 即可。
impl Error for FileStatsError {
    // source() 返回底层错误，使调用者可以用 match 或 downcast 检查
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileStatsError::IoError(e) => Some(e),
            FileStatsError::ParseError(_) => None,
            FileStatsError::EmptyInput => None,
        }
    }
}

// --- 2c. 实现 From<std::io::Error> 进行自动转换 ---
// 有了这个实现，? 运算符在遇到 io::Error 时会自动调用 .into()
// 将其转换为 FileStatsError::IoError，不需要手动 .map_err()。
impl From<io::Error> for FileStatsError {
    fn from(error: io::Error) -> Self {
        FileStatsError::IoError(error)
    }
}

// ----------------------------------------------------------------------------
// 3. 核心函数
// ----------------------------------------------------------------------------

/// 读取文件内容，传播 I/O 错误。
/// 返回值使用标准库的 io::Error —— 这个错误会在上层被 From trait 自动转换。
fn read_file_content(path: &str) -> Result<String, io::Error> {
    // fs::read_to_string 直接返回 Result<String, io::Error>
    // 如果文件不存在（ENOENT），这里会返回 Err(io::Error)
    fs::read_to_string(path)
}

/// 将文件内容按行解析为 i32 数字列表。
/// 返回 Result<Vec<i32>, String> —— 自定义错误字符串。
fn parse_numbers(content: &str) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // 跳过空行和注释行（以 # 开头）
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue; // 空行/注释行不算错误，直接跳过
        }

        // 尝试解析为 i32
        match trimmed.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                // 返回自定义错误信息，包含行号和具体内容
                return Err(format!(
                    "第 {} 行不是有效的整数: \"{}\"",
                    line_num + 1,
                    trimmed
                ));
            }
        }
    }

    Ok(numbers)
}

/// 根据一组数字计算出统计结果。
/// 参数 numbers 可以为空切片，此时返回的 StatsResult 中 count = 0，
/// min/max 设为 0，average 为 None。
fn compute_stats(numbers: &[i32]) -> StatsResult {
    if numbers.is_empty() {
        return StatsResult {
            count: 0,
            sum: 0,
            average: None,
            min: 0,
            max: 0,
        };
    }

    let count = numbers.len();
    let sum: i64 = numbers.iter().map(|&n| n as i64).sum();
    let average = Some(sum as f64 / count as f64);
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    StatsResult {
        count,
        sum,
        average,
        min,
        max,
    }
}

// ----------------------------------------------------------------------------
// 4. 主分析流程 —— 使用 ? 运算符（推荐方式）
// ----------------------------------------------------------------------------
// ? 运算符的工作方式：
//   - 当 Result 是 Ok(v) 时，提取出 v 并继续执行
//   - 当 Result 是 Err(e) 时，**立即从当前函数返回** Err(e.into())
//   - .into() 会调用 From trait 将错误类型转换为函数的返回错误类型
//
// 因此下面每个 ? 都等价于：
//   let value = match expr {
//       Ok(v) => v,
//       Err(e) => return Err(e.into()),
//   };

/// 执行完整的分析流程（使用 ? 运算符进行错误传播）
fn run_analysis(path: &str) -> Result<(), FileStatsError> {
    println!("═══════════════════════════════════════════");
    println!("  Rust 错误处理演示 —— ? 运算符版");
    println!("═══════════════════════════════════════════");
    println!();
    println!("📂 正在读取文件: {}", path);

    // ? 在这里做了两件事：
    //   1. read_file_content 返回 io::Error
    //   2. ? 调用 FileStatsError::from(io_error)，自动转换为我们的错误类型
    let content = read_file_content(path)?;

    println!("✅ 文件读取成功 ({} 字节)", content.len());

    // 解析数字时返回的是 Result<Vec<i32>, String>，
    // 这里无法使用 From 自动转换（String ≠ FileStatsError），
    // 所以需要手动使用 .map_err() 将 String 转换为 FileStatsError::ParseError
    let numbers = parse_numbers(&content).map_err(FileStatsError::ParseError)?;

    if numbers.is_empty() {
        // 提前返回自定义错误
        return Err(FileStatsError::EmptyInput);
    }

    println!("✅ 解析出 {} 个有效数字", numbers.len());

    // 计算统计信息（这个函数不会失败，返回纯值）
    let stats = compute_stats(&numbers);

    // 打印结果
    println!();
    println!("{}", stats);

    Ok(())
}

// ----------------------------------------------------------------------------
// 5. 手动 match 版本 —— 对比展示"长写法"
// ----------------------------------------------------------------------------
// 在 ? 出现之前，Rust 程序员需要这样写。理解它有助于掌握 ? 的本质。

/// 执行完整的分析流程（使用 match 手动处理每个 Result）
fn run_analysis_manual(path: &str) -> Result<(), FileStatsError> {
    println!("═══════════════════════════════════════════");
    println!("  Rust 错误处理演示 —— 手动 match 版");
    println!("═══════════════════════════════════════════");
    println!();

    // Step 1: 读取文件 —— 手动 match
    let content = match read_file_content(path) {
        Ok(text) => text,
        Err(e) => {
            // 手动转换为 FileStatsError
            return Err(FileStatsError::IoError(e));
        }
    };

    println!("✅ 文件读取成功 ({} 字节)", content.len());

    // Step 2: 解析数字 —— 手动 match
    let numbers = match parse_numbers(&content) {
        Ok(nums) => nums,
        Err(msg) => {
            return Err(FileStatsError::ParseError(msg));
        }
    };

    if numbers.is_empty() {
        return Err(FileStatsError::EmptyInput);
    }

    println!("✅ 解析出 {} 个有效数字", numbers.len());

    let stats = compute_stats(&numbers);
    println!();
    println!("{}", stats);

    Ok(())
}

// ----------------------------------------------------------------------------
// 6. 演示 panic!、unwrap()、expect()、unwrap_or() 等工具函数
// ----------------------------------------------------------------------------

/// 演示不可恢复错误 —— panic!
///
/// Rust 中数组越界、除零等操作会触发 panic。
/// 设置环境变量 RUST_BACKTRACE=1 可以查看调用栈：
///   $ RUST_BACKTRACE=1 cargo run
///
/// panic! 宏会在运行时展开调用栈（unwinding），释放已获取的资源，
/// 然后终止当前线程（或整个进程）。
fn demo_panic() {
    println!("\n--- panic! 演示 ---");

    // 情况1: 主动调用 panic! 宏
    // panic!("这是一个主动触发的不可恢复错误！");
    // 上面这行被注释掉了，取消注释会立即终止程序。

    // 情况2: 数组越界 —— 编译器在运行时会插入边界检查
    let arr = [10, 20, 30];
    println!("数组: {:?}", arr);
    println!("arr[1] = {} (合法访问)", arr[1]);

    // 下面这行取消注释会导致 panic: index out of bounds
    // println!("arr[5] = {} (越界访问)", arr[5]);

    // 安全的替代方案：使用 .get() 返回 Option<&T>
    match arr.get(5) {
        Some(val) => println!("arr[5] = {}", val),
        None => println!("arr[5] 不存在 —— 使用 .get() 安全处理了越界"),
    }
}

/// 演示 unwrap() vs expect() 的区别
///
/// - unwrap(): 提取 Ok 值，如果是 Err 则 panic。信息量少。
/// - expect("msg"): 提取 Ok 值，如果是 Err 则 panic 并打印自定义消息。
///   比 unwrap() 更有利于调试。
///
/// 何时使用（谨慎！）：
///   ✅ 原型开发、测试、示例代码 —— 快速迭代
///   ✅ 已知不可能失败的情况 —— 比如你已经手动检查过
///   ❌ 生产代码中应尽量使用 ? 或 match
fn demo_unwrap_expect() {
    println!("\n--- unwrap() / expect() 演示 ---");

    let some_result: Result<i32, &str> = Ok(42);

    // unwrap(): "我知道这是 Ok，如果不是就 crash"
    let val1 = some_result.unwrap(); // 没问题，是 Ok(42)
    println!("unwrap() 提取值: {}", val1);

    // expect(): 和 unwrap() 一样，但 panic 时会带上自定义信息
    let val2 = some_result.expect("这绝不应该失败，因为它是 Ok(42)");
    println!("expect() 提取值: {}", val2);

    // 对于 Err 的情况：
    let _err_result: Result<i32, &str> = Err("出错了");

    // 以下代码会 panic，已注释：
    // let _ = err_result.unwrap();                // panic with: called `Result::unwrap()` on an `Err` value: "出错了"
    // let _ = err_result.expect("解析配置失败");   // panic with: 解析配置失败: "出错了"

    // 展示了 unwrap_or() 和 unwrap_or_else() 的安全替代方案
}

/// 演示 unwrap_or() 和 unwrap_or_else() —— 提供默认值
fn demo_unwrap_or() {
    println!("\n--- unwrap_or() / unwrap_or_else() 演示 ---");

    let ok_val: Result<i32, &str> = Ok(100);
    let err_val: Result<i32, &str> = Err("失败了");

    // unwrap_or(default): 如果是 Err，使用提供的默认值
    println!("Ok(100).unwrap_or(0)  = {}", ok_val.unwrap_or(0)); // 输出 100
    println!("Err.unwrap_or(0)     = {}", err_val.unwrap_or(0)); // 输出 0（默认值）

    // unwrap_or_else(closure): 如果是 Err，调用闭包生成默认值（惰性求值）
    // 适用于默认值计算成本较高的情况
    println!(
        "Ok(100).unwrap_or_else(|e| e.len() as i32) = {}",
        ok_val.unwrap_or_else(|e: &str| e.len() as i32) // 不会被调用
    );
    println!(
        "Err.unwrap_or_else(|e| e.len() as i32)        = {}",
        err_val.unwrap_or_else(|e: &str| e.len() as i32) // 闭包被调用
    );

    // 实际应用场景：从环境变量读取配置，失败则使用默认值
    let port: u16 = std::env::var("APP_PORT") // Result<String, VarError>
        .unwrap_or_else(|_| "8080".to_string()) // 默认 "8080"
        .parse() // Result<u16, ParseIntError>
        .unwrap_or(8080); // 解析失败也用 8080
    println!("从环境变量 APP_PORT 读取端口（默认 8080）: {}", port);
}

/// 演示 .map_err() —— 手动转换错误类型
fn demo_map_err() {
    println!("\n--- .map_err() 演示 ---");

    // 场景：你有一个 Result<T, ErrorA>，但函数要求 Result<T, ErrorB>
    // map_err 让你在传播错误的同时转换错误类型

    let result_a: Result<i32, &str> = Err("something broke");

    // 将 &str 错误转换为自定义错误
    let result_b: Result<i32, FileStatsError> =
        result_a.map_err(|msg| FileStatsError::ParseError(msg.to_string()));

    match result_b {
        Ok(n) => println!("成功: {}", n),
        Err(e) => println!("转换后的错误: {}", e),
    }

    // 但是！有了 From trait 实现，? 会自动调用 .map_err(Into::into)
    // 所以很多时候你不需要手动写 .map_err()
}

// ----------------------------------------------------------------------------
// 7. 创建示例数据文件
// ----------------------------------------------------------------------------

/// 创建一个示例数据文件用于演示
fn create_sample_file() -> Result<String, io::Error> {
    let path = "/tmp/rust_error_demo_numbers.txt";
    let sample_data = "\
# Rust 错误处理示例数据文件
# 以下每行一个数字，空行和注释行会被跳过
42
100
-17
256
0
99

# 更多数字
1024
-512
33
7
";
    fs::write(path, sample_data)?;
    println!("📝 已创建示例文件: {}", path);
    Ok(path.to_string())
}

// ----------------------------------------------------------------------------
// 8. main() —— 程序入口
// ----------------------------------------------------------------------------
// 注意：main() 返回 (), 不使用 Result<(), E>。
// 我们在 main 内部用 match 处理所有可能的错误。

fn main() {
    println!("╔═════════════════════════════════════════════════════╗");
    println!("║     第十二章：错误处理 — Result / panic! / ?      ║");
    println!("╚═════════════════════════════════════════════════════╝");
    println!();

    // ---- 创建示例文件 ----
    let file_path = match create_sample_file() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ 无法创建示例文件: {}", e);
            return;
        }
    };

    // ---- 方式 A: 使用 ? 运算符（推荐） ----
    match run_analysis(&file_path) {
        Ok(()) => println!("\n✅ run_analysis() 成功完成"),
        Err(e) => {
            eprintln!("\n❌ 分析失败: {}", e);
            // source() 可以获取底层错误
            if let Some(source) = e.source() {
                eprintln!("   底层原因: {}", source);
            }
        }
    }

    println!();
    println!("───────────────────────────────────────────");
    println!();

    // ---- 方式 B: 使用手动 match（对比） ----
    match run_analysis_manual(&file_path) {
        Ok(()) => println!("\n✅ run_analysis_manual() 成功完成"),
        Err(e) => eprintln!("\n❌ 手动版分析失败: {}", e),
    }

    // ---- 尝试读取不存在的文件（演示错误处理） ----
    println!();
    println!("───────────────────────────────────────────");
    println!();

    let missing_path = "/tmp/nonexistent_file_12345.txt";
    println!("📂 尝试读取不存在的文件: {}", missing_path);

    match run_analysis(missing_path) {
        Ok(()) => {} // 不会执行到这里
        Err(e) => {
            // 由于我们实现了 Display，可以友好地打印
            println!("📛 预期中的错误: {}", e);
            println!("   （这证明了 ? 运算符正确地传播了 I/O 错误）");
        }
    }

    // ---- 演示各种工具函数 ----
    println!();
    println!("═══════════════════════════════════════════");
    println!("  工具函数演示");
    println!("═══════════════════════════════════════════");

    demo_panic();
    demo_unwrap_expect();
    demo_unwrap_or();
    demo_map_err();

    // ---- 总结 ----
    println!();
    println!("═══════════════════════════════════════════");
    println!("  总结：Rust 错误处理工具箱");
    println!("═══════════════════════════════════════════");
    println!("  Result<T, E>    — 可恢复错误，强制处理");
    println!("  panic!          — 不可恢复错误，终止程序");
    println!("  ? 运算符        — 传播错误，简洁优雅");
    println!("  unwrap/expect   — 快速原型（不用于生产）");
    println!("  unwrap_or(_else)— 提供默认值");
    println!("  From trait      — 自动错误类型转换");
    println!("  match           — 显式处理，掌控一切");
    println!("═══════════════════════════════════════════");

    // 清理
    let _ = fs::remove_file(&file_path);
    println!("\n🧹 已清理示例文件: {}", file_path);
}
