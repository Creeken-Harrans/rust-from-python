# C/C++ → Rust 概念系统对照

本文档为具有 C/C++ 初步基础的学习者提供系统化的概念对照，帮助理解 Rust 设计背后的动机和机制。

**阅读前提**：
- 你了解 C 的基本语法（变量、函数、指针、结构体、`malloc/free`）
- 你接触过 C++ 的类、RAII、模板、智能指针等概念
- 你对 Rust 的所有权、借用、生命周期还不熟悉

**重要提示**：
- 本对照表旨在帮助理解，**相似不等于等价**
- 每个类比都有边界，不要机械地将 C/C++ 经验映射到 Rust
- Rust 的概念有独立的动机和设计上下文

---

## 1. 基础结构对照

### 1.1 程序入口

| 语言 | 入口形式 | 说明 |
|------|---------|------|
| C | `int main(int argc, char *argv[])` | 返回 `int`，0 表示成功 |
| C++ | `int main(int argc, char *argv[])` | 与 C 相同 |
| Rust | `fn main()` | 返回 `()`（unit），无需返回错误码。可以通过 `std::process::exit()` 设置退出码 |

Rust 的 `main` 也可以返回 `Result<(), E>`：

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ? 运算符会传播错误，非零退出码
    Ok(())
}
```

### 1.2 变量

| 主题 | C | C++ | Rust | 关键差异 |
|------|---|-----|------|---------|
| 默认可变性 | 默认可变 | 默认可变（可用 `const`） | 默认不可变，显式 `mut` | Rust 强调最小可变性，编译器可据此优化 |
| 常量 | `const`、宏 `#define` | `const`、`constexpr` | `const`（编译期求值）、`static`（运行时可能可变） | Rust 的 `const` 表示编译期已知值；`static` 表示全局生命周期 |
| 类型推断 | 无（C23 前） | `auto` | `let` 自动推断 | Rust 的 Hindley-Milner 推断比 `auto` 更强大 |
| Shadowing（遮蔽） | 不支持同一作用域 | 不支持同一作用域 | 支持 `let x = ...; let x = ...;` | Shadowing 不是赋值，可改变类型 |

```rust
// Rust: Shadowing vs 赋值
let x = 5;
// x = 6;        // 错误！x 不可变
let x = "hello"; // OK！新的绑定，类型可以不同

let mut y = 5;
y = 6;           // OK！mut 允许修改，但类型不变
```

**为什么 Rust 默认不可变？** 在 C/C++ 中，意外修改是常见 Bug 来源（函数副作用、循环修改变量等）。Rust 要求显式声明修改意图，让编译器和读者都能清晰理解数据流向。

### 1.3 结构体

| 主题 | C | C++ | Rust |
|------|---|-----|------|
| 定义 | `struct Name { ... };` | `struct Name { ... };` 或 `class` | `struct Name { ... }`（无分号） |
| 方法 | 无（函数指针） | 成员函数（在类定义内） | `impl Name { fn method(&self) ... }`（分离定义） |
| 继承 | 无 | 单继承/多继承 | 无类继承（通过 Trait + 组合） |
| 构造 | 手动初始化 | 构造函数/析构函数 | 约定 `new()` 关联函数 + `Drop` trait |
| 访问控制 | 无 | `public`/`protected`/`private` | `pub`（公开）/ 默认私有 |

**关键差异**：
- Rust 的 `struct` + `impl` 将数据和方法分离——这与 C++ 的类不同，更接近于"为类型实现行为"
- Rust 没有继承层次——代码复用通过泛型、Trait 默认实现和组合完成
- Rust 没有构造函数语法——`new()` 只是社区约定，不是语言特性

### 1.4 枚举

| 主题 | C | C++ 11+ | Rust |
|------|---|---------|------|
| 本质 | 整数别名 | 有作用域的整数值 | 代数数据类型（每个变体可携带不同数据） |
| 携带数据 | 不支持 | `std::variant`（C++17） | 原生支持 |
| 穷尽性检查 | 无 | `switch` 可警告（`-Wswitch`） | `match` **强制穷尽** |

