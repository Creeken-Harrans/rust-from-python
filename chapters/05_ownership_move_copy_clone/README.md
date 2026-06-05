# 第五章: 所有权、移动与复制 (Ownership, Move, Copy, Clone)

> **核心术语中英对照**
>
> | 英文 | 中文 | 说明 |
> |---|---|---|
> | Ownership | 所有权 | 值的归属权, 同一时刻只有一个所有者 |
> | Owner | 所有者 | 拥有某个值的变量 |
> | Move | 移动 | 所有权从一个变量转移到另一个变量 |
> | Copy | 复制语义 | 编译期位复制, 赋值后原变量仍可用 |
> | Clone | 克隆 | 显式深拷贝, 在堆上分配新内存 |
> | Deep Copy | 深拷贝 | 完整复制数据内容, 包括堆内存 |
> | Drop | 析构 | 值离开作用域时自动执行的清理逻辑 |
> | Borrow | 借用 | 在不获取所有权的情况下访问数据 (下一章详细讲解) |

---

## 1. 要解决什么问题 (Problem)

在编程中有一个非常经典的两难问题: **谁负责释放内存?**

```python
# Python 中, 两个变量指向同一个列表
a = [1, 2, 3]
b = a          # a 和 b 都指向同一个列表对象
b.append(4)
print(a)       # [1, 2, 3, 4] —— a 也被修改了！
```

这种"两个变量共享同一份数据"的设计带来了几个严重问题:

1. **双重释放 (Double Free)**: 如果两个变量都认为自己"拥有"这份内存并尝试释放, 程序崩溃。
2. **悬垂指针 (Dangling Pointer)**: 一个变量释放了内存, 另一个变量还在使用。
3. **数据竞争 (Data Race)**: 多线程同时修改同一份数据, 结果不可预测。
4. **意外的副作用**: 像上面的 Python 例子, `b` 的修改意外影响了 `a`。

Rust 通过**所有权系统 (Ownership System)** 在编译期彻底解决了这些问题, 不需要垃圾回收 (Garbage Collection), 也不需要手动 `malloc/free`。

---

## 2. Python 程序员通常会怎么理解 (Python Perspective)

### Python 的引用计数机制

```python
# Python 中, 赋值是"引用复制"
s1 = "hello"
s2 = s1
# s1 和 s2 指向同一个字符串对象
# Python 通过引用计数决定何时回收内存

# 对于不可变对象 (immutable), 这很安全
# 因为不会有"修改一个而影响另一个"的问题

# 但对于可变对象 (mutable), 则容易出问题
a = [1, 2, 3]
b = a
b.append(4)  # 同时影响了 a
print(a)     # [1, 2, 3, 4] —— 意外！
```

Python 程序员习惯了"一切皆是引用"的心智模型。这意味着:

- 赋值的成本很低 (只复制指针, 不复制数据)
- 你需要**小心**可变对象的共享访问
- 垃圾回收 (`gc`) 在后台自动处理内存释放

### Rust 的不同之处

Rust 的赋值行为**取决于类型**:

| 类型 | 赋值行为 | 原变量是否有效 | 类比 |
|------|---------|--------------|------|
| `i32`, `bool`, `char` 等 Copy 类型 | 复制值 | 有效 | Python 不可变对象 |
| `String`, `Vec<T>` 等非 Copy 类型 | **移动 (Move)** 所有权 | **无效!** | 无直接类比, 这是 Rust 特有的 |

```rust
// Rust 中的 Move —— Python 程序员最困惑的地方
let s1 = String::from("hello");
let s2 = s1;      // s1 的所有权被移动到 s2
// println!("{}", s1);  // 编译错误! s1 已经失效
```

---

## 3. Rust 为什么采用不同设计 (Why Rust Differs)

### Python 的引用计数有多贵

Python 的每个对象都需要:
- 一个引用计数字段 (8 字节)
- 原子操作增减引用计数 (多线程开销)
- 循环引用检测 (gc 模块)
- 运行时类型信息 (type object pointer)

这些开销在性能敏感场景中累积显著。

### Rust 的编译期解决方案

Rust 将所有权的追踪从**运行时**移到了**编译期**:

1. **零运行时开销 (Zero-Cost Abstraction)**: 编译完成后, 所有权信息被完全消除, 生成的机器码与手动管理内存的 C 代码一样高效。
2. **编译期保证**: 在程序运行之前, 编译器就能证明不存在 double free、use-after-free、data race。
3. **无垃圾回收暂停**: 没有 GC, 程序运行不受垃圾回收的间歇性暂停影响。

