# 练习：Unsafe Rust 与 FFI

> 通过动手练习理解 Unsafe Rust 的安全边界与 FFI 基础
>
> 核心原则：**用最小的 unsafe 内核构建安全的抽象**

---

## 练习等级说明

- **Level 1** -- 基础练习，巩固语法和概念，预计 10-20 分钟
- **Level 2** -- 综合练习，设计安全的 API 包装 unsafe 代码，预计 30-45 分钟
- **Level 3** -- 挑战练习，涉及 FFI 和实际 C 库调用，预计 60-90 分钟

---

## Level 1: 基础练习

### 练习 1.1: 裸指针基础操作

**目标：** 理解裸指针的创建、解引用和指针运算。

**任务：**

1. 创建一个 `[i32; 8]` 数组，使用裸指针遍历所有元素并计算总和
2. 使用 `ptr.add(offset)` 方法定位元素
3. 比较裸指针版本和 safe 索引访问版本的结果
4. 在 unsafe 块前添加 SAFETY 注释

**要求：**

```rust
fn sum_with_raw_pointer(arr: &[i32]) -> i32 {
    let ptr = arr.as_ptr();
    let len = arr.len();
    let mut sum = 0;
    // 你的代码：用裸指针遍历数组，计算总和
    // SAFETY: ...
    sum
}
```

**验证命令：**

```bash
cargo test test_exercise_1_1
```

**预期结果：**
- 裸指针版本和 `arr.iter().sum()` 结果一致
- 每个 unsafe 块都有 SAFETY 注释
- 代码通过 `cargo clippy` 检查

**提示：** 创建裸指针是安全的，只有解引用才需要 unsafe。`arr.as_ptr()` 返回 `*const i32`，对 const 裸指针的解引用读取是安全的（前提是数据有效）。

---

### 练习 1.2: 编写你的第一个 unsafe 函数

**目标：** 学习如何正确定义和调用 unsafe 函数。

**任务：**

1. 定义一个 unsafe 函数 `unsafe fn zero_memory(ptr: *mut u8, len: usize)`
   - 使用 `std::ptr::write_bytes` 将指定内存区域填充为 0
2. 编写一个安全的包装函数 `fn clear_buffer(buf: &mut [u8])`
   - 内部调用 `zero_memory`
   - 确保所有输入验证在 safe 层完成
3. 测试：用不同长度的 `Vec<u8>` 调用 `clear_buffer`

**要求：**

```rust
/// # Safety
///
/// 调用者必须确保：
/// - `ptr` 非空，且指向至少 `len` 字节的有效内存
/// - 内存区域未被其他线程同时访问
unsafe fn zero_memory(ptr: *mut u8, len: usize) {
    // 你的代码
    // SAFETY: ...
}

/// 安全的包装函数：将缓冲区清零。
fn clear_buffer(buf: &mut [u8]) {
    // 你的代码（不需要 unsafe 签名）
}
```

**验证命令：**

```bash
cargo test test_exercise_1_2
```

**提示：** `std::ptr::write_bytes(ptr, 0, len)` 用指定字节值填充内存。这是一个 unsafe 函数，所以需要在 `zero_memory` 内部再次使用 unsafe 块（Rust 2024 要求）。

---

### 练习 1.3: 理解 union 的基本用法

**目标：** 掌握 union 的声明和字段访问。

**任务：**

1. 定义一个 `#[repr(C)]` union `Number`，包含：
   - `int_val: i32`
   - `float_val: f32`
   - `bytes: [u8; 4]`
2. 创建一个函数，将 `f32` 的字节表示打印为十六进制
3. 创建一个函数，将 `i32` 解释为 `f32`（注意：这在实际代码中通常是 bug）

**要求：**

```rust
#[repr(C)]
union Number {
    int_val: i32,
    float_val: f32,
    bytes: [u8; 4],
}

fn print_float_bytes(value: f32) {
    // 你的代码
    // 将 f32 存储到 union，然后以 bytes 形式读取
}

fn interpret_i32_as_f32(value: i32) -> f32 {
    // 你的代码
    // 注意添加注释说明这通常不安全
}
```

**验证命令：**

```bash
cargo test test_exercise_1_3
```

**提示：** 访问 union 的非活跃字段不是未定义行为（在 Rust 中），但结果可能没有意义。关键是你要**知道**哪个字段是活跃的。

---

## Level 2: 综合练习