```rust
// Rust 枚举：每个变体可以有不同类型和数量的数据
enum ConnectionState {
    Disconnected,                          // 无数据
    Connecting { attempt: u32 },            // 命名字段
    Connected(String),                      // 元组风格
    Failed { reason: String, code: i32 },   // 多个命名字段
}
```

这与 C 中 `enum + union` 的模式有相似目的，但 Rust 的类型系统保证了访问的安全性——你无法在 `Disconnected` 状态下错误地读取 `reason` 字段。

### 1.5 分支控制

| 主题 | C/C++ | Rust |
|------|-------|------|
| 条件分支 | `if` / `switch` | `if`（是表达式）/ `match` |
| 表达式性 | `switch` 不是表达式 | `if` 和 `match` 都是表达式 |
| 穷尽性 | `switch` 无强制 | `match` 编译器强制覆盖所有可能 |

```rust
let result = if score >= 60 { "及格" } else { "不及格" };
// if 是表达式，可以直接绑定到变量

// match 强制穷尽：
match state {
    ConnectionState::Disconnected => println!("未连接"),
    ConnectionState::Connecting { attempt } => println!("第 {attempt} 次重试"),
    ConnectionState::Connected(peer) => println!("已连接到 {peer}"),
    ConnectionState::Failed { reason, .. } => println!("失败: {reason}"),
    // 如果遗漏任何变体→编译错误
}
```

### 1.6 错误处理

| 主题 | C | C++ | Rust |
|------|---|-----|------|
| 可恢复错误 | 返回码、`errno` | 异常、`std::expected`（C++23）、错误码 | `Result<T, E>` |
| 不可恢复 | `abort()` | `std::terminate()` | `panic!` |
| 调用者忽略 | 容易忽略返回值 | 异常可被忽略（`catch(...)`） | `Result` 不能静默忽略（编译器警告） |
| 空值 | `NULL`（宏） | `nullptr` | `Option<T>` —— 无普遍存在的空引用 |
| 可选值 | 指针可能为 NULL | `std::optional`（C++17） | `Option<T>` |

**关键差异**：
- C 的错误码容易被遗忘；C++ 异常有控制流不可见的问题
- Rust 的 `Result<T, E>` 将失败编码进**类型签名**——调用者无法假装错误不发生
- Rust 的 `?` 运算符不吞没错误：它传播错误并可能自动转换类型

### 1.7 构建工具链

| 主题 | C/C++ 生态 | Rust |
|------|-----------|------|
| 编译器调用 | 直接调用 `gcc`/`clang`/`cl`，或通过构建系统 | `cargo build`（统一入口） |
| 包管理 | 多种选择（vcpkg, Conan, 系统包管理器），或手动管理 | Cargo 内置（crates.io） |
| 构建配置 | Makefile, CMake, Meson, Bazel 等 | `Cargo.toml`（TOML 格式） |
| 测试 | 多种框架（Google Test, Catch2, CTest） | `cargo test`（内置） |
| 文档 | Doxygen 等外部工具 | `cargo doc`（内置，生成 rustdoc HTML） |
| 格式化 | clang-format 等独立工具 | `cargo fmt`（rustfmt） |
| Lint | clang-tidy, cppcheck 等 | `cargo clippy`（内置 550+ lint） |

**说明**：
- C/C++ 生态的构建工具选择丰富，各有适用场景。CMake 是事实标准，但在跨平台复杂项目中仍有挑战。
- Cargo 将常见流程统一到一个入口，牺牲了部分定制性，换取了极高的一致性。
- Cargo 不覆盖所有复杂构建场景（如混合语言编译、自定义代码生成），这些场景需要 `build.rs` 或其他工具辅助。

---

## 2. 内存与资源管理对照

### 2.1 栈与堆

| 主题 | C | C++ | Rust |
|------|---|-----|------|
| 栈分配 | 局部变量、`alloca`（不推荐） | 局部变量 | 局部变量（默认） |
| 堆分配 | `malloc`/`calloc` | `new`/`delete`、容器、智能指针 | `Box::new()`、`Vec::new()` 等 |
| 释放 | `free`（手动） | `delete`（手动）或智能指针（自动） | 所有权 + `Drop`（自动） |
| 未释放 | 内存泄漏 | 可能泄漏（裸 `new` 无对应 `delete`） | 安全 Rust 中资源泄漏较少，但引用环仍可能导致泄漏 |

