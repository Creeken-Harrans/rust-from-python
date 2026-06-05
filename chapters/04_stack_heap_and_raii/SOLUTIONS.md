# 第四章 练习答案 — 栈、堆与 RAII

---

## 练习 1: size_of 探索

### 练习 1-1: size_of 探索

#### 结论

不同类型的栈上大小由编译器和硬件平台决定。在 64 位系统上：
- 标量类型占用 1~16 bytes（取决于位宽和对齐）
- 引用 `&T` 固定为一个指针大小（8 bytes on 64-bit）
- 数组完全在栈上，大小为 N × sizeof(T)
- `Option<T>` 可以利用"无效位模式"(niche optimization)节省空间

#### 参考实现

```rust
use std::mem::size_of;

fn exercise_1_size_explorer() {
    println!("--- 练习 1: size_of 探索 ---");

    // 无符号整数
    println!("u8        = {} bytes", size_of::<u8>());
    println!("u16       = {} bytes", size_of::<u16>());
    println!("u32       = {} bytes", size_of::<u32>());
    println!("u64       = {} bytes", size_of::<u64>());

    // 有符号整数
    println!("i8        = {} bytes", size_of::<i8>());
    println!("i16       = {} bytes", size_of::<i16>());
    println!("i32       = {} bytes", size_of::<i32>());
    println!("i64       = {} bytes", size_of::<i64>());

    // 浮点数
    println!("f32       = {} bytes", size_of::<f32>());
    println!("f64       = {} bytes", size_of::<f64>());

    // 布尔和字符
    println!("bool      = {} bytes", size_of::<bool>());
    println!("char      = {} bytes", size_of::<char>());  // 4 bytes: Unicode 标量值

    // 平台相关整数
    println!("usize     = {} bytes", size_of::<usize>()); // 8 on 64-bit
    println!("isize     = {} bytes", size_of::<isize>()); // 8 on 64-bit

    // 引用
    println!("&i32      = {} bytes", size_of::<&i32>());  // 8: 一个指针
    println!("&str      = {} bytes", size_of::<&str>());  // 16: 指针 + 长度 (胖指针)

    // Option 的 niche optimization
    println!("Option<i32>     = {} bytes  (数据4 + 判别4 = 8, 或更多因对齐)", size_of::<Option<i32>>());
    println!("Option<&i32>    = {} bytes  (niche: null 指针 = None, 无需额外字节)", size_of::<Option<&i32>>());
    println!("Option<bool>    = {} bytes  (niche: 2 < 256 种可能, 1 byte 足够)", size_of::<Option<bool>>());
    println!("Option<Option<bool>> = {} bytes", size_of::<Option<Option<bool>>>());

    // 数组
    println!("[u8; 0]    = {} bytes  (零大小类型, 不占空间)", size_of::<[u8; 0]>());
    println!("[u8; 1]    = {} bytes", size_of::<[u8; 1]>());
    println!("[u8; 100]  = {} bytes", size_of::<[u8; 100]>());
    println!("[u8; 1024] = {} bytes", size_of::<[u8; 1024]>());

    // 元组
    println!("()        = {} bytes  (零大小类型)", size_of::<()>());
    println!("(i32,)    = {} bytes", size_of::<(i32,)>());
    println!("(i32, i32)     = {} bytes", size_of::<(i32, i32)>());
    println!("(i32, f64, char) = {} bytes (4+8+4=16, 可能有对齐填充)", size_of::<(i32, f64, char)>());
}
```

#### 思考题解答

**1. 为什么 `Option<i32>` 比 `i32` 大？**

`i32` 占 4 bytes。`Option<i32>` 需要区分 `Some(i32)` 和 `None`，需要一个"判别位"(discriminant)。在大多数平台上，编译器会分配额外的 4 bytes（共 8 bytes）来存储这个判别位。这是因为 i32 的 2^32 种位模式都是"有效"的，编译器找不到"无效位模式"来表示 None。

然而，`Option<bool>` 只有 1 byte——bool 只有 0 和 1 两种有效位模式，编译器可以用 2（或任何 >1 的值）来表示 None。这叫 **niche optimization**(生态位优化)。

**2. 为什么 `Option<&i32>` 的大小和 `&i32` 一样？**

引用 `&i32` 是一个指针（8 bytes on 64-bit）。Rust 的引用**永远不会为空**（Rust 没有 null 引用）。因此，全零的位模式（即 null 指针）对于 `&i32` 来说是无效的。编译器利用这个"niche"：当指针全零时表示 `None`，否则表示 `Some(ptr)`。这样 `Option<&i32>` 就不需要额外的判别字节。

**3. 为什么 `[u8; 0]` 的大小是 0？**

零长度数组不包含任何数据。Rust 允许零大小类型 (ZST, Zero-Sized Types)，它们在运行时占用 0 字节内存，但在编译期参与类型检查。这对于泛型编程非常有用（如 `()` 单元类型也是 ZST）。

**4. 在 64 位系统上，`usize` 的大小是多少？为什么？**

`usize` 是 8 bytes（64 bits）。`usize` 的大小等于目标平台的指针大小，因为它的主要用途是表示内存地址和数组索引。在 64 位系统上需要 64 位才能寻址完整的地址空间。

**5. `char` 为什么是 4 bytes？**

Rust 的 `char` 代表一个 **Unicode 标量值**(Unicode Scalar Value)，范围是 U+0000 到 U+D7FF 以及 U+E000 到 U+10FFFF，需要 21 位来表示。Rust 选择用 4 bytes (u32) 来存储每个 char，这与 C 的 `char`（1 byte）不同，但与 Python 3 的字符串内部编码思路类似（虽然 Python 内部是动态选择的）。

#### 常见错误

