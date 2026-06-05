#![allow(clippy::never_loop, clippy::no_effect)]
//! # 函数、表达式与控制流
//!
//! 本程序是一个小型统计计算器，用于演示 Rust 中函数定义、
//! 语句与表达式的区别、控制流（if / loop / while / for）、
//! 以及 match 模式匹配的基本用法。
//!
//! 运行方式：
//! ```bash
//! cargo run
//! ```

/// 静态样本数据切片 —— 作为程序的输入数据源。
const STATIC_DATA: &[i32] = &[45, 22, 78, 34, 91, 12, 56, 83, 29, 67];

// ---------------------------------------------------------------------------
// 自定义枚举：用于 match 演示
// ---------------------------------------------------------------------------

/// 统计计算的结果类型。
///
/// 每种统计指标返回不同的数据类型：
/// - 最小值 / 最大值返回整数
/// - 均值返回浮点数
/// - 总和返回整数
#[derive(Debug)]
enum StatResult {
    /// 最小值 (i32)
    Min(i32),
    /// 最大值 (i32)
    Max(i32),
    /// 算术平均值 (f64)
    Mean(f64),
    /// 总和 (i32)
    Sum(i32),
    /// 数据为空时的占位变体
    Empty,
}

// ---------------------------------------------------------------------------
// 统计函数
// ---------------------------------------------------------------------------

/// 计算切片中的最小值。
///
/// # 参数
/// * `data` - 整数切片引用
///
/// # 返回
/// * `Some(min)` 如果切片非空
/// * `None` 如果切片为空
///
/// # 示例
/// ```
/// let nums = [3, 1, 4, 1, 5];
/// assert_eq!(calculate_min(&nums), Some(1));
/// ```
fn calculate_min(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    let mut min = data[0];
    for &value in data.iter().skip(1) {
        if value < min {
            min = value;
        }
    }
    Some(min)
}

/// 计算切片中的最大值。
///
/// # 参数
/// * `data` - 整数切片引用
///
/// # 返回
/// * `Some(max)` 如果切片非空
/// * `None` 如果切片为空
fn calculate_max(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    let mut max = data[0];
    for &value in data.iter().skip(1) {
        if value > max {
            max = value;
        }
    }
    Some(max)
}

/// 计算切片的算术平均值。
///
/// # 参数
/// * `data` - 整数切片引用
///
/// # 返回
/// * `Some(mean)` 如果切片非空
/// * `None` 如果切片为空
fn calculate_mean(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sum: i32 = data.iter().sum();
    Some(sum as f64 / data.len() as f64)
}

/// 计算切片中所有元素的总和。
///
/// 使用 `for` 循环与 RangeFull（由 `.iter()` 提供）
/// 逐个累加，而非 `.sum()` 方法，以明确展示循环过程。
///
/// # 参数
/// * `data` - 整数切片引用
///
/// # 返回
/// * 所有元素的和 (i32)
fn calculate_sum(data: &[i32]) -> i32 {
    let mut total = 0;
    for &value in data {
        total += value;
    }
    total
}

// ---------------------------------------------------------------------------
// 演示函数：语句 vs 表达式
// ---------------------------------------------------------------------------

