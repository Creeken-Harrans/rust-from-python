# 排错指南 (Troubleshooting)

本指南涵盖 Rust/Cargo 学习过程中常见的环境和编译问题。

---

## 环境问题

### `cargo: command not found`

**原因**: Rust 未安装，或 `~/.cargo/bin` 不在 `PATH` 中。

**解决**:
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或重新加载环境变量
source ~/.cargo/env

# 验证安装
cargo --version
```

**Windows 用户**: 下载并运行 [rustup-init.exe](https://rustup.rs/)，安装后重启终端。

---

### Rust 工具链版本过旧

**原因**: 长时间未更新。

**解决**:
```bash
rustup update
# 验证
rustc --version
```

---

### Linker 缺失 (`linker 'cc' not found`)

**原因**: 系统缺少 C 链接器（Linux 上通常是 `gcc` 或 `clang`）。

**解决**:

**Ubuntu/Debian**:
```bash
sudo apt install build-essential
```

**Fedora/RHEL**:
```bash
sudo dnf install gcc
```

**macOS**:
```bash
xcode-select --install
```

**Windows**: 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) 并选择"C++ build tools"。

---

### 依赖下载超时 (`timed out`)

**原因**: 网络问题，访问 crates.io 不稳定。

**尝试**:
1. 重试命令（Cargo 通常会自动重试）
2. 检查网络连接
3. 使用国内镜像源（见下方"国内镜像配置"）

**国内镜像配置**:

在 `~/.cargo/config.toml` 中（没有则创建）:
```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = 'sparse+https://mirrors.ustc.edu.cn/crates.io-index/'
```

或使用清华源:
```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = 'sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/'
```

---

### `cargo check` 与 `cargo build` 的区别

很多初学者混淆两者：

| 命令 | 生成文件 | 速度 | 用途 |
|------|---------|------|------|
| `cargo check` | 不生成 | 快 | 快速检查类型和语法错误 |
| `cargo build` | 生成二进制 | 较慢 | 产生可运行的可执行文件 |

**建议**: 开发时使用 `cargo check` 快速迭代，确认正确后再 `cargo build` 或 `cargo run`。

---

## 编译错误

### `use of moved value: ...`（使用了被移动的值）

**典型场景**:
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // ERROR: s1 已被移动到 s2
```

**原因**: `String` 不实现 `Copy`，赋值时发生移动（Move），`s1` 不再有效。

**修复方式**:

1. **使用借用**（推荐）:
```rust
let s1 = String::from("hello");
let s2 = &s1;  // s2 借用 s1
println!("{}", s1);  // 仍然有效
```

2. **显式克隆**（谨慎使用）:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝
println!("{}", s1);  // 两个都有效
```

3. **返回所有权**:
```rust
let s1 = String::from("hello");
let s1 = process_and_return(s1);  // 传入并返回所有权
```

**不应滥用 `clone()`**: 过度使用 `clone()` 会掩盖设计问题且降低性能。应优先考虑借用。

---

### `cannot borrow ... as mutable because it is also borrowed as immutable`（存在冲突借用）

**典型场景**:
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // 不可变借用
v.push(4);            // ERROR: 需要可变借用
println!("{}", first);
```

**原因**: 同一作用域内同时存在不可变借用和可变借用。

**修复**:
```rust
let mut v = vec![1, 2, 3];
let first = v[0];    // Copy 出来，不需要借用
v.push(4);
println!("{}", first);  // OK: 拥有自己的值
```

或者调整代码结构让借用不重叠。

---

### `does not live long enough`（生命周期不足）

**典型场景**:
```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s 在此函数结束时被释放
}
```

**原因**: 返回的引用指向了局部变量，引用比数据活得久——悬垂引用。

**修复**: 返回拥有所有权的值而不是引用：
```rust
fn not_dangling() -> String {
    let s = String::from("hello");
    s  // 移动所有权给调用者
}
```

---

### `expected &str, found String`（类型不匹配）