- 以为 `&str` 的大小是 8 bytes。实际上 `&str` 是**胖指针**(fat pointer)，包含指针(8 bytes) 和长度(8 bytes)，共 16 bytes。
- 以为 `[u8; 0]` 是编译错误。它完全合法，size 为 0。
- 忽视对齐填充的影响。元组 `(i32, f64, char)` 可能不是 4+8+4=16 bytes，编译器会插入填充字节以满足对齐要求。

#### 验证方式

```bash
cargo run
# 观察输出中的各个 size_of 值
```

---

## 练习 2: 自定义 Drop 观察析构顺序

#### 结论

Rust 的析构顺序是 **LIFO (Last In, First Out)**——后创建的先析构。这符合栈的工作方式：变量按声明顺序入栈，出作用域时逆序出栈并调用 Drop。

#### 参考实现

```rust
struct Tracker {
    id: u32,
    message: String,
}

impl Tracker {
    fn new(id: u32, message: &str) -> Self {
        println!("Tracker {id} created: {message}");
        Tracker {
            id,
            message: message.to_string(),
        }
    }
}

impl Drop for Tracker {
    fn drop(&mut self) {
        println!("Tracker {} dropped: {}", self.id, self.message);
    }
}

fn exercise_2_nested_scope() {
    println!("--- 练习 2: 嵌套作用域 ---");
    println!("进入作用域 L1");

    let _t1 = Tracker::new(1, "L1-A");
    let _t2 = Tracker::new(2, "L1-B");

    println!("  进入作用域 L2");
    {
        let _t3 = Tracker::new(3, "L2-A");
        let _t4 = Tracker::new(4, "L2-B");

        println!("    进入作用域 L3");
        {
            let _t5 = Tracker::new(5, "L3-A");
            let _t6 = Tracker::new(6, "L3-B");
            println!("    离开作用域 L3");
        }
        // L3 内变量按逆序析构: t6 → t5
        println!("  离开作用域 L2");
    }
    // L2 内变量按逆序析构: t4 → t3
    println!("离开作用域 L1");
}
// L1 内变量按逆序析构: t2 → t1
```

#### 预期输出

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
Tracker 6 dropped: L3-B
Tracker 5 dropped: L3-A
  离开作用域 L2
Tracker 4 dropped: L2-B
Tracker 3 dropped: L2-A
离开作用域 L1
Tracker 2 dropped: L1-B
Tracker 1 dropped: L1-A
```

#### 为什么这样设计

Rust 的 Drop 顺序是确定性、可预测的。LIFO 顺序天然防止了一个值在析构时访问已被释放的依赖值。例如，如果结构体 A 包含对结构体 B 的引用，B 在 A 之后声明，则 B 先析构，A 后析构——这是安全的，因为 A 析构时 B 还存在。如果按先进先出 (FIFO) 顺序，则 A 析构时 B 可能已不存在，造成悬垂引用。

RAII (Resource Acquisition Is Initialization) 的历史：
- **C 的 malloc/free**：完全手动，容易忘记 free 或在错误路径上泄漏
- **C++ 的 RAII**：通过构造函数获取资源、析构函数释放资源，利用栈展开保证释放。但 C++ 的 RAII 依赖程序员正确编写构造/析构函数，且与异常安全紧密相关。
- **Rust 的 RAII**：编译器强制保证 Drop 被调用。Rust 没有异常（只有 panic），Drop 在 panic 展开时也会被调用（除非编译时设置 abort-on-panic）。Rust 的 RAII 是语言级别的保证，不需要程序员写 `try-finally` 或 `goto fail`。

#### 常见错误

- 以为 Drop 顺序是 FIFO（声明顺序）。实际是 LIFO。
- 在 Drop 中尝试访问已被 move 的字段——Drop 接受 `&mut self`，字段可能已被 `Option::take()` 等方法移出。
- 忘记互递归的 Drop 可能导致栈溢出。

#### 验证方式

直接运行代码，观察输出中 "created" 和 "dropped" 的顺序是否相反。

---

## 练习 3: 手动 drop 与编译器交互

#### 结论

`std::mem::drop()` 的本质是接收一个值并获取其所有权，然后什么都不做——值在函数结束时因离开作用域而被 Drop。它签名是 `pub fn drop<T>(_x: T) {}`，利用了 Rust 所有权和 Drop 规则。

调用 `drop(x)` 后，`x` 的所有权已转移，编译器禁止再次使用。

#### 参考实现

```rust
fn exercise_3_manual_drop() {
    println!("--- 练习 3: 手动 drop ---");

    // 1. 创建 String
    let s = String::from("hello");
    println!("s = {}", s);

    // 2. 手动 drop
    drop(s);

    // 3. 尝试再次使用 s —— 编译错误!
    // println!("{}", s);
    // 编译错误信息:
    // error[E0382]: borrow of moved value: `s`
    //   --> src/main.rs:...
    //    |
    //    |     let s = String::from("hello");
    //    |         - move occurs because `s` has type `String`,
    //    |           which does not implement the `Copy` trait
    //    |     drop(s);
    //    |          - value moved here
    //    |     println!("{}", s);
    //    |                    ^ value borrowed here after move
    //
    // Rust 阻止这样做是因为:
    // - drop(s) 将 s 的所有权转移到 drop 函数内部
    // - drop 函数返回后, s 已被释放 (Drop trait 被调用)
    // - 之后再使用 s 就是 use-after-free —— Rust 在编译期禁止了它

    // 4. 包含 String 的结构体
    struct MyData {
        name: String,
        value: i32,
    }

    impl Drop for MyData {
        fn drop(&mut self) {
            println!("MyData::drop called for: {}", self.name);
        }
    }

    let data = MyData {
        name: String::from("test-data"),
        value: 42,
    };
    println!("data.name = {}, data.value = {}", data.name, data.value);

    drop(data);

    // 编译错误: data 已被移动
    // println!("{}", data.name);  // error[E0382]
}
```

#### `drop()` 函数签名分析

```rust
pub fn drop<T>(_x: T) {}
```

为什么只需要这个签名就足够？

1. `_x: T` 表示函数接收 `T` 的所有权（按值传参）
2. 因为 `T` 没有 trait bound，它适用于任何类型
3. 参数名以 `_` 开头，告诉编译器"我们故意不使用这个值"（避免 unused 警告）
4. 函数体为空 `{}`——值在函数结束时因离开作用域而自动调用 Drop
5. 这本质上利用了 Rust 的所有权 + Drop 规则，零运行时开销

#### 编译器如何检测 "use after drop"

1. 编译器跟踪每个值的所有权状态
2. `drop(x)` 调用导致 `x` 的所有权被移走——编译器标记 `x` 为 "moved"
3. 之后任何对 `x` 的访问都会被编译器阻止，因为它处于"未初始化"状态
4. 这是 Rust 借用检查器(Borrow Checker)的核心职责之一

#### 常见错误

- 试图在 `drop()` 后使用变量——编译器会明确拒绝，错误码 E0382
- 以为 `drop()` 只是"标记为可释放"——实际上它立即释放
- 在循环中手动 `drop()` 大量数据——通常不需要，让作用域自然结束即可

#### 验证方式

取消代码中的注释行，尝试编译，观察编译器错误信息，并记录错误码和描述。

---

## 练习 4: String 容量管理观察

#### 结论

Rust 的 `String`（以及 `Vec<T>`）采用 **翻倍扩容**（doubling）策略。当 `push` 导致 `len > cap` 时，容量会翻倍（或从 0 增长到某个初始值）。这使得 `push` 的摊还时间复杂度为 O(1)。

#### 参考实现

```rust
fn observe_string_growth() {
    println!("--- 练习 4: String 容量增长 ---");

    let mut s = String::new();
    let mut prev_cap = s.capacity();
    println!("初始: len={}, cap={}", s.len(), prev_cap);

    for i in 1..=100 {
        s.push('a');
        let current_cap = s.capacity();
        if current_cap != prev_cap {
            println!(
                "capacity changed: {} -> {} at len = {}",
                prev_cap, current_cap, s.len()
            );
            prev_cap = current_cap;
        }
    }
    println!("最终: len={}, cap={}", s.len(), s.capacity());
}

