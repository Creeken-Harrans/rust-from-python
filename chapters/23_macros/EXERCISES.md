# 第 23 章练习: Rust 宏

## 练习指南

所有练习都在本章对应的 Cargo 项目中进行：

```bash
cd /home/Creeken/Temp/Rust_/rust-from-python/chapters/23_macros
```

### 推荐的命令

```bash
cargo build                          # 编译项目
cargo run                            # 运行程序
cargo check                          # 快速检查代码是否有编译错误
cargo expand main                    # 展开 main 函数的所有宏（需要先 cargo install cargo-expand）
cargo clippy                         # 代码质量检查
rustc --explain EXXXX                # 查看编译错误的详细解释
```

### 实验方式建议

1. 直接在 `src/main.rs` 中添加你的宏定义和测试代码
2. 使用 `cargo check` 快速验证语法正确性
3. 运行 `cargo run` 查看完整输出
4. 使用 `cargo expand` 查看宏展开后的代码（这是理解宏最有效的方式）

---

## Level 1 练习（基础）

### 练习 1.1: 补全宏定义

下面的宏定义缺少了某些部分。阅读代码，补全缺失的部分，使其能够编译和运行。

```rust
// TODO: 补全这个宏定义
// 它应该接收一个表达式并将其乘以 2 后打印
macro_rules! print_double {
    // 你的代码：
    
}

fn main() {
    // 期望输出: 10 的两倍是 20
    print_double!(10);           // 输出: 10 的两倍是 20
    // 期望输出: 3 + 4 的两倍是 14
    print_double!(3 + 4);        // 输出: 3 + 4 的两倍是 14
}
```

**要求**：使用 `stringify!` 来打印原始表达式，使用 `{}` 和 `{:?}` 的差异体会 `stringify!` 的作用。

**提示**：参考 `src/main.rs` 中的 `say!` 宏。

---

### 练习 1.2: 实现 `max!` 宏

编写一个名为 `max!` 的宏，接收两个表达式，返回其中较大的那个。

```rust
macro_rules! max {
    // 你的代码：
    
}

fn main() {
    let a = max!(3, 7);
    println!("max!(3, 7) = {}", a);    // 应该输出 7

    let b = max!(10 * 2, 15 + 1);
    println!("max!(10 * 2, 15 + 1) = {}", b); // 应该输出 20
    
    let c = max!(-5, -10);
    println!("max!(-5, -10) = {}", c); // 应该输出 -5
}
```

**要求**：宏体应该是一个表达式（而非语句），这样 `let a = max!(3, 7);` 才能工作。

**思考**：为什么不用函数来实现 `max`？（提示：考虑不同类型、表达式惰性求值）

---

### 练习 1.3: 使用标准库宏

阅读并运行以下代码，回答每个 `matches!` 的判断结果。然后自己编写一段代码，使用 `dbg!` 宏来调试一个复杂表达式。

```rust
fn main() {
    // 1. 分析 matches! 的判断结果
    let x = Some(42);
    println!("matches!(x, Some(1..=50)) = {}", matches!(x, Some(1..=50)));
    println!("matches!(x, None) = {}", matches!(x, None));
    
    let y: Result<i32, &str> = Err("oops");
    println!("matches!(y, Ok(_)) = {}", matches!(y, Ok(_)));
    println!("matches!(y, Err(_)) = {}", matches!(y, Err(_)));
    
    let z = vec![1, 2, 3, 4, 5];
    println!("matches!(z.as_slice(), [1, .., 5]) = {}", matches!(z.as_slice(), [1, .., 5]));
    println!("matches!(z.as_slice(), [1, 2, 3]) = {}", matches!(z.as_slice(), [1, 2, 3]));

    // 2. 使用 dbg! 调试下面的表达式
    // 在表达式中间插入 dbg! 来观察 a + b 和 a + b * 2 的值
    let a = 10;
    let b = 3;
    let result = a + b * 2; // 在此表达式中用 dbg! 包裹部分来调试
    println!("result = {}", result);
}
```

---

## Level 2 练习（进阶）

### 练习 2.1: 实现 `hashmap!` 宏

仿照 `vec!` 宏的设计，实现一个 `hashmap!` 宏，用于快速创建 `HashMap`。

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    // 空 HashMap
    () => {
        // 你的代码
    };
    // 有键值对的 HashMap: hashmap!("key1" => val1, "key2" => val2, ...)
    ($($key:expr => $value:expr),* $(,)?) => {
        // 你的代码：创建一个 HashMap 并逐个插入键值对
        
    };
}

