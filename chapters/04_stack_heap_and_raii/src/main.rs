#![allow(clippy::approx_constant)]
// ============================================================================
// 栈、堆与 RAII — Rust 内存管理基础演示
// ============================================================================
// 本章演示 Rust 中两种内存区域的核心概念:
//   - 栈 (Stack): LIFO 结构, 存放固定大小的数据 (Copy 类型)
//   - 堆 (Heap): 动态内存, 存放运行时大小可变的数据 (如 String, Vec)
//   - RAII: 资源获取即初始化 — 离开作用域时自动调用 Drop
//
// Rust 2024 edition 语法:
//   - 使用 pub(crate) / pub(super) 替代旧版路径
//   - 使用 {} 匹配时不再需要 `ref` 关键字 (binding modes 增强)
//   - 使用 impl 块中 `use` 语句的简化语法
// ============================================================================

use std::mem::size_of;

// ---------------------------------------------------------------------------
// Resource: 自定义结构体, 实现了 Drop trait, 用于观察析构顺序
// ---------------------------------------------------------------------------
// 当 Resource 离开作用域时, Rust 会自动调用 drop(), 打印一条日志。
// 这让你直观地看到 "谁先释放, 谁后释放"。
struct Resource {
    name: String, // String 字段, 数据在堆上
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("  -> 创建 Resource: \"{name}\"");
        Resource {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        // 析构函数: Rust 保证在值离开作用域时调用
        println!("  <- 释放 Resource: \"{}\"", self.name);
    }
}

// ---------------------------------------------------------------------------
// StackData: 仅包含固定大小字段的结构体 (全部为 Copy 类型)
// ---------------------------------------------------------------------------
// 因为所有字段都是 Copy 类型 (i32, f64, bool), 整个结构体可以
// 完全存放在栈上, 并可标记为 Copy。
#[derive(Debug, Clone, Copy)]
struct StackData {
    x: i32,
    y: f64,
    flag: bool,
}

// ---------------------------------------------------------------------------
// demonstrate_stack: 展示栈上的数据及其固定大小
// ---------------------------------------------------------------------------
fn demonstrate_stack() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  第一部分: 栈 (Stack) — 固定大小的数据        ║");
    println!("╚══════════════════════════════════════════════╝");

    // --- 基本标量类型 ---
    // 这些类型的大小在编译期已知, 直接放在栈上。
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = 'R';

    println!("\n--- 基本标量类型 (栈上) ---");
    println!(
        "  i32      值 = {integer},     size_of = {} bytes",
        size_of::<i32>()
    );
    println!(
        "  f64      值 = {float},  size_of = {} bytes",
        size_of::<f64>()
    );
    println!(
        "  bool     值 = {boolean},     size_of = {} bytes",
        size_of::<bool>()
    );
    println!(
        "  char     值 = {character},     size_of = {} bytes",
        size_of::<char>()
    );

    // --- 固定大小数组 ---
    // 数组的类型是 [T; N], 大小在编译期确定, 整个数组在栈上。
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("\n--- 固定大小数组 (栈上) ---");
    println!("  [i32; 5] arr = {arr:?}");
    println!("  size_of::<[i32; 5]>() = {} bytes", size_of::<[i32; 5]>());
    println!("  (5 个 i32 × 4 bytes = 20 bytes)");

    // --- 元组 ---
    // 元组可以包含不同类型, 但只要每个元素大小固定, 整个元组就在栈上。
    let tuple: (i32, f64, bool) = (100, 2.718, false);
    println!("\n--- 元组 (栈上) ---");
    println!("  (i32, f64, bool) tuple = {tuple:?}");
    println!(
        "  size_of::<(i32, f64, bool)>() = {} bytes",
        size_of::<(i32, f64, bool)>()
    );
    println!("  (4 + 8 + 1 = 13 bytes, 但编译器可能有对齐填充)");

    // --- StackData 结构体 ---
    let sd = StackData {
        x: 10,
        y: 20.5,
        flag: true,
    };
    println!("\n--- 自定义结构体 StackData (栈上) ---");
    println!("  sd = {sd:?}");
    println!("  sd.x={}, sd.y={}, sd.flag={}", sd.x, sd.y, sd.flag);
    println!(
        "  size_of::<StackData>() = {} bytes",
        size_of::<StackData>()
    );
    println!("  (i32=4 + f64=8 + bool=1 = 13, 加上对齐可能为 16 或 24)");

    // --- 引用的 size ---
    // 引用 (&T) 本质上是一个指针, 大小等于 usize。
    let _ref_to_sd: &StackData = &sd;
    println!("\n--- 引用的大小 (也是栈上的一个指针) ---");
    println!(
        "  size_of::<&StackData>() = {} bytes (在 64 位系统上等于 8)",
        size_of::<&StackData>()
    );
    println!("  (引用本身在栈上, 指向的数据可能在栈或堆上)");
}

