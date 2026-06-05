// ============================================================================
// broken_examples/dangling.rs
// ============================================================================
//
// 悬垂引用（Dangling Reference）—— 生命周期要解决的核心问题
//
// 下面的代码不 能编译通过。它演示了 Rust 的生命周期机制要防范的情况：
// 引用指向的数据已经被释放了。
//
// 如果你尝试编译这个文件（例如复制到 main.rs），你会得到如下错误：
//
//   error[E0597]: `value` does not live long enough
//     --> src/main.rs:xx:yy
//      |
//      |     let r = generate_dangling();
//      |             ------------------ `value` is dropped here while still borrowed
//      |     ...
//      |     println!("{}", r);
//      |                    - borrow later used here
//
// 这个错误证明：借用检查器在编译时就捕获了"use-after-free"问题，
// 不需要运行时检查，不需要垃圾回收器。

// 悬垂引用的经典例子：返回一个局部变量的引用
//
// fn generate_dangling() -> &str {
//     let value = String::from("这条数据在函数返回时会被释放");
//     &value  // ❌ 编译错误！value 在这里被 drop，但返回的引用还指向它
// }

// 正确的做法：返回拥有的数据（String），让调用者持有所有权
fn generate_owned() -> String {
    let value = String::from("这条数据的生命周期由调用者管理");
    value
}

// 或者：传入引用，返回引用——让生命周期和输入参数关联
fn process_existing<'a>(input: &'a str) -> &'a str {
    // 返回的引用和 input 指向同一块数据，编译器知道这是安全的
    input
}

// 另一个悬垂引用的例子：在作用域内创建值，在外面使用引用
//
// fn scope_dangling() {
//     let r;
//     {
//         let x = 5;
//         r = &x;  // ❌ 编译错误！x 活得不够久
//     }
//     // x 在这里已经被 drop 了，但 r 还想引用它
//     println!("r = {}", r);
// }

// 正确的做法：让引用和数据在同一个作用域内
fn scope_correct() {
    let x = 5;
    let r = &x;   // ✅ x 和 r 在同一个作用域内，x 活到作用域结束
    println!("r = {}", r);
}

fn main() {
    println!("=== 悬垂引用示例（不会执行——此文件仅供参考）===");
    println!();

    println!("这个文件演示了生命周期防止的悬垂引用问题。");
    println!("所有标记为 ❌ 的代码都无法通过编译。");
    println!();
    println!("如果你想自己验证，可以把 ❌ 的代码取消注释，");
    println!("然后尝试编译——你会看到编译器的错误信息精确地描述了问题。");
    println!();

    // 正确的做法可以运行
    let owned = generate_owned();
    println!("正确做法 produce_owned(): \"{}\"", owned);

    let input = String::from("原始数据");
    let reference = process_existing(&input);
    println!("正确做法 process_existing(): \"{}\"", reference);

    scope_correct();
    println!();
    println!("重点：Rust 在编译时发现悬垂引用，而不是等到运行时崩溃。");
    println!("这是 Rust 不使用 GC 也能保证内存安全的核心机制。");
}
