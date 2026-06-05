# 闭包与迭代器 — 练习

## 练习说明

所有练习应在 `src/main.rs` 所在项目中完成。每道题后附有推荐命令和预期输出提示。

---

## Level 1：基础练习

### 练习 1-1：理解 iter / iter_mut / into_iter

**任务：** 补全以下代码，使所有测试通过。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // 使用 iter() 计算所有元素的平方和
        let sum_of_squares: i32 = todo!();
        assert_eq!(sum_of_squares, 55);
        // 证明原 vec 仍然可用
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_iter_mut() {
        let mut v = vec![10, 20, 30];
        // 使用 iter_mut() 将每个元素翻倍
        todo!();
        assert_eq!(v, vec![20, 40, 60]);
    }

    #[test]
    fn test_into_iter() {
        let v = vec![String::from("rust"), String::from("python")];
        // 使用 into_iter() 将每个字符串转为大写并收集
        let upper: Vec<String> = todo!();
        assert_eq!(upper, vec!["RUST", "PYTHON"]);
        // 注意：此处 v 已被移动，不能再使用
    }
}
```

**关键点：**
- `iter()` 返回 `&T`，不影响原集合
- `iter_mut()` 返回 `&mut T`，可以原地修改
- `into_iter()` 获取所有权，原集合被消费

**推荐命令：** `cargo test`

---

### 练习 1-2：闭包环境捕获

**任务：** 分析以下代码，回答每段代码中（1）闭包实现了哪些 trait（Fn/FnMut/FnOnce）；（2）在注释所在行是否能访问被捕获的变量。

```rust
// 代码段 A
let x = 42;
let f = || println!("{}", x);
f();
f();
println!("{}", x);  // (A1) 这里能访问 x 吗？为什么？

// 代码段 B
let mut y = 10;
let mut g = || {
    y += 1;
    println!("{}", y);
};
g();
g();
println!("{}", y);  // (B1) 这里能访问 y 吗？为什么？

// 代码段 C
let z = String::from("hello");
let h = || {
    let taken = z;  // z 被移动到闭包内
    println!("{}", taken);
};
h();
// println!("{}", z);  // (C1) 取消注释会发生什么？为什么？
// h();                 // (C2) 取消注释会发生什么？为什么？
```

**关键点：**
- 编译器根据闭包体对捕获变量的操作自动推导 trait
- 移动（move）消耗变量的所有权
- FnOnce 闭包只能调用一次

**推荐命令：** `cargo check` 测试每段代码的编译结果。

---

### 练习 1-3：迭代器适配器基本使用

**任务：** 用迭代器链（一行代码）完成以下转换：

```rust
let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// (a) 获取所有偶数的平方，取前3个
let a: Vec<i32> = todo!();
assert_eq!(a, vec![4, 16, 36]);

// (b) 跳过前5个，取剩余的3个
let b: Vec<i32> = todo!();
assert_eq!(b, vec![6, 7, 8]);

// (c) 将所有元素加倍后，找出第一个大于10的数
let c: Option<i32> = todo!();
assert_eq!(c, Some(12));
```

**关键点：**
- `filter` → `map` → `take` 是常用组合
- `skip` → `take` 可用于分页
- `find` 是短路消费器，找到即停止

**推荐命令：** `cargo test`

---

## Level 2：进阶练习

### 练习 2-1：实现自定义消费器 — median

**任务：** 实现一个函数，使用迭代器计算 `i32` 数组的中位数。

```rust
/// 计算中位数。如果数组为空返回 None。
/// 对于偶数个元素，返回中间两个元素的平均值（f64）。
fn median(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    // 提示：
    // 1. 用 data.iter().copied() 创建迭代器
    // 2. 用 collect::<Vec<i32>>() 收集并排序
    // 3. 根据长度的奇偶性计算中位数

    todo!()
}

#[test]
fn test_median() {
    assert_eq!(median(&[]), None);
    assert_eq!(median(&[5]), Some(5.0));
    assert_eq!(median(&[1, 2, 3, 4, 5]), Some(3.0));
    assert_eq!(median(&[1, 2, 3, 4]), Some(2.5));
    assert_eq!(median(&[3, 1, 4, 1, 5, 9]), Some(3.5));
}
```

**关键点：**
- `collect` + `sort` 是处理需要顺序的迭代器数据的常见模式
- 注意 `i32` 转 `f64` 的类型转换
- 处理边界条件（空输入）

**推荐命令：** `cargo test test_median`

---

### 练习 2-2：实现一个返回闭包的函数

**任务：** 实现以下三个返回闭包的函数，每个使用不同的闭包 trait。

```rust
/// 返回一个闭包，对输入加上固定的值。
/// 使用 Fn trait。
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    todo!()
}

