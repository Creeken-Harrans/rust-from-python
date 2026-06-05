# Python → Rust 概念对照表

本文档为具有 Python 编程基础的学习者提供系统化的概念对照。

**重要提示**: 
- 本对照表旨在帮助理解，不是精确等价关系。每个相似项都有重要差异。
- Rust 概念的学习不应停留在"Python 中的 X 等价于 Rust 中的 Y"层面。
- 理解 Rust 的设计动机比记住对照关系更重要。

---

## 1. 变量与类型

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| 变量赋值 `x = 5` | 变量绑定 `let x = 5;` | **Rust 默认不可变**。标签可以被重新绑定（遮蔽/Shadowing）。 |
| 动态类型 | 静态类型 + 类型推断 | Rust 在编译期确定类型，但大多数时候无需手动标注。 |
| `type(x)` | `std::mem::size_of::<T>()` | Rust 不提供运行时的"获取任意值类型"能力。 |
| `isinstance(x, int)` | 编译期 trait bound | 类型检查在编译时完成。 |
| `x += 1` | `x += 1;`（需要 `x` 为 `mut`） | Rust 中 `+=` 需要变量声明为 `mut`。 |
| 变量名只是标签 | 变量绑定具有所有权（Ownership）语义 | 赋值可能意味着 Move（移动所有权）而不仅仅是重新绑定名称。 |

### 关于"默认不可变"

```python
# Python: 变量可以随意重新赋值
x = 5
x = "hello"  # 完全合法
```

```rust
// Rust: 函数内部的变量默认不可变，但可以用 let 重新绑定
let x = 5;
// x = 6;  // 编译错误！
let x = "hello";  // 遮蔽（Shadowing）：创建新绑定，隐藏旧的
let mut y = 5;
y = 6;  // OK: mut 声明允许修改
```

**关键洞察**: Python 的变量名是可以随意重新绑定的标签；Rust 的 `let` 绑定在不可变时是强约束。遮蔽（`let x = ...`）和修改（`x = ...` 需要 `mut`）是两个不同概念——遮蔽可以改变类型，修改不能。

---

## 2. 数据类型

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| `int` | `i32`（默认）、`i64`、`u32` 等 | Rust 整数有固定大小和符号；Python int 是任意精度。 |
| `float` | `f64`（默认）、`f32` | Rust 浮点数遵循 IEEE 754，与 Python float 类似但有精度限制。 |
| `bool` | `bool` | 基本相同，但 Rust 不做隐式真值转换。 |
| `str` | `&str`（字符串切片）和 `String`（拥有所有权的字符串） | **这是新手最易混淆的地方**。&str 是借用（不可变引用），String 拥有数据。 |
| `bytes` | `&[u8]` / `Vec<u8>` | 类似切片与拥有所有权的关系。 |
| `None` | `Option::None` | Rust 没有 null。None 是 `Option<T>` 枚举的一个变体，编译器强制处理。 |
| `list` | `Vec<T>` | **元素类型必须一致**。索引越界会 panic。 |
| `dict` | `HashMap<K, V>` | **键和值有具体类型**。没有字面量语法。 |
| `tuple` | `(T1, T2, ...)` | 大小固定，元素类型在编译期确定。 |
| `set` | `HashSet<T>` | 类似 HashMap，但只有键。 |
| `range(10)` | `0..10` | 都是惰性的。Rust 的 Range 是迭代器适配对象。 |

### 字符串：`String` vs `&str`

这是 Python 学习者最容易困惑的地方：

```python
# Python: 所有字符串行为基本一致
s = "hello"
t = s          # 复制引用
u = s.upper()  # 创建新字符串
```

```rust
// Rust: 区分所有权和借用
let s = String::from("hello");  // 拥有所有权的字符串（在堆上）
let slice: &str = &s;           // 借用：指向 s 的视图
let literal: &str = "hello";    // 字面量直接是 &str（编译期嵌入二进制）

// 函数选择：
fn read_only(text: &str) { ... }     // 只读文本 → 用 &str
fn modify_and_keep(text: String) { ... }  // 需要拥有/修改 → 用 String
```

**为什么 Rust 这样设计？** 因为它要精确控制内存分配。每次创建新字符串都意味着堆分配，而 `&str` 只是借用视图，无分配。

---

