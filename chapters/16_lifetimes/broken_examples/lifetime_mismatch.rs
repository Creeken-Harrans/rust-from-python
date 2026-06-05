// ============================================================================
// broken_examples/lifetime_mismatch.rs
// ============================================================================
//
// 生命周期不匹配（Lifetime Mismatch）—— 当标注和实际不符
//
// 下面的代码演示了几种常见的生命周期不匹配情况。
// 所有标记为 ❌ 的代码都无法编译通过。

// 情况 1：返回值试图使用两个不同生命周期的参数
// 编译器无法确定返回值的生命周期应该关联到谁
//
// ⚠️ 注意：下面这种写法本身就会在编译时被拒绝：
//
// fn ambiguous_return<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//     // ❌ 编译错误! 编译器不知道返回值的生命周期是来自 'a 还是 'b
//     if x.len() > y.len() { x } else { y }
// }
//
// 编译器错误信息大致：
//   error[E0106]: missing lifetime specifier
//     --> this function's return type contains a borrowed value,
//         but the signature does not say whether it is borrowed from `x` or `y`
//
// 这就是为什么 longest 函数必须写 fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// 通过让两者共享同一个生命周期参数 'a，编译器知道返回值不能比
// 两个参数中"活得短的"那个活得更久。


// 情况 2：结构体字段的生命周期和实际数据不匹配
//
// struct Holder<'a> {
//     data: &'a str,
// }
//
// fn create_holder() {
//     let local = String::from("局部数据");
//     // ❌ 编译错误! local 活得不够久
//     // let holder = Holder { data: &local };
//     // 实际上如果 holder 被返回/移出，local 会被 drop
// }


// 情况 3：返回值的生命周期比参数更"长"——逻辑上不可能
//
// 你不能要求一个引用比它指向的数据活得更久。
//
// fn return_longer<'a>(x: &'a str) -> &'static str {
//     // ❌ 编译错误！x 的生命周期是 'a，可能比 'static 短
//     x
// }


// 情况 4：不同作用域导致的隐性生命周期冲突
//
// fn scope_mismatch() {
//     let result;
//     {
//         let inner = String::from("inner scope string");
//         // ❌ 编译错误! inner 活得不够久
//         // result = longest("outer", &inner);
//     }  // inner 在这里释放
//     // result 在这里使用——但 inner 已经不存在了
// }


// --------------- 正确的写法 ---------------

struct Holder<'a> {
    data: &'a str,
}

impl<'a> Holder<'a> {
    fn new(s: &'a str) -> Self {
        Holder { data: s }
    }

    fn announce_and_return<'b>(&self, announcement: &'b str) -> &'a str {
        println!("公告: {}", announcement);
        self.data
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("=== 生命周期不匹配示例 ===");
    println!();

    // 正确做法：确保数据比引用活得更久
    let data = String::from("持久数据");
    let holder = Holder::new(&data);
    let result = holder.announce_and_return("这是一个公告");
    println!("holder 返回: \"{}\"", result);
    println!("原始 data: \"{}\"", data);

    // 正确做法：让两个引用在同一个作用域内
    let outer = String::from("外部字符串");
    let inner = String::from("内部字符串");
    let longer = longest(&outer, &inner);
    println!("longest 结果: \"{}\"", longer);

    println!();
    println!("重点：生命周期不匹配在所有正确编写的 Rust 代码中");
    println!("都会被编译器在编译时捕获，不会等到运行时才崩溃。");
    println!();
    println!("如果你遇到生命周期编译错误，通常只需要问自己两个问题：");
    println!("  1. 谁拥有这个数据？谁负责释放它？");
    println!("  2. 引用是否可能比它指向的数据活得更久？");
}
