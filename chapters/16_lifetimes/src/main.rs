// ============================================================================
// 生命周期（Lifetimes）—— 理解引用有效性的关键
// ============================================================================
//
// 核心观念：
//   1. 生命周期标注（lifetime annotations）不会改变值实际存活的时间
//   2. 生命周期标注只描述多个引用之间的"关系"——谁和谁活得一样长
//   3. 借用检查器（borrow checker）一直在后台检查生命周期，
//      标注只是让关系变得显式，方便编译器验证（也方便人类理解）
//   4. 大多数日常代码中，生命周期省略规则（elision rules）自动处理，
//      你不需要手动写标注

// ---------------------------------------------------------------------------
// 经典示例：longest —— 必须显式标注生命周期
// ---------------------------------------------------------------------------

/// 返回两个字符串切片中较长的那个。
///
/// 生命周期 'a 的含义：
///   - 参数 x 和 y 的引用都必须至少存活 'a 这么久
///   - 返回值的引用也存活至少 'a 这么久
///   - 换句话说：返回值引用的数据，和 x、y 二者中"活得短的"那个一样长
///
/// 这里的 'a 并不改变任何值的实际存活时间；它只是告诉编译器：
/// "返回的引用不会比两个参数引用中的任何一个活得更久"。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// ---------------------------------------------------------------------------
// 简单生命周期 —— 省略规则可以自动推导
// ---------------------------------------------------------------------------

/// 返回字符串的第一个单词（空格前的内容）。
///
/// 这里没有写生命周期参数，但编译器根据"省略规则 2"自动推导出：
///   fn first_word<'a>(s: &'a str) -> &'a str
///
/// 因为只有一个输入引用，它的生命周期被自动赋给所有输出引用。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// ---------------------------------------------------------------------------
// 生命周期省略规则详解
// ---------------------------------------------------------------------------

/// 演示 Rust 的 3 条生命周期省略规则。
///
/// 规则 1：每个引用参数都获得自己的生命周期参数。
///   fn foo(x: &str, y: &str)           → fn foo<'a, 'b>(x: &'a str, y: &'b str)
///   fn foo(x: &str, y: &i32)           → fn foo<'a, 'b>(x: &'a str, y: &'b i32)
///
/// 规则 2：如果恰好只有一个输入生命周期参数，它被赋给所有输出生命周期。
///   fn foo(x: &str) -> &str            → fn foo<'a>(x: &'a str) -> &'a str
///   这就是为什么 first_word 不需要手动写标注。
///
/// 规则 3：如果有 &self 或 &mut self，它的生命周期被赋给所有输出生命周期。
///   fn method(&self, x: &str) -> &str  → fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str
///   注意：返回值的生命周期来自 &self，不是 x！
fn demonstrate_lifetime_elision() {
    println!("=== 生命周期省略（Lifetime Elision）规则 ===");
    println!();
    println!("Rust 编译器有 3 条省略规则，自动为常见模式推导生命周期：");
    println!();
    println!("规则 1: 每个引用参数各自获得独立的生命周期参数");
    println!("  fn foo(x: &str, y: &str)      展开为  fn foo<'a, 'b>(x: &'a str, y: &'b str)");
    println!();
    println!("规则 2: 若只有一个输入生命周期参数，则赋给所有输出引用");
    println!("  fn foo(x: &str) -> &str        展开为  fn foo<'a>(x: &'a str) -> &'a str");
    println!("  这就是 first_word 不需要标注的原因！");
    println!();
    println!("规则 3: 若方法中有 &self / &mut self，其生命周期赋给所有输出引用");
    println!(
        "  fn foo(&self, x: &str) -> &str 展开为  fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str"
    );
    println!();
    println!("注意：返回值的生命周期来自 &self，而不是参数 x！");
    println!();

    // 实际演示：first_word 编译通过，虽然我们没有写任何生命周期标注
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("省略规则实战: first_word(\"{}\") = \"{}\"", sentence, word);
    println!();
}

// ---------------------------------------------------------------------------
// 结构体中的生命周期标注
// ---------------------------------------------------------------------------

/// 一个摘录结构体，持有对某段文本的引用。
///
/// 因为结构体字段持有引用，编译器要求标注生命周期，
/// 以此来保证：Excerpt 实例本身不会比它引用的数据活得更久。
struct Excerpt<'a> {
    content: &'a str,
}

