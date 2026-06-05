# 迁移思维练习答案 — 泛型、Trait 与 Trait Bound

## 迁移思维练习答案

### 1. C++ 模板和 Rust 泛型的主要设计差异在哪里？

C++ 模板在实例化时才做类型检查（延迟检查），导致错误信息可能极其冗长——如果 std::vector<MyType> 中的 MyType 缺少 operator<，错误可能追溯到几十层模板展开。Rust 泛型通过 Trait Bound 在声明时就约束了类型参数（如 `T: PartialOrd`），编译器在调用处就能给出清晰错误："T 没有实现 PartialOrd"。C++20 的 Concepts 正是向 Rust Trait Bound 的方向靠近。此外，Rust 不支持模板特化（避免由此产生的复杂度），也不支持非类型模板参数（但 const generics 在逐步补齐能力）。

### 2. Python 鸭子类型需要多少运行时检查，Rust 的 Trait Bound 如何提前到编译期？

Python 的 duck typing 完全依赖运行时：调用 `obj.do_something()` 时，如果 obj 没有这个方法，程序在运行时报 AttributeError 并可能崩溃。Rust 的 Trait Bound 在编译期验证类型是否实现了所需的方法签名——如果 T 没有实现某个 trait，代码根本编译不通过。程序不会因为"类型没有这个方法"而运行时崩溃。从 Python 迁移到 Rust 时，这种思维转变意味着：以前靠"跑起来看会不会炸"验证的事情，现在由类型系统在编辑时就告诉你。

### 3. Trait 和继承（Python/C++ 的类继承）在代码复用上的思路有什么不同？

继承通过"是一个"关系实现代码复用——子类继承父类的字段和方法，可以重写部分行为。Trait 则通过"能做什么"来定义能力——一个类型可以实现多个 trait，每个 trait 定义一组独立的行为。Trait 没有字段继承，只有方法签名的约定和默认实现。这种设计让组合优于继承变得更容易：一个类型通过实现 Display + Debug + Serialize 获得打印、调试、序列化能力，而不是深陷在继承层级中。
