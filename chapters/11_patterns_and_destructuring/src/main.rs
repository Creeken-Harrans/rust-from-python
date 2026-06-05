#![allow(clippy::match_single_binding)]
// 模式与解构 - Rust 模式匹配的完整指南
//
// Patterns are one of Rust's most powerful features. This program demonstrates
// every major pattern type: destructuring, match guards, @ bindings, or patterns,
// refutable vs irrefutable patterns, and more.

// ---------------------------------------------------------------------------
// Data types used throughout the demonstrations
// ---------------------------------------------------------------------------

/// 事件枚举 - 模拟 GUI 框架中的各类事件
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress { key: char, ctrl: bool },
    Scroll { delta: i32 },
    Quit,
    Resize { width: u32, height: u32 },
    Touch { x: i32, y: i32, pressure: f64 },
}

/// 一个示例结构体，用于演示结构体解构
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    color: String,
    label: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    host: String,
    port: u16,
    timeout_secs: u32,
    retries: u32,
    verbose: bool,
}

// ---------------------------------------------------------------------------
// 核心事件处理 - 展示所有 match 模式类型
// ---------------------------------------------------------------------------

/// 处理单个事件，返回描述字符串。
///
/// 此函数集中展示了：
/// - 结构体解构 (destructuring)
/// - 嵌套匹配
/// - @ 绑定
/// - 匹配守卫 (match guards)
/// - 通配符
/// - 或模式 (|)
fn process_event(event: &Event) -> String {
    match event {
        // ---- 结构体解构：从 Click 变体中提取 x, y ----
        Event::Click { x, y } => {
            format!("鼠标点击于坐标 ({x}, {y})")
        }

        // ---- 或模式：Quit 与 KeyPress('q') 共享同一分支 ----
        // 注意：或模式必须放在更具体的 KeyPress 分支之前，
        // 否则 ctrl: true/false 会先捕获所有 KeyPress 变体
        Event::Quit | Event::KeyPress { key: 'q', .. } => "退出程序".to_string(),

        // ---- 嵌套匹配：检查 ctrl 标志是否为 true ----
        Event::KeyPress { key, ctrl: true } => {
            format!("组合键按下: Ctrl+{key}")
        }
        Event::KeyPress { key, ctrl: false } => {
            format!("按键按下: {key}")
        }

        // ---- @ 绑定：绑定整个值的同时进行解构 ----
        size @ Event::Resize { width, height } => {
            let area = (*width as u64) * (*height as u64);
            format!("窗口调整大小 → {width}x{height} (面积={area})  [@绑定原始值: {size:?}]")
        }

        // ---- 匹配守卫：仅在 delta > 0 时匹配 ----
        Event::Scroll { delta } if *delta > 0 => {
            format!("向上滚动 {delta} 行")
        }
        Event::Scroll { delta } if *delta < 0 => {
            format!("向下滚动 {} 行", delta.abs())
        }
        Event::Scroll { delta } => {
            format!("滚动量为零，无变化 (delta={delta})")
        }

        // ---- 通配符：处理所有尚未匹配的变体 ----
        _ => {
            format!("未处理的事件: {event:?}")
        }
    }
}

// ---------------------------------------------------------------------------
// 元组解构演示
// ---------------------------------------------------------------------------

/// 展示各种元组解构形式：不同大小、嵌套元组
fn demonstrate_tuple_destructuring() {
    println!("========== 元组解构 (Tuple Destructuring) ==========");

    // 基本元组解构
    let pair = (42, "hello");
    let (num, text) = pair;
    println!("基本解构: ({num}, {text})");

    // 三元组解构
    let triple = (1, 2.5, 'A');
    let (a, b, c) = triple;
    println!("三元组解构: ({a}, {b}, {c})");

    // 嵌套元组解构
    let nested = ((1, 2), (3, 4), 5);
    let ((x1, y1), (x2, y2), z) = nested;
    println!("嵌套元组解构: (({x1},{y1}), ({x2},{y2}), {z})");

    // 使用 .. 忽略部分字段
    let long_tuple = (1, 2, 3, 4, 5);
    let (first, .., last) = long_tuple;
    println!("忽略中间元素: first={first}, last={last}");

    // 只取需要的部分
    let (_, second, _, _, _) = long_tuple;
    println!("只取第二个: {second}");

    // 函数返回元组的解构
    fn returns_tuple() -> (i32, String, bool) {
        (100, "Rust".to_string(), true)
    }
    let (code, name, ok) = returns_tuple();
    println!("函数返回元组解构: code={code}, name={name}, ok={ok}");

    println!();
}

