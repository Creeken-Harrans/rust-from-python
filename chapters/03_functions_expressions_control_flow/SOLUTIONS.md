# 参考答案

建议先独立完成练习，再阅读本文件。参考答案用于比较思路，不是用来复制的。

---

## Level 1：基础巩固

### 练习 1.1：补全函数 —— 计算中位数

#### 结论

中位数计算要点：先排序，再根据长度奇偶分情况处理。

#### 思路

1. 将切片复制为可变 `Vec` 并排序
2. 空切片直接返回 `None`
3. 奇数长度取中间元素；偶数长度取中间两个元素的平均值

#### 参考实现

```rust
fn calculate_median(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mut sorted = data.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        // 奇数：取正中间
        Some(sorted[mid] as f64)
    } else {
        // 偶数：中间两个的平均
        Some((sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0)
    }
}
```

#### 常见错误

- 用 `sorted[mid]` 而非 `sorted[mid - 1]` 处理偶数情况
- 忘记空切片返回 `None`
- 直接在原数据上调用 `.sort()` 但 `&[i32]` 不可变

#### 验证方式

```rust
assert_eq!(calculate_median(&[]), None);
assert_eq!(calculate_median(&[5]), Some(5.0));
assert_eq!(calculate_median(&[1, 3, 5]), Some(3.0));
assert_eq!(calculate_median(&[1, 2, 3, 4]), Some(2.5));
```

---

### 练习 1.2：if 表达式改写

#### 结论

Rust 的 `if` 是表达式，可直接作为 `let` 绑定的初始值。

#### 参考实现

```rust
fn grade(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}

fn main() {
    for s in [95, 82, 71, 55] {
        println!("score {} → grade {}", s, grade(s));
    }
}
```

#### 为什么这样设计

- 每个分支隐式返回，不需要 `return` 关键字
- 所有分支必须返回同一类型（这里都是 `&str`）
- 与 Python 三目 `a if cond else b` 不同 —— Rust `if` 本身就是表达式，不需额外三目

#### 常见错误

- 分支间类型不一致（一个 `&str` 一个 `String`）
- 在最后一个分支添分号（变成语句，返回 `()`）

---

### 练习 1.3：for 循环与 Range 练习

#### 参考实现

```rust
// 1. 乘法表
println!("12x12 乘法表:");
for i in 1..=12 {
    for j in 1..=12 {
        print!("{:4}", i * j);
    }
    println!();
}

// 2. 0-100 之间被 7 整除的数
print!("0-100 被 7 整除: ");
for n in (0..=100).step_by(7) {
    print!("{} ", n);
}
println!();

// 3. 带标注的遍历
for (idx, &val) in STATIC_DATA.iter().enumerate() {
    let label = if val < 30 { " (low)" } else { "" };
    println!("  [{}] = {}{}", idx, val, label);
}
```

#### 常见错误

- `step_by(0)` → panic
- `..=` 与 `..` 混淆（上界是否包含）
- 遍历时忘记 `&` 导致类型不匹配

---

## Level 2：应用实践

### 练习 2.1：实现方差计算函数

#### 参考实现

```rust
fn calculate_variance(data: &[i32]) -> Option<f64> {
    let mean = calculate_mean(data)?;  // ? 在 Option 上：None 时提前返回
    let sum_sq: f64 = data.iter()
        .map(|&x| (x as f64 - mean).powi(2))
        .sum();
    Some(sum_sq / data.len() as f64)
}

fn calculate_std_dev(data: &[i32]) -> Option<f64> {
    calculate_variance(data).map(|v| v.sqrt())
}
```

#### 验证方式

```rust
// 数据集 [45, 22, 78, 34, 91, 12, 56, 83, 29, 67]
// 均值 ≈ 51.7, 方差 ≈ 689, 标准差 ≈ 26.2
println!("方差: {:?}", calculate_variance(STATIC_DATA));
println!("标准差: {:?}", calculate_std_dev(STATIC_DATA));
```