```rust
// Rust 中默认尝试在栈上分配
let x = 42;              // 栈上
let v = vec![1, 2, 3];   // Vec 结构体在栈上，元素在堆上

// 显式堆分配
let b = Box::new([0u8; 8192]);  // 大数组推荐放堆上
```

### 2.2 RAII 与 Drop

| 主题 | C | C++ | Rust |
|------|---|-----|------|
| RAII 支持 | 无语言支持 | 构造函数/析构函数（核心习惯用法） | `Drop` trait（作用域结束时自动调用） |
| 覆盖范围 | N/A | 内存、文件、锁、连接 | 内存、文件、锁、连接、临时目录、事务 |
| 手动提前释放 | `free()` | `delete`、`reset()`、作用域 `{}` | `drop(value)` 或让值离开作用域 `{}` |

**RAII 不是 Rust 独有的**。C++ 是 RAII 的起源语言。Rust 的贡献在于：将 RAII 与所有权系统深度结合，编译器能检查大量资源使用错误，而不仅仅依赖运行时的析构顺序。

```
资源管理流程对比：

C:
  申请资源 ──→ 手动释放 ──→ 遗漏时泄漏 / 重复释放 / 使用已释放资源

C++:
  构造对象 ──→ RAII 守卫持有资源 ──→ 析构时释放
                                  ──→ 但悬垂引用、use-after-move 仍可能发生

Rust:
  值获得所有权 ──→ 所有者离开作用域 ──→ Drop 自动运行
               ──→ 编译器同时检查：无悬垂引用、无 use-after-move
```

### 2.3 悬垂指针与内存安全

| 场景 | C | C++ | Rust（安全） |
|------|---|-----|-------------|
| 返回局部变量地址 | 未定义行为 | 未定义行为 | 编译错误 |
| use-after-free | 未定义行为 | 未定义行为（智能指针可缓解） | 编译错误 |
| double-free | 未定义行为 | 未定义行为（智能指针可缓解） | 编译错误（所有权系统） |
| 数组越界 | 未定义行为 | 未定义行为（`.at()` 抛异常） | panic（可恢复）或编译检查 |
| 空指针解引用 | 未定义行为 | 未定义行为 | `Option<T>` 强制检查 |
| 数据竞争 | 未定义行为 | 未定义行为 | 编译错误（`Send`/`Sync` + 借用规则） |

**说明**：
- C++ 的智能指针（`unique_ptr`, `shared_ptr`）大幅减少了这些问题，但不能完全消除（尤其在涉及原始引用和复杂所有权时）
- Rust 在 **安全代码** 中通过类型系统静态阻止这些行为。`unsafe` 代码中（如 FFI 边界），程序员需要承担与 C/C++ 类似的责任。

---

## 3. 指针、引用与借用对照

### 3.1 核心差异

| 写法 | 是否拥有对象 | 是否可为空 | 借用规则约束 | 安全解引用保证 |
|------|:---:|:---:|:---:|:---:|
| C `T*` | 通常否 | 可为 NULL | 否 | 语言不保证 |
| C++ `T*` | 通常否 | 可为 `nullptr` | 否 | 语言不保证 |
| C++ `T&` | 否 | 不应为空（可能发生） | 不等价于 Rust 借用 | 程序员保证 |
| Rust `&T` | 否 | 通常不为空 | **是**（共享借用规则） | **是** |
| Rust `&mut T` | 否 | 通常不为空 | **是**（独占借用规则） | **是** |
| Rust `*const T` / `*mut T` | 否 | 可为空 | 不受安全借用规则保护 | 需 `unsafe` |
| Rust `Box<T>` | **是** | 拥有堆对象 | 受所有权规则约束 | 通过安全接口访问 |

### 3.2 Rust 引用 ≠ C++ 引用

**Rust 的 `&T` 不仅仅是"受限的 C++ 引用"**：

- C++ 引用在创建后通常不能重新绑定，但它不拥有借用规则的静态验证
- C++ 中可以写 `int& r = getRef(); delete &r;` 并继续使用 `r`——这是未定义行为但编译通过
- Rust 的借用检查器**在编译期**验证所有引用始终有效

