# Chapter 24: Unsafe Rust 与 FFI 概述

> 理解 Rust 的安全边界：为什么需要 unsafe，以及如何正确地使用它

> 📚 **相关章节**：[04 栈堆RAII](../04_stack_heap_and_raii/) | [06 借用](../06_references_borrowing_slices/) | [16 生命周期](../16_lifetimes/) | [23 宏](../23_macros/)

---

## 目录

1. [Safe Rust vs Unsafe Rust](#1-safe-rust-vs-unsafe-rust)
2. [Unsafe 到底是什么（以及不是什么）](#2-unsafe-到底是什么以及不是什么)
3. [Unsafe Rust 的五大超能力](#3-unsafe-rust-的五大超能力)
4. [安全抽象模式：用安全 API 包装 unsafe 代码](#4-安全抽象模式用安全-api-包装-unsafe-代码)
5. [编写 SAFETY 注释](#5-编写-safety-注释)
6. [何时应该使用 Unsafe](#6-何时应该使用-unsafe)
7. [何时不应该使用 Unsafe](#7-何时不应该使用-unsafe)
8. [FFI：外部函数接口](#8-ffi外部函数接口)
9. [Extern "C" 和链接](#9-extern-c-和链接)
10. [为什么初学者不应该用 unsafe "修复"借用检查器错误](#10-为什么初学者不应该用-unsafe-修复借用检查器错误)
11. [真实世界中的 unsafe：标准库内部](#11-真实世界中的-unsafe标准库内部)
12. [核心术语对照](#12-核心术语对照)
13. [Python 对照](#13-python-对照)
14. [Python、C 与 C++ 对照](#14-pythonc-与-c-对照)
15. [延伸阅读](#15-延伸阅读)

---

## 1. Safe Rust vs Unsafe Rust

### 1.1 Safe Rust

Safe Rust 是 Rust 的默认模式。在 safe Rust 中，编译器通过以下机制**静态地**保证内存安全：

- **所有权系统（Ownership）**：每个值有唯一的所有者
- **借用检查器（Borrow Checker）**：在编译时防止悬垂引用和数据竞争
- **生命周期（Lifetimes）**：确保引用不会比它指向的数据活得更久
- **类型系统（Type System）**：防止类型混淆和非法转换

在这些规则的约束下，safe Rust 代码**不可能**产生以下 bug：

- 空指针解引用（Null pointer dereference）
- 悬垂指针（Dangling pointer）
- 双重释放（Double free）
- 使用已释放的内存（Use-after-free）
- 数据竞争（Data race）
- 缓冲区溢出（Buffer overflow）（在不使用 unsafe 的情况下）

### 1.2 Unsafe Rust

然而，有些操作是 safe Rust 无法表达的，例如：

- 与操作系统底层 API 交互（通常暴露为 C 函数）
- 直接操作硬件（内存映射 I/O）
- 实现某些高性能数据结构（如自引用结构、侵入式链表）
- 与其他语言（C/C++）进行互操作

这时就需要 **Unsafe Rust**。Unsafe Rust 为程序员提供了额外的五种"超能力"，但同时要求程序员手动维护 Rust 的安全不变量。

### 1.3 关键区别

| 特性 | Safe Rust | Unsafe Rust |
|------|-----------|-------------|
| 借用检查 | 编译器严格检查 | **仍然检查！**借用规则不放松 |
| 类型检查 | 完全类型安全 | 完全类型安全（union 除外） |
| 裸指针解引用 | 不允许 | 允许 |
| 可变静态变量 | 不允许 | 允许 |
| 调用 unsafe 函数 | 必须在 unsafe 块中 | 可以调用 |
| 数据竞争防护 | 编译时保证 | **程序员手动保证** |
| 内存泄漏 | 可能（如 Rc 循环） | 可能 |

---

## 2. Unsafe 到底是什么（以及不是什么）

### 2.1 三个最常见的误解

#### 误解一："unsafe 关闭了借用检查器"

**错误！** 借用检查器在 unsafe 块内**仍然正常工作**。

```rust
let mut x = 5;
let r1 = &mut x;
unsafe {
    // let r2 = &mut x; // 编译错误！借用规则仍然生效
    *r1 = 10;
}
println!("{}", r1);
```

即使在 unsafe 块内，你也不能创建同一数据的两个可变引用。借用检查器只对**裸指针**"网开一面"——因为裸指针完全不受借用检查器的约束。

#### 误解二："unsafe 禁用了所有安全检查"

**错误！** Unsafe 只解锁了五种特定操作。其他所有安全检查——类型检查、模式匹配、所有权规则、生命周期检查——全部保持有效。

#### 误解三："unsafe 代码一定不安全"

**错误！** 正确编写的 unsafe 代码是完全安全的。标准库中大量使用 unsafe 来实现高效的抽象，但暴露给用户的是完全安全的 API。例如 `Vec<T>`、`Box<T>`、`Rc<T>` 的内部实现都包含 unsafe 代码，但你使用它们时不需要任何 unsafe 块。

### 2.2 Unsafe 的真正含义

Unsafe 意味着：

> **"编译器，我知道你无法静态验证这段代码的安全性。我已经手动审查过了，我保证它不会违反 Rust 的内存安全规则。请信任我。"**

Unsafe 将验证安全性的责任**从编译器转移到了程序员**。

---

## 3. Unsafe Rust 的五大超能力

使用 `unsafe` 关键字可以执行以下五种在 safe Rust 中被禁止的操作：

### 3.1 解引用裸指针（Dereference Raw Pointers）

```rust
let x = 42;
let ptr: *const i32 = &x as *const i32;  // 安全：创建裸指针
// println!("{}", *ptr);                 // 编译错误：解引用裸指针是 unsafe 的
unsafe {
    println!("{}", *ptr);                // 正确：在 unsafe 块中解引用
}
```

**裸指针 vs 引用：**

| 特性 | 引用 `&T` / `&mut T` | 裸指针 `*const T` / `*mut T` |
|------|---------------------|------------------------------|
| 借用检查 | 由编译器保证 | 无检查 |
| 生命周期 | 有明确生命周期 | 无生命周期 |
| 空指针 | 不可能为空 | 可以为 null |
| 对齐要求 | 编译器保证 | 不保证 |
| 创建方式 | 借用 | `&x as *const T`, `Box::into_raw()`, `std::ptr::null()` |
| 自动解引用 | 自动 | 需要手动解引用 |

### 3.2 调用 Unsafe 函数或方法

```rust
/// # Safety
/// 调用者必须确保 ptr 非空且指向有效的已初始化 i32。
unsafe fn read_from_ptr(ptr: *const i32) -> i32 {
    *ptr
}

let value = 42;
// SAFETY: ptr 指向有效的 i32 变量 value，value 在 unsafe 块期间存活。
let result = unsafe { read_from_ptr(&value) };
```

**关键约定：** 每个 `unsafe fn` 应该用文档注释说明其 Safety 前置条件（即调用者必须保证什么）。

### 3.3 访问或修改可变静态变量

```rust
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;  // 读取和写入 mutable static 必须使用 unsafe
    }
}
```

**为什么这是危险的？**

可变静态变量是全局的。在多线程环境中，多个线程可能同时读写它，导致**数据竞争（Data Race）**。数据竞争在 Rust 中属于**未定义行为（Undefined Behavior）**。

**推荐替代方案：**

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);  // 不需要 unsafe！

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);     // 完全安全
}
```

### 3.4 实现 Unsafe Trait

```rust
// Send 和 Sync 是 unsafe trait
// 编译器不会自动验证你的实现是否正确
unsafe impl Send for MyStruct {}
unsafe impl Sync for MyStruct {}
```

**常见的 unsafe trait：**
- `Send`：标记类型可以安全地在线程间转移所有权
- `Sync`：标记类型的不可变引用可以安全地在线程间共享

当你的类型包含裸指针时，编译器不会自动实现 `Send` 和 `Sync`（因为它无法验证裸指针指向的数据是否线程安全）。如果你确信自己的实现是正确的，可以通过 `unsafe impl` 手动实现。

### 3.5 访问 Union 的字段

```rust
#[repr(C)]
union IntOrFloat {
    int_val: i32,
    float_val: f32,
}

let u = IntOrFloat { int_val: 42 };
unsafe {
    println!("{}", u.int_val);  // 访问 union 字段是 unsafe 的
}
```

**为什么这是 unsafe 的？**

Union 的所有字段共享同一块内存。编译器不跟踪哪个字段是"活跃的"（即最后被写入的）。如果你读取了错误的字段，你会得到未定义行为。类比 Python 的 `struct` 模块或 C 的 `union`。

---

## 4. 安全抽象模式：用安全 API 包装 Unsafe 代码

这是 Unsafe Rust 中**最重要**的设计模式。

### 4.1 核心原则

> 将 unsafe 代码限制在最小的"unsafe 内核"中，然后用安全的 API 暴露给外部。

```rust
/// 一个安全的分割可变切片的函数。
/// 签名是安全的（没有 unsafe 关键字）。
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);  // 安全检查

    // 内部的 unsafe 代码由 assert 保证安全
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### 4.2 为什么这样设计？

| 层级 | 职责 |
|------|------|
| **Safe API（对外）** | 验证所有输入，确保不会违反安全不变量 |
| **Unsafe 内核（内部）** | 执行无法用 safe Rust 表达的操作，信任上层已验证 |

这样做的好处：

1. **调用者不需要了解 unsafe**：他们只需调用安全的函数
2. **bug 的表面面积最小**：只有 unsafe 内核中的代码可能出错
3. **审查范围集中**：代码审查时只需重点关注 unsafe 内核
4. **可测试性**：安全 API 可以正常测试

### 4.3 标准库中的例子

```rust
// Vec::push 是安全的 API
let mut v = vec![1, 2, 3];
v.push(4);  // 完全安全！

// 但 Vec 内部使用 unsafe 来管理内存
// （简化版内部实现示意）：
// unsafe {
//     let end = self.as_mut_ptr().add(self.len);
//     std::ptr::write(end, value);
// }
```

`Box<T>`、`Rc<T>`、`Arc<T>`、`RefCell<T>`、`String`——几乎所有标准库中的"安全"类型，内部都包含 unsafe 代码。

---

## 5. 编写 SAFETY 注释

### 5.1 为什么需要 SAFETY 注释？

SAFETY 注释解释**为什么**某个 unsafe 操作实际上是安全的。它是代码审查和维护的关键文档。

### 5.2 格式

Rust 社区约定在**每个** unsafe 块或 unsafe 函数前使用 `// SAFETY:` 注释：

```rust
// SAFETY: ptr 来自 &value，value 是一个有效的已初始化 i32，
//         且在此 unsafe 块期间 value 仍然存活。
unsafe {
    println!("{}", *ptr);
}
```

### 5.3 好的 SAFETY 注释包含什么？

1. **为什么条件被满足**：不是重复代码做了什么，而是解释为什么它安全
2. **哪些不变量被维护**：明确说明 Rust 的哪些安全规则被遵守
3. **引用了哪些前提条件**：如果是 unsafe 函数，说明调用者保证了什么

```rust
// 糟糕的 SAFETY 注释 ❌
// SAFETY: 这是安全的
unsafe { *ptr = 42; }

// 好的 SAFETY 注释 ✅
// SAFETY: 此切片来自 `values`，其生命周期至少与当前函数一样长。
//         i 已经过边界检查（i < self.len()），因此 ptr.add(i) 指向
//         切片内的有效元素。没有其他引用同时访问该元素。
unsafe {
    *ptr.add(i) = new_value;
}
```

### 5.4 Unsafe 函数的 Safety 文档

```rust
/// 从裸指针读取一个 i32 值。
///
/// # Safety
///
/// 调用者必须确保：
/// - `ptr` 非空（non-null）
/// - `ptr` 指向一个有效的、已初始化的 i32 值
/// - `ptr` 指向的内存不会在并发线程中被修改
/// - `ptr` 满足 i32 的内存对齐要求
unsafe fn read_i32(ptr: *const i32) -> i32 {
    *ptr
}
```

---

## 6. 何时应该使用 Unsafe

### 6.1 正当的使用场景

#### FFI：与 C 库交互

当你需要调用 C 函数或向 C 暴露 Rust 函数时，unsafe 是不可避免的。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe {
    let x = abs(-42);  // FFI 调用天然是 unsafe 的
}
```

#### 性能关键的热路径

在某些极端性能敏感的场景中，可以跳过边界检查来获得更好的性能：

```rust
// 安全的索引访问（带边界检查）
let x = array[i];

// unsafe 的索引访问（无边界检查，更快）
unsafe {
    let x = *array.get_unchecked(i);
}
```

但前提是你已经通过其他方式验证了索引在范围内。

#### 实现底层数据结构

某些数据结构在 safe Rust 中是不可能或低效的：

- 双向链表（需要两个可变引用指向相邻节点）
- 自引用结构（如 async 状态机）
- 侵入式容器（节点嵌入在元素内部）

#### 与硬件交互

内存映射 I/O（Memory-Mapped I/O）、DMA 等需要直接操作物理地址：

```rust
const MMIO_BASE: usize = 0x4000_0000;
let ptr = MMIO_BASE as *mut u32;
unsafe {
    ptr.write_volatile(0x1);  // 向硬件寄存器写入
}
```

#### 实现不可变类型的内部可变性

`RefCell`、`Cell`、`Mutex` 等都是通过 unsafe 实现的。

### 6.2 判断标准

在使用 unsafe 之前，问自己：

1. 这个操作用 safe Rust 真的做不到吗？（先尝试 safe 方案）
2. 我能用更少的 unsafe 代码完成吗？（最小化 unsafe 表面）
3. 我能提供一个安全的 API 来包装它吗？（安全抽象原则）
4. 我是否清楚所有需要维护的安全不变量？

---

## 7. 何时不应该使用 Unsafe

### 7.1 不应该使用 unsafe 的场景

#### 为了"绕过借用检查器"

```rust
// ❌ 糟糕的做法：用 unsafe 来"修复"生命周期问题
let mut x = 5;
let r1: *const i32 = &x;
let r2: *mut i32 = &mut x;  // 这不是问题因为这是裸指针
// 但这样做失去了所有保证
```

如果借用检查器拒绝你的代码，**先想想你的设计是否合理**，而不是直接用 unsafe 绕过它。大多数情况下，借用检查器的拒绝是有道理的——它正在防止一个真正的 bug。

#### 为了避免学习生命周期

```rust
// ❌ 不好的原因："我不想标注生命周期"
fn get_first<T>(a: &T, b: &T) -> &T { a }  // 编译器要求标注
// 好的做法：学习并正确标注生命周期
fn get_first<'a, T>(a: &'a T, _b: &T) -> &'a T { a }
```

#### 为了"简化"错误处理

```rust
// ❌ 不好
unsafe { MaybeUninit::assume_init() }  // 跳过初始化检查

// ✅ 好的做法：正确处理所有错误情况
match result {
    Ok(val) => val,
    Err(e) => return Err(e),
}
```

#### 为了一点点微不足道的性能提升

```rust
// ❌ 过早优化：为了省一次边界检查用了 unsafe
unsafe { *array.get_unchecked(i) }

// ✅ 先测量，如果确实需要再考虑 unsafe
let x = array[i];  // 大部分情况下性能差异可忽略
```

### 7.2 经验法则

> **如果 safe Rust 能表达你的意图，就不要使用 unsafe。**

Unsafe 是 Rust 为"必须"的场景准备的逃生舱，而不是日常编程工具。

---

## 8. FFI：外部函数接口

### 8.1 什么是 FFI？

FFI（Foreign Function Interface，外部函数接口）允许一种编程语言调用另一种语言编写的函数。在 Rust 中，FFI 主要用于与 C 语言（以及任何遵循 C ABI 的语言）进行互操作。

### 8.2 从 Rust 调用 C 函数

```rust
// 1. 声明外部函数
extern "C" {
    // libc 中的函数
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

// 2. 在 link 属性中指定要链接的库
// 或者在 build.rs 中配置：
// fn main() {
//     println!("cargo:rustc-link-lib=m");  // 链接数学库
// }

fn main() {
    // 3. 在 unsafe 块中调用
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
    }
}
```

### 8.3 从 C 调用 Rust 函数

```rust
// 1. 函数必须有 extern "C" 和 #[no_mangle]
#[no_mangle]  // 保留原始函数名，防止 Rust 的名称修饰（mangling）
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// 2. 编译为共享库
// [lib]
// crate-type = ["cdylib"]
```

然后在 C 中：

```c
// 声明外部 Rust 函数
extern int32_t add_numbers(int32_t a, int32_t b);

int main() {
    int result = add_numbers(3, 5);  // 调用 Rust 函数
    printf("3 + 5 = %d\n", result);
    return 0;
}
```

### 8.4 为什么 FFI 是 unsafe 的？

1. **C 代码不受 Rust 规则约束**：C 函数可以做任何事情——修改全局状态、产生数据竞争、返回无效指针
2. **类型安全无法保证**：C 的类型系统比 Rust 弱得多
3. **内存管理不匹配**：C 使用手动内存管理，Rust 使用 RAII
4. **线程安全未知**：C 函数不声明它是否是线程安全的
5. **ABI 不匹配风险**：结构体布局、调用约定、对齐要求可能不同

### 8.5 FFI 的最佳实践

1. **最小化 unsafe 表面**：将 FFI 调用封装在安全的 Rust API 中
2. **验证所有输入**：不要信任来自 C 的数据（可能为 null、越界、类型错误）
3. **注意生命周期**：从 C 返回的指针没有 Rust 的生命周期保证
4. **正确处理资源**：确保在 Rust 侧正确地分配/释放 C 内存
5. **使用 `#[repr(C)]`**：确保 Rust 结构体使用与 C 兼容的内存布局

---

## 9. Extern "C" 和链接

### 9.1 extern 语法

```rust
// extern "C" 块：声明外部 C 函数
extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

// extern "C" 函数定义：导出给 C 调用的 Rust 函数
pub extern "C" fn my_rust_function(x: i32) -> i32 {
    x * 2
}
```

### 9.2 ABI 字符串

`extern` 后面可以跟不同的 ABI（Application Binary Interface）标识符：

| ABI | 说明 |
|-----|------|
| `"C"` | C ABI（最常用） |
| `"system"` | 目标平台的系统 ABI（Windows 上是 `"stdcall"`，Unix 上是 `"C"`） |
| `"Rust"` | 默认的 Rust ABI（不保证跨版本稳定） |

### 9.3 链接 C 库

**方法一：`#[link]` 属性**

```rust
#[link(name = "m")]  // 链接 libm（数学库）
extern "C" {
    fn cos(x: f64) -> f64;
}
```

**方法二：build.rs**

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=m");  // 链接数学库
    println!("cargo:rustc-link-search=native=/path/to/libs");  // 库搜索路径
}
```

**方法三：使用 libc crate**

```rust
// Cargo.toml: [dependencies] libc = "0.2"
use libc::{c_char, c_int};

extern "C" {
    fn puts(s: *const c_char) -> c_int;
}
```

### 9.4 `#[repr(C)]`

当通过 FFI 传递结构体时，必须使用 `#[repr(C)]` 确保内存布局与 C 兼容：

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

// 在 C 中对应的结构体：
// typedef struct {
//     double x;
//     double y;
// } Point;
```

### 9.5 `#[no_mangle]`

Rust 默认会对函数名进行名称修饰（name mangling）。`#[no_mangle]` 保留原始函数名，以便 C 代码可以找到它。

---

## 10. 为什么初学者不应该用 Unsafe "修复"借用检查器错误

### 10.1 借用检查器是你的朋友，不是敌人

借用检查器的每一个"拒绝"都在告诉你：你的代码有一个**真正的**内存安全问题。用 unsafe 绕过它，只是把 bug 从"编译时错误"变成了"运行时未定义行为"。

### 10.2 典型场景

```rust
// 场景：初学者想写一个双向链表
struct Node {
    value: i32,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

// 编译错误：不能同时有两个可变引用
// 初学者可能想：用 *mut Node 裸指针代替 Box
struct Node {
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}
// 这样就绕过了借用检查器——但也失去了所有安全保证
```

正确的做法是：**改变设计思路**，而不是用 unsafe 绕过编译器。对于这个例子，可以考虑使用 `Rc<RefCell<Node>>` 或使用现有的生态库（如 `std::collections::LinkedList`）。

### 10.3 心态转变

| 初学者心态 | 正确心态 |
|-----------|---------|
| "这个生命周期标注太繁琐了" | "这个生命周期关系表达了重要的所有权关系" |
| "借用检查器太严格了" | "借用检查器发现了一个潜在的内存 bug" |
| "用 unsafe 就简单了" | "unsefe 只是把 bug 推迟到运行时" |
| "这段代码看起来没问题" | "我需要用 SAFETY 注释证明它没问题" |

---

## 11. 真实世界中的 Unsafe：标准库内部

标准库是学习 unsafe 用法的**最佳教材**。以下是一些代表性例子：

### 11.1 `Vec<T>` 的内部实现

`Vec<T>` 是一个完全安全的类型，但它的内部大量使用 unsafe：

- 使用裸指针管理堆内存（替代手动 `malloc`/`free`）
- 使用 `ptr::write` 和 `ptr::read` 来移动元素
- 通过 unsafe 实现 `from_raw_parts` 等构造函数

```rust
// Vec 内部使用类似这样的代码（简化版）：
impl<T> Vec<T> {
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();  // 扩大容量
        }
        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            std::ptr::write(end, value);
        }
        self.len += 1;
    }
}
```

### 11.2 `Box<T>` 的背后

`Box<T>` 是对堆分配内存的安全包装：

```rust
// Box 的核心是不安全的内存分配
// 使用 GlobalAlloc trait 和 alloc::alloc 函数
```

### 11.3 `RefCell<T>` 的内部可变性

`RefCell` 将借用检查从**编译时**推迟到**运行时**：

```rust
// RefCell 使用 unsafe 来实现内部可变性，
// 但在运行时维护借用计数器来保证安全
```

### 11.4 `slice::split_at_mut`

标准库的 `split_at_mut` 使用 unsafe 将可变切片分割为两个：

```rust
pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    // 使用 unsafe 创建两个不重叠的可变子切片
    // 对外暴露完全安全的 API
}
```

### 11.5 关键启示

- 标准库中的 unsafe 代码经过了**极其严格的审查**
- 每个 unsafe 用法都有**充分的安全论证**
- 原则：**最小化 unsafe 表面 + 安全抽象**

---

## 12. 核心术语对照

| 术语（英文） | 术语（中文） | 说明 |
|-------------|-------------|------|
| Unsafe Rust | 不安全 Rust | Rust 中允许执行五种危险操作的子语言 |
| Safe Rust | 安全 Rust | Rust 的默认模式，由编译器保证内存安全 |
| Raw Pointer | 裸指针 | `*const T` 或 `*mut T`，不受借用检查器约束 |
| FFI (Foreign Function Interface) | 外部函数接口 | 跨语言调用的机制 |
| Safe Abstraction | 安全抽象 | 用安全的 API 包装 unsafe 实现 |
| extern "C" | 外部 C 块 | 声明或定义使用 C ABI 的函数 |
| Union | 联合体 | 所有字段共享同一块内存的类型 |
| Mutable Static | 可变静态变量 | 全局可变状态，读写都需要 unsafe |
| Undefined Behavior (UB) | 未定义行为 | 程序行为不再被 Rust 保证，可能导致任意结果 |
| Soundness | 健全性 | unsafe 代码不违反 Rust 内存安全规则的属性 |
| Safety Invariant | 安全不变量 | 必须维护才能保证内存安全的条件 |
| Borrow Checker | 借用检查器 | 在编译时保证引用安全性的机制 |
| Data Race | 数据竞争 | 多线程同时访问同一内存且至少有一个写的未定义行为 |
| name mangling | 名称修饰 | 编译器修改函数名以包含类型信息 |
| ABI | 应用二进制接口 | 函数调用约定、类型布局等底层约定 |

---

## 13. Python 对照

### 13.1 Python 的 "unsafe" 边界

Python 也有跨越安全边界的机制：

| Python | Rust |
|--------|------|
| `ctypes` 模块 | `extern "C"` 块 |
| `cffi` 库 | FFI（直接支持） |
| C 扩展（CPython API） | `#[no_mangle] pub extern "C" fn` |
| `struct.pack` / `struct.unpack` | `union` + unsafe 解引用 |

### 13.2 Python C 扩展 vs Rust FFI

**Python C 扩展：**

```python
# 使用 ctypes 调用 C 共享库
import ctypes
lib = ctypes.CDLL('./mylib.so')
result = lib.add(3, 5)  # 没有编译时类型检查！
```

```c
// Python C 扩展（CPython API）
static PyObject* my_function(PyObject* self, PyObject* args) {
    // 需要手动解析参数、检查类型、处理错误
    return PyLong_FromLong(42);
}
```

**Rust FFI：**

```rust
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

// 虽然有 unsafe 块，但 Rust 编译器仍然检查：
// - 函数签名（参数类型和返回类型）
// - 所有权和借用规则（在 unsafe 块内部）
// - 其他安全规则
unsafe {
    let result = add(3, 5);
}
```

### 13.3 关键对比

| 维度 | Python (ctypes/cffi) | Python (C 扩展) | Rust FFI |
|------|---------------------|-----------------|----------|
| 类型安全 | 运行时检查 | 手动保证 | 编译时检查 + unsafe 边界 |
| 错误处理 | 异常 | PyErr_SetString | Result<T, E> |
| 性能开销 | 中等（运行时转换） | 低（直接调用） | 极低（零成本抽象） |
| 安全边界 | 隐式 | 隐式 | **显式**（unsafe 关键字） |
| 文档要求 | 无强制要求 | 无强制要求 | SAFETY 注释（约定俗成） |

**Rust 的优势：** 安全边界是**显式的**、**文档化的**、**编译时可见的**。当你看到 `unsafe`，你就知道需要注意安全了。

---

## 14. Python、C 与 C++ 对照

### 14.1 三种语言的"不安全"哲学

C 和 C++ 的设计哲学是：**低层操作是默认行为，安全由程序员自己负责**。你可以直接操作指针、手动分配/释放内存、调用任意函数地址——编译器不会阻止你，也几乎不会帮你检查。这种"默认全开"的模式赋予了极高的灵活性，但也意味着所有内存安全问题都依赖于程序员的纪律与经验。在 C/C++ 中，空指针解引用、缓冲区溢出、use-after-free 等灾难性 bug 在语法上与普通代码毫无区别，编译器不会给出任何警告。

Rust 则反其道而行之：**高风险操作被明确标记为 `unsafe`**。在默认的 safe Rust 中，编译器静态证明了所有内存安全属性。当你确实需要绕开这些检查时，必须显式地写上 `unsafe` 关键字——这是一个信号，告诉读者和审查者："这里需要额外关注"。

**关键认知：`unsafe` 并不是"关掉编译器"。** 它只解锁了第 3 节中讨论的五种额外能力：解引用裸指针、调用 unsafe 函数、访问可变静态变量、实现 unsafe trait、访问 union 字段。除此之外，借用检查、类型检查、所有权规则等所有安全检查**依然完整有效**。

### 14.2 FFI 中的关键考量

当 Rust 与 C/C++ 通过 FFI 互操作时，以下细节必须慎重处理：

**ABI 与数据布局。** 不同语言的默认内存布局可能不同。Rust 结构体必须使用 `#[repr(C)]` 来保证与 C 兼容的字段顺序和对齐方式。没有这个标注，Rust 编译器可以自由重排字段以优化内存占用——这在跨 FFI 边界时将导致数据错乱。

**空指针。** C 代码大量使用 NULL 表示"无值"或错误。Rust 的引用（`&T`、`&mut T`）在语义上无法为 null，但通过 FFI 接收到的裸指针（`*const T`、`*mut T`）可以为 null。在将 C 的裸指针转换为 Rust 引用之前，**必须先检查非空**，否则将直接触发未定义行为。

**所有权边界：谁分配，谁释放？** 这是一个经典的 FFI 陷阱。如果 C 代码分配了内存（`malloc`），Rust 代码不应该使用 `Box` 或 `drop` 来释放它——必须调用对应的 C 释放函数（`free` 或自定义析构函数）。反之亦然，Rust 分配的内存若在 C 侧用 `free` 释放同样危险。资源的所有权和释放责任必须在 API 文档中明确约定。

**字符串编码。** C 字符串是 null 结尾的字节序列（通常是 UTF-8 或 ASCII），没有长度信息。Rust 字符串（`&str`、`String`）是带长度前缀的有效 UTF-8。互操作时必须使用 `std::ffi::CStr`（从 C 读）和 `std::ffi::CString`（向 C 写）进行转换，而不是直接强制类型转换。

**错误码转换。** C 通常用返回值（如 `-1`、`NULL`、非零 int）表示错误，并通过 `errno` 传递详细信息。Rust 使用 `Result<T, E>` 枚举来表示操作的成功或失败。在 FFI 边界，应将 C 的错误码模式尽早翻译为 Rust 的 `Result`，将"可能是错误的值"转换为类型安全的错误处理。

**回调与生命周期。** 当 C 代码保存一个函数指针以便稍后回调时，必须确保该回调在回调发生期间仍然存活。Rust 的借用检查器无法跨 FFI 边界追踪生命周期，因此这个保证完全由程序员承担。

**线程安全。** C 函数通常不声明自己是否线程安全。如果 Rust 侧在多线程环境中调用 C 函数，程序员必须自行确认该函数的线程安全性，或通过锁（`Mutex`）等机制确保串行化访问。

### 14.3 三个关键澄清

**第一：`unsafe` 不会禁用安全引用的借用检查。** 即使在 `unsafe` 块内部，对普通引用 `&T` 和 `&mut T` 的借用规则**仍然严格生效**。你无法在 `unsafe` 块中创建同一变量的两个可变引用——编译器仍然会拒绝。`unsafe` 只是允许你创建和操作不受借用检查约束的**裸指针**，而从裸指针推导出"看起来安全"的使用，其正确性由你自行负责。

**第二：使用 `unsafe` 不代表设计糟糕。** 某些系统编程任务是 safe Rust **无法表达**的——例如调用操作系统 API、实现侵入式数据结构、直接操作硬件寄存器。在这样的场景下，`unsafe` 是**正确且必要**的工具。关键在于**封装**：将 unsafe 代码限制在小模块内部，对外暴露安全的 API。Rust 标准库中的 `Vec`、`Box`、`Rc`、`Mutex` 等所有基础类型，内部都依赖 unsafe 实现——但这丝毫不影响它们作为安全抽象的优秀设计。

**第三：FFI 需要系统性处理四个核心问题。** 谁分配内存？谁负责释放？C 字符串如何转换为 Rust 字符串？C 的错误码（返回值/errno）如何映射到 Rust 的 `Result`？这四个问题必须在 FFI 边界的设计阶段就给出明确答案，而不是在实现时临时应对。

### 14.4 能力对照表

| 能力 | C/C++ 默认 | Rust 安全代码 | Rust unsafe |
|------|:---:|:---:|:---:|
| 解引用裸指针 | ✅ | ❌ | ✅ |
| 调用任意函数指针 | ✅ | ✅ (限定) | ✅ |
| 修改全局可变状态 | ✅ | ❌ (需特定类型) | ✅ |
| 手动管理内存 | ✅ | ❌ (所有权自动) | ✅ (需手动) |
| FFI 调用 | ✅ | ❌ | ✅ |

---

## 15. 延伸阅读

### 15.1 官方文档

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — Rust 的"黑魔法书"，深入讨论 unsafe
- [Rust Reference: Unsafety](https://doc.rust-lang.org/reference/unsafety.html)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)

### 15.2 推荐工具

- [Miri](https://github.com/rust-lang/miri) — Rust 的 MIR 解释器，可以检测 unsafe 代码中的未定义行为
- [cargo-geiger](https://github.com/rust-secure-code/cargo-geiger) — 检测项目中的 unsafe 代码比例
- [loom](https://github.com/tokio-rs/loom) — 并发代码的排列测试工具

### 15.3 书籍推荐

- *Programming Rust* (2nd Edition) — Jim Blandy, Jason Orendorff, Leonora Tindall
- *Rust for Rustaceans* — Jon Gjengset（第 5 章深入讨论 unsafe）

---

## 附录：本章示例代码运行

```bash
cd chapters/24_unsafe_rust_and_ffi_overview
cargo build
cargo run
cargo test
cargo clippy
```

---

**核心信息：Unsafe Rust 不是"关闭检查"的 Rust，而是"程序员手动保证安全"的 Rust。用最小的 unsafe 内核构建安全抽象，是 Rust 系统编程的精髓。**
