# 第 3 章：函数、表达式与控制流

## 本章目标

通过本章的学习，你将能够：

1. **定义和调用函数** —— 理解 Rust 中函数的完整语法，包括参数、返回值和文档注释。
2. **区分语句和表达式** —— 掌握 Rust 最核心的语言设计理念：几乎一切都是表达式。
3. **使用块表达式** —— 用 `{}` 包裹代码块并求值，这是 Rust 编程的日常操作。
4. **使用 `if` 作为表达式** —— 告别三元运算符，Rust 的 `if` 天生就能产生值。
5. **掌握三种循环** —— `loop`、`while`、`for` 各司其职，特别是 `loop` 的 `break` 可携带返回值。
6. **初步接触 `match`** —— Rust 最强大的模式匹配，初探其语法与穷尽性检查。
7. **对比 Python** —— 通过对照加深对 Rust 独特设计理念的理解。

---

## 为什么需要学习这一章

如果你从 Python 来，可能会觉得"函数和循环有什么好学的？"但 Rust 在这些基础元素上做了两项根本性改变：

### 1. 表达式优先（Expression-Oriented）

Python 中的 `if`、`loop` 是**语句**——它们执行操作但不产生值。你需要额外的变量赋值或三元表达式来获取结果：

```python
# Python: if 是语句，不能直接赋值
if mean > 50:
    category = "high"
else:
    category = "low"

# 或者用三元表达式（但它是另一个语法）
category = "high" if mean > 50 else "low"
```

Rust 的 `if` 本身就是表达式：

```rust
// Rust: if 是表达式，可以直接赋值
let category = if mean > 50.0 { "high" } else { "low" };
```

### 2. 分号改变语义

这是 Rust 新手最容易犯的错误。在 Rust 中：

- **无分号** = 表达式，产生值
- **有分号** = 语句，返回 `()`（unit 类型）

```rust
// 表达式：返回 42
fn answer() -> i32 {
    42
}

// 编译错误！分号把表达式变成了语句，返回 ()
fn broken() -> i32 {
    42;  // error[E0308]: mismatched types, expected `i32`, found `()`
}
```

### 3. 循环能返回值的唯一语言之一

```rust
let result = loop {
    count += 1;
    if count >= 10 {
        break count;  // break 可以返回值！
    }
};
// result == 10
```

这些特性结合起来，让 Rust 代码变得非常**紧凑而表达力强**，但对初学者来说也需要刻意练习。本章就是为此而设计。

---

## 背景知识

### 语句与表达式：一个古老的语言设计分歧

大多数编程语言区分"做事情的代码"（语句）和"产生值的代码"（表达式）：

| 语言 | 设计哲学 | `if` 性质 |
|------|----------|----------|
| C / Java / Python | 语句为主 | `if` 是语句 |
| Lisp / ML / Rust | 表达式为主 | `if` 是表达式 |

Rust 属于 ML 家族（Ocaml、Haskell、F#），其中几乎每个语法结构都是表达式。

### Rust 的函数设计

Rust 函数的核心特性：

- **显式类型标注**：参数和返回值都必须声明类型，没有类型推断的余地。
- **隐式返回**：函数体最后一个**表达式**（无分号）即返回值，不需要 `return` 关键字。
- **`return` 仍可用**：用于提前退出，但不应是最后一行的冗余写法。
- **文档注释**：`///` 开头的注释会被 `cargo doc` 收集生成 HTML 文档。

### 控制流的 Rust 哲学

- **`loop`**：当你不确定循环次数，或需要返回值的无限循环时使用。
- **`while`**：经典的前置条件循环。
- **`for`**：遍历迭代器的语法糖，Rust 没有 C 风格的三段式 `for(;;)`。
- **`match`**：穷尽性模式匹配，编译器强制处理所有可能情况。

---

## 核心术语中英对照