// Vec<i32> 版本的实验
fn observe_vec_growth() {
    println!("\n--- Vec 容量增长 ---");

    let mut v: Vec<i32> = Vec::new();
    let mut prev_cap = v.capacity();
    println!("初始: len={}, cap={}", v.len(), prev_cap);

    for i in 1..=100 {
        v.push(i);
        let current_cap = v.capacity();
        if current_cap != prev_cap {
            println!(
                "capacity changed: {} -> {} at len = {}",
                prev_cap, current_cap, v.len()
            );
            prev_cap = current_cap;
        }
    }
    println!("最终: len={}, cap={}", v.len(), v.capacity());
}
```

#### 预期输出示例

```
--- 练习 4: String 容量增长 ---
初始: len=0, cap=0
capacity changed: 0 -> 8  at len = 1
capacity changed: 8 -> 16 at len = 9
capacity changed: 16 -> 32 at len = 17
capacity changed: 32 -> 64 at len = 33
capacity changed: 64 -> 128 at len = 65
最终: len=100, cap=128
```

#### 为什么采用这种增长策略？

1. **摊还 O(1)**：虽然单次扩容可能需要 O(n) 复制数据，但经过 n 次 push，总复制次数为 O(n)，每次 push 的平均成本为 O(1)。证明：从容量 1 翻倍到容量 n，总共复制了约 1+2+4+...+n/2 < n 个元素。

2. **空间换时间**：翻倍策略用较多的预留空间换取更少的扩容次数。如果采用线性增长（每次+1 或 +固定值），扩容频繁且总复制成本为 O(n^2)。

3. **与主流实现一致**：C++ 的 `std::vector`、Java 的 `ArrayList`、Go 的 slice 都采用类似策略（通常是 1.5x 或 2x 增长）。

4. **为什么是约 2x 而不是 1.5x？** Rust 的 `Vec` 使用 2x 增长（实际实现中是从 0 开始逐步翻倍）。2x 更简单，但 1.5x 在某些场景下可以重用之前释放的内存块（因为前一次分配 + 新分配 < 后一次翻倍所需的空间），减少内存碎片。

#### 常见错误

- 以为 `capacity()` 返回堆上数据的大小——它返回的是**已分配的容量**（元素个数），不是字节数。
- 混淆 `len` 和 `cap`：`len` 是当前有效元素数，`cap` 是已分配的容量。
- 当 `cap > len` 时，多余的空间存在于堆上，但不影响 `size_of::<String>()`——栈上的句柄大小始终是 24 bytes。

#### 验证方式

运行代码，观察 capacity 变化的规律（大约是 2x 翻倍）。

---

## 练习 5: Box 与大小的关系

#### 结论

`Box<T>` 本质上是一个**指针**。无论 `T` 是什么类型（即使 T 是 1MB 的数组），`Box<T>` 在栈上的大小始终等于一个指针的大小（8 bytes on 64-bit 系统）。数据本身在堆上。

#### 参考实现

```rust
use std::mem::size_of;

fn box_size_demo() {
    println!("--- 练习 5: Box<T> 的大小 ---");

    println!("Box<u8>         = {} bytes", size_of::<Box<u8>>());
    println!("Box<u64>        = {} bytes", size_of::<Box<u64>>());
    println!("Box<[u8; 1]>    = {} bytes", size_of::<Box<[u8; 1]>>());
    println!("Box<[u8; 1024]> = {} bytes", size_of::<Box<[u8; 1024]>>());
    println!("Box<[u8; 1048576]> = {} bytes (1 MB 数组)", size_of::<Box<[u8; 1048576]>>());

    // 所有值应该都是 8 (在 64 位系统上)

    // 验证 Box 的数据在堆上
    let mut b = Box::new([1, 2, 3, 4, 5]);
    println!("\nBox 内的数据: {:?}", b);
    b[0] = 99;  // 通过解引用修改堆上的数据
    println!("修改后: {:?}", b);
}
```

#### 为什么 Box<T> 的大小都是一样的？

`Box<T>` 的内存布局：

```
栈上: [ptr: *mut T]   ← 8 bytes (固定)
       |
       ↓ 指向
