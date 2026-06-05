# 参考答案

建议先独立完成练习，再阅读本文件。

**重要**：unsafe Rust 的练习答案不鼓励在生产代码中复制。这些练习的目的是理解 unsafe 的边界和责任——unsafe 不会关闭所有检查，只解锁 5 项能力。不要用 unsafe 逃避借用检查器。

---

## Level 1: 基础练习

### 练习 1.1: 裸指针基础操作

#### 结论

创建裸指针（`as_ptr()`）是安全的；解引用裸指针必须放在 `unsafe {}` 块内。裸指针遍历数组与 `arr.iter().sum()` 结果一致。

#### 参考实现

```rust
fn sum_with_raw_pointer(arr: &[i32]) -> i32 {
    let ptr = arr.as_ptr();     // *const i32 — 创建，安全
    let len = arr.len();
    let mut sum = 0;
    // SAFETY: ptr 指向 arr 的有效内存，长度 len 由调用者保证正确，
    //         在 unsafe 块内访问的内存范围完全在数组边界内。
    unsafe {
        for i in 0..len {
            sum += *ptr.add(i); // 解引用，需要 unsafe
        }
    }
    sum
}
```

#### 常见错误

- 把 `as_ptr()` 也放进 `unsafe` 块 — 不必要
- 忘记 `add(i)` 可能超出分配区 — 确保 `i < len`

#### 验证方式

```rust
let arr = [1, 2, 3, 4, 5, 6, 7, 8];
assert_eq!(sum_with_raw_pointer(&arr), arr.iter().sum::<i32>());
```

---

### 练习 1.2: 编写你的第一个 unsafe 函数

#### 结论

`unsafe fn` 的契约在文档注释中声明（`# Safety` 部分）。调用方负责满足前置条件。安全包装函数在 safe 层完成所有验证，让调用者无需接触 unsafe。

#### 参考实现

```rust
/// # Safety
///
/// 调用者必须确保：
/// - `ptr` 非空，且指向至少 `len` 字节的有效可写内存
/// - 内存区域未被其他线程同时访问（无数据竞争）
unsafe fn zero_memory(ptr: *mut u8, len: usize) {
    // SAFETY: 前置条件由调用者保证（见上）
    unsafe { std::ptr::write_bytes(ptr, 0, len); }
}

/// 安全的包装函数：将缓冲区清零。
/// 所有输入验证在 safe 层完成，内部 unsafe 的契约已通过类型系统保证。
fn clear_buffer(buf: &mut [u8]) {
    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    // SAFETY: ptr 来自 &mut [u8] 的 as_mut_ptr()，
    //         非空、对齐、指向 len 字节的有效内存，且通过 &mut 保证了独占访问。
    unsafe { zero_memory(ptr, len); }
}

#[test]
fn test_clear_buffer() {
    let mut buf = vec![1u8, 2, 3, 4, 5];
    clear_buffer(&mut buf);
    assert_eq!(buf, vec![0, 0, 0, 0, 0]);
}
```

#### 为什么这样设计

- 安全包装函数 `clear_buffer` 接收 `&mut [u8]`，类型系统已保证 `ptr` 非空、对齐、生命周期有效、独占访问
- unsafe 的"责任范围"被限制在单行 `write_bytes` 调用中
- 调用方不需要写 `unsafe {}` — 抽象已经完成

#### 常见错误

- 在 `clear_buffer` 中使用 `unsafe fn` 签名 — 没必要，用 safe 签名然后在内部使用 `unsafe {}` 块

---

### 练习 1.3: 理解 union 的基本用法

#### 结论

union 允许多个字段共享同一内存。访问 union 字段始终需要 `unsafe`（编译器无法验证哪个字段是活跃的）。

#### 参考实现

