#![allow(
    clippy::vec_init_then_push,
    clippy::approx_constant,
    clippy::eq_op,
    clippy::assertions_on_constants
)]
// ============================================================
// 第 23 章: 宏 (Macros) — Rust 的元编程
// ============================================================
// Rust 的宏不是简单的文本替换（不像 C 的 #define），
// 而是工作在 AST（抽象语法树）层面的代码生成。
// 宏在编译期展开，可以生成任意合法的 Rust 代码。

// -------------------------------------------------------
// 1. 简单的声明式宏 (Declarative Macros with macro_rules!)
// -------------------------------------------------------

/// 最简单的宏：展开为一行打印语句
macro_rules! hello {
    () => {
        println!("Hello! 来自 hello! 宏的问候");
    };
}

/// `say!` 宏：打印表达式及其值
/// `stringify!` 是编译器内置宏，将表达式转为字符串字面量
macro_rules! say {
    ($expr:expr) => {
        println!("  say! → {} = {:?}", stringify!($expr), $expr);
    };
}

/// `create_function!` 宏：生成一个函数
/// 演示宏如何生成代码结构（函数无法做到这一点）
/// 使用闭包风格语法：`create_function!(名称, |参数| 函数体)`
/// 参数名从调用处捕获，保证宏卫生正确传递
macro_rules! create_function {
    ($func_name:ident, |$param:ident| $body:expr) => {
        fn $func_name($param: i32) -> String {
            format!(
                "函数 {} 被调用，输入 = {}, 结果 = {}",
                stringify!($func_name),
                $param,
                $body
            )
        }
    };
}

/// `vec_of_squares!` 宏：生成包含数字平方的 Vec
macro_rules! vec_of_squares {
    // 匹配零个或多个表达式，用逗号分隔
    ($($x:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x * $x);
            )*
            v
        }
    };
}

// -------------------------------------------------------
// 2. 重复模式 (Repetition Patterns)
// -------------------------------------------------------