| 英文 | 中文 | 说明 |
|------|------|------|
| Function | 函数 | 命名的可重用代码块，接受参数并返回值 |
| Parameter | 参数 | 函数签名中声明的输入变量 |
| Argument | 实参 | 调用函数时传入的具体值 |
| Return Value | 返回值 | 函数执行完毕后产出的值 |
| Statement | 语句 | 执行操作但不产生值的代码；以分号结尾 |
| Expression | 表达式 | 求值后产生一个值的代码；通常不以分号结尾 |
| Block Expression | 代码块表达式 | `{ }` 包裹的代码序列，最后一个表达式的值为块的值 |
| Range | 范围 | `a..b`（左闭右开）或 `a..=b`（左闭右闭） |
| Pattern Matching | 模式匹配 | 将值与模式进行结构比对并解构 |
| Control Flow | 控制流 | 程序执行的顺序控制，包括分支和循环 |
| Arm (match arm) | 匹配分支 | `match` 中的每个 `pattern => expression` |
| Iteration | 迭代 | 通过循环依次访问集合中的每个元素 |
| Unit Type | 单元类型 | `()`，类似于 Python 的 `None` 但有类型 |
| Semicolon | 分号 | `;`，将表达式转为语句的关键语法标记 |

---

## 项目目录结构

```
03_functions_expressions_control_flow/
├── Cargo.toml        # 包配置文件
├── README.md         # 本章说明（本文件）
├── EXERCISES.md      # 练习题目
└── src/
    └── main.rs       # 统计计算器主程序（约 500 行）
```

---

## 运行命令

```bash
# 进入本章目录
cd chapters/03_functions_expressions_control_flow

# 编译并运行
cargo run

# 仅编译（检查类型和语法错误）
cargo check

# 生成并打开文档（查看 /// 文档注释的效果）
cargo doc --open

# 运行 clippy 静态检查
cargo clippy
```

---

## 预期输出

运行 `cargo run` 后，你将看到以下输出（按演示模块分组）：

```
╔══════════════════════════════════════════════╗
║   函数、表达式与控制流 —— 统计计算器        ║
║   Rust 从 Python 视角的学习之旅              ║
╚══════════════════════════════════════════════╝

📊 数据样本: [45, 22, 78, 34, 91, 12, 56, 83, 29, 67]
--- 统计结果 ---
  最小值: 12
  最大值: 91
  均值:   51.70
  总和:   517
  元素数: 10

===== 语句 (Statement) vs 表达式 (Expression) =====
表达式（无分号）: let value = 42;  → value = 42
分号把表达式变成语句: let unit = { 42; };  → unit = ()（即 unit 类型）
函数隐式返回（无分号）: returns_forty_two() = 42
⚠ 如果在返回位置加分号: `42;` → 返回 () 而非 i32 → 编译错误！

===== 块表达式 (Block Expression) =====
数据: [10, 20, 30, 40, 50]
块表达式计算的总和: 150
块表达式返回元组: count=5, total=150, avg=30.00

===== if 作为表达式 =====
均值 = 51.70
类别 = high（if 表达式返回值）
评级 = 中等（嵌套 if 表达式）
💡 Python 对照: category = "high" if mean > 50.0 else "low"

===== loop 循环 + break 携带返回值 =====
loop 通过 break 返回的值: 5
在数据中查找第一个 >= 50 的元素: index = 2
loop 计数到 10 后 break 返回: 10

===== while 循环 =====
逐步弹出元素: 67 29 83 56 12 91 34 78 22 45 
while let 弹出: 3 2 1 
💡 提示: while 不能像 loop 那样通过 break 返回值

===== for 循环与 Range =====
Range 0..=5 (含上界): 0 1 2 3 4 5 
Range 0..5  (不含上界): 0 1 2 3 4 
遍历 STATIC_DATA: 45 22 78 34 91 12 56 83 29 67 
带索引遍历:
  [0] = 45
  [1] = 22
  ...
💡 Python 对照: for i in range(6)  →  Rust: for i in 0..6

===== match 表达式 =====
match 分支结果 → 最小值: 12
match 分支结果 → 最大值: 91
match 分支结果 → 均值: 51.70
match 分支结果 → 总和: 517
match 分支结果 → 数据为空

均值 88.50 → 良好

===== 自定义类型与函数组合 =====
  上海 (人口 24870000) → 超大城市
  杭州 (人口 12200000) → 超大城市
  丽江 (人口 290000) → 中等城市

✅ 所有演示完成！
```