```rust
#[repr(C)]  // 确保与 C ABI 兼容
union Number {
    int_val: i32,
    float_val: f32,
    bytes: [u8; 4],
}

fn print_float_bytes(value: f32) {
    let num = Number { float_val: value };
    // SAFETY: bytes 和 float_val 共享同一 4 字节内存，
    //         读取 bytes 等价于读取 f32 的底层表示，不是 UB。
    let bytes = unsafe { num.bytes };
    println!("f32 {} 的字节表示: {:02x} {:02x} {:02x} {:02x}",
        value, bytes[0], bytes[1], bytes[2], bytes[3]);
}

fn interpret_i32_as_f32(value: i32) -> f32 {
    // 注意：这种位级重解释（type punning）通常是设计错误。
    // f32::from_bits() 是更安全的选择。
    // 此处仅为演示 union 的 unsafe 字段访问。
    let num = Number { int_val: value };
    // SAFETY: int_val 和 float_val 大小相同，读取不会越界。
    //         但结果取决于平台的字节序和 f32 表示。
    unsafe { num.float_val }
}
```

#### 关键理解

- **unsafe 不关闭所有检查**：union 访问仍需 unsafe，因为编译器无法验证逻辑正确性
- union 的大小等于最大字段的大小（此处 4 字节）
- `#[repr(C)]` 确保与 C 的 union 内存布局一致

---

## Level 2: 综合练习

### 练习 2.1: 实现安全的 Vec 简化版

#### 结论

**unsafe 不会关闭所有检查** — 借用规则、类型检查、Drop 语义仍在 unsafe 块外生效。我们的 `SimpleVec` 对外 API 完全 safe，unsafe 代码被封装在实现细节中。

#### 思路

使用 `std::alloc` 管理原始堆内存，手动追踪 `len`、`capacity` 和 `ptr`。`Drop` 确保 `SimpleVec` 离开作用域时正确释放。

#### 参考实现（关键部分）

```rust
use std::alloc::{self, Layout};
use std::ptr;

struct SimpleVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    pub fn new() -> Self {
        SimpleVec { ptr: std::ptr::null_mut(), len: 0, capacity: 0 }
    }

    pub fn len(&self) -> usize { self.len }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        // SAFETY: ptr.add(len) 在分配范围内（len < capacity）
        unsafe { self.ptr.add(self.len).write(value); }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: ptr.add(len) 指向已初始化的有效 T
        Some(unsafe { self.ptr.add(self.len).read() })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len { return None; }
        // SAFETY: index < len，ptr.add(index) 指向有效已初始化 T
        Some(unsafe { &*self.ptr.add(index) })
    }

    fn grow(&mut self) {
        let new_cap = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr: *mut T;
        if self.capacity == 0 {
            // SAFETY: layout 非零大小
            new_ptr = unsafe { alloc::alloc(layout) as *mut T };
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            // SAFETY: ptr 来自同分配器，old_layout 正确，new_size 有效
            new_ptr = unsafe { alloc::realloc(self.ptr as *mut u8, old_layout, layout.size()) as *mut T };
        }
        self.ptr = new_ptr;
        self.capacity = new_cap;
    }
}

impl<T> Drop for SimpleVec<T> {
    fn drop(&mut self) {
        if self.capacity == 0 { return; }
        // 逐元素调用 drop
        for i in 0..self.len {
            // SAFETY: 元素已初始化，只调用一次（len 正确追踪）
            unsafe { ptr::drop_in_place(self.ptr.add(i)); }
        }
        let layout = Layout::array::<T>(self.capacity).unwrap();
        // SAFETY: ptr 来自 alloc 分配，layout 正确，元素已全部 drop
        unsafe { alloc::dealloc(self.ptr as *mut u8, layout); }
    }
}
```

#### 安全抽象原则

1. **unsafe 只在实现层**：`push`/`pop`/`get` 对外签名不含 `unsafe`
2. **SAFETY 注释解释每一个 unsafe 块**：谁保证了什么，为什么可以确定是安全的
3. **panic safety**：`Drop` 中释放前确保所有元素已 `drop_in_place`

#### 常见错误

- 忘记实现 `Drop` → 内存泄漏
- `pop` 中 `read()` 后忘记减 `len` → 双重释放
- `grow` 中的 `realloc` 可能返回 null → 实际 alloc API 不会（会 abort），但需注意

---

