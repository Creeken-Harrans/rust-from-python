# 参考答案

建议先独立完成练习，再阅读本文件。
本文件提供的是参考实现和设计分析，不代表所有题目只有一种正确写法。

---

## Level 1: 基础练习

### 练习 1-1：类型识别

#### 结论

Rust 会通过类型推断为每个未标注类型的变量推导出具体类型。理解默认类型规则（整数默认 `i32`，浮点默认 `f64`）是写出正确 Rust 代码的前提。

#### 思路

逐行分析每个变量的类型推断规则：

| 代码 | 推断类型 | 推理依据 |
|------|----------|----------|
| `let a = 42;` | `i32` | 整数字面量默认推断为 `i32` |
| `let b = 3.14;` | `f64` | 浮点字面量默认推断为 `f64` |
| `let c: char = 'R';` | `char` | 显式标注 |
| `let d = true;` | `bool` | 布尔字面量只有一种类型 |
| `let e = [1, 2, 3, 4, 5];` | `[i32; 5]` | 元素默认 `i32`，长度 5 |
| `let f = (100, "hello", false);` | `(i32, &str, bool)` | 各元素独立推断 |
| `let g = "rustacean";` | `&str` | 字符串字面量是 `&str` |
| `let h: u8 = 255;` | `u8` | 显式标注 |
| `let i = 0_isize;` | `isize` | 后缀 `_isize` 明确指定 |
| `let j = b'A';` | `u8` | 字节字面量 `b'A'` 的类型是 `u8` |
| `let k: () = ();` | `()` | 显式标注，单元类型 |
| `let l = [0u8; 16];` | `[u8; 16]` | 元素类型 `u8`，长度 16 |
| `let m = (42,);` | `(i32,)` | 单元素元组，注意逗号不能省略 |

**关键区分**：
- `(100, "hello", false)` 的类型是 `(i32, &str, bool)`，不是某个统一的 "tuple" 类型——Rust 中每个元组类型由其元素类型和数量唯一确定。
- `(42,)` vs `(42)`：前者是单元素元组 `(i32,)`，后者只是加了括号的 `i32` 值——逗号是关键。
- `b'A'` 是 `u8` 而非 `char`——字节字面量和字符字面量是不同类型。

#### 参考实现：验证代码

```rust
fn main() {
    let a = 42;
    let b = 3.14;
    let c: char = 'R';
    let d = true;
    let e = [1, 2, 3, 4, 5];
    let f = (100, "hello", false);
    let g = "rustacean";
    let h: u8 = 255;
    let i = 0_isize;
    let j = b'A';
    let k: () = ();
    let l = [0u8; 16];
    let m = (42,);

    // 使用 std::any::type_name 验证类型（注意：输出格式因编译器而异）
    println!("a: {}", std::any::type_name_of_val(&a)); // i32
    println!("b: {}", std::any::type_name_of_val(&b)); // f64
    println!("c: {}", std::any::type_name_of_val(&c)); // char
    println!("d: {}", std::any::type_name_of_val(&d)); // bool
    println!("e: {}", std::any::type_name_of_val(&e)); // [i32; 5]
    println!("f: {}", std::any::type_name_of_val(&f)); // (i32, &str, bool)
    println!("g: {}", std::any::type_name_of_val(&g)); // &str
    println!("h: {}", std::any::type_name_of_val(&h)); // u8
    println!("i: {}", std::any::type_name_of_val(&i)); // isize
    println!("j: {}", std::any::type_name_of_val(&j)); // u8
    println!("k: {}", std::any::type_name_of_val(&k)); // ()
    println!("l: {}", std::any::type_name_of_val(&l)); // [u8; 16]
    println!("m: {}", std::any::type_name_of_val(&m)); // (i32,)
}
```

#### 常见错误

- 以为 `0_isize` 是 `usize`——`isize` 是有符号的，`usize` 是无符号的。
- 以为 `b'A'` 是 `char`——它不是，它是 `u8`（ASCII 字节值 65）。
- 以为 `(42,)` 和 `(42)` 类型相同——前者是元组，后者是整数。
- 以为 `"rustacean"` 的类型是 `String`——它是 `&str`（字符串切片引用），`String` 是堆分配的，需要用 `String::from()` 或 `.to_string()` 创建。

#### 验证方式

```bash
cargo run
# 观察 type_name_of_val 的输出，与自己的推理对照
```

---

### 练习 1-2：修复编译错误

#### 结论

每个编译错误都对应一个 Rust 的核心规则：不可变性、类型一致性、边界安全、类型标注和类型转换。

#### 思路：逐错误分析

**错误 1**：`x = 6;`
- **原因**：`x` 用 `let` 声明（无 `mut`），不可变。对不可变变量赋值是编译错误。
- **修复**：`let mut x = 5;`（给 `let` 添加 `mut`）

**错误 2**：`y = "hello";`
- **原因**：`y` 的类型是 `i32`，不能赋值 `&str` 类型的值。类型不匹配。
- **修复**：删除这行，或者改为正确的类型：`let y: &str = "hello";`

**错误 3**：`arr[3]`
- **原因**：`arr` 的长度是 3，有效索引是 `0..=2`，`arr[3]` 越界。注意：这种越界在**编译期无法**完全检查（索引是运行时值），但 Rust 的编译器和 clippy 可能会给警告。运行时必定 panic。
- **修复**：改为 `arr[2]` 或使用 `arr.get(3)` 安全访问。