**典型场景**:
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
let name = String::from("Alice");
greet(name);  // ERROR: expected &str, found String
```

**修复**: 传入引用：
```rust
greet(&name);  // &String 自动强制转换为 &str
```

或者用切片语法:
```rust
greet(&name[..]);  // 显式转换为 &str
```

**常见选择指南**:
- 函数参数需要读取文本 → 用 `&str`（更通用）
- 函数需要拥有和修改文本 → 用 `String`（有所有权）
- 函数需要返回新文本 → 用 `String`（无法返回局部变量的引用）

---

### `the trait bound ... is not satisfied`（Trait 约束不满足）

**典型场景**:
```rust
fn print_all<T>(items: &[T]) {
    for item in items {
        println!("{}", item);  // ERROR: T 没有实现 Display
    }
}
```

**修复**: 添加 trait 约束：
```rust
use std::fmt::Display;
fn print_all<T: Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}
```

---

### `called 'unwrap()' on a 'None' value`（unwrap 引发 panic）

**原因**: 对 `None` 或 `Err` 值调用 `unwrap()`。

**不要在生产代码中这样**:
```rust
let value = maybe_value.unwrap();  // 如果是 None 则 panic
```

**改为模式匹配**:
```rust
match maybe_value {
    Some(v) => println!("Value: {}", v),
    None => println!("No value found"),
}
```

**或用 `unwrap_or_*` 系列**:
```rust
let value = maybe_value.unwrap_or("default");  // 提供默认值
let value = maybe_value.unwrap_or_else(|| compute_default());
```

---

### Clippy 警告

**常见 Clippy 警告及处理**:

| 警告 | 含义 | 处理 |
|------|------|------|
| `clippy::needless_return` | 多余的 `return` | 移除函数末尾的 `return` |
| `clippy::redundant_clone` | 不必要的 `clone()` | 检查是否确实需要克隆 |
| `clippy::unnecessary_unwrap` | 不必要的 `unwrap()` | 改用更安全的方式 |
| `clippy::comparison_to_empty` | 应使用 `is_empty()` | `x == ""` → `x.is_empty()` |
| `clippy::manual_flatten` | 应使用 `flatten()` | `if let Some(x) = opt { Some(x) } else { None }` → `opt` |

**注意**: Clippy 的建议不一定都要照做。理解警告原因后决定是否修改。

如需在特定位置忽略某条规则：
```rust
#[allow(clippy::some_lint_name)]
let x = some_code();
```

---

## Workspace 问题

### `can't find crate for ...`

**原因**: Workspace member 路径不存在或拼写错误。

**检查**:
```bash
# 查看 workspace 包含哪些 member
cargo metadata --format-version 1 | grep -A 5 '"workspace_members"'
```

**修复**: 确保 `Cargo.toml` 中 `members` 路径正确。
```toml
[workspace]
members = [
    "chapters/*",
    "projects/*",
]
```

---

### 嵌套 Workspace 问题

**症状**: 在 Workspace 内部创建了另一个包含 `[workspace]` 的 `Cargo.toml`。

**修复**: 
- 如果子 worksapce 只是演示用，将其从父 workspace 的 `members` 中排除：
```toml
[workspace]
members = ["chapters/*"]
exclude = ["chapters/26_workspace_architecture/demo_workspace"]
```
- Cargo 不支持嵌套 workspace。

---

## 版本和 Edition 问题

### `error: failed to parse manifest`

**可能原因**: 
1. `Cargo.toml` 语法错误
2. `edition` 值不支持（如拼写错误）

**正确写法**:
```toml
[package]
edition = "2024"
```

**注意**: `edition` 必须是字符串 `"2024"`，不是整数 `2024`。

---

### `feature 'edition2024' is required`

**原因**: Rust 版本过旧，不支持 Rust 2024 Edition。

**解决**:
```bash
rustup update stable
```

Rust 2024 Edition 需要 Rust 1.82+ 版本。

---

## 最佳实践：遇到错误时

1. **仔细阅读错误信息** — Rust 的错误信息通常非常详细，包含原因和建议修复。
2. **从第一行错误开始修复** — 后续错误通常由第一个错误引发。
3. **使用 `cargo check` 快速迭代** — 不必每次都完整编译。
4. **阅读相关章节** — 本教程对应章节会详细解释常见错误。
5. **参考编译器建议** — `rustc` 的 `help:` 信息经常直接给出正确写法。
6. **`rustc --explain EXXXX`** — 获取特定错误码的详细解释。例如：`rustc --explain E0382`
