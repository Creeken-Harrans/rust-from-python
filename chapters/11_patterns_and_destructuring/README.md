# 模式与解构 (Patterns and Destructuring)

> 核心术语：Pattern 模式, Destructuring 解构, Match Guard 匹配守卫, @ Binding, Refutable Pattern 可反驳模式, Irrefutable Pattern 不可反驳模式

## 什么是模式 (Pattern)

在 Rust 中，**模式 (Pattern)** 是一种特殊的语法，用于匹配值的结构。它不仅仅是简单的条件判断——模式可以将复合数据"拆开"，提取出内部字段并绑定到变量上。这是 Rust 最具表现力的特性之一。

一个模式由以下元素组合而成：

- 字面量 (Literals)：`42`, `'a'`, `"hello"`
- 变量绑定 (Variable bindings)：`x`, `name`
- 通配符 (Wildcards)：`_`
- 解构语法 (Destructuring)：`Point { x, y }`, `(a, b, c)`, `Some(v)`
- 范围 (Ranges)：`1..=5`, `'a'..='z'`
- 或模式 (Or patterns)：`A | B | C`
- @ 绑定 (@ bindings)：`id @ Some(..)`
- 匹配守卫 (Match guards)：`x if x > 0`

模式的根本目的是：**同时完成"判断值的形状"和"提取值的内部数据"两件事**。

## 模式的使用位置

Rust 中模式可以出现在多个位置，每个位置对模式的可反驳性（refutability）有不同要求。

### 1. match 表达式

`match` 是模式匹配最核心的使用场景。每个分支（arm）由一个模式和一个守卫（可选）组成：

```rust
match value {
    Pattern1 => expr1,
    Pattern2 if guard => expr2,
    _ => default_expr,  // 通配符捕获所有剩余情况
}
```

Rust 要求 `match` 是**穷尽的 (exhaustive)**——必须覆盖所有可能的值。编译器会检查这一点。

### 2. if let 表达式

`if let` 是 `match` 的语法糖，只处理一种匹配情况：

```rust
if let Some(x) = maybe_value {
    // 仅在 maybe_value 是 Some 时执行
    println!("值是: {x}");
} else {
    // 可选 else 分支，处理不匹配的情况
    println!("没有值");
}
```

支持 `else if let` 链式判断：

```rust
if let Event::KeyPress { key, ctrl: true } = &event {
    println!("Ctrl+{key}");
} else if let Event::KeyPress { key, .. } = &event {
    println!("普通按键 {key}");
} else {
    println!("其他事件");
}
```

### 3. while let 条件循环

`while let` 在模式持续匹配时重复执行循环体：

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("弹出: {top}");
}
// 输出: 弹出: 3, 弹出: 2, 弹出: 1
```

这在处理迭代器、流式数据、任务队列时非常有用。

### 4. let 语句

`let` 语句本身就是一个模式匹配——但它要求**不可反驳模式 (irrefutable pattern)**：

```rust
let (x, y) = (1, 2);           // 元组解构
let Point { x, y } = point;     // 结构体解构
let _ = some_expression();      // 忽略值
```

以下代码**无法编译**，因为 `Some(x)` 是可反驳模式：

```rust
// 错误！let 要求不可反驳模式
// let Some(x) = maybe_value;
```

### 5. 函数参数

函数参数也是模式——它们必须是不可反驳的：

```rust
fn print_point(Point { x, y }: Point) {
    println!("({x}, {y})");
}

fn sum((a, b): (i32, i32)) -> i32 {
    a + b
}
```

函数参数模式让代码更加简洁——直接在签名中解构，省去函数体内的赋值语句。

### 6. for 循环

`for` 循环的迭代变量位置也可以使用模式：

```rust
let pairs = vec![(1, "一"), (2, "二")];
for (i, name) in pairs {
    println!("{i}: {name}");
}