**错误 4**：`"256".parse().unwrap()`
- **原因**：`.parse()` 的目标类型不明确——编译器不知道你要解析成 `i32`、`u8` 还是其他类型。
- **修复**：`let parsed: u32 = "256".parse().unwrap();`（显式标注类型）或 `"256".parse::<i32>().unwrap();`（turbofish 语法）。

**错误 5**：`let w: i8 = z;`
- **原因**：`z` 是 `u8` 类型，不能隐式转换为 `i8`。Rust 没有隐式类型转换，即使 `u8` 和 `i8` 大小相同。
- **修复**：`let w: i8 = z as i8;`（使用 `as` 显式转换）

#### 参考实现

```rust
fn main() {
    // 修复 1：添加 mut 使变量可变
    let mut x = 5;
    x = 6;               // ✅ 现在可以赋值

    // 修复 2：保持类型一致
    let y: i32 = 10;
    // y = "hello";      // 删除此行 —— 不能将 &str 赋给 i32

    // 修复 3：使用有效索引
    let arr = [1, 2, 3];
    let elem = arr[2];   // ✅ 索引 2 是最后一个元素（有效范围 0..=2）

    // 修复 4：显式标注 parse 的目标类型
    let parsed: u32 = "256".parse().unwrap(); // ✅ 标注为 u32

    // 修复 5：使用 as 进行显式类型转换
    let z = 128u8;
    let w: i8 = z as i8; // ✅ 显式转换（注意：128 在 i8 中会溢出为 -128）

    // 验证
    println!("x = {x}");
    println!("y = {y}");
    println!("elem = {elem}");
    println!("parsed = {parsed}");
    println!("z = {z}, w = {w}");
}
```

**特别注意**：`128u8 as i8` 的结果是 `-128`（因为 `u8` 的 128 的二进制表示是 `10000000`，解释为有符号 `i8` 时变成了 `-128`——这是二进制补码的截断行为）。如果你想安全转换而不丢失信息，应该使用 `i8::try_from(z)` 或 `From` trait 的 `try_from`，它在溢出时返回 `Err`。

#### 常见错误

- 盲目给所有变量加 `mut`——只有确实需要修改的变量才加 `mut`。
- 使用 `as` 转换时不检查溢出——`u8` 的 255 转 `i8` 会得到 `-1`。
- 以为 Rust 像 C 一样支持隐式类型提升——Rust **从不**做隐式类型转换。

#### 验证方式

```bash
cargo build   # 必须编译通过
cargo run     # 确认程序正常运行，无 panic
```

---

### 练习 1-3：元组操作练习

#### 结论

元组是 Rust 中最灵活的复合类型——它可以包含任意数量和类型的元素。元组数组 `[(T1, T2, T3); N]` 可以用于存储固定数量的结构化数据。

#### 思路

1. 创建单个元组存储同学信息 `(name, age, score)`。
2. 使用模式解构将元组拆分为独立变量。
3. 使用 `.0`、`.1`、`.2` 索引访问元组元素。
4. 创建元组数组，用 for 循环遍历。

#### 参考实现

```rust
/// 演示元组的创建、解构、索引访问和遍历。
fn tuple_exercise() {
    println!("===== 元组操作练习 =====");

    // 1. 创建包含三个元素的元组
    let student: (&str, i32, f64) = ("张三", 20, 87.5);
    println!("原始元组: {:?}", student);

    // 2. 模式解构
    let (name, age, score) = student;
    println!("模式解构: 姓名={}, 年龄={}, 成绩={}", name, age, score);

    // 3. 索引访问
    println!(
        "索引访问: student.0={}, student.1={}, student.2={}",
        student.0, student.1, student.2
    );

    // 4. 元组数组
    let students: [(&str, i32, f64); 3] = [
        ("张三", 20, 87.5),
        ("李四", 19, 92.0),
        ("王五", 21, 76.5),
    ];

    println!("\n全班同学信息:");
    for (idx, (name, age, score)) in students.iter().enumerate() {
        println!(
            "  [{idx}] 姓名: {name:<6} 年龄: {age:<3} 成绩: {score:.1}"
        );
    }
}
```

在 `main()` 中调用：

```rust
fn main() {
    // ... 已有的 main 代码 ...
    tuple_exercise();
}
```

#### 为什么这样设计

- **模式解构** `let (name, age, score) = student;` 比逐个 `.0`、`.1`、`.2` 更可读，且编译器会检查元素数量是否匹配。
- **元组数组类型标注** `[(&str, i32, f64); 3]` 中，每个元素的元组必须完全同构——不能第一个元素是 `(&str, i32, f64)` 而第二个是 `(String, u8, f32)`。
- **不使用 `Vec`** 是刻意约束——数组长度在编译期确定，分配在栈上，不需要堆分配。

#### 常见错误

- 忘记单元素元组的逗号：`(42)` 是整数，`(42,)` 才是元组。
- 模式解构时元素数量不匹配：`let (name, age) = student;`（少了 score）→ 编译错误 `mismatched types`。
- 元组数组的元素类型不一致——元组类型是精确的，`(&str, i32, f64)` 和 `(String, i32, f64)` 是不同的类型。

