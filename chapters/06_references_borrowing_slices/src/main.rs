#![allow(rustdoc::broken_intra_doc_links, clippy::ptr_arg)]
// ============================================================================
// 第六章：引用、借用与切片
// References, Borrowing & Slices
//
// 核心概念：
// - 引用 (Reference)：不获取所有权地访问数据，使用 & 创建，使用 * 解引用
// - 借用 (Borrowing)：创建引用的行为，就像借书——你仍拥有书，别人可以读
// - 借用规则：
//   1. 任意时刻，可以有多个不可变引用 (&T)，或者恰好一个可变引用 (&mut T)
//   2. 引用必须始终有效（Rust 禁止悬垂引用）
// - 切片 (Slice)：对连续数据的"借用视图"，如 &str（字符串切片）、&[T]（数组切片）
// ============================================================================

/// 演示引用 (&) 和解引用 (*)
///
/// 引用就像指向数据的"安全指针"——你可以通过它读取数据，
/// 但不拥有数据本身。解引用 (*) 可以从引用中获取所指向的值。
fn demonstrate_references() {
    println!("========== 1. 引用 (&) 与解引用 (*) ==========");

    // 创建一个 String（所有权在 s）
    let s = String::from("Hello, Rust!");

    // 创建一个对 s 的引用——不转移所有权！
    // r 的类型是 &String，它是一个引用，而非 String 本身
    let r: &String = &s;

    println!("s 的值: {s}");
    println!("r 的值: {r}"); // 自动解引用，r 可以直接显示
    println!("r 指向的地址: {:p}", r); // 打印引用 r 的指针地址
    println!("s 的地址: {:p}", &s); // 打印 s 的地址——两者相同！

    // 解引用：使用 * 获取引用指向的实际值
    // 注意：*r 会尝试移动 String，所以这里用 &*r 或者直接比较
    // 实际上在 println 中 Rust 会自动解引用
    println!("*r 解引用后的值: {}", *r);

    // 所有权仍然在 s——我们可以继续使用 s
    println!("所有权仍在 s: {s}");
    println!("s 的长度: {}", s.len());
    drop(s); // s 在这里被释放，r 在之前已经不再使用

    // 对整数等 Copy 类型的引用和解引用
    let x = 42;
    let rx = &x;
    let y = *rx; // 因为 i32 是 Copy 类型，*rx 复制了值
    println!("x = {x}, *rx = {}, y = {y}", *rx);

    println!();
}

/// 演示不可变借用 (&T)
///
/// 核心规则：**可以有多个不可变引用**，它们都是只读的。
/// 这就像多个人同时读同一本书——互不干扰。
fn demonstrate_immutable_borrows() {
    println!("========== 2. 不可变借用 (&T) —— 多个引用共存 ==========");

    let text = String::from("学习 Rust 引用");

    // 同时创建三个不可变引用——完全合法！
    let r1 = &text;
    let r2 = &text;
    let r3 = &text;

    // 所有引用都可以正常使用
    println!("r1: {r1}");
    println!("r2: {r2}");
    println!("r3: {r3}");

    // 原变量仍可用（因为引用不转移所有权）
    println!("text 仍可用: {text}");

    // 你甚至可以把引用传给函数
    print_length(&text);
    print_length(r1);
    print_length(r2);

    println!();
}

/// 一个接受不可变引用的简单函数
fn print_length(s: &String) {
    println!("  -> 引用内容的长度: {}", s.len());
}

/// 演示可变借用 (&mut T)
///
/// 核心规则：**同一时刻只能有一个可变引用**。
/// 可变引用给予你修改数据的独占权限。
fn demonstrate_mutable_borrow() {
    println!("========== 3. 可变借用 (&mut T) —— 独占修改权限 ==========");

    // 注：变量本身需要是 mut 的，才能被可变借用
    let mut message = String::from("Hello");

    println!("修改前: {message}");

    // 创建一个可变引用
    let rm = &mut message;

    // 通过可变引用修改数据
    rm.push_str(", World!");
    rm.push('!');

    println!("通过 &mut 修改后: {rm}");

    // rm 最后一次使用在此——之后不可变引用可以创建
    // （这就是 NLL 的作用，见下一节）

    // 现在可以创建不可变引用了
    let ri = &message;
    println!("不可变引用查看: {ri}");
    println!("最终 message: {message}");

    println!();
}

