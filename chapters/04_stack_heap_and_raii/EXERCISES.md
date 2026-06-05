# 第四章练习题: 栈、堆与 RAII

> 练习是理解内存模型的最好方式。下面的练习从基础到进阶, 覆盖本章核心概念。
> 每个练习都配有预期输出或检查点, 帮助你验证理解。

---

## 练习 1: 基础 — size_of 探索 (★☆☆☆☆)

**目标**: 熟悉 `std::mem::size_of` 的使用, 理解不同类型的栈上大小。

创建一个新的二进制项目或直接在 `main.rs` 中修改, 新增一个名为 `exercise_1_size_explorer`
的函数, 打印以下类型的大小 (使用 `size_of`):

- `u8`, `u16`, `u32`, `u64`
- `i8`, `i16`, `i32`, `i64`
- `f32`, `f64`
- `bool`, `char`
- `usize`, `isize`
- `&i32`, `&str`
- `Option<i32>`, `Option<&i32>`, `Option<bool>`
- `[u8; 0]`, `[u8; 1]`, `[u8; 100]`, `[u8; 1024]`
- `()`, `(i32,)`, `(i32, i32)`, `(i32, f64, char)`

**思考题:**

1. 为什么 `Option<i32>` 比 `i32` 大？(提示: niche optimization — 某些情况下编译器不需要额外字节)
2. 为什么 `Option<&i32>` 的大小和 `&i32` 一样？
3. 为什么 `[u8; 0]` 的大小是 0？
4. 在 64 位系统上, `usize` 的大小是多少？为什么？
5. `char` 为什么是 4 bytes？(提示: Rust 的 char 是 Unicode 标量值)

**预期输出示例:**

```
--- 练习 1: size_of 探索 ---
u8       = 1 bytes
u16      = 2 bytes
u32      = 4 bytes
u64      = 8 bytes
...
Option<i32>    = 8 bytes  (4 bytes data + 4 bytes discriminant)
Option<&i32>   = 8 bytes  (niche optimization: null pointer = None)
Option<bool>   = 1 bytes  (niche optimization: 2 < 256)
[0u8; 0]       = 0 bytes
[0u8; 1024]    = 1024 bytes
```

---

## 练习 2: 基础 — 自定义 Drop 观察析构顺序 (★★☆☆☆)

**目标**: 亲手实现一个带 Drop 的结构体, 观察析构顺序。

创建以下结构体并编写测试代码:

```rust
struct Tracker {
    id: u32,
    message: String,
}
```

要求:

1. 为 `Tracker` 实现 `new(id, message)` 构造函数, 打印 `"Tracker {id} created: {message}"`
2. 为 `Tracker` 实现 `Drop` trait, 打印 `"Tracker {id} dropped: {message}"`
3. 在 `main()` 中编写三个嵌套作用域, 每个作用域创建 2 个 Tracker, 观察 drop 顺序
4. 在代码注释中回答: drop 顺序是什么？这个顺序叫什么？

**预期行为:**

```
--- 练习 2: 嵌套作用域 ---
进入作用域 L1
  Tracker 1 created: L1-A
  Tracker 2 created: L1-B
  进入作用域 L2
    Tracker 3 created: L2-A
    Tracker 4 created: L2-B
    进入作用域 L3
      Tracker 5 created: L3-A
      Tracker 6 created: L3-B
    离开作用域 L3
    Tracker 6 dropped: L3-B    ← 注意: 先创建的后释放
    Tracker 5 dropped: L3-A
  离开作用域 L2
  Tracker 4 dropped: L2-B
  Tracker 3 dropped: L2-A
离开作用域 L1
Tracker 2 dropped: L1-B
Tracker 1 dropped: L1-A
```

---

## 练习 3: 进阶 — 手动 drop 与编译器交互 (★★★☆☆)

**目标**: 理解 `drop()` 函数的效果, 以及编译器的 move 检查。

编写代码完成以下任务:

1. 创建一个 `String`, 打印其内容
2. 使用 `drop()` 手动释放这个 String
3. 尝试在 drop 之后再次使用这个 String — 预期编译错误
4. 在注释中写出编译错误的英文信息, 并解释为什么 Rust 要阻止你这样做
5. 创建一个包含 `String` 的结构体 `MyData`, 实现 Drop, 尝试在 `drop()` 之后访问其字段

**预期编译错误示例:**

```rust
let s = String::from("hello");
drop(s);
println!("{}", s);  // 编译错误!
// error[E0382]: borrow of moved value: `s`
```

**思考题:**

1. `drop()` 函数签名是什么？(提示: `pub fn drop<T>(_x: T) {}` — 为什么只需要这个签名?)
2. 如果你是编译器, 你会如何检测 "use after drop" 错误？

---

## 练习 4: 进阶 — String 容量管理观察 (★★★☆☆)

**目标**: 深入理解 String 的 capacity 增长策略。

编写一个函数 `observe_string_growth()`, 完成:

1. 创建一个空的 `String::new()`, 打印初始 len 和 cap
2. 循环 100 次, 每次 `push_str("a")` (追加一个字符), 当 capacity 发生变化时打印日志:
   `"capacity changed: {old} -> {new} at len = {len}"`
3. 回答以下问题 (写在代码注释中):
   - capacity 增长的规律是什么？(大约是翻倍还是线性增长?)
   - 为什么采用这种增长策略？
4. 额外挑战: 使用 `Vec<i32>` 重复这个实验, 观察 Vec 的 capacity 增长是否类似

**预期输出示例:**

```
--- 练习 4: String 容量增长 ---
初始: len=0, cap=0
capacity changed: 0 -> 8  at len=1
capacity changed: 8 -> 16 at len=9
capacity changed: 16 -> 32 at len=17
...
```

---

## 练习 5: 进阶 — Box 与大小的关系 (★★☆☆☆)

**目标**: 理解 `Box<T>` 无论 T 多大, 自身大小始终等于一个指针。

编写一个函数 `box_size_demo()`:

1. 打印 `size_of::<Box<u8>>()`
2. 打印 `size_of::<Box<u64>>()`
3. 打印 `size_of::<Box<[u8; 1]>>()`
4. 打印 `size_of::<Box<[u8; 1024]>>()`
5. 打印 `size_of::<Box<[u8; 1048576]>>()` (1 MB 数组)
6. 在注释中解释: 为什么所有这些值都一样？
7. 创建一个 `Box<[i32; 5]>`, 修改数组元素, 验证 Box 的数据确实在堆上

**预期:** 所有 Box 类型的 `size_of` 都返回 8 (在 64 位系统上)。

---

## 练习 6: 进阶 — 对比 Rust String 和 Python str 的内存模型 (★★★☆☆)

**目标**: 通过一个具体的例子, 理解两种语言在内存管理上的根本差异。

**第一部分 (Rust 端):**

在 Rust 中创建:
- 1 个 i32 变量
- 1 个 `[i32; 3]` 数组
- 1 个 `String`
- 1 个 `Vec<i32>`

对每个变量, 用注释画出 ASCII 内存布局图 (类似 README.md 中的图), 标明:
- 哪些部分在栈上
- 哪些部分在堆上
- 每个部分的大小

**第二部分 (Python 对照):**

在注释中, 用 Python 创建对应的变量:
- `x = 42`
- `arr = [1, 2, 3]`
- `s = "hello"`
- `lst = [10, 20, 30]`

同样画出 ASCII 内存布局图, 标明:
- Python 对象在堆上的结构 (包含引用计数、类型指针等)
- 变量名与对象的关系

**第三部分 (分析):**

在注释中回答:
1. 哪种语言的整数运算开销更小？为什么？
2. 对于长度为 3 的数组, 哪种语言的内存消耗更少？
3. GC (垃圾回收) 和 RAII 哪种策略更适合实时系统？为什么？

---

## 练习 7: 综合 — 实现一个简单的内存追踪器 (★★★★☆)

**目标**: 综合运用 Drop、String、堆分配等知识, 实现一个小型工具。