### 3.3 Rust 引用 ≠ C 指针

- C 指针可以进行算术运算；Rust 引用不能（需要裸指针）
- C 指针可以为 NULL；Rust 引用设计上不为空（`NonNull` 保证）
- C 指针不携带生命周期信息；Rust 引用类型包含生命周期参数

### 3.4 借用规则的动机：数据竞争

```rust
// Rust 阻止的模式（C/C++ 中合法但危险）：
let mut data = vec![1, 2, 3];
let r1 = &data;       // 共享借用
let r2 = &mut data;   // 编译错误！不能同时共享和独占借用
// r1 和 r2 同时存在时，修改 data 可能导致迭代器失效
```

### 3.5 切片不只是指针

```rust
// &[T] 是"胖指针"：包含起始地址 + 元素数量
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];  // 包含: 指针到 arr[1] + 长度 3

// &str 同理：包含数据地址 + 字节长度
let s: &str = "你好世界";  // 12 字节 + 长度信息
```

C/C++ 中传递数组往往靠"指针 + 长度"的传参约定；Rust 将两者打包为切片类型。

### 3.6 裸指针

Rust 保留了裸指针用于以下场景：
- FFI（与 C 库交互）
- 实现某些高性能数据结构
- 构建安全抽象的内部基础设施

```rust
let x = 42;
let raw: *const i32 = &x;
// let val = *raw;  // 编译错误！解引用裸指针需要 unsafe

unsafe {
    println!("{}", *raw);  // OK，但在 unsafe 块中
}
```

---

## 4. Move 语义对照

### 4.1 Rust Move 与 C++ Move：相似但不等价

**Rust Move（所有权转移）**：
- 对非 `Copy` 类型，赋值或传参后，**旧绑定失效**
- 编译器层面的静态规则——通常不涉及运行时代码生成
- `String` 的 Move：栈上的 {指针, 长度, 容量} ~24 字节被复制到新位置，旧位置被编译器标记为无效

```rust
let first = String::from("hello");
let second = first;
// println!("{first}"); // 编译错误：first 的所有权已转移给 second
println!("{second}");     // OK
```

**C++ Move（移动语义）**：
- `std::move` 将左值转换为右值引用，触发移动构造函数（如果存在）
- 被移动的对象通常处于"有效但未指定状态"（`valid but unspecified`）
- 访问被移动的对象不是编译错误，但通常应避免

```cpp
std::string first = "hello";
std::string second = std::move(first);
// first 现在可能为空，可以安全地重新赋值
first = "world";  // 合法
```

### 4.2 关键差异表

| 操作 | Rust 含义 | 是否显式 | 常见代价 |
|------|----------|:---:|------|
| Move | 转移所有权，旧绑定**失效** | 通常隐式 | 通常轻量（栈上字段复制） |
| Copy | 按位复制（仅限简单类型） | 隐式（类型实现 `Copy` 时） | 应适用于轻量类型 |
| Clone | 显式创建独立副本 | 显式调用 `.clone()` | 可能昂贵（堆分配、深拷贝） |
| Borrow | 临时借用，不转移所有权 | 显式写 `&` 或 `&mut` | 避免不必要复制 |

### 4.3 不要滥用 `clone()`

```rust
// ❌ 绕过设计：遇到所有权错误就用 clone
let s1 = String::from("hello");
let s2 = s1.clone();  // 真的需要独立副本吗？
let s3 = s1.clone();  // 又一份？

// ✅ 先思考：是否可以用借用？
let s1 = String::from("hello");
let s2 = &s1;  // 借用，无分配
let s3 = &s1;  // 可以同时存在多个共享借用
```

`clone()` 有明确的适用场景（需要独立所有权时），但不是解决所有权问题的通用方案。高频使用 `clone()` 通常意味着对所有权模型需要重新理解。

---

## 5. Trait 对照

### 5.1 Trait 与相关概念

| 概念 | Rust Trait 与之的关系 |
|------|---------------------|
| C++ 抽象基类 | Trait 类似纯虚函数接口，但 Trait 不包含字段，可以有默认实现 |
| C++ Concepts（C++20） | Trait Bound 与 Concepts 目的相似（约束泛型参数），但 Rust 在语言层面强制检查 |
| C++ 模板约束 | Trait Bound 是声明式的；C++ 模板约束更多是事后检查（SFINAE、Concepts） |
| Python 鸭子类型 | Trait 是编译期的"结构类型"检查，不是运行时 |

