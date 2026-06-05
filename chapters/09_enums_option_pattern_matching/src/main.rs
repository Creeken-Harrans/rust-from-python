// ============================================================================
// 枚举、Option 与模式匹配 —— 用户档案查询系统
// Rust 的类型安全核心：用类型系统在编译期消除空指针异常
// ============================================================================

use std::fmt;

// ============================================================================
// 1. 枚举定义：UserStatus
// ============================================================================
/// 用户状态枚举 —— 演示带数据的变体与纯标签变体
#[derive(Debug, Clone, PartialEq)]
enum UserStatus {
    /// 活跃用户 —— 纯标签变体，不携带数据
    Active,
    /// 未激活用户 —— 纯标签变体
    Inactive,
    /// 被封禁用户 —— 携带封禁原因的字符串
    Banned(String),
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatus::Active => write!(f, "活跃"),
            UserStatus::Inactive => write!(f, "未激活"),
            UserStatus::Banned(reason) => write!(f, "已封禁({reason})"),
        }
    }
}

// ============================================================================
// 2. 结构体：UserProfile
// ============================================================================
#[derive(Debug, Clone)]
struct UserProfile {
    username: String,
    /// 邮箱是可选的 —— 用 Option 代替 null
    email: Option<String>,
    /// 年龄是可选的 —— 用户可以不提供
    age: Option<u8>,
    status: UserStatus,
}

// ============================================================================
// 3. 模拟用户数据库（返回 Option）
// ============================================================================
fn find_user(username: &str) -> Option<UserProfile> {
    match username.to_lowercase().as_str() {
        "alice" => Some(UserProfile {
            username: "Alice".to_string(),
            email: Some("alice@rust-lang.org".to_string()),
            age: Some(30),
            status: UserStatus::Active,
        }),
        "bob" => Some(UserProfile {
            username: "Bob".to_string(),
            email: None, // Bob 没有提供邮箱
            age: None,   // Bob 也没有提供年龄
            status: UserStatus::Inactive,
        }),
        "charlie" => Some(UserProfile {
            username: "Charlie".to_string(),
            email: Some("charlie@example.com".to_string()),
            age: Some(25),
            status: UserStatus::Banned("违反社区准则".to_string()),
        }),
        "dave" => Some(UserProfile {
            username: "Dave".to_string(),
            email: Some("dave@rust-lang.org".to_string()),
            age: None,
            status: UserStatus::Active,
        }),
        // 通配符：匹配所有未列出的用户名
        _ => None,
    }
}

// ============================================================================
// 4. 从邮箱提取域名 —— 返回 Option<&str>
// ============================================================================
fn get_email_domain(profile: &UserProfile) -> Option<&str> {
    match &profile.email {
        None => None,
        Some(email) => email.find('@').map(|pos| &email[pos + 1..]),
    }
}

// ============================================================================
// 5. 描述用户 —— 穷尽性 match（必须覆盖所有变体）
// ============================================================================
fn describe_user(profile: &UserProfile) -> String {
    // 匹配 Option<u8>：Some(age) 或 None
    let age_str = match profile.age {
        Some(a) => format!("{a}岁"),
        None => "年龄未知".to_string(),
    };

    // 匹配 Option<String>：Some(email) 或 None
    let email_str = match &profile.email {
        Some(e) => e.clone(),
        None => "未提供".to_string(),
    };

    // 穷尽性匹配：编译器强制覆盖 UserStatus 的所有三个变体
    // 删除任何一个分支都会导致编译错误
    let status_str = match &profile.status {
        UserStatus::Active => "正常使用中".to_string(),
        UserStatus::Inactive => "账号未激活，请查收验证邮件".to_string(),
        UserStatus::Banned(reason) => format!("已被封禁，原因：{reason}"),
    };

    format!(
        "用户: {} | 邮箱: {} | 年龄: {} | 状态: {}",
        profile.username, email_str, age_str, status_str
    )
}

// ============================================================================
// 6. 演示：if let —— 只关心一种模式时的简洁写法
// ============================================================================
fn print_active_users(users: &[UserProfile]) {
    for user in users {
        // if let: 只匹配一个变体，忽略其他所有情况
        if let UserStatus::Active = &user.status {
            println!("  [活跃] {}", user.username);
        }
    }
}