/// 返回一个闭包，每次调用时依次从 Vec 中弹出元素。
/// 使用 FnMut trait（需要修改内部状态）。
fn make_popper(mut items: Vec<String>) -> impl FnMut() -> Option<String> {
    // 提示：items 需要在闭包中修改，考虑 move + 可变性
    todo!()
}

/// 消费一个字符串并返回一个闭包，该闭包只能被调用一次，
/// 调用时返回拥有所有权的字符串长度。
/// 使用 FnOnce trait。
fn make_length_getter(s: String) -> impl FnOnce() -> (String, usize) {
    todo!()
}

#[test]
fn test_adder() {
    let add_five = make_adder(5);
    assert_eq!(add_five(10), 15);
    assert_eq!(add_five(0), 5);
    // 可以多次调用
    assert_eq!(add_five(100), 105);
}

#[test]
fn test_popper() {
    let mut pop = make_popper(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]);
    assert_eq!(pop(), Some(String::from("c")));
    assert_eq!(pop(), Some(String::from("b")));
    assert_eq!(pop(), Some(String::from("a")));
    assert_eq!(pop(), None);
}

#[test]
fn test_length_getter() {
    let get = make_length_getter(String::from("hello rust"));
    let (s, len) = get();
    assert_eq!(s, "hello rust");
    assert_eq!(len, 10);
    // get();  // 不能第二次调用 — 它是 FnOnce
}
```

**关键点：**
- `Fn` 闭包可多次调用，不修改捕获的变量
- `FnMut` 闭包修改内部状态，需要 `mut` 声明
- `FnOnce` 闭包消费捕获的变量，只能调用一次
- `move` 关键字在返回闭包时几乎是必须的（需要转移所有权给返回的闭包）
- `impl Trait` 作为返回类型时，只能返回**一种**具体类型

**推荐命令：** `cargo test test_adder test_popper test_length_getter`

---

## Level 3：综合挑战

### 练习 3-1：CSV 解析器 — 迭代器管道实战

**任务：** 实现一个简化版 CSV 行解析器。给定一行 CSV 文本，用迭代器完成解析并进行分析。

```rust
/// 解析一行 CSV 数据，返回：
/// - 所有字段（以逗号分隔）
/// - 数值字段的总和（忽略非数字字段）
/// - 字段数
/// - 最长字段的长度
#[derive(Debug, PartialEq)]
struct CsvStats {
    fields: Vec<String>,
    numeric_sum: f64,
    field_count: usize,
    longest_field_len: usize,
}

/// 解析 CSV 行。要求：
/// 1. 使用 split + 迭代器适配器完成所有逻辑
/// 2. 不能使用显式 for 循环
/// 3. 数字解析失败的字段在求和时忽略（视为 0 贡献）
fn parse_csv_line(line: &str) -> CsvStats {
    let fields: Vec<String> = todo!(); // hint: split(',').map(|s| s.trim().to_string())

    let numeric_sum: f64 = todo!(); // hint: fields.iter().filter_map(...).sum()

    let field_count = todo!();
    let longest_field_len = todo!(); // hint: fields.iter().map(...).fold(...)

    CsvStats {
        fields,
        numeric_sum,
        field_count,
        longest_field_len,
    }
}

#[test]
fn test_csv_parse() {
    let result = parse_csv_line("apple, 42, banana, 3.14, cherry, 100");
    assert_eq!(result.field_count, 6);
    assert_eq!(result.longest_field_len, 6); // "banana"
    // 42 + 3.14 + 100 = 145.14
    assert!((result.numeric_sum - 145.14).abs() < 0.001);
}

#[test]
fn test_csv_empty_fields() {
    let result = parse_csv_line("1, , 2, , 3");
    assert_eq!(result.field_count, 5);
    // 空格和空字符串不能被解析为数字
    assert_eq!(result.numeric_sum, 6.0);
}

#[test]
fn test_csv_single_field() {
    let result = parse_csv_line("hello");
    assert_eq!(result.field_count, 1);
    assert_eq!(result.longest_field_len, 5);
    assert_eq!(result.numeric_sum, 0.0);
}
```

**实现提示：**

```rust
// 提示 1：解析数字可以用 filter_map
let sum: f64 = fields.iter()
    .filter_map(|f| f.parse::<f64>().ok())
    .sum();