---

## 代码讲解

### 1. 整体程序结构

程序由以下部分组成：

```
main.rs
├── 常量定义：STATIC_DATA
├── 枚举定义：StatResult
├── 统计函数组
│   ├── calculate_min()    → Option<i32>
│   ├── calculate_max()    → Option<i32>
│   ├── calculate_mean()   → Option<f64>
│   └── calculate_sum()    → i32
├── 演示函数组
│   ├── demonstrate_statement_vs_expression()
│   ├── demonstrate_block_expression()
│   ├── demonstrate_if_expression()
│   ├── demonstrate_loop()
│   ├── demonstrate_while()
│   ├── demonstrate_for_and_range()
│   └── demonstrate_match()
├── 辅助类型与函数
│   ├── struct CityStats
│   └── fn classify_city()
└── main() 入口
```

每个函数都有 `///` 文档注释，可通过 `cargo doc` 生成 HTML 文档。

### 2. 函数定义

Rust 函数的基本语法：

```rust
/// 文档注释（可选，但强烈推荐用于公开 API）
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 函数体
    expr  // 最后一个表达式（无分号）= 返回值
}
```

关键规则：
- 参数名在前，类型在后（与 Python 的 type hints 顺序相反）
- 返回类型用 `-> Type` 标注
- 函数体是一个**块表达式**，最后一个无分号的表达式即为返回值
- 可以用 `return` 提前退出，但不用于最后一行的正常返回

本程序中的 `calculate_min` 示例：

```rust
fn calculate_min(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;         // 提前返回
    }
    let mut min = data[0];
    for &value in data.iter().skip(1) {
        if value < min {
            min = value;
        }
    }                           // for 循环内部是语句
    Some(min)                   // 最后一个表达式，无分号 → 返回值
}
```

### 3. 语句与表达式的核心区别

这是本章最重要的概念。Rust 中几乎所有东西都是表达式，只有少量纯粹的语句：

**语句（Statement）**:
- `let x = 5;` —— 变量绑定
- `fn foo() {}` —— 函数定义（模块级别）
- 任何以分号结尾的表达式都变成语句，返回值变为 `()`

**表达式（Expression）**:
- 字面量：`42`, `"hello"`
- 函数调用：`calculate_min(&data)`
- 块：`{ ... }`
- 控制流：`if`, `loop`, `while`, `for`, `match`
- 宏调用：`println!("hi")`

**分号是转换器**：分号将表达式变为语句。这是 Rust 中最微妙的语法规则：

```rust
let x = 5;     // 语句（let 绑定）
5              // 表达式（字面量）
5;             // 语句（表达式 + 分号）
{ 5 }          // 表达式（块表达式，值为 5）
{ 5; }         // 语句（块的最后一行有分号，返回 ()）
```

程序中演示了这一区别：

```rust
// 表达式（无分号），返回 42
let value = 42;

// 分号把表达式变成语句，返回 ()
let unit: () = {
    42;
};

// 函数体中：
fn returns_forty_two() -> i32 {
    42      // 无分号 → 这是返回值
}
// 如果写成 42; → 编译错误！类型不匹配
```

### 4. if 表达式

Rust 不需要像 Python 那样的三元运算符 `a if cond else b`，因为 `if` 本身就是表达式：

```rust
let category = if mean > 50.0 {
    "high"
} else {
    "low"
};
// category 的类型是 &str
```

**核心约束**：所有分支必须返回**相同类型**。以下代码无法编译：

```rust
// 编译错误！i32 和 &str 类型不同
let x = if condition { 42 } else { "hello" };
```

程序中还展示了嵌套 `if` 表达式，类似于 Python 的 `if/elif/else` 链：

```rust
let grade = if mean >= 80.0 {
    "优秀"
} else if mean >= 60.0 {
    "良好"
} else if mean >= 40.0 {
    "中等"
} else {
    "待提高"
};
```

**与 Python 对比**：

```python
# Python 需要两种语法
if mean > 50:
    category = "high"
else:
    category = "low"

# 或用三元表达式（完全不同的语法）
category = "high" if mean > 50 else "low"
```