#### 验证方式

```bash
cargo run
# 确认元组操作输出清晰可读
```

---

## Level 2: 进阶练习

### 练习 2-1：遮蔽 vs mut 深度理解

#### 结论

遮蔽（shadowing）是**创建新绑定**，旧的绑定被"隐藏"但未被销毁；`mut` 是**修改原绑定**的值。两者在作用域行为、类型变化能力、不可变性保留方面有本质区别。

核心区别：
- `let mut x = 5; x = 10;` —— 修改**同一个** `x` 绑定的值，类型不变。
- `let x = 5; let x = "hello";` —— 创建了**两个不同的** `x` 绑定，第二个隐藏了第一个，类型可以不同。

#### 思路

设计 4 个演示点：
1. 作用域行为对比
2. 类型变化能力
3. 多次遮蔽（类型变换链）
4. 不可变性保留

#### 参考实现

```rust
/// 深度演示遮蔽 (shadowing) 与 mut 的区别。
///
/// 包含四个演示部分：
/// 1. 作用域行为
/// 2. 类型变化能力
/// 3. 多次遮蔽类型变换
/// 4. 不可变性保留
fn demonstrate_shadowing_vs_mut() {
    println!("===== 遮蔽 vs mut 深度理解 =====\n");

    // ---------------------------------------------------------------
    // 1. 作用域行为
    // ---------------------------------------------------------------
    println!("--- 1. 作用域行为 ---");

    let mut mut_var = 10;
    let shadow_var = 10;

    {
        // mut 在内层作用域修改原值
        mut_var += 1;
        println!("  内层作用域: mut_var 被修改为 {}", mut_var);

        // shadowing 在内层作用域创建新绑定
        let shadow_var = 99;
        println!("  内层作用域: shadow_var 被遮蔽为 {}", shadow_var);
    }

    // 退出内层作用域后：
    println!("  退出内层作用域后:");
    println!("    mut_var = {}    ← mut 的修改保留（修改的是原绑定）", mut_var);
    println!(
        "    shadow_var = {}  ← 遮蔽效果消失（内层的 let 是全新的绑定）",
        shadow_var
    );
    println!("  结论: mut 修改共享同一个绑定；遮蔽仅限于当前作用域\n");

    // ---------------------------------------------------------------
    // 2. 类型变化能力
    // ---------------------------------------------------------------
    println!("--- 2. 类型变化能力 ---");

    let value = 42i32;
    println!("  原始: value = {} ({})", value, std::any::type_name_of_val(&value));
    // 遮蔽可以改变类型：
    let value = "现在是 &str 类型了";
    println!(
        "  遮蔽后: value = {} ({})",
        value,
        std::any::type_name_of_val(&value)
    );

    // mut 不能改变类型 —— 以下代码编译失败：
    // let mut y = 10;
    // y = "hello"; // ❌ error[E0308]: mismatched types
    println!("  结论: 遮蔽可以改变类型，mut 不能\n");

    // ---------------------------------------------------------------
    // 3. 多次遮蔽（类型变换链）
    // ---------------------------------------------------------------
    println!("--- 3. 多次遮蔽：类型变换链 ---");

    let chain = 42i32;
    println!("  Step 1 (i32):    chain = {}", chain);
    let chain = std::f64::consts::PI;
    println!("  Step 2 (f64):    chain = {:.6}", chain);
    let chain = '🦀';
    println!("  Step 3 (char):   chain = {}", chain);
    let chain = "Rust";
    println!("  Step 4 (&str):    chain = {}", chain);
    // 使用 let mut 的遮蔽（新绑定是可变的）
    let mut chain = (100, "hello");
    println!("  Step 5 (元组):   chain = {:?}", chain);
    chain.0 += 1; // 修改新绑定的内容
    println!(
        "  Step 5 修改后:  chain = {:?} (let mut 创建的绑定可修改)",
        chain
    );
    println!();

    // ---------------------------------------------------------------
    // 4. 不可变性保留
    // ---------------------------------------------------------------
    println!("--- 4. 不可变性保留 ---");

    let immutable_var = "原来的值";
    println!("  定义: let immutable_var = \"{}\";", immutable_var);

    // 遮蔽为同名的不可变绑定（新绑定也不可变）
    let immutable_var = "被遮蔽的新值";
    println!("  遮蔽: let immutable_var = \"{}\";", immutable_var);

    // 尝试修改被遮蔽后的变量（虽然它来自遮蔽，但新绑定默认仍不可变）：
    // immutable_var = "尝试修改"; // ❌ 编译错误：不能修改不可变变量

    // 如果要让遮蔽产生的变量可变，需要显式标注：
    let mut shadowed_mut = "先遮蔽";
    println!("  定义: let mut shadowed_mut = \"{}\";", shadowed_mut);
    let shadowed_mut = 42; // 遮蔽为不可变的 i32
    // shadowed_mut = 43;   // ❌ 编译错误：新绑定没有 mut
    println!("  遮蔽: let shadowed_mut = {}; (新绑定不可变)", shadowed_mut);
    println!("  结论: 即使被遮蔽的旧绑定是 mut，新绑定默认仍不可变\n");

    println!("===== 演示结束 =====");
}
```

在 `main()` 中调用：