### 练习 2.1: 实现安全的 Vec 简化版

**目标：** 学习如何用 unsafe 代码构建安全的数据结构抽象。

**任务：**

实现一个简化版的动态数组 `SimpleVec<T>`，支持以下操作：

1. `new() -> Self` -- 创建空数组
2. `push(&mut self, value: T)` -- 添加元素
3. `pop(&mut self) -> Option<T>` -- 移除并返回最后一个元素
4. `len(&self) -> usize` -- 返回长度
5. `get(&self, index: usize) -> Option<&T>` -- 安全索引访问
6. 实现 `Drop` trait 来正确释放内存

**内部结构：**

```rust
struct SimpleVec<T> {
    ptr: *mut T,      // 指向堆内存的裸指针
    len: usize,       // 当前元素数量
    capacity: usize,  // 当前分配的容量
}
```

**关键要求：**

- 所有 unsafe 代码必须在内部，对外 API 完全安全
- 每个 unsafe 操作必须有 SAFETY 注释
- 正确处理内存分配和释放
- 正确处理 ZST（零大小类型）或添加 `#[allow]` 限制
- 正确处理 panicking 时的内存安全（考虑 `Drop` 守卫）

**提示：**
- 使用 `std::alloc::{alloc, dealloc, realloc, Layout}` 管理内存
- 使用 `std::ptr::{read, write, add}` 操作元素
- 使用 `std::mem::ManuallyDrop` 或 `std::ptr::drop_in_place` 处理 Drop

**验证命令：**

```bash
cargo test test_exercise_2_1
```

**测试用例：**

```rust
#[test]
fn test_simple_vec() {
    let mut v = SimpleVec::new();
    assert_eq!(v.len(), 0);

    v.push(1);
    v.push(2);
    v.push(3);
    assert_eq!(v.len(), 3);

    assert_eq!(v.get(0), Some(&1));
    assert_eq!(v.get(2), Some(&3));
    assert_eq!(v.get(3), None);

    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.len(), 2);

    v.push(4);
    assert_eq!(v.pop(), Some(4));
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
}
```

---

### 练习 2.2: 从 C 库调用开始的实际 FFI

**目标：** 实际调用 C 标准库函数，学习 FFI 绑定编写。

**任务：**

1. 在 `build.rs` 中链接 C 数学库（`libm`，或在 Linux 上直接用 `libc`）
2. 声明并调用以下 C 标准库函数：
   - `abs(i32) -> i32` -- 绝对值
   - `sqrt(f64) -> f64` -- 平方根（需要链 `libm`）
3. 创建一个安全的 Rust 包装函数：
   - `fn safe_sqrt(x: f64) -> Option<f64>` -- 对负数返回 None
   - `fn safe_abs(x: i32) -> i32` -- 直接包装
4. 手写 FFI 声明（不要用 `libc` crate，只使用标准库）

**要求：**

```rust
// 手写 extern 块（不使用 libc crate）
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

/// 计算平方根。对负数返回 None。
fn safe_sqrt(x: f64) -> Option<f64> {
    // 你的代码
    // 在 unsafe 块中调用 sqrt
    // 对负数返回 None
}

/// 计算绝对值。总是安全的。
fn safe_abs(x: i32) -> i32 {
    // 你的代码
}
```

**`build.rs` 示例：**

```rust
fn main() {
    // Linux: 数学函数在 libc 中，不需要额外链接
    // 某些平台需要: println!("cargo:rustc-link-lib=m");
}
```

**验证命令：**

```bash
cargo test test_exercise_2_2
cargo run --bin exercise_2_2
```

**测试用例：**

```rust
#[test]
fn test_safe_ffi() {
    assert_eq!(safe_abs(-42), 42);
    assert_eq!(safe_abs(0), 0);
    assert_eq!(safe_abs(100), 100);

    assert_eq!(safe_sqrt(4.0), Some(2.0));
    assert_eq!(safe_sqrt(0.0), Some(0.0));
    assert_eq!(safe_sqrt(-1.0), None);

    // 浮点近似
    let result = safe_sqrt(2.0).unwrap();
    assert!((result - 1.4142135623730951).abs() < 1e-10);
}
```

**提示：**
- FFI 调用天然是 unsafe 的，因为 C 代码不受 Rust 的规则约束
- 安全包装函数应该在 unsafe 调用之前验证所有输入
- `sqrt(-1.0)` 在 C 中返回 `NaN`，但我们应该避免它（返回 None）