// ============================================================================
// 7. 演示：let else —— 模式不匹配时提前返回
// ============================================================================
fn must_find_user(username: &str) -> String {
    // let else: 如果 find_user 返回 None，立即执行 else 分支
    let Some(profile) = find_user(username) else {
        return format!("let else: 用户 '{username}' 不存在，无法继续处理");
    };

    // 到达这里说明 profile 一定是 Some，可以直接使用
    let domain = get_email_domain(&profile)
        .map(|d| d.to_string())
        .unwrap_or_else(|| "无邮箱".to_string());

    format!("let else: 找到 {}，邮箱域名: {}", profile.username, domain)
}

// ============================================================================
// 8. 演示：while let —— 循环处理 Option/Result 直到耗尽
// ============================================================================
fn process_query_queue() {
    println!("\n========== while let 示例：处理查询队列 ==========");
    let mut queries = vec!["alice", "unknown", "bob", "charlie", "nobody", "dave"];

    // while let: 只要 pop() 返回 Some，就继续循环
    // pop() 返回 Option<&str>：Some(name) 或 None（队列空时）
    while let Some(username) = queries.pop() {
        print!("  查询 '{username}' ... ");

        // 嵌套使用 let else：找不到用户就跳过
        let Some(profile) = find_user(username) else {
            println!("未找到该用户");
            continue;
        };

        println!("找到: {}", describe_user(&profile));
    }
}

// ============================================================================
// 9. 演示：嵌套模式匹配
// ============================================================================
fn classify_user_detailed(profile: &UserProfile) -> String {
    // 同时对两个字段进行匹配——嵌套模式
    match (&profile.email, &profile.status) {
        (Some(email), UserStatus::Active) => {
            format!("{} 是活跃用户，邮箱: {}", profile.username, email)
        }
        (Some(_), UserStatus::Banned(reason)) => {
            format!("{} 已被封禁({reason})，但其邮箱仍然存在", profile.username)
        }
        (None, UserStatus::Banned(reason)) => {
            format!("{} 已被封禁({reason})，且无邮箱信息", profile.username)
        }
        (None, UserStatus::Inactive) => {
            format!("{} 未激活，也未提供邮箱", profile.username)
        }
        (Some(email), UserStatus::Inactive) => {
            format!("{} 未激活，但提供了邮箱: {}", profile.username, email)
        }
        // 通配符 _ 匹配剩余所有情况（本例中为 (None, Active)）
        (None, UserStatus::Active) => {
            format!("{} 是活跃用户，但未提供邮箱", profile.username)
        }
    }
}

// ============================================================================
// 10. 为什么 Rust 没有 null：对比演示
// ============================================================================
fn rust_has_no_null() {
    println!("\n========== 为什么 Rust 没有 null ==========");

    // 在 Python/Java/C 中，你可能会写：
    //   email = user.email
    //   domain = email.split('@')[1]  // 💥 如果 email 是 None/null，运行时崩溃！

    // 在 Rust 中，编译器强制你处理缺失的情况：
    let profile = UserProfile {
        username: "TestNull".to_string(),
        email: None, // 没有邮箱
        age: Some(20),
        status: UserStatus::Active,
    };

    // 尝试直接访问 profile.email 会得到 Option<String>，而不是 String
    // 你无法绕过 Option 直接使用它

    // 方法 1: match 显式处理
    let domain_v1 = match get_email_domain(&profile) {
        Some(d) => d.to_string(),
        None => "（无邮箱域名）".to_string(),
    };
    println!("  方法 1 (match): 域名 = {domain_v1}");

    // 方法 2: if let 简洁处理
    if let Some(domain) = get_email_domain(&profile) {
        println!("  方法 2 (if let): 域名 = {domain}");
    } else {
        println!("  方法 2 (if let): 未提供邮箱");
    }

    // 方法 3: unwrap_or 带默认值（不推荐对缺失做假设，但有时需要）
    let domain_v3 = get_email_domain(&profile).unwrap_or("未知域名");
    println!("  方法 3 (unwrap_or): 域名 = {domain_v3}");

    // 如果强行 unwrap() —— 编译通过但运行时 panic
    // let crash = get_email_domain(&profile).unwrap(); // 💥 panic!
    println!("  ⚠ 如果写了 .unwrap() 在 None 上，会 panic，但这是显式的崩溃，不是空指针异常。");
    println!(
        "  编译器在编译期就确保了所有 Option 都被处理——只是你选择用 unwrap 说 '我赌它一定有值'。"
    );
}

