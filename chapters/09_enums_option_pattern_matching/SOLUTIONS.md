# 迁移思维练习答案 — 枚举、Option 与模式匹配

## 迁移思维练习答案

### 1. C 中用多个 boolean 字段表达状态的代码，如何改为 Rust Enum？

将分散的 boolean 字段替换为一个 Enum，每个变体代表一个有效状态并携带该状态特有的数据。例如，将 `is_connected: bool, is_connecting: bool, peer: char*, attempt: int` 改为 `enum ConnectionState { Disconnected, Connecting { attempt: u32 }, Connected { peer: String } }`。这样做的好处是：编译器确保你永远不会在 Disconnected 状态下错误地读取 peer 字段——peer 只在 Connected 变体中存在，必须通过模式匹配解构才能访问。状态和数据之间的约束从"程序员必须记住的规矩"变成了"编译器强制执行的类型规则"。

### 2. Python 中返回 None 表示"没找到"的模式，如何改为 Option<T>？

Python 函数返回 None 时，调用者可能忘记检查而导致运行时 AttributeError（如 `result.do_something()` 报错 `'NoneType' object has no attribute 'do_something'`）。Rust 的 Option<T> 是独立的类型，与 T 类型不兼容——你不能把 Option<T> 当作 T 使用。编译器强制调用者通过 match、if let、unwrap 等方式显式处理 None 情况。这是从"运行时的君子协定"到"编译期强制验证"的根本转变。

### 3. 为什么 Rust 的 Result 和 Option 不需要 Python 那样的异常处理？

Result 和 Option 都是普通类型，不是异常机制。它们通过类型签名在函数接口层面就表达了"可能失败"或"可能为空"的语义，调用者的代码中处理这些情况是常规的控制流（match/if let），不需要 try/except 那样的特殊语法结构。这种方式让错误处理路径和正常路径在同一层控制流中可见，不像异常可以跨越多层调用栈——这提高了代码的可读性和可预测性。