let map = HashMap::from([("key1", 10), ("key2", 20)]);
for (key, value) in &map {
    println!("{key} -> {value}");
}
```

## 解构 (Destructuring)

解构是模式最强大的能力——将复合数据拆解为组成部分。

### 元组解构

```rust
let pair = (42, "hello");
let (num, text) = pair;          // 基本解构

let nested = ((1, 2), (3, 4), 5);
let ((x1, y1), (x2, y2), z) = nested;  // 嵌套解构

let (first, .., last) = (1, 2, 3, 4, 5);  // 忽略中间元素
let (_, second, ..) = (1, 2, 3, 4, 5);    // 忽略第一个
```

### 结构体解构

```rust
struct Point { x: i32, y: i32 }
struct Config { host: String, port: u16, timeout: u32 }

// 基本解构
let Point { x, y } = point;

// 字段重命名
let Point { x: px, y: py } = point;

// 忽略其余字段
let Config { host, port, .. } = config;
```

### 枚举解构

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({x}, {y})"),
    Message::Write(text) => println!("文本: {text}"),
    Message::ChangeColor(r, g, b) => println!("颜色: ({r}, {g}, {b})"),
}
```

### 嵌套解构

可以**任意深度**地嵌套解构：

```rust
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

let rect = Rectangle {
    top_left: Point { x: 0, y: 0 },
    bottom_right: Point { x: 100, y: 80 },
};

// 嵌套解构：一层层拆开
let Rectangle {
    top_left: Point { x: x1, y: y1 },
    bottom_right: Point { x: x2, y: y2 },
} = rect;
```

## 引用模式 (ref, ref mut)

在模式匹配中，默认情况下变量绑定会**移动**值。如果需要在匹配的同时**借用**值，使用 `ref` 和 `ref mut`：

```rust
let opt = Some(String::from("hello"));

match opt {
    // ref s: 借用 String，而非移动它
    Some(ref s) => println!("借用: {s}"),
    None => (),
}
// opt 在此处仍然可用！

let mut counter = 0;
match counter {
    // ref mut c: 可变借用
    ref mut c => *c += 1,
}
```

**为什么需要 ref？** 因为 Rust 的所有权系统。如果不使用 `ref`，匹配的值会被移动到模式变量中，之后原值不再可用。`ref` 解决了"在匹配时既不移动值又能访问它"的问题。

在现代 Rust 中，更常见的做法是对整个 match 表达式取引用：

```rust
match &opt {
    Some(s) => println!("{s}"),  // s 自动是 &String
    None => (),
}
```

但 `ref` / `ref mut` 在需要**部分借用**（只借用结构体的某些字段）时仍然不可替代。

## 匹配守卫 (Match Guard)

**匹配守卫 (Match Guard)** 是在模式之后附加的 `if` 条件，提供比模式本身更精细的判断：

```rust
match event {
    Event::Scroll { delta } if delta > 0 => {
        println!("向上滚动");
    }
    Event::Scroll { delta } if delta < 0 => {
        println!("向下滚动");
    }
    Event::Scroll { delta } => {
        println!("无滚动");
    }
    _ => (),
}
```

匹配守卫可以访问模式中绑定的变量，这使得它比简单的字面量模式更灵活：

```rust
match number {
    n if n % 2 == 0 => println!("{n} 是偶数"),
    n if n % 2 != 0 => println!("{n} 是奇数"),
}

match pair {
    (x, y) if x == y => println!("对角线上的点"),
    (x, y) if x > y => println!("在右下方"),
    (x, y) => println!("在左上方"),
}
```

守卫与 `|` 模式的交互值得注意：守卫作用于整个 `|` 模式：

```rust
// 守卫 if y > 0 对 A(..) 和 B(..) 都生效
match val {
    A(x) | B(x) if x > 0 => { /* ... */ }
    _ => (),
}
```

## @ 绑定 (@ Binding)

**@ 绑定** 允许在解构的同时，将整个匹配的值绑定到一个变量：