// ============================================================================
// 11. 演示：@ 绑定 —— 在模式匹配中同时绑定整个值和子部分
// ============================================================================
fn demo_at_binding() {
    println!("\n========== @ 绑定示例 ==========");

    let profiles = vec![find_user("alice"), find_user("charlie"), find_user("bob")];

    for maybe_profile in &profiles {
        match maybe_profile {
            // @ 绑定：将整个匹配的值绑定到 p，同时匹配内部字段
            Some(p @ UserProfile { email: Some(e), .. }) => {
                println!("  @绑定: {} 有邮箱 {e}（完整档案: {:?}）", p.username, p);
            }
            Some(p) => {
                println!("  @绑定: {} 没有邮箱", p.username);
            }
            None => {
                println!("  @绑定: 用户不存在");
            }
        }
    }
}

// ============================================================================
// 12. 演示：matches! 宏 —— 返回 bool 的模式匹配
// ============================================================================
fn demo_matches_macro() {
    println!("\n========== matches! 宏示例 ==========");

    let alice = find_user("alice");
    let charlie = find_user("charlie");

    // matches! 宏：检查值是否匹配某个模式，返回 bool
    let alice_is_active = matches!(
        &alice,
        Some(UserProfile {
            status: UserStatus::Active,
            ..
        })
    );
    let charlie_is_banned = matches!(
        &charlie,
        Some(UserProfile {
            status: UserStatus::Banned(_),
            ..
        })
    );

    println!("  Alice 是活跃用户？ {alice_is_active}");
    println!("  Charlie 被封禁了？ {charlie_is_banned}");

    // 用在条件判断中
    if matches!(&alice, Some(UserProfile { age: Some(age), .. }) if *age >= 18) {
        println!("  Alice 是成年人（用 matches! + match guard 检查）");
    }
}

// ============================================================================
// main —— 总演示入口
// ============================================================================
fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   第 9 章：枚举、Option 与模式匹配 —— 用户档案查询系统     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // ----- 基础匹配 -----
    println!("\n========== Section 1: 基础查询与描述 ==========");
    let usernames = ["alice", "bob", "charlie", "unknown"];

    for name in usernames {
        print!("查询 '{name}' ... ");
        match find_user(name) {
            Some(profile) => println!("{}", describe_user(&profile)),
            None => println!("未找到用户 '{name}'"),
        }
    }

    // ----- if let 演示 -----
    println!("\n========== Section 2: if let 筛选活跃用户 ==========");
    let all_profiles: Vec<UserProfile> = ["alice", "bob", "charlie", "dave"]
        .iter()
        .filter_map(|&n| find_user(n))
        .collect();
    print_active_users(&all_profiles);

    // ----- let else 演示 -----
    println!("\n========== Section 3: let else 提前返回 ==========");
    println!("  {}", must_find_user("alice"));
    println!("  {}", must_find_user("ghost"));

    // ----- while let 演示 -----
    process_query_queue();

    // ----- 嵌套模式匹配 -----
    println!("\n========== Section 5: 嵌套模式匹配 ==========");
    for profile in &all_profiles {
        println!("  {}", classify_user_detailed(profile));
    }

    // ----- 为什么没有 null -----
    rust_has_no_null();

    // ----- @ 绑定 -----
    demo_at_binding();

    // ----- matches! 宏 -----
    demo_matches_macro();

    // ----- Option 组合子（额外收获）-----
    println!("\n========== Section 8: Option 组合子 ==========");
    println!("  除了模式匹配，Option 还提供了丰富的组合子方法：");

    let alice = find_user("alice");
    // map: 在 Some 时转换值
    let username_upper = alice.as_ref().map(|p| p.username.to_uppercase());
    println!("  map: {username_upper:?}");

    // and_then: 链式调用，任一环节为 None 则整体为 None
    let domain = alice.as_ref().and_then(|p| get_email_domain(p));
    println!("  and_then: {domain:?}");

    // filter: 按条件过滤
    let is_adult = alice
        .as_ref()
        .and_then(|p| p.age.filter(|&a| a >= 18).map(|_| &p.username));
    println!("  filter: {:?} 是成年人", is_adult);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║   Rust 的模式匹配 + Option<T> 共同在编译期消灭了空指针异常  ║");
    println!("║   \"Null references: the billion-dollar mistake\" — Tony Hoare  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