// ---------------------------------------------------------------------------
// 结构体解构演示
// ---------------------------------------------------------------------------

/// 展示结构体的各种解构方式：字段重命名、忽略字段、嵌套解构
fn demonstrate_struct_destructuring() {
    println!("========== 结构体解构 (Struct Destructuring) ==========");

    // ---- 基本结构体解构 ----
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("基本解构 Point: x={x}, y={y}");

    // ---- 字段重命名（使用 `:`）----
    let p2 = Point { x: 100, y: 200 };
    let Point { x: px, y: py } = p2;
    println!("字段重命名: px={px}, py={py}");

    // ---- 使用 .. 忽略其余字段 ----
    let cfg = Config {
        host: "localhost".to_string(),
        port: 8080,
        timeout_secs: 30,
        retries: 3,
        verbose: true,
    };
    let Config { host, port, .. } = &cfg;
    println!("部分解构 Config (忽略其余): host={host}, port={port}");

    // 只取 timeout，其余全忽略
    let Config { timeout_secs, .. } = cfg;
    println!("只取 timeout_secs: {timeout_secs}");

    // ---- 嵌套结构体解构 ----
    let rect = Rectangle {
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point { x: 100, y: 80 },
        color: "blue".to_string(),
        label: Some("主窗口".to_string()),
    };
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
        color,
        label,
    } = &rect;
    println!("嵌套解构 Rectangle:");
    println!("  top_left=({x1},{y1}), bottom_right=({x2},{y2})");
    println!("  color={color}, label={label:?}");

    // ---- 在 match 中解构结构体 ----
    match &rect {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            color,
            ..
        } => {
            println!("match 中结构体解构: 左上角在原点, 颜色={color}");
        }
        _ => println!("左上角不在原点"),
    }

    println!();
}

// ---------------------------------------------------------------------------
// 枚举解构演示
// ---------------------------------------------------------------------------

/// 展示 Option 和 Result 的枚举解构，以及 ref 模式
fn demonstrate_enum_destructuring() {
    println!("========== 枚举解构 (Enum Destructuring) ==========");

    // ---- Option 解构 ----
    let some_val: Option<i32> = Some(42);
    match some_val {
        Some(v) => println!("Option::Some 解构: 值为 {v}"),
        None => println!("Option::None"),
    }

    // 嵌套 Option 解构
    let nested_opt: Option<Option<i32>> = Some(Some(99));
    match nested_opt {
        Some(Some(inner)) => println!("嵌套 Option::Some(Some({inner}))"),
        Some(None) => println!("嵌套 Option::Some(None)"),
        None => println!("嵌套 Option::None"),
    }

    // ---- Result 解构 ----
    let ok_result: Result<i32, &str> = Ok(200);
    match ok_result {
        Ok(code) => println!("Result::Ok({code})"),
        Err(msg) => println!("Result::Err({msg})"),
    }

    let err_result: Result<i32, &str> = Err("文件未找到");
    match err_result {
        Ok(_) => unreachable!(),
        Err(e) => println!("Result::Err 解构: {e}"),
    }

    // ---- ref 模式：借用而非移动 ----
    let opt_string = Some(String::from("拥有的字符串"));
    match opt_string {
        // ref s 表示借用该 String，而非移动它
        Some(ref s) => {
            println!("通过 ref 借用: {s}");
            println!("  字符串长度: {}", s.len());
        }
        None => println!("无值"),
    }
    // opt_string 仍然可用，因为我们只是借用了它
    println!("opt_string 匹配后仍可用: {opt_string:?}");

    println!();
}

// ---------------------------------------------------------------------------
// ref 与 ref mut 模式演示
// ---------------------------------------------------------------------------