```rust
// Rust 只有一种语法，if 天然支持
let category = if mean > 50.0 { "high" } else { "low" };
```

### 5. 循环的三种形式

Rust 提供三种循环，各有适用场景：

#### loop —— 无限循环

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count >= 10 {
        break count;  // break 携带返回值！
    }
};
// result == 10
```

`loop` 的独特能力：`break` 可以携带一个值，该值成为整个 `loop` 表达式的返回值。`while` 和 `for` 做不到这一点。

适用场景：需要持续运行直到某个复杂条件满足，并且需要从循环体中获取最终结果值。

#### while —— 条件循环

```rust
let mut remaining = vec![1, 2, 3];
while !remaining.is_empty() {
    if let Some(val) = remaining.pop() {
        println!("{}", val);
    }
}
```

与 Python 的 `while` 几乎一样：每次迭代前检查条件。注意 `while` 的 `break` 不能返回值（返回 `()`）。

程序中还引入了 `while let` 模式，这是 `while` 与模式匹配的组合语法糖：

```rust
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

#### for —— 迭代循环

Rust 没有 C 风格的三段式 `for (int i = 0; i < n; i++)`。Rust 的 `for` 本质上是迭代器的语法糖：

```rust
// Range 语法
for i in 0..5 { }       // 0, 1, 2, 3, 4  （不含上界）
for i in 0..=5 { }      // 0, 1, 2, 3, 4, 5（包含上界）

// 遍历集合
for &num in STATIC_DATA { }

// 带索引遍历
for (idx, &num) in STATIC_DATA.iter().enumerate() { }

// 步进
for i in (0..20).step_by(3) { }
```

**与 Python 对照**：

| Python | Rust |
|--------|------|
| `for i in range(n):` | `for i in 0..n` |
| `for i in range(start, end):` | `for i in start..end` |
| `for i, item in enumerate(lst):` | `for (i, &item) in lst.iter().enumerate()` |
| `for i in range(0, n, step):` | `for i in (0..n).step_by(step)` |

### 6. break 携带返回值

这是 Rust 的一个独特设计。`loop` 是唯一一种 `break` 可以携带返回值的循环：

```rust
let mut idx: usize = 0;
let found = loop {
    if idx >= data.len() {
        break -1_i32;     // 没找到，返回 -1（需显式类型标注或后缀）
    }
    if data[idx] >= threshold {
        break idx as i32; // 找到，返回索引
    }
    idx += 1;
};
```

这个特性让 `loop` 成为一种"可失败的搜索"语法——不需要额外的 `found` 标志变量。

### 7. match 初探

`match` 是 Rust 最强大的控制流构造。本章只做初步接触，第 9 章会深入。

基本语法：

```rust
match value {
    Pattern1 => expression1,
    Pattern2 => expression2,
    // 编译器强制要求覆盖所有可能情况！
}
```

程序中的例子：

```rust
enum StatResult {
    Min(i32),
    Max(i32),
    Mean(f64),
    Sum(i32),
    Empty,
}

let description = match result {
    StatResult::Min(v) => format!("最小值: {}", v),
    StatResult::Max(v) => format!("最大值: {}", v),
    StatResult::Mean(v) => format!("均值: {:.2}", v),
    StatResult::Sum(v) => format!("总和: {}", v),
    StatResult::Empty => "数据为空".to_string(),
};
```

关键特性：
- **穷尽性检查（Exhaustiveness Check）**：如果遗漏了某个变体，编译器会报错。这是 Rust 安全性的重要保障。
- **可解构**：`StatResult::Min(v)` 在匹配的同时提取了内部数据。
- **match 也是表达式**：每个分支返回的值成为 `match` 的整体返回值。

---

## 与 Python 的对照

### 1. if 作为表达式 vs Python 三元表达式

| 特性 | Rust | Python |
|------|------|--------|
| 赋值中使用条件 | `let x = if c { a } else { b };` | `x = a if c else b` |
| 多分支赋值 | `let x = if c1 { a } else if c2 { b } else { c };` | 需多层嵌套三元或独立 if |
| 类型约束 | 所有分支必须是同一类型 | 动态类型，无约束 |
| 语法一致性 | `if` 处处是表达式 | `if` 是语句，三元是特殊语法 |

