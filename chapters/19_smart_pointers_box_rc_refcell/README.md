# 第 19 章: 智能指针 — Box\<T\>、Rc\<T\> 与 RefCell\<T\>

---

## 目录

1. [什么是智能指针](#什么是智能指针)
2. [Box\<T\> — 堆分配](#boxt--堆分配)
3. [Deref 与 Drop trait](#deref-与-drop-trait)
4. [Rc\<T\> — 引用计数](#rct--引用计数)
5. [RefCell\<T\> — 内部可变性](#refcellt--内部可变性)
6. [Rc\<RefCell\<T\>\> — 共享可变数据](#rcrefcellt--共享可变数据)
7. [引用循环与 Weak\<T\>](#引用循环与-weakt)
8. [类型选择速查表](#类型选择速查表)
9. [重要概念澄清](#重要概念澄清)
10. [Python 对照](#python-对照)

---

## 什么是智能指针

### 基本概念

在 Rust 中，"智能指针"（Smart Pointer）是指那些行为类似指针，但同时拥有额外元数据和能力的类型。普通的引用（`&T` 和 `&mut T`）只是"借用"数据，而智能指针通常"拥有"它们指向的数据。

Rust 标准库中的核心智能指针包括：

| 类型 | 用途 | 章节 |
|------|------|------|
| `Box<T>` | 堆分配，递归类型，trait 对象 | 本章 |
| `Rc<T>` | 单线程引用计数，共享所有权 | 本章 |
| `Arc<T>` | 多线程引用计数，共享所有权 | 第 20 章 |
| `RefCell<T>` | 内部可变性（运行时借用检查） | 本章 |
| `Cell<T>` | 内部可变性（Copy 类型） | 补充 |
| `Weak<T>` | 非拥有引用，打破引用循环 | 本章 |
| `Mutex<T>` / `RwLock<T>` | 多线程互斥访问 | 第 20 章 |

### 智能指针 vs 普通引用

普通引用（`&T`）只是借用数据，不涉及所有权：

```rust
let x = 42;
let r = &x;       // r 借用 x，x 仍然拥有数据
println!("{}", r); // 解引用
// x 仍然有效，r 超出作用域不影响 x
```

智能指针通常拥有数据：

```rust
let b = Box::new(42); // b 拥有堆上的 42
// 当 b 超出作用域时，堆上的 42 被释放
```

### 为什么叫"智能"指针？

因为它们不仅仅是存储一个地址，还包含额外的逻辑：

- **Box\<T\>**：知道如何在堆上分配/释放内存
- **Rc\<T\>**：维护引用计数，知道何时可以安全释放
- **RefCell\<T\>**：在运行时执行借用检查，不是编译时
- **String** 和 **Vec\<T\>**：在某种意义上也是智能指针（它们拥有堆分配的数据并知道如何管理内存）

---

## Box\<T\> — 堆分配

### 基础用法

`Box<T>` 将数据分配在堆（heap）上，栈上只保留一个指向堆的指针。这是最直接的智能指针。

```rust
let b = Box::new(5);
println!("b = {}", b);  // 自动解引用，输出: b = 5
```

### 为什么需要 Box？

#### 1. 存储大型数据

Rust 的栈空间通常是有限的（默认约 8MB）。如果一个类型非常大（比如一个 [u8; 1_000_000] 数组），在栈上分配可能导致栈溢出（stack overflow）。使用 `Box` 可以将数据放在堆上：

```rust
// 错误的做法——可能导致栈溢出
// let large_array = [0u8; 10_000_000]; // 10MB，栈可能放不下

// 正确的做法——放在堆上
let large_array = Box::new([0u8; 10_000_000]);
// 栈上只有一个 8 字节的指针，数据在堆上
```

对于递归数据结构也是如此。如果不使用 Box，编译器无法计算类型的大小：

```rust
// 这段代码无法编译: ConsList 的大小取决于自身
// enum ConsList {
//     Nil,
//     Cons(i32, ConsList),  // 错误: 递归类型有无限大小
// }

// 使用 Box 解决问题
enum ConsList {
    Nil,
    Cons(i32, Box<ConsList>),  // Box 是指针，大小已知
}
```

#### 2. 递归类型

如上所述，递归类型在编译期无法确定大小——一个 `ConsList` 包含另一个 `ConsList`，而那个又包含另一个……这是一个无限的类型大小。`Box` 解决了这个问题，因为 `Box<ConsList>` 的大小是固定的（就是一个指针的大小）。

我们的示例程序定义了一个 ConsList 枚举：

```rust
#[derive(Debug)]
enum ConsList {
    Nil,
    Cons(i32, Box<ConsList>),
}
```

可以像这样构建和使用：

```rust
let list = ConsList::Cons(
    1,
    Box::new(ConsList::Cons(2, Box::new(ConsList::Nil))),
);
```

#### 3. Trait 对象

当需要存储实现了某个 trait 的不同类型时，使用 `Box<dyn Trait>` 来抹除具体类型：

```rust
trait Animal {
    fn speak(&self) -> &'static str;
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> &'static str { "汪汪!" }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) -> &'static str { "喵~" }
}

let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in &animals {
    println!("{}", animal.speak());
}
```

这与面向对象语言中的"多态"类似，但需要显式标注 `dyn` 关键字。

### Box 的大小

在 64 位平台上：

- `Box<T>` 本身的大小 = 8 字节（一个指向堆的指针）
- `T` 的大小取决于类型本身
- 无论 `T` 有多大，`Box<T>` 始终是 8 字节

```rust
use std::mem::size_of;

// 假设 LargeData 是 8192 字节
// let ld = LargeData { ... };  // 栈上分配 8192 字节
// let boxed: Box<LargeData> = Box::new(LargeData { ... });
// size_of_val(&boxed) == 8  // 只是一个指针
```

---

## Deref 与 Drop trait

### Deref trait

`Deref` trait 让智能指针可以像普通引用一样被解引用（使用 `*` 运算符）。当编译器看到 `*x` 时，它实际上调用 `<T as Deref>::deref(&x)`。

`Box<T>` 实现了 `Deref<Target = T>`，所以 `*boxed_value` 返回对 `T` 的引用。

**Deref 强制转换（Deref Coercion）**：Rust 会自动在需要时调用 `deref`。例如：

```rust
let b = Box::new(String::from("hello"));
// 不需要写 (*b).len() —— Deref 强制转换自动处理
println!("{}", b.len()); // 自动转换为 (&(*b)).len()

// 等价于:
// b → &(*b) → &String → &str (通过 String 的 Deref 实现)
```

这个特性让智能指针用起来像普通引用一样自然。

### Drop trait

`Drop` trait 定义了一个类型的"清理"逻辑。对于 `Box<T>`：

- 当 `Box<T>` 超出作用域时，其 `Drop` 实现会被调用
- `Drop` 释放 `Box` 在堆上分配的内存
- 同时也会调用 `T` 的 `Drop` 实现（如果 `T` 实现了 `Drop`）

```rust
{
    let b = Box::new(String::from("这个字符串在堆上"));
    // b 在这里有效
} // b 超出作用域 → Box::drop 被调用 → 堆内存被释放 → String::drop 被调用
```

对于 `Rc<T>`，`Drop` 的实现更加精细：
- 每次 `Rc` 被丢弃时，引用计数减 1
- 当引用计数归零时，实际的数据才被释放

这就是引用计数智能指针的核心机制。

---

## Rc\<T\> — 引用计数

### 基本概念

`Rc<T>`（Reference Counting）实现了**共享所有权**——多个"所有者"可以同时拥有同一块数据。这在单线程环境中非常有用。

```rust
use std::rc::Rc;

let data = Rc::new(String::from("共享数据"));
let clone1 = Rc::clone(&data);  // 不拷贝数据，只增加计数
let clone2 = Rc::clone(&data);  // 引用计数 = 3

println!("引用计数: {}", Rc::strong_count(&data)); // 3
```

### Rc::clone 不是深拷贝

这是初学者最容易混淆的地方。`Rc::clone` **不会**复制底层数据——它只增加引用计数：

```rust
let huge_string = Rc::new(String::from("假设这是一个非常巨大的字符串……"));
let also_huge = Rc::clone(&huge_string);  // 极快！只增加一个整数
// huge_string 和 also_huge 指向内存中的同一个 String
```

对比：
- `Rc::clone(&rc)` → 增加引用计数，共享同一块数据
- `(*rc).clone()` → 深拷贝底层数据，创建独立副本

### 引用计数的生命周期

```rust
let data = Rc::new(42);          // ref_count = 1
{
    let clone1 = Rc::clone(&data); // ref_count = 2
    {
        let clone2 = Rc::clone(&data); // ref_count = 3
    } // clone2 超出作用域 → ref_count = 2
} // clone1 超出作用域 → ref_count = 1
// 当 data 超出作用域时 → ref_count = 0 → 数据被释放
```

### Rc 的重要限制

**Rc 不提供可变性**。一旦创建了 `Rc<T>`，你只能通过不可变引用（`&T`）访问数据：

```rust
let config = Rc::new(String::from("配置"));
// config.push_str("更新");     // 错误: Rc 没有实现 DerefMut
// let m = Rc::get_mut(&mut config); // 只有在 ref_count == 1 时可用
```

此外，`Rc<T>` 没有实现 `Send` 和 `Sync` trait，这意味着它**不能**在多个线程之间共享。它的引用计数操作不是原子性的。对于多线程场景，使用 `Arc<T>`（Atomic Reference Counting）。

### 实际应用场景

`Rc` 非常适合以下场景：
- 多个组件需要共享配置/状态
- 图数据结构（一个节点被多个其他节点引用）
- 编译器或解释器中的共享符号表
- 任何需要"图"或"DAG"而非"树"的数据结构

---

## RefCell\<T\> — 内部可变性

### 什么是内部可变性

"内部可变性"（Interior Mutability）是 Rust 中的一个设计模式，允许你在只拥有不可变引用（`&self`）的情况下修改内部数据。`RefCell<T>` 是单线程内部可变性的主要工具。

```rust
use std::cell::RefCell;

let data = RefCell::new(42);
// data 本身不是 mut！
*data.borrow_mut() += 1;  // 修改内部值
println!("{}", data.borrow()); // 43
```

### borrow() 和 borrow_mut()

`RefCell` 提供两个方法来访问内部数据：

| 方法 | 返回 | 语义 |
|------|------|------|
| `borrow()` | `Ref<T>` | 不可变借用，可以有多个 |
| `borrow_mut()` | `RefMut<T>` | 可变借用，只能有一个 |

`Ref` 和 `RefMut` 实现了 `Deref`，所以可以像普通引用一样使用：

```rust
let cell = RefCell::new(vec![1, 2, 3]);

// 不可变借用
let borrowed = cell.borrow();
println!("长度: {}", borrowed.len()); // OK

// 可变借用（在不可变借用释放之后）
drop(borrowed);
let mut borrowed_mut = cell.borrow_mut();
borrowed_mut.push(4);
```

### 编译时 vs 运行时借用检查

这是理解 `RefCell` 的关键对比：

| 方面 | 标准引用（&T / &mut T） | RefCell |
|------|------------------------|---------|
| 检查时机 | **编译时** | **运行时** |
| 违反后果 | 编译错误 | **panic!** |
| 检查者 | 借用检查器（borrow checker） | RefCell 内部计数器 |
| 灵活性 | 受限于借用规则 | 更灵活但需要自行保证安全 |

```rust
// 编译时检查 —— 编译失败（安全！）
// let mut x = 42;
// let r1 = &x;
// let r2 = &mut x;  // 错误: 不能同时存在不可变借用和可变借用

// 运行时检查 —— 运行时 panic（也是安全的，但更难调试！）
let cell = RefCell::new(42);
let r1 = cell.borrow_mut();
// let r2 = cell.borrow_mut();  // 运行时 panic: 已经可变借用
```

### 为什么需要 RefCell

有时，一个类型的方法签名声明为 `&self`（不可变借用），但方法的内部逻辑确实需要修改某些数据。经典的例子：

1. **mock 对象**：测试时需要记录方法调用次数
2. **缓存/记忆化**：需要存储计算结果
3. **观察者模式**：需要修改回调列表

```rust
// 例如，一个记录调用次数的方法
// fn process(&self) { ... }  // 签名是 &self
// 但我们需要更新内部计数器

use std::cell::RefCell;

struct Counter {
    count: RefCell<u32>,
}

impl Counter {
    fn increment(&self) {  // 注意: &self, 不是 &mut self
        *self.count.borrow_mut() += 1;
    }
}
```

### Cell<T> — RefCell 的轻量替代

对于 `Copy` 类型（如整数），`Cell<T>` 是更好的选择：

```rust
use std::cell::Cell;

let c = Cell::new(42);
c.set(100);  // 完全替换值
println!("{}", c.get());  // 100
```

`Cell<T>` 不会有借用冲突，因为它使用 `get`/`set` 而不是返回引用。

---

## Rc\<RefCell\<T\>\> — 共享可变数据

### 为什么需要这个组合

单独来看：
- `Rc<T>` 允许多个所有者，但**不允许**修改数据
- `RefCell<T>` 允许修改数据，但只有**一个所有者**

组合在一起：
- `Rc<RefCell<T>>` 允许多个所有者**且**可以修改数据

这是单线程环境中实现"共享可变状态"的惯用模式。

### 图节点示例

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: String,
    edges: RefCell<Vec<Rc<Node>>>,  // 可变 + 共享
}

impl Node {
    fn new(value: &str) -> Rc<Node> {
        Rc::new(Node {
            value: String::from(value),
            edges: RefCell::new(Vec::new()),
        })
    }

    fn add_edge(&self, target: Rc<Node>) {
        // 通过 &self 修改 edges！这就是 RefCell 的能力
        self.edges.borrow_mut().push(target);
    }
}

let alice = Node::new("Alice");
let bob = Node::new("Bob");
alice.add_edge(bob);  // alice 不需要是 mut
```

### 模式总结

`Rc<RefCell<T>>` 的实际使用步骤：

1. **创建**：`let x = Rc::new(RefCell::new(value));`
2. **克隆所有权**：`let y = Rc::clone(&x);`（增加引用计数）
3. **访问内部值**：`x.borrow()` 或 `x.borrow_mut()`（RefCell）
4. **自动清理**：最后一次 `Rc` 被丢弃时，数据和 RefCell 一起被释放

---

## 引用循环与 Weak\<T\>

### 引用循环：Rc + RefCell 的危险

`Rc<T>` 和多所有权可能创建**引用循环**，导致内存泄漏。考虑以下场景：

```
Node A (Rc, strong_count=2)  →  Node B (Rc, strong_count=2)
     ↑                                  |
     └──────────────────────────────────┘
```

- A 持有 B 的 Rc，B 持有 A 的 Rc
- A 的引用计数 ≥ 1（来自 B），B 的引用计数 ≥ 1（来自 A）
- 当外部引用消失后，两者仍互相引用，永远无法释放
- 这就是**内存泄漏**

### 具体例子：父子关系

```rust
struct Parent {
    name: String,
    children: RefCell<Vec<Rc<Child>>>,  // 父持有子的 Rc
}

struct Child {
    name: String,
    parent: RefCell<Rc<Parent>>,  // ⚠️ 子也持有父的 Rc
    // 如果父和子互持 Rc → 引用循环 → 内存泄漏！
}
```

### Weak\<T\> 解决方案

`Weak<T>` 是 `Rc<T>` 的"弱引用"版本：

- **不增加** `strong_count`（增加 `weak_count`）
- 不保证数据仍然有效——`upgrade()` 返回 `Option<Rc<T>>`
- 当所有强引用被丢弃后，数据被释放，`upgrade()` 返回 `None`

修复亲子关系：

```rust
struct Parent {
    name: String,
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    name: String,
    parent: RefCell<Weak<Parent>>,  // ✅ 使用 Weak 而非 Rc！
}
```

### Weak 的创建和使用

```rust
use std::rc::{Rc, Weak};

let rc = Rc::new(42);

// 从 Rc 创建 Weak — 不增加 strong_count
let weak: Weak<i32> = Rc::downgrade(&rc);
// Rc::weak_count(&rc) 现在为 1

// 使用 Weak — 需要"升级"为 Rc
match weak.upgrade() {
    Some(shared) => println!("值: {}", *shared),
    None => println!("原始数据已被释放"),
}

// 释放原始数据
drop(rc);
// 此时 weak.upgrade() 返回 None
```

### 引用计数的种类

| 计数 | 方法 | 含义 |
|------|------|------|
| `strong_count` | `Rc::strong_count(&rc)` | 所有权计数，归零时释放数据 |
| `weak_count` | `Rc::weak_count(&rc)` | 弱引用计数，不阻止释放 |

### 何时使用 Weak

- **父子关系**：父母拥有子女（Rc），子女引用父母（Weak）
- **缓存/观察者**：数据源持有数据（Rc），观察者持有弱引用（Weak）
- **防止循环**：任何可能存在引用环的数据结构

---

## 类型选择速查表

### 单线程场景

| 需求 | 推荐类型 | 理由 |
|------|---------|------|
| 堆分配单一大值 | `Box<T>` | 最简单，单所有者 |
| 递归类型（如链表） | `Box<T>` | 编译器需要确定大小 |
| trait 对象（多态） | `Box<dyn Trait>` | 编译期类型擦除 |
| 多所有者，只读 | `Rc<T>` | 共享所有权，零开销 |
| 单所有者，内部修改 | `RefCell<T>` | 通过 &self 修改 |
| 多所有者，可修改 | `Rc<RefCell<T>>` | 共享 + 可变，运行时检查 |
| 打破引用循环 | `Weak<T>` | 不增加 strong_count |

### 多线程场景（第 20 章预览）

| 需求 | 推荐类型 | 理由 |
|------|---------|------|
| 多线程只读共享 | `Arc<T>` | 原子引用计数 |
| 多线程互斥修改 | `Arc<Mutex<T>>` | 互斥锁 + 共享所有权 |
| 多线程读写分离 | `Arc<RwLock<T>>` | 读写锁 + 共享所有权 |

### 选择决策树

```
需要堆分配？
├── 是 → 需要多个所有者？
│   ├── 是 → 需要修改数据？
│   │   ├── 是 → 多线程？
│   │   │   ├── 是 → Arc<Mutex<T>> 或 Arc<RwLock<T>>
│   │   │   └── 否 → Rc<RefCell<T>>
│   │   └── 否 → 多线程？
│   │       ├── 是 → Arc<T>
│   │       └── 否 → Rc<T>
│   └── 否 → 需要多态？
│       ├── 是 → Box<dyn Trait>
│       └── 否 → Box<T>
└── 否 → 使用普通的 &T / &mut T
```

---

## 重要概念澄清

### RefCell 不是"绕过 Rust 的规则"

这是一个常见的误解。**RefCell 没有绕过 Rust 的借用规则**——它只是将规则验证从编译时移到了运行时。

- **标准引用**（`&T`，`&mut T`）：借用检查器在**编译时**验证规则。如果违反规则，程序**无法编译**。
- **RefCell**：借用规则在**运行时**由 RefCell 的内部计数器验证。如果违反规则，程序会 **panic**。

这两种方式都**强制执行**相同的借用规则：
- 可以同时存在多个不可变借用
- **或**恰好一个可变借用
- 不能同时存在可变借用和不可变借用

区别在于**何时**发现违规——编译时（安全网，更好的错误信息）vs 运行时（需要测试覆盖来发现问题）。

### 何时适合使用 RefCell

`RefCell` 适合以下情况：
1. 你确定代码在运行时不会违反借用规则，但编译器无法静态验证
2. 需要实现内部可变性的设计模式（如观察者、mock 测试）
3. 数据结构需要"自引用"或"图结构"

不适合的情况：
1. 违反借用规则的可能性很高
2. 性能敏感的热路径（运行时检查有开销）
3. 简单的情况——优先使用 `&mut T`

### 性能考虑

| 类型 | 开销 | 说明 |
|------|------|------|
| `Box<T>` | 仅堆分配和释放 | 几乎零额外运行时开销 |
| `Rc<T>` | 原子操作（强计数） | clone 时 incr，drop 时 decr |
| `RefCell<T>` | 运行时借用计数 | borrow/borrow_mut 检查计数器 |
| `Mutex<T>` | 锁获取/释放 | 操作系统级同步 |

---

## Python 对照

对于来自 Python 背景的读者，理解 Rust 的智能指针可以在对照中获得更好的直觉：

### Python：一切皆引用

在 Python 中，所有对象都分配在堆上，变量只是引用（指针）：

```python
# Python
x = [1, 2, 3]
y = x               # y 和 x 指向同一个列表
y.append(4)         # x 也被修改: [1, 2, 3, 4]
```

### Rust：显式所有权和指针

Rust 中，默认是**值语义**和**所有权**：

```rust
let x = vec![1, 2, 3];
let y = x;          // x 的所有权移动到 y，x 不再可用
// y.push(4);       // OK
// println!("{:?}", x); // 错误: x 已被移动

// 要共享数据，必须显式使用智能指针
let x = Rc::new(vec![1, 2, 3]);
let y = Rc::clone(&x);  // 显式共享，类似 Python 的 y = x
```

### 对照表

| 概念 | Python | Rust |
|------|--------|------|
| 所有对象都是引用 | 默认 | 需要显式（`Rc`，`Arc`，`&T`） |
| 共享可变数据 | 默认（但危险） | `Rc<RefCell<T>>`（显式且检查） |
| 内存安全 | GC / 引用计数 | 编译时借用检查 + 运行时检查 |
| 数据释放 | GC 回收 | 确定性（作用域结束 / 计数归零） |
| 空引用 | `None`，`is None` | `Option<T>`，编译时检查 |
| 可变性 | 一切默认可变 | 默认不可变，需要 `mut` |

### Python 的引用计数 vs Rust 的 Rc

Python 使用引用计数（结合 GC 处理循环）：

```python
import sys
a = [1, 2, 3]
print(sys.getrefcount(a))  # 引用计数

b = a
print(sys.getrefcount(a))  # 增加了！
```

Rust 的 `Rc::strong_count` 提供类似的可见性：

```rust
let a = Rc::new(vec![1, 2, 3]);
println!("{}", Rc::strong_count(&a)); // 1
let b = Rc::clone(&a);
println!("{}", Rc::strong_count(&a)); // 2
```

关键区别：Python 自动使用引用计数处理所有对象，并使用 GC 处理循环；Rust 让你**选择**何时使用引用计数（`Rc`），并让你**显式管理**循环（通过 `Weak`）。

### 为什么 Rust 不默认使用引用计数？

1. **性能**：引用计数每次 clone/drop 都有开销
2. **可预测性**：确定性释放 vs GC 的不确定性
3. **明确性**：所有权模型让数据流清晰可见
4. **零成本抽象**：只需为实际使用的东西付出代价

---

## 核心术语对照

| 中文 | 英文 | 说明 |
|------|------|------|
| 智能指针 | Smart Pointer | 有额外元数据/能力的指针类型 |
| 堆分配 | Heap Allocation | 在堆上分配内存 |
| 递归类型 | Recursive Type | 类型定义中引用自身 |
| trait 对象 | Trait Object | 动态分发，运行时多态 |
| 解引用 | Dereference | 使用 `*` 获取指针指向的值 |
| 引用计数 | Reference Counting | 跟踪引用数量的技术 |
| 内部可变性 | Interior Mutability | 通过 &self 修改内部状态 |
| 强引用计数 | Strong Count | 所有权计数 |
| 弱引用 | Weak Reference | 不拥有数据的引用 |
| 引用循环 | Reference Cycle | 互持引用导致无法释放 |
| 内存泄漏 | Memory Leak | 无法释放不再需要的内存 |
| 借用检查器 | Borrow Checker | 编译时检查引用规则的组件 |
| 运行时借用检查 | Runtime Borrow Checking | RefCell 在运行时执行借用规则 |
| 原子引用计数 | Atomic Reference Counting | Arc 的线程安全引用计数 |

---

## 本章代码结构

```
src/main.rs
├── demo_box()           — Box 基础、递归类型、trait 对象
├── demo_rc()            — Rc 共享所有权、引用计数
├── demo_refcell()       — 内部可变性、运行时借用检查
├── demo_rc_refcell()    — Rc + RefCell 组合，图节点
├── demo_reference_cycles() — 引用循环、Weak 方案
├── demo_type_selection()   — 类型选择总结表
└── tests                 — 7 个单元测试
```

运行方式：

```bash
cargo run    # 运行所有演示
cargo test   # 运行所有测试
```

---

*本章介绍了 Rust 中核心的智能指针类型。掌握 Box、Rc、RefCell 及其组合用法，是理解 Rust 内存管理模型的关键一步。在第 20 章中，我们将看到这些概念的线程安全版本：Arc、Mutex 和 RwLock。*