堆上: [T 的实际数据]   ← sizeof(T) bytes (可变)
```

`Box<T>` 只是一个拥有堆分配内存所有权的指针。数据的大小不影响指针本身的大小——就像无论你的房子有多大，你手里的钥匙大小不变。

#### 为什么这样设计

- **栈大小可预测**：无论你操作多大的数据，栈上的句柄始终是指针大小。这保证了函数调用的栈帧大小不依赖动态数据。
- **移动开销恒定**：移动 `Box<T>` 只是复制 8 bytes 的指针（以及释放原指针），不管 T 多大。这就是为什么 Rust 的 Move 是轻量级的——它实际上是浅拷贝 + 原变量失效。
- **RAII 自动管理**：Box<T> 实现 Drop，离开作用域时自动释放堆内存。

#### 常见错误

- 以为 `Box<[u8; 1024]>` 的 size_of 是 1024——实际是 8。Box 只是一个指针。
- 混淆 `size_of::<Box<T>>()` 和实际的堆分配大小。前者永远是 8，后者取决于 T。

#### 验证方式

运行代码，确认所有 Box 类型的 size_of 输出相同（8 bytes on 64-bit）。

---

## 练习 6: 对比 Rust String 和 Python str 的内存模型

#### 结论

Rust 和 Python 在内存管理上有根本性的哲学差异：Rust 使用编译期确定的 RAII + 所有权模型，Python 使用运行时的引用计数 + GC。这导致即使是相同的"整数"操作，内存开销和性能也完全不同。

#### 参考实现（内存布局图，写在注释中）

```rust
fn memory_layout_comparison() {
    println!("--- 练习 6: Rust vs Python 内存布局 ---");

    // ====================
    // Rust 端
    // ====================

    let x: i32 = 42;
    /*
     * Rust i32 内存布局:
     *
     * 栈 (Stack)
     * ┌─────────────┐
     * │ x: 0x2A000000 │ ← 4 bytes, 直接在栈上
     * └─────────────┘
     *
     * 总计: 4 bytes, 全在栈上
     * 无堆分配, 无引用计数, 无类型指针
     */

    let arr: [i32; 3] = [1, 2, 3];
    /*
     * Rust [i32; 3] 内存布局:
     *
     * 栈 (Stack)
     * ┌──────┬──────┬──────┐
     * │  1   │  2   │  3   │ ← 3 × 4 = 12 bytes, 直接在栈上
     * └──────┴──────┴──────┘
     *
     * 总计: 12 bytes, 全在栈上
     */

    let s: String = String::from("hello");
    /*
     * Rust String 内存布局:
     *
     * 栈 (Stack)                    堆 (Heap)
     * ┌────────────┐               ┌─┬─┬─┬─┬─┬───┐
     * │ ptr ───────┼──────────────→│h│e│l│l│o│...│
     * │ len = 5    │               └─┴─┴─┴─┴─┴───┘
     * │ cap = 5    │                ← 5 bytes 数据
     * └────────────┘
     * 栈上: 24 bytes (ptr+len+cap)
     * 堆上: 5 bytes 数据 + 可能 padding
     */

    let v: Vec<i32> = vec![10, 20, 30];
    /*
     * Rust Vec<i32> 内存布局:
     *
     * 栈 (Stack)                    堆 (Heap)
     * ┌────────────┐               ┌────┬────┬────┬───┐
     * │ ptr ───────┼──────────────→│ 10 │ 20 │ 30 │...│
     * │ len = 3    │               └────┴────┴────┴───┘
     * │ cap = 3    │               ← 3 × 4 = 12 bytes
     * └────────────┘
     * 栈上: 24 bytes, 堆上: 12+ bytes
     */

    // ====================
    // Python 对照 (概念性, 写在注释中)
    // ====================

    /*
     * Python: x = 42
     *
     * 栈 / 帧                              堆 (Heap)
     * ┌────────┐                          ┌──────────────────┐
     * │ x ─────┼─────────────────────────→│ PyObject (int)    │
     * └────────┘                          │  ob_refcnt: 1     │ ← 8 bytes
     *                                     │  ob_type: *PyType │ ← 8 bytes
     *                                     │  ob_size: 0       │ ← 8 bytes
     *                                     │  ob_digit: [42]   │ ← 至少 4 bytes
     *                                     └──────────────────┘
     * Python 的整数是堆上的 PyLongObject, 至少 ~28 bytes (CPython)
     */

    /*
     * Python: arr = [1, 2, 3]
     *
     * 栈 / 帧                              堆 (Heap)
     * ┌────────┐                          ┌────────────────────┐
     * │ arr ───┼─────────────────────────→│ PyListObject        │
     * └────────┘                          │  ob_refcnt: 1       │
     *                                     │  ob_type: *PyType   │
     *                                     │  ob_item: *PyObject │──→ [*obj1, *obj2, *obj3]
     *                                     │  allocated: 3       │   每个 obj 是 PyLongObject
     *                                     └────────────────────┘
     * 列表本身 + 3 个整数对象 + 指针数组 → 大量堆分配
     */

    /*
     * Python: s = "hello"
     *
     * ┌────────┐                          ┌──────────────────┐
     * │ s ─────┼─────────────────────────→│ PyUnicodeObject   │
     * └────────┘                          │  ob_refcnt: 1     │
     *                                     │  ob_type: *PyType │
     *                                     │  length: 5        │
     *                                     │  hash: -1         │
     *                                     │  state: ...       │
     *                                     │  data: "hello"    │
     *                                     └──────────────────┘
     * Python 字符串也是堆对象, 含引用计数和类型信息
     */

    /*
     * Python: lst = [10, 20, 30]
     * 类似 arr, 每个元素又是独立的 PyLongObject
     */
}