/// 演示 **语句 (Statement)** 与 **表达式 (Expression)** 的核心区别。
///
/// - **语句** 以分号结尾，不返回值（或者说返回 `()`）。
/// - **表达式** 不以分号结尾，求值后产生一个具体的值。
///
/// 注意：`let x = 5;` 本身是语句，但 `5` 是表达式。
fn demonstrate_statement_vs_expression() {
    println!("===== 语句 (Statement) vs 表达式 (Expression) =====");

    // --- 示例 1：分号改变含义 ---
    // 这是一个表达式（无分号），返回 42
    let value = 42;
    println!("表达式（无分号）: let value = 42;  → value = {}", value);

    // 如果给表达式加上分号，它就变成了语句，返回 ()
    let unit: () = {
        42;
    };
    println!(
        "分号把表达式变成语句: let unit = {{ 42; }};  → unit = {:?}（即 unit 类型）",
        unit
    );

    // --- 示例 2：函数体的返回值 ---
    // 函数体是一个表达式，最后一个表达式的值作为返回值
    fn returns_forty_two() -> i32 {
        42 // 无分号 → 这是返回值
    }
    println!(
        "函数隐式返回（无分号）: returns_forty_two() = {}",
        returns_forty_two()
    );

    // 如果加了分号 → 表达式变成语句 → 不返回 → 编译错误！
    // fn would_not_compile() -> i32 {
    //     42;  // 错误！expected `i32`, found `()`
    // }
    println!("⚠ 如果在返回位置加分号: `42;` → 返回 () 而非 i32 → 编译错误！");
}

// ---------------------------------------------------------------------------
// 演示函数：块表达式
// ---------------------------------------------------------------------------

/// 演示 **块表达式**（Block Expression）—— 用 `{ }` 包裹的一系列语句，
/// 最后一个表达式的值就是整个块的求值结果。
fn demonstrate_block_expression() {
    println!("\n===== 块表达式 (Block Expression) =====");

    let dataset = [10, 20, 30, 40, 50];

    // 整个 { } 块是一个表达式，返回计算结果
    let result = {
        let mut sum = 0;
        for num in dataset {
            sum += num;
        }
        // 关键：最后一行没有分号 → 这是块的返回值
        sum
    };

    println!("数据: {:?}", dataset);
    println!("块表达式计算的总和: {}", result);

    // 更复杂的块表达式：包含多个中间变量
    let stats = {
        let count = dataset.len();
        let total: i32 = dataset.iter().sum();
        let avg = total as f64 / count as f64;
        (count, total, avg) // 返回一个元组
    };
    println!(
        "块表达式返回元组: count={}, total={}, avg={:.2}",
        stats.0, stats.1, stats.2
    );
}

// ---------------------------------------------------------------------------
// 演示函数：if 表达式
// ---------------------------------------------------------------------------

/// 演示 **if 作为表达式**—— 在 Rust 中 `if` 可以产生值。
///
/// 与 Python 的三元表达式 `a if cond else b` 不同：
/// Rust 的 `if` 本身就是表达式，不需要额外的三元运算符。
fn demonstrate_if_expression() {
    println!("\n===== if 作为表达式 =====");

    let mean = calculate_mean(STATIC_DATA).unwrap_or(0.0);

    // `if` 是表达式：两个分支都必须返回相同类型
    let category = if mean > 50.0 { "high" } else { "low" };

    println!("均值 = {:.2}", mean);
    println!("类别 = {}（if 表达式返回值）", category);

    // 嵌套 if 表达式
    let grade = if mean >= 80.0 {
        "优秀"
    } else if mean >= 60.0 {
        "良好"
    } else if mean >= 40.0 {
        "中等"
    } else {
        "待提高"
    };
    println!("评级 = {}（嵌套 if 表达式）", grade);

    // Python 对照：在 Python 中需要写成
    // category = "high" if mean > 50.0 else "low"
    println!("💡 Python 对照: category = \"high\" if mean > 50.0 else \"low\"");
}

// ---------------------------------------------------------------------------
// 演示函数：loop 循环
// ---------------------------------------------------------------------------