```rust
fn main() {
    // ... 已有的 main 代码 ...
    demonstrate_shadowing_vs_mut();
}
```

#### 为什么这样设计

- **作用域行为**是遮蔽与 mut 最直观的区别。`mut` 修改的是"同一个存储位置"，遮蔽在特定作用域内创建"全新的绑定"。这个区别与 Rust 的所有权系统天然契合——遮蔽不会影响旧绑定引用的数据。
- **类型变化**能力是遮蔽的独特优势。在同一段代码中，你可以先让变量表示 `i32` 的索引，然后用遮蔽让它表示 `&str` 的名称——这避免了 `index` 和 `name` 两个不同变量名，但保持了可读性（因为它暗示"这是同一个逻辑概念，只是类型转换了"）。
- **不可变性保留**体现了 Rust 的保守哲学：即使你遮蔽了一个 `mut` 变量，新绑定默认仍不可变。你需要显式写 `let mut` 来获得可变的遮蔽绑定。

#### 常见错误

- 以为遮蔽后旧值会被销毁——旧值继续存在（如果它被其他地方引用，在退出作用域前不会被 drop）。
- 以为 `let mut` 的遮蔽使新绑定也可变——不会，`let` 和 `let mut` 是独立的声明。
- 过度使用遮蔽——在同一个作用域内，同一个变量名经历多次遮蔽会使代码难以跟踪。

#### 验证方式

```bash
cargo run
# 观察每个演示部分的输出，确认理解
```

---

### 练习 2-2：实现安全数组工具函数

#### 结论

通过切片引用 `&[i32]`，函数可以同时接受数组和 `Vec` 的引用，体现了 Rust 中"面向 trait 编程"的优势（切片是对连续内存的抽象视图）。

#### 思路

1. `safe_get`：使用 `get()` 方法（返回 `Option`）而非直接索引（会 panic）。
2. `print_element`：组合 `safe_get` + `match` 做优雅的错误处理。
3. `array_sum`：用 `for` 循环累加。
4. `swap_elements`：用 `get()` 做边界检查，`Result` 表示成功/失败。
5. 在 `main` 中全面测试。

#### 参考实现

```rust
/// 安全地获取数组元素，使用 `get()` 方法。
///
/// `get()` 返回 `Option<&i32>`，越界时返回 `None` 而非 panic。
/// 比直接索引 `arr[index]` 更安全。
fn safe_get(arr: &[i32], index: usize) -> Option<i32> {
    // get() 返回 Option<&i32>，用 copied() 转为 Option<i32>
    arr.get(index).copied()
}

/// 调用 `safe_get` 并优雅地打印结果。
///
/// 使用 `match` 分别处理 Some（找到元素）和 None（越界）两种情况。
fn print_element(arr: &[i32], index: usize) {
    match safe_get(arr, index) {
        Some(val) => println!("  arr[{}] = {}", index, val),
        None => println!("  arr[{}] = ??? (索引越界，数组长度: {})", index, arr.len()),
    }
}

/// 计算数组中所有元素的和。
///
/// 使用 for 循环手动累加，不使用迭代器的 `sum()` 方法，
/// 以便清晰展示循环过程。
fn array_sum(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &value in arr {
        sum += value;
    }
    sum
}

/// 交换数组中两个位置的元素。
///
/// 使用 `get()` 方法进行边界检查，避免越界 panic。
/// 如果任一索引越界，返回 `Err` 并附带错误信息；
/// 否则执行交换并返回 `Ok(())`。
///
/// # 参数
/// * `arr` - 可变的整数切片引用
/// * `i` - 第一个位置索引（usize，标准索引类型）
/// * `j` - 第二个位置索引（usize，标准索引类型）
///
/// # 为什么用 usize
/// 索引类型使用 `usize` 而非 `u32` 或 `u64`，因为：
/// 1. `usize` 是指针宽度（32 位架构上 32 位，64 位架构上 64 位）
/// 2. 语言内建的数组/切片索引操作需要 `usize`
/// 3. 用 `u32` 需要手动转换 `arr[i as usize]`，不自然
fn swap_elements(arr: &mut [i32], i: usize, j: usize) -> Result<(), String> {
    let len = arr.len();

    // 边界检查：使用 get() 同时检查两个索引
    if i >= len {
        return Err(format!("索引 {} 越界，数组长度: {}", i, len));
    }
    if j >= len {
        return Err(format!("索引 {} 越界，数组长度: {}", j, len));
    }

    // 两个索引都在范围内，执行交换
    // 使用 swap 方法（标准库提供，不需要手动临时变量）
    arr.swap(i, j);
    Ok(())
}

/// 测试所有安全数组工具函数。
fn test_array_utils() {
    println!("===== 安全数组工具函数测试 =====\n");

    let data = [10, 20, 30, 40, 50];

    // 测试 safe_get 和 print_element
    println!("--- safe_get 和 print_element ---");
    print_element(&data, 0);  // 正常访问
    print_element(&data, 2);  // 正常访问
    print_element(&data, 4);  // 最后一个元素
    print_element(&data, 5);  // 越界
    print_element(&data, 100); // 严重越界

    // 测试 array_sum
    println!("\n--- array_sum ---");
    let sum = array_sum(&data);
    println!("  data 总和: {} (预期: 150)", sum);
    assert_eq!(sum, 150);

    // 空数组求和
    let empty: [i32; 0] = [];
    println!("  空数组总和: {} (预期: 0)", array_sum(&empty));

    // 测试 swap_elements
    println!("\n--- swap_elements ---");

    let mut swap_data = [10, 20, 30, 40, 50];
    println!("  交换前: {:?}", swap_data);

    match swap_elements(&mut swap_data, 0, 4) {
        Ok(()) => println!("  交换后 (索引 0 <-> 4): {:?}", swap_data),
        Err(e) => println!("  交换失败: {}", e),
    }

    match swap_elements(&mut swap_data, 0, 10) {
        Ok(()) => println!("  交换后: {:?}", swap_data),
        Err(e) => println!("  越界交换失败: {}", e),
    }

    // 演示 &[i32] 既可以传数组引用也可以传 Vec 引用
    println!("\n--- 切片参数的灵活性 ---");
    let vec_data = vec![1, 2, 3, 4, 5];
    println!("  Vec 的总和 (通过 &[i32] 参数): {}", array_sum(&vec_data));
    print_element(&vec_data, 2); // Vec 引用也可以传

    println!("\n===== 测试完成 =====");
}
```