// 实现块：带生命周期参数
impl<'a> Excerpt<'a> {
    /// 获取内容 —— 省略规则自动生效。
    ///
    /// 根据省略规则 3（&self 规则），返回值生命周期来自 &self，
    /// 编译器推导为：fn get_content<'b>(&'b self) -> &'b str
    /// 注意到这里编译器使用了新的生命周期 'b，而不是结构体的 'a ——
    /// 因为方法签名的生命周期在调用时确定，不需要和结构体参数同名。
    fn get_content(&self) -> &str {
        self.content
    }

    /// 找出 self.content 和另一个字符串中较长的"单词"。
    ///
    /// 这里需要显式标注，因为有两个输入引用（&self 和 other），
    /// 而省略规则不能确定返回值应该关联到哪个输入的生命周期。
    fn longest_word(&self, other: &'a str) -> &'a str {
        if self.content.len() >= other.len() {
            self.content
        } else {
            other
        }
    }

    /// 展示结构体内容
    fn display(&self) {
        println!("  Excerpt 内容: \"{}\"", self.content);
    }
}

// ---------------------------------------------------------------------------
// 'static 生命周期 —— 它到底意味着什么？
// ---------------------------------------------------------------------------

/// 演示 'static 生命周期。
///
/// 关键概念：
///   - 字符串字面量（string literals）的类型是 &'static str
///     它们被直接嵌入二进制文件的数据段，程序运行期间始终有效
///   - const 常量隐含 'static 生命周期
///   - 'static 不意味着"变量永远存活"！它只意味着：
///     "这个引用在整个程序运行期间都是合法的"
///
/// 常见误区：
///   ```rust,ignore
///   let x: &'static str = "hello";  // OK: 字面量是 'static
///   let s = String::from("hello");
///   let y: &'static str = &s;       // 错误! s 不会活到 'static 那么久
///   ```
///
/// 如果你确实需要一个在整个程序期间都有效的字符串，
/// 应该使用 Box::leak 或 OnceLock / LazyLock 等机制 ——
/// 但这是非常罕见的场景，绝大多数代码不需要 'static。
fn demonstrate_static() {
    println!("=== 'static 生命周期 ===");
    println!();

    // 'static 字符串字面量
    let static_str: &'static str = "我是一个字符串字面量，类型是 &'static str";
    println!("静态字符串: {}", static_str);

    // const 也是 'static
    const GREETING: &str = "你好，世界！";
    // GREETING 的类型实际是 &'static str（编译器自动添加）
    println!("const 字符串: {}", GREETING);

    // 函数可以接受 &'static str 参数
    fn takes_static(s: &'static str) {
        println!("  收到 static 引用: {}", s);
    }
    takes_static(static_str);
    takes_static(GREETING);

    // 但不能把普通字符串的引用传进去 —— 因为它不是 'static
    // 下面这行如果取消注释会编译失败：
    // let local = String::from("local");
    // takes_static(&local);  // 错误！local 不是 'static

    println!();
    println!("关键理解:");
    println!("  - 'static 不意味着值永远不释放");
    println!("  - 它只保证引用在整个程序运行期间有效");
    println!("  - 字符串字面量天然是 'static — 它们嵌入在二进制中");
    println!("  - 绝大多数代码不需要 'static，省略规则已经足够了");
    println!();
}

// ---------------------------------------------------------------------------
// 不同参数使用不同生命周期
// ---------------------------------------------------------------------------

/// 演示不同参数可以使用不同的生命周期参数。
///
/// 这里 x 的生命周期是 'a，y 的生命周期是匿名（编译器给另一个）。
/// 返回值只和 x 关联 —— 编译器知道返回值不会比 x 活得更久，
/// 但和 y 的生命周期无关。
fn combine_and_return<'a>(x: &'a str, y: &str) -> &'a str {
    // y 只是用来打印，不参与返回值
    println!("  combine_and_return: y = \"{}\", 但只返回 x", y);
    x
}

// ---------------------------------------------------------------------------
// 泛型生命周期示例：找到切片中的最长元素
// ---------------------------------------------------------------------------

/// 在切片中找到最长的那个元素的引用。
///
/// 这个函数展示了生命周期 + 泛型一起使用的语法。
fn longest_in_slice<T>(slice: &[T]) -> Option<&T>
where
    T: AsRef<str>,
{
    slice.iter().max_by_key(|item| item.as_ref().len())
}