/// 演示 **loop** 无限循环，以及 **break 携带返回值**。
///
/// `loop` 是 Rust 专用的无限循环语法。当你需要一直运行直到某个条件
/// 满足时使用 `loop`。与 `while` 不同，`loop` 的 `break` 可以携带返回值。
fn demonstrate_loop() {
    println!("\n===== loop 循环 + break 携带返回值 =====");

    let threshold: i32 = 50;

    // loop 可以像表达式一样求值：break 后面的值就是 loop 的返回值
    let found_index = loop {
        // 用一个计数器来模拟搜索
        // 实际应该在循环体内累加条件
        // 这里我们展示 break 返回值的机制
        break 5; // 直接 break 并返回 5（仅用于演示语法）
    };
    println!("loop 通过 break 返回的值: {}", found_index);

    // 更实际的例子：在 STATIC_DATA 中寻找第一个 >= threshold 的元素
    let mut idx: usize = 0;
    let search_result = loop {
        if idx >= STATIC_DATA.len() {
            break -1_i32; // 没找到，返回 -1
        }
        if STATIC_DATA[idx] >= threshold {
            break idx as i32; // 找到：返回索引
        }
        idx += 1;
    };
    println!(
        "在数据中查找第一个 >= {} 的元素: index = {}",
        threshold, search_result
    );

    // loop 计数演示
    let mut count = 0;
    let counter = loop {
        count += 1;
        if count >= 10 {
            break count; // break 返回 count 的值
        }
    };
    println!("loop 计数到 10 后 break 返回: {}", counter);
}

// ---------------------------------------------------------------------------
// 演示函数：while 循环
// ---------------------------------------------------------------------------