### 5.2 Trait 不等于类继承

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("画一个半径为 {} 的圆", self.radius); }
}

impl Drawable for Rectangle {
    fn draw(&self) { println!("画 {}x{} 的矩形", self.width, self.height); }
}

// 静态分派：
fn render_static<T: Drawable>(item: &T) { item.draw(); }

// 动态分派（Trait Object）：
fn render_dynamic(item: &dyn Drawable) { item.draw(); }
```

**核心区别**：
- Trait 实现与类型定义分离（孤儿规则控制谁能实现）
- Trait 不创建"is-a"继承关系——`Circle` 不"继承"`Drawable`
- Trait Object 需要显式通过 `dyn Trait` 使用

### 5.3 选择表

| 需求 | 优先考虑 |
|------|---------|
| 编译期已知具体类型，重视零成本抽象 | 泛型 + Trait Bound（静态分派） |
| 需要保存多种实现到同一集合 | `Box<dyn Trait>` 等 Trait Object（动态分派） |
| 状态集合封闭且变体较少 | Enum（不需要 Trait） |
| 行为共享但不需要继承树 | Trait + 组合 |
| 扩展第三方类型的能力 | Trait（孤儿规则允许范围内） |

---

## 6. 智能指针对照

| 意图 | C++ 常见工具 | Rust 常见工具 | 注意事项 |
|------|-------------|-------------|---------|
| 独占堆对象 | `std::unique_ptr<T>` | `Box<T>` | 类比有帮助，但销毁时机确定性和 Move 语义有所不同 |
| 单线程共享所有权 | `std::shared_ptr<T>` | `Rc<T>` | `Rc<T>` 不是线程安全的——它甚至没有实现 `Send` |
| 多线程共享所有权 | `std::shared_ptr<T>` + 同步 | `Arc<T>` | `Arc<T>` 只解决引用计数原子性，不自动保证内部数据线程安全 |
| 弱引用 | `std::weak_ptr<T>` | `Weak<T>` | 用于避免引用环 |
| 内部可变性（单线程）| `mutable` 关键字、`const_cast` | `Cell<T>`、`RefCell<T>` | `RefCell` 将借用检查推迟到运行时 |
| 内部可变性（多线程）| 互斥锁 + `shared_ptr` | `Mutex<T>`、`RwLock<T>` | `Arc<Mutex<T>>` 是常见组合 |

### 6.1 `Arc<T>` 不等于自动线程安全

```rust
use std::sync::Arc;

let data = Arc::new(42);
// 多个线程可以安全地共享 data（引用计数是原子的）
// 但不能修改 data（Arc 只提供 &T 访问）

// 需要通过 Arc<Mutex<T>> 来安全修改
let shared = Arc::new(std::sync::Mutex::new(42));
```

### 6.2 `RefCell<T>` 不是"关闭借用检查器"

`RefCell<T>` 将借用规则从编译期推迟到运行时：
- `borrow()` → 获取不可变借用（可多个，运行时会检查）
- `borrow_mut()` → 获取可变借用（仅一个，运行时会检查）
- 违反规则 → **运行时 panic**（不是无声地允许错误行为）

```rust
use std::cell::RefCell;

let cell = RefCell::new(42);
let r1 = cell.borrow_mut();
// let r2 = cell.borrow();  // 运行时 panic！已经存在可变借用
```

### 6.3 决策树

```text
是否需要堆上拥有数据？
├─ 是，只有一个所有者 → Box<T>
└─ 需要共享所有权
   ├─ 仅单线程 → Rc<T>
   └─ 可能跨线程 → Arc<T>