实现一个结构体 `MemTracker`:

```rust
struct MemTracker {
    label: String,
    bytes_allocated: usize,
}
```

要求:

1. **构造函数**: `MemTracker::new(label: &str, size: usize)` — 创建一个追踪器,
   表示分配了 `size` bytes 的内存, 打印 `"ALLOC [{label}] {size} bytes"`
2. **Drop 实现**: 打印 `"FREE  [{label}] {size} bytes"`, 表示释放了内存
3. **借用检查演示**: 创建一个函数 `use_tracker(t: &MemTracker)`, 接受引用并打印
   `"Using: {label}"` — 注意这不会触发 drop
4. **取用演示**: 创建一个函数 `consume_tracker(t: MemTracker)`, 获取所有权并立即
   结束 — 观察 tracker 自动 drop
5. 在 main 中分别演示两种传递方式, 用注释说明:
   - 引用传递 (`&T`): 不转移所有权, 调用者仍然可以继续使用
   - 按值传递 (`T`): 转移所有权, 调用者不能再使用原变量

**预期输出示例:**

```
--- 练习 7: MemTracker ---
ALLOC [buffer_A] 1024 bytes
Using: buffer_A               ← 借出, 不释放
Using: buffer_A               ← 可以再次借出
FREE  [buffer_A] 1024 bytes   ← 离开作用域, 释放

ALLOC [buffer_B] 4096 bytes
                               ← consume_tracker 内部
FREE  [buffer_B] 4096 bytes   ← consume_tracker 结束, buffer_B 释放
// 此处 buffer_B 已不可用
```

---

## 练习 8: 挑战 — 理解编译器优化: 实际内存位置 (★★★★★)

**目标**: 使用 Rust 的不安全代码和裸指针, 窥探变量在内存中的实际位置。
**警告**: 此练习使用 `unsafe` 代码和裸指针, 仅用于学习目的。

使用裸指针获取变量的实际内存地址, 判断一个数据是否在栈上:

```rust
fn is_on_stack<T>(_val: &T) -> bool {
    // 在 64 位 Linux 上, 栈通常在地址 0x7fff... 附近
    // 堆通常在地址 0x55... 或 0x7f... 附近
    let ptr = _val as *const T as usize;
    // 一个近似判断: 高位地址通常是栈
    ptr > 0x7f00_0000_0000  // 这是一个简化, 实际因 OS 而异
}
```

要求:

1. 创建一个局部 `i32`, 打印其地址
2. 创建一个局部 `String`, 打印其栈上句柄的地址
3. 通过 `s.as_ptr()` 获取堆上数据的地址, 比较与栈地址的差异
4. 创建一个 `Box<i32>`, 分别打印指针 (栈上的 Box) 和指向的数据 (堆上的 i32) 的地址
5. 在注释中解释: 为什么地址差很多？不同地址范围有什么含义？
6. (选做) 打印函数的地址: `let fn_ptr = demonstrate_stack as *const () as usize;`,
   观察代码段、栈、堆的地址范围

**注意**: 此练习的输出因系统而异, 地址的具体值不重要, 重要的是观察不同内存区域的
地址范围差异。

---

## 练习 9: 思考题 — 设计决策 (无代码, ★★★☆☆)

在注释中回答以下问题:

1. Rust 为什么让 `i32` 等基本类型默认 Copy, 而 `String` 和 `Vec` 不 Copy？
   - (提示: 想想如果 String 是 Copy, 会发生什么)

2. 如果 Rust 像 Python 一样对所有类型采用引用计数 + GC, 会有什么优缺点？

3. 栈内存和堆内存哪种更适合以下场景？为什么？
   - a) 一个游戏循环中每帧创建的临时 3D 向量 (x, y, z)
   - b) 一个从文件读取的、大小未知的文本内容
   - c) 一个操作系统内核的中断处理程序中的临时数据
   - d) 一个 Web 服务器的请求体缓冲区 (可能从几 KB 到几 MB)

4. RAII 能管理文件、网络连接、锁等非内存资源吗？如果能, 请给出一个概念性示例。