在 `main()` 中调用：

```rust
fn main() {
    // ... 已有的 main 代码 ...
    test_array_utils();
}
```

#### 为什么这样设计

- **切片 `&[i32]` 的优势**：它是"对连续内存的借用视图"，数组引用 `&[i32; 5]` 可以自动强制转换（deref coercion）为 `&[i32]`，`Vec<i32>` 的引用 `&vec` 也可以。这让你写一个函数就能服务多种数据结构。
- **`usize` 用于索引**：这是 Rust 的标准做法。`usize` 保证能表示内存中任意对象的地址（和指针同宽度），因此天然适合做数组索引。如果用 `u32`，在 64 位系统上可能无法索引大数组（虽然实际中很少需要，但语义上不匹配）。
- **`Option` 和 `Result` 的组合**：`safe_get` 返回 `Option`（没有值 vs 有值），`swap_elements` 返回 `Result`（成功 vs 错误信息）。两者都是 Rust 中表达"可能失败"的标准方式，替代了 Python 的 try/except。

#### 常见错误

- 在 `safe_get` 中直接用 `arr[index]`——这会在越界时 panic 而非返回 `None`，失去"安全"的意义。
- `swap_elements` 返回 `Result<(), &str>` 但错误信息必须是 `String`（堆分配）——如果用 `&str`，错误信息引用的临时字符串会失效，产生生命周期问题。
- 忘记 `&mut self` —— `swap_elements` 需要 `&mut [i32]`，不能用 `&[i32]`。
- 忘记 `.copied()` —— `get()` 返回 `Option<&i32>`（引用），如果函数返回 `Option<i32>`（值），需要 `.copied()` 解引用并拷贝。

#### 验证方式

```bash
cargo run
# 检查所有测试用例的输出
# 断言 assert_eq! 通过则说明 array_sum 正确
```

---

## Level 3: 综合练习

### 练习 3-1：多维度的类型系统探索

#### 结论

通过系统性地探索 Rust 类型系统的内存布局、类型转换和字符编码，你能建立起对 Rust "零成本抽象"和"显式语义"设计理念的直觉。

#### 思路

将程序分为三个独立函数：
1. `explore_memory()` —— 打印各种类型的大小
2. `explore_type_casting()` —— 类型转换矩阵
3. `explore_char_unicode()` —— 字符与 Unicode 探索

#### 参考实现