// 提示 2：最长字段长度用 fold
let max_len = fields.iter()
    .map(|f| f.len())
    .fold(0, |max, len| if len > max { len } else { max });

// 提示 3：字段数直接用 len()
let field_count = fields.len();
```

**关键点：**
- `filter_map` 结合 `parse().ok()` 优雅处理解析失败
- `fold` 可以实现自定义的聚合逻辑
- 迭代器管道让数据流向一目了然
- 每种处理步骤职责单一

**推荐命令：** `cargo test test_csv_parse test_csv_empty_fields test_csv_single_field`

---

## 思考题

### 思考题 1：迭代器的"双重借用"问题

阅读以下代码并回答后续问题：

```rust
fn first_and_last_words(text: &str) -> Option<(&str, &str)> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let first = words.first()?;
    let last = words.last()?;
    Some((first, last))
}
```

**问题：**
1. 上述实现分配了一个 `Vec`。能否只用迭代器（不分配中间 Vec）来完成同样的功能？如果可以，写出代码。
2. 如果不能用单个迭代器链完成，解释原因。这里涉及 Rust 的什么限制？
3. `Iterator` trait 是否有 `first()` 和 `last()` 方法？它们和 `next()` 以及 `next_back()` 有什么关系？注意 `DoubleEndedIterator` trait。
4. 说说 `Option::?` 操作符在迭代器场景下的作用。

<details>
<summary>提示（点击展开）</summary>

```rust
fn first_and_last_words(text: &str) -> Option<(&str, &str)> {
    let mut words = text.split_whitespace();
    let first = words.next()?;
    let last = words.next_back()?;  // 需要 DoubleEndedIterator
    Some((first, last))
}
// 注意：如果只有一个单词，first 和 last 相同
```

`split_whitespace()` 返回的迭代器实现了 `DoubleEndedIterator`，可以同时从两端消费。

如果原迭代器不实现 `DoubleEndedIterator`，则需要先收集到 Vec，或者只取首尾（分别迭代到头得到 last）。

</details>

---

### 思考题 2：闭包 vs 函数指针

**问题：**
1. 函数指针 `fn(i32) -> i32` 和闭包 trait `Fn(i32) -> i32` 有什么区别？
2. 什么场景下应该使用函数指针而不是闭包 trait？
3. 闭包是否都能转换成函数指针？`\|x\| x + 1` 呢？`\|x\| x + captured_var` 呢？
4. 以下代码能编译吗？为什么？

```rust
let nums = [1, 2, 3];
let result: Vec<i32> = nums.iter().map(|&x| x * 2).collect(); // OK

fn double(x: &i32) -> i32 { x * 2 }
let result2: Vec<i32> = nums.iter().map(double).collect(); // ???

let factor = 3;
let result3: Vec<i32> = nums.iter().map(|&x| x * factor).collect(); // OK
// let result4: Vec<i32> = nums.iter().map(factor_times as fn(???) -> i32).collect(); // ???
```

<details>
<summary>提示（点击展开）</summary>

- 不捕获环境的闭包可以自动转换为函数指针 `fn`
- 捕获了环境的闭包**不能**转换为函数指针（因为包含了环境数据）
- `map(double)` 可以编译，因为 `double` 是函数指针，实现了 `FnMut`
- 捕获 `factor` 的闭包不能转为函数指针

</details>

---

## 练习检查清单

完成练习后，使用以下命令验证：

```bash
# 编译检查所有代码
cargo check

# 运行所有测试
cargo test

# 运行主程序查看输出
cargo run

# 检查是否有未使用的导入
cargo clippy -- -W clippy::all

# 查看代码行数统计
wc -l src/main.rs
```

## 预期学习成果

完成本章所有练习后，你应该能够：

- [ ] 区分 `Fn`、`FnMut`、`FnOnce` 并选择合适的 trait bound
- [ ] 理解闭包环境捕获的三种方式（引用、可变引用、所有权转移）
- [ ] 使用 `move` 关键字在异步/多线程场景中安全地移动数据
- [ ] 流畅使用 `iter()`、`iter_mut()`、`into_iter()` 并理解其所有权含义
- [ ] 用迭代器适配器（map、filter、enumerate、take、skip、chain、zip）构建数据处理管道
- [ ] 用消费器（collect、fold、find、sum、for_each、any、all）提取最终结果
- [ ] 理解惰性求值原理，知道中间分配在何时发生
- [ ] 在命令式循环与迭代器之间做出权衡，写出可读性最优的代码
- [ ] 用闭包返回值和 `impl Trait` 语法创建函数工厂