```rust
match event {
    // size 绑定到整个 Event::Resize 值
    // width 和 height 绑定到内部字段
    size @ Event::Resize { width, height } => {
        println!("事件: {size:?}");        // 完整的枚举值
        println!("尺寸: {width}x{height}"); // 解构出的字段
    }
    _ => (),
}
```

@ 绑定在需要同时访问整体和部分时非常有用：

```rust
match range {
    r @ 1..=10 => println!("小范围: {r:?}"),
    r @ 11..=100 => println!("中范围: {r:?}"),
    r => println!("大范围: {r:?}"),
}

// 在枚举中
match result {
    ok @ Ok(_) => println!("成功结果: {ok:?}"),
    err @ Err(_) => println!("错误结果: {err:?}"),
}
```

## 或模式 (|)

**或模式 (Or Pattern)** 用 `|` 连接多个模式，任一匹配即进入该分支：

```rust
match key {
    'q' | 'Q' | '\x1b' => println!("退出"),
    'w' | 'W' | 'k'    => println!("向上"),
    's' | 'S' | 'j'    => println!("向下"),
    _ => println!("未绑定按键"),
}

// 与枚举结合
match event {
    Event::Quit | Event::KeyPress { key: 'q', .. } => {
        println!("程序退出");
    }
    _ => (),
}
```

或模式大大减少了重复代码——多个模式共享同一处理逻辑时无需重复编写。

## 不可反驳模式 vs 可反驳模式

这是理解 Rust 模式系统的一个关键概念。

### 不可反驳模式 (Irrefutable Pattern)

**不可反驳模式** 是无论任何值都一定匹配的模式。例如：

- 单个变量：`x`
- 元组解构（所有分量已知）：`(x, y)`
- 结构体解构：`Point { x, y }`
- 通配符：`_`

这些模式只用于**必须成功**的上下文：

| 位置 | 原因 |
|------|------|
| `let` 语句 | 变量绑定必须成功 |
| 函数参数 | 每次调用都必须接收参数 |
| `for` 循环 | 每次迭代都必须解构 |

### 可反驳模式 (Refutable Pattern)

**可反驳模式** 是可能匹配失败的模式。例如：

- `Some(x)` —— 如果是 `None` 就失败
- `Ok(v)` —— 如果是 `Err` 就失败
- `1..=10` —— 超出范围的数值会失败
- `Event::Click { .. }` —— 其他变体不匹配

这些模式只用于**允许失败**的上下文：

| 位置 | 原因 |
|------|------|
| `match` 分支 | 每个分支本身可能失败，整体穷尽即可 |
| `if let` | 明确设计为处理"可能不匹配"的情况 |
| `while let` | 循环在匹配失败时终止 |

### 编译器如何检查

Rust 编译器会检查模式的可反驳性：

```rust
// 编译错误：let 需要不可反驳模式
// let Some(x) = maybe_value;

// 编译警告：if let 中使用了不可反驳模式（永远匹配）
// if let x = 5 { /* 总是执行 */ }

// 正确用法
if let Some(x) = maybe_value { /* 仅在 Some 时执行 */ }
let (x, y) = (1, 2);  // 元组解构是不可反驳的
```

理解这个区别，就能理解为什么某些位置只能用某些模式——它是 Rust 类型安全和穷尽性检查的重要组成部分。

## Python 对比：Python 中的解构 vs Rust

Python 也支持解构，让我们对比一下两种语言。

### 元组/列表解构

Python:
```python
a, b = (1, 2)
first, *middle, last = [1, 2, 3, 4, 5]
(x1, y1), (x2, y2) = ((1, 2), (3, 4))
```

Rust:
```rust
let (a, b) = (1, 2);
let (first, .., last) = (1, 2, 3, 4, 5);  // 注意：Rust 用 .., Python 用 *
let ((x1, y1), (x2, y2)) = ((1, 2), (3, 4));
```

### 结构体/字典解构

Python 3.10+ (Structural Pattern Matching):
```python
match point:
    case Point(x=0, y=0):
        print("原点")
    case Point(x=x, y=y):
        print(f"({x}, {y})")
```