---

### 练习 2.2：用 loop 实现猜数字游戏骨架

#### 参考实现

```rust
fn guess_game(secret: u32, guesses: &[u32]) -> (u32, bool) {
    let mut attempts: u32 = 0;
    loop {
        if attempts as usize >= guesses.len() {
            break (attempts, false); // 所有猜测用完，未中
        }
        let guess = guesses[attempts as usize];
        attempts += 1;
        if guess == secret {
            break (attempts, true);
        }
    }
}

// 测试
fn main() {
    let cases = [
        (42, &[10u32, 20, 42, 30][..], "应该猜中"),
        (99, &[10, 20, 30], "应该未猜中"),
    ];
    for (secret, guesses, desc) in cases {
        let (tries, hit) = guess_game(secret, guesses);
        println!("秘密={}, 尝试{}次, 命中={}", secret, tries, hit);
    }
}
```

#### 常见错误

- 忘记检查数组越界 → 改 `while` 或加边界检查
- `break (attempts, false)` 写成 `break attempts, false`（无括号的 break 只接受一个值）

---

## Level 3：设计思考

### 练习 3.1：实现迷你数据分析器

#### 思路

设计要点：将统计结果汇总到 `struct DataSummary`，使用已有函数计算各项指标。

#### 参考实现

```rust
#[derive(Debug)]
struct DataSummary {
    count: usize,
    min: Option<i32>,
    max: Option<i32>,
    mean: Option<f64>,
    median: Option<f64>,
    variance: Option<f64>,
    std_dev: Option<f64>,
}

fn analyze(data: &[i32]) -> DataSummary {
    DataSummary {
        count: data.len(),
        min: calculate_min(data),
        max: calculate_max(data),
        mean: calculate_mean(data),
        median: calculate_median(data),
        variance: calculate_variance(data),
        std_dev: calculate_std_dev(data),
    }
}
```

#### 为什么这样设计

- 用 `Option` 表达空数据集无法计算的指标
- `count` 始终有效（非空为 0）
- 复用已有函数而非重写，体现组合

#### 验证方式

```rust
let summary = analyze(STATIC_DATA);
println!("{:#?}", summary);
assert_eq!(summary.count, STATIC_DATA.len());
```

---

## 思考题

### Q1：为什么 Rust 选择"表达式优先"而非"语句优先"？

**参考分析**：

Rust 中几乎一切都是表达式（`if`、`loop`、块、`match`），这使得：
1. **赋值更简洁**：`let x = if cond { a } else { b };` 无需三目运算符
2. **错误更少**：不存在"忘记 return"的 bug —— 函数体就是表达式
3. **模式匹配一致**：`match` 和 `if let` 都返回值的表达式

Python 的 `if`/`for` 是语句（3.8+ 有赋值表达式 `:=`），Java/C 传统上也以语句为主。函数式语言（Haskell、OCaml）和 Rust 共享此设计偏好。

代价：初学者需要理解分号改变语义（表达式末尾加分号变成语句）。

### Q2：什么时候用 `loop`，什么时候用 `while`？

| 场景 | 推荐 | 理由 |
|------|------|------|
| 需要 `break` 返回值 | `loop` | `while` 的 `break` 不返回值 |
| 条件明确 | `while` | 更简洁 |
| 无限循环（如服务器） | `loop` | 语义清晰 |
| 不确定迭代次数 | `loop` | 配合 `break` 灵活控制 |

### Q3：`match` 穷尽性检查的实际工程价值？

在重构时价值最大。假设你有一个 `HttpStatus` 枚举，分布在 30 个文件中被 `match`。新增变体 `HttpStatus::TooManyRequests` 后，编译器在所有 `match` 处报告缺失分支——完成了一次零遗漏的影响范围分析。

---

*练习的目的不是一次做对，而是通过反复尝试和修正，让 Rust 的表达式思维成为你的第二天性。*