---

## Level 3: 挑战练习

### 练习 3.1: Rust 与 C 的双向互操作

**目标：** 从零构建一个完整的 Rust-C 互操作示例，理解双向 FFI。

**任务：**

1. 编写一个 C 源文件 `src/operations.c`，实现：
   - `int32_t add(int32_t a, int32_t b)` -- 加法
   - `int32_t multiply(int32_t a, int32_t b)` -- 乘法
2. 编写 `build.rs` 或 `build.rs` + `cc` crate 来编译 C 代码并链接
3. 在 Rust 中声明并调用这些 C 函数
4. 在 Rust 中实现一个标记为 `#[no_mangle] pub extern "C"` 的函数：
   - `fn rust_fibonacci(n: i32) -> i32` -- 计算第 n 个斐波那契数
5. 在 C 代码中通过声明调用 `rust_fibonacci`
6. 编写测试验证双向调用

**项目结构：**

```
exercise_ffi/
├── Cargo.toml
├── build.rs
├── src/
│   ├── main.rs
│   └── operations.c
```

**C 代码示例（`src/operations.c`）：**

```c
#include <stdint.h>

// 声明将由 Rust 实现的函数
extern int32_t rust_fibonacci(int32_t n);

int32_t add(int32_t a, int32_t b) {
    return a + b;
}

int32_t multiply(int32_t a, int32_t b) {
    return a * b;
}

// 调用 Rust 函数的 C 函数
int32_t call_rust_fibonacci(int32_t n) {
    return rust_fibonacci(n);
}
```

**Rust 代码关键部分：**

```rust
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn multiply(a: i32, b: i32) -> i32;
    fn call_rust_fibonacci(n: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn rust_fibonacci(n: i32) -> i32 {
    // 你的实现
    // 使用迭代而非递归，避免栈溢出
}

/// C 代码调用 Rust fibonacci 的安全包装
fn safe_fibonacci(n: u32) -> u32 {
    assert!(n <= 46, "结果会溢出 i32"); // fib(47) > i32::MAX
    unsafe { call_rust_fibonacci(n as i32) as u32 }
}
```

**`build.rs`：**

```rust
fn main() {
    cc::Build::new()
        .file("src/operations.c")
        .compile("operations");
}
```

**`Cargo.toml` 依赖：**

```toml
[build-dependencies]
cc = "1.0"
```

**验证命令：**

```bash
cargo test test_exercise_3_1
cargo run
```

**测试用例：**

```rust
#[test]
fn test_c_operations() {
    unsafe {
        assert_eq!(add(2, 3), 5);
        assert_eq!(multiply(4, 5), 20);
    }
}

#[test]
fn test_bidirectional_ffi() {
    assert_eq!(safe_fibonacci(0), 0);
    assert_eq!(safe_fibonacci(1), 1);
    assert_eq!(safe_fibonacci(10), 55);
    assert_eq!(safe_fibonacci(20), 6765);
}
```

**提示：**
- 使用 `cc` crate 可以方便地编译 C 代码
- `#[no_mangle]` 保留原始函数名，`extern "C"` 使用 C ABI
- C 代码中通过 `extern` 声明来调用 Rust 函数
- 确保 C 和 Rust 之间的类型匹配（使用 `std::os::raw::*` 或 `core::ffi::*`）

---

## 思考题

### 思考题: Unsafe Rust 的哲学

**问题：**

想象你正在为一个 Rust 项目做代码审查。你发现一位初级开发者提交了以下代码：

```rust
fn get_two_elements_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    // 开发者说："借用检查器不让我返回两个可变引用，
    // 但我知道 i 和 j 不相等，所以用 unsafe 绕过它"
    let ptr = slice.as_mut_ptr();
    unsafe {
        (&mut *ptr.add(i), &mut *ptr.add(j))
    }
}
```

请**详细**回答以下问题：

1. **这个函数有什么问题？** （至少列出 3 个具体问题）
   - 提示：考虑边界检查、别名规则、生命周期

2. **如果这是生产代码，可能引发什么 bug？**
   - 提示：考虑编译器优化可能做出的假设

3. **正确的实现应该是什么样的？** （不依赖标准库的 `split_at_mut`）
   - 至少用两种方法：
     a. 使用 `split_at_mut` 两次
     b. 完全不用 unsafe 实现

