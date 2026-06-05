# 迁移思维练习答案 — Unsafe Rust 与 FFI

## 迁移思维练习答案

### 1. 哪些 C/C++ 中的常见操作在 Rust 中需要 unsafe？

解引用裸指针（*const T, *mut T）、调用 C 函数（通过 FFI 声明 extern "C" 的函数）、访问或修改可变全局变量（static mut）、实现 unsafe trait（如 Send、Sync 的 unsafe impl）、直接读写 union 的字段、通过 transmute 进行任意类型的位级转换。这些操作在 C/C++ 中是默认行为，不需要任何标记；在 Rust 中被显式标注 unsafe，将"这里需要人工审查"的信号写入代码。

### 2. 为什么 unsafe 代码应该封装在安全抽象中？

unsafe 的责任转移到了程序员身上——你必须手动验证所有安全前提：指针非空且对齐、引用的数据在访问期间有效（生命周期正确）、不存在数据竞争（同时的读写）、不违反 Rust 的别名规则（&mut 独占）。将这些验证集中在小而可审计的 unsafe 模块中，对外提供安全 API，可以大大缩小"需要人工验证"的范围。标准库中的 Vec、String、Mutex、BufReader 内部都使用了 unsafe，但对外接口是完全安全的——用户不需要操心内部的不变式是否被保持。

### 3. 从 Python 调用 Rust（通过 FFI/pyo3）的典型实践是什么？

使用 pyo3 或 maturin 工具链是最常见的方案：在 Rust 侧用 `#[pyclass]` 和 `#[pymethods]` 标注要暴露给 Python 的结构体和方法，用 `#[pyfunction]` 标注自由函数。Python 侧看起来就像 import 了一个正常模块。需要注意的点：错误处理需要在 Rust 的 Result 和 Python 异常之间转换（pyo3 提供了自动转换）、跨越 FFI 边界的数据需要序列化/转换而非传递裸指针、GIL 的管理（一般 pyo3 会自动处理，但涉及多线程时需要显式释放 GIL）。