// ---------------------------------------------------------------------------
// demonstrate_heap: 展示堆上的数据及栈上的 "句柄"
// ---------------------------------------------------------------------------
fn demonstrate_heap() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  第二部分: 堆 (Heap) — 动态大小的数据         ║");
    println!("╚══════════════════════════════════════════════╝");

    // --- String ---
    // String 在栈上的部分只有 {ptr, len, cap} (24 bytes on 64-bit)。
    // 实际的字符串内容 (UTF-8 字节) 存储在堆上。
    let s = String::from("Hello, 堆内存!");
    println!("\n--- String: 栈上句柄 + 堆上数据 ---");
    println!("  s = \"{s}\"");
    println!("  s.len()     = {}  (字符串字节长度)", s.len());
    println!("  s.capacity() = {}  (已分配的堆缓冲区大小)", s.capacity());
    println!(
        "  size_of::<String>() = {} bytes  (仅句柄, 不含堆数据!)",
        size_of::<String>()
    );

    // 解释 String 的 24 bytes 构成:
    //   - 指针 (ptr):  8 bytes — 指向堆上的字节数组
    //   - 长度 (len):  8 bytes — 当前有效字节数
    //   - 容量 (cap):  8 bytes — 已分配字节数 (含预留空间)
    println!("  String 栈上结构: ptr(8) + len(8) + cap(8) = 24 bytes");

    // --- Vec<i32> ---
    // Vec 和 String 完全相同: 栈上 24 bytes 句柄, 数据在堆上。
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("\n--- Vec<i32>: 同样栈上句柄 + 堆上数据 ---");
    println!("  v = {v:?}");
    println!("  v.len()     = {}  (元素个数)", v.len());
    println!("  v.capacity() = {}  (已分配的堆容量)", v.capacity());
    println!("  size_of::<Vec<i32>>() = {} bytes", size_of::<Vec<i32>>());
    println!(
        "  堆上数据占用: {} i32 × 4 bytes = {} bytes",
        v.len(),
        v.len() * 4
    );

    // --- Box<T> ---
    // Box<T> 是单纯的指针 — 只有一个 ptr (8 bytes on 64-bit)。
    let boxed: Box<i32> = Box::new(999);
    println!("\n--- Box<T>: 单纯的堆指针 ---");
    println!("  boxed = {boxed}");
    println!(
        "  size_of::<Box<i32>>()  = {} bytes (就是一个指针)",
        size_of::<Box<i32>>()
    );
    println!(
        "  size_of::<Box<[i32; 100]>>() = {} bytes (同样一个指针)",
        size_of::<Box<[i32; 100]>>()
    );
    println!("  (Box 将任意大小的数据放到堆上, 但栈上只放 8 bytes 指针)");

    // --- 对比总结 ---
    println!("\n--- 栈 vs 堆 大小对比 ---");
    println!("  ┌─────────────────────┬──────────────┬────────────────────┐");
    println!("  │ 类型                │ size_of (栈) │ 实际数据在哪       │");
    println!("  ├─────────────────────┼──────────────┼────────────────────┤");
    println!(
        "  │ i32                 │ {:>4} bytes    │ 栈                 │",
        size_of::<i32>()
    );
    println!(
        "  │ [i32; 5]            │ {:>4} bytes    │ 栈                 │",
        size_of::<[i32; 5]>()
    );
    println!(
        "  │ String              │ {:>4} bytes    │ 栈(句柄) + 堆(数据)│",
        size_of::<String>()
    );
    println!(
        "  │ Vec<i32>            │ {:>4} bytes    │ 栈(句柄) + 堆(数据)│",
        size_of::<Vec<i32>>()
    );
    println!(
        "  │ Box<i32>            │ {:>4} bytes    │ 栈(指针) + 堆(数据)│",
        size_of::<Box<i32>>()
    );
    println!("  └─────────────────────┴──────────────┴────────────────────┘");
}

// ---------------------------------------------------------------------------
// demonstrate_scope_and_drop: 作用域与 Drop 顺序
// ---------------------------------------------------------------------------
// RAII 的核心规则: 资源在离开作用域时自动释放。
// 释放顺序是 LIFO (后进先出) — 后创建的先析构, 符合栈的结构。
fn demonstrate_scope_and_drop() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  第三部分: 作用域 (Scope) 与 Drop 顺序       ║");
    println!("╚══════════════════════════════════════════════╝");

    println!("\n--- 例子 1: 同一作用域内的创建与释放顺序 ---");
    {
        println!("进入作用域 A");
        let _a = Resource::new("A");
        let _b = Resource::new("B");
        let _c = Resource::new("C");
        // 可见: 创建顺序 A → B → C
        println!("即将离开作用域 A (注意释放顺序是相反的: C → B → A)");
    } // <- _c, _b, _a 按逆序析构

    println!("\n--- 例子 2: 嵌套作用域 ---");
    {
        println!("进入外层作用域");
        let _outer = Resource::new("外层");

        {
            println!("  进入内层作用域");
            let _inner = Resource::new("内层");
            println!("  即将离开内层作用域");
        } // <- _inner 在此析构, _outer 仍在

        println!("内层已退出的证明: _inner 已释放, _outer 仍存活");
        let _another = Resource::new("另一个外层");
    } // <- _another, _outer 按逆序析构

    println!("\n--- 例子 3: 提前 drop 释放 ---");
    {
        let early = Resource::new("提前释放");
        // std::mem::drop() 主动释放一个值, 不等其离开作用域。
        // 这并非 free — 而是告诉 Rust: "我现在不需要这个值了"。
        println!("手动调用 drop(early)...");
        drop(early);
        println!("early 已被主动释放, 不需要等到作用域结束");
        // 注意: 如果在此之后尝试使用 early, 编译器会报错
        // println!("{}", early.name); // 编译错误: use after move
    }
}

