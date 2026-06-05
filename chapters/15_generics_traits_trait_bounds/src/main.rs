#![allow(rustdoc::invalid_html_tags)]
// ============================================================
// Chapter 15: 泛型 (Generics) 与 特征 (Traits)
// Rust 的多态抽象 —— 零成本抽象的核心理念
// ============================================================

use std::fmt::Debug;

// -----------------------------------------------------------
// 1. 定义特征 (Traits)
// -----------------------------------------------------------

/// `Summary` 特征：任何可被"摘要"的类型都需要实现这个特征。
/// 它包含一个 `summarize` 方法和一个默认实现。
pub trait Summary {
    /// 返回该类型的摘要字符串。
    /// 默认实现：返回 "(Read more...)" 作为占位。
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    /// 另一个方法，返回作者信息，无默认实现 —— 实现者必须提供。
    fn author(&self) -> String;
}

/// `DisplayInfo` 特征：提供详细的展示信息。
pub trait DisplayInfo {
    fn info(&self) -> String;
}

// -----------------------------------------------------------
// 2. 结构体定义
// -----------------------------------------------------------

/// 新闻文章结构体
#[derive(Debug, Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub author: String,
}

/// 推文结构体
#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet_count: u64,
}

/// 博客帖子结构体（用于演示默认 Summary 实现）
#[derive(Debug, Clone)]
pub struct BlogPost {
    pub title: String,
    pub body: String,
    pub author_name: String,
}

// -----------------------------------------------------------
// 3. 为具体类型实现特征
// -----------------------------------------------------------

// NewsArticle 的 Summary——自定义实现，生成较长摘要
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // 安全截取前 60 个字符（按 Unicode 字符边界，而非字节）
        let preview: String = self.content.chars().take(60).collect();
        format!(
            "📰 [新闻] {} —— 作者: {} | 摘要: {}...",
            self.headline, self.author, preview
        )
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

// DisplayInfo for NewsArticle — 详细展示信息
impl DisplayInfo for NewsArticle {
    fn info(&self) -> String {
        format!(
            "头条: {}\n内容长度: {} 字符\n作者: {}",
            self.headline,
            self.content.len(),
            self.author
        )
    }
}

// Tweet 的 Summary——不同的实现，简短风格
impl Summary for Tweet {
    fn summarize(&self) -> String {
        // 安全截取前 40 个字符（按 Unicode 字符边界，而非字节）
        let preview: String = self.content.chars().take(40).collect();
        format!(
            "🐦 @{}: {} — 转发 {} 次",
            self.username, preview, self.retweet_count
        )
    }

    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

// DisplayInfo for Tweet — 推文特有的展示格式
impl DisplayInfo for Tweet {
    fn info(&self) -> String {
        format!(
            "用户: @{}\n内容: {}\n转发数: {}",
            self.username, self.content, self.retweet_count
        )
    }
}

// BlogPost 使用 Summary 的默认实现，但必须提供 author()
impl Summary for BlogPost {
    fn author(&self) -> String {
        self.author_name.clone()
    }
    // summarize() 使用默认实现 → "(Read more...)"
}

// DisplayInfo for BlogPost — 简洁的展示格式
impl DisplayInfo for BlogPost {
    fn info(&self) -> String {
        format!(
            "博客: {}\n作者: {}\n正文长度: {} 字符",
            self.title,
            self.author_name,
            self.body.len()
        )
    }
}

// -----------------------------------------------------------
// 4. 全覆盖实现 (Blanket Implementation) —— 概念展示
// -----------------------------------------------------------
// ⚠️ 注意：由于我们已经在上面为 NewsArticle 和 Tweet 分别手动实现了
// DisplayInfo，如果再添加全覆盖实现，会导致冲突（Rust 不允许重叠的实现）。
// 因此这里以注释形式展示全覆盖实现的概念：
//
// ```rust
// /// 全覆盖实现：为所有实现了 Summary + Debug 的类型自动提供 DisplayInfo
// impl<T: Summary + Debug> DisplayInfo for T {
//     fn info(&self) -> String {
//         format!(
//             "[全覆盖 DisplayInfo]\n摘要: {}\n作者: {}\n调试输出: {:?}",
//             self.summarize(),
//             self.author(),
//             self
//         )
//     }
// }
// ```
//
// 这样一来，任何实现了 Summary + Debug 的类型都会自动获得 DisplayInfo 实现，
// 无需手动为每种类型单独 impl。这在实际 Rust 标准库中非常常见，例如：
//
// ```rust
// // 标准库中的真实代码：
// impl<T: fmt::Display> ToString for T {
//     fn to_string(&self) -> String { ... }
// }
// ```
//
// 全覆盖实现也必须遵守孤儿规则：特征和类型中至少有一个在本地 crate。
// 例如：不能 `impl<T> Display for T`，因为 Display 和 T 都不是本地的。
// -----------------------------------------------------------

// -----------------------------------------------------------
// 5. 泛型结构体 (Generic Struct)
// -----------------------------------------------------------

/// 泛型报告结构体：可以包装任意类型的数据
#[derive(Debug)]
pub struct Report<T> {
    /// 报告所包含的数据
    pub data: T,
    /// 报告生成的时间戳
    pub timestamp: String,
}

/// 为 Report<T> 实现方法，其中 T 必须实现 Summary
impl<T: Summary> Report<T> {
    /// 创建一个新的报告
    pub fn new(data: T) -> Self {
        Report {
            data,
            timestamp: String::from("2026-06-05 10:00:00"),
        }
    }