```rust
use std::mem::size_of;

/// 探索各种 Rust 类型的内存占用。
///
/// 打印整数类型、浮点类型、布尔、字符、元组、数组的内存大小，
/// 观察：
/// - 整数类型大小严格等于其位宽 / 8
/// - char 固定 4 字节（Unicode 标量值）
/// - 元组大小可能大于元素大小之和（因对齐填充）
/// - 数组大小 = 元素大小 × 元素数量
fn explore_memory() {
    println!("═══════════════════════════════════");
    println!("  第一部分：内存布局探索");
    println!("═══════════════════════════════════\n");

    // --- 整数类型 ---
    println!("── 整数类型 ──");
    println!("  i8:    {} 字节", size_of::<i8>());
    println!("  i16:   {} 字节", size_of::<i16>());
    println!("  i32:   {} 字节", size_of::<i32>());
    println!("  i64:   {} 字节", size_of::<i64>());
    println!("  i128:  {} 字节", size_of::<i128>());
    println!("  isize: {} 字节 (指针宽度)", size_of::<isize>());
    println!("  u8:    {} 字节", size_of::<u8>());
    println!("  u16:   {} 字节", size_of::<u16>());
    println!("  u32:   {} 字节", size_of::<u32>());
    println!("  u64:   {} 字节", size_of::<u64>());
    println!("  u128:  {} 字节", size_of::<u128>());
    println!("  usize: {} 字节 (指针宽度)", size_of::<usize>());

    // --- 浮点数 & bool & char ---
    println!("\n── 浮点数 & bool & char ──");
    println!("  f32:   {} 字节", size_of::<f32>());
    println!("  f64:   {} 字节", size_of::<f64>());
    println!("  bool:  {} 字节", size_of::<bool>());
    println!("  char:  {} 字节 (Unicode 标量值，始终 4 字节)", size_of::<char>());

    // --- 单元元组 ---
    println!("\n── 单元元组 ──");
    println!("  ():    {} 字节 (零大小类型 ZST)", size_of::<()>());

    // --- 元组大小随元素变化 ---
    println!("\n── 元组大小 ──");
    println!("  (i32,):          {} 字节", size_of::<(i32,)>());
    println!("  (i32, i32):      {} 字节", size_of::<(i32, i32)>());
    println!("  (i32, i32, i32): {} 字节", size_of::<(i32, i32, i32)>());
    // 观察：3 个 i32 刚好 12 字节，说明没有填充（i32 对齐到 4 字节，自然对齐）

    // --- 数组大小 ---
    println!("\n── 数组大小 ──");
    println!("  [u8; 0]:  {} 字节 (ZST 数组)", size_of::<[u8; 0]>());
    println!("  [u8; 1]:  {} 字节", size_of::<[u8; 1]>());
    println!("  [u8; 16]: {} 字节 (16 × 1)", size_of::<[u8; 16]>());
    println!("  [u8; 64]: {} 字节 (64 × 1)", size_of::<[u8; 64]>());

    // --- 字段顺序对大小的影响 ---
    println!("\n── 结构体填充（字段顺序影响）──");
    println!(
        "  (bool, char):     {} 字节 (bool=1, char=4, 对齐导致填充)",
        size_of::<(bool, char)>()
    );
    println!(
        "  (char, bool):     {} 字节 (char=4, bool=1, 对齐可能不同)",
        size_of::<(char, bool)>()
    );
    // (bool, char): bool 1字节后需要对齐到char的4字节边界 → 3字节填充 → 共8字节
    // (char, bool): char 4字节 + bool 1字节 = 5字节，但元组对齐到最大元素(char=4字节) → 8字节

    println!("\n📝 小结：");
    println!("  - 整数大小 = 位宽 / 8，精确一致");
    println!("  - char 始终 4 字节（内部是 u32）");
    println!("  - 元组可能有对齐填充，顺序影响总大小");
    println!("  - 数组大小 = 元素大小 × 元素数量（无额外开销）");
}

/// 探索 i32, u8, f64, char 四个类型之间的 as 转换。
///
/// 打印每种转换的源值、目标值和任何截断/损失信息。
fn explore_type_casting() {
    println!("\n═══════════════════════════════════");
    println!("  第二部分：类型转换矩阵");
    println!("═══════════════════════════════════\n");

    let sources: [(i32, u8, f64, char); 4] = [
        // i32,        u8,       f64,        char
        (300_i32, 200_u8, 3.14_f64, 'A'),
        (-50_i32, 100_u8, -2.5_f64, '中'),
        (65_i32, 66_u8, 0.0_f64, '\0'),
        (0_i32, 255_u8, 1e20_f64, '🦀'),
    ];

    println!("  转换方向         | 源值               → 目标值            | 说明");
    println!("  ----------------+--------------------+--------------------+------------------");

    for (i32_val, u8_val, f64_val, ch_val) in &sources {
        // i32 → u8 (截断)
        let i32_to_u8 = *i32_val as u8;
        println!(
            "  i32 → u8        | {:<18} → {:<18} | 可能截断，{} vs {}",
            i32_val, i32_to_u8, i32_val,
            if *i32_val >= 0 && *i32_val <= 255 { "安全" } else { "⚠ 截断" }
        );

        // i32 → f64 (可能丢失精度)
        let i32_to_f64 = *i32_val as f64;
        println!("  i32 → f64       | {:<18} → {:<18.2} | 安全（f64 能精确表示 i32 范围）", i32_val, i32_to_f64);

        // i32 → char
        let i32_to_char = char::from_u32(*i32_val as u32);
        println!(
            "  i32 → char      | {:<18} → {:<18?} | {}",
            i32_val, i32_to_char,
            if i32_to_char.is_some() { "有效字符" } else { "⚠ 无效 Unicode" }
        );

        // u8 → i32 (安全，扩展)
        let u8_to_i32 = *u8_val as i32;
        println!("  u8 → i32        | {:<18} → {:<18} | 安全（扩展，不会丢失）", u8_val, u8_to_i32);

        // u8 → f64 (安全)
        let u8_to_f64 = *u8_val as f64;
        println!("  u8 → f64        | {:<18} → {:<18.2} | 安全", u8_val, u8_to_f64);

        // u8 → char
        let u8_to_char = *u8_val as char;
        println!("  u8 → char       | {:<18} → {:<18} | ASCII 范围内安全", u8_val, u8_to_char);

        // f64 → i32 (截断小数)
        let f64_to_i32 = *f64_val as i32;
        println!(
            "  f64 → i32       | {:<18.4} → {:<18} | 小数截断，{}",
            f64_val, f64_to_i32,
            if *f64_val > i32::MAX as f64 || *f64_val < i32::MIN as f64 { "⚠ 溢出！" } else { "值域内" }
        );

        // f64 → u8
        let f64_to_u8 = *f64_val as u8;
        println!(
            "  f64 → u8        | {:<18.4} → {:<18} | {}",
            f64_val, f64_to_u8,
            if *f64_val >= 0.0 && *f64_val <= 255.0 { "安全" } else { "⚠ 截断/溢出" }
        );

        // f64 → char
        let f64_to_char = char::from_u32(*f64_val as u32);
        println!(
            "  f64 → char      | {:<18.4} → {:<18?} | {}",
            f64_val, f64_to_char,
            if f64_to_char.is_some() { "有效字符" } else { "⚠ 无效 Unicode" }
        );

        // char → i32
        let ch_to_i32 = *ch_val as i32;
        println!("  char → i32      | {:<18} → {:<18} | 安全（char → u32 → i32）", ch_val, ch_to_i32);

        // char → u8
        let ch_to_u8 = *ch_val as u8;
        println!(
            "  char → u8       | {:<18} → {:<18} | {}",
            ch_val, ch_to_u8,
            if (*ch_val as u32) <= 255 { "安全（ASCII 范围内）" } else { "⚠ 截断！丢失 Unicode 信息" }
        );

        // char → f64
        let ch_to_f64 = *ch_val as u32 as f64;
        println!("  char → f64      | {:<18} → {:<18.2} | 安全", ch_val, ch_to_f64);

        println!(); // 每组之间空行
    }

    println!("📝 小结：");
    println!("  - `as` 转换从不 panic，但可能静默截断/溢出");
    println!("  - 扩大转换（小→大）通常安全");
    println!("  - 缩小转换（大→小）可能丢失信息");
    println!("  - char → u8 对于非 ASCII 字符会截断");
    println!("  - 生产代码建议用 `From`/`TryFrom` trait 而非 `as`");
}

/// 探索字符类型与 Unicode 编码。
///
/// 演示：
/// - Rust char 是 4 字节 Unicode 标量值
/// - 中文字符、emoji、ASCII 的 Unicode 码位对比
/// - char 转 u8 的截断问题
fn explore_char_unicode() {
    println!("\n═══════════════════════════════════");
    println!("  第三部分：字符与 Unicode 探索");
    println!("═══════════════════════════════════\n");

    // 包含多种字符类型的数组
    let chars: [char; 8] = [
        'A',           // ASCII 字母
        '9',           // ASCII 数字
        ' ',           // ASCII 空格
        '你',          // 中文（CJK 统一表意文字）
        '好',          // 中文
        '🦀',          // emoji（螃蟹）
        'π',           // 数学符号（希腊字母）
        '∞',           // 数学符号（无穷大）
    ];

    println!("  字符  | Unicode 码位 (u32) | 转 u8 后     | 安全?  | 说明");
    println!("  ------+---------------------+--------------+--------+-------------------");

    for ch in &chars {
        let codepoint = *ch as u32;
        let as_u8 = *ch as u8;
        let safe = codepoint <= 255;
        let category = match codepoint {
            0..=127 => "ASCII",
            128..=255 => "Latin-1 扩展",
            256..=0xFFFF => "BMP（基本多语言平面）",
            _ => "Supplementary（补充平面）",
        };

        println!(
            "  {:>4}   | U+{:04X} ({:<10}) | {:<12} | {} | {}",
            ch, codepoint, codepoint, as_u8,
            if safe { "安全  " } else { "⚠截断" },
            category,
        );
    }

    println!("\n📝 小结：");
    println!("  - Rust 的 char 是 4 字节（u32），表示一个 Unicode 标量值");
    println!("  - 与 C 语言的 1 字节 char 完全不同");
    println!("  - ASCII 字符（U+0000-U+007F）可安全转为 u8");
    println!("  - 中文、emoji 等非 ASCII 字符转为 u8 会丢失信息");
    println!("  - char 不能直接参与算术运算（需要先转为 u32）");
    println!("  - 字符串内部的 UTF-8 编码长度不一：'A'=1字节，'你'=3字节，'🦀'=4字节");
}

fn main() {
    explore_memory();
    explore_type_casting();
    explore_char_unicode();

    println!("\n═══════════════════════════════════");
    println!("  总结");
    println!("═══════════════════════════════════");
    println!("  从这三个部分的实验中学到：");
    println!("  1. Rust 类型的内存占用精确可控，没有隐式开销");
    println!("  2. `as` 转换是弱类型的逃生舱，但不安全——");
    println!("     生产代码应优先使用 From/TryFrom");
    println!("  3. char ≠ u8，理解这一点是理解 Unicode 的关键");
    println!("  4. 对齐填充是性能与空间的权衡——");
    println!("     CPU 访问对齐的数据更快");
}
```