### 2. Rust 代码块 vs Python 缩进

Python 用缩进定义代码块，Rust 用 `{}`。更重要的是语义差异：

**Python** 的块不产生值：
```python
result = {
    temp = 1 + 2    # SyntaxError! 不能在表达式上下文中使用语句
}
```

**Rust** 的块是表达式：
```rust
let result = {
    let temp = 1 + 2;    // 语句
    temp * 3             // 表达式 → 这是块的返回值
};
// result == 9
```

### 3. Rust for vs Python for

| Python | Rust | 说明 |
|--------|------|------|
| `for x in [1,2,3]:` | `for x in &[1,2,3]` | 直接遍历集合 |
| `for i in range(5):` | `for i in 0..5` | 数字范围 |
| `for i in range(1,6):` | `for i in 1..=5` | 包含上界 |
| `enumerate(seq)` | `.iter().enumerate()` | 带索引 |
| `range(0,10,2)` | `(0..10).step_by(2)` | 步进 |
| 没有三句式 for | N/A | Rust 没有 C 风格 for |
| N/A | `for item in collection` | Rust 默认是 move 迭代，需 `&` 来借用 |

### 4. 函数签名与类型约束

```python
# Python: 类型标注是可选的、运行时不检查
def calculate_mean(data: list[int]) -> float | None:
    if not data:
        return None
    return sum(data) / len(data)
```

```rust
// Rust: 类型是强制的、编译时检查
fn calculate_mean(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sum: i32 = data.iter().sum();
    Some(sum as f64 / data.len() as f64)
}
```

关键差异：
- Python 的类型标注是**建议性的**，Rust 的类型是**强制性的**。
- Rust 的 `Option<f64>` 在类型层面区分"有值"和"无值"，Python 用 `None`(动态类型)。
- Rust 需要显式的类型转换（`as f64`），Python 的数值类型会自动提升。

### 5. 返回值

```python
# Python: 必须显式 return，否则隐式返回 None
def add(a, b):
    return a + b

def oops(a, b):
    a + b  # 忘记 return → 返回 None
```

```rust
// Rust: 最后一个表达式自动成为返回值
fn add(a: i32, b: i32) -> i32 {
    a + b  // 无分号 → 自动返回
}

// 如果加了分号 → 编译错误！
fn oops(a: i32, b: i32) -> i32 {
    a + b; // 分号 → 返回 () → 类型不匹配
}
```

---

## Python、C 与 C++ 对照

在控制流和表达式的设计上，Rust 与 C/C++ 家族走了截然不同的路。以下几组对比揭示了最关键的差异。

### 1. 表达式优先 vs 语句优先

C/C++ 是"语句优先"的语言。`if`、`while`、`for` 都是语句——它们执行逻辑，但不产生值。当你需要条件性返回值时，必须借助三元运算符或额外变量：

```c
// C/C++：if 是语句，不能直接赋值
const char* category;
if (mean > 50.0) {
    category = "high";
} else {
    category = "low";
}
// 或用三元运算符——那是另一套语法
const char* category = (mean > 50.0) ? "high" : "low";
```

Rust 是"表达式优先"的语言。上述所有控制流构造本身就是表达式，直接产生值：

```rust
let category = if mean > 50.0 { "high" } else { "low" };
let result = loop { break 42; };  // loop 也是表达式
let desc = match x { 1 => "one", _ => "other" };  // match 也是表达式
```

这意味着 Rust 不需要三元运算符——`if` 本身就能胜任。这种一致性减少了语法概念的数量，也鼓励了更紧凑、更少可变中间变量的代码风格。

### 2. 分号改变语义

在 C/C++ 中，分号是语句的终止符——每条语句都以分号结束，不加分号就是语法错误。分号不改变"值"的语义。

在 Rust 中，分号扮演着一个微妙却至关重要的角色：**它将表达式转换为语句**。一个不加分号的表达式产生它的值，加上分号后变为返回 `()`（unit 类型）的语句：

