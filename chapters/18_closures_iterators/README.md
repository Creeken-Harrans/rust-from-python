# 闭包与迭代器 — Rust 的函数式编程工具

## 目录

1. [什么是闭包（Closure）](#什么是闭包closure)
2. [闭包的三种 trait：Fn、FnMut、FnOnce](#闭包的三种-traitfnfnmutfnonce)
3. [环境捕获](#环境捕获)
4. [move 关键字](#move-关键字)
5. [闭包与泛型](#闭包与泛型)
6. [什么是迭代器（Iterator）](#什么是迭代器iterator)
7. [iter() vs iter_mut() vs into_iter()](#iter-vs-iter_mut-vs-into_iter)
8. [迭代器适配器（Adapter）](#迭代器适配器adapter)
9. [迭代器消费器（Consumer）](#迭代器消费器consumer)
10. [惰性求值（Lazy Evaluation）](#惰性求值lazy-evaluation)
11. [性能：零成本抽象](#性能零成本抽象)
12. [何时使用迭代器 vs 循环](#何时使用迭代器-vs-循环)
13. [Python 对照](#python-对照)
14. [核心术语速查](#核心术语速查)

---

## 什么是闭包（Closure）

闭包是一种**匿名函数**，它可以捕获其所在作用域中的变量。在 Rust 中，闭包的语法非常简洁：

```rust
// 闭包定义语法：|参数| 表达式
let add_one = |x: i32| x + 1;
let result = add_one(5); // 6

// 多参数闭包
let multiply = |a, b| a * b;

// 多行闭包体
let complex = |x| {
    let y = x * 2;
    y + 1
};
```

### 闭包 vs 函数

| 特性 | 函数 `fn` | 闭包 |
|------|-----------|------|
| 名称 | 必须有名称 | 可以匿名 |
| 类型声明 | 必须声明参数和返回类型 | 通常可以推断 |
| 捕获环境 | 不能 | 可以 |
| 作为参数 | `fn(T) -> U` | `Fn(T) -> U` trait |
| 递归 | 支持 | 不支持（需特殊技巧） |

闭包最强大的特性是**环境捕获**——它可以访问定义处作用域中的变量，这是普通函数做不到的：

```rust
let prefix = "Hello";
// 闭包捕获了 prefix
let greet = |name| format!("{}, {}!", prefix, name);
// 普通函数做不到这一点：
// fn greet(name: &str) -> String { format!("{}, {}!", prefix, name) } // 编译错误！
```

---

## 闭包的三种 trait：Fn、FnMut、FnOnce

Rust 用三个 trait 对闭包进行分类，按调用限制从宽松到严格排列：

### Fn — 不可变借用

`Fn` 闭包通过**不可变引用**捕获环境变量。可以多次调用，不会修改捕获的变量。

```rust
let name = String::from("Alice");
let say_hello = || println!("Hello, {}", name);
// say_hello 通过 &name 捕获 name

say_hello(); // 可以多次调用
say_hello();
println!("{}", name); // name 仍然可用
```

**何时自动成为 Fn：** 闭包体内只读取捕获的变量，不修改它们。

### FnMut — 可变借用

`FnMut` 闭包通过**可变引用**捕获环境变量。可以多次调用，每次调用可能修改捕获的变量。

```rust
let mut counter = 0;
let mut increment = || {
    counter += 1;
    println!("count: {}", counter);
};
// increment 通过 &mut counter 捕获 counter

increment(); // count: 1
increment(); // count: 2
increment(); // count: 3
// println!("{}", counter); // 错误：counter 被可变借用中
```

**何时自动成为 FnMut：** 闭包体内修改了捕获的变量（如赋值），或者调用了捕获变量的可变方法。

### FnOnce — 消费所有权

`FnOnce` 闭包**获取捕获变量的所有权**。只能调用一次，因为调用后会释放（drop）被捕获的值。

```rust
let message = String::from("重要消息");
let consume = || {
    // message 被移动到闭包中
    let owned = message;
    println!("消费: {}", owned);
    // owned 在此处被 drop
};

consume();
// consume(); // 错误：consume 只能调用一次
// println!("{}", message); // 错误：message 已被移动
```

**何时自动成为 FnOnce：** 闭包体内移动了捕获变量的所有权（如将 String 赋值给另一个变量，或将其传给另一个函数）。

### 三者关系

```
FnOnce (最宽松的 trait bound，所有闭包都实现)
├── FnMut (实现 FnMut 的闭包也自动实现 FnOnce)
│   └── Fn (实现 Fn 的闭包也自动实现 FnMut 和 FnOnce)
```

从最严格到最宽松：
- `Fn` 是最严格的：只读，可多次调用
- `FnMut` 中间：可修改，可多次调用
- `FnOnce` 是最宽松的：可消费，只能调用一次

**作为函数参数的 trait bound 时，应该选择能满足需求的最宽松的那个。** 例如，如果只需要读取，就用 `Fn`，这样调用者可以传入任何类型的闭包。

### 编译器如何决定闭包类型

编译器根据闭包体对捕获变量的操作自动推导 trait 实现：

| 闭包对捕获变量的操作 | 实现的 trait |
|---------------------|-------------|
| 只读取（不可变引用） | Fn + FnMut + FnOnce |
| 修改（可变引用） | FnMut + FnOnce |
| 移动/消费所有权 | FnOnce |

---

## 环境捕获

闭包捕获环境变量有三种方式：

### 1. 按引用捕获（不可变借用）

```rust
let x = 10;
let print_x = || println!("x = {}", x);
// x 被不可变借用
print_x();
println!("x 仍然可用: {}", x); // OK
```

### 2. 按可变引用捕获

```rust
let mut x = 10;
let mut double_x = || {
    x *= 2;
};
// x 被可变借用
double_x();
double_x();
// println!("{}", x); // 错误：x 被可变借用
drop(double_x); // 释放可变借用
println!("x = {}", x); // OK: x = 40
```

### 3. 按值捕获（获取所有权）

```rust
let s = String::from("hello");
let consume_s = || {
    let moved = s; // s 被移动到闭包中
    println!("{}", moved.len());
};
// println!("{}", s); // 错误：s 已被移动
```

### Rust 编译器的最小捕获原则

Rust 编译器默认采用**最小捕获原则**（capture with the least privilege needed）：如果闭包体只读取 `x`，就只捕获不可变引用；如果修改 `x`，就捕获可变引用；如果消费 `x` 的所有权，就获取所有权。这最大限度地保持了原变量的可用性。

---

## move 关键字

`move` 关键字强制闭包**获取所有捕获变量的所有权**，即使闭包体只需要读取它们。这常用于：

### 场景 1：跨线程传递闭包

```rust
use std::thread;

let msg = String::from("hello from thread");
let handle = thread::spawn(move || {
    // msg 被 move 到新线程中
    println!("{}", msg);
});
handle.join().unwrap();
// println!("{}", msg); // 错误：msg 已被移动
```

### 场景 2：闭包的生命周期超出变量作用域

```rust
fn create_greeter(greeting: String) -> impl Fn(&str) -> String {
    // 必须 move，因为 greeting 在函数返回后被释放
    move |name| format!("{}, {}!", greeting, name)
}

let greeter = create_greeter(String::from("你好"));
println!("{}", greeter("世界")); // 你好, 世界!
```

### 场景 3：需要闭包拥有数据

```rust
let data = vec![1, 2, 3, 4, 5];
let task = move || {
    // data 的所有权被移动到闭包中
    let sum: i32 = data.iter().sum();
    println!("sum = {}", sum);
    // data 在这里被释放
};
task();
// println!("{:?}", data); // 错误
```

### move 与非 Copy 类型

当捕获的变量实现了 `Copy` trait（如 i32、bool），`move` 实际上复制该值；对于非 `Copy` 类型（如 String、Vec），`move` 转移所有权。

```rust
let x = 42; // i32 实现了 Copy
let closure = move || println!("{}", x);
println!("{}", x); // OK！x 被 Copy 了，原值仍然可用

let s = String::from("hello"); // String 没有实现 Copy
let closure2 = move || println!("{}", s);
// println!("{}", s); // 错误！s 被 move 了
```

---

## 闭包与泛型

使用泛型和 trait bound 可以编写接受任意闭包作为参数的函数。

### 基本模式

```rust
// F 是实现 Fn(&str) -> String 的任意类型
fn apply_to_words<F>(text: &str, f: F) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    text.split_whitespace().map(|w| f(w)).collect()
}

// FnMut：可修改环境
fn filter_words<F>(text: &str, mut predicate: F) -> Vec<&str>
where
    F: FnMut(&&str) -> bool,
{
    text.split_whitespace().filter(|w| predicate(w)).collect()
}

// FnOnce：消费环境
fn consume_words<F>(text: &str, consumer: F)
where
    F: FnOnce(String),
{
    let all = text.split_whitespace().collect::<Vec<&str>>().join(" ");
    consumer(all);
}
```

### impl Trait 语法（更简洁）

```rust
fn apply_to_words(text: &str, f: impl Fn(&str) -> String) -> Vec<String> {
    text.split_whitespace().map(|w| f(w)).collect()
}
```

### 使用闭包 trait 作为返回类型

```rust
fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

let mut counter = make_counter(0);
assert_eq!(counter(), 1);
assert_eq!(counter(), 2);
```

### 三种 trait 的兼容性

作为参数 bound 时：
- 接受 `Fn` 的函数可以接受任何闭包
- 接受 `FnMut` 的函数只能接受 `FnMut` 和 `Fn` 闭包，不能接受 `FnOnce`（因为 `FnOnce` 只能调用一次）
- 接受 `FnOnce` 的函数可以接受任何闭包

选择 trait bound 的原则：**使用你能接受的最严格的 bound**（即 `Fn` > `FnMut` > `FnOnce`），这样调用者可以传入更多类型的闭包。

---

## 什么是迭代器（Iterator）

迭代器是 Rust 中处理序列的核心抽象。它表示一个可以按顺序产生一系列值的"数据流"。

```rust
// Iterator trait 核心定义（简化版）
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

迭代器只需要实现一个方法：`next()`。每次调用 `next()` 返回 `Some(元素)`，直到序列耗尽返回 `None`。

### 创建迭代器

```rust
// 从集合创建
let v = vec![1, 2, 3];
let iter = v.iter(); // &i32 迭代器

// 从范围
let range = 0..5; // 0, 1, 2, 3, 4

// 从其他迭代器转换
let chars = "hello".chars(); // char 迭代器
let lines = "a\nb\nc".lines(); // &str 迭代器
```

### 迭代器的核心设计理念

1. **惰性（Lazy）**：创建迭代器不立即执行任何操作
2. **可组合（Composable）**：通过适配器链式组合，构建处理管道
3. **零成本抽象（Zero-cost Abstraction）**：编译后生成与手写循环相同的机器码

---

## iter() vs iter_mut() vs into_iter()

这是初学者最容易混淆的概念之一。三者对应三种不同的所有权模型：

### iter() — 不可变引用迭代

返回元素的**不可变引用**（`&T`）。原集合保持不变，迭代后仍可使用。

```rust
let v = vec![1, 2, 3, 4, 5];

// 迭代不可变引用
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
println!("原 vec: {:?}", v); // [1, 2, 3, 4, 5] — 仍然可用
println!("结果: {:?}", doubled); // [2, 4, 6, 8, 10]
```

### iter_mut() — 可变引用迭代

返回元素的**可变引用**（`&mut T`）。可以修改原集合中的元素。

```rust
let mut v = vec![1, 2, 3];

// 原地修改
v.iter_mut().for_each(|x| *x *= 10);
println!("{:?}", v); // [10, 20, 30]
```

### into_iter() — 所有权迭代

**消费集合**，返回元素的所有权（`T`）。迭代后原集合不可再用。

```rust
let v = vec![String::from("a"), String::from("b")];

// 获取所有权，原 vec 被消费
let upper: Vec<String> = v.into_iter()
    .map(|s| s.to_uppercase())
    .collect();
// println!("{:?}", v); // 编译错误：v 已被移动
println!("{:?}", upper); // ["A", "B"]
```

### 对比表

| 方法 | 返回类型 | 原集合状态 | 适用场景 |
|------|---------|-----------|---------|
| `iter()` | `&T` | 不可变借用，仍然可用 | 只读遍历 |
| `iter_mut()` | `&mut T` | 可变借用，仍然可用 | 原地修改 |
| `into_iter()` | `T`（所有权） | 被消费，不可再用 | 转移所有权，转换集合 |

### 对于 for 循环的隐式调用

```rust
let v = vec![1, 2, 3];

// for 循环实际上调用了 into_iter()
for x in v {
    println!("{}", x);
}
// v 在此处已被消费

// 如果要保留 v，需要使用引用
let v = vec![1, 2, 3];
for x in &v { // 等价于 v.iter()
    println!("{}", x);
}
// v 仍然可用

for x in &mut v { // 等价于 v.iter_mut()
    *x += 1;
}
```

---

## 迭代器适配器（Adapter）

适配器将一种迭代器转换为另一种迭代器。它们是**惰性**的——不消费迭代器，只改变其行为。

### 常用适配器速览

| 适配器 | 功能 | 示例 |
|--------|------|------|
| `map` | 转换每个元素 | `.map(\|x\| x * 2)` |
| `filter` | 保留满足条件的元素 | `.filter(\|x\| x > 5)` |
| `enumerate` | 附加索引 | `.enumerate()` 生成 `(index, value)` |
| `take` | 仅取前 n 个元素 | `.take(5)` |
| `skip` | 跳过前 n 个元素 | `.skip(3)` |
| `chain` | 拼接两个迭代器 | `a.iter().chain(b.iter())` |
| `zip` | 配对两个迭代器 | `a.iter().zip(b.iter())` |
| `cloned` | 从 `&T` 转为 `T`（需要 Clone） | `.cloned()` |
| `copied` | 从 `&T` 转为 `T`（需要 Copy） | `.copied()` |
| `flat_map` | 展平嵌套迭代器 | `.flat_map(\|x\| x.split(','))` |
| `rev` | 反转迭代顺序 | `(0..5).rev()` |
| `cycle` | 无限循环（需谨慎） | `.cycle()` |
| `step_by` | 按步长跳过 | `(0..10).step_by(2)` |
| `fuse` | 确保 next 返回 None 后永远返回 None | `.fuse()` |
| `inspect` | 调试：对每个元素执行操作但不改变 | `.inspect(\|x\| println!("{x}"))` |

### 适配器链式调用示例

```rust
let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 链式组合多个适配器
let result: Vec<i32> = nums
    .iter()
    .filter(|&&x| x % 2 == 0)    // 只保留偶数: 2, 4, 6, 8, 10
    .map(|&x| x * x)              // 平方: 4, 16, 36, 64, 100
    .take(3)                      // 取前3个: 4, 16, 36
    .collect();                   // 消费并收集到 Vec

assert_eq!(result, vec![4, 16, 36]);
```

### inspect — 调试利器

```rust
let result: Vec<i32> = (1..=5)
    .inspect(|x| println!("before map: {x}"))
    .map(|x| x * 10)
    .inspect(|x| println!("after map: {x}"))
    .collect();
// 输出:
// before map: 1
// after map: 10
// before map: 2
// after map: 20
// ...
```

---

## 迭代器消费器（Consumer）

消费器**真正驱动**迭代器的执行。一旦调用消费器，迭代器就会被消耗。

### 常用消费器速览

| 消费器 | 功能 | 返回类型 |
|--------|------|---------|
| `collect()` | 收集到集合 | `Vec<T>`, `HashMap<K,V>`, `String` 等 |
| `fold(base, f)` | 折叠/归约 | 任意类型 |
| `count()` | 计数 | `usize` |
| `sum()` | 求和（需要 `Sum` trait） | 数字类型 |
| `product()` | 求积 | 数字类型 |
| `for_each(f)` | 对每个元素执行操作 | `()` |
| `find(predicate)` | 查找第一个满足条件的 | `Option<T>` |
| `position(predicate)` | 查找第一个满足条件的索引 | `Option<usize>` |
| `any(predicate)` | 是否有满足条件的 | `bool` |
| `all(predicate)` | 是否全部满足条件 | `bool` |
| `nth(n)` | 取第 n 个元素（从0开始） | `Option<T>` |
| `last()` | 取最后一个 | `Option<T>` |
| `max()` / `min()` | 最大/最小值 | `Option<T>` |
| `partition(f)` | 按条件分为两组 | `(Vec<T>, Vec<T>)` |
| `reduce(f)` | 类似 fold 但用第一个元素作为初值 | `Option<T>` |

### collect() 的类型标注

`collect()` 是最常用的消费器，需要类型标注告诉编译器要收集成什么集合：

```rust
let nums = [1, 2, 3, 4, 5];

// 显式标注返回类型
let v: Vec<i32> = nums.iter().copied().collect();
let even: Vec<i32> = nums.iter().filter(|&x| x % 2 == 0).copied().collect();

// turbofish 语法
let v = nums.iter().copied().collect::<Vec<i32>>();
let h: std::collections::HashSet<i32> = nums.iter().copied().collect::<_>();
let s: String = "hello".chars().collect();
```

### fold — 最通用的消费器

`fold` 可以表达几乎所有其他消费器的逻辑：

```rust
// 用 fold 实现 sum
let sum = (1..=10).fold(0, |acc, x| acc + x); // 55

// 用 fold 实现 max
let max = [3, 7, 2, 9, 1].iter().fold(i32::MIN, |max, &x| {
    if x > max { x } else { max }
}); // 9

// 用 fold 实现多种统计
let (sum, count) = [1, 2, 3, 4, 5]
    .iter()
    .fold((0, 0), |(sum, count), &x| (sum + x, count + 1));
// sum = 15, count = 5
```

### find 和 position 的区别

```rust
let words = ["apple", "banana", "cherry"];

// find 返回元素
let element = words.iter().find(|&&w| w.starts_with('b'));
println!("{:?}", element); // Some("banana")

// position 返回索引
let index = words.iter().position(|&w| w.starts_with('b'));
println!("{:?}", index); // Some(1)
```

---

## 惰性求值（Lazy Evaluation）

迭代器的核心特性。适配器链不会立即执行，直到消费器被调用。

### 核心概念

```rust
let nums = [1, 2, 3, 4, 5];

// 这一行不会做任何实际计算！
let lazy = nums.iter()
    .map(|x| x * 2)
    .filter(|x| x > 5);

// 直到这里，collect() 才驱动整个链条执行
let result: Vec<i32> = lazy.collect();
```

### 惰性求值的优势

1. **零中间分配**：不会为 `map` 或 `filter` 创建中间 Vec，所有操作在一次遍历中完成
2. **短路求值**：`find`、`any` 等在找到结果后立即停止
3. **无限序列**：可以处理无限的迭代器（配合 `take`）

```rust
// 懒惰的短路求值
let mut ops = 0;
let first = (1..100)
    .map(|x| { ops += 1; x * 2 })
    .find(|&x| x > 10);
println!("ops = {}, result = {:?}", ops, first);
// ops = 6, result = Some(12)
// 只执行了6次 map，而不是100次！
```

### 无限迭代器

```rust
// 生成所有自然数，取前10个
let first_ten: Vec<usize> = (0..).take(10).collect();
assert_eq!(first_ten, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

// 无限重复
let threes: Vec<i32> = std::iter::repeat(3).take(5).collect();
assert_eq!(threes, vec![3, 3, 3, 3, 3]);
```

### 可视化惰性求值

```
迭代器链：nums.iter().map(f).filter(g).take(3).collect()

执行过程（每个元素走完整个管道再处理下一个）：
  elem1 → map(f) → filter(g) ✓ → take 计数=1
  elem2 → map(f) → filter(g) ✗ → 跳过
  elem3 → map(f) → filter(g) ✓ → take 计数=2
  elem4 → map(f) → filter(g) ✓ → take 计数=3 → 停止！
  elem5 从未被处理！
```

---

## 性能：零成本抽象

Rust 的迭代器是**零成本抽象**的经典案例。编译后，`map`、`filter`、`fold` 等适配器通常生成与手写 `for` 循环相同的机器码。

### 为什么是零成本？

1. **单态化（Monomorphization）**：每个闭包类型都是唯一的，编译器为每种组合生成专用代码
2. **内联（Inlining）**：闭包体被内联到消费器的循环中，没有函数调用开销
3. **LLVM 优化**：生成的 IR 经过 LLVM 的循环优化（如向量化、循环展开）

### 示例：两者生成相同代码

```rust
// 手写循环
let mut sum = 0;
for &x in &data {
    if x > 5 {
        sum += x * 2;
    }
}

// 迭代器（编译后通常与上面相同）
let sum: i32 = data.iter()
    .filter(|&&x| x > 5)
    .map(|&x| x * 2)
    .sum();
```

### 何时需要关注性能

- 极简单的循环体：两者几乎无差别
- 嵌套迭代器 + `flat_map`：注意避免不必要的分配
- 大数组：迭代器通常可以受益于 LLVM 自动向量化

---

## 何时使用迭代器 vs 循环

### 适合用迭代器的场景

- **数据转换流水线**：过滤 → 映射 → 收集
- **聚合运算**：求和、求积、极值
- **简洁的条件检查**：`any`、`all`
- **需要惰性求值**：避免中间分配
- **代码可读性**：声明式风格让意图更清晰

### 适合用循环的场景

- **复杂控制流**：需要 `break`、`continue` 多层嵌套
- **早期返回**：在遇到特定条件时从函数返回
- **副作用为主**：大量 I/O 操作、外部状态修改
- **调试**：容易插入 `println!` 或断点
- **复杂状态管理**：多个变量需要复杂的状态转换

### 可以混用

两者不是互斥的。你可以在一个函数中使用迭代器进行数据转换，用循环进行状态管理：

```rust
fn process(data: &[i32]) -> Vec<i32> {
    // 用迭代器进行数据清洗
    let cleaned: Vec<i32> = data.iter()
        .filter(|&&x| x > 0)
        .copied()
        .collect();

    // 用循环进行复杂的状态机处理
    let mut result = Vec::new();
    let mut state = 0;
    for &x in &cleaned {
        match state {
            0 => { state = if x > 10 { 1 } else { 0 }; }
            1 => { result.push(x * 2); state = 0; }
            _ => unreachable!(),
        }
    }
    result
}
```

### 可读性判断标准

问自己：这段代码在做什么？

- 如果答案是"转换数据 A 为 B" → 迭代器
- 如果答案是"执行一系列步骤" → 循环
- 如果不确定 → 写两个版本，看看哪个更容易被同事理解

---

## Python 对照

对于从 Python 过来的开发者，以下对照表可以帮助理解 Rust 的概念。

### 闭包/匿名函数

| Python | Rust |
|--------|------|
| `lambda x: x + 1` | `\|x\| x + 1` |
| `def f(callback):` | `fn f<F: Fn(i32) -> i32>(callback: F)` |
| 闭包捕获：默认按引用 | `move` 关键字控制 |

```python
# Python
def make_multiplier(n):
    return lambda x: x * n  # 捕获 n

mult = make_multiplier(3)
print(mult(5))  # 15
```

```rust
// Rust
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n  // move 捕获 n（i32 实现了 Copy，实际是复制）
}

let mult = make_multiplier(3);
println!("{}", mult(5)); // 15
```

### 列表推导 vs 迭代器

| Python | Rust |
|--------|------|
| `[x*2 for x in lst]` | `lst.iter().map(\|x\| x*2).collect::<Vec<_>>()` |
| `[x for x in lst if x>0]` | `lst.iter().filter(\|&x\| x>0).collect()` |
| `sum(lst)` | `lst.iter().sum()` |
| `any(lst)` / `all(lst)` | `lst.iter().any(\|&x\| ...)` / `all` |

```python
# Python 列表推导
squares = [x*x for x in range(10) if x % 2 == 0]
# [0, 4, 16, 36, 64]
```

```rust
// Rust 迭代器
let squares: Vec<i32> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
// [0, 4, 16, 36, 64]
```

### 生成器表达式 vs 惰性迭代器

Python 的生成器表达式是惰性的，类似 Rust 的迭代器：

```python
# Python 生成器表达式（惰性）
gen = (x*2 for x in range(1000000))
first_3 = [next(gen) for _ in range(3)]  # [0, 2, 4]
```

```rust
// Rust 迭代器（天生惰性）
let lazy = (0..1000000).map(|x| x * 2);
let first_3: Vec<i32> = lazy.take(3).collect(); // [0, 2, 4]
```

### Fn/FnMut/FnOnce vs Python 闭包

| Rust trait | Python 等价概念 |
|-----------|---------------|
| `Fn` | 读取外部变量的闭包（默认行为） |
| `FnMut` | 使用 `nonlocal` 修改外部变量的闭包 |
| `FnOnce` | 消费外部变量所有权的闭包（Python GC 无此概念） |

```python
# Python FnMut 等价
counter = 0
def increment():
    nonlocal counter  # 需要 nonlocal 声明才能修改
    counter += 1
    return counter
```

```rust
// Rust FnMut
let mut counter = 0;
let mut increment = || {
    counter += 1;  // 编译器自动推断为 FnMut
    counter
};
```

### 关键差异

| 概念 | Python | Rust |
|------|--------|------|
| 所有权 | GC 管理，不存在"移动" | 闭包可能获取变量所有权 |
| 惰性 | 生成器是惰性的，推导式不是 | 所有迭代器都是惰性的 |
| 类型 | 动态类型 | 静态单态化，零成本 |
| 并行 | GIL 限制 | 可用 `par_iter()` (rayon) 并行 |

### rayon：并行迭代器

Rust 的一大优势：迭代器可以轻松并行化：

```rust
use rayon::prelude::*;

let sum: i32 = (0..1_000_000)
    .into_par_iter()  // 只需改为 par_iter
    .map(|x| x * 2)
    .sum();
// 自动多线程并行计算，Python 因 GIL 做不到
```

---

## 核心术语速查

| 术语 | 英文 | 含义 |
|------|------|------|
| 闭包 | Closure | 可捕获环境的匿名函数 |
| Fn | Fn | 不可变借用环境的闭包，可多次调用 |
| FnMut | FnMut | 可变借用环境的闭包，可多次调用 |
| FnOnce | FnOnce | 获取环境所有权的闭包，只能调用一次 |
| 迭代器 | Iterator | 惰性序列处理器 |
| 惰性求值 | Lazy Evaluation | 在消费时才执行计算 |
| 适配器 | Adapter | 转换迭代器行为的方法（map、filter 等） |
| 消费器 | Consumer | 驱动迭代器执行的方法（collect、fold 等） |
| iter | iter | 不可变引用迭代 |
| iter_mut | iter_mut | 可变引用迭代 |
| into_iter | into_iter | 所有权迭代，消费原集合 |
| move | move | 强制闭包获取捕获变量的所有权 |
| 零成本抽象 | Zero-cost Abstraction | 高级抽象在编译后不产生额外运行时开销 |
| 单态化 | Monomorphization | 编译器为每种泛型/闭包组合生成专用代码 |

---

## 进一步阅读

- [Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust Reference — Closure types](https://doc.rust-lang.org/reference/types/closure.html)
- [std::iter 模块文档](https://doc.rust-lang.org/std/iter/index.html)
- [rayon 并行迭代器](https://docs.rs/rayon/latest/rayon/iter/index.html)
- [itertools crate — 更多适配器](https://docs.rs/itertools/latest/itertools/)