#### 为什么这样设计

- **内存布局探索**：`size_of` 是编译期确定的——编译器在编译时就知道每种类型在栈上的布局。这体现了 Rust 的"零成本抽象"：抽象（元组、结构体、枚举）在运行时没有额外的 tagging 或 boxing 开销。
- **类型转换矩阵**：`as` 关键字是 Rust 中唯一的隐式类型转换方式（虽然它需要显式写出），但它不提供安全保障——溢出和截断不会触发 panic。这促使开发者使用更安全的 `From`/`TryFrom` trait。
- **字符与 Unicode**：`char` 的内部表示是 `u32`（Unicode 标量值），这确保了 Rust 原生支持所有 Unicode 字符（包括 emoji）。但这也意味着 `char` 不能直接映射到 C 的 1 字节 `char`。

#### 常见错误

- 以为 `size_of::<(bool, char)>()` 等于 `1 + 4 = 5` 字节——实际是 8 字节（因为 `char` 需要 4 字节对齐，`bool` 后面有 3 字节的 padding）。
- 用 `as` 做 float → int 转换时忘记可能溢出（如 `1e20_f64 as i32` 会得到 `i32::MAX` 的饱和值）。
- `char::from_u32()` 对于某些码位范围（如 surrogate pairs 0xD800-0xDFFF）会返回 `None`——不是所有 u32 值都是有效的码位。

