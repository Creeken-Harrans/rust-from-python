# 第 9 章：枚举、Option 与模式匹配

> Rust 的类型安全核心 —— 用类型系统在编译期消灭空指针异常（NullPointerException）

---

## 目录

1. [问题引入：没有 null 的语言如何表达"可能没有值"](#1-问题引入没有-null-的语言如何表达可能没有值)
2. [Python 视角：你熟悉的 None 和 match](#2-python-视角你熟悉的-none-和-match)
3. [Rust 的设计：Enum + Option + 模式匹配](#3-rust-的设计enum-option-模式匹配)
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

> **说明**：本节为教学示例，部分函数（如 `describe_status`、`get_domain`）为概念插图，与 `src/main.rs` 中的 `Display` impl、`get_email_domain`、`must_find_user` 等实际代码在命名上有所不同，但覆盖全部核心概念。建议配合 `src/main.rs` 运行对照学习。

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

## Python、C 与 C++ 对照

C 语言的枚举本质上是给整数常量起名字：

```c
enum Color { Red, Green, Blue };  // Red=0, Green=1, Blue=2
enum Color c = Red;
c = 42;  // 完全合法——C 编译器不限制枚举变量的取值范围
```

C 枚举有三大局限：

1. **不能携带数据**——你无法表达"连接状态是 Connected，且对端地址是 192.168.1.1"。每个变体只是一个整数标签，没有任何数据载荷能力。
2. **不是真正的独立类型**——枚举变量和 `int` 可以自由互转。函数签名 `void set_state(enum State s)` 无法阻止调用者传入任意整数值（如 `set_state(99)`）。
3. **没有穷尽性检查**——`switch` 不强制覆盖所有枚举值，遗漏的分支静默穿过，不产生任何警告或错误。

C++ 的 `enum class`（C++11）解决了作用域污染和隐式整数转换问题，但本质仍然是整数标签：

```cpp
enum class Color { Red, Green, Blue };
Color c = Color::Red;
// c = 42;                  // ❌ 编译错误：不能隐式转换 int
int i = static_cast<int>(c); // 需要显式转换
```

`enum class` 是强类型的，但**依然不能携带数据**，也**没有穷尽性检查**。它是"更安全的 C 枚举"，但远不是代数数据类型。

C++17 引入了 `std::variant`，向 Rust 枚举靠近了一步：

```cpp
#include <variant>
#include <string>

using ConnectionState = std::variant<
    std::monostate,              // Disconnected
    unsigned int,                // Connecting { attempt }
    std::string,                 // Connected { peer }
    std::string                  // Failed { reason } — 问题：两个变体都是 string
>;
```

`std::variant` 的局限很明显：

- **无法命名变体**——你看到的是 `variant<monostate, uint, string, string>` 而非 `Connected(peer)` 和 `Failed(reason)`，可读性很差。代码注释需要承担类型本该承担的表达力。
- **类型必须互不相同**——上面的 `Connected` 和 `Failed` 都携带 `std::string`，编译器无法区分它们。真实项目中必须借助标签结构体等工作量不小的技巧来绕过。
- **穷尽性靠模板而非语言层面检查**——`std::visit` 遗漏一个类型时，报错是模板展开失败的数百行晦涩信息，而非一条清晰的"你漏了 Failed 变体"。

Rust 的枚举是代数数据类型（Algebraic Data Type），每个变体拥有独立的名称和数据结构：

```rust
enum ConnectionState {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { peer: String },
    Failed { reason: String },
}
```

同样的连接状态，在传统 C/C++ 中需要用多个布尔字段加可选字符串来拼凑：

```c
// 传统 C/C++ 做法：用多个字段模拟一个状态
struct Connection {
    bool is_connected;
    bool is_connecting;
    bool is_failed;
    int  attempt_count;      // 只在 is_connecting 时有效
    char peer[64];           // 只在 is_connected 时有效
    char fail_reason[256];   // 只在 is_failed 时有效
};
```

这种做法的根本缺陷在于**非法状态可表示**：

- `is_connected == true && is_failed == true` —— 这个组合在语义上不可能，但在类型系统中完全合法。你必须在代码中额外防御它，否则就是 bug。
- **字段语义靠约定**：调用者必须"知道" `attempt_count` 只在 `is_connecting` 时有效，编译器不帮你检查。任何误读约定都是运行时错误。
- **新增状态会污染整个结构体**：加一个 `is_reconnecting` 字段不仅增加存储开销，还会引入更多非法状态组合（`is_reconnecting && is_connected`？`is_reconnecting && is_failed`？），复杂度呈指数增长。

Rust 的枚举从类型层面消除了这些问题：`ConnectionState` 在任意时刻**恰好是**四种状态之一，不可能出现 `Connected` 和 `Failed` 同时成立的情况。这就是"让非法状态不可表示"（making illegal states unrepresentable）——类型系统本身保证了数据一致性，不需要靠文档约定或运行时断言来补救。

---

## Option\<T\> 深入：空指针问题与类型安全

C/C++ 中，任何指针都可以是 `NULL`（或 `nullptr`）：

```c
char* find_user(const char* name);  // 返回值可能是 NULL，也可能不是

char* user = find_user("alice");
printf("%s\n", user);  // 💥 如果 user 是 NULL → segmentation fault
```

函数的类型签名**不会告诉你**返回值到底可不可能为空。`char*` 可能指向有效内存，可能是 `NULL`，还可能是悬垂指针。你只能靠文档、命名约定（如 `maybe_find_user`）或运行时崩溃来猜测真相。Tony Hoare——`null` 引用的发明者——在 2009 年将其称为"十亿美元错误"：这个单一设计决策数十年来导致了不计其数的安全漏洞、系统崩溃和调试时间。

Python 的 `None` 在概念上类似，但有了类型提示（type hints）后情况有所好转：

```python
def find_user(name: str) -> UserProfile | None:  # 类型提示标明可能返回 None
    ...
```

问题在于：类型提示是**可选的**，且只在静态分析工具（如 mypy）运行时生效。Python 解释器自身不强制执行类型提示——你完全可以忽略 `None` 检查而直接使用返回值，直到运行时抛出 `AttributeError`。

Rust 的 `Option<T>` 从根本上解决了这个问题：**`Option<T>` 和 `T` 是两个不同的类型**。你不可能绕过 `Option` 直接使用内部值：

```rust
let email: Option<String> = Some("alice@example.com".into());
// email.to_uppercase();  // ❌ 编译错误：Option<String> 没有 to_uppercase() 方法
```

类型标注 `Option<String>` 本身就是文档——任何人看到这个签名都立刻知道返回值可能不存在。更重要的是，编译器强制执行：你**必须**通过 `match`（或等价的组合子方法）处理 `Some` 和 `None` 两种情况，否则代码无法编译。不可能"忘了检查 None"。

Rust 引用（`&T`）的另一个关键保证是：**引用永远不为空**。`&T` 一定指向一个有效的 `T`，你无法创造一个值为 `null` 的 `&str`。当你确实需要一个"可能不存在的引用"时，使用 `Option<&T>`——它明确表达了可空性，且编译器强制检查。这与 C/C++ 中任何指针都可以为 `NULL` 形成了根本性的差异：

| 语言 | "可能为空的引用/指针" | 编译器强制检查 |
|------|----------------------|--------------|
| C | `T*`（任何指针都可能空，类型不表达） | 无 |
| C++ | `T*` 或 `std::optional<std::reference_wrapper<T>>` | 无 / 弱（静态分析工具） |
| Python | `Optional[T]`（类型提示，运行时不管） | 仅静态分析工具（mypy） |
| Rust | `Option<&T>` | 编译期强制 |

`match` 是强制调用者面对现实的机制：当函数返回 `Option<UserProfile>`，调用者**必须**写出 `Some(profile) => { ... }` 和 `None => { ... }` 两个分支（或用 `if let` 明确跳过其中一个）。这不是代码风格的建议，是编译器层面的硬性规则。空指针异常——这个困扰了业界半个世纪的问题——在 Rust 中不是"更不容易发生"，而是**在编译期被消灭了**。

---

## match 穷尽性：从 C switch 到编译器强制检查

C 语言的 `switch` 对穷尽性没有任何要求：

```c
enum Status { Active, Inactive, Banned };

enum Status s = Banned;
switch (s) {
    case Active:   printf("活跃\n"); break;
    case Inactive: printf("未激活\n"); break;
    // Banned 被遗漏了——没有警告，没有错误，编译通过，静默跳过
}
```

更危险的是 C `switch` 的默认行为是**贯穿（fallthrough）**——忘了写 `break` 就会继续执行下一个 `case` 的代码。这是 C 语言中经典且高频的 bug 来源。

C++ 的 `switch` 在 GCC/Clang 中可以通过 `-Wswitch` 选项对遗漏的 `enum class` 值产生**警告**，但这有两个致命弱点：一是警告可以被忽略或关闭，二是添加 `default:` 标签会吞掉所有未显式列出的分支，使警告失效。

Rust 的 `match` 采取完全不同的策略：**编译期强制穷尽性检查，缺失任何变体就是编译错误**。

```rust
match status {
    UserStatus::Active => "活跃",
    UserStatus::Inactive => "未激活",
    // ❌ 编译错误：error[E0004]: non-exhaustive patterns: `Banned` not covered
}
```

这个特性在重构时价值巨大。假设你在代码库中有一个 `UserStatus` 枚举，被分布在多个模块中的 50 个 `match` 表达式使用。现在产品需求变更，你需要新增一个变体 `UserStatus::Suspended`。在 C/C++ 项目中，你必须人工搜索所有 `switch` 语句，逐一判断是否需要添加 `case——`遗漏是常态，bug 往往在上线后才发现。

在 Rust 项目中，你只需在枚举定义处添加一行 `Suspended`，然后编译。编译器会在**所有 50 个 `match` 处**精确报错，列出文件、行号和缺失的变体。你逐条修复，直到编译通过。编译器帮你完成了"影响范围分析"的工作——不会遗漏，不需搜索，不可能在上线后才发现某个角落没有处理新状态。

| 特性 | C `switch` | C++ `switch` | Rust `match` |
|------|-----------|-------------|-------------|
| 穷尽性检查 | 无 | 可选警告（`-Wswitch`） | 编译期强制错误 |
| 默认贯穿 | 是（fallthrough） | 是（fallthrough） | 否（每分支自动终止，手动 fallthrough 需 `=>` 后无分号） |
| 新增变体时 | 静默忽略 | 可能产生警告（无 `default` 时） | 所有 `match` 处精确报编译错误 |
| 分支表达力 | 整数 / 枚举常量 | 整数 / 枚举常量 | 任意嵌套模式、解构、守卫条件、`@` 绑定 |

Rust 的模式匹配不只是"更安全的 `switch`"——它把运行时的不确定性变成了编译期的确定性，是语言层面对正确性的承诺。

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

> 📚 **相关章节**：[08 结构体与方法](../08_structs_methods_associated_functions/) | [10 集合类型](../10_collections_vec_string_hashmap/) | [11 模式与解构](../11_patterns_and_destructuring/) | [15 泛型与特征](../15_generics_traits_trait_bounds/)

*下一章预告：第 10 章 —— Result 与错误处理，深入探讨 Rust 如何用类型系统优雅地处理可恢复错误。*
