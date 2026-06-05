# 第 9 章：枚举、Option 与模式匹配

> Rust 的类型安全核心 —— 用类型系统在编译期消灭空指针异常（NullPointerException）

---

## 目录

1. [问题引入：没有 null 的语言如何表达"可能没有值"](#1-问题引入没有-null-的语言如何表达可能没有值)
2. [Python 视角：你熟悉的 None 和 match](#2-python-视角你熟悉的-none-和-match)
3. [Rust 的设计：Enum + Option + 模式匹配](#3-rust-的设计enum--option--模式匹配)
4. [核心规则](#4-核心规则)
5. [完整代码示例](#5-完整代码示例)
6. [常见错误与陷阱](#6-常见错误与陷阱)
7. [编译器怎么说](#7-编译器怎么说)
8. [如何修复](#8-如何修复)
9. [适用边界与最佳实践](#9-适用边界与最佳实践)
10. [章节总结](#10-章节总结)

---

## 1. 问题引入：没有 null 的语言如何表达"可能没有值"

在编程中，我们经常遇到"可能有、也可能没有"的场景：

- 用户可能填写了邮箱，也可能没填
- 查询一个用户名，结果可能找到，也可能找不到
- 从缓存中取数据，可能命中，也可能未命中

在大多数语言中，这个问题用 `null`（Java/C#）、`None`（Python）、`nil`（Ruby/Go）或 `NULL`（C/C++）来解决。看起来很简单，但它带来了一个巨大的问题：

```
// 这段代码在 Python 中看起来没问题：
email = user.email              # email 可能是 None
domain = email.split('@')[1]    # 💥 如果 email 是 None，AttributeError!
```

**Tony Hoare**（null 引用的发明者）在 2009 年公开道歉：

> "I call it my billion-dollar mistake. It was the invention of the null reference in 1965."
>
> —— 我称它为我的十亿美元错误。那就是 1965 年我发明的空引用。

为什么叫"十亿美元错误"？因为几十年来，空指针异常导致的系统崩溃、安全漏洞、调试时间，累计造成的损失远超十亿美元。

**Rust 的回答是**：直接把 `null` 从语言中删除，用 `Option<T>` 枚举来代替。任何可能缺失的值，其类型本身就是 `Option<T>`，编译器**强制**你处理两种情况——有值（`Some(T)`）和没有值（`None`）。你无法像在 Python 中那样"忘了检查 None"——编译器不让你编译通过。

---

## 2. Python 视角：你熟悉的 None 和 match

### 2.1 Python 的 None

在 Python 中，`None` 是一个单例对象，任何变量都可以是 `None`：

```python
def get_email_domain(user: dict) -> str | None:
    email = user.get("email")
    if email is None:
        return None
    return email.split("@")[1]

# 问题：调用者可能忘记检查返回值
user = {"username": "Alice"}  # 没有 email 字段
domain = get_email_domain(user)
print(domain.upper())  # 💥 AttributeError: 'NoneType' object has no attribute 'upper'
```

类型提示 `str | None` 是后来加上的，但它是**可选的**。类型检查工具（如 mypy）可以在静态分析时发现问题，但这不保证运行时安全。

### 2.2 Python 3.10+ 的 match 语句

Python 3.10 引入了 `match` 语句（结构模式匹配）：

```python
def describe_user(user: dict) -> str:
    match user:
        case {"status": "active", "email": str(e), "age": int(a)}:
            return f"{user['name']} 活跃，邮箱 {e}，{a} 岁"
        case {"status": "banned", "reason": str(r)}:
            return f"{user['name']} 被封禁：{r}"
        case _:
            return f"{user['name']} 状态未知"
```

Python 的 `match` 很强大，但它不做**穷尽性检查**（exhaustiveness checking）。如果你漏了一个 `case`，代码照样运行，只是走到 `case _` 通配分支。此外，Python 的 `match` 在运行时做类型检查，不在编译时。

### 2.3 Python 的 try/except 与 Rust 的 Option

Python 中常这样处理"可能找不到"：

```python
try:
    user = user_db["charlie"]  # 如果 key 不存在会抛 KeyError
except KeyError:
    user = None
```

而 Rust 中，查找操作直接返回 `Option<UserProfile>`：

```rust
let user: Option<UserProfile> = find_user("charlie");
// 编译器知道你有一个 Option，你必须处理它
```

**核心差异**：

| 方面 | Python | Rust |
|------|--------|------|
| 缺失值的类型 | 任何类型都可以是 `None`（`Optional[T]` 只是类型提示） | 必须是 `Option<T>`，类型系统强制 |
| 检查时机 | 运行时（除非用了 mypy） | 编译期 |
| 穷尽性检查 | 无（`match` 的 `case _` 永远匹配） | 强制（漏一个分支就编译不过） |
| 忘记处理的后果 | 运行时 `AttributeError` | 编译错误 |

---

## 3. Rust 的设计：Enum + Option + 模式匹配

### 3.1 什么是 Rust 枚举（Enum）

在 C 语言中，枚举只是给整数起名字：

```c
enum Color { Red, Green, Blue };  // Red=0, Green=1, Blue=2
```

Rust 的枚举完全不同。每个**变体（Variant）**可以：

1. 不携带数据（类似 C 枚举）
2. 携带任意类型的数据（元组风格）
3. 携带命名字段（结构体风格）

```rust
enum UserStatus {
    Active,                    // 纯标签
    Inactive,                  // 纯标签
    Banned(String),            // 携带一个 String（封禁原因）
}
```

每个变体都是一个**独立的类型构造器**。`UserStatus::Banned` 是一个函数 `Fn(String) -> UserStatus`。

### 3.2 Option\<T\>：类型安全的"可能没有"

`Option<T>` 是 Rust 标准库中定义的一个普通枚举：

```rust
pub enum Option<T> {
    None,       // 没有值
    Some(T),    // 有值，包裹在 Some 中
}
```

它如此重要，以至于 `Option`、`Some`、`None` 都被自动导入到每个 Rust 程序的命名空间中（prelude），你不需要 `use std::option::Option`。

**关键理解**：`Option<String>` 和 `String` 是**两个完全不同的类型**。你不能把 `Option<String>` 当作 `String` 使用。这就是类型安全的核心——类型系统本身就告诉你"这个值可能不存在"。

```rust
let email: Option<String> = Some("alice@example.com".to_string());
let name: String = "Alice".to_string();

// 编译错误：Option<String> 和 String 是不同的类型
// let result = email.to_uppercase();  // ❌ Option<String> 没有 to_uppercase() 方法

// 正确做法：先取出里面的值
match email {
    Some(e) => println!("{}", e.to_uppercase()),
    None => println!("没有邮箱"),
}
```

### 3.3 模式匹配（Pattern Matching）

模式匹配不只是"更花哨的 if-else"。它是一种**解构 + 判断**的组合操作：

```rust
match value {
    Pattern1 => expression1,
    Pattern2 => expression2,
    // 编译器检查：是否覆盖了所有可能的情况？
}
```

模式可以：
- 匹配字面值：`1`, `"hello"`
- 匹配变量（绑定）：`x`, `name`
- 解构元组：`(x, y)`
- 解构结构体：`UserProfile { username, .. }`
- 解构枚举：`Some(x)`, `None`
- 使用通配符：`_`（匹配任意值但不绑定）
- 使用守卫条件：`x if x > 5`
- 使用 `@` 绑定：`p @ UserProfile { .. }`
- 使用 `|` 或模式：`Active | Inactive`
- 嵌套组合以上所有

---

## 4. 核心规则

### 规则 1：match 必须穷尽（Exhaustiveness）

编译器**强制** `match` 覆盖所有可能的模式。这对于枚举特别重要——如果你新增了一个变体，所有 `match` 的地方编译器都会提醒你。

```rust
match status {
    UserStatus::Active => "活跃",
    UserStatus::Inactive => "未激活",
    // ❌ 编译错误：遗漏了 Banned 变体
}
```

### 规则 2：\Option\<T\> 不是 T

你不能绕过 `Option` 直接使用内部值。必须通过模式匹配或组合子方法（`.unwrap()`、`.map()`、`.and_then()` 等）来获取。

### 规则 3：if let 是 match 的语法糖

```rust
// 这两个等价：
if let Some(x) = optional_value {
    println!("{x}");
}

match optional_value {
    Some(x) => println!("{x}"),
    _ => (),  // 什么都不做
}
```

### 规则 4：let else 是"不匹配就离开"的控制流

```rust
let Some(value) = optional_value else {
    return;  // 必须发散：return / break / continue / panic!
};
// 此后的代码中 value 是确定的 T 类型
```

`else` 块**必须**是发散表达式（diverge），即不会正常执行到后续代码。常见的发散表达式有 `return`、`break`、`continue`、`panic!()`、`unreachable!()`。

### 规则 5：while let 持续匹配直到失败

```rust
while let Some(item) = iterator.next() {
    // 只要 next() 返回 Some，就继续循环
}
// 当 next() 返回 None 时循环结束
```

---

## 5. 完整代码示例

### 5.1 基础枚举与 Option

```rust
#[derive(Debug)]
enum UserStatus {
    Active,
    Inactive,
    Banned(String),
}

struct UserProfile {
    username: String,
    email: Option<String>,
    age: Option<u8>,
    status: UserStatus,
}

fn find_user(username: &str) -> Option<UserProfile> {
    match username {
        "alice" => Some(UserProfile {
            username: "Alice".into(),
            email: Some("alice@example.com".into()),
            age: Some(30),
            status: UserStatus::Active,
        }),
        _ => None,
    }
}
```

### 5.2 穷尽性匹配

```rust
fn describe_status(status: &UserStatus) -> &str {
    match status {
        UserStatus::Active => "活跃",
        UserStatus::Inactive => "未激活",
        UserStatus::Banned(reason) => {
            println!("封禁原因: {reason}");
            "已封禁"
        }
    }
    // 编译器确认：UserStatus 只有 3 个变体，全部覆盖
}
```

### 5.3 嵌套模式匹配

```rust
fn classify(user: &UserProfile) -> String {
    match (&user.email, &user.status) {
        (Some(e), UserStatus::Active) => format!("活跃用户，邮箱: {e}"),
        (Some(_), UserStatus::Banned(r)) => format!("被封禁({r})，邮箱仍存在"),
        (None, UserStatus::Banned(_)) => "被封禁，无邮箱".into(),
        (None, _) => "无邮箱信息".into(),
    }
    // 注意：(Some(e), Active) 和 (None, _) —— 编译器检查所有组合
}
```

注意：这里用了 `&user.email`（引用）而不是 `user.email`（移动所有权），这样 `user` 在匹配后仍然可用。

### 5.4 if let —— 简洁的单模式匹配

```rust
// 只关心封禁用户
if let UserStatus::Banned(reason) = &user.status {
    println!("注意：该用户被封禁，原因：{reason}");
}
// 其他状态自动忽略
```

### 5.5 let else —— 提前返回

```rust
fn get_domain(username: &str) -> Result<String, String> {
    let Some(profile) = find_user(username) else {
        return Err(format!("用户 {username} 不存在"));
    };

    let Some(email) = profile.email else {
        return Err(format!("用户 {username} 没有邮箱"));
    };

    let Some(at_pos) = email.find('@') else {
        return Err(format!("邮箱格式错误: {email}"));
    };

    Ok(email[at_pos + 1..].to_string())
}
```

`let else` 避免了"箭头式代码"——不需要逐层嵌套 `if let`。

### 5.6 while let —— 循环消费

```rust
let mut stack = vec!["alice", "bob", "charlie"];
while let Some(name) = stack.pop() {
    // 逐个处理，直到栈空
    match find_user(name) {
        Some(p) => println!("找到: {}", p.username),
        None => println!("{name}: 无此用户"),
    }
}
```

### 5.7 @ 绑定

```rust
match find_user("alice") {
    Some(p @ UserProfile { email: Some(e), .. }) => {
        // p: 整个 UserProfile 的引用
        // e: email 字段的内容
        println!("用户 {} 的邮箱是 {e}", p.username);
    }
    Some(_) => println!("无邮箱"),
    None => println!("无此用户"),
}
```

### 5.8 match 守卫（Guard）

```rust
match profile.age {
    Some(age) if age < 18 => println!("未成年用户: {age}岁"),
    Some(age) if age >= 60 => println!("老年用户: {age}岁"),
    Some(age) => println!("成年用户: {age}岁"),
    None => println!("年龄未知"),
}
```

### 5.9 matches! 宏 —— 返回 bool 的模式匹配

```rust
let is_active = matches!(&profile.status, UserStatus::Active);
let has_email = matches!(&profile.email, Some(_));

if matches!(&profile, UserProfile { age: Some(a), .. } if *a >= 18) {
    println!("成年用户");
}
```

---

## 6. 常见错误与陷阱

### 6.1 忘记处理 None

```rust
// ❌ 编译错误
let email = user.email;           // 类型是 Option<String>
let domain = email.split('@');   // Option<String> 没有 split 方法！
```

错误信息：
```
error[E0599]: no method named `split` found for enum `Option<String>`
```

### 6.2 match 分支不穷尽

```rust
// ❌ 编译错误
match status {
    UserStatus::Active => "活跃",
    UserStatus::Inactive => "未激活",
    // 缺少 Banned 分支
}
```

错误信息：
```
error[E0004]: non-exhaustive patterns: `Banned(_)` not covered
```

### 6.3 在 match 中移动了所有权

```rust
// ❌ 编译错误（如果后续还要使用 profile）
match profile.email {
    Some(e) => println!("{e}"),  // e 被移动到 match 分支中
    None => (),
}
println!("{:?}", profile.email); // ❌ profile.email 已经被移动
```

**修复**：使用引用 `match &profile.email { ... }`。

### 6.4 let else 的 else 块不是发散表达式

```rust
// ❌ 编译错误
let Some(x) = optional_value else {
    println!("没有值");
    // 这里没有 return/break/continue/panic! —— 代码会继续往后执行
};
```

错误信息：
```
error[E0308]: `else` block must diverge
```

**修复**：在 `else` 块末尾加上 `return`、`break`、`continue` 或 `panic!()`。

### 6.5 混淆 as_ref() 和引用

```rust
let maybe: Option<String> = Some("hello".into());
// 想要 Option<&str>
let wrong: Option<&String> = Some(&maybe.unwrap()); // 麻烦且不安全

// 正确
let right: Option<&str> = maybe.as_deref();
```

`Option::as_ref()` 将 `Option<T>` 变为 `Option<&T>`；`Option::as_deref()` 将 `Option<String>` 变为 `Option<&str>`。

---

## 7. 编译器怎么说

Rust 编译器在模式匹配方面的错误信息非常友好。

### 7.1 穷尽性检查

```rust
match status {
    UserStatus::Active => "活跃",
}
```

编译器输出：
```
error[E0004]: non-exhaustive patterns: `Inactive` and `Banned(_)` not covered
  --> src/main.rs:XX:YY
   |
XX |     match status {
   |           ^^^^^^ patterns `Inactive` and `Banned(_)` not covered
   |
note: `UserStatus` defined here
  --> src/main.rs:XX:YY
   |
XX | enum UserStatus {
   |      ----------
...
XX |     Inactive,
   |     ^^^^^^^^ not covered
XX |     Banned(String),
   |     ^^^^^^ not covered
   = help: ensure that all possible cases are being handled, possibly by adding
           wildcards or more match arms
```

编译器精确地告诉你了**哪些变体没有被覆盖**，甚至提示你可以通过添加通配符或更多分支来修复。

### 7.2 类型不匹配

```rust
let s: String = Some("hello".to_string());
```

编译器输出：
```
error[E0308]: mismatched types
  --> src/main.rs:XX:YY
   |
XX |     let s: String = Some("hello".to_string());
   |            ------   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `Option<String>`
   |            |
   |            expected due to this
   |
   = note: expected struct `String`
                found enum `Option<String>`
help: consider using `Option::unwrap` or pattern matching
```

编译器告诉你：你声明了 `String` 类型，但右边是 `Option<String>`。还贴心地给出了修复建议。

### 7.3 let else 不发散

```rust
let Some(x) = optional else {
    println!("missing");
};
```

编译器输出：
```
error[E0308]: `else` block must diverge
  --> src/main.rs:XX:YY
   |
XX |       let Some(x) = optional else {
   |  ____________________-
XX | |         println!("missing");
XX | |     };
   | |_____- this must diverge
   |
help: try adding `return` at the end of the block
```

---

## 8. 如何修复

### 8.1 处理 Option 的四种层级

**层级 1：无脑 unwrap（仅用于原型/测试）**
```rust
let email = user.email.unwrap(); // None 时 panic
```

**层级 2：提供默认值**
```rust
let email = user.email.unwrap_or_else(|| "unknown@unknown.com".into());
```

**层级 3：模式匹配（生产级）**
```rust
let display_email = match &user.email {
    Some(e) => e.clone(),
    None => "未提供邮箱".to_string(),
};
```

**层级 4：传播 Option（最惯用）**
```rust
fn get_domain(user: &UserProfile) -> Option<&str> {
    user.email.as_ref()?.split('@').nth(1)
}
// `?` 操作符在 Option 上：遇到 None 就提前返回 None
```

### 8.2 将 match 从"移动"改为"借用"

```rust
// 所有权移动（之后不能再使用 status）
match status { ... }

// 借用（之后还可以使用 status）
match &status { ... }

// 可变借用
match &mut status { ... }
```

### 8.3 用通配符 _ 处理不关心的分支

```rust
match status {
    UserStatus::Banned(reason) => format!("封禁: {reason}"),
    _ => "其他状态".to_string(),  // Active 和 Inactive 统一处理
}
```

---

## 9. 适用边界与最佳实践

### 9.1 什么时候用 match 而不是 if let

| 场景 | 推荐 |
|------|------|
| 只关心一种情况，其他全部忽略 | `if let` |
| 需要覆盖所有变体（穷尽性） | `match` |
| 两个或三个分支，不需要穷尽性 | `match` 也可以用 |
| 需要编译器在新增变体时提醒 | `match`（无 `_` 通配） |

**最佳实践**：如果你在处理一个可能新增变体的枚举，**不要**在 `match` 中使用 `_` 通配符。这样当你新增变体时，编译器会在所有 `match` 处报错，提醒你更新。这就是"让编译器帮你重构"。

### 9.2 什么时候用 let else 而不是 if let

```rust
// 风格 1: if let + else { return } —— 嵌套深
if let Some(a) = get_a() {
    if let Some(b) = get_b() {
        if let Some(c) = get_c() {
            do_something(a, b, c);
        } else { return; }
    } else { return; }
} else { return; }

// 风格 2: let else —— 扁平
let Some(a) = get_a() else { return; };
let Some(b) = get_b() else { return; };
let Some(c) = get_c() else { return; };
do_something(a, b, c);
```

`let else` 是**防御性编程的利器**：先检查所有前置条件，不满足就立即退出，主逻辑保持扁平。

### 9.3 Option vs Result

- `Option<T>`：值可能不存在（语义："这个函数可能不返回结果"）
- `Result<T, E>`：操作可能失败（语义："这个函数可能出错，出错时有错误信息"）

简单规则：如果缺失是**正常的**（如用户没填邮箱），用 `Option`；如果缺失代表**异常**（如数据库连接失败），用 `Result`。

### 9.4 避免 Option 的过度嵌套

```rust
// 不好：Option<Option<T>>
fn get_config() -> Option<Option<String>> { ... }

// 好：扁平化
fn get_config() -> Option<String> { ... }
```

`Option<Option<T>>` 几乎永远是个设计问题。用自定义枚举代替：

```rust
enum ConfigValue<T> {
    NotSet,    // 替代外层 None
    Set(T),    // 替代 Some(T)
    ExplicitlyEmpty, // 替代 Some(None)
}
```

---

## 10. 章节总结

### 核心概念一览

| 概念 | 说明 |
|------|------|
| **Enum（枚举）** | 一种类型，有多个命名的变体，每个变体可以携带数据 |
| **Variant（变体）** | 枚举的一个分支，可以是纯标签，也可以携带数据 |
| **Option\<T\>** | 标准库枚举，`Some(T)` 表示有值，`None` 表示没有值 |
| **match** | 穷尽性模式匹配，编译器强制覆盖所有可能 |
| **if let** | `match` 的语法糖，只匹配一种模式 |
| **let else** | 模式不匹配时提前发散（return/break/continue） |
| **while let** | 循环匹配，直到模式不匹配为止 |
| **Pattern（模式）** | 可解构、可嵌套、可带守卫条件的匹配模板 |

### 关键认知转变（从 Python 到 Rust）

1. **null/None → Option\<T\>**：从"世界上任何值都可能是 null"变为"只有 Option 才可能缺失"。类型本身就是文档。

2. **运行时检查 → 编译期强制**：你不可能"忘了"处理 Option——编译器不让你编译。

3. **match 不是 if-else**：穷尽性检查是模式匹配的本质特征。`if-else` 是"条件分支"；`match` 是"穷尽解构"。

4. **错误处理不要用 Option**：缺失值用 `Option`，业务错误用 `Result`。它们泾渭分明。

### 一句话总结

> Rust 用 `Option<T>` 替代了 null，用 `match` 的穷尽性检查确保每个可能缺失的值都被处理。这不是"更安全"，而是"不可能不安全"——编译器在编译期就消灭了空指针异常的可能性。
>
> Tony Hoare 的十亿美元错误，在 Rust 中不存在。

---

## 术语表

| 英文 | 中文 | 含义 |
|------|------|------|
| Enum | 枚举 | 有多个变体的类型 |
| Variant | 变体 | 枚举的一个可能取值 |
| Option | 可选值 | 表示"可能有，也可能没有"的类型 |
| Some | 有值 | Option 的变体，包裹实际值 |
| None | 无值 | Option 的变体，表示不存在 |
| match | 匹配 | 穷尽性模式匹配表达式 |
| Exhaustiveness | 穷尽性 | 编译器检查所有变体是否被覆盖 |
| if let | 如果匹配 | 单模式条件匹配 |
| let else | 否则 | 模式不匹配时发散 |
| Pattern | 模式 | 可解构的匹配模板 |
| Wildcard (`_`) | 通配符 | 匹配任意值但不绑定 |
| Guard (`if`) | 守卫条件 | 为模式匹配增加额外布尔条件 |
| Combinator | 组合子 | Option/Result 的链式处理方法 |
| Diverge | 发散 | 不返回的函数（如 panic!） |
| Prelude | 预导入 | 自动导入到所有程序的标准类型/函数 |

---

*下一章预告：第 10 章 —— Result 与错误处理，深入探讨 Rust 如何用类型系统优雅地处理可恢复错误。*