fn memory_analysis() {
    // 分析回答 (写在注释/打印中)
    println!("\n--- 分析 ---");
    println!("1. 整数运算开销:");
    println!("   Rust: i32 在栈上, 赋值/加减是单条 CPU 指令");
    println!("   Python: 每次运算需要创建新 PyLongObject (堆分配),");
    println!("         增减引用计数, 检查是否需要 GC");
    println!("   => Rust 整数运算开销小 1-3 个数量级");

    println!("\n2. 长度为3的数组内存消耗:");
    println!("   Rust [i32; 3]: 12 bytes 栈上, 无堆分配");
    println!("   Python [1,2,3]: 列表对象 (~56 bytes) + 3×整数对象 (~28 bytes each)");
    println!("                   + 指针数组 (3×8 bytes)");
    println!("                   ≈ 56 + 84 + 24 = ~164 bytes, 全在堆上");
    println!("   => Rust 内存消耗小一个数量级");

    println!("\n3. GC vs RAII 对实时系统的适用性:");
    println!("   GC: 回收时机不确定, 可能导致'停顿'(stop-the-world),");
    println!("       不适合硬实时系统 (飞行控制、自动驾驶等)");
    println!("   RAII: 释放时机确定 (离开作用域立即释放),");
    println!("         开销可预测, 适合实时系统");
    println!("   => RAII 更适合实时系统");
}
```

#### 为什么这样设计

- **Rust 选择栈优先**：编译期能确定大小的数据放在栈上，极快且自动管理。只有大小不定的数据才上堆，且由 RAII 管理。
- **Python 选择"一切都是对象"**：所有数据都在堆上，通过引用计数 + GC 管理。这带来了灵活性（动态类型、内省），但代价是内存和性能开销。
- **语言目标不同**：Python 目标是开发效率和简洁性，Rust 目标是性能和安全性。两者的内存模型反映了各自的定位。

#### 常见错误

- 以为 Python 的整数 `x = 42` 是栈上的值——实际上它是对堆上 PyObject 的引用。
- 以为 Python 小整数缓存（-5 到 256）是栈分配——仍然是堆对象，只是复用。
- 低估 GC 对实时系统的影响——即使用分代 GC 优化，"stop-the-world" 暂停仍然可能发生。

#### 验证方式

- Rust 端：运行代码，观察 size_of 输出
- Python 端：使用 `sys.getsizeof()` 查看对象大小（只看到对象本身，不包含引用的子对象）

---

## 练习 7: 实现一个简单的内存追踪器

#### 结论

所有权转移（`T`）和引用借用（`&T`）是 Rust 资源管理的两种基本模式。`MemTracker` 使用 Drop 追踪资源的分配和释放，直观展示 RAII 的工作方式。

#### 参考实现

```rust
struct MemTracker {
    label: String,
    bytes_allocated: usize,
}

impl MemTracker {
    fn new(label: &str, size: usize) -> Self {
        println!("ALLOC [{}] {} bytes", label, size);
        MemTracker {
            label: label.to_string(),
            bytes_allocated: size,
        }
    }
}

impl Drop for MemTracker {
    fn drop(&mut self) {
        println!("FREE  [{}] {} bytes", self.label, self.bytes_allocated);
    }
}

// 引用传递: 不转移所有权, 调用者仍可使用
fn use_tracker(t: &MemTracker) {
    println!("Using: {}", t.label);
    // t 是借用, 离开此函数时不会调用 Drop
}

// 按值传递: 转移所有权, 函数结束时 tracker 自动 drop
fn consume_tracker(t: MemTracker) {
    println!("Consuming: {}", t.label);
    // t 在此离开作用域, Drop 被调用
}

fn exercise_7_memtracker() {
    println!("--- 练习 7: MemTracker ---");

    // 引用传递演示
    let buffer_a = MemTracker::new("buffer_A", 1024);
    use_tracker(&buffer_a);   // 借出, 不释放
    use_tracker(&buffer_a);   // 可以再次借出 (只要没有可变引用)
    // buffer_a 在这里离开作用域, Drop 被调用

    println!();

    // 按值传递演示
    let buffer_b = MemTracker::new("buffer_B", 4096);
    consume_tracker(buffer_b);  // 所有权转移, 函数内 drop
    // buffer_b 已被移动, 不能再使用
    // println!("{}", buffer_b.label); // ❌ 编译错误: E0382

    println!();
    println!("main 函数即将结束, 所有剩余 tracker 将被释放");
}
```

#### 预期输出

```
--- 练习 7: MemTracker ---
ALLOC [buffer_A] 1024 bytes
Using: buffer_A
Using: buffer_A
FREE  [buffer_A] 1024 bytes

ALLOC [buffer_B] 4096 bytes
Consuming: buffer_B
FREE  [buffer_B] 4096 bytes

main 函数即将结束, 所有剩余 tracker 将被释放
```

#### 所有权模型对比

| 传递方式 | 签名 | 所有权 | 调用后原变量 | 何时 Drop |
|----------|------|--------|-------------|-----------|
| 引用 `&T` | `fn use_tracker(t: &MemTracker)` | 不转移 | 仍可用 | 离开原作用域 |
| 按值 `T` | `fn consume_tracker(t: MemTracker)` | 转移 | 不可用 | 函数结束时 |

#### 为什么这样设计

- **`&T` 语义**："我需要查看这个值，但我不拥有它"。多个函数可以同时借用同一个值。
- **`T` 语义**："我接管这个值，并负责它的生命周期"。适合需要独占数据或将数据"消费"掉的场景。
- **编译器检查**：Rust 在编译时保证引用不会比数据活得更长，消耗值不会被意外重用。

#### 常见错误

- 在 `consume_tracker` 后尝试使用原变量——编译错误 E0382
- 尝试在持有 `&T` 的同时创建 `&mut T`——违反借用规则
- 在 `Drop` 实现中 panic——可能导致 "double panic" 而 abort

#### 验证方式

运行代码，观察 ALLOC 和 FREE 的配对输出顺序。

---

## 练习 8: 理解编译器优化 — 实际内存位置

#### 结论

通过裸指针观察变量的实际内存地址，可以直观理解栈、堆、代码段的不同地址范围。在 64 位 Linux 上，栈通常在高地址（`0x7ff...`），堆在较低地址（`0x55...` 或 `0x7f...`），代码段在中低地址。

#### 参考实现

```rust
// 注意: 此练习使用裸指针, 仅用于学习目的
// 实际代码中不应依赖地址范围来判断栈/堆