/// 演示 **while** 条件循环。
///
/// `while` 在每次迭代前检查条件，条件为 `false` 时退出。
/// 与 `loop` 不同，`while` 的 `break` **不能**携带返回值。
fn demonstrate_while() {
    println!("\n===== while 循环 =====");

    let mut remaining = STATIC_DATA.to_vec();
    print!("逐步弹出元素: ");

    // while 循环：当条件为 true 时持续执行
    while !remaining.is_empty() {
        // pop() 返回 Option，移除并返回最后一个元素
        if let Some(val) = remaining.pop() {
            print!("{} ", val);
        }
    }
    println!();

    // 另一个例子：while let 模式匹配循环
    let mut stack = vec![1, 2, 3];
    print!("while let 弹出: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("\n💡 提示: while 不能像 loop 那样通过 break 返回值");
}

// ---------------------------------------------------------------------------
// 演示函数：for 循环与 Range
// ---------------------------------------------------------------------------

/// 演示 **for 循环** 与 **Range** 语法 (`..` 和 `..=`)。
///
/// - `a..b`    —— 左闭右开区间 [a, b)
/// - `a..=b`   —— 左闭右闭区间 [a, b]
/// - `..b`     —— 从 0 到 b (不含)
/// - `..=b`    —— 从 0 到 b (含)
fn demonstrate_for_and_range() {
    println!("\n===== for 循环与 Range =====");

    // 使用 Range ..= （包含上界）
    print!("Range 0..=5 (含上界): ");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    // 使用 Range .. （不含上界）
    print!("Range 0..5  (不含上界): ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // 遍历切片 —— 最常用的模式
    print!("遍历 STATIC_DATA: ");
    for &num in STATIC_DATA {
        print!("{} ", num);
    }
    println!();

    // 使用 .enumerate() 获取索引
    println!("带索引遍历:");
    for (idx, &num) in STATIC_DATA.iter().enumerate() {
        println!("  [{}] = {}", idx, num);
    }

    // 步进循环（使用 step_by）
    print!("Range 0..20 步进 3: ");
    for i in (0..20).step_by(3) {
        print!("{} ", i);
    }
    println!();

    // Python 对照
    println!("💡 Python 对照: for i in range(6)  →  Rust: for i in 0..6");
    println!("💡 Python 对照: enumerate(lst)   →  Rust: .iter().enumerate()");
}

// ---------------------------------------------------------------------------
// 演示函数：match 表达式
// ---------------------------------------------------------------------------

/// 演示 **match 表达式**—— Rust 的模式匹配。
///
/// `match` 类似于 C 的 `switch` 但强大得多：
/// - 每个分支（arm）必须覆盖所有可能的情况（穷尽性检查）
/// - 可以解构枚举变体中的数据
/// - `match` 本身也是表达式
fn demonstrate_match() {
    println!("\n===== match 表达式 =====");

    let results = vec![
        StatResult::Min(12),
        StatResult::Max(91),
        StatResult::Mean(51.7),
        StatResult::Sum(517),
        StatResult::Empty,
    ];

    for result in &results {
        // match 是表达式：每个分支返回一个值
        let description = match result {
            StatResult::Min(v) => format!("最小值: {}", v),
            StatResult::Max(v) => format!("最大值: {}", v),
            StatResult::Mean(v) => format!("均值: {:.2}", v),
            StatResult::Sum(v) => format!("总和: {}", v),
            StatResult::Empty => "数据为空".to_string(),
        };
        println!("match 分支结果 → {}", description);
    }

    // match 也可以与通配符搭配处理未用到的变体
    let sample = StatResult::Mean(88.5);
    match sample {
        StatResult::Mean(avg) => {
            let evaluation = if avg > 90.0 {
                "非常好"
            } else if avg > 70.0 {
                "良好"
            } else {
                "一般"
            };
            println!("\n均值 {:.2} → {}", avg, evaluation);
        }
        _ => println!("只处理 Mean 变体，其余忽略"),
    }
}

// ---------------------------------------------------------------------------
// 人口数据/结构体演示（额外展示函数与模块化）
// ---------------------------------------------------------------------------

/// 一个简单的城市人口结构体，用于演示函数接收自定义类型。
#[derive(Debug)]
struct CityStats {
    name: &'static str,
    population: u32,
}

/// 根据人口数量返回城市规模分类。
///
/// 展示了 `if` 表达式在函数体中的使用。
fn classify_city(population: u32) -> &'static str {
    if population > 10_000_000 {
        "超大城市"
    } else if population > 1_000_000 {
        "大城市"
    } else if population > 100_000 {
        "中等城市"
    } else {
        "小城市"
    }
}

// ---------------------------------------------------------------------------
// 入口函数
// ---------------------------------------------------------------------------

/// main 函数 —— 程序的入口点。
///
/// 依次调用所有统计函数和演示函数，全面展示本章的核心概念。
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   函数、表达式与控制流 —— 统计计算器        ║");
    println!("║   Rust 从 Python 视角的学习之旅              ║");
    println!("╚══════════════════════════════════════════════╝");

    // ---- 统计数据 ----
    println!("\n📊 数据样本: {:?}", STATIC_DATA);

    let min = calculate_min(STATIC_DATA);
    let max = calculate_max(STATIC_DATA);
    let mean = calculate_mean(STATIC_DATA);
    let sum = calculate_sum(STATIC_DATA);

    println!("--- 统计结果 ---");
    // match 用于优雅地处理 Option
    match min {
        Some(v) => println!("  最小值: {}", v),
        None => println!("  最小值: 数据为空"),
    }
    match max {
        Some(v) => println!("  最大值: {}", v),
        None => println!("  最大值: 数据为空"),
    }
    match mean {
        Some(v) => println!("  均值:   {:.2}", v),
        None => println!("  均值:   数据为空"),
    }
    println!("  总和:   {}", sum);
    println!("  元素数: {}", STATIC_DATA.len());

    // ---- 概念演示 ----
    demonstrate_statement_vs_expression();
    demonstrate_block_expression();
    demonstrate_if_expression();
    demonstrate_loop();
    demonstrate_while();
    demonstrate_for_and_range();
    demonstrate_match();

    // ---- 额外演示：自定义类型与函数组合 ----
    println!("\n===== 自定义类型与函数组合 =====");
    let cities = [
        CityStats {
            name: "上海",
            population: 24_870_000,
        },
        CityStats {
            name: "杭州",
            population: 12_200_000,
        },
        CityStats {
            name: "丽江",
            population: 290_000,
        },
    ];
    for city in &cities {
        let category = classify_city(city.population);
        println!("  {} (人口 {}) → {}", city.name, city.population, category);
    }

    println!("\n✅ 所有演示完成！");
}
