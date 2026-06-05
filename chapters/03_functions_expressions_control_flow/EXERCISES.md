# 第 3 章 练习：函数、表达式与控制流

## 如何使用本章练习

- 将答案代码放在 `src/main.rs` 中（覆盖或新建函数皆可）。
- 运行 `cargo run` 验证结果。
- 运行 `cargo clippy` 检查代码风格。
- 鼓励先手写思考，再用编译器验证。

---

## Level 1：基础巩固

### 练习 1.1：补全函数 —— 计算中位数

在 `src/main.rs` 中添加以下函数，补全注释缺失的部分。

要求：
- 对切片进行排序（使用 `to_vec()` 创建可变副本然后 `.sort()`）
- 根据元素个数的奇偶计算中位数
- 空切片返回 `None`

```rust
/// 计算数据的中位数。
///
/// 中位数定义：排序后位于中间位置的值。
/// - 奇数个元素：取正中间的元素
/// - 偶数个元素：取中间两个元素的平均值
fn calculate_median(data: &[i32]) -> Option<f64> {
    // TODO: 在此补全实现
    None
}
```

提示：
- `let mut sorted = data.to_vec();`
- `sorted.sort();`
- `let mid = sorted.len() / 2;`
- 用 `if` 表达式判断奇偶

### 练习 1.2：if 表达式改写

下面的 Python 代码用多层 `if/elif` 对成绩分段。将其改写为 Rust 的 `if` 表达式（作为 `let` 绑定的初始值）：

```python
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
```

在 `main()` 中调用并打印几个测试分数（如 95, 82, 71, 55）的结果。

### 练习 1.3：for 循环与 Range 练习

在 `main()` 中添加以下逻辑：

1. 用 `for` 循环和 `..=` 打印 1 到 12 的乘法表（12x12）。
2. 用 `for` 循环和 `.step_by()` 打印 0 到 100 之间所有能被 7 整除的数。
3. 用 `for` 循环和 `.enumerate()` 遍历 `STATIC_DATA`，打印每个元素及其索引，但对小于 30 的元素标注 "(low)"。

---

## Level 2：应用实践

### 练习 2.1：实现方差计算函数

标准差是均值的"离散度量"。实现以下两个函数：

```rust
/// 计算总体方差: Σ(xi - μ)² / N
fn calculate_variance(data: &[i32]) -> Option<f64> {
    // 需要先计算 mean，然后计算每个元素与均值的差的平方和
    // 使用 for 循环
    // TODO
    None
}

/// 计算总体标准差: sqrt(variance)
fn calculate_std_dev(data: &[i32]) -> Option<f64> {
    // 调用 calculate_variance，对结果调用 .sqrt()
    // TODO
    None
}
```

提示：
- `calculate_mean` 已经实现，可以直接调用。
- `(x as f64 - mean).powi(2)` 计算差的平方。
- `f64::sqrt(v)` 计算平方根。
- 用 `for &x in data` 遍历。

在 `main()` 中打印方差和标准差。

### 练习 2.2：用 loop 实现猜数字游戏的骨架

模拟一个猜数字游戏（不需要用户输入，用硬编码的序列）。

```rust
/// 模拟猜数字逻辑（不依赖用户输入）。
///
/// secret 是目标数字，guesses 是猜测序列。
/// 使用 loop 遍历猜测，break 时返回 (尝试次数, 是否猜中)。
fn guess_game(secret: u32, guesses: &[u32]) -> (u32, bool) {
    // TODO: 用 loop 实现
    // 返回 (猜测次数, 是否猜中)
    // 如果猜中，break 携带找到时的信息
    // 如果没猜中任何一次，break 携带失败信息
    (0, false)
}
```

在 `main()` 中测试：
- `secret = 42, guesses = [10, 30, 42, 80]` → 应该在第 3 次猜中
- `secret = 42, guesses = [10, 30, 80]` → 3 次都没猜中

---

## Level 3：综合挑战

### 练习 3.1：实现一个迷你数据分析器

创建一个枚举 `DataSummary` 和函数 `analyze_data`，综合运用本章所有知识点：

```rust
/// 数据分析结果枚举。
enum DataSummary {
    Stats {
        count: usize,
        min: i32,
        max: i32,
        mean: f64,
        median: f64,
        std_dev: f64,
    },
    InsufficientData {
        reason: &'static str,
    },
}

/// 分析数据并返回摘要。
///
/// 要求：
/// 1. 数据为空或只有一个元素 → 返回 InsufficientData
/// 2. 否则返回完整的 Stats 摘要
/// 3. 在函数内部使用块表达式计算某些中间值
/// 4. 使用 match 在调用处打印不同格式的结果
fn analyze_data(data: &[i32]) -> DataSummary {
    // TODO
    DataSummary::InsufficientData { reason: "未实现" }
}
```

在 `main()` 中：
1. 对 `STATIC_DATA` 调用 `analyze_data`。
2. 使用 `match` 解构返回的 `DataSummary`。
3. 如果是 `Stats`，用美观的格式打印所有统计指标。
4. 如果是 `InsufficientData`，打印原因。

---

## 思考题

### 为什么 Rust 选择"表达式优先"的设计？

Rust 的设计受到了 ML 家族语言（Ocaml、Haskell）的深刻影响。请思考并回答：

1. **代码简洁性**：表达式优先如何减少样板代码？从本章的 `if` 表达式和 `loop` 返回值中举例说明。
2. **类型安全**：当 `if` 是表达式时，编译器如何利用分支的类型约束来帮助你发现 bug？
3. **与所有权系统的协同**：Rust 的所有权系统要求精确跟踪值的生命周期。表达式优先的设计如何与这个目标协同？（提示：考虑块表达式和变量作用域）
4. **Python 为什么没有这样做？** Python 的设计哲学是"显式优于隐式"，这与 Rust 的隐式返回形成了有趣的对比。讨论这种权衡。

将你的思考写在 `main.rs` 的注释中（在文件末尾添加一个 `// 思考题答案:` 部分），或者写在单独的笔记中。

---

## 推荐命令

```bash
# 编译检查（快速反馈）
cargo check

# 运行程序
cargo run

# 代码风格检查
cargo clippy

# 自动格式化
cargo fmt

# 生成文档（查看你的 /// 注释效果）
cargo doc --open

# 运行测试（如果你添加了 #[test] 函数）
cargo test
```

### 快速验证技巧

对于简单函数，你可以在 `main()` 底部添加断言来快速验证：

```rust
fn main() {
    // ... 已有的演示代码 ...

    // 快速验证练习 1.1
    let test_odd = &[3, 1, 2];         // 中位数应该是 2
    let test_even = &[4, 1, 3, 2];     // 中位数应该是 2.5
    println!("练习 1.1: odd median = {:?}", calculate_median(test_odd));
    println!("练习 1.1: even median = {:?}", calculate_median(test_even));
}
```

### 调试技巧

当遇到编译错误时，从错误信息的**第一个错误**开始解决——后续错误往往是第一个错误的连锁反应。

常见错误速查：

| 错误代码 | 常见原因 | 解决 |
|----------|----------|------|
| E0308 | 类型不匹配（分号、if 分支类型不同）| 去掉分号或统一类型 |
| E0004 | match 分支不全 | 补全所有变体或加 `_` 通配 |
| E0382 | 使用了已移动的值（for 循环消耗）| 用 `&` 借用 |
| E0599 | 方法不存在 | 检查导入或类型是否正确 |

---

*练习的目的不是一次做对，而是通过反复尝试和修正，让 Rust 的表达式思维成为你的第二天性。*