/// 展示 ref 和 ref mut 在模式匹配中的用法
fn demonstrate_ref_patterns() {
    println!("========== ref 与 ref mut 模式 ==========");

    // ---- ref：在模式中创建不可变引用 ----
    let value = String::from("不可变引用示例");
    match value {
        // ref v 等同于在匹配前写 &value，但 ref 更灵活
        ref v => {
            println!("ref 绑定: {v}");
            // v 是 &String 类型
            let _: &String = v;
        }
    }
    // value 仍然可用，因为 ref 只创建了引用
    println!("原始值仍可用: {value}");

    // ---- ref mut：在模式中创建可变引用 ----
    let mut counter = 0;
    match counter {
        ref mut c => {
            *c += 1;
            println!("ref mut 修改后: {c}");
        }
    }
    println!("counter 被 ref mut 修改为: {counter}");

    // ---- 在枚举匹配中使用 ref mut ----
    let mut maybe_num = Some(10);
    if let Some(ref mut n) = maybe_num {
        *n *= 2;
        println!("通过 ref mut 修改 Option 内部值: {n}");
    }
    println!("修改后的 maybe_num: {maybe_num:?}");

    // ---- 对比：不用 ref 会怎样？ ----
    // 如果不使用 ref，值会被移动到 match 分支中，
    // 之后无法再使用原变量。
    let owned = vec![1, 2, 3];
    match owned {
        // 没有 ref，owned 被移动到 v
        v => println!("移动语义: {v:?}"),
    }
    // owned 在此处不再可用 —— 已被移动
    // println!("{owned:?}"); // 编译错误！

    println!();
}

// ---------------------------------------------------------------------------
// 不可反驳模式 vs 可反驳模式
// ---------------------------------------------------------------------------

/// 展示不可反驳模式（irrefutable）与可反驳模式（refutable）的区别
fn demonstrate_refutability() {
    println!("========== 不可反驳模式 vs 可反驳模式 ==========");

    // ---- 不可反驳模式 (Irrefutable Pattern) ----
    // 这些模式总是匹配成功，可以用于 let、函数参数、for 循环

    println!("--- 不可反驳模式 (总是匹配) ---");

    // let 语句：必须使用不可反驳模式
    let (a, b) = (1, 2); // 总是匹配
    println!("let 解构: ({a}, {b})");

    // 函数参数中的模式（也是不可反驳的）
    fn take_point(Point { x, y }: Point) {
        println!("函数参数解构 Point: x={x}, y={y}");
    }
    take_point(Point { x: 5, y: 10 });

    // for 循环中的模式
    let pairs = vec![(1, "一"), (2, "二"), (3, "三")];
    for (num, name) in &pairs {
        println!("  for 循环解构: {num} -> {name}");
    }

    // ---- 可反驳模式 (Refutable Pattern) ----
    // 这些模式可能匹配失败，只能用于 match、if let、while let

    println!("\n--- 可反驳模式 (可能失败) ---");

    // if let：只处理匹配的情况
    let maybe: Option<i32> = Some(100);
    if let Some(val) = maybe {
        println!("if let 匹配 Some: {val}");
    }

    let none_val: Option<i32> = None;
    if let Some(_val) = none_val {
        println!("这行不会打印");
    } else {
        println!("if let 未匹配 — 进入 else 分支");
    }

    // if let 配合 else if let 链
    let event = Event::KeyPress {
        key: 'a',
        ctrl: false,
    };
    if let Event::KeyPress { key, ctrl: true } = &event {
        println!("if let: Ctrl+{key}");
    } else if let Event::KeyPress { key, .. } = &event {
        println!("if let: 普通按键 {key}");
    } else {
        println!("if let: 其他事件");
    }

    // while let：循环直到模式不匹配
    let mut stack = vec![1, 2, 3];
    println!("while let 弹出栈:");
    while let Some(top) = stack.pop() {
        println!("  弹出: {top}");
    }
    println!("栈已空");

    // ---- 错误示例（注释掉，因为无法编译）----
    // let Some(x) = some_option; // 编译错误！let 需要不可反驳模式
    // if let (a, b) = (1, 2) {} // 编译警告！if let 中用了不可反驳模式

    println!();
}