/// 演示通过可变引用修改结构体字段
fn modify_through_ref() {
    println!("--- 通过 &mut 修改结构体字段 ---");

    struct Point {
        x: i32,
        y: i32,
    }

    let mut p = Point { x: 10, y: 20 };

    let rp = &mut p;
    rp.x += 5; // 通过可变引用修改字段
    rp.y *= 2; // 可以修改多个字段

    println!("修改后的 Point: ({}, {})", p.x, p.y);
    println!();
}

/// 演示非词法生命周期 (Non-Lexical Lifetimes, NLL)
///
/// NLL 的含义：引用的"生命周期"不再严格等于其所在的代码块（词法作用域），
/// 而是基于**实际使用**。编译器在引用的最后一次使用之后，就认为它"结束"了。
///
/// 这意味着：在可变引用的最后一次使用之后，你可以立即创建不可变引用——
/// 即使它们在同一个代码块中。
fn demonstrate_nll() {
    println!("========== 4. 非词法生命周期 (NLL) ==========");

    let mut data = String::from("NLL 演示");

    // 创建可变引用
    let rm = &mut data;
    rm.push_str("：可变引用在这里被最后一次使用");
    // ↑ rm 的最后一次使用——在此之后 rm 就"失效"了
    // 即使我们还在同一个代码块中！

    // 现在可以创建不可变引用——NLL 让这成为可能
    // 如果没有 NLL（Rust 2015 风格），这段代码会编译失败
    let ri1 = &data;
    let ri2 = &data;

    println!("不可变引用 ri1: {ri1}");
    println!("不可变引用 ri2: {ri2}");

    // 甚至可以在不可变引用的最后使用之后再创建新的可变引用
    // ri1 和 ri2 在上面 println 中最后一次使用
    // NLL 认为它们在此之后不再活跃
    let _ = ri1; // 显式标记不再使用（实际不需要，NLL 自动处理）
    let _ = ri2;

    let rm2 = &mut data; // 新的可变引用——合法！
    rm2.push_str("：再次修改");
    println!("最终结果: {rm2}");

    println!();
}

/// 经典示例：找到字符串中的第一个单词
///
/// 参数是 &str（字符串切片），返回的是 &str（同样是切片引用）。
/// 不分配新内存，不获取所有权——纯粹借用。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // 返回从开头到第一个空格之间的切片
        }
    }

    s // 没有空格，返回整个字符串的切片
}