fn main() {
    // 空 HashMap
    let empty: HashMap<&str, i32> = hashmap!();
    println!("空 HashMap: {:?}", empty);
    
    // 带内容的 HashMap
    let scores = hashmap!(
        "Alice" => 95,
        "Bob" => 82,
        "Charlie" => 78,
    );
    println!("成绩单: {:?}", scores);
    println!("Alice 的成绩: {}", scores["Alice"]);
    
    // 不同类型
    let config = hashmap!(
        "host" => "localhost",
        "port" => 8080,
        "debug" => true,
    );
    println!("配置: {:?}", config);
}
```

**要求**：
- 支持空 `HashMap` 的创建
- 支持末尾逗号
- 在 `main` 中验证所有功能正常工作

**提示**：需要在宏开头添加 `use std::collections::HashMap;` 或在宏内部使用完整路径 `::std::collections::HashMap::new()`。

**额外挑战**：你能让这个宏支持显式类型标注吗？比如 `hashmap!(String => i32, "key" => 42)`？

---

### 练习 2.2: 实现 `bench!` 宏

编写一个宏，用于简单地对一个表达式进行多次执行并计时。这个宏需要：
1. 打印被测量的表达式（使用 `stringify!`）
2. 执行表达式 N 次（N 由用户指定）
3. 测量总耗时
4. 打印平均每次执行时间

```rust
use std::time::Instant;

macro_rules! bench {
    // 你的设计：
    // bench!(次数, 表达式)
    // 例如: bench!(1_000_000, 2 * 3 + 4)
    
}

fn main() {
    // 测试简单的算术
    bench!(1_000_000, 2 * 3 + 4);
    
    // 测试 Vec 操作
    let v = vec![1, 2, 3, 4, 5];
    bench!(100_000, v.iter().sum::<i32>());
    
    // 测试字符串操作
    let s = "Hello, Rust Macros!";
    bench!(1_000_000, s.len());
}
```

**要求**：
- 使用 `std::time::Instant` 进行计时
- 打印格式清晰：被测试的表达式、执行次数、总耗时、平均耗时
- 注意：bench 宏内部必须阻止编译器优化掉"无用"的表达式（可以使用 `std::hint::black_box` 或简单的累加变量）

**提示**（防止编译器优化掉计算）：

```rust
// 在宏内部，将表达式结果赋给一个变量
// 然后用某种方式"使用"这个变量，防止编译器认为它没用而优化掉
let mut _sum = 0;
for _ in 0..$count {
    _sum += $expr; // 这不一定适合所有类型，你需要找到更好的方法
}
// 之后打印 _sum 的值（虽然这不是计时目标）
```

更好的方式是使用 `std::hint::black_box`：

```rust
let _ = std::hint::black_box($expr);
```

---

## Level 3 练习（高级）

### 练习 3.1: 实现 `sorted!` 编译期排序宏

这是一个有挑战性的练习。实现一个宏 `sorted!`，它接收一组表达式，**在编译期**使用递归宏将它们排序，并生成一个排好序的 Vec。

**但注意**：声明式宏无法在编译期进行比较运算（如 `1 < 2` 这样的比较）。因此本题的要求是：

实现一个宏 `sorted!`，递归地将输入**按输入顺序原样**输出为一个 Vec——即你不需要真的排序，但要展示**宏的递归能力**：用递归宏重组输入的 token 序列。

```rust
macro_rules! sorted {
    // 基本情况：空
    () => {
        Vec::new()
    };
    // 基本情况：单个元素
    ($x:expr) => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };
    // 递归情况：找到最小值，放到前面，然后递归处理剩余部分
    // 你的代码：
    
}

fn main() {
    let v = sorted!(5, 2, 8, 1, 9, 3);
    println!("排序后的 Vec: {:?}", v);
    // 期望输出: [1, 2, 3, 5, 8, 9]
}
```

**更实际的实现思路**：
由于声明式宏无法在编译期比较值，一个更实用的方案是：

```rust
macro_rules! sorted {
    ($($x:expr),* $(,)?) => {
        {
            let mut v = vec![$($x),*];
            v.sort();  // 运行时排序
            v
        }
    };
}
```

但这不够有趣。高级挑战的真正目标是：**用递归宏实现编译期的 token 重排能力**。

**真正的编译期排序**（自选挑战）：

提示：你可以定义多个辅助宏，通过"比较"语法树而非值来工作：

```rust
// 声明式宏可以做的一种"比较"：静态地选择较小的字面量
// 但这需要你对每个可能的输入做模式匹配——这在通用情况下不可行
// 因此这个练习的核心是体验宏递归和 token 操作
```

**如果你不想做真正的排序**，可以改为实现 `reversed!` 宏，它递归地将输入反转：

```rust
// reversed!(1, 2, 3) 输出 vec![3, 2, 1]
macro_rules! reversed {
    // 你的实现...
}
```

反转在宏层面更容易实现——你只需要将第一个元素推到已知的末尾位置。

---

## 思考题

### 思考: 宏的滥用与最佳实践

阅读以下虚构的 Rust 代码库。这是一位"宏狂热者"写的代码。分析这段代码的问题，并回答：

```rust
// ===== 文件: over_macro.rs (虚构的反面教材) =====

// 用宏替代所有控制流
macro_rules! if_else {
    ($cond:expr, $true_block:block, $false_block:block) => {
        if $cond $true_block else $false_block
    };
}

// 用宏替代 match
macro_rules! match_it {
    ($val:expr, { $($pat:pat => $body:expr),* $(,)? }) => {
        match $val {
            $($pat => $body),*
        }
    };
}