## 3. 复合数据类型

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| `class` | `struct` + `impl` | 结构体定义数据，`impl` 定义方法。分离数据和行为的关注点不同。 |
| `dataclass` | `struct` | Rust struct 天然就是数据容器，不需要装饰器。 |
| `@property` | 方法 + 命名约定 | Rust 无属性访问器。通常用 `get_xxx()` 方法。 |
| `__init__` | 构造函数通常命名为 `new()`（约定） | Rust 没有专门的构造函数语法，关联函数 `new()` 是社区约定。 |
| `self` | `&self`、`&mut self`、`self` | **必须显式选择所有权级别**——这反映了 Rust 的核心设计哲学。 |
| Duck typing（鸭子类型） | Trait + Generics | 编译期检查，不是运行时。 |
| Abstract Base Class | Trait | Rust trait 不能包含字段，但可以有默认方法。 |
| `__str__` / `__repr__` | `Display` / `Debug` trait | 通过派生宏 `#[derive(Debug)]` 可自动实现 Debug。 |
| 继承 | 无经典继承 | Rust 通过 Trait、组合和泛型实现代码复用。 |
| Mixin | Trait 默认实现 | Trait 可以有带默认实现的方法。 |

### `self` 参数：Python vs Rust

```python
class Counter:
    def __init__(self):
        self.count = 0
    
    def increment(self):      # self 是约定，总是引用
        self.count += 1
```

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {          // 没有 self → 关联函数（类似 @classmethod）
        Counter { count: 0 }
    }
    
    fn increment(&mut self) {   // &mut self → 可变借用（需要修改 self）
        self.count += 1;
    }
    
    fn value(&self) -> i32 {    // &self → 不可变借用（只读）
        self.count
    }
    
    fn consume(self) -> i32 {   // self → 获取所有权（消耗 self）
        self.count
    }
}
```

---

## 4. 函数与控制流

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| `def f(x):` | `fn f(x: i32) -> i32 { }` | Rust 必须标注参数和返回值类型。 |
| 无返回值函数 | 隐式返回 `()`（单元类型 Unit Type） | 类似 Python 隐式返回 `None`。 |
| `return x` | `return x;` 或结尾表达式（无分号） | 推荐使用结尾表达式风格。 |
| 默认参数 | 不支持 | 通常使用 Builder 模式或 `Option<T>` 参数。 |
| 可变参数 `*args` | 宏（Macro） | `println!` 能接受可变参数正是因为它是一个宏。 |
| `lambda x: x + 1` | `|x| x + 1` | Rust 闭包的捕获规则更精确（Fn/FnMut/FnOnce）。 |
| `if condition:` | `if condition { }` | Rust 的 if 是表达式，可以返回值。 |
| `x if c else y` | `if c { x } else { y }` | Rust 不需要三元运算符，if 就是表达式。 |
| `for item in iterable:` | `for item in &collection { }` | 需要注意迭代方式（iter/iter_mut/into_iter）。 |
| `while condition:` | `while condition { }` | 基本相同。 |
| `try/except` | `match` 或 `?` 运算符 + `Result<T, E>` | **这不是语法替换，而是整个错误处理哲学的差异。** |
| 异常传播 | `?` 运算符 | 自动将错误向上传播。 |
| `raise Exception(...)` | `panic!(...)`（不可恢复）或 `return Err(...)`（可恢复） | Rust 倾向将错误编码为返回值。 |

---

## 5. 错误处理

这是 Python 和 Rust 最大的哲学差异之一：

```python
# Python: 异常机制
def read_config(path):
    try:
        with open(path) as f:
            return json.load(f)
    except FileNotFoundError:
        return {}
    except json.JSONDecodeError as e:
        raise ConfigError(f"Invalid config: {e}")