#### 验证方式

```bash
cargo build
cargo run
# 检查所有三部分的输出，确认没有 panic
```

---

## 思考题

### Rust 为什么设计"默认不可变"？

#### 1. 安全性

默认不可变从语言层面消除了"意外修改"这类最隐蔽的 bug。考虑以下场景：

```python
# Python：默认可变
config = {"timeout": 30, "retries": 3}
# ... 中间 200 行代码 ...
process_request(config)  # 意外修改了 config["timeout"]
# ... 更多代码 ...
do_something(config)     # 使用的已经是修改后的值！
```

在 Python 中，你无法从代码中直接知道 `process_request` 是否会修改 `config`——必须查看函数文档或源码。而在 Rust 中：

```rust
let config = Config { timeout: 30, retries: 3 };
process_request(&config);  // 不可变借用，编译器保证不会修改
do_something(&config);     // 安全——config 一定没变
```

如果你想让 `process_request` 修改 config，必须显式传递 `&mut config`，这在调用处就能看到。

#### 2. 并发

不可变性是并发安全的基础原则之一。Rust 的 `Send` + `Sync` trait 体系与默认不可变协同工作：

- 不可变数据可以安全地在任意多个线程间共享（`&T` 是 `Sync` 的）。
- 可变数据只能被一个线程独占访问（`&mut T` 不是 `Sync` 的）。

这种"不可变共享"和"可变独占"的组合，使得 Rust 在编译期就能防止数据竞争。下面的代码：

```rust
let data = vec![1, 2, 3];
// 隐式不可变——可以安全地给多个线程共享
std::thread::scope(|s| {
    s.spawn(|| println!("{:?}", &data));
    s.spawn(|| println!("{:?}", &data));
});
// 如果 data 是 let mut 且被一个线程可变借用，
// 编译器会阻止其他线程同时访问它
```

#### 3. 编译器优化

当编译器知道一个值不会改变时，它可以做许多优化：

- **常量传播（Constant Propagation）**：不可变变量在编译后的行为类似常量，编译器可以在多处直接使用其值而无需重复加载。
- **死代码消除（Dead Code Elimination）**：如果编译器能证明一个不可变变量在某个代码路径上永远不会被使用，可以直接消除它。
- **别名分析（Alias Analysis）**：C/C++ 中两个指针可能指向同一个位置（aliasing），这限制了优化。Rust 的 `&T`（不可变引用）可以允许多个别名，但 `&mut T`（可变引用）保证是唯一的——这使得 LLVM 可以应用更激进的优化。GCC/Clang 的 `restrict` 关键字和 Rust 的 `&mut` 有相似的作用。

#### 4. 代码可读性

```rust
let mut x = 5;  // "注意！这个变量会被修改"
let y = 10;     // "这个变量在整个作用域内保持不变，你可以信任它"
```

`mut` 关键字是一种**信号**——它告诉你"这个变量的值会变化，阅读下方代码时请注意追踪它的变化"。在没有 `mut` 时，你可以安全地假设变量值一旦绑定就不会改变。这种信号在 Python 中完全缺失——你必须阅读函数内部的每一行才能知道 `x` 是否被修改。

#### 5. 个人体验

以下是一个在 Python 中常见的 bug 模式：

```python
# Python：bug 示例
def add_user_to_group(user_list, group):
    user_list.append(user)  # 💥 意外修改了调用者传入的列表！
    group.members.append(user)

users = ["Alice", "Bob"]
add_user_to_group(users, admin_group)
print(users)  # ["Alice", "Bob", "Charlie"] —— 被意外修改了！
```

在 Rust 中，同样的意图需要显式表达：

```rust
fn add_user_to_group(user_list: &mut Vec<User>, group: &mut Group, user: User) {
    user_list.push(user);       // 调用者必须传 &mut，知道会被修改
    group.members.push(user);
}

let mut users = vec![alice, bob];  // mut 标记了可改性
add_user_to_group(&mut users, &mut admin_group, charlie);
// 调用处明确知道 users 和 admin_group 会被修改
```

如果函数只需要读取而不修改，使用 `&Vec<User>` 而非 `&mut`，调用者可以放心传入任何引用——包括那些在其他地方被借用的引用。

**总结**：Rust 的默认不可变是一种"契约式编程"（Design by Contract）的体现——它让你在语言层面编码了"这个值会不会变"的意图，并由编译器强制执行。这不仅消除了 bug，还让并发编程变得更安全，并给了编译器更多优化空间。对于 Python 开发者来说，这可能是学习 Rust 时最需要适应的变化之一，但它带来的安全性回报是巨大的。

---

## 推荐检查命令

```bash
cargo build   # 编译检查
cargo run     # 运行程序
cargo clippy  # 更严格的代码检查
cargo fmt     # 格式化
```