4. **这个函数签名本身有什么设计问题？** 为什么标准库不提供这样的函数？
   - 提示：考虑 API 设计的"坑"（pitfalls）

5. **给这位开发者写一段 Code Review 评论**，解释：
   - 为什么 unsafe 在这里不合适
   - 正确的解决方案是什么
   - 未来如何识别类似的错误模式

**要求：**
- 每个问题至少 50 字的回答
- 提供代码示例支持你的观点
- 引用 Rust 的安全规则或官方文档

---

## 推荐练习命令

```bash
# 编译并运行所有练习
cargo build
cargo run

# 运行特定练习的测试
cargo test test_exercise_1_1
cargo test test_exercise_2_1
cargo test test_exercise_3_1

# 使用 clippy 检查代码质量
cargo clippy

# 使用 Miri 检测 unsafe 代码中的 UB（需要 nightly）
# rustup toolchain install nightly
# cargo +nightly miri test

# 检查项目中的 unsafe 代码比例
# cargo install cargo-geiger
# cargo geiger

# 编译 Release 版本测试性能
cargo build --release
cargo run --release
```

---

## 练习检查清单

在完成每个练习后，请确认以下条目：

- [ ] 所有 unsafe 块前都有 `// SAFETY:` 注释
- [ ] unsafe 代码被限制在最小范围内
- [ ] 对外暴露的是安全 API（不需要 unsafe 的签名）
- [ ] 代码通过了 `cargo build` 和 `cargo test`
- [ ] 代码通过了 `cargo clippy` 检查
- [ ] 没有使用 unsafe 来"绕过"借用检查器
- [ ] 对于 FFI 练习：验证了所有来自 C 的输入

---

## 参考答案提示

如果你遇到困难，可以参考以下提示而非直接查看完整答案：

1. **练习 1.1**：`arr.as_ptr()` 获取 `*const i32`，用 `.add(i)` 偏移，`*ptr` 解引用读取
2. **练习 1.2**：在 `zero_memory` 内部需要 `unsafe { std::ptr::write_bytes(ptr, 0, len); }`
3. **练习 1.3**：`Number { float_val: value }.bytes` 读取（在 unsafe 中）
4. **练习 2.1**：参考 `std::vec::Vec` 源码的结构，但大大简化
5. **练习 2.2**：`sqrt` 在大多数 Linux 上不需要额外链接（在 libc 中）
6. **练习 3.1**：`cc` crate 会自动处理编译和链接

---

**记住：Unsafe Rust 不是"更自由的 Rust"，而是"责任更大的 Rust"。每一个 unsafe 块都是你对编译器的承诺。**

---

## 迁移思维练习

> 以下问题帮助你思考 C/C++ 中的底层操作在 Rust 中何时需要 unsafe，以及如何构建安全的抽象边界。

### 问题 1：哪些 C/C++ 中的常见操作在 Rust 中需要 unsafe？

C/C++ 程序员几乎每天都会做的事情：解引用裸指针、调用汇编/内联函数、操作硬件寄存器（MMIO）、将一段内存 reinterpret 为另一种类型、在 union 中读取非活跃字段、在多线程中共享可变数据而不加锁……请逐一思考这些操作在 Rust 中为什么被标记为 unsafe。它们有什么共同特征？Rust 的 Safe 子集提供了什么保证，而这些操作打破了这个保证？

**提示**：Safe Rust 保证"内存安全"和"无数据竞争"——unsafe 代码可以临时突破这些保证，但开发者需要手动维护编译器无法自动验证的不变式（invariants）。

### 问题 2：为什么 unsafe 代码应该封装在安全抽象中？

Rust 社区有一句口号："unsafe 代码应该是一个实现的细节，而不是 API 的特征"。如果你把 unsafe 暴露为公开 API（让调用方也需要写 unsafe 才能使用），会发生什么问题？请对比 C/C++ 的实践——C 函数大多"不安全"但不需要标注，调用方需要靠文档和约定来确保安全；Rust 将 unsafe 标注为类型系统的一部分，这如何改变了你在设计 API 时对安全边界的思考？设想你写了一个使用 FFI 调用 C 库的 wrapper crate，你的 crate 的用户应该在什么条件下才需要写 unsafe？

**提示**：安全抽象的目标是让 unsafe 调用方不需要知道内部实现——比如 `Vec<T>` 内部有大量 unsafe，但它的所有公开 API 都是 safe 的。