/// 演示字符串切片 (&str)
///
/// 切片是对集合中一部分数据的"借用视图"。
/// &str 是 String（或字符串字面量）的切片引用。
///
/// 【重要】对于 ASCII 字符（英文），1 字符 = 1 字节，边界安全。
/// 对于 UTF-8 字符（中文），1 字符可能 = 3 字节，必须对齐字符边界。
fn demonstrate_slices() {
    println!("========== 5. 字符串切片 (&str) ==========");

    // 使用 ASCII 字符串演示切片——每个字符刚好 1 字节，任意索引都安全
    let sentence = String::from("Rust is a systems programming language");

    // 从 String 创建各种切片（英文：所有字节边界也是字符边界）
    let full: &str = &sentence[..]; // 完整切片
    let first_five: &str = &sentence[0..5]; // 前5个字节："Rust "
    let start_five: &str = &sentence[..5]; // 等价于 [0..5]
    let from_three: &str = &sentence[3..]; // 从第3个字节到末尾："t is a..."
    let partial: &str = &sentence[10..24]; // 第10-24字节："systems progr"

    println!("原始 String:           '{sentence}'");
    println!("完整切片 [..]:          '{full}'");
    println!("前5字节 [0..5]:         '{first_five}'");
    println!("前5字节 [..5]:          '{start_five}'");
    println!("第3字节起 [3..]:        '{from_three}'");
    println!("第10-24字节 [10..24]:   '{partial}'");

    // UTF-8 注意事项：中文字符占 3 个字节，切片必须在字符边界上
    let chinese = String::from("你好世界");
    // 字节布局: 你(0-2) 好(3-5) 世(6-8) 界(9-11)
    // 有效边界: 0, 3, 6, 9, 12
    let ni_hao: &str = &chinese[..6]; // ✅ 边界 6 = "你好"
    // let bad: &str = &chinese[..4];   // ❌ 边界 4 在"好"中间，运行时 panic!
    println!("中文 '你好世界' 的前两字 [..6]:  '{ni_hao}'");

    // 字符串字面量本身就是 &str
    let literal: &str = "我是一个字符串字面量";
    println!("字面量 &str: {literal}");

    // 使用 first_word 函数
    let words_input = String::from("hello world from Rust");
    let word = first_word(&words_input);
    println!("first_word('{words_input}') = '{word}'");

    let no_space = first_word("NoSpaceHere!");
    println!("first_word('NoSpaceHere!') = '{no_space}'");

    // 重要：切片引用不能超出原数据生命周期
    // 下面是一个**编译错误**的示例（被注释掉），展示悬垂引用：
    //
    // let slice_from_fn = {
    //     let tmp = String::from("临时数据");
    //     first_word(&tmp) // 切片引用从 tmp 借用
    //     // tmp 在这里被释放——但 slice_from_fn 试图持有 &tmp 的引用？
    //     // 编译器错误：`tmp` does not live long enough
    // };
    // println!("{slice_from_fn}"); // 如果编译器不阻止，这里就是悬垂引用
    //
    // Rust 在编译时阻止了这种悬垂引用，保证内存安全。

    println!();
}

/// 演示数组切片 (&[i32])
///
/// 数组切片 &[T] 是对数组一部分的借用视图，
/// 与 &str 是 String 的切片视图完全相同的模式。
fn demonstrate_array_slices() {
    println!("========== 6. 数组切片 (&[T]) ==========");

    let numbers = [10, 20, 30, 40, 50, 60, 70, 80];

    // 创建数组切片——不复制数据，只是"视图"
    let all: &[i32] = &numbers[..]; // 完整切片
    let first_three: &[i32] = &numbers[..3]; // 前3个元素
    let mid: &[i32] = &numbers[2..5]; // 第2到第5个元素
    let from_four: &[i32] = &numbers[4..]; // 从第4个到末尾

    println!("原始数组:    {numbers:?}");
    println!("完整切片:    {all:?}");
    println!("前3个 [..3]:  {first_three:?}");
    println!("第2-5 [2..5]: {mid:?}");
    println!("第4起 [4..]:  {from_four:?}");

    // 将切片传给函数
    let total = sum_slice(&numbers[..]);
    println!("完整数组求和: {total}");

    let part_total = sum_slice(first_three);
    println!("前3个元素求和: {part_total}");

    // 也可以直接对数组字面量切片
    let literal_sum = sum_slice(&[1, 2, 3, 4, 5]);
    println!("字面量数组求和: {literal_sum}");

    // 对可变数组的可变切片
    let mut values = [100, 200, 300, 400, 500];
    {
        let slice_mut: &mut [i32] = &mut values[1..4]; // 可变切片
        slice_mut[0] = 999; // 通过可变切片修改原数组
        slice_mut[2] = 777;
    }
    println!("通过可变切片修改后: {values:?}");

    println!();
}