```rust
// 无分号：块的值是 42
let x = { 42 };

// 有分号：块的值是 ()
let y = { 42; };  // y 的类型是 ()，不是 i32！
```

这个差异直接影响了函数返回值。在 C/C++ 中，你需要显式 `return`；在 Rust 中，函数体最后一个表达式的值（不加分号）自动成为返回值。新手最常见的编译错误就是在返回位置多打了一个分号——这会让编译器看到 `()` 而不是期望的类型。

### 3. `switch` vs `match` 的设计差异（预览）

C/C++ 的 `switch` 和 Rust 的 `match` 虽然都做多路分支，但在设计哲学上相差甚远：

**C/C++ `switch` 的已知问题**：

```c
switch (value) {
    case 1:
        printf("one");  // 忘记 break！会落入下一个 case
    case 2:
        printf(" or two");
        break;
    default:
        printf("something else");
}
```

`switch` 的"贯穿"（fall-through）语义是 C 历史上无数 bug 的来源。编译器不会检查你是否遗漏了某个 `case`，也不会强制你写 `default`。它本质上只是一个跳转表上的语法糖。

**Rust `match` 的设计选择**：

```rust
match value {
    1 => println!("one"),       // 不存在 fall-through
    2 => println!("two"),
    _ => println!("something else"),  // 编译器强制覆盖所有情况
}
```

Rust 的 `match` 做了三个根本性改进：
- **无贯穿**：每个分支独立，不需要 `break`
- **穷尽性检查**：编译器强制你处理所有可能的值，漏掉任何一个都是编译错误
- **表达式求值**：每个分支可以返回一个值，整个 `match` 表达式的结果就是匹配分支的值

第 9 章将详细展开 `match` 的模式匹配能力——包括解构结构体、枚举、范围匹配和守卫条件。眼下你只需知道：Rust 的 `match` 不仅仅是 C `switch` 的升级版，它是一门控制流的"核武器"。

## 常见错误

### 错误 1: 在返回位置加分号

这是 Rust 新手最高频的错误。

```rust
// ❌ 错误
fn sum(a: i32, b: i32) -> i32 {
    a + b;  // 分号让表达式变成语句 → 返回 () → 类型不匹配
}

// ✅ 正确
fn sum(a: i32, b: i32) -> i32 {
    a + b   // 无分号 → 表达式 → 正常返回
}
```

编译器错误信息：
```
error[E0308]: mismatched types
 expected `i32`, found `()`
```

**解决办法**：删除最后一个表达式的分号。

**Python 对照**：Python 没有这个问题，因为必须用 `return` 关键字。Rust 的隐式返回是一种权衡：更简洁但有分号陷阱。

### 错误 2: if 表达式分支类型不匹配

```rust
// ❌ 错误
let x = if true {
    42        // i32
} else {
    "hello"   // &str → 类型不匹配！
};

// ✅ 正确
let x = if true {
    42
} else {
    0        // 都是 i32
};
```

编译器错误信息：
```
error[E0308]: `if` and `else` have incompatible types
```

**解决办法**：确保所有分支返回相同类型。如果需要不同行为，考虑使用 `match` 或重构逻辑。

### 错误 3: loop vs while 的选择困惑

```rust
// 场景：需要 break 返回值
// ❌ while 不能返回值
let result = while condition {  // 编译错误
    if done { break value; }
};

// ✅ 用 loop
let result = loop {
    if !condition { break default_value; }
    if done { break value; }
};
```

**原则**：
- 需要 break 返回值 → 用 `loop`
- 有明确的前置条件 → 用 `while`
- 遍历集合或范围 → 用 `for`

### 错误 4: 忘记 for 循环中 `&` 的作用

```rust
let data = vec![1, 2, 3];

// for item in data —— 这会消耗 data（move），之后 data 不可用
// for &item in &data —— 借用 data，之后 data 仍可用
for &item in &data {
    println!("{}", item);
}
// data 仍然可用！
```

这个问题涉及所有权，在后续章节会深入讲解。

### 错误 5: match 分支不完整