    /// 生成报告的文本内容
    pub fn generate(&self) -> String {
        format!(
            "═══════════════════════════════\n\
             报告时间: {}\n\
             {}\n\
             ═══════════════════════════════",
            self.timestamp,
            self.data.summarize()
        )
    }

    /// 更新报告的时间戳
    pub fn update_timestamp(&mut self, ts: &str) {
        self.timestamp = ts.to_string();
    }
}

/// 为 Report<Tweet> 提供特化实现 —— 只有 T = Tweet 时可用
impl Report<Tweet> {
    /// 推文报告特有方法：显示转发热度
    pub fn engagement_score(&self) -> String {
        if self.data.retweet_count > 100 {
            format!(
                "🔥 高热度推文 ({} 转发) —— 来自 @{}",
                self.data.retweet_count, self.data.username
            )
        } else {
            format!(
                "📊 普通推文 ({} 转发) —— 来自 @{}",
                self.data.retweet_count, self.data.username
            )
        }
    }

    /// 推文报告特有方法：显示简洁卡片
    pub fn card(&self) -> String {
        format!(
            "┌──────────────────────────────┐\n\
             │ @{:<27}│\n\
             │ {:<30}│\n\
             │ 🔄 {:<27}│\n\
             └──────────────────────────────┘",
            self.data.username,
            // 截断长内容
            &self.data.content.chars().take(28).collect::<String>(),
            self.data.retweet_count
        )
    }
}

// -----------------------------------------------------------
// 6. 泛型函数与特征约束 (Trait Bounds)
// -----------------------------------------------------------

/// `generate_report` — 使用 `<T: Summary>` 语法做特征约束
/// 这是静态分派 (Static Dispatch)：编译器在编译时为每个具体类型
/// 生成独立的函数副本。这个过程称为**单态化 (Monomorphization)**。
/// 优点：零运行时开销（零成本抽象）。
/// 代价：编译时间稍长，生成的二进制体积稍大。
pub fn generate_report<T: Summary>(item: &T) -> String {
    format!(
        "╔══════════ 报告 ══════════╗\n\
         ║ {:<24}║\n\
         ╚════════════════════════╝",
        item.summarize()
    )
}

/// `generate_detailed_report` — 使用 `where` 子句做多特征约束
/// `where` 子句在约束很多时更可读。
/// 这里要求 T 同时实现 Summary、DisplayInfo 和 Debug。
pub fn generate_detailed_report<T>(item: &T) -> String
where
    T: Summary + DisplayInfo + Debug,
{
    format!(
        "╔══════════ 详细报告 ══════════╗\n\
         ║ 摘要: {:<20}║\n\
         ║ 信息: {:<20}║\n\
         ║ 调试: {:?}{}\n\
         ╚════════════════════════════╝",
        item.summarize(),
        item.info(),
        item,
        " ".repeat(20_usize.saturating_sub(format!("{:?}", item).len()))
    )
}

/// `notify` — 使用 `impl Trait` 语法作为参数类型
/// 这是 `<T: Summary>` 的语法糖，语义完全相同。
/// 适合参数较少的场景。
pub fn notify(item: &impl Summary) {
    println!("🔔 通知: {}", item.summarize());
}

/// `create_and_print` — 泛型函数接受一个闭包工厂
/// 结合了泛型参数 T 和 `impl Fn()` 特征。
/// 工厂闭包负责创建 T 类型的实例。
pub fn create_and_print<T: Summary>(factory: impl Fn() -> T) -> String {
    let item = factory();
    let summary = item.summarize();
    format!("📦 工厂产物: {summary}")
}

/// `return_summarizable` — impl Trait 在返回值位置
/// 注意：用 `impl Trait` 返回时，函数体内所有返回路径
/// 必须返回**同一种具体类型**（不能 if 返回 Tweet, else 返回 NewsArticle）。
/// 这与 trait object (`Box<dyn Trait>`) 不同，后者使用动态分派。
pub fn return_summarizable(kind: &str) -> impl Summary {
    // 所有分支必须返回相同类型 —— 这里都返回 Tweet
    match kind {
        "tech" => Tweet {
            username: String::from("rustlang"),
            content: String::from("Announcing Rust 2026 Edition! 🚀"),
            retweet_count: 5200,
        },
        "fun" => Tweet {
            username: String::from("ferris"),
            content: String::from("Happy coding! 🦀"),
            retweet_count: 42,
        },
        _ => Tweet {
            username: String::from("rust_daily"),
            content: String::from("Today's tip: use clippy!"),
            retweet_count: 88,
        },
    }
}

// -----------------------------------------------------------
// 7. 辅助：打印分隔线
// -----------------------------------------------------------
fn section(title: &str) {
    println!("\n{}", "─".repeat(60));
    println!("  {title}");
    println!("{}", "─".repeat(60));
}

// -----------------------------------------------------------
// 8. main() — 演示所有概念
// -----------------------------------------------------------
fn main() {
    println!("🦀 第 15 章：泛型与特征 (Generics & Traits)");
    println!("    Rust 的多态抽象 — 零成本抽象实践\n");

    // ---- 8a. 创建数据 ----
    section("创建新闻文章");

    let article = NewsArticle {
        headline: String::from("Rust 2026 Edition 正式发布"),
        content: String::from(
            "Rust 团队宣布 2026 Edition 正式稳定。新版本包含更简洁的 \
             impl Trait 语法、改进的异步编程支持、以及更智能的借用检查器。\
             社区反响热烈，预计将推动 Rust 在企业级应用中的进一步普及。",
        ),
        author: String::from("Rust 编辑团队"),
    };
    println!("已创建: {:?}", article);

    section("创建推文");

    let tweet = Tweet {
        username: String::from("ferris"),
        content: String::from("刚刚尝试了 Rust 2026 Edition 的新特性，简直太棒了！"),
        retweet_count: 1523,
    };
    println!("已创建: {:?}", tweet);

    section("创建博客帖子");
    let blog = BlogPost {
        title: String::from("学习 Rust 的心得"),
        body: String::from("Rust 的所有权系统一开始很难，但一旦理解就豁然开朗..."),
        author_name: String::from("Creeken"),
    };
    println!("已创建: {:?}", blog);

    // ---- 8b. 特征方法调用 ----
    section("Summary 特征");

    println!("NewsArticle.summarize() → {}", article.summarize());
    println!("Tweet.summarize()      → {}", tweet.summarize());
    // BlogPost 使用默认实现
    println!("BlogPost.summarize()   → {}", blog.summarize());
    println!("NewsArticle.author()   → {}", article.author());
    println!("Tweet.author()         → {}", tweet.author());
    println!("BlogPost.author()      → {}", blog.author());

    section("DisplayInfo 特征 (全覆盖实现)");

    // 因为全覆盖实现: impl<T: Summary + Debug> DisplayInfo for T
    // NewsArticle / Tweet / BlogPost 都自动获得了 info()
    println!("NewsArticle.info():\n{}", article.info());
    println!("Tweet.info():\n{}", tweet.info());
    println!("BlogPost.info():\n{}", blog.info());

    // ---- 8c. 泛型函数 ----
    section("泛型函数 generate_report<T: Summary>");

    println!("{}", generate_report(&article));
    println!("{}", generate_report(&tweet));
    println!("{}", generate_report(&blog));

    section("泛型函数 generate_detailed_report<T: Summary + DisplayInfo + Debug>");

    // generate_detailed_report 要求 T: Summary + DisplayInfo + Debug
    // DisplayInfo 通过全覆盖实现已经提供，所以三个类型都满足
    println!("{}", generate_detailed_report(&article));
    println!("{}", generate_detailed_report(&tweet));
    println!("{}", generate_detailed_report(&blog));

    section("notify(item: &impl Summary)");

    notify(&article);
    notify(&tweet);
    notify(&blog);

    section("create_and_print — 泛型 + 闭包工厂");

    let output = create_and_print(|| NewsArticle {
        headline: String::from("闭包工厂新闻"),
        content: String::from("这条新闻是通过闭包工厂创建的。"),
        author: String::from("工厂作者"),
    });
    println!("{output}");

    let output = create_and_print(|| Tweet {
        username: String::from("closure_fan"),
        content: String::from("闭包 + 泛型 = ❤️"),
        retweet_count: 7,
    });
    println!("{output}");

    section("return_summarizable — impl Trait 在返回值位置");

    let tech_tweet = return_summarizable("tech");
    println!("tech: {}", tech_tweet.summarize());

    let fun_tweet = return_summarizable("fun");
    println!("fun:  {}", fun_tweet.summarize());

    let other_tweet = return_summarizable("other");
    println!("other: {}", other_tweet.summarize());

    // ---- 8d. 泛型结构体 Report<T> ----
    section("泛型结构体 Report<T>");

    let news_report = Report::new(article.clone());
    println!("{}", news_report.generate());

    let tweet_report: Report<Tweet> = Report::new(tweet.clone());
    println!("{}", tweet_report.generate());

    let blog_report = Report::new(blog.clone());
    println!("{}", blog_report.generate());

    // ---- 8e. 特化实现 Report<Tweet> ----
    section("特化实现: impl Report<Tweet>");

    println!("Engagement: {}", tweet_report.engagement_score());
    println!("Card:\n{}", tweet_report.card());

    // ---- 8f. 多特征约束 ----
    section("多特征约束 with +");

    // 编写一个内联函数展示多特征约束
    fn analyze<T: Summary + DisplayInfo + Clone>(item: &T) -> String {
        let cloned = item.clone();
        format!(
            "分析结果:\n  摘要: {}\n  信息: {}\n  克隆后摘要: {}",
            item.summarize(),
            item.info(),
            cloned.summarize()
        )
    }

    println!("{}", analyze(&article));

    // ---- 8g. 理解单态化 ----
    section("单态化 (Monomorphization) 与零成本抽象");

    println!(
        "┌─ 单态化 (Monomorphization) ─────────────────────────────────┐\n\
         │                                                          │\n\
         │  当你在 Rust 中编写泛型代码时，编译器在编译期（而非运行   │\n\
         │  期）生成泛型函数/结构体的具体版本。这个过程称为\"单态化\"。│\n\
         │                                                          │\n\
         │  例如，调用 generate_report(&article) 时：                │\n\
         │                                                          │\n\
         │    编译器生成:                                            │\n\
         │    fn generate_report_NewsArticle(item: &NewsArticle)     │\n\
         │    fn generate_report_Tweet(item: &Tweet)                 │\n\
         │    fn generate_report_BlogPost(item: &BlogPost)           │\n\
         │                                                          │\n\
         │  零成本抽象 (Zero-Cost Abstraction)：                     │\n\
         │  ─────────────────────────────────────                   │\n\
         │  • 泛型抽象在运行时没有任何额外开销                        │\n\
         │  • 性能等价于手写具体类型的代码                            │\n\
         │  • 不需要虚函数表 (vtable) 来分发方法调用                  │\n\
         │  • 这就是\"静态分派 (Static Dispatch)\"                  │\n\
         │                                                          │\n\
         │  对比：                                                   │\n\
         │  • 静态分派 (Static Dispatch)：编译期确定调用目标         │\n\
         │  • 动态分派 (Dynamic Dispatch)：运行期通过 vtable 查找    │\n\
         │    (使用 dyn Trait / trait object 时)                     │\n\
         │                                                          │\n\
         │  代价：                                                   │\n\
         │  • 编译时间稍长（编译器要做更多工作）                      │\n\
         │  • 二进制体积稍大（每种具体类型都有独立副本）              │\n\
         │  • 但这些代价换来的是运行时的最优性能                      │\n\
         │                                                          │\n\
         └──────────────────────────────────────────────────────────┘"
    );

    section("总结");

    println!(
        "✅ 本程序展示了以下 Rust 泛型与特征核心概念：\n\
         \n\
         1. 定义特征 (trait): Summary, DisplayInfo\n\
         2. 为结构体实现特征: impl Summary for NewsArticle/Tweet/BlogPost\n\
         3. 默认特征方法: Summary::summarize() 有默认实现\n\
         4. 全覆盖实现 (Blanket Impl): impl<T: Summary+Debug> DisplayInfo for T\n\
         5. 泛型函数 + 特征约束: fn generate_report<T: Summary>(&T)\n\
         6. where 子句: fn generate_detailed_report<T>(&T) where T: Summary + DisplayInfo\n\
         7. impl Trait 语法: fn notify(&impl Summary)\n\
         8. impl Trait 在返回值位置: fn return_summarizable() -> impl Summary\n\
         9. 泛型结构体: struct Report<T>\n\
         10. 泛型方法: impl<T: Summary> Report<T>\n\
         11. 特化实现: impl Report<Tweet>\n\
         12. 多特征约束: T: Summary + DisplayInfo + Clone\n\
         13. 闭包 + 泛型: fn create_and_print<T>(impl Fn() -> T)\n\
         \n\
         🎯 核心理念：零成本抽象 (Zero-Cost Abstraction)\n\
            你不需要为抽象付费——编译器在编译期完成所有工作，\n\
            运行时没有任何额外开销。"
    );
}