fn exercise_8_memory_address() {
    println!("--- 练习 8: 实际内存位置 ---");

    // 1. 局部 i32 的地址
    let stack_int: i32 = 42;
    let stack_int_ptr: *const i32 = &stack_int;
    println!("局部 i32 地址:     {:p}", stack_int_ptr);

    // 2. 局部 String 的句柄在栈上
    let s = String::from("Hello, memory!");
    let handle_ptr: *const String = &s;
    println!("String 句柄地址:   {:p} (栈上)", handle_ptr);

    // 3. String 堆数据的地址
    let heap_data_ptr = s.as_ptr();
    println!("String 数据地址:   {:p} (堆上)", heap_data_ptr);
    println!(
        "  栈地址 - 堆地址 ≈ {} bytes 差异",
        handle_ptr as usize - heap_data_ptr as usize
    );

    // 4. Box<i32>: 栈上的 Box 和堆上的 i32
    let b = Box::new(999);
    let box_ptr: *const Box<i32> = &b;
    let boxed_data_ptr: *const i32 = &*b;
    println!("Box 自身地址:      {:p} (栈上)", box_ptr);
    println!("Box 指向的数据地址: {:p} (堆上)", boxed_data_ptr);
    println!(
        "  Box 栈地址 - Box 堆地址 ≈ {} bytes 差异",
        box_ptr as usize - boxed_data_ptr as usize
    );

    // 5. 函数指针 (代码段)
    let fn_ptr = exercise_8_memory_address as *const () as usize;
    println!("函数代码段地址:    0x{:x}", fn_ptr);

    // 地址范围分析 (64位 Linux 典型值, 实际值因 ASLR 而异)
    println!("\n地址范围分析:");
    println!("  代码段 (text):  通常在 0x55_xxxx_xxxx_xxxx 附近");
    println!("  堆 (heap):      通常在 0x55_xxxx_xxxx_xxxx 或 0x7f_xxxx_xxxx_xxxx");
    println!("  栈 (stack):     通常在 0x7f_ff_xxxx_xxxx 附近 (高地址)");
    println!("  库映射 (mmap):  通常在 0x7f_xxxx_xxxx_xxxx");
    println!();
    println!("  为什么地址差很多?");
    println!("  - 栈从高地址向低地址增长 (传统设计)");
    println!("  - 堆从低地址向高地址增长 (brk/sbrk)");
    println!("  - 中间留给动态库映射 (mmap)");
    println!("  - 这样设计可以最大化堆可增长的空间");
    println!("  - ASLR (地址空间布局随机化) 为安全引入随机偏移");
}
```

#### 为什么这样设计（地址空间布局）

现代操作系统使用虚拟内存。每个进程有独立的虚拟地址空间：

```
0x0000_0000_0000_0000
  ↓ [未映射, 用于捕获空指针解引用]
  ↓ [代码段 .text]  
  ↓ [数据段 .data / .bss]
  ↓ [堆 heap — brk/sbrk 向上增长]
  ↓ ... 大段未使用空间 ...
  ↓ [动态库 mmap 区域]
  ↓ ... 大段未使用空间 ...
  ↓ [栈 stack — 从高地址向下增长]
0x7f_ff_ffff_ffff
```

这种"堆栈相向增长"的设计让堆有充裕的向上空间，栈有充裕的向下空间，避免两者早期碰撞。

#### 常见错误

- 认为 `stack_int` 的地址可以用来判断任意地址——地址因 ASLR 而异
- 混淆 `s.as_ptr()`（堆数据地址）和 `&s`（栈句柄地址）
- 在 release 模式下，编译器可能将变量优化到寄存器中，使得某些变量根本没有地址

#### 验证方式

运行代码，观察不同变量的地址差异。注意 ASLR 会让每次运行的地址不同，但相对关系（栈高堆低）不变。

---

## 练习 9: 设计决策（思考题）

#### 结论

Rust 的所有权系统和 RAII 是一套经过深思熟虑的设计，每一个"为什么这样"后面都有具体的工程理由。

#### 思考题解答

**1. Rust 为什么让 `i32` 等基本类型默认 Copy，而 `String` 和 `Vec` 不 Copy？**

如果 `String` 是 Copy（浅拷贝），会发生严重的 double-free 问题：

```rust
// 假设 String 是 Copy (危险场景)
let s1 = String::from("hello");
let s2 = s1;  // 浅拷贝: s2.ptr == s1.ptr, 两者指向同一堆内存
// s1 和 s2 离开作用域时, 各自调用 Drop
// → 同一块堆内存被释放两次 → double free → 未定义行为!
```

`i32` 是 Copy 安全的，因为它不涉及堆内存——复制就是真正的独立副本。这是 Rust 的"位复制即安全复制"原则：只有完全在栈上的、没有资源所有权的类型才能实现 Copy。

**2. 如果 Rust 像 Python 一样对所有类型采用引用计数 + GC，优缺点？**

优点：
- 编程更简单，没有所有权/借用规则的学习曲线
- 不需要显式标注生命周期
- 循环数据结构更容易表达

缺点：
- 运行时开销（每次赋值/传参都要增减引用计数）
- GC 停顿不可预测，不适合实时系统
- 内存占用更大（每个对象都有引用计数字段）
- 无法在编译期保证线程安全（需要运行时锁）
- 失去了"零成本抽象"的优势
- 无法表达确定性析构（`Drop` 调用时机不确定）

**3. 栈 / 堆选择：**

| 场景 | 选择 | 理由 |
|------|------|------|
| a) 游戏循环中每帧创建的临时 3D 向量 | **栈** | 大小固定 (12 bytes)、生命周期极短 (一帧)、分配/释放免费 (栈指针移动) |
| b) 从文件读取的未知大小文本 | **堆** | 大小编译期未知、需要动态增长 (String) |
| c) 操作系统内核中断处理程序中的临时数据 | **栈** | 中断上下文不能休眠、不能触发缺页、不能堆分配 (可能阻塞)；栈分配是确定性的 |
| d) Web 服务器请求体缓冲区 (几KB~几MB) | **堆** | 大小不确定且可能很大、栈空间有限 (通常 8MB)；过大的栈分配可能导致栈溢出 |

**核心决策标准**：
- **编译期已知大小 + 短期存活 → 栈**
- **编译期未知大小 或 需要比创建者活得更久 → 堆**
- **大小过大 (>几KB) → 堆**（避免栈溢出）

**4. RAII 能管理非内存资源吗？**

能。RAII 的核心思想是将任何资源的生命周期绑定到一个值的生命周期。标准库中已有大量例子：

```rust
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