共享之后是否需要修改内部数据？
├─ 不需要 → 直接共享
├─ 单线程，接受运行时借用检查 → RefCell<T>
└─ 多线程，需要同步 → Mutex<T> / RwLock<T>
```

---

## 7. 泛型与模板对照

### 7.1 核心差异

| 主题 | C++ 模板 | Rust 泛型 |
|------|---------|----------|
| 约束表达 | Concepts (C++20) 或隐式（SFINAE） | Trait Bound（强制声明） |
| 类型检查时机 | 实例化时（延迟检查） | 声明时（提前检查） |
| 错误信息 | 历史上极其冗长（Concepts 改善中） | 通常较清晰（指向违反的 Trait Bound） |
| 编译期多态 | 模板特化、SFINAE、CRTP | 泛型 + Trait Bound（单态化） |
| 运行时多态 | 虚函数（vtable） | `dyn Trait`（Trait Object） |
| 代码生成 | 单态化（模板实例化） | 单态化（类型擦除后生成） |
| 编译时间 | 可能很长（大量模板展开） | 通常较快（Trait Bound 减少推测） |

### 7.2 单态化不是零代价

Rust 的泛型通过单态化为每种具体类型生成独立代码：
- 优点：无虚函数调用开销，可内联优化
- 代价：代码体积增加（每种类型一份副本），编译时间增加

```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 编译器生成：
// fn max_i32(a: i32, b: i32) -> i32 { ... }
// fn max_f64(a: f64, b: f64) -> f64 { ... }
// 每份副本可能被独立内联和优化
```

### 7.3 Trait Object 的代价

```rust
// 静态分派：编译期确定，无间接调用
fn process_static<T: Drawable>(item: &T) { item.draw(); }

// 动态分派：运行时通过 vtable 查找，有间接调用开销
fn process_dynamic(item: &dyn Drawable) { item.draw(); }
```

---

## 8. Unsafe 与安全边界

### 8.1 `unsafe` 不是什么

| 常见误解 | 准确说法 |
|---------|---------|
| `unsafe` 关闭所有检查 | `unsafe` 只开放 5 种能力（解引用裸指针、调用 unsafe 函数、访问可变静态变量、实现 unsafe trait、访问 union 字段） |
| `unsafe` 代码一定不安全 | `unsafe` 代码的责任是程序员需要证明额外的安全前提——编译器信任程序员 |
| 应该避免所有 `unsafe` | 系统编程中某些操作必须用 `unsafe`，关键是将其封装在小而可审计的边界中 |

### 8.2 安全抽象模式

```rust
/// 安全抽象：对外提供安全接口
pub fn safe_split_at<T>(slice: &[T], mid: usize) -> (&[T], &[T]) {
    assert!(mid <= slice.len());  // 安全前提检查
    // SAFETY: mid <= len 已验证，两个子切片不重叠
    unsafe {
        (
            std::slice::from_raw_parts(slice.as_ptr(), mid),
            std::slice::from_raw_parts(slice.as_ptr().add(mid), slice.len() - mid),
        )
    }
}
```

### 8.3 FFI 考虑事项

从 C/C++ 迁移到 Rust 时，FFI 边界需要特别关注：
- **数据布局**：`#[repr(C)]` 确保与 C 兼容的内存布局
- **空指针**：C 函数可能返回 NULL，需转为 `Option<&T>` 或检查
- **所有权**：明确哪一方负责释放（Rust 的 `Box` vs C 的 `free`）
- **字符串编码**：C 字符串以 `\0` 结尾；Rust 使用带长度的 UTF-8
- **错误约定**：C 通常返回错误码，Rust 可包装为 `Result`
- **回调生命周期**：C 函数指针不携带生命周期信息，需手动保证

---

## 9. 并发对照

| 主题 | C | C++ | Rust |
|------|---|-----|------|
| 线程创建 | pthread / `thrd_create` (C11) | `std::thread` | `std::thread::spawn` |
| 互斥锁 | pthread mutex | `std::mutex` | `std::sync::Mutex<T>` **包装数据** |
| 数据竞争检测 | 工具（ThreadSanitizer） | 工具（ThreadSanitizer） | 编译期（`Send`/`Sync` trait） |
| 消息传递 | 手动实现或库 | `std::queue` + 互斥 | `std::sync::mpsc::channel` |
| 并发保证 | 程序员负责 | 程序员负责（更丰富的标准库帮助） | 类型系统强制部分保证 |

### 9.1 Rust 避免了什么，没避免什么