/// 接受数组切片引用，计算所有元素之和
///
/// 注意：&[i32] 是一个"胖指针"——包含起始地址和长度。
/// 这意味着不需要单独传递长度参数。Rust 会在运行时进行边界检查。
fn sum_slice(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    sum
}

/// 在一个句子中找到最长的单词
///
/// 泛型生命周期语法 'a 告诉编译器：返回的引用与参数 text 的引用
/// 具有相同的生命周期。这是 Rust 确保引用始终有效的方式。
///
/// 当前只是"预览"生命周期语法——完整讨论在后面的章节中。
fn longest_word(text: &str) -> &str {
    let mut longest: &str = "";
    let mut max_len = 0;

    for word in text.split_whitespace() {
        let len = word.chars().count(); // 使用字符数而非字节数（正确处理 UTF-8）
        if len > max_len {
            max_len = len;
            longest = word;
        }
    }

    longest
}

/// 演示生命周期标注的必要性
///
/// 当函数返回引用时，编译器需要知道该引用从哪个参数借用。
/// 如果有多个引用参数，生命周期标注告诉编译器它们之间的关系。
fn demonstrate_lifetime_preview() {
    println!("========== 7. 生命周期标注预览 ==========");

    let text = String::from("Rust 的生命周期确保内存安全");
    let result = longest_word(&text);
    println!("原文: '{text}'");
    println!("最长单词: '{result}'");

    // 生命周期保证了引用不会悬垂：
    // 下面的代码如果取消注释，编译器会拒绝：
    //
    // let dangling;
    // {
    //     let tmp = String::from("临时");
    //     dangling = &tmp; // 编译错误：`tmp` does not live long enough
    // }
    // println!("{dangling}"); // tmp 已释放，dangling 是悬垂引用！

    println!();
}

/// 总结借用规则
fn print_borrowing_rules_summary() {
    println!("========== 借用规则总结 ==========");
    println!();
    println!("规则 1: 在任意给定时间，你可以拥有 其中之一 而非两者：");
    println!("        - 任意数量的不可变引用 (&T)");
    println!("        - 恰好一个可变引用 (&mut T)");
    println!();
    println!("规则 2: 引用必须始终有效。");
    println!("        Rust 在编译时禁止悬垂引用 (dangling references)。");
    println!();
    println!("为什么？这些规则防止了 数据竞争 (data race)：");
    println!("  - 一个写操作 + 同时的读操作 -> 读到不完整数据");
    println!("  - 两个写操作同时发生   -> 数据损坏");
    println!("  - Rust 在编译时消除这些可能性，无需运行时检查！");
    println!();
    println!("Python 对比：");
    println!("  Python 中，所有对象都是引用计数的，没有所有权概念。");
    println!("  Python 切片如 s[0:5] 会创建新字符串（分配内存）。");
    println!("  Rust 的切片 &str 是零成本的借用视图（不分配内存）。");
    println!();
    println!("核心术语：");
    println!("  Reference      - 引用 (&)");
    println!("  Borrowing      - 借用");
    println!("  Dereference    - 解引用 (*)");
    println!("  Mutable Ref    - 可变引用 (&mut)");
    println!("  Immutable Ref  - 不可变引用 (&)");
    println!("  Slice          - 切片 (&str, &[T])");
    println!("  Dangling Ref   - 悬垂引用（Rust 禁止）");
    println!("  NLL            - 非词法生命周期");
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      第六章：引用、借用与切片                                ║");
    println!("║      References, Borrowing & Slices                         ║");
    println!("║      在不转移所有权的情况下安全地访问数据                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_references();
    demonstrate_immutable_borrows();
    demonstrate_mutable_borrow();
    modify_through_ref();
    demonstrate_nll();
    demonstrate_slices();
    demonstrate_array_slices();
    demonstrate_lifetime_preview();
    print_borrowing_rules_summary();

    println!("==============================================================");
    println!("所有演示完成！请阅读 README.md 了解更多细节。");
    println!("==============================================================");
}