// 用宏替代类型定义
macro_rules! define_struct {
    ($name:ident, $($field:ident: $ty:ty),*) => {
        struct $name {
            $($field: $ty),*
        }
    };
}

// 用宏替代函数（甚至单行逻辑）
macro_rules! add {
    ($a:expr, $b:expr) => { $a + $b };
}

macro_rules! print {
    ($($arg:tt)*) => { println!($($arg)*) };
}

// 用宏替代常量和配置
macro_rules! DB_HOST {
    () => { "localhost" };
}
macro_rules! DB_PORT {
    () => { 5432 };
}

// 用宏隐藏复杂的错误处理
macro_rules! try_or_log {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                eprintln!("错误: {:?}", e);
                return Err(e);
            }
        }
    };
}

fn main() {
    let result = try_or_log!(std::fs::read_to_string("config.txt"));
    let x = add!(1, 2);
    print!("x = {}", x);
    if_else!(x > 3, {
        print!("x 大于 3");
    }, {
        print!("x 不大于 3");
    });
}
```

**请分析并回答以下问题**：

1. 这段代码中哪些宏是**完全不应该存在**的？为什么？（至少列出 3 个）
2. 哪些宏虽然存在争议但**在某些场景下可以接受**？说明你的理由。
3. `try_or_log!` 宏看起来最"合理"，但它有什么隐患？
4. 如果用 Rust 的惯用方式（不使用这些自定义宏）重写 `main` 函数中的逻辑，代码会是什么样子？写出你认为更合理的版本。
5. 总结：什么时候宏是"恰到好处"的？用你自己的话概括一个判断标准。

**额外思考**：
- 在大型团队中使用过多自定义宏会导致什么问题？
- 宏的错误信息通常不如函数的错误信息友好，这对团队开发有什么影响？
- 是否有"好宏"和"坏宏"的客观标准？还是完全取决于上下文？

---

## 练习参考解答思路

以下不是完整答案，而是解题方向的提示。建议先自行尝试后再参考。

### 1.1 提示

`stringify!` 将 token 序列转为字符串。你的宏需要：
- 匹配一个表达式 `$x:expr`
- 使用 `stringify!($x)` 获取表达式的字符串形式
- 用 `$x * 2` 计算结果

### 1.2 提示

宏体需要求值为一个表达式（不是语句），这样才能被赋值。可以直接使用 if 表达式（它在 Rust 中是表达式，不是语句）：

```rust
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}
```

### 2.1 提示

```rust
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
}
```

### 2.2 提示

关键挑战是防止编译器优化。使用 `std::hint::black_box`（稳定）或 `std::intrinsics::black_box`（不稳定）。在宏中创建一个累加器并最终打印其值可以防止优化：

```rust
// 警告：这只对数值类型有效
let mut acc = 0;
let start = Instant::now();
for _ in 0..$count {
    acc = acc.wrapping_add($expr);
}
let elapsed = start.elapsed();
println!("{} x{}: {:?} total, {:?} avg (checksum={})",
    stringify!($expr), $count, elapsed,
    elapsed / $count as u32, acc);
```

### 3.1 提示

递归宏实现反转的思想：

```rust
macro_rules! reversed_rec {
    // 递归终止：空输入 → 空输出
    ([$($reversed:expr),*],) => {
        vec![$($reversed),*]
    };
    // 递归步骤：取出第一个元素放到 accumulator 的头部
    ([$($reversed:expr),*], $head:expr $(, $tail:expr)* $(,)?) => {
        reversed_rec!([$head $(, $reversed)*], $($tail),*)
    };
}

macro_rules! reversed {
    ($($x:expr),* $(,)?) => {
        reversed_rec!([], $($x),*)
    };
}
```

---

## 练习完成检查清单

完成以下所有项后，你的宏学习才算完整：

- [ ] 能独立编写简单的 `macro_rules!` 宏（如 `say!` 风格）
- [ ] 理解 `$(...)*` 和 `$(...)+` 的区别并能正确使用
- [ ] 能使用 `stringify!` 进行宏调试
- [ ] 能熟练使用标准库中的 `println!`, `vec!`, `dbg!`, `matches!`, `assert!`
- [ ] 理解 `todo!` 和 `unimplemented!` 的用途和区别
- [ ] 知道过程宏的三种类型及其应用场景
- [ ] 理解为什么过程宏需要单独的 crate
- [ ] 理解宏卫生的概念，知道它解决了什么问题
- [ ] 能判断一个场景是否适合使用宏（而非函数或泛型）
- [ ] 使用过 `cargo expand` 展开宏并阅读生成的代码
- [ ] 完成了至少 2 个 Level 1 练习
- [ ] 完成了一个 Level 2 练习
- [ ] 思考并回答了思考题中的问题

---

## 延伸阅读

- [Rust Reference - Macros](https://doc.rust-lang.org/reference/macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust By Example - Macros](https://doc.rust-lang.org/rust-by-example/macros.html)
- [syn crate 文档](https://docs.rs/syn/) — 过程宏的解析库
- [quote crate 文档](https://docs.rs/quote/) — 过程宏的代码生成库
- [proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) — 过程宏练习项目