// 文件: File 实现 Drop, 离开作用域时自动关闭
fn example_file() {
    let mut f = File::create("/tmp/test.txt").unwrap();
    f.write_all(b"hello").unwrap();
    // f 离开作用域 → Drop → 关闭文件句柄 → OS 资源释放
}

// 锁: MutexGuard 实现 Drop, 离开作用域时自动解锁
fn example_lock() {
    let m = Mutex::new(0);
    {
        let mut guard = m.lock().unwrap();  // 获取锁
        *guard += 1;
        // guard 离开作用域 → Drop → 自动解锁
        // 即使 panic 发生, 栈展开也会调用 Drop, 不会死锁!
    }
}

// 网络连接: TcpStream 实现 Drop, 自动关闭连接
use std::net::TcpStream;
fn example_network() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    // stream 离开作用域 → Drop → 关闭连接
    Ok(())
}
```

对比 C 语言：

```c
// C 版本: 需要手动清理, 每个错误路径都要记得 close
void c_example() {
    int fd = open("/tmp/test.txt", O_WRONLY | O_CREAT);
    if (fd < 0) return;
    
    int result = write(fd, "hello", 5);
    if (result < 0) {
        close(fd);  // 必须手动关闭!
        return;
    }
    
    // ... 更多可能的错误路径, 每个都要 close(fd) ...
    
    close(fd);  // 正常路径也要手动关闭
}
```

#### 常见错误

- 以为 RAII 只管理内存——它管理一切实现了 Drop 的资源
- 在 Drop 中做复杂操作（如 I/O）——Drop 中的错误处理有限制
- 在 RAII 类型中持有原始指针但没有正确实现 Drop——导致泄漏或 double free

---

## 练习 10: 实现一个简单的栈上字符串（固定容量）

#### 结论

通过 const generics 实现完全在栈上的字符串类型，可以深入理解 `String` 为什么需要堆分配，以及固定大小和动态大小的本质区别。

#### 参考实现

```rust
use std::fmt;

struct StackString<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    /// 创建空的 StackString
    fn new() -> Self {
        StackString {
            data: [0u8; N],
            len: 0,
        }
    }

    /// 从 &str 创建, 内容必须在容量范围内
    fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        assert!(
            bytes.len() <= N,
            "StackString<{N}>: input length {} exceeds capacity",
            bytes.len()
        );
        let mut data = [0u8; N];
        data[..bytes.len()].copy_from_slice(bytes);
        StackString {
            data,
            len: bytes.len(),
        }
    }

    /// 返回字符串切片
    fn as_str(&self) -> &str {
        // 安全: data[..len] 保证是有效的 UTF-8 (from_str 保证了)
        std::str::from_utf8(&self.data[..self.len]).unwrap()
    }

    /// 追加一个 ASCII 字符 (简化: 不处理 multi-byte UTF-8)
    fn push_ascii(&mut self, ch: u8) {
        assert!(
            self.len < N,
            "StackString<{N}>: capacity exceeded on push"
        );
        self.data[self.len] = ch;
        self.len += 1;
    }

    /// 追加一个 char (正确处理 UTF-8)
    fn push(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);
        let char_bytes = encoded.as_bytes();
        assert!(
            self.len + char_bytes.len() <= N,
            "StackString<{N}>: capacity exceeded, need {} bytes for '{}'",
            char_bytes.len(),
            ch
        );
        self.data[self.len..self.len + char_bytes.len()].copy_from_slice(char_bytes);
        self.len += char_bytes.len();
    }
}

impl<const N: usize> Drop for StackString<N> {
    fn drop(&mut self) {
        println!(
            "StackString<{N}> dropped: \"{}\" (len={})",
            self.as_str(),
            self.len
        );
    }
}

impl<const N: usize> fmt::Display for StackString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn exercise_10_stack_string() {
    println!("--- 练习 10: StackString ---");

    use std::mem::size_of;

    // 比较大小
    println!("size_of::<StackString<16>>() = {} bytes", size_of::<StackString<16>>());
    println!("size_of::<StackString<32>>() = {} bytes", size_of::<StackString<32>>());
    println!("size_of::<String>()          = {} bytes", size_of::<String>());

    // 解释: StackString<16> = [u8; 16] + usize (len) + 对齐填充
    //       大约 16 + 8 = 24 bytes, 都在栈上
    //       String = ptr(8) + len(8) + cap(8) = 24 bytes 栈上, 数据在堆上
    //
    // 虽然 size_of 可能差不多, 但关键区别是:
    // - StackString<16>: 数据在栈上, 没有堆分配
    // - String: 数据在堆上, 有堆分配开销