```

```rust
// Rust: Result 类型
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;  // ? 传播 I/O 错误
    let config = serde_json::from_str(&content)?;  // ? 传播解析错误
    Ok(config)
}
```

**核心差异**:
- Python 的异常在调用栈中向上冒泡，调用者可能完全不知道会有什么异常。
- Rust 的 `Result<T, E>` 将失败可能性编码进类型签名——调用者不能忽略。
- `?` 运算符比 `try/except` 更轻量，但只能向上传播，不能在中间做复杂恢复。

---

## 6. 模块与工程结构

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| `.py` 文件 | `mod` 模块 | Rust 模块需要显式声明（`mod xxx;`），不是自动从文件名推断。 |
| `import xxx` | `use xxx;` | Rust 的 use 创建路径别名，mod 声明模块存在。 |
| `from x import y` | `use x::y;` | 类似。 |
| `__init__.py` | `mod.rs` 或同名文件 | Rust 2018+ 推荐使用 `模块名.rs` 放在父目录。 |
| `setup.py` / `pyproject.toml` | `Cargo.toml` | Cargo 同时负责构建、依赖、发布。 |
| `pip install` | 在 `Cargo.toml` 添加依赖 | Cargo 自动下载和构建。 |
| `venv` / `virtualenv` | 不需要 | Cargo 的依赖解析和 `Cargo.lock` 提供了可重现的构建。 |
| `requirements.txt` | `Cargo.toml [dependencies]` | Cargo.toml 同时声明依赖和版本约束。 |

---

## 7. 并发与异步

| Python 概念 | Rust 中最接近的概念 | 关键差异 |
|------------|-------------------|---------|
| `threading.Thread` | `std::thread::spawn` | Rust 线程在类型系统层面防止数据竞争。 |
| `queue.Queue` | `std::sync::mpsc::channel` | Rust Channel 的类型安全保证。 |
| `multiprocessing.Lock` | `std::sync::Mutex<T>` | Rust 的 Mutex 包装数据——不能忘记加锁就访问。 |
| `asyncio` | Tokio（第三方） + `Future` | Rust 标准库只定义 `Future` trait，不提供 Runtime。需要第三方（如 Tokio）。 |
| `async def` | `async fn` | 语法相似。Rust 需要 Runtime 执行 Future。 |
| `await` | `.await` | 后缀语法而非 Python 的前缀语法。 |
| GIL | 无 GIL | Rust 可以实现真正的 CPU 并行。 |

---

## 8. 内存管理

| Python 行为 | Rust 行为 | 说明 |
|------------|----------|------|
| 引用计数 + 循环检测 GC | 编译期所有权检查 | Rust 无 GC，无引用计数开销。 |
| 对象生命周期由 GC 决定 | 由作用域和所有权决定 | Rust 的释放时机是确定的。 |
| `del x` | 值离开作用域自动释放 | 可提前调用 `drop(x)` 或让值离开作用域。 |
| `copy.copy()` | 取决于是否实现 `Copy` / `Clone` | 有些类型自动 Copy，有些需要显式 Clone。 |
| `weakref.ref` | `Weak<T>` | 用于避免引用循环。 |

---

## 9. 常见思维陷阱

### 陷阱 1: "Rust 的 String 就是 Python 的 str"

**真相**: `String` 是有所有权的、可变的字符串缓冲区；`&str` 是不可变的视图引用。Python 的 `str` 是不可变的、但通过引用管理——没有直接对应的 Rust 概念。

### 陷阱 2: "Option 就是换了个名字的 None"

**真相**: Python 中 None 可以出现在任何地方且常被忽略。Rust 的 `Option<T>` 不是 `T`——编译器强制你处理 None 的情况，你不能"不小心"把 Option 当值用。

### 陷阱 3: "Result 和异常差不多"

**真相**: 异常不改变函数签名，调用者可以完全忽视异常直到运行时崩溃。`Result` 是类型的一部分——编译器知道你可能会失败，并迫使你处理。

### 陷阱 4: "move 就是深拷贝"

**真相**: Move 是所有权转移，不是数据复制。String 的 Move 只复制了栈上的 {指针, 长度, 容量} 三个字段（~24 字节），堆上的实际字符串数据没有复制。

### 陷阱 5: "生命周期会让变量活得更久"

**真相**: 生命周期标注 (`'a`) 只是描述引用之间的关系，帮助编译器验证引用有效性。它不会改变任何值的实际存活时间。

### 陷阱 6: "借用检查器是我的敌人"

**真相**: 借用检查器保护你免于内存错误和数据竞争。每个被拒绝的代码都对应着一个真实的安全隐患。理解它的规则后，它会成为你最好的工具而非障碍。

### 陷阱 7: "只要编译通过了就没 Bug"

**真相**: Rust 编译器保证内存安全和线程安全，但无法防止逻辑错误、算法错误、业务规则错误等。测试仍然是必要的。

---

## 10. 学习路径建议

如果你是 Python 开发者，建议按以下顺序适应 Rust：

1. **接受显式**: 从 Python 的"隐式便利"转向 Rust 的"显式控制"
2. **先不要抗拒**: 借用检查器的规则感觉受限是正常的，先遵守再理解
3. **信任编译器**: Rust 的错误信息是你的朋友，仔细阅读
4. **多写多改**: 通过写代码建立所有权直觉
5. **重新理解"成本"**: Rust 让你意识到你以前从未注意过的内存分配
6. **用 cargo check 迭代**: 不要每次都完整编译