**Rust 编译期防止**（安全代码中）：
- 数据竞争（data race）
- 忘记加锁就访问共享数据（`Mutex<T>` 包装数据）
- 跨线程发送非线程安全的类型（`Send` trait 检查）

**Rust 不会自动防止**：
- 死锁（deadlock）
- 活锁（livelock）
- 饥饿（starvation）
- 锁粒度不合理导致的性能问题
- 业务逻辑错误

### 9.2 `Send` 与 `Sync`

```rust
// Send: 值可以安全转移到另一个线程
// 大多数类型都是 Send。Rc<T> 不是 Send

// Sync: 对该类型的共享引用可以安全地跨线程使用
// &T 需要是 Sync 才能在线程间共享
// Mutex<T> 是 Sync（当 T 是 Send 时）
```

---

## 10. 构建与依赖管理对照

| 操作 | C/C++ 常见方式 | Rust |
|------|--------------|------|
| 添加依赖 | 手动下载、包管理器、git submodule | 编辑 `Cargo.toml` → 自动下载 |
| 版本管理 | 多种策略（无统一方式） | SemVer + `Cargo.lock` |
| 条件编译 | `#ifdef` / `#ifndef` | `#[cfg(feature = "xxx")]` |
| 可选依赖 | 手动管理 | `[features]` + 可选依赖 |
| Profile | Debug/Release 自定义 | `[profile.dev]` / `[profile.release]` |

Cargo Feature 是**编译期能力选择**，不是运行时开关：

```toml
[features]
default = ["std"]
std = []
json = ["serde", "serde_json"]  # 启用 json 需要 serde 依赖
```

---

## 11. 需要纠正的常见类比

以下类比常见但不准确：

| 常见不当类比 | 为什么不等价 | 准确说法 |
|-------------|-------------|---------|
| "Rust 的 Move 就是 C++ `std::move`" | C++ `std::move` 只是值类别转换；Rust Move 是编译期所有权转移，旧绑定失效 | Rust Move 和 C++ Move 解决相似问题，机制不同 |
| "Rust 的引用就是 C++ 引用" | Rust 引用受借用规则静态验证，C++ 引用没有等价保护 | Rust 引用是受借用规则约束的非拥有型访问 |
| "Rust 的 Trait 就是 C++ 抽象类" | Trait 不包含字段、无构造/析构、不形成继承树 | Trait 是行为约束，通过组合使用 |
| "生命周期标注就是手动管理内存" | 生命周期标注**不负责分配或释放**，只描述引用关系 | 生命周期标注帮助编译器验证引用有效性 |
| "Rust 的 `Arc` 就是 C++ `shared_ptr`" | `Arc` 不提供内部可变性（需配合 `Mutex`）；`shared_ptr` 的控制块与 `Arc` 的内部实现不同 | 目的相似，组合方式和约束不同 |
| "Rust 泛型就是 C++ 模板" | 约束机制、类型检查时机、编译模型不同 | 两者都通过单态化实现编译期多态，但语言机制不同 |

---

## 12. 学习建议

如果你有 C/C++ 背景，学习 Rust 时：

1. **先忘掉"翻译"思维**——不要试图把 C++ 代码逐行翻译成 Rust
2. **重新理解变量**——Rust 的变量是"所有权绑定"，不是内存槽位
3. **接受 `mut` 和 `&` 的显式性**——它们是设计，不是繁琐
4. **信任编译器**——被拒绝的代码模式通常对应真实的风险
5. **不要绕过问题**——`clone()` 和 `unsafe` 不是学习捷径
6. **Rust 的"难"集中于前期**——一旦掌握所有权和借用，后续学习加速

---

## 扩展阅读

- [Rust for C++ developers](https://github.com/nrc/r4cppp) — Nick Cameron 的面向 C++ 学习者的 Rust 指南
- [Learn Rust the Dangerous Way](http://cliffle.com/p/dangerust/) — 从 C 的角度理解 Rust
- [Rust 官方 FFI 文档](https://doc.rust-lang.org/nomicon/ffi.html) — 深入 FFI 细节
- 本教程的 [MENTAL_MODELS.md](MENTAL_MODELS.md) — 建立 Rust 思维模型
- 本教程的 [MISCONCEPTIONS.md](MISCONCEPTIONS.md) — 常见误解澄清
