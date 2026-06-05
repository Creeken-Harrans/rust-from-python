# 第 23 章: 宏 (Macros) — Rust 的元编程

## 目录

1. [什么是宏](#什么是宏)
2. [函数 vs 宏](#函数-vs-宏)
3. [为什么 println! 有感叹号](#为什么-println-有感叹号)
4. [声明式宏: macro_rules!](#声明式宏-macro_rules)
5. [宏中的模式匹配](#宏中的模式匹配)
6. [重复模式详解](#重复模式详解)
7. [标准库常用宏](#标准库常用宏)
8. [过程宏概述](#过程宏概述)
9. [为什么过程宏需要独立 crate](#为什么过程宏需要独立-crate)
10. [宏卫生](#宏卫生)
11. [何时不应该使用宏](#何时不应该使用宏)
12. [实际项目中的宏示例](#实际项目中的宏示例)
13. [Python 对照](#python-对照)
14. [调试宏的技巧](#调试宏的技巧)
15. [总结](#总结)

---

## 什么是宏

**宏（Macro）是"编写代码的代码"**——一种元编程（Metaprogramming）技术。

在 Rust 中，宏不是简单的文本替换（不像 C 语言的 `#define`），而是工作在 **AST（抽象语法树）** 层面的代码生成。编译器在编译期间将宏展开为 Rust 代码，然后对展开后的代码进行类型检查、编译和优化。

宏的核心价值在于：
- **减少样板代码（Boilerplate）**：用少量代码生成大量重复模式
- **实现可变参数**：函数做不到的变长参数，宏可以
- **编译期计算**：在编译时生成代码，零运行时开销
- **领域特定语言（DSL）**：在 Rust 内部创建小型的专用语言

Rust 的宏分为两大类：
1. **声明式宏（Declarative Macros）**：使用 `macro_rules!` 定义，通过模式匹配来匹配和生成代码
2. **过程宏（Procedural Macros）**：使用 Rust 代码操作 TokenStream，功能更强大但更复杂

---

## 函数 vs 宏

### 什么时候用函数

- 逻辑简单、参数固定
- 需要类型安全（函数签名明确标注参数和返回类型）
- 代码可读性是首要考虑
- 需要在调试器中单步执行

### 什么时候用宏

- 需要**可变数量的参数**（如 `println!`、`vec!`）
- 需要**生成代码结构**（如创建函数、实现 trait）
- 需要在**调用处就地展开**代码（避免函数调用开销，虽然通常不是主要原因）
- 需要**编译期计算**和代码生成
- 实现**领域特定语言（DSL）**

### 关键对比表

| 特性 | 函数 `fn` | 宏 `macro_rules!` |
|------|-----------|-------------------|
| 展开时机 | 运行时调用 | 编译期展开 |
| 参数数量 | 固定 | 可变（0 到 N 个） |
| 参数类型 | 必须标注类型 | 模式匹配，灵活 |
| 返回值 | 必须声明类型 | 可生成任意代码（无固定"返回值"） |
| 代码生成 | 不能生成新函数/结构体等 | 可以生成任意 Rust 语法结构 |
| 类型检查 | 定义时检查签名 | 展开后检查（错误信息可能不直观） |
| 调试 | 可在调试器中单步 | 无法直接调试宏内部 |
| 调用语法 | `func(args)` | `macro!(args)` 或 `macro![args]` |
| 命名规范 | snake_case | snake_case + `!` 后缀 |
| 自动补全 | IDE 支持好 | IDE 支持有限（展开前） |

### 核心区别实例

```rust
// 函数：参数数量固定
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// 宏：可以接收任意数量的参数
macro_rules! sum_all {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

// 使用
let a = sum_all!(1, 2, 3, 4, 5);        // 可以
let b = sum_all!(1, 2);                  // 也可以
let c = sum_all!(1, 2, 3, 4, 5, 6, 7);  // 还是可以
```

函数无法做到 `sum_all!` 这样接收任意数量的参数（除非使用 slice 或迭代器，但那改变了调用方式）。

---

## 为什么 println! 有感叹号

Rust 中所有宏调用后面都有一个 `!`，这是一个**语法标记**，让程序员和编译器都能一眼区分宏调用和函数调用。

`println!` 之所以是宏而非函数，有几个关键原因：

1. **可变参数**：`println!("{}, {}, {}", a, b, c)` 可以接收任意数量的参数，这是函数做不到的
2. **编译期格式字符串验证**：编译器在展开宏时检查格式字符串中的 `{}` 数量是否与参数数量匹配
3. **字符串内联**：格式字符串必须在编译期是已知的常量

```rust
// 如果 println 是函数（伪代码），你只能这样调用：
// fn println(format: &str) — 无法传递额外参数！

// 但 println! 是宏，可以这样：
println!("x = {}, y = {}", x, y);   // 两个额外参数
println!("hello");                   // 零个额外参数
println!("a={}, b={}, c={}", a, b, c); // 三个额外参数
```

`!` 是一个约定而非语法强制——编译器通过名称查找宏定义，但 `!` 让代码的意图在视觉上更加明显。读到 `something!()` 时，你知道"这里发生了代码展开"。

---

## 声明式宏: macro_rules!

`macro_rules!` 是 Rust 中最常用的宏定义方式。它使用**模式匹配**的语法来描述如何将输入的 token 转换为输出的代码。

### 基本语法

```rust
macro_rules! macro_name {
    (模式1) => {
        展开代码1
    };
    (模式2) => {
        展开代码2
    };
    // ... 更多分支
}
```

### 最简单的宏

```rust
macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
}

// 调用
hello!();  // 展开为 println!("Hello, world!");
```

### 带参数的宏

```rust
macro_rules! say {
    ($expr:expr) => {
        println!("{} = {}", stringify!($expr), $expr);
    };
}

say!(2 + 3);  // 输出: 2 + 3 = 5
```

这里的 `$expr:expr` 是**片段说明符（Fragment Specifier）**，表示匹配任意表达式并绑定到 `$expr`。

### 生成代码的宏

```rust
macro_rules! create_function {
    ($name:ident, $body:expr) => {
        fn $name(x: i32) -> i32 {
            $body
        }
    };
}

create_function!(square, x * x);
// 展开后：
// fn square(x: i32) -> i32 {
//     x * x
// }

println!("{}", square(5)); // 25
```

这是函数做不到的——在编译期生成一个新的**函数定义**本身。

### 片段说明符总览

| 说明符 | 匹配内容 | 示例 |
|--------|----------|------|
| `:ident` | 标识符（变量名、函数名等） | `foo`, `my_var` |
| `:expr` | 表达式 | `1 + 2`, `x.method()` |
| `:ty` | 类型 | `i32`, `Vec<String>` |
| `:stmt` | 语句 | `let x = 1;` |
| `:block` | 代码块 | `{ ... }` |
| `:pat` | 模式 | `Some(x)`, `1..=10` |
| `:path` | 路径 | `std::collections::HashMap` |
| `:meta` | 元数据（属性内部） | `derive(Debug)` |
| `:item` | 项（函数、结构体等） | `fn foo() {}` |
| `:tt` | 单个 token 树 | 任意 token |
| `:literal` | 字面量 | `42`, `"hello"`, `true` |
| `:lifetime` | 生命周期标注 | `'a`, `'static` |
| `:vis` | 可见性修饰符 | `pub`, `pub(crate)` |

---

## 宏中的模式匹配

宏的模式匹配类似于 `match` 表达式，但匹配的是**语法结构**而非值。

### 基本模式

```rust
macro_rules! match_example {
    // 匹配单个表达式
    (single $x:expr) => { /* ... */ };
    // 匹配两个表达式，用逗号分隔
    (pair $a:expr, $b:expr) => { /* ... */ };
    // 匹配一个标识符后跟一个表达式
    (named $name:ident = $value:expr) => { /* ... */ };
}
```

### 多分支匹配

```rust
macro_rules! respond {
    (hello) => { println!("你好！"); };
    (bye)   => { println!("再见！"); };
    ($other:expr) => { println!("收到: {:?}", $other); };
}

respond!(hello);  // 你好！
respond!(bye);    // 再见！
respond!(42);     // 收到: 42
```

### 模式匹配的威力

宏的模式匹配允许你根据**输入的结构**产生完全不同的代码。这是函数参数做不到的——函数只能根据**参数的值**走不同分支，而宏可以根据**源代码的结构**走不同分支。

---

## 重复模式详解

重复模式是 `macro_rules!` 最强大的特性之一，允许匹配和生成**任意数量**的语法片段。

### 三种重复运算符

| 运算符 | 含义 |
|--------|------|
| `*` | 重复零次或多次 |
| `+` | 重复一次或多次 |
| `?` | 重复零次或一次（可选） |

### 语法

```rust
$( 重复内容 )运算符
```

### 基本示例

```rust
macro_rules! print_all {
    ($($item:expr),*) => {
        $(
            println!("item = {:?}", $item);
        )*
    };
}

print_all!(1, 2, 3, "hello");
// 展开为：
// println!("item = {:?}", 1);
// println!("item = {:?}", 2);
// println!("item = {:?}", 3);
// println!("item = {:?}", "hello");
```

### 分隔符变体

```rust
// 逗号分隔
macro_rules! with_commas {
    ($($x:expr),*) => { /* ... */ };
}

// 分号分隔
macro_rules! with_semicolons {
    ($($x:expr);*) => { /* ... */ };
}

// 无分隔符
macro_rules! no_sep {
    ($($x:expr)*) => { /* ... */ };
}
```

### 使用重复捕获的变量

```rust
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

let v = my_vec![1, 2, 3, 4, 5]; // vec![1, 2, 3, 4, 5]
```

### 多个重复模式

可以在同一个宏中使用多个独立的重复模式：

```rust
macro_rules! pair_up {
    ($($key:expr => $value:expr),*) => {
        $(
            println!("{} -> {}", $key, $value);
        )*
    };
}

pair_up!("name" => "Alice", "age" => 30, "city" => "Beijing");
```

### 允许末尾逗号

Rust 习惯允许在列表末尾有多余的逗号。Rust 2024 中可以这样实现：

```rust
macro_rules! flexible_list {
    ($($x:expr),* $(,)?) => {
        // $(,)? 匹配零个或一个末尾逗号
        $(
            println!("{}", $x);
        )*
    };
}
```

### 递归宏

宏可以递归调用自身，实现类似循环的效果：

```rust
macro_rules! count {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => {
        1usize + count!($($tail),*)
    };
}

let n = count!(a, b, c, d); // 4
```

---

## 标准库常用宏

Rust 标准库提供了大量实用的宏，理解它们是高效编写 Rust 代码的关键。

### 输出宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `println!` | 打印到 stdout，带换行 | `println!("Hello, {}!", name)` |
| `print!` | 打印到 stdout，无换行 | `print!("Loading...")` |
| `eprintln!` | 打印到 stderr，带换行 | `eprintln!("Error: {}", msg)` |
| `eprint!` | 打印到 stderr，无换行 | `eprint!("Warning: "）` |
| `format!` | 格式化为 String | `let s = format!("{:.2}", pi)` |

### 集合宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `vec!` | 创建 Vec | `vec![1, 2, 3]` |
| `vec![]` | 同 `vec!` | `vec![0; 100]` 创建 100 个 0 |

### 断言宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `assert!` | 断言条件为 true | `assert!(x > 0, "x must be positive")` |
| `assert_eq!` | 断言两个值相等 | `assert_eq!(result, expected)` |
| `assert_ne!` | 断言两个值不等 | `assert_ne!(result, 0)` |
| `debug_assert!` | 仅在 debug 模式下断言 | `debug_assert!(ptr.is_aligned())` |

### 调试宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `dbg!` | 打印调试信息并返回值 | `let x = dbg!(compute())` |
| `matches!` | 模式匹配返回 bool | `matches!(x, Some(1..=10))` |

`dbg!` 是一个特别有用的宏——它打印文件名、行号、表达式和值，然后**返回该值的所有权**，所以你可以在表达式中间插入它：

```rust
let result = dbg!(expensive_calculation()) * 2;
// 输出: [src/main.rs:42:16] expensive_calculation() = 42
```

### 占位宏

| 宏 | 用途 |
|----|------|
| `todo!` | 标记未完成的代码，编译通过但运行 panic |
| `unimplemented!` | 类似 `todo!`，语义为"尚未实现" |
| `unreachable!` | 标记逻辑上不应到达的代码路径 |

```rust
fn not_ready_yet() -> String {
    todo!("下个版本实现这个功能")
}

fn handle_error() {
    unreachable!("这个分支在逻辑上不可能被触发")
}
```

### 其他常用宏

| 宏 | 用途 |
|----|------|
| `include_str!` | 在编译期将文件内容嵌入为 &str |
| `include_bytes!` | 在编译期将文件内容嵌入为 &[u8] |
| `env!` | 在编译期读取环境变量 |
| `option_env!` | 编译期读取可选环境变量 |
| `concat!` | 连接字符串字面量 |
| `file!` | 当前文件名 |
| `line!` | 当前行号 |
| `column!` | 当前列号 |
| `stringify!` | 将表达式转为字符串 |
| `cfg!` | 编译期条件判断，返回 bool |
| `panic!` | 触发 panic |
| `write!` / `writeln!` | 写入到 `fmt::Write` 实现者 |
| `compile_error!` | 编译期产生错误 |
| `thread_local!` | 定义线程局部变量 |

---

## 过程宏概述

**过程宏（Procedural Macros）** 是比 `macro_rules!` 更强大但也更复杂的宏机制。它们运行在编译期，接收和产生 **TokenStream**（token 流），允许用 Rust 代码以编程方式处理 Rust 代码。

### 三种过程宏

#### 1. 派生宏（Derive Macros）

最常见的过程宏类型，用在 `#[derive(...)]` 中：

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}
```

`Debug`、`Clone` 等 derive 宏会为 `Person` 自动生成对应的 trait 实现代码。你也可以自己写 derive 宏。

常用第三方 derive 宏：
- `serde::Serialize` / `serde::Deserialize` — 序列化
- `thiserror::Error` — 错误类型
- `clap::Parser` — CLI 参数解析

#### 2. 属性宏（Attribute Macros）

定义新的属性，可以应用于函数、结构体、模块等：

```rust
#[tokio::main]
async fn main() {
    // tokio::main 将 async fn main 转换为一个同步的 main，
    // 内部初始化 tokio 运行时并调用 async 函数
}

#[test]
fn my_test() {
    // #[test] 是内置的属性宏，标记测试函数
}
```

属性宏可以**替换、修改或包装**被标注的代码项。

#### 3. 函数式过程宏（Function-like Procedural Macros）

调用方式像 `macro_rules!` 宏，但功能更强：

```rust
let sql = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id);
// sqlx::query! 在编译期连接数据库验证 SQL 语法
```

函数式过程宏接收括号内的 token 流，返回新的 token 流。

### 过程宏的工作方式

一个过程宏是一个 Rust 函数：

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // 解析 input（被标注的项的 token 流）
    // 生成新的代码（实现 MyTrait 的代码）
    // 返回生成的 token 流
}
```

编译器在编译时调用这个函数，将输入的 token 流传入，接收输出的 token 流并合并到编译产物中。

---

## 为什么过程宏需要独立 crate

这是一个 Rust 编译模型的根本性约束：

### 技术原因

1. **过程宏在编译期执行**：编译器需要先编译过程宏 crate，然后**加载它**（作为动态库或可执行代码），最后才用它们处理其他 crate
2. **宿主和宏分离**：宏 crate 编译为 `.so`/`.dll`，编译器在编译主 crate 时加载它们
3. **循环依赖问题**：如果过程宏和调用它的代码在同一个 crate，就会出现"编译宏需要编译 crate，编译 crate 需要宏"的死锁

### 配置方式

```toml
# 在过程宏 crate 的 Cargo.toml 中：
[lib]
proc-macro = true

[dependencies]
syn = "2"       # 解析 token 流为 AST
quote = "1"     # 将 AST 转换回 token 流
```

```toml
# 在使用该过程宏的 crate 的 Cargo.toml 中：
[dependencies]
my_macros = { path = "../my_macros" }
```

### 声明式宏没有这个限制

`macro_rules!` 定义的宏在**同一个 crate 内就可以使用**，因为它们在编译器的宏展开阶段处理，不需要额外编译一个 crate。

### 实际项目结构

```
my_project/
├── my_macros/          # 过程宏 crate (proc-macro = true)
│   ├── Cargo.toml
│   └── src/lib.rs
├── my_lib/             # 库 crate
│   ├── Cargo.toml      # 依赖 my_macros
│   └── src/lib.rs      # 使用 my_macros 中的宏
└── my_app/             # 二进制 crate
    ├── Cargo.toml
    └── src/main.rs
```

---

## 宏卫生

**宏卫生（Macro Hygiene）** 是 Rust 宏系统的一个重要特性。

### 什么是宏卫生

宏卫生意味着宏内部定义的变量**不会意外地**与宏调用处的变量发生冲突。

### C 语言的对比（不卫生的宏）

```c
#define SWAP(a, b) { int temp = a; a = b; b = temp; }

int main() {
    int temp = 42;  // 外部有个名为 temp 的变量
    int x = 1, y = 2;
    SWAP(x, y);     // 宏内部的 temp 与外部 temp 冲突！
    // 编译错误或意外行为
}
```

### Rust 的宏卫生

```rust
macro_rules! my_swap {
    ($a:expr, $b:expr) => {
        let temp = $a;  // 宏内部的 temp
        // 这里 $a 和 $b 需要是可变绑定……
    };
}

// Rust 的卫生机制确保宏内部的变量不会影响外部
```

### 卫生的细节

- **变量**：宏内部 `let` 绑定的变量有独立的"语法上下文"，不会与调用处同名变量冲突
- **标识符**：宏中引入的标识符默认不会与外部交互
- **跨 crate**：从其他 crate 导入的宏也是卫生的

### 有意破坏卫生

有时你确实希望宏内部定义的标识符"泄漏"到调用处。比如在调用处引入一个变量名。这时你需要让宏**接收一个标识符作为参数**：

```rust
macro_rules! make_getter {
    ($struct:ident, $field:ident) => {
        impl $struct {
            pub fn $field(&self) -> &i32 {
                &self.$field
            }
        }
    };
}
```

这样调用者明确提供了标识符，不存在意外的名字冲突。

---

## 何时不应该使用宏

宏是强大的工具，但也带来了代价。以下情况应该**避免使用**宏：

### 1. 函数能做到的事情

如果普通函数能完成同样的工作，优先使用函数。函数有明确的类型签名、更好的错误信息、更易调试。

```rust
// ❌ 不必要的宏
macro_rules! add {
    ($a:expr, $b:expr) => { $a + $b };
}

// ✅ 直接用函数
fn add(a: i32, b: i32) -> i32 { a + b }
```

### 2. 严重损害可读性

宏展开是透明的——阅读代码的人看不到宏内部做了什么。过度使用宏会让代码变得像"黑魔法"：

```rust
// ❌ 过度宏化
define_controller!(UserController, 
    routes => [index, show, create],
    model => User,
    auth => required
);
// 读这段代码的人完全不知道发生了什么
```

### 3. 复杂的控制流

如果宏内部有复杂的条件判断和循环，展开后的代码可能非常难理解。

### 4. 可以交给泛型和 trait 的场景

Rust 的泛型系统已经很强大了，很多"代码生成"的需求可以通过泛型 + trait 实现，而且类型安全更好。

```rust
// ❌ 为每种类型生成函数（C 风格）
create_parse_func!(parse_int, i32);
create_parse_func!(parse_float, f64);

// ✅ Rust 风格：使用泛型
fn parse<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.parse()
}
```

### 5. 调试密集的代码

宏无法单步调试。如果某段代码需要频繁调试，将其抽取为函数会更合适。

### 原则

> **先用函数，发现做不到或太冗长时再考虑宏。**
> 
> 好的宏是"你几乎感觉不到它在工作"的——它看起来像是语言的自然扩展。

---

## 实际项目中的宏示例

### 1. 日志宏（log crate）

```rust
// log crate 的典型用法
log::info!("用户 {} 登录成功，IP: {}", username, ip);
log::error!("连接失败: {}", err);
log::debug!("请求参数: {:?}", params);

// 宏的好处：
// - 可编译期关闭 debug! 级别的日志（零开销）
// - 自动添加文件名、行号、模块路径
// - 格式化字符串编译期检查
```

### 2. serde 序列化

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    #[serde(default = "default_timeout")]
    timeout: u64,
}
```

`serde` 的 derive 宏分析结构体定义，自动生成序列化和反序列化代码——这手动写起来非常冗长且容易出错。

### 3. CLI 解析（clap）

```rust
#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    /// 输入文件路径
    input: String,
    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}
```

直接从结构体定义生成 CLI 参数解析代码。

### 4. 错误处理（thiserror）

```rust
#[derive(Error, Debug)]
enum MyError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("解析失败: {msg}")]
    Parse { msg: String },
}
```

自动实现 `Display` 和 `Error` trait。

### 5. 测试框架

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[should_panic(expected = "divide by zero")]
    fn test_division_by_zero() {
        let _ = 1 / 0;
    }
}
```

`#[test]`、`#[should_panic]` 等是内置属性宏，将普通函数转换为测试用例。

---

## Python 对照

Python 没有编译期宏的概念，但有一些类似的思想：

### Python 装饰器 vs Rust 属性宏

```python
# Python 装饰器 — 运行时包装函数
@timer
def slow_function():
    time.sleep(1)
    return 42
```

```rust
// Rust 属性宏 — 编译期转换代码
#[instrument]
fn slow_function() -> i32 {
    std::thread::sleep(Duration::from_secs(1));
    42
}
```

**本质区别**：
- Python 装饰器在**运行时**包装函数，修改行为
- Rust 属性宏在**编译期**展开，生成新代码
- Python 装饰器有运行时开销；Rust 宏展开后零运行时开销

### Python 元类 vs Rust Derive 宏

```python
# Python 元类 — 在类创建时介入
class ModelMeta(type):
    def __new__(cls, name, bases, attrs):
        # 修改类的属性
        return super().__new__(cls, name, bases, attrs)
```

```rust
// Rust derive 宏 — 编译期生成代码
#[derive(Debug)]
struct Point { x: i32, y: i32 }
```

Python 元类在类**定义时**执行（仍是运行时），Rust derive 宏在**编译时**展开。

### Python 没有的：代码生成能力

Python 无法在"编译期"生成新的代码结构。最接近的是：

- `eval()` / `exec()` — 运行时执行字符串代码（性能差，不安全）
- `__init_subclass__` — 子类创建时修改（有限）
- `inspect` 模块 — 仅能检查已有代码，不能生成

Rust 宏可以在编译期：
- 生成新的函数定义
- 生成新的结构体和枚举
- 实现 trait
- 验证 DSL 语法正确性
- 所有这些**都在编译期完成，零运行时开销**

### Python 的优势

- **灵活性**：运行时可以动态创建类、修改方法
- **简单**：装饰器只是一个函数，学习曲线低
- **动态性**：可以根据运行时条件决定行为

### Rust 宏的优势

- **零开销**：编译期完成，运行时无性能损失
- **安全**：宏展开后经过完整的类型检查
- **编译期错误检查**：像 `println!` 格式字符串错误在编译期捕获

---

## 调试宏的技巧

由于宏在编译期展开，直接调试比较困难。以下是一些有用的技巧：

### 1. 使用 `cargo expand`

```bash
# 安装 cargo-expand
cargo install cargo-expand

# 展开当前 crate 的所有宏
cargo expand

# 展开特定函数
cargo expand main

# 展开特定模块
cargo expand my_module
```

这会显示宏展开后的实际代码，非常有用。

### 2. 使用 `rust-analyzer`

在 VS Code 中，将光标放在宏调用上，使用 "Expand macro recursively" 命令查看展开结果。

### 3. 使用 `compile_error!`

在宏中添加编译错误来"断点"：

```rust
macro_rules! debug_me {
    ($x:expr) => {
        compile_error!(stringify!($x)); // 编译时显示 $x 的内容
    };
}
```

### 4. 使用 `dbg!`

如果宏展开后的代码包含 `dbg!`，运行时就能看到中间值。

### 5. 小步迭代

- 先用最简单的模式编写宏
- 逐步添加更多分支
- 每次用 `cargo check` 验证

### 6. 使用 `stringify!`

在宏中用 `stringify!` 打印变量的"源代码形式"，帮助理解匹配了哪个分支：

```rust
macro_rules! trace_match {
    ($x:expr) => {
        println!("匹配到: {}", stringify!($x));
    };
}
```

---

## 编译和运行

```bash
cd /home/Creeken/Temp/Rust_/rust-from-python/chapters/23_macros
cargo build       # 编译
cargo run         # 运行
cargo check       # 仅检查类型（快）
cargo expand main # 展开宏（需安装 cargo-expand）
```

### 预期输出

程序会依次演示：
1. `hello!()` — 简单宏
2. `say!()` — 打印表达式和值
3. `create_function!` — 生成函数
4. `vec_of_squares!` — 生成平方 Vec
5. `my_vec!` — 自定义 vec 实现
6. `print_all!` — 批量打印
7. `count_items!` — 递归计数
8. `debug_struct!` — 调试结构体
9. 标准库宏：`format!`, `vec!`, `assert!`, `dbg!`, `matches!`
10. 函数 vs 宏对比表
11. 过程宏简介
12. 宏卫生演示

---

## 总结

### 核心要点

1. **宏是"写代码的代码"**，在编译期展开为 Rust 代码
2. **两种宏**：声明式宏 (`macro_rules!`) 和过程宏 (proc macros)
3. **声明式宏**通过模式匹配工作，适合大多数场景
4. **过程宏**更强大但需要独立 crate，适合复杂代码生成
5. **宏是卫生的**：内部变量不会污染外部命名空间
6. **能不用宏就不用**：优先使用函数、泛型和 trait
7. **`!` 后缀**是宏的视觉标记，区分于函数调用
8. **重复模式** `$(...)*` / `$(...)+` 是宏最核心的能力之一

### 学习路径建议

1. 先熟练使用标准库宏（`println!`, `vec!`, `dbg!`, `matches!` 等）
2. 尝试写简单的声明式宏（如 `hello!`, `say!`）
3. 掌握重复模式（如 `my_vec!`）
4. 阅读开源项目中的宏（如 `log` crate 的宏）
5. 学习过程宏（需要额外学习 `syn` 和 `quote` crate）

### 记住

> **宏的强大伴随责任。好的宏让人感觉不到它是宏——就像语言本身的特性一样自然。**

---

**核心术语索引**

| 中文 | English | 说明 |
|------|---------|------|
| 宏 | Macro | 元编程机制 |
| 声明式宏 | Declarative Macro | `macro_rules!` 定义的宏 |
| 过程宏 | Procedural Macro | 操作 TokenStream 的宏 |
| 派生宏 | Derive Macro | `#[derive(...)]` |
| 属性宏 | Attribute Macro | `#[attr]` 形式的过程宏 |
| 元编程 | Metaprogramming | 编写操作代码的代码 |
| 重复模式 | Repetition Pattern | `$(...)*` / `$(...)+` |
| 宏卫生 | Macro Hygiene | 宏内部变量不污染外部 |
| 片段说明符 | Fragment Specifier | `:expr`, `:ident`, `:ty` 等 |
| Token 流 | TokenStream | 过程宏的输入/输出类型 |
