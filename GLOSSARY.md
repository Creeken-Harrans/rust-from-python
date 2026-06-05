# 术语表 (Glossary)

本术语表收录本教程涉及的重要 Rust 术语，按字母顺序组织。每个术语包含中文翻译和简要解释，部分术语附有 Python 对照。

---

## B

### Borrow (借用)
通过引用（Reference）临时访问数据，而不获取其所有权（Ownership）。借用分为不可变借用（`&T`）和可变借用（`&mut T`）。借用规则：同一时刻可以有多个不可变借用，或恰好一个可变借用，两者不能共存。

**Python 对照**: Python 中没有直接的对应概念。Python 的对象引用可以随时共享和修改，运行时不检查数据竞争。

参见: [[#Ownership]], [[#Reference]], [[#Borrow Checker]]

### Borrow Checker (借用检查器)
Rust 编译器的核心组件之一，在编译期静态验证所有借用规则。借用检查器确保引用始终有效、不存在数据竞争。

### Box\<T\>
见 [[#Smart Pointer]]。

---

## C

### Cargo
Rust 的官方构建系统和包管理器。负责编译代码、下载依赖、运行测试、生成文档等。类似 Python 的 `pip` + `setuptools` + `virtualenv` + `tox` 的组合，但集成度更高且功能一致。

### Channel (信道)
用于线程间消息传递（Message Passing）的通信机制。Rust 标准库提供 `std::sync::mpsc`（多生产者单消费者）信道。

**Python 对照**: 类似 Python `queue.Queue`，但 Rust 的信道通过类型系统保证线程安全。

### Clippy
Rust 的官方静态分析工具（Linter），提供超过 550 条 lint 规则，帮助发现常见错误、改进代码风格和性能。

### Closure (闭包)
可以捕获其环境中变量的匿名函数。Rust 的闭包有三种 trait：`Fn`（不可变借用捕获）、`FnMut`（可变借用捕获）、`FnOnce`（获取所有权捕获）。

**Python 对照**: Python 的 `lambda` 和嵌套函数。Python 闭包通常通过引用捕获，没有 Rust 的所有权区分。

### Concurrency (并发)
多个任务在同一时间段内交替执行（可能在同一核心上）。与并行（Parallelism）不同：并发是"交替做多件事"，并行是"同时做多件事"。

**Python 对照**: Python 有 `asyncio`（协程并发）和 `threading`/`multiprocessing`（并行）。GIL 限制使得 Python 多线程通常无法实现真正的 CPU 并行。

### Copy Trait (复制特征)
标记类型可以按位复制（Bitwise Copy）的 trait。实现 `Copy` 的类型在赋值时不会发生移动（Move），而是自动复制。所有标量类型（整数、浮点数、布尔值、字符）及其组成的元组都实现了 `Copy`。

### Crate (箱)
Rust 的编译单元。Crate 可以是二进制 Crate（Binary Crate，产生可执行文件）或库 Crate（Library Crate，产生库文件）。一个 Package 可以包含多个 Crate。

**Python 对照**: 库 Crate 类似 Python 包（package），二进制 Crate 类似带有 `__main__.py` 的 Python 包或独立脚本。

---

## D

### Data Race (数据竞争)
当两个或更多线程同时访问同一内存位置，其中至少一个是写操作，且没有同步机制时，发生数据竞争。数据竞争导致未定义行为（Undefined Behavior）。Rust 的类型系统在编译期防止数据竞争。

**Python 对照**: Python 的 GIL 在 CPython 层面防止了许多数据竞争，但这限制了多线程性能。Python 仍需锁（Lock）来保证复合操作的原子性。

### Deref (解引用)
通过 `*` 运算符或 `Deref` trait 从引用或智能指针获取其指向的值。Rust 的 Deref Coercion（解引用强制转换）可以自动将 `&Box<T>` 转换为 `&T`、`&String` 转换为 `&str` 等。

### Drop (析构)
当值离开作用域时自动调用的清理逻辑。`Drop` trait 的 `drop()` 方法在值被释放前执行，常用于释放文件句柄、网络连接、锁等资源。类似 C++ 的析构函数（Destructor）或 Python 的 `__del__`，但 Python 的 `__del__` 调用时机不确定。

### Dynamic Dispatch (动态分派)
运行时确定调用哪个具体方法的机制。通过 `dyn Trait`（Trait Object）实现。与静态分派（Static Dispatch，泛型的单态化）相对。

---

## E

### Enum (枚举)
一种可以包含多个变体（Variant）的类型，每个变体可以携带不同类型和数量的数据。Rust 的枚举是代数数据类型（Algebraic Data Type, ADT）中的和类型（Sum Type）。

**Python 对照**: Python 3.11+ 的 `enum.Enum` 是类似的但功能更弱——Python 的枚举变体不能携带数据。Rust 的枚举更接近带标签的联合体（Tagged Union）。

### Expression (表达式)
计算并返回值的代码片段。Rust 中几乎一切都是表达式：字面量、变量、函数调用、`if`、`loop`、代码块等都是表达式。语句（Statement）不返回值，以分号结尾。

**Python 对照**: Python 中 `if` 和 `for` 不是表达式（Python 3.8+ 有赋值表达式 `:=` 但功能有限）。

---

## F

### Feature (特性)
Cargo 的可选功能标记。通过 Cargo.toml 的 `[features]` 定义，允许按需启用依赖或条件编译。不是运行时开关，而是编译期选项。

### FFI (外部函数接口)
Foreign Function Interface，允许 Rust 调用其他语言（主要是 C）的函数，或被其他语言调用。涉及 `unsafe` 代码。

### Future (未来值)
表示一个尚未完成的异步计算。Rust 的 `Future` trait 定义了异步计算的核心抽象。Future 需要 Executor（执行器/Runtime）来驱动执行。

**Python 对照**: 类似 Python 的 `Coroutine` 或 `asyncio.Future`。关键区别：Rust 的 Future 是惰性的，不调用 `.await` 就不会执行。

---

## G

### Generic (泛型)
允许代码适用于多种类型的机制。Rust 的泛型通过单态化（Monomorphization）实现零成本抽象——编译器为每种具体类型生成独立的代码副本。

**Python 对照**: Python 的函数天然支持任意类型（鸭子类型），但无法在"编译期"检查类型一致性。Python 3.12+ 的 `TypeVar` 和 `Generic` 提供类型注解层面的泛型支持。

---

## H

### Heap (堆)
用于存储大小在编译期未知或可能变化的数据的内存区域。堆分配需要显式请求（如 `String::new()`、`Box::new()`），释放由所有权系统自动管理。

**Python 对照**: Python 几乎所有对象都分配在堆上，由垃圾回收（GC）管理。Rust 区分栈和堆，性能意识更明确。

---

## I

### Interior Mutability (内部可变性)
允许在拥有不可变引用时修改内部数据的模式。`RefCell<T>` 是标准实现：编译期借用检查被推迟到运行时（运行时仍然检查规则，违反会 panic）。`Mutex<T>` 和 `RwLock<T>` 也提供内部可变性，用于线程间共享。

### Iterator (迭代器)
提供按顺序处理元素序列的抽象。Rust 的迭代器是惰性的（Lazy）：不调用消费方法（Consumer）就不会执行。分三类获取方式：`iter()`（不可变引用）、`iter_mut()`（可变引用）、`into_iter()`（获取所有权）。

**Python 对照**: Rust 迭代器与 Python 的迭代器协议（`__iter__`/`__next__`）概念相似。关键区别：Rust 的 `iter`/`iter_mut`/`into_iter` 通过所有权区分迭代方式，Python 没有此机制。

---

## L

### Lifetime (生命周期)
编译器用来追踪引用有效范围的标注。生命周期标注（如 `'a`）描述的是引用之间的关系（哪个引用活得更久），而不是延长对象的存活时间。大多数情况下编译器通过生命周期省略规则（Lifetime Elision）自动推断。

**Python 对照**: Python 没有生命周期概念——垃圾回收自动处理所有对象的存活时间。这是 Rust 最独特也最具挑战性的概念之一。

---

## M

### Macro (宏)
生成代码的代码。Rust 的声明式宏（`macro_rules!`）通过模式匹配生成代码（如 `println!`、`vec!`）。过程宏（Procedural Macro）更强大，可自定义 `#[derive]`、属性等。

**Python 对照**: Python 的装饰器（Decorator）在功能上有一定相似性，但原理完全不同。Python 无编译期代码生成机制。

### Module (模块)
Rust 中组织代码的单元。通过 `mod` 关键字定义。模块树与文件系统目录结构相关但不等同——`mod` 声明建立了显式的模块层级。

**Python 对照**: Python 的模块通过文件系统定义（每个 `.py` 文件是一个模块），Rust 的模块需要显式声明。

### Move (移动)
将值的所有权从一个变量转移到另一个变量。移动后，原变量不再有效。Rust 默认对非 `Copy` 类型进行移动而不是复制。

**Python 对照**: Python 赋值通常复制引用，不会使原变量"失效"。Rust 的 Move 不是深拷贝——它只转移所有权，不复制数据。

### Mutability (可变性)
Rust 中变量默认不可变（Immutable）。`mut` 关键字声明变量可修改。可变性的控制也扩展到引用：`&mut T` 提供可变借用。

**Python 对照**: Python 变量默认可变，不需要声明。但 Python 区分可变对象（list、dict）和不可变对象（int、str、tuple），这与 Rust 的 mut 绑定是不同层面的概念。

---

## O

### Ownership (所有权)
Rust 的核心内存管理机制。三条规则：(1) 每个值有且仅有一个所有者（Owner）；(2) 所有者离开作用域时值被释放；(3) 所有权可以通过移动（Move）转移。

**Python 对照**: Python 使用引用计数 + 垃圾回收管理内存。Rust 在编译期确定所有权的转移和释放时机，无运行时开销。

---

## P

### Package (包)
一个 Cargo 项目的顶层单元。一个 Package 包含一个 `Cargo.toml` 文件，可以包含一个或多个 Crate（最多一个库 Crate + 多个二进制 Crate）。

### Panic (恐慌)
Rust 处理不可恢复错误的机制。发生 panic 时程序展开调用栈（Unwinding）或直接中止（Abort），清理资源后退出。类似 Python 中未被捕获的异常导致程序崩溃。

### Pattern Matching (模式匹配)
通过 `match` 表达式和 `if let`、`while let`、`let else` 等语法对值的结构进行检查和解构。Rust 的模式匹配是穷尽的（Exhaustive）：编译器强制覆盖所有可能的情况。

**Python 对照**: Python 3.10+ 的 `match`/`case` 语句提供了类似但功能更有限的模式匹配。Rust 的模式匹配更深入类型系统。

---

## R

### RAII (资源获取即初始化)
Resource Acquisition Is Initialization。资源在值创建时获取，在值离开作用域时通过 `Drop` trait 自动释放。这是 Rust 管理文件、网络连接、锁等资源的核心理念。

**Python 对照**: Python 的 `with` 语句和上下文管理器（Context Manager，`__enter__`/`__exit__`）提供类似功能，但需要显式使用 `with`。Rust 的所有值默认都是"上下文管理的"。

### Rc\<T\> / Arc\<T\>
见 [[#Smart Pointer]]。

### Reference (引用)
指向某个值的"指针"，不拥有其所有权。通过 `&` 创建，通过 `*` 解引用。引用始终有效——Rust 编译器保证引用的生命周期不超过被引用值。

**Python 对照**: Python 变量名本身就是引用，但这些引用通过引用计数管理。Rust 的引用是编译期检查的、无运行时成本的借用。

### RefCell\<T\>
见 [[#Smart Pointer]] 和 [[#Interior Mutability]]。

### Result\<T, E\>
Rust 处理可恢复错误的核心类型。`Ok(T)` 表示成功，`Err(E)` 表示失败。编译器不强制使用 `Result`，但一旦函数返回 `Result`，调用者必须显式处理。

**Python 对照**: 类似 Python 中通过异常（Exception）处理的错误。关键区别：Rust 的错误处理被编码在类型签名中，调用者无法"忘记"处理。

### Runtime (运行时)
执行异步任务的引擎。Rust 标准库只提供 `Future` trait，具体的执行由第三方 Runtime（如 Tokio、async-std）提供。这给了开发者选择权，但也增加了学习曲线。

**Python 对照**: Python 的 `asyncio` 既是标准库的一部分也提供了完整的 Runtime。Rust 的异步生态需要"自带 Runtime"。

---

## S

### Slice (切片)
对连续数据序列的借用视图。`&str` 是对 UTF-8 字符串的切片，`&[T]` 是对数组或 Vec 的切片。切片不拥有数据，是对原始数据的引用。

**Python 对照**: Python 切片（如 `list[1:3]`）通常创建新对象（拷贝）。Rust 切片只是借用视图，不复制数据。

### Smart Pointer (智能指针)
封装了额外行为和所有权的指针类型。包括：
- **Box\<T\>**: 堆分配的独占所有权，用于递归类型或大对象。
- **Rc\<T\>**: 单线程引用计数，用于共享所有权。
- **Arc\<T\>**: 原子引用计数，用于多线程安全共享所有权。
- **RefCell\<T\>**: 提供内部可变性，将借用检查推迟到运行时。
- **Mutex\<T\>**: 提供线程间互斥访问。

### Stack (栈)
用于存储大小固定、生命周期可预测的数据的内存区域。栈分配和释放极快（只是移动栈指针），按后进先出（LIFO）顺序管理。

### Statement (语句)
执行操作但不返回值（或返回 `()`）的代码指令。`let x = 5;` 是语句，`5` 是表达式。语句以分号结尾。

### Static Dispatch (静态分派)
编译期确定调用哪个方法，通过泛型的单态化（Monomorphization）实现。无运行时开销，但会导致代码膨胀（每个具体类型生成一份代码）。与动态分派（Dynamic Dispatch）相对。

### Struct (结构体)
自定义数据类型，将多个命名字段组合在一起。三种形式：经典结构体（Named Field Struct）、元组结构体（Tuple Struct）、单元结构体（Unit-Like Struct）。

**Python 对照**: Python 的 `dataclass`、`NamedTuple` 或普通类。Rust 的结构体字段类型固定且编译期检查。

---

## T

### Trait (特征)
定义类型之间共享行为的接口。Trait 可以包含方法签名和默认实现。类似其他语言的接口（Interface），但更强大（可以有默认实现、关联类型、Blanket Implementation）。

**Python 对照**: 接近 Python 的抽象基类（ABC）+ Protocol 的组合。关键区别：Rust Trait 是编译期约束，Trait 实现与类型定义分离（孤儿规则 Orphan Rule）。

### Trait Bound (特征约束)
对泛型参数施加的约束，要求类型实现了特定 Trait。如 `fn foo<T: Display>(x: T)` 要求 `T` 实现 `Display`。

**Python 对照**: Python 的 Protocol 可以通过类型检查器（mypy）实现类似约束，但不是语言强制。

### Trait Object (特征对象)
通过 `dyn Trait` 使用动态分派的值。`Box<dyn Trait>` 是最常见形式。允许不同类型通过统一接口操作，但有运行时开销。

### Type Inference (类型推断)
编译器自动推断变量和表达式的类型，无需手动标注。Rust 的 Hindley-Milner 类型推断很强大，但在函数签名和某些场景仍需要显式标注。

---

## U

### Unsafe Rust
允许执行编译器无法静态验证安全的操作的 Rust 子集。`unsafe` 关键字不会关闭所有安全检查，只允许：(1) 解引用裸指针；(2) 调用 unsafe 函数；(3) 访问可变静态变量；(4) 实现 unsafe trait；(5) 访问 union 字段。Unsafe 代码应封装在安全抽象中。

---

## W

### Workspace
多个 Cargo Package 的集合，共享一个 `Cargo.lock` 和一个 `target/` 目录。可用于组织大型项目或教程。

---

## 常用缩写

| 缩写 | 全称 | 说明 |
|------|------|------|
| RAII | Resource Acquisition Is Initialization | 资源获取即初始化 |
| FFI | Foreign Function Interface | 外部函数接口 |
| NLL | Non-Lexical Lifetimes | 非词法生命周期 |
| API | Application Programming Interface | 应用编程接口 |
| DSL | Domain Specific Language | 领域特定语言 |
| FFI | Foreign Function Interface | 外部函数接口 |
| REPL | Read-Eval-Print Loop | 交互式解释器循环 |
| JIT | Just-In-Time compilation | 即时编译 |
| AOT | Ahead-Of-Time compilation | 提前编译（Rust 使用的） |