// ---------------------------------------------------------------------------
// main - 组合所有演示
// ---------------------------------------------------------------------------

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║     Rust 模式与解构 — 完整演示程序                   ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // ---- 1. 元组解构 ----
    demonstrate_tuple_destructuring();

    // ---- 2. 结构体解构 ----
    demonstrate_struct_destructuring();

    // ---- 3. 枚举解构 ----
    demonstrate_enum_destructuring();

    // ---- 4. ref / ref mut 模式 ----
    demonstrate_ref_patterns();

    // ---- 5. 不可反驳 vs 可反驳 ----
    demonstrate_refutability();

    // ---- 6. 综合事件处理 ----
    println!("========== 综合事件处理 (process_event) ==========");

    let events = vec![
        Event::Click { x: 150, y: 300 },
        Event::KeyPress {
            key: 's',
            ctrl: true,
        },
        Event::KeyPress {
            key: 'a',
            ctrl: false,
        },
        Event::Scroll { delta: 5 },
        Event::Scroll { delta: 0 },
        Event::Scroll { delta: -3 },
        Event::Quit,
        Event::KeyPress {
            key: 'q',
            ctrl: false,
        },
        Event::Resize {
            width: 1920,
            height: 1080,
        },
        Event::Touch {
            x: 200,
            y: 400,
            pressure: 0.8,
        },
    ];

    for event in &events {
        let description = process_event(event);
        println!("事件: {event:?}");
        println!("  → {description}");
    }

    println!();

    // ---- 7. 模式类型分类总结 ----
    println!("========== 模式类型分类总结 ==========");
    println!();

    let patterns = [
        (
            "不可反驳模式 (Irrefutable)",
            "总是匹配成功",
            "let, 函数参数, for 循环",
            "let (x, y) = (1, 2);  fn f(Point {x,y}: Point)",
        ),
        (
            "可反驳模式 (Refutable)",
            "可能匹配失败",
            "match, if let, while let",
            "if let Some(v) = opt  |  while let Some(v) = iter.next()",
        ),
        (
            "结构体解构",
            "从 struct 中提取字段",
            "let, match, if let, 函数参数",
            "let Point { x, y } = p;  let Config { host, .. } = cfg;",
        ),
        (
            "元组解构",
            "从 tuple 中提取元素",
            "let, match, for, 函数参数",
            "let (a, b, c) = (1, 2, 3);  for (k, v) in map",
        ),
        (
            "枚举解构",
            "从 enum 变体中提取数据",
            "match, if let, while let",
            "match opt { Some(v) => ..., None => ... }",
        ),
        (
            "ref 模式",
            "在模式中创建引用而非移动",
            "match, if let, while let",
            "match val { ref v => ... }; Some(ref mut x) => *x += 1;",
        ),
        (
            "匹配守卫 (Match Guard)",
            "在模式后添加 if 条件",
            "match 分支",
            "Event::Scroll { delta } if *delta > 0 => ...",
        ),
        (
            "@ 绑定",
            "绑定整个值同时解构内部",
            "match, if let",
            "size @ Resize { width, height } => ...",
        ),
        (
            "或模式 (|)",
            "多个模式共享同一分支",
            "match, if let",
            "Quit | KeyPress { key: 'q', .. } => ...",
        ),
        (
            "通配符 (_)",
            "匹配任意值并忽略",
            "match, let, 函数参数",
            "let _ = some_fn();  match val { _ => ... }",
        ),
        (
            "范围模式 (.. / ..=)",
            "匹配数值范围",
            "match",
            "match num { 1..=5 => ..., 6..10 => ... }",
        ),
    ];

    println!(
        "{:<30} | {:<16} | {:<24} | {:<50}",
        "模式类型", "可反驳性", "可用位置", "示例"
    );
    println!("{:-<30}-+-{:-<16}-+-{:-<24}-+-{:-<50}", "", "", "", "");
    for (name, refutability, locations, example) in &patterns {
        println!(
            "{:<30} | {:<16} | {:<24} | {:<50}",
            name, refutability, locations, example
        );
    }

    println!();
    println!("程序执行完毕。");
}
