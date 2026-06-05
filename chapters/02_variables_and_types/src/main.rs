#![allow(clippy::approx_constant, clippy::vec_init_then_push)]
// ============================================================
// 02_variables_and_types — 变量与类型
// Rust 从 Python 视角：变量绑定、标量类型与复合类型
// ============================================================

use std::mem::size_of;

// -----------------------------------------------------------
// 辅助函数：打印某种类型的内存大小和类型名称
// -----------------------------------------------------------
fn print_type_info<T>(type_name: &str) {
    // size_of::<T>() 返回该类型在栈上占用的字节数
    println!("  {:<6} 占用 {} 字节", type_name, size_of::<T>());
}

// -----------------------------------------------------------
// 常量：使用 const 关键字，必须显式标注类型
// -----------------------------------------------------------
const MAX_POINTS: u32 = 100_000;

// -----------------------------------------------------------
// 入口
// -----------------------------------------------------------
fn main() {
    println!("===== Rust 变量与类型演示 =====\n");

    // ========================================================
    // 第一部分：变量绑定 (Variable Binding)
    // ========================================================
    println!("--- 1. 变量绑定与不可变性 ---");

    // `let` 将值"绑定"到一个名称上。默认不可变（immutable）。
    let x = 5;
    println!("  x = {}", x);

    // 下面这行如果取消注释，会导致编译错误：
    //     error[E0384]: cannot assign twice to immutable variable `x`
    // x = 6;

    // 解决方案：使用 `mut` 关键字声明可变变量
    let mut counter = 0;
    println!("  counter 初始值 = {}", counter);

    counter += 1; // Rust 没有 `++` 自增运算符
    counter += 1;
    println!("  counter 递增两次后 = {}", counter);
    println!("  说明: mut 允许重新赋值，但变量类型不能改变\n");

    // ========================================================
    // 第二部分：遮蔽 (Shadowing) vs mut
    // ========================================================
    println!("--- 2. 遮蔽 (Shadowing) ---");

    // 遮蔽：用 let 重新声明同名变量，可以改变类型
    let value = 42i32;
    println!("  value (i32) = {}", value);

    // 遮蔽 value：类型从 i32 变成 &str（字符串切片）
    let value = "现在是字符串了";
    println!("  value (被遮蔽为 &str) = {}", value);

    // mut 不能改变类型——以下代码会编译失败：
    // let mut y = 10;
    // y = "hello"; // error[E0308]: mismatched types

    // mut vs 遮蔽 的区别总结：
    // ┌──────────┬────────────────┬──────────────────┐
    // │ 特性     │ mut            │ 遮蔽 (let 重新绑定) │
    // ├──────────┼────────────────┼──────────────────┤
    // │ 改变值   │ ✓              │ ✓                │
    // │ 改变类型 │ ✗              │ ✓                │
    // │ 可变性   │ 一个可变绑定    │ 新建绑定，旧值被隐藏  │
    // │ 语法     │ let mut x = …  │ let x = …        │
    // └──────────┴────────────────┴──────────────────┘
    println!("  说明: mut 只能改值不能改类型；遮蔽可以同时改变值和类型\n");

    // ========================================================
    // 第三部分：常量 (Constants)
    // ========================================================
    println!("--- 3. 常量 ---");
    println!("  MAX_POINTS = {}", MAX_POINTS);
    // const 与 let 的区别：
    //  - const 必须标注类型，let 可推断
    //  - const 可用于全局作用域，let 仅限函数内
    //  - const 在编译期求值，let 在运行时绑定
    //  - const 命名惯例：全大写加下划线 (SCREAMING_SNAKE_CASE)
    println!("  说明: const 在编译期确定，必须显式标注类型\n");

    // ========================================================
    // 第四部分：标量类型 (Scalar Types)
    // ========================================================
    println!("--- 4. 标量类型 ---");
    println!("  整数类型：");

    // --- 有符号整数 ---
    let int8: i8 = -128; // 范围: -128 .. 127
    let int16: i16 = -32_768;
    let int32: i32 = -2_147_483_648; // 默认整数类型
    let int64: i64 = -9_223_372_036_854_775_808;
    let int_arch: isize = -42; // 指针宽度：32位架构 = 32位，64位架构 = 64位

    println!("    i8    = {}   (范围: -128..127)", int8);
    println!("    i16   = {}  (范围: -32768..32767)", int16);
    println!("    i32   = {} (默认整数类型)", int32);
    println!("    i64   = {}", int64);
    println!("    isize = {}  (指针宽度)", int_arch);

    // --- 无符号整数 ---
    let uint8: u8 = 255;
    let uint16: u16 = 65_535;
    let uint32: u32 = 4_294_967_295;
    let uint64: u64 = 18_446_744_073_709_551_615;
    let uint_arch: usize = 1_000; // 指针宽度无符号版，常用于索引和长度

    println!("    u8    = {}  (范围: 0..255)", uint8);
    println!("    u16   = {}", uint16);
    println!("    u32   = {}", uint32);
    println!("    u64   = {}", uint64);
    println!("    usize = {}  (常用于数组索引和集合长度)", uint_arch);

    // --- 浮点数 ---
    let float32: f32 = 3.14159;
    let float64: f64 = 2.718281828459045; // 默认浮点类型

    println!("    f32   = {}", float32);
    println!("    f64   = {} (默认浮点类型)", float64);

    // --- 布尔值 ---
    let t: bool = true;
    let f: bool = false;
    println!("    bool  = {} 或 {}", t, f);

    // --- 字符 ---
    // Rust 的 char 是 4 字节，表示一个 Unicode 标量值（scalar value），
    // 而不仅仅是 ASCII。这和 C 语言的 1 字节 char 完全不同。
    let letter: char = 'A';
    let emoji: char = '🦀'; // 螃蟹 emoji，也是一个合法的 char
    let chinese: char = '你';
    println!(
        "    char  = '{}' '{}' '{}' (每个 char 占 4 字节，Unicode 标量值)",
        letter, emoji, chinese
    );
    println!();

    // --- 打印各类型内存大小 ---
    println!("  各类型在栈上的内存大小 (字节)：");
    print_type_info::<i8>("i8");
    print_type_info::<i16>("i16");
    print_type_info::<i32>("i32");
    print_type_info::<i64>("i64");
    print_type_info::<isize>("isize");
    print_type_info::<u8>("u8");
    print_type_info::<u16>("u16");
    print_type_info::<u32>("u32");
    print_type_info::<u64>("u64");
    print_type_info::<usize>("usize");
    print_type_info::<f32>("f32");
    print_type_info::<f64>("f64");
    print_type_info::<bool>("bool");
    print_type_info::<char>("char"); // 注意：char 是 4 字节！
    println!();

    // --- 类型推断与显式标注 ---
    println!("--- 5. 类型推断 (Type Inference) ---");

    // 编译器根据上下文推断类型
    let inferred_int = 42; // 默认推断为 i32
    let inferred_float = 3.14; // 默认推断为 f64
    let mut inferred_vec = Vec::new(); // 推断为某类型的 Vec，等第一次 push 时确定
    inferred_vec.push(1u8); // 现在编译器知道是 Vec<u8>
    println!("  inferred_int   = {} (编译器推断为 i32)", inferred_int);
    println!("  inferred_float = {} (编译器推断为 f64)", inferred_float);
    println!(
        "  inferred_vec   = {:?} (编译器推断为 Vec<u8>)",
        inferred_vec
    );

    // 当编译器无法推断时，需要显式标注类型
    let parsed: u32 = "42".parse().expect("解析失败");
    println!(
        "  parsed (u32)   = {} (必须标注类型，否则 parse 不知道目标类型)",
        parsed
    );
    println!();

    // ========================================================
    // 第六部分：复合类型 (Compound Types)
    // ========================================================
    println!("--- 6. 元组 (Tuple) ---");

    // 创建元组：可以包含任意数量和类型的值
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    println!("  tup = {:?}", tup);

    // 方式一：模式解构 (Pattern Destructuring)
    let (a, b, c) = tup;
    println!("  模式解构: a={}, b={}, c='{}'", a, b, c);

    // 方式二：句点索引 (Dot Indexing)，从 0 开始
    println!(
        "  索引访问: tup.0={}, tup.1={}, tup.2='{}'",
        tup.0, tup.1, tup.2
    );

    // 单元元组 (Unit Tuple)：空元组，类似 Python 的 None / 无返回值
    let unit: () = ();
    println!("  unit (单元元组) = {:?}", unit);
    println!("  说明: () 是零大小类型，类似 Python 的 None，但它是类型而非值\n");

    println!("--- 7. 数组 (Array) ---");

    // 数组：固定长度，所有元素类型相同，分配在栈上
    // 类型签名：[T; N] 其中 T 是元素类型，N 是编译期常量长度
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("  arr = {:?}", arr);
    println!("  类型签名: [i32; 5] — 5 个 i32 元素");

    // 简化写法：用分号创建包含相同值的数组
    let zeros: [u8; 8] = [0; 8]; // 8 个 0
    println!("  zeros = {:?}", zeros);

    // 数组索引：从 0 开始
    println!(
        "  arr[0] = {}, arr[1] = {}, arr[4] = {}",
        arr[0], arr[1], arr[4]
    );

    // 数组长度
    println!("  arr.len() = {}", arr.len());

    // 重要：Rust 的数组索引在运行时做边界检查 (bounds checking)。
    // 如果越界，程序会 panic（崩溃），而不是返回未定义行为。
    //
    // 下面这行如果取消注释会在运行时 panic：
    //     println!("{}", arr[10]); // thread 'main' panicked: index out of bounds
    //
    // 这和 C/C++ 完全不同（C 语言越界是未定义行为），也不同于 Python
    // 的 IndexError（可以被 try/except 捕获）。
    // Rust 中可以使用 arr.get(10) 返回 Option 来安全访问。
    match arr.get(10) {
        Some(val) => println!("  越界访问不安全: {}", val),
        None => println!("  arr.get(10) 安全返回 None（而不是 panic）"),
    }
    println!();

    // ========================================================
    // 第七部分：类型转换 (Type Casting)
    // ========================================================
    println!("--- 8. 类型转换 (casting with `as`) ---");

    let original: i32 = 300;
    // 使用 `as` 关键字进行显式类型转换
    let as_u8: u8 = original as u8; // i32 -> u8，可能截断
    let as_f64: f64 = original as f64; // i32 -> f64
    let as_char: char = 65u8 as char; // ASCII 码 -> 字符

    println!("  i32 ({}) -> u8  = {} (注意：可能截断)", original, as_u8);
    println!("  i32 ({}) -> f64 = {}", original, as_f64);
    println!("  65u8 -> char    = '{}'", as_char);

    // 从浮点数到整数的转换会截断小数部分
    let pi: f64 = 3.14159;
    let pi_as_i32: i32 = pi as i32;
    println!("  f64 ({}) -> i32 = {} (小数部分被截断)", pi, pi_as_i32);
    println!();

    // ========================================================
    // 结束
    // ========================================================
    println!("===== 演示结束 =====");
}