    // 使用演示
    let mut ss = StackString::<16>::from_str("Hello");
    println!("ss = {}", ss);
    println!("ss.len() = {}", ss.len());

    ss.push(' ');
    ss.push('R');
    ss.push('u');
    ss.push('s');
    ss.push('t');
    println!("after push: ss = {}", ss);

    // 超出容量应该 panic:
    // let ss2 = StackString::<4>::from_str("too long"); // panic!

    println!("\n离开作用域, StackString 将被 drop:");
}
```

#### 为什么这样设计

`StackString<const N: usize>` 和 `String` 的设计差异：

| 特性 | StackString<N> | String |
|------|---------------|--------|
| 容量 | 编译期固定 | 运行时可增长 |
| 数据位置 | 栈 | 堆 |
| 分配开销 | 无 | 有 (堆分配) |
| 最大限制 | 栈帧大小 (如 8MB) | 可用内存 (GB 级) |
| 移动开销 | O(N) (复制整个数组) | O(1) (复制 24 bytes 指针) |
| 适用场景 | 短字符串、嵌入式、no_std | 通用目的 |

String 在堆上分配的原因：
1. 容量可以动态增长（翻倍策略）
2. 移动开销 O(1)（只复制指针）
3. 不会占用大量栈空间

StackString 在栈上的适用场景：
1. 嵌入式系统（无堆分配器）
2. 性能关键路径（避免堆分配）
3. 编译期已知最大长度的场景

#### UTF-8 处理说明

上面的 `push_ascii` 简化了实现——它只接受 `u8` 即单个字节。正确支持 Unicode 的 `push` 方法需要处理 char 到 UTF-8 的编码：

```rust
fn push(&mut self, ch: char) {
    let mut buf = [0u8; 4];
    let encoded = ch.encode_utf8(&mut buf);
    // encoded.len() 可能是 1~4 bytes
    // ... 追加到 self.data 并更新 self.len
}
```

#### 常见错误

- 忘记处理 UTF-8 边界——直接在任意字节索引处截断可能导致 panic
- 以为 StackString 比 String 的 size_of 小——实际上 StackString<N> 的 size 是 N + 8（+ 对齐），大 N 时比 String 大得多
- 在不适合的场景使用 StackString——如需要动态增长或传递大字符串时

#### 验证方式

```bash
cargo run
# 观察 size_of 比较和 StackString 的创建/drop 输出
```

---

## 迁移思维练习答案

### 1. 如何将一段 C 的 malloc/free 代码改为 Rust 的 RAII 设计？

在 C 中用 malloc/free 管理的内存，在 Rust 中应使用拥有所有权的类型（String、Vec、Box）替代。这些类型在离开作用域时自动调用 Drop，无需手动释放。关键是让数据的所有者明确：谁创建谁负责，作用域结束时自动清理。对于文件、锁等资源，同样使用实现了 Drop 的类型（File、MutexGuard 等），不需要 C 风格的显式 close/unlock。

**Rust Drop 如何消除"忘记释放"的风险：**

```c
// C 版本: 每个 return 路径都要手动 free
void c_process() {
    char *buf = malloc(1024);
    if (!buf) return;
    
    if (error_condition_1) {
        free(buf);  // 必须记住!
        return;
    }
    
    // ... 处理 ...
    
    if (error_condition_2) {
        free(buf);  // 又必须记住!
        return;
    }
    
    free(buf);  // 正常路径也要记住!
}
```

```rust
// Rust 版本: Drop 自动处理所有退出路径
fn rust_process() {
    let buf = vec![0u8; 1024];  // 等价于 malloc(1024)
    
    if error_condition_1() {
        return;  // buf 自动 Drop, 堆内存自动释放!
    }
    
    // ... 处理 ...
    
    if error_condition_2() {
        return;  // buf 自动 Drop!
    }
    
    // buf 在此自动 Drop
}
// 即使 panic 发生, 栈展开也会调用 Drop
```

### 2. 哪些资源管理场景在 Python 中被隐藏了，但在 C/Rust 中需要显式处理？

文件句柄的生命周期在 Python 中虽然提供了 `with` 语句，但它实际上是可选的，忘记关闭时依赖 GC 延迟释放可能导致文件锁不释放或缓冲区未刷新。大块内存的分配与释放时机在 Python 中由 GC 控制，不可预测——一个引用计数环路可能让内存长期驻留。锁的获取与释放在 Python 中由于 GIL 的存在，很多场景根本不需要锁；但在 Rust 的多线程中，锁的生命周期和获取/释放时机必须精确定义。网络连接的超时与关闭在 Python 中常被忽略，而在 Rust 中连接对象的 Drop 实现决定了资源何时归还。

### 3. RAII 与 Python 的 `__del__` 或 `with` 语句有什么本质不同？

Python 的 `__del__` 由 GC 调用，调用时机不确定（可能永不调用），且不能依赖它做关键资源释放。`with` 语句需要调用者显式使用，如果忘记就用不了。Rust 的 RAII 是语言级别的保障：值离开作用域时 Drop 必定被调用，无需调用者做任何额外动作。这是"靠纪律"和"靠编译器"的区别。

| 机制 | 调用时机 | 确定性 | 需要调用者配合 |
|------|---------|--------|---------------|
| Python `__del__` | GC 时 | 不确定 | 否 |
| Python `with` | 退出 with 块 | 确定 | 是（必须用 with） |
| C `free()` | 程序员调用时 | 确定 | 是（手动调用） |
| C++ RAII | 离开作用域 | 确定 | 否（自动） |
| Rust RAII | 离开作用域 | 确定 | 否（自动，编译器强制） |

---

> 练习 1-5 为入门基础，练习 6-10 为进阶挑战。建议按顺序完成，每个练习都动手写代码并观察输出/编译错误。