/// `my_vec!` — 简化版的 vec! 实现
/// $(...)* 表示重复零次或多次
/// $(...),* 表示重复间用逗号分隔
macro_rules! my_vec {
    // 空 Vec
    () => {
        Vec::new()
    };
    // 一个或多个元素，逗号分隔，允许末尾逗号
    ($($x:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    };
}

/// `print_all!` — 逐个打印所有参数
/// 演示在重复体中使用多条语句
macro_rules! print_all {
    ($($item:expr),* $(,)?) => {
        $(
            println!("  [print_all] item = {:?}", $item);
        )*
    };
}

/// `count_items!` — 计数宏，演示递归宏模式
macro_rules! count_items {
    () => { 0usize };
    ($head:expr $(, $tail:expr)* $(,)?) => {
        1usize + count_items!($($tail),*)
    };
}

/// `debug_struct!` — 生成一个带 Debug 输出的结构体初始化代码
macro_rules! debug_struct {
    ($struct_name:ident { $($field:ident : $value:expr),* $(,)? }) => {
        {
            println!("构造 {} 结构体:", stringify!($struct_name));
            $(
                println!("  .{} = {:?}", stringify!($field), $value);
            )*
            // 返回一个元组来模拟结构体
            ($($value),*)
        }
    };
}

// -------------------------------------------------------
// 3. 使用 create_function! 在模块级别生成函数
// -------------------------------------------------------
create_function!(double_me, |x| x * 2);
create_function!(triple_me, |x| x * 3);

// -------------------------------------------------------
// 4. main 函数：演示所有宏
// -------------------------------------------------------
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║    第 23 章: Rust 宏 — 元编程实战            ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // --- 4a. 简单宏 ---
    println!("━━━ 1. 简单声明式宏 ━━━");
    hello!();
    hello!(); // 可以多次调用

    let x = 42;
    let name = "Rust";
    say!(x);
    say!(name);
    say!(x + 8);
    say!(name.len());

    // --- 4b. 代码生成宏 ---
    println!("\n━━━ 2. 代码生成宏 create_function! ━━━");
    println!("{}", double_me(10));
    println!("{}", triple_me(10));

    // --- 4c. vec_of_squares! ---
    println!("\n━━━ 3. vec_of_squares! 宏 ━━━");
    let squares = vec_of_squares!(1, 2, 3, 4, 5);
    println!("  平方数组: {:?}", squares);
    let squares2 = vec_of_squares!(10, 20);
    println!("  平方数组2: {:?}", squares2);

    // --- 4d. 重复模式 my_vec! ---
    println!("\n━━━ 4. my_vec! — 简化版 vec! 实现 ━━━");
    let v1 = my_vec![1, 2, 3, 4, 5];
    println!("  my_vec![1,2,3,4,5] = {:?}", v1);
    let v2: Vec<i32> = my_vec![];
    println!("  my_vec![] = {:?}", v2);
    let v3 = my_vec!["苹果", "香蕉", "橘子"];
    println!("  my_vec![\"苹果\", \"香蕉\", \"橘子\"] = {:?}", v3);

    // --- 4e. print_all! ---
    println!("\n━━━ 5. print_all! — 逐个打印 ━━━");
    print_all!(100, "hello", 3.14, true);

    // --- 4f. count_items! ---
    println!("\n━━━ 6. count_items! — 递归计数宏 ━━━");
    let n = count_items!(a, b, c, d, e);
    println!("  count_items!(a,b,c,d,e) = {}", n);
    let n2 = count_items!();
    println!("  count_items!() = {}", n2);

    // --- 4g. debug_struct! ---
    println!("\n━━━ 7. debug_struct! — 调试结构体构造 ━━━");
    let _point = debug_struct!(Point {
        x: 10,
        y: 20,
        label: "原点"
    });

    // -------------------------------------------------------
    // 5. 标准库中的常用宏
    // -------------------------------------------------------
    println!("\n━━━ 8. 标准库常用宏 ━━━");

    // println! / format! / eprintln!
    let formatted = format!("今天是 {} 月 {} 日", 6, 5);
    println!("  format!  → {}", formatted);
    eprintln!("  eprintln! → 这条输出到 stderr（你看不到的话可能被合并了）");

    // vec!
    let std_vec = vec![10, 20, 30, 40, 50];
    println!("  vec!     → {:?}", std_vec);

    // assert! / assert_eq!
    assert!(1 + 1 == 2, "数学还能出错？");
    assert_eq!(2 + 2, 4);
    println!("  assert!/assert_eq! → 全部通过（如果失败会 panic）");

    // dbg! — 调试打印，返回原值
    let val = dbg!(42 * 2);
    println!("  dbg! 返回值 = {}", val);

    // matches! — 模式匹配，返回 bool
    let some_val = Some(42);
    let is_some = some_val.is_some();
    let is_forty_two = matches!(some_val, Some(42));
    println!("  matches!(Some(42), Some(_))  = {}", is_some);
    println!("  matches!(Some(42), Some(42)) = {}", is_forty_two);

    // todo! / unimplemented! — 占位宏
    // 这两个会触发 panic，所以这里只展示概念
    println!("  todo!()         → 标记未完成代码，编译通过但运行时会 panic");
    println!("  unimplemented!() → 类似 todo!，语义上表示'尚未实现'");
    println!("  (这两个宏在此不做实际调用，以免程序崩溃)");

    // unreachable! — 标记不会到达的代码分支
    println!("  unreachable!()  → 标记逻辑上不会到达的代码路径");

    // -------------------------------------------------------
    // 6. 函数 vs 宏的对比
    // -------------------------------------------------------
    println!("\n━━━ 9. 函数 vs 宏 — 关键区别 ━━━");
    println!("  ┌─────────────────┬──────────────────┬────────────────────┐");
    println!("  │ 特性            │ 函数 (fn)        │ 宏 (macro_rules!)  │");
    println!("  ├─────────────────┼──────────────────┼────────────────────┤");
    println!("  │ 展开时机        │ 运行时调用       │ 编译期展开         │");
    println!("  │ 可变参数        │ 不支持           │ 支持               │");
    println!("  │ 代码生成        │ 不能生成新函数   │ 可以生成任意代码   │");
    println!("  │ 类型检查        │ 调用时检查       │ 展开后检查         │");
    println!("  │ 调试难度        │ 容易             │ 较难               │");
    println!("  │ 可读性          │ 高               │ 视复杂度而定       │");
    println!("  │ 命名规范        │ snake_case       │ 后跟 !             │");
    println!("  └─────────────────┴──────────────────┴────────────────────┘");

    // 演示：函数做不到的——宏可以
    println!("\n  函数无法做到：在调用处插入内联代码（宏可以）。");
    println!("  例如 my_vec![a, b, c] 在调用位置就地展开为 push 循环。");
    println!("  函数做不到：接收任意数量的不同类型参数（宏可以）。");

    // -------------------------------------------------------
    // 7. 过程宏（Proc Macros）简介
    // -------------------------------------------------------
    println!("\n━━━ 10. 过程宏 (Procedural Macros) 简介 ━━━");
    println!("  过程宏是更强大的宏，操作 TokenStream 而非模式匹配。");
    println!();
    println!("  三种类型：");
    println!("    ① 派生宏 (Derive macros)");
    println!("       例: #[derive(Debug, Clone, PartialEq)]");
    println!("       在结构体/枚举上自动生成 trait 实现。");
    println!("    ② 属性宏 (Attribute macros)");
    println!("       例: #[tokio::main], #[serde(rename_all = \"camelCase\")]");
    println!("       在项目上添加自定义属性，可修改该项。");
    println!("    ③ 函数式过程宏 (Function-like proc macros)");
    println!("       例: sqlx::query!(\"SELECT ...\")");
    println!("       像函数一样调用，但接收 TokenStream。");
    println!();
    println!("  重要限制：过程宏必须定义在独立的 crate 中");
    println!("  类型为 proc-macro = true，因为它们在编译期被编译器加载执行。");

    // -------------------------------------------------------
    // 8. 宏卫生 (Macro Hygiene) 简介
    // -------------------------------------------------------
    println!("\n━━━ 11. 宏卫生 (Hygiene) ━━━");
    println!("  Rust 宏是'卫生的'(hygienic)，意味着：");
    println!("  - 宏内部定义的变量不会污染调用处的命名空间");
    println!("  - 不会意外捕获调用处的标识符");
    println!("  - 这不同于 C 的 #define 文本替换（C 宏不卫生）");

    // 演示宏卫生
    // 即使宏内部使用了变量 'v'，也不会与外部的 'v' 冲突
    let v = 999;
    let hygienic_vec = my_vec![1, 2, 3];
    println!("  外部 v = {} (未被宏内部变量影响)", v);
    println!("  my_vec! 结果 = {:?}", hygienic_vec);

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║            宏章节演示完成                     ║");
    println!("╚══════════════════════════════════════════════╝");
}