// ---------------------------------------------------------------------------
// main —— 统一演示
// ---------------------------------------------------------------------------

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║         第 16 章：生命周期（Lifetimes）                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // ---------- 1. longest 经典示例 ----------
    {
        println!("=== 1. longest 函数 —— 经典生命周期示例 ===");
        println!();

        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("longest(\"{}\", \"{}\") = \"{}\"", string1, string2, result);

        // 生命周期标注确保了安全性：下面的代码在编译时被阻止
        //
        // let result;
        // {
        //     let string3 = String::from("临时字符串");
        //     result = longest(string1.as_str(), string3.as_str());
        //     // string3 在这里被 drop
        // }
        // // result 在这里使用 —— 但 string3 已经不在了！
        // // 编译器会报告错误，因为 'a 必须同时覆盖 string1 和 string3，
        // // 而 string3 活得不够久
        //
        // 这就是生命周期标注的价值：在编译时捕获潜在的 use-after-free！

        println!();
        println!("关键是：生命周期标注并不改变 string1 或 string2 存活多久。");
        println!("它只是告诉编译器 'result 的有效期不会超过两个参数中较短的那个'。");
        println!();
    }

    // ---------- 2. 省略规则演示 ----------
    demonstrate_lifetime_elision();

    // ---------- 3. 结构体中的生命周期 ----------
    {
        println!("=== 3. 结构体中的生命周期 ===");
        println!();

        let novel = String::from("从前有座山。山里有座庙。庙里有个老和尚在讲故事。");
        let excerpt = Excerpt {
            content: &novel[..21], // "从前有座山。山里有座庙。"
        };

        excerpt.display();
        println!("  get_content(): \"{}\"", excerpt.get_content());

        let other_text = "庙里有个老和尚";
        let longer = excerpt.longest_word(other_text);
        println!("  longest_word(other): \"{}\"", longer);

        // 注意：excerpt 不能比 novel 活得更久
        // 编译器通过结构体的生命周期参数确保这一点

        println!();
        println!("结构体中的生命周期标注确保了：");
        println!("  Excerpt 实例永远不会比它引用的数据活得更久");
        println!();
    }

    // ---------- 4. 'static 演示 ----------
    demonstrate_static();

    // ---------- 5. 不同参数不同生命周期 ----------
    {
        println!("=== 5. 不同参数使用不同的生命周期 ===");
        println!();

        let long_lived = String::from("我会活很久");
        let short_lived = String::from("我可能活得短");

        let result = combine_and_return(&long_lived, &short_lived);
        println!("  返回结果: \"{}\"", result);
        // result 的生命周期只和 long_lived 绑定，short_lived 释放后 result 仍然有效

        println!();
        println!("注意：y 参数的生命周期独立于返回值，");
        println!("所以 short_lived 可以比 result 更早释放。");
        println!();
    }

    // ---------- 6. 泛型 + 生命周期 ----------
    {
        println!("=== 6. 泛型 + 生命周期 ===");
        println!();

        let words = vec![
            String::from("短"),
            String::from("比较长的字符串"),
            String::from("中等长度"),
        ];

        match longest_in_slice(&words) {
            Some(longest_word) => println!("切片中最长的元素: \"{}\"", longest_word),
            None => println!("切片为空"),
        }

        println!();
    }

    // ---------- 总结 ----------
    {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                        总结                              ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();
        println!("1. 生命周期标注不改变值存活的时间——只描述引用之间的关系");
        println!("2. 借用检查器一直在后台做生命周期检查，标注只是让它显式化");
        println!("3. 省略规则处理了日常 90% 的情况，你很少需要手动写标注");
        println!("4. 结构体持有引用时必须标注生命周期");
        println!("5. 'static 表示引用在整个程序运行期间有效，不等于'永不释放'");
        println!("6. 不同参数可以有不同的生命周期，返回值只关联需要的那个");
        println!();
        println!("Python 对照：");
        println!("  Python 没有生命周期的概念，GC 自动管理所有内存。");
        println!("  Rust 的生命周期是零成本抽象——没有运行时开销，");
        println!("  所有检查在编译期完成。这是 Rust 能在没有 GC 的情况下");
        println!("  保证内存安全的基石。");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_first_longer() {
        let result = longest("hello world", "hi");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_longest_second_longer() {
        let result = longest("hi", "hello world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_first_word_normal() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_first_word_single() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn test_excerpt_get_content() {
        let text = String::from("测试内容");
        let excerpt = Excerpt { content: &text };
        assert_eq!(excerpt.get_content(), "测试内容");
    }

    #[test]
    fn test_longest_in_slice() {
        let words = vec!["a", "abc", "ab"];
        let result = longest_in_slice(&words);
        assert_eq!(result, Some(&"abc"));
    }

    #[test]
    fn test_longest_in_slice_empty() {
        let words: Vec<&str> = vec![];
        let result = longest_in_slice(&words);
        assert_eq!(result, None);
    }
}