### 核心权衡

```
Python:  开发速度快, 运行时安全 (GC), 性能不如 C/C++
Rust:   编译期安全 (严格的编译器), 性能媲美 C/C++, 但学习曲线陡峭
C/C++:  性能最高, 但安全完全依赖开发者, UB 遍地
```

---

## 4. 核心规则 (Rules)

### 所有权三大法则

```
法则 1: 每个值 (value) 有且仅有一个所有者 (owner)
法则 2: 所有者离开作用域时, 值被自动释放 (drop)
法则 3: 任何时刻只能有一个所有者
```

### 作用域与 Drop

```rust
{
    let s = String::from("hello");  // s 进入作用域, 拥有字符串的所有权
    // ... 使用 s ...
}   // s 离开作用域, String 的 drop() 被自动调用, 内存被安全释放
```

### Move 语义

当**非 Copy 类型**发生赋值或作为函数参数传递时, 所有权**移动**:

```rust
let s1 = String::from("hello");
let s2 = s1;               // Move: s1 → s2
// drop(s1) 不会发生, 因为 s1 不再拥有该值
```

这个设计保证了:
- 不会发生 double free: 只有一个变量会 drop 该值
- 不会 use-after-free: 编译器禁止使用已移动的变量

### Copy 语义

**Copy trait** 标记的类型赋值时自动复制:

```rust
let x = 5;
let y = x;   // Copy: x 被复制, x 和 y 都有效
```

Copy 类型的条件 (编译器自动推导的规则):
- 类型的所有字段都实现了 Copy
- 类型没有实现 Drop trait
- 类型在内存中是"纯数据" (栈上数据)

Copy 类型包括:
- 所有整数: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点数: `f32`, `f64`
- 布尔: `bool`
- 字符: `char`
- 全 Copy 的元组: `(i32, bool)`, `(char, f64)` 等
- 全 Copy 的数组: `[i32; 5]`, `[bool; 3]` 等
- 引用: `&T` (但只是引用本身是 Copy, 不是所指向的数据)

注意: **不是 Copy 的类型**包括 `String`, `Vec<T>`, `HashMap<K,V>`, `Box<T>`, 以及任何实现了 `Drop` 的类型。

### Clone 语义

`Clone` trait 提供显式的深拷贝:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 显式深拷贝, 分配新的堆内存
// s1 和 s2 都有效, 各自拥有独立的数据副本
```

Clone 与 Copy 的关键区别:
- `Copy`: 隐式, 编译期位复制 (memcpy), 零开销, 原变量仍可用
- `Clone`: 显式 (`.clone()`), 可能涉及堆分配, 有运行时开销, 原变量仍可用
- Move: 隐式, 所有权转移, 原变量失效

---

## 5. 可运行示例 (Working Examples)

### 示例 1: Move (移动)

```rust
fn demonstrate_move() {
    let s1 = String::from("hello");
    let s2 = s1;    // s1 的所有权移动到 s2

    // 如果取消下面注释, 编译器报错:
    // error[E0382]: borrow of moved value: `s1`
    // println!("s1 = {}", s1);

    println!("s2 = {}", s2);  // 正常工作
}
```

### 示例 2: Copy (复制)

```rust
fn demonstrate_copy() {
    let x = 5;
    let y = x;       // i32 是 Copy, x 仍然有效

    println!("x = {}, y = {}", x, y);  // 两者都可用

    let a = true;
    let b = a;       // bool 也是 Copy
    println!("a = {}, b = {}", a, b);
}
```

### 示例 3: Clone (深拷贝)

```rust
fn demonstrate_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 堆上分配新内存, 复制内容

    println!("s1 = {}, s2 = {}", s1, s2);  // 两者独立

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();

    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
```

### 示例 4: 函数参数与返回值的所有权

```rust
fn take_ownership(s: String) -> String {
    println!("收到: {}", s);
    let result = s.to_uppercase();
    result  // 所有权转移给调用者
}

fn borrow_then_return() {
    let original = String::from("rust");
    let processed = take_ownership(original);
    // original 已经失效
    println!("取回: {}", processed);
}
```

### 示例 5: 结构体的移动语义

```rust
#[derive(Debug)]
struct Person {
    name: String,  // 非 Copy
    age: i32,      // Copy
}

