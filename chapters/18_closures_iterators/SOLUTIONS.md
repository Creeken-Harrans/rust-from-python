# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：闭包语法

```rust
let add_one = |x: i32| -> i32 { x + 1 };
let double = |x| x * 2;                    // 类型推断
let greet = |name: &str| format!("Hello, {}", name);
```

闭包与 `fn` 的区别：闭包可以捕获环境变量，`fn` 不行。

---

### 1-2：闭包捕获方式

```rust
let text = String::from("hello");
let print_text = || println!("{}", text);    // Fn: 不可变借用
// let consume = || drop(text);              // FnOnce: 获取所有权
// let mut modify = || text.push('!');       // FnMut: 可变借用
```

三个 trait 的包含关系：`Fn` ⊆ `FnMut` ⊆ `FnOnce`。实现了 `Fn` 的闭包自动实现 `FnMut` 和 `FnOnce`。

---

### 1-3：迭代器基础

```rust
let nums = vec![1, 2, 3, 4, 5];

// 惰性迭代器（不消费数据，不会执行）
let doubled = nums.iter().map(|x| x * 2);

// 消费方法触发计算
let result: Vec<_> = doubled.collect();
assert_eq!(result, vec![2, 4, 6, 8, 10]);
```

迭代器是惰性的——在调用 `.collect()` 等消费方法之前，链上的 `map`/`filter` 等都不会执行。

---

## Level 2：组合应用

### 2-1：迭代器链

```rust
let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let even_squares: Vec<i32> = nums.iter()
    .filter(|&&x| x % 2 == 0)   // 保留偶数
    .map(|&x| x * x)             // 求平方
    .collect();
assert_eq!(even_squares, vec![4, 16, 36, 64, 100]);
```

每个步骤返回新迭代器，不创建中间集合——零额外分配。

---

### 2-2：闭包作为参数

```rust
fn transform<T, F>(data: Vec<T>, f: F) -> Vec<T>
where F: Fn(T) -> T
{
    data.into_iter().map(f).collect()
}

let result = transform(vec![1, 2, 3], |x| x * 10);
assert_eq!(result, vec![10, 20, 30]);
```

泛型参数 `F` 配合 trait bound `Fn(T) -> T`，编译器为每个闭包单态化生成独立代码（零成本抽象）。

---

## Level 3：设计思考

### 3-1：iterator 三种获取方式的适用场景

| 方法 | 借用类型 | 使用场景 |
|------|---------|---------|
| `.iter()` | `&T` | 只读遍历，之后还要用原集合 |
| `.iter_mut()` | `&mut T` | 遍历并修改元素 |
| `.into_iter()` | 获取所有权 | 消费集合，不再需要原数据 |

选择取决于生命周期：`iter()` 最通用，`into_iter()` 在最"绝不回头"的场景下最简洁。

### 3-2：闭包与函数指针

```rust
fn add_one(x: i32) -> i32 { x + 1 }

let f: fn(i32) -> i32 = add_one;           // 函数指针（不捕获环境）
let c = |x: i32| x + 1;                     // 闭包（可捕获环境）
```

闭包不能直接转为函数指针（如果捕获了环境）。非捕获闭包可以自动强制为 `fn` 指针。

---

## 迁移思维练习

### Python 迭代器 vs Rust 迭代器

| 方面 | Python | Rust |
|------|--------|------|
| 惰性 | 是（生成器） | 是（默认，无消费不执行） |
| 链式调用 | `map()`、`filter()` | `.map()`、`.filter()`（无中间分配） |
| 获取方式 | 一种 `for x in seq` | 三种 `iter/iter_mut/into_iter` |
| 所有权 | 不涉及 | `into_iter()` 消费原数据 |

**迁移提示**：Python 的迭代器不会区分借用/消费，因为 Python 是引用计数。Rust 的三种 `iter` 变体是所有权系统在迭代器上的自然延伸——先判断遍历后是否需要原始数据，再选择方法。

---

*闭包和迭代器是 Rust 函数式风格的核心。掌握惰性求值和 trait 体系（Fn/FnMut/FnOnce）后，能写出简洁而高效的流水线代码。*