```rust
enum Color { Red, Green, Blue }

// ❌ 编译错误：遗漏了 Blue
let name = match color {
    Color::Red => "红",
    Color::Green => "绿",
};

// ✅ 完整覆盖
let name = match color {
    Color::Red => "红",
    Color::Green => "绿",
    Color::Blue => "蓝",
};

// ✅ 或用通配符
let name = match color {
    Color::Red => "红",
    _ => "其他颜色",  // 覆盖所有剩余情况
};
```

编译器错误信息：
```
error[E0004]: non-exhaustive patterns: `Blue` not covered
```

Rust 的穷尽性检查是你安全网的一部分——它迫使你在添加新枚举变体时考虑到所有使用 `match` 的地方。

---

## 练习建议

1. **先运行再看代码**：运行 `cargo run`，观察输出，对每个模块的功能有大体概念。
2. **逐个函数阅读**：从 `main()` 开始，追踪每个被调用的函数，理解数据流。
3. **修改实验**：
   - 修改 `STATIC_DATA` 的内容，观察统计结果的变化。
   - 在 `demonstrate_if_expression` 中修改均值阈值，观察分类变化。
   - 尝试在 `demonstrate_loop` 中修改搜索阈值。
4. **故意制造错误**：
   - 在 `calculate_min` 最后一个 `Some(min)` 后加分号，观察编译器报错。
   - 在 `if` 表达式的两个分支中返回不同类型，观察错误信息。
   - 在 `match` 中删除一个分支，观察穷尽性检查报错。
5. **做 EXERCISES.md 中的练习题**：从简单的填空题到设计题，循序渐进。
6. **运行 `cargo doc --open`**：查看文档注释生成的 HTML 页面，理解 `///` 的作用。

---

## 本章小结

本章覆盖了 Rust 编程中最基础也最独特的三个概念：

1. **函数** —— 带显式类型的参数和返回值，隐式返回最后一个表达式的值。文档注释 `///` 是惯用做法。

2. **语句 vs 表达式** —— Rust 是表达式优先的语言。除 `let` 绑定和函数/模块定义外，几乎所有语法结构都是表达式。分号 `;` 是表达式到语句的转换器，理解了它就能避免大量编译错误。

3. **控制流** —— 三种循环（`loop` 含 break 返回值、`while` 条件循环、`for` 迭代循环）各有适用场景。`if` 和 `match` 都是表达式，可以直接用于 `let` 绑定。`match` 的穷尽性检查是编译器提供的安全保障。

### 核心要点速查

| 概念 | 关键语法 | 记住 |
|------|----------|------|
| 函数 | `fn name(x: T) -> R { }` | 类型在参数名`:`后面 |
| 返回值 | 最后一个表达式无分号 | 分号=语句，无分号=表达式 |
| 块表达式 | `let x = { ...; val };` | 最后一行是返回值 |
| if 表达式 | `let x = if c { a } else { b };` | 所有分支同类型 |
| loop | `break value;` | 唯一能返回值的循环 |
| for + Range | `for i in 0..n` | 不含上界；`0..=n`含上界 |
| match | `match x { Pat => expr, }` | 必须穷尽所有可能 |

---

## 下一章衔接

第 4 章「**栈、堆与 RAII**」将进入 Rust 内存管理的基础知识：

- 理解两种内存区域：栈（Stack）和堆（Heap）
- 了解 Rust 如何在编译期决定数据存放位置
- RAII（资源获取即初始化）—— Rust 不需要 GC 的原因
- `Drop` trait —— Rust 的"析构函数"

本章学到的：
- 函数定义语法 → 第 4 章中定义带 `Drop` 实现的类型
- 语句与表达式 → 贯穿 Rust 编程的每一行代码
- `match` 表达式 → 第 5 章中匹配 `Option` 和 `Result`
- `for` 循环 → 第 10 章中深度使用迭代器

建议在进入下一章之前，完成 EXERCISES.md 中的 Level 1 和 Level 2 练习题。

---

*"Rust 的表达式优先设计不是语法糖——它是一种编程思维方式的改变。当你习惯了'一切皆表达式'，你会发现代码变得更紧凑、更安全、更易读。"*