fn demonstrate_struct_move() {
    let alice = Person { name: String::from("Alice"), age: 30 };
    let bob = alice;  // 整个结构体被移动!
    // println!("{:?}", alice);  // 编译错误: alice 已移动
    println!("{:?}", bob);      // 正常
}
```

注意: 即使 `age` 字段 (`i32`) 是 Copy 类型, 但因为 `name` 字段 (`String`) 不是 Copy, 所以整个 `Person` 结构体也不是 Copy。移动 `Person` 会将包括 `age` 在内的所有字段一起移动。

### 示例 6: Drop 演示

```rust
struct Resource { id: u32 }

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource #{}  被释放", self.id);
    }
}

{
    let _r1 = Resource { id: 1 };
    let _r2 = Resource { id: 2 };
    // 离开作用域时, 先 drop _r2, 再 drop _r1 (逆序)
}
```

输出:
```
Resource #2  被释放
Resource #1  被释放
```

---

## 6. 常见错误示例 (Error Examples)

### 错误 1: 使用已移动的值

```rust
// 编译错误:
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // error[E0382]: borrow of moved value: `s1`
```

### 错误 2: 部分移动 (Partial Move)

```rust
// 编译错误:
struct Person { name: String, age: i32 }
let p = Person { name: String::from("Alice"), age: 30 };
let name = p.name;     // name 字段被移动出去
// println!("{:?}", p);  // error[E0382]: p 不完整了
```

### 错误 3: 在循环中误用 move

```rust
// 编译错误:
let names = vec![String::from("Alice"), String::from("Bob")];
for name in names {          // names 被移动到迭代器
    println!("{}", name);
}
// println!("{:?}", names);  // error[E0382]: names 已移动
```

### 错误 4: 函数参数的所有权丢失

```rust
fn process(s: String) {
    println!("{}", s);
    // s 在这里被 drop
}

let my_string = String::from("hello");
process(my_string);
// process(my_string);  // 编译错误! my_string 已经失效
```

如果你确实需要多次使用, 需要:
- 克隆: `process(my_string.clone());`
- 传递引用: `process(&my_string);` (借用, 下一章讲)
- 函数返回所有权: 修改 `process` 返回 `String`

---

## 7. 编译器为什么拒绝错误代码 (Why Compiler Rejects)

### 错误 `E0382: borrow of moved value`

当我们写:

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```

编译器看到了什么?

1. `let s1 = String::from("hello");` -- 创建 "hello" 字符串, s1 是所有者
2. `let s2 = s1;` -- s1 类型是 `String`, 非 Copy, 发生 Move。编译器将 s1 标记为"已移动" (moved)。
3. `println!("{}", s1);` -- 编译器的 borrow checker 发现 s1 已经被移动, 拒绝编译。

编译器的推理: "s1 已把所有权交给 s2, s1 不再持有有效数据。使用 s1 等同于读取已释放的内存。我不能让这种事发生。"

### Rust 编译器的预防哲学

```
C++:   编译器什么都不管, 运行时 Segment Fault
Python: 运行时自动管理, 你感觉不到问题 (但有 GC 开销)
Rust:  编译器在编译期阻止你犯错, 你必须修复才能继续
```

Rust 编译器不是你的敌人 —— 它是帮你**提前发现 bug** 的伙伴。每一个编译错误都是潜在的运行时崩溃。

---

## 8. 正确修复 (Fixes)

### Move 错误的四种修复策略

**策略 1: 使用 Clone (显式深拷贝)**

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝
println!("s1 = {}, s2 = {}", s1, s2);  // 都可用
```

适用场景: 你确实需要两份独立的数据副本。

**策略 2: 使用引用 (借用, 下一章详细讲)**

```rust
let s1 = String::from("hello");
let s2 = &s1;    // 不可变引用, 不获取所有权
println!("s1 = {}, s2 = {}", s1, s2);  // 都可用
```

适用场景: 你只需要临时读取数据, 不需要拥有它。

**策略 3: 返回所有权**

```rust
fn process(s: String) -> String {  // 接收所有权
    println!("处理: {}", s);
    s  // 返回所有权
}
let s = String::from("hello");
let s = process(s);  // s 重新获得所有权
println!("{}", s);   // 可以继续使用
```

适用场景: 你需要在函数中修改数据, 之后还要继续使用。

**策略 4: 重构设计, 让所有权更早落入最终使用者**

```rust
// 原设计: 中间函数获取了不必要的所有权
fn bad_middle(s: String) { /* ... */ }