---

## 练习 10: 额外挑战 — 实现一个简单的栈上字符串 (固定容量) (★★★★★)

**目标**: 实现一个最大容量固定的 "字符串" 类型, 完全在栈上分配。

设计一个 `StackString<const N: usize>` 类型 (使用 const generics):

```rust
struct StackString<const N: usize> {
    data: [u8; N],
    len: usize,
}
```

要求:

1. `new()` — 创建空的 StackString
2. `from_str(s: &str)` — 如果 `s.len() <= N`, 从 slice 复制数据; 否则 panic
3. `as_str()` — 返回 `&str`
4. `push(&mut self, ch: char)` — 追加一个字符, 超出容量则 panic
5. 实现 `Drop` (打印释放信息)
6. 实现 `std::fmt::Display` trait
7. 比较 `StackString<16>` 和 `String` 的 `size_of`, 解释差异

**预期:** `StackString<16>` 的 `size_of` 大约是 17 bytes (16 bytes data + 1 byte len
加上对齐), 而 `String` 的 `size_of` 始终是 24 bytes (三个 usize)。

**提示:** 注意 UTF-8 编码 — 一个 `char` 可能占用 1-4 个 bytes。简化起见, 你可以先
只支持 ASCII 字符 (每个 char 占 1 byte), 然后在注释中讨论如何处理完整的 UTF-8。

---

## 练习答案提示

- 练习 1: 注意 niche optimization — Rust 编译器利用无效位模式来优化 enum 大小
- 练习 2: Drop 顺序是 LIFO (Last In, First Out) — 和栈的工作方式一致
- 练习 3: `drop()` 本质是获取所有权然后什么都不做 — 被获取所有权的值变成无法访问
- 练习 4: capacity 增长策略是翻倍 (doubling), 摊还复杂度 O(1) per push
- 练习 5: `Box<T>` 是指针, `T` 的数据在堆上, 栈上的 Box 永远是指针大小
- 练习 6: Python 中即使是整数操作也涉及引用计数增减、对象分配 (小整数缓存除外)
- 练习 8: 需要 `#![feature(core_intrinsics)]` 或直接使用 `unsafe { ... }` 和
  `as *const T` 转换
- 练习 10: const generics 在 Rust 中是 `const N: usize` 语法, 属于进阶特性

---

> 完成练习后, 对照 README.md 中的概念检查自己的理解。
> 如果练习 1-5 都能独立完成并理解输出, 你已经准备好进入所有权章节了！

---

## 迁移思维练习

> 以下问题帮助你思考 C/C++ 中的内存管理模式如何重新建模为 Rust 的 RAII 设计。

### 问题 1：如何将一段 C 的 malloc/free 代码改为 Rust 的 RAII 设计？

假设你有一段 C 代码：先用 `malloc` 分配一块缓冲区，在函数的多个分支中都有可能提前 `return`，每个分支都必须记得调用 `free`。请思考：如果翻译成 Rust，你会在哪些地方用 `Box<T>` 或 `Vec<T>` 替代手动管理？Rust 的 Drop 机制如何消除"忘记释放"的风险？尤其是当函数中有提前返回或 panic 时，Rust 的 RAII 如何保证资源一定被回收？

**提示**：关注 Rust 的所有权和 Drop 的确定性——Rust 在作用域结束时自动调用 Drop，无论函数以什么方式退出。

### 问题 2：哪些资源管理场景在 Python 中被隐藏了，但在 C/Rust 中需要显式处理？

回顾你写过的 Python 代码：文件句柄、socket 连接、锁等资源，Python 提供了 `with` 语句和 GC 来辅助管理，很多程序员甚至不感知资源何时被释放。这些场景在 C 和 Rust 中分别需要怎么处理？C 和 Rust 在显式资源管理上有什么本质区别？为什么 Rust 不需要 `goto fail` 式的清理代码？

**提示**：Python 的 GC 只管理内存，不管文件句柄等 OS 资源；Rust 的 RAII 统一管理所有资源类型的生命周期。
