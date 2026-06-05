# 第八章：结构体与方法 — 自定义数据类型和行为

## 目录

1. [本章目标](#本章目标)
2. [为什么需要学习这一章](#为什么需要学习这一章)
3. [背景知识](#背景知识)
4. [核心术语](#核心术语)
5. [项目结构](#项目结构)
6. [运行命令与预期输出](#运行命令与预期输出)
7. [代码讲解](#代码讲解)
    - [7.1 结构体定义（Struct Definition）](#71-结构体定义struct-definition)
    - [7.2 impl 块：方法与关联函数](#72-impl-块方法与关联函数)
    - [7.3 方法 vs 关联函数](#73-方法-vs-关联函数)
    - [7.4 元组结构体（Tuple Structs）](#74-元组结构体tuple-structs)
    - [7.5 单元结构体（Unit-Like Structs）](#75-单元结构体unit-like-structs)
    - [7.6 Debug 派生宏](#76-debug-派生宏)
    - [7.7 结构体更新语法](#77-结构体更新语法)
    - [7.8 字段初始化简写](#78-字段初始化简写)
8. [与 Python 的对照](#与-python-的对照)
    - [8.1 Python class vs Rust struct + impl](#81-python-class-vs-rust-struct--impl)
    - [8.2 `__init__` vs `new()`](#82-__init__-vs-new)
    - [8.3 self 参数对比](#83-self-参数对比)
    - [8.4 综合对照表](#84-综合对照表)
9. [常见错误](#常见错误)
    - [9.1 忘记 `pub`](#91-忘记-pub)
    - [9.2 方法所有权混淆](#92-方法所有权混淆)
    - [9.3 缺少 `#[derive(Debug)]`](#93-缺少-derivedebug)
    - [9.4 `&mut self` 与不可变变量](#94-mut-self-与不可变变量)
    - [9.5 结构体更新语法的所有权问题](#95-结构体更新语法的所有权问题)
10. [本章小结](#本章小结)
11. [下一章衔接](#下一章衔接)

---

## 本章目标

通过本章的学习，你将能够：

1. **定义三种结构体**：命名字段结构体（Named Field Struct）、元组结构体（Tuple Struct）、单元结构体（Unit-Like Struct）
2. **理解 `impl` 块**：将数据（结构体）与行为（方法/关联函数）组织在一起
3. **区分方法与关联函数**：方法接收 `self`/`&self`/`&mut self`，关联函数不接收 `self`
4. **掌握三种 self 接收者**：`&self` 不可变借用、`&mut self` 可变借用、`self` 所有权转移
5. **使用派生宏 `#[derive(Debug)]`** 自动实现 Debug trait，支持 `{:?}` 和 `{:#?}` 格式化输出
6. **比较 Rust struct 与 Python class**：理解两种语言在数据封装上的相同与不同

---

## 为什么需要学习这一章

在 Python 中，当你需要表示一个自定义数据类型时，你会写一个 `class`：

```python
class Rectangle:
    def __init__(self, width, height):
        self.width = width
        self.height = height

    def area(self):
        return self.width * self.height
```

Rust 不是面向对象语言，没有"类"的概念。但 Rust 提供了另一种方式：**结构体（struct）** 存储数据，**impl 块** 定义行为。这种分离使得数据和行为的关系更加清晰和灵活。

学习本章是进入 Rust 实战的**分水岭**。在此之后，你将告别只使用原始类型（primitive types）的阶段，开始构建自己的领域类型——就像 Python 开发者用 `class` 构建自己的抽象一样。

---

## 背景知识

### 从"数据 + 函数"到"结构体 + 方法"

在第七章（枚举与模式匹配）中，我们学习了如何用 `enum` 表达"这一种或那一种"的联合类型。但很多时候，我们需要一个**同时包含多个字段**的复合类型——这就是结构体。

回顾前面章节，我们已经见过一种结构体：`String`。`String` 本质上就是一个结构体，它内部包含指向堆内存的指针、长度和容量三个字段。现在，我们要学会**自己定义结构体**。

### 为什么数据和方法要分开？

在许多面向对象语言中，类把字段和方法耦合在一个定义体内。Rust 选择将这两者分离：

- **struct**：定义数据布局（data layout）
- **impl**：定义行为（behavior）

这种分离的好处：

1. **灵活性**：你可以为同一个 struct 编写多个 `impl` 块，甚至分布在不同的文件/模块中
2. **可见性控制**：数据和方法的 `pub` 可以独立控制
3. **trait 实现分离**：自己的方法和 trait 方法写在不同的 `impl` 块中，一目了然

---

## 核心术语

| 术语（English） | 中文 | 解释 |
|---|---|---|
| **Struct** | 结构体 | 自定义数据类型，将多个相关字段组合在一起 |
| **Field** | 字段 | 结构体中的数据成员，有名称和类型 |
| **Named Field Struct** | 命名字段结构体 | 最常见的结构体，每个字段有名字（如 `width: f64`） |
| **Tuple Struct** | 元组结构体 | 字段没有名字，按位置索引（如 `Point(f64, f64, f64)`） |
| **Unit-Like Struct** | 单元结构体 | 没有任何字段的结构体，用作类型标记（如 `struct ConfigMarker;`） |
| **Method** | 方法 | 以 `self`/`&self`/`&mut self` 为第一参数的函数，通过实例调用 |
| **Associated Function** | 关联函数 | 定义在 `impl` 块中但不接收 `self` 的函数，通过类型名调用（如 `Rectangle::square()`） |
| **impl** | 实现块 | `impl TypeName { ... }`，为类型定义方法和关联函数 |
| **self** | 自身（所有权） | 方法接收者，表示获取 `self` 的所有权 |
| **&self** | 自身引用（不可变借用） | 方法接收者，表示只读借用 `self` |
| **&mut self** | 自身可变引用（可变借用） | 方法接收者，表示可变借用 `self` |
| **Derive Macro** | 派生宏 | 形如 `#[derive(Debug)]` 的属性，让编译器自动生成 trait 实现 |
| **Debug** | Debug trait | 格式化 trait，支持 `{:?}`（紧凑）和 `{:#?}`（美化）输出 |

---

## 项目结构

```
08_structs_methods_associated_functions/
├── Cargo.toml          # 包元数据，edition = "2024"
├── src/
│   └── main.rs         # 完整示例代码（~200 行）
├── README.md           # 本章讲解（本文件）
└── EXERCISES.md        # 练习题
```

### Cargo.toml

```toml
[package]
name = "structs_and_methods"
version = "0.1.0"
edition = "2024"
description = "结构体与方法 - 自定义数据类型和行为"
```

> **注意**：edition 设定为 `"2024"`，这是 Rust 的最新版本，需要 Rust 1.85+ 编译器。

---

## 运行命令与预期输出

### 编译并运行

```bash
cd 08_structs_methods_associated_functions
cargo run
```

### 仅编译检查（不运行）

```bash
cargo check    # 快速检查语法和类型
cargo build    # 完整编译
```

### 预期输出

```
===== 第八章：结构体与方法 =====

--- 字段初始化简写 ---
rect1 = Rectangle { width: 30.0, height: 20.0 }

--- 方法调用 ---
area      = 600.00
perimeter = 100.00
diagonal  = 36.06

--- 关联函数 ---
square (15x15) = Rectangle {
    width: 15.0,
    height: 15.0,
}
square area    = 225.00

--- 结构体更新语法 ---
rect2 = Rectangle { width: 50.0, height: 20.0 }

--- can_hold 检查 ---
large 能容纳 small?  true
small 能容纳 large?  false
rect2 能容纳 rect1? true
square 能容纳 tall?  false

--- &mut self 可变借用 ---
缩放前: Rectangle { width: 5.0, height: 5.0 }
scale(3.0) 后: Rectangle { width: 15.0, height: 15.0 }
grow(1.5, 2.0) 后: Rectangle { width: 16.5, height: 17.0 }

--- self 所有权方法 ---
原始: Rectangle { width: 7.0, height: 4.0 }
double() 后: Rectangle { width: 14.0, height: 8.0 }
8x6 加 margin 2.0 后: Rectangle { width: 12.0, height: 10.0 }

--- 元组结构体 ---
p1     = Point(3.0, 4.0, 0.0)
p2     = Point(6.0, 8.0, 12.0)
origin = Point(0.0, 0.0, 0.0)
p1 到原点距离 = 5.0000
p1 到 p2 距离  = 13.0000
p1 double 后   = Point(6.0, 8.0, 0.0)
p1.x = 3.0, p1.y = 4.0, p1.z = 0.0

--- 单元结构体 ---
ConfigMarker   = ConfigMarker
DebugModeEnabled = DebugModeEnabled
size_of::<ConfigMarker>() = 0 byte(s)

--- Debug 格式对比 ---
{:?}   = Rectangle { width: 12.34, height: 56.78 }
{:#?} = Rectangle {
    width: 12.34,
    height: 56.78,
}

--- 综合：矩形数组 ---
  [0] Rectangle { width: 10.0, height: 5.0 }  area=50.0  perimeter=30.0  diag=11.18
  [1] Rectangle { width: 7.0, height: 7.0 }  area=49.0  perimeter=28.0  diag=9.90
  [2] Rectangle { width: 3.0, height: 9.0 }  area=27.0  perimeter=24.0  diag=9.49
  [3] Rectangle { width: 15.0, height: 12.0 }  area=180.0  perimeter=54.0  diag=19.21

===== 第八章演示结束 =====
```

---

## 代码讲解

### 7.1 结构体定义（Struct Definition）

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
```

**关键点**：

- `struct` 关键字声明一个结构体类型 `Rectangle`
- 花括号 `{}` 内是命名字段（named fields），每个字段格式为 `name: Type`
- `#[derive(Debug)]` 是一个属性（attribute），它让编译器自动为 `Rectangle` 生成 `Debug` trait 的实现代码
- 字段名使用 **snake_case**（Rust 惯例）
- 字段类型使用 `f64`（64 位浮点数）

**为什么使用 `f64`？** 因为 `f64` 是 Rust 浮点运算的默认推荐类型，在现代 CPU 上性能与 `f32` 相当，但精度更高。

### 7.2 impl 块：方法与关联函数

```rust
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self { ... }
    pub fn square(size: f64) -> Self { ... }
    pub fn area(&self) -> f64 { ... }
    pub fn scale(&mut self, factor: f64) { ... }
    pub fn double(self) -> Self { ... }
}
```

**关键点**：

- `impl Rectangle { }` 为 `Rectangle` 类型添加方法实现
- `Self`（大写 S）在 `impl` 块中是 `Rectangle` 的类型别名
- 一个类型可以有多个 `impl` 块——这在实现不同 trait 时尤其有用

### 7.3 方法 vs 关联函数

这是最容易混淆的地方，让我们彻底搞清楚。

#### 关联函数（Associated Function）

**定义**：`impl` 块中，第一参数不是 `self`/`&self`/`&mut self` 的函数。

```rust
pub fn new(width: f64, height: f64) -> Self { ... }
pub fn square(size: f64) -> Self { ... }
```

**调用方式**：使用 `::` 语法（双冒号），前缀是类型名。

```rust
let rect = Rectangle::new(10.0, 20.0);      // 关联函数调用
let sq = Rectangle::square(15.0);           // 关联函数调用
```

`new()` 是最常见的关联函数——它像 Python 的 `__init__` 一样用于构造实例，但请注意：
- Rust 的 `new()` 只是一个约定名称，不是语言关键字
- 你可以叫它 `create()`、`from_size()` 或任何名字
- `new()` 返回 `Self`（即 `Rectangle`），通常但不一定返回新实例

#### 方法（Method）

**定义**：`impl` 块中，第一参数是 `self`/`&self`/`&mut self` 的函数。

```rust
pub fn area(&self) -> f64 { ... }           // &self 方法
pub fn scale(&mut self, factor: f64) { ... } // &mut self 方法
pub fn double(self) -> Self { ... }          // self 方法
```

**调用方式**：使用 `.` 语法（点号），前缀是实例。

```rust
let area = rect.area();        // &self 方法 — 只读
rect.scale(2.0);               // &mut self 方法 — 可变，rect 必须是 mut
let new_rect = rect.double();  // self 方法 — 消费 rect，rect 之后不可用
```

#### 三种 self 接收者对比

| 接收者 | 含义 | 调用后原变量 | 典型用途 |
|---|---|---|---|
| `&self` | 不可变借用（只读引用） | 仍可使用 | 查询方法：`area()`、`perimeter()`、`can_hold()` |
| `&mut self` | 可变借用（可写引用） | 仍可使用 | 修改方法：`scale()`、`grow()` |
| `self` | 所有权转移（移动） | **不可再使用** | 消耗型转换：`double()`、`with_margin()` |

**记忆口诀**：
- `&self` — "我只需要看一看"
- `&mut self` — "我需要改一改"
- `self` — "我拿走了，你不能再用了"

### 7.4 元组结构体（Tuple Structs）

```rust
#[derive(Debug)]
struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}
```

**关键点**：

- 元组结构体的字段没有名字，用 `self.0`、`self.1`、`self.2` 按位置访问
- 它是一种"命名的元组"——有类型名，但字段无名
- 适合字段含义直观且不需要命名的场景（如坐标、RGB 颜色）

**何时使用元组结构体？**

- 字段数量少（通常不超过 3 个）
- 字段含义从类型名就能推断（`Point(x, y, z)` 很明显）
- 不需要字段级别的文档注释

### 7.5 单元结构体（Unit-Like Structs）

```rust
#[derive(Debug)]
struct ConfigMarker;

#[derive(Debug)]
struct DebugModeEnabled;
```

**关键点**：

- 没有 `()` 也没有 `{}`，只有一个名字和分号
- 大小为零字节（`std::mem::size_of::<ConfigMarker>() == 0`），不消耗内存
- 用作类型级别的标记（type-level marker）

**典型用途**：

1. **实现 trait 但不需数据**：比如为 `ConfigMarker` 实现某个 trait
2. **类型状态模式**：让编译器在不同状态间做类型检查
3. **事件标记**：在消息系统中区分不同类型的事件

### 7.6 Debug 派生宏

```rust
#[derive(Debug)]
struct Rectangle { ... }
```

**什么是派生宏？** 派生宏（derive macro）是一种代码生成机制。你写 `#[derive(Debug)]`，编译器在编译时自动生成 `Debug` trait 的实现。

**两种输出格式**：

```rust
println!("{:?}", rect);    // 紧凑格式：Rectangle { width: 30.0, height: 20.0 }
println!("{:#?}", rect);   // 美化格式：多行缩进输出
```

**对比**：

```
// {:?} — 紧凑：适合日志和调试
Rectangle { width: 12.34, height: 56.78 }

// {:#?} — 美化：适合阅读嵌套结构
Rectangle {
    width: 12.34,
    height: 56.78,
}
```

**其他常用派生宏**：
- `#[derive(Clone)]` — 自动实现深拷贝
- `#[derive(Copy)]` — 自动实现位拷贝（仅适用于简单类型）
- `#[derive(PartialEq)]` — 自动实现 `==` 比较
- `#[derive(Default)]` — 自动实现默认值

### 7.7 结构体更新语法

```rust
let rect2 = Rectangle {
    width: 50.0,   // 覆盖此字段
    ..rect1         // 其余字段从 rect1 复制
};
```

`..rect1` 表示"其余字段从 `rect1` 取值"。这类似于 JavaScript 的 spread 语法 `{...rect1, width: 50}`。

> **注意**：`..rect1` 会移动（move）那些未实现 `Copy` 的字段。因为 `f64` 实现了 `Copy`，所以本例中 `rect1` 之后仍可使用。如果字段是 `String`，则 `rect1` 会被部分移动而不可再用。

### 7.8 字段初始化简写

```rust
let width = 30.0;
let height = 20.0;

// 全写（冗余）
let rect1 = Rectangle {
    width: width,
    height: height,
};

// 简写（当变量名和字段名相同时）
let rect1 = Rectangle { width, height };
```

当变量名与字段名完全一致时，Rust 允许省略 `: value` 部分。这与 JavaScript 的 `{ width, height }` 对象简写语法相似。

---

## 与 Python 的对照

### 8.1 Python class vs Rust struct + impl

#### Python 版本

```python
class Rectangle:
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

    def area(self) -> float:
        return self.width * self.height

    def scale(self, factor: float) -> None:
        self.width *= factor
        self.height *= factor

    @classmethod
    def square(cls, size: float) -> 'Rectangle':
        return cls(size, size)
```

#### Rust 版本

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    pub fn square(size: f64) -> Self {
        Self { width: size, height: size }
    }
}
```

**核心差异**：

1. **定义位置**：Python 中数据（`self.width`）和方法在同一个 `class` 体内。Rust 中数据在 `struct` 中，方法在 `impl` 块中——两者分离。
2. **构造**：Python 用 `__init__`（在 `class` 内），Rust 用关联函数 `new()`（在 `impl` 块内），`new()` 只是约定，不是关键字。
3. **类型注解**：Python 的类型注解是可选的、运行时不影响行为。Rust 的类型是编译时强制的。
4. **self 语法**：Python 的 `self` 必须显式写为第一参数。Rust 的 `self`/`&self`/`&mut self` 也是第一参数，但简洁得多——不需要标注类型。

### 8.2 `__init__` vs `new()`

| 特性 | Python `__init__` | Rust `new()` |
|---|---|---|
| 是否关键字 | 是，magic method | 否，仅为约定 |
| 调用方式 | `Rectangle(10, 20)` | `Rectangle::new(10.0, 20.0)` |
| 返回值 | `None`（修改 `self`） | `Self`（新实例） |
| 可重载 | 仅一个 `__init__` | 可定义多个构造关联函数 |
| 可见性 | 始终公开 | 可 `pub` 或私有 |

Rust 的优势在于可以定义多个"构造函数"：

```rust
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self { ... }
    pub fn square(size: f64) -> Self { ... }
    pub fn from_diagonal(diag: f64) -> Self { ... }  // 额外构造器
}
```

在 Python 中实现类似效果通常需要 `@classmethod` 或工厂函数：

```python
@classmethod
def square(cls, size: float) -> 'Rectangle':
    return cls(size, size)
```

### 8.3 self 参数对比

| Rust | Python | 含义 |
|---|---|---|
| `&self` | `self`（只读方法） | 不可变借用，只能读不能改 |
| `&mut self` | `self`（修改方法） | 可变借用，可以修改字段 |
| `self` | 无直接对应 | 获取所有权，原变量不可用 |

Python 的 `self` 本质上总是一个引用（Python 中一切都是引用），但没有所有权概念。Python 也不能在方法中"消费"对象——即使在 `__del__` 中，你也无法阻止外部继续持有引用。

Rust 的 `self`（所有权转移）在 Python 中没有直接对应。最接近的概念是 "move" 语义的自定义类型，但 Python 依赖垃圾回收，不提供编译期的所有权移动。

### 8.4 综合对照表

| 概念 | Python | Rust |
|---|---|---|
| 数据定义 | `class` 内 `self.attr` + `__init__` | `struct` 定义字段 |
| 方法组 | `class` 内 `def method(self)` | `impl` 块内 `fn method(&self)` |
| 构造器 | `__init__(self, ...)` | `fn new(...) -> Self`（约定） |
| 静态方法 | `@staticmethod` | 关联函数（无 self 参数） |
| 类方法 | `@classmethod` | 关联函数（无 self，用 `Self` 类型） |
| 打印 | `__repr__` / `__str__` | `#[derive(Debug)]` / `Display` trait |
| 私有成员 | `_convention` / `__name_mangling` | `pub` 可见性修饰符 |
| 继承 | 单继承 + 多继承 + MRO | 无继承，用 trait + 组合 |
| 实例化 | `ClassName(args)` | `TypeName::new(args)` 或 `TypeName { fields }` |

---

## Python、C 与 C++ 对照

如果你有 C 或 C++ 的经验，Rust 的结构体和方法系统与你熟悉的 OOP 模式之间既有表面相似，也有根本分歧。

### 1. Rust struct + impl vs C struct vs C++ class

**C 的 struct** 是纯粹的数据容器——没有方法，没有可见性控制，没有构造/析构：

```c
typedef struct {
    double width;
    double height;
} Rectangle;

// 函数只能放在外面，以结构体为参数
double rect_area(const Rectangle* r) {
    return r->width * r->height;
}
```

**C++ 的 class** 将数据和行为捆绑在一起，并提供继承、多态、访问控制等全套 OOP 机制：

```cpp
class Rectangle {
    double width, height;
public:
    Rectangle(double w, double h) : width(w), height(h) {}
    double area() const { return width * height; }
};
```

**Rust 走了第三条路**：数据在 `struct` 里，行为在 `impl` 块里——两者是分离的：

```rust
struct Rectangle { width: f64, height: f64 }

impl Rectangle {
    pub fn area(&self) -> f64 { self.width * self.height }
}
```

这不只是语法上的"把方法挪到外面"。分离意味着你可以为一个 struct 写多个 `impl` 块（分布在不同的文件/模块中），也可以独立控制每个方法和每个字段的可见性。当你为某个 trait 写 `impl` 时，trait 方法与自身方法自然地隔离在不同 `impl` 块中——一目了然。

### 2. 方法接收者：显式 self vs 隐式 this

C++ 的方法隐式携带 `this` 指针，你不需要在签名中声明它：

```cpp
class Rectangle {
public:
    double area() const { return width * height; }
    // const 修饰符表示 this 是 const Rectangle*，但不显式出现
};
```

Rust 要求你**显式声明**方法的接收者：

```rust
impl Rectangle {
    pub fn area(&self) -> f64 { self.width * self.height }
    // &self 是显式的——等价于 self: &Self
    pub fn scale(&mut self, factor: f64) { self.width *= factor; }
    // &mut self 表示可变借用
    pub fn into_parts(self) -> (f64, f64) { (self.width, self.height) }
    // self 获取所有权，调用后原变量不可用
}
```

Rust 不提供编译时隐式修改的 `const` 修饰符——`&self` vs `&mut self` 在类型层面就区分了只读和可写。更重要的是，`self`（所有权转移）在 C++ 中没有直接对应——C++ 的"移动语义"需要 `std::move` + 右值引用，而 Rust 将所有权作为语言的一等概念嵌入到了方法签名中。

### 3. 没有类继承体系

C++ 的核心 OOP 机制基于继承——你可以从一个基类派生，获得它的字段和方法，再通过虚函数实现多态：

```cpp
class Shape {
public:
    virtual double area() const = 0;
    virtual ~Shape() = default;
};
class Circle : public Shape { ... };
class Rectangle : public Shape { ... };
```

Rust **没有类继承**。你不能定义一个 struct "继承自"另一个 struct。Rust 使用两种替代手段来达成代码复用和多态：

- **组合（Composition）**：一个 struct 包含另一个 struct 的实例作为字段。
- **Trait**：定义共享的行为接口，任何类型可以独立实现它们。通过 `dyn Trait` 实现动态分发，通过泛型 + trait bound 实现静态分发。

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
}

struct Rectangle { width: f64, height: f64 }
impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}
```

这种设计避免了 C++ 继承体系的经典痛点——菱形继承、虚基类、运行时开销难以预测等问题。trait 让接口和实现保持正交：任何类型在任何地方都可以实现一个 trait，而不需要修改类型的原始定义。trait 的深入讲解将在后续章节展开，眼下只需记住：**Rust 用组合承载数据，用 trait 承载行为——两者各司其职，互不侵入**。

## 常见错误

### 9.1 忘记 `pub`

```rust
// 错误：字段和方法默认私有
struct Rectangle {
    width: f64,   // 模块外不可访问
    height: f64,  // 模块外不可访问
}

impl Rectangle {
    fn area(&self) -> f64 { ... }  // 模块外不可调用
}
```

**解决方案**：明确标注 `pub`。

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 { ... }
}
```

> Rust 的默认可见性是私有的。这可能与 Python 的习惯不同——Python 默认一切公开，靠 `_` 前缀约定"私有"。

### 9.2 方法所有权混淆

```rust
let rect = Rectangle::new(10.0, 20.0);
let big = rect.double();  // double() 接收 self，rect 所有权被移动
println!("{rect:?}");     // 编译错误！rect 已不可用
```

**常见误解**：新手可能认为 `double()` 只是"返回一个新矩形"，没想到它会消耗原变量。

**规则**：
- `&self` — 完全不消耗，调用后一切照旧
- `&mut self` — 不消耗，但需要变量声明为 `mut`
- `self` — 完全消耗，调用后原变量不可再用

### 9.3 缺少 `#[derive(Debug)]`

```rust
struct Rectangle { width: f64, height: f64 }

let r = Rectangle { width: 10.0, height: 20.0 };
println!("{r:?}");  // 编译错误：Rectangle 没有实现 Debug
```

**解决方案**：添加 `#[derive(Debug)]` 或手动实现 `Debug` trait。

```rust
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }
```

### 9.4 `&mut self` 与不可变变量

```rust
let rect = Rectangle::new(10.0, 20.0);
rect.scale(2.0);  // 编译错误！rect 不是 mut
```

**解决方案**：将变量声明为 `let mut`。

```rust
let mut rect = Rectangle::new(10.0, 20.0);
rect.scale(2.0);  // 正确
```

这条错误背后的原理是 Rust 的**可变性规则**：变量默认不可变。调用 `&mut self` 方法需要可变绑定。这与 Python 形成对比——Python 中任何变量都可以在任何时候被修改。

### 9.5 结构体更新语法的所有权问题

```rust
let rect1 = Rectangle { width: 10.0, height: 20.0 };
let rect2 = Rectangle { width: 5.0, ..rect1 };
// rect1 在此仍可用，因为 f64 实现了 Copy

// 但如果字段包含 String：
struct NamedRect {
    name: String,   // String 不是 Copy
    width: f64,
    height: f64,
}
let nr1 = NamedRect { name: String::from("A"), width: 10.0, height: 20.0 };
let nr2 = NamedRect { name: String::from("B"), ..nr1 };
// nr1.name 被移动到 nr2，nr1 不可再用！
```

---

## 本章小结

本章介绍了 Rust 中自定义数据类型的基础设施：

1. **结构体（Struct）** 是数据的容器。三种形态覆盖了不同场景：
   - 命名字段结构体：最常见，字段有名字
   - 元组结构体：字段无名字，按位置访问
   - 单元结构体：无字段，作类型标记

2. **impl 块** 是行为（behavior）的容器。同一类型的多个 `impl` 块可分散在不同位置。

3. **方法与关联函数** 的区别在于第一个参数是否为 `self`：
   - 关联函数用 `::` 调用（`Rectangle::new()`）
   - 方法用 `.` 调用（`rect.area()`）

4. **三种 self 接收者** 精确表达了方法对实例的权限：
   - `&self` — 只读
   - `&mut self` — 可读写
   - `self` — 所有权转移

5. **派生宏** 让编译器自动生成常见 trait 的实现，减少样板代码。

**关键心智模型**：Rust 的 struct + impl 不是"类"，而是"数据 + 行为"的清晰分离。这让你能更精确地控制可见性、所有权和可变性。

---

## 下一章衔接

在下一章（第九章：枚举与模式匹配），我们将看到 `enum` 如何与结构体结合使用。你将学习：

- 如何在 `enum` 的变体中嵌入结构体数据
- 如何用 `match` 模式匹配解构嵌套的结构体
- `Option<T>` 和 `Result<T, E>` 这两个标准库枚举如何利用结构体来携带数据

**预习提示**：想一想，如果你需要表示"一个形状，要么是圆形要么是矩形"，你会如何组织代码？在 Rust 中，你会用一个 `enum Shape`，它的两个变体分别包含 `Circle` 和 `Rectangle` 结构体。这就是结构体与枚举配合的典型模式。

---

> **学习建议**：在进入下一章前，请完成 `EXERCISES.md` 中的所有练习题。亲手写出编译成功的代码，比阅读十遍教程更有价值。