// 改进: 让中间函数只借用
fn better_middle(s: &str) { /* ... */ }

// 最终使用者才获取所有权
fn final_step(s: String) { /* 拥有并处理 */ }
```

---

## 9. 适用边界 (Boundaries)

### 什么时候使用 Clone 是合理的

- **原型阶段 (Prototyping)**: 快速验证业务逻辑, 先用 `.clone()` 让代码编译通过, 之后再优化。
- **确实需要独立副本**: 比如两个线程各自需要互不干扰的数据。
- **数据量很小**: 克隆一个只有几个元素的 `Vec` 的开销可以忽略。
- **克隆是业务要求**: 比如你需要在日志中记录原始数据, 同时也要修改一份副本。

### 什么时候 Clone 是代码坏味道

- **"绕过"编译错误**: 每次出现 move error 就加 `.clone()` 而不思考所有权设计, 最终代码里到处是 clone, 性能糟糕。
- **本可以用引用 (Borrow)**: 如果你只需要读取数据, 使用 `&T` 而不是克隆整个数据。
- **大数据集**: 克隆 1GB 的数组因为你不想重构函数签名 —— 这是在掩盖设计问题。

### Clone 的性能成本

```rust
let s = String::from("hello");
let s2 = s.clone();
// clone() 实际上做了:
// 1. 在堆上分配 5 字节新空间
// 2. 将 "hello" 的每个字节复制到新空间
// 3. String 对象记录新的堆指针、长度、容量

// 相比之下, borrow 是零开销的:
let s3 = &s;  // 只是复制一个指针, 毫无开销
```

### Copy 和 Clone 的类型边界

```
Copy  ⊆  Clone

所有 Copy 类型都必须实现 Clone, 但反之不成立。
Copy 的 clone() 实际上就是位复制, 编译器自动实现。
```

你可以为类型实现 Clone 而不同时实现 Copy, 但不能反过来。例如, `String` 实现了 `Clone` 但不是 `Copy` —— 这是合理的, 因为 `String` 的复制需要堆分配, 不能是隐式的位复制。

---

## 10. 小结 (Summary)

### 本章核心结论

| 概念 | 一句话 |
|------|--------|
| Ownership (所有权) | 每个值只有一个主人, 主人走了值就没了 |
| Move (移动) | 非 Copy 类赋值 = 移交所有权, 原变量废了 |
| Copy (复制语义) | 整数/bool/char 等栈上数据, 赋值即复制, 原变量还能用 |
| Clone (克隆) | 想保留原变量又需要独立副本时, 显式调 `.clone()` |
| Drop (析构) | 主人离开时自动收拾干净, 不用担心内存泄漏 |

### 与 Python 的关键差异

```
Python 赋值:   复制引用, 两个变量指向同一对象
Rust 赋值:
  - Copy 类型: 复制值, 两个变量独立
  - 非 Copy 类型: 移动所有权, 原变量无效

Python 内存管理: 运行时引用计数 + GC
Rust 内存管理:   编译期所有权分析, 零运行时开销

Python 的错误:   运行时报错
Rust 的错误:     编译时报错 (move error, borrow error)
```

### 进阶预览

下一章学习 **借用 (Borrowing)**, 即如何在**不转移所有权**的情况下访问数据, 这是 Rust 中最强大的工具之一, 也是所有权系统的"另一半"。

借用分为:
- **不可变借用 (immutable borrow)**: `&T`, 可以同时有多个
- **可变借用 (mutable borrow)**: `&mut T`, 同一时刻只能有一个

借用规则: 要么一个可变引用, 要么多个不可变引用, 但不能同时。

---

## 运行命令

```bash
# 编译并运行本章所有示例
cd chapters/05_ownership_move_copy_clone
cargo run

# 只检查编译是否通过 (不运行)
cargo check

# 查看编译器更详细的错误输出 (可以试注释掉某个错误行)
cargo build
```

## 练习

本章配套练习见 [EXERCISES.md](./EXERCISES.md)。

---

## 参考资源

- [The Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust By Example - Ownership and moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust Reference - Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [Rust Reference - Clone trait](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [Rust Reference - Drop trait](https://doc.rust-lang.org/std/ops/trait.Drop.html)