// ---------------------------------------------------------------------------
// string_internals_demo: String 内部结构详解
// ---------------------------------------------------------------------------
// String 在栈上: { ptr: *mut u8, len: usize, cap: usize }
// 实际字节在堆上: [H][e][l][l][o][...]
fn string_internals_demo() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  第四部分: String 内部结构深入               ║");
    println!("╚══════════════════════════════════════════════╝");

    let s1 = String::from("Hello");
    println!("  s1 = \"{s1}\"");
    println!("  s1.len()      = {}", s1.len());
    println!("  s1.capacity() = {}", s1.capacity());
    println!("  size_of::<String>() = {} bytes", size_of::<String>());

    // 演示 push 如何影响 capacity
    let mut s2 = String::with_capacity(4);
    println!("\n--- 观察 String 的容量增长 ---");
    println!(
        "  初始 (with_capacity(4)): len={}, cap={}",
        s2.len(),
        s2.capacity()
    );

    s2.push_str("AB");
    println!(
        "  push_str(\"AB\") 后:      len={}, cap={}",
        s2.len(),
        s2.capacity()
    );

    s2.push_str("CD");
    println!(
        "  push_str(\"CD\") 后:      len={}, cap={}",
        s2.len(),
        s2.capacity()
    );

    s2.push('E');
    println!(
        "  push_str(\"E\") 后:       len={}, cap={}  <-- cap 自动增长!",
        s2.len(),
        s2.capacity()
    );

    // 堆内存布局示意
    println!("\n--- String 内存布局示意 (以 s2 为例) ---");
    println!("  栈上 (String 句柄):");
    println!("    ┌──────────────────┐");
    println!("    │ ptr  → 0xHEAP... │  (8 bytes, 指向堆)");
    println!("    │ len  = {}        │  (8 bytes)", s2.len());
    println!("    │ cap  = {}         │  (8 bytes)", s2.capacity());
    println!("    └──────────────────┘");
    println!("  堆上 (实际数据):");
    println!("    ┌─┬─┬─┬─┬───┬─────────┐");
    print!("    │");
    for byte in s2.as_bytes() {
        print!("{:02x}|", byte);
    }
    println!();
    print!("    │");
    for byte in s2.as_bytes() {
        print!("{:>2} |", *byte as char);
    }
    println!();
    println!("    └─┴─┴─┴─┴───┴─────────┘");
    println!(
        "    有效数据 (len={}), 剩余容量 (cap-len={})",
        s2.len(),
        s2.capacity() - s2.len()
    );
}

// ---------------------------------------------------------------------------
// main: 入口函数, 依次调用所有演示
// ---------------------------------------------------------------------------
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  Rust 内存基础: 栈 (Stack) · 堆 (Heap) · RAII       ║");
    println!("║  本章演示栈/堆的区别, String 内部结构, Drop 机制     ║");
    println!("╚══════════════════════════════════════════════════════╝");

    // 环境信息
    println!("\n环境信息:");
    println!(
        "  usize = {} bytes (指针大小, {} 位系统)",
        size_of::<usize>(),
        (usize::BITS as usize)
    );

    demonstrate_stack();
    demonstrate_heap();
    demonstrate_scope_and_drop();
    string_internals_demo();

    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║  演示结束。                                         ║");
    println!("║  核心要点:                                          ║");
    println!("║  · 栈: 固定大小, 快速, LIFO                         ║");
    println!("║  · 堆: 动态大小, 灵活, 需要指针间接访问              ║");
    println!("║  · RAII: 离开作用域自动释放, 无需手动 free          ║");
    println!("╚══════════════════════════════════════════════════════╝");
}

// ============================================================================
// 注意事项:
// 1. Rust 2024 edition 要求使用 size_of 而不是旧的 std::mem::size_of 路径
//    (实际两者都可用, 但 use 语句更清晰)
// 2. Drop trait 是 Rust 内存安全的核心 — 你永远不需要手动调用 free()
// 3. Copy trait 表示值可以通过简单的内存拷贝来复制 (不涉及所有权转移)
// 4. String/Vec 不能实现 Copy, 因为拷贝其栈上句柄会导致双重释放
// ============================================================================