### 练习 2.2: FFI —— 调用 C 标准库函数

#### 结论

FFI 的 unsafe 边界集中在 `extern "C"` 声明和 FFI 调用点。安全包装函数负责：验证参数、管理生命周期、确定"谁负责释放资源"。

#### 参考实现

```rust
// FFI 声明 — 这些声明本身不需要 unsafe
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

/// 安全的绝对值包装。
fn safe_abs(x: i32) -> i32 {
    // SAFETY: abs 对所有 i32 值都有定义（包括 i32::MIN，
    //         其结果虽然是负的但非 UB），无内存安全性问题。
    unsafe { abs(x) }
}

/// 安全平方根包装 — 验证前置条件。
fn safe_sqrt(x: f64) -> Result<f64, &'static str> {
    if x < 0.0 {
        return Err("不能对负数求平方根");
    }
    // SAFETY: x >= 0.0，满足 sqrt 的前置条件
    Ok(unsafe { sqrt(x) })
}
```

#### FFI 责任清单

| 责任 | 谁负责 | 如何保证 |
|------|--------|---------|
| 参数类型匹配 | 程序员（extern 声明） | 类型签名与 C 头文件一致 |
| 生命周期 | 程序员（安全包装） | 引用参数在调用期间有效 |
| 释放资源 | 取决于"谁分配谁释放"约定 | 文档 + `Drop` |
| ABI 兼容 | `extern "C"` | 编译器保证 |

---

## Level 3: 挑战练习

### 练习 3.1: 双向 FFI （C ↔ Rust）

#### 核心设计

```rust
// Rust 侧 — 暴露给 C 的函数
#[no_mangle]
pub extern "C" fn rust_process_data(ptr: *const u8, len: usize) -> i32 {
    // SAFETY: 调用者（C 侧）保证 ptr 非空、len 正确、内存有效
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    // 处理...
    0 // 成功
}

// Rust 侧 — 调用 C 函数
extern "C" { fn c_library_init(config_path: *const i8) -> i32; }
```

#### 关键安全不变式

1. `rust_process_data` 的调用者（C）保证参数有效
2. `#[no_mangle]` + `extern "C"` 确保函数可从 C 侧通过符号名调用
3. 分配与释放由单一语言负责（不跨语言边界传递所有权）

---

## 迁移思维练习答案

### 1. 哪些 C/C++ 中的常见操作在 Rust 中需要 unsafe？

解引用裸指针、调用 C 函数（extern "C"）、访问可变全局变量（static mut）、实现 unsafe trait、访问 union 字段、`transmute` 位级类型转换。这些在 C/C++ 中是默认行为，Rust 将其显式标注 unsafe——把"这里需要人工审查"写入代码。

### 2. 为什么 unsafe 代码应该封装在安全抽象中？

缩小"需要人工验证"的范围到最小可审计单元。标准库的 `Vec`、`String`、`Mutex` 都使用 unsafe 实现，但对外 API 完全安全。原则：**用最小的 unsafe 内核构建安全的抽象**。

### 3. 从 Python 调用 Rust（通过 FFI/pyo3）的典型实践？

pyo3 + maturin：`#[pyclass]`/`#[pymethods]` 标注 Python 可调用的类型和方法。错误在 Rust `Result` 和 Python 异常间自动转换。跨 FFI 边界不传裸指针——数据需要序列化转换。

---

## 核心原则回顾

1. **unsafe 不会关闭所有检查** — 仅解锁 5 项能力：解引用裸指针、调用 unsafe 函数、访问可变静态变量、实现 unsafe trait、访问 union 字段
2. **不要用 unsafe 逃避借用检查器** — 如果编译器拒绝你的代码，先反思设计，不是用 unsafe 绕过
3. **封装为小型安全抽象** — SAFETY 注释解释每个 unsafe 块的前提和保证
4. **FFI 边界清晰** — 明确谁分配、谁释放、生命周期、ABI

---

*Unsafe Rust 是 Rust 的"逃生舱"——在需要底层控制时可用，但务必封装在可审计的安全边界内。*