Rust:
```rust
match point {
    Point { x: 0, y: 0 } => println!("原点"),
    Point { x, y } => println!("({x}, {y})"),
}
```

### 关键差异

| 特性 | Python | Rust |
|------|--------|------|
| 穷尽性检查 | 无（可选分支） | 编译器强制执行 |
| 所有权语义 | 无 | 移动/借用/ref 区分 |
| 模式位置 | match/case, 赋值 | let, match, if let, while let, for, fn params |
| 类型级保证 | 运行时 | 编译时（部分运行时如范围） |
| @ 绑定 | 使用 `as` | 使用 `@` |
| 匹配守卫 | `case x if x > 0:` | `pattern if condition =>` |
| 或模式 | `case 1 \| 2:` | `1 | 2 =>` |

### 最大的区别：编译器参与度

Python 的 match/case 是纯粹的**运行时检查**——类型错误只能在程序运行时发现。Rust 的 match 是**编译时检查**——编译器会验证：

1. 是否穷尽了所有可能（exhaustiveness）
2. 变量的所有权是否被正确处理
3. 模式中引用的字段是否确实存在于类型中

这意味着重构 Rust 代码时，如果你给枚举添加了新变体，编译器会在所有 match 表达式处报错，引导你更新代码。这种"编译器驱动的重构"是 Rust 最令人安心的特性之一。

## 进阶主题

### 模式中的范围 (Range Patterns)

Rust 支持在模式中使用范围：

```rust
match x {
    1..=5  => println!("一到五"),
    6..=10 => println!("六到十"),
    _      => println!("其他"),
}

match c {
    'a'..='z' => println!("小写字母"),
    'A'..='Z' => println!("大写字母"),
    _         => println!("非字母"),
}
```

注意：`..=` 是闭区间（包含两端），`..` 是半开区间（不包含右端）。

### 切片模式 (Slice Patterns)

```rust
let arr = [1, 2, 3, 4, 5];
match arr {
    [first, .., last] => println!("首尾: {first}, {last}"),
    [single] => println!("单元素: {single}"),
    [] => println!("空切片"),
}

match arr {
    [1, rest @ ..] => println!("以1开头，其余: {rest:?}"),
    _ => (),
}
```

### 模式的组合使用

在实际项目中，这些模式特性经常组合使用：

```rust
match response {
    Ok(data @ Response { status: 200..=299, .. }) => {
        // 成功响应：既拿到完整值 data，又解构了 status
        cache.store(data);
        render(data);
    }
    Ok(Response { status: 301 | 302, headers, .. })
        if headers.contains_key("Location") =>
    {
        // 重定向：或模式 + 守卫
        follow_redirect(&headers["Location"]);
    }
    Ok(err @ Response { status, .. }) if status >= 400 => {
        // 错误响应：@ 绑定 + 守卫
        log_error(err);
    }
    Err(e) if e.kind() == ErrorKind::Timeout => {
        // IO 错误：守卫判断具体错误类型
        retry();
    }
    Err(e) => {
        panic!("意外错误: {e}");
    }
}
```

这个例子展示了 @ 绑定、或模式、匹配守卫和嵌套解构如何协同工作，让代码既简洁又表达力极强。

## 总结

Rust 的模式系统是其类型系统和所有权系统的自然延伸。理解模式，就能写出更简洁、更安全、更可维护的 Rust 代码：

- **模式 = 判断 + 提取**：同时完成条件判断和数据提取
- **穷尽性检查**：编译器确保你处理了所有情况
- **所有权感知**：ref/ref mut 让你在匹配时不丢失所有权
- **位置多样性**：同一个模式语法在 let/match/if let/for/fn params 中通用
- **组合能力**：解构、守卫、@ 绑定、或模式可以任意组合

对 Python 开发者而言，Rust 的模式系统可能一开始显得复杂，但一旦适应，你会发现自己再也离不开编译器的帮助——它在你编写代码时就告诉你遗漏了什么，而不是等到运行时崩溃。
