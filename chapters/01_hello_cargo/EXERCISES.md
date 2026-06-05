# 第 1 章练习: Hello Cargo

## 练习说明

- **Level 1** — 基础操作，熟悉 Cargo 命令和工作流。建议所有读者完成。
- **Level 2** — 加深理解，需要修改代码或理解构建系统细节。
- **Level 3** — 综合挑战，要求独立完成一个小型实践任务。
- **思考题** — 不需要写代码，但需要理解概念后才能回答。

请确保在练习开始前已经成功运行过 `cargo run` 并看到了完整的程序输出。

---

## Level 1: 基础操作 (3 题)

### L1-1: 尝试所有 Cargo 命令

依次执行以下命令，观察每个命令的输出:

```bash
# 1. 仅检查代码 (不生成二进制)
cargo check

# 2. Debug 构建
cargo build

# 3. 运行
cargo run

# 4. Release 构建
cargo build --release

# 5. 格式化检查
cargo fmt -- --check

# 6. 格式化代码
cargo fmt

# 7. 运行 clippy
cargo clippy

# 8. 清理
cargo clean

# 9. 重新构建 (观察速度)
cargo build
```

**目标**: 理解每个命令的作用时机和输出差异。特别注意 `cargo check` 和 `cargo build` 的执行时间差异——`cargo check` 通常快得多，因为它跳过代码生成 (codegen) 步骤。

**记录**: 把每个命令执行后你观察到的结果写下来。例如 `cargo clean` 之后 `target/` 目录是否还存在？第二次 `cargo build` 比第一次快多少？

---

### L1-2: 观察 stdout 和 stderr

本章程序同时使用了 `println!` (stdout) 和 `eprintln!` (stderr)。运行以下命令并记录差异:

```bash
# 1. 正常输出 (stdout + stderr 都显示)
cargo run

# 2. 丢弃 stderr，只看 stdout
cargo run 2>/dev/null

# 3. 丢弃 stdout，只看 stderr
cargo run 1>/dev/null

# 4. 将 stdout 写入文件，stderr 仍然显示在终端
cargo run > output.txt

# 5. 查看文件内容
cat output.txt
```

**问题**: 哪些行出现在 `output.txt` 中？哪些没有？为什么？

**扩展**: 如果你想将 stdout 和 stderr 分别保存到不同文件，应该怎么写命令？

```bash
cargo run > stdout.txt 2> stderr.txt
```

---

### L1-3: 查看编译产物的体积

```bash
# Debug 构建
cargo build
ls -lh target/debug/hello_cargo
file target/debug/hello_cargo

# Release 构建
cargo build --release
ls -lh target/release/hello_cargo
file target/release/hello_cargo

# 使用 strip 进一步缩小体积 (可选)
strip target/release/hello_cargo
ls -lh target/release/hello_cargo
```

**记录**: Debug 可执行文件的大小是多少？Release 的是多少？相差多少倍？`file` 命令显示两者有什么不同 (是否包含 "debug_info"、"not stripped" 等标记)？

**解释**: 为什么 Release 构建的二进制更小？提示: 阅读 README.md 中 "Debug vs Release 构建的区别" 一节。

---

## Level 2: 加深理解 (2 题)

### L2-1: 修改程序并添加自己的信息

在 `src/main.rs` 的 `main()` 函数末尾，添加一个新的部分，输出你自己的个性化信息:

```rust
println!("═══════════════════════════════════════════");
println!("  我的练习笔记");
println!("═══════════════════════════════════════════");
println!("  姓名: [你的名字]");
println!("  学习日期: [今天日期]");
println!("  Rust 版本: 运行 rustc --version 获取");
println!();
```

然后:
1. 运行 `cargo fmt` 确保格式正确
2. 运行 `cargo clippy` 检查是否有建议
3. 运行 `cargo run` 查看输出
4. 故意引入一个格式问题 (比如删除某行的分号)，运行 `cargo check` 看错误信息，然后修复

**目标**: 体验 "修改 → 检查 → 格式化 → lint → 运行" 的完整开发循环。

---

### L2-2: 理解 const fn 和编译期计算

本章程序的 `compute_at_compile_time()` 函数在编译时执行。你需要完成以下任务:

1. **验证它是编译期计算的**: 在 `main()` 中添加以下代码:

```rust
// 尝试在运行时修改 THE_ANSWER — 这行代码会编译失败
// THE_ANSWER = 100;  // 取消注释看错误信息
```

然后取消注释，运行 `cargo check`，阅读并理解错误信息。理解后把该行注释回去。

2. **添加自己的 const fn**: 在 `src/main.rs` 中 `compute_at_compile_time()` 旁边编写一个新的 `const fn`:

```rust
/// Computes the factorial of a small number at compile time.
/// Demonstrates that const fn can use loops.
const fn const_factorial(n: u32) -> u64 {
    let mut result: u64 = 1;
    let mut i: u32 = 1;
    while i <= n {
        result = result * (i as u64);
        i += 1;
    }
    result
}

/// Another compile-time constant using our new const fn
const FACT_5: u64 = const_factorial(5);
```

然后在 `main()` 中打印 `FACT_5` 的值 (预期: 120)，并添加一行输出说明它是编译期计算的。

3. **运行测试**:
```bash
cargo check
cargo fmt
cargo clippy
cargo run
```

**扩展思考**: 如果 `n` 很大 (比如 100)，`const_factorial` 会发生什么？试试看——编译器会怎么做？(提示: 编译器对 const fn 有执行步数限制，防止编译无限循环。)

---

## Level 3: 综合挑战 (1 题)

### L3-1: 创建自己的 Cargo 项目

不使用 `cargo new`，**手动创建**一个全新的 Cargo 项目 `my_first_project`，要求:

**步骤 1**: 创建目录结构

```bash
mkdir -p my_first_project/src
cd my_first_project
```

**步骤 2**: 手动编写 `Cargo.toml`

要求: 
- 包名为 `my_first_project`
- 版本 `0.1.0`
- Edition `"2024"`
- 描述包含你自己的话
- 添加一个 `license` 字段，值为 `"MIT"`

**步骤 3**: 手动编写 `src/main.rs`

要求程序至少包含:
- 一个 `const` 常量 (如你的名字)
- 一个 `const fn` 函数
- 同时使用 `println!` 和 `eprintln!`
- 打印 "这是我的第一个 Rust 项目！"
- 输出当前项目的包名 (硬编码即可，不需要读取 Cargo.toml)
- 有至少两个 `///` doc comment

**步骤 4**: 验证一切正常

```bash
cargo check    # 先检查
cargo fmt      # 格式化
cargo clippy   # lint
cargo build    # 构建
cargo run      # 运行
cargo build --release   # 发布构建
ls -lh target/release/my_first_project   # 查看产物
```

**步骤 5**: 对比两个项目

把 `hello_cargo/` (本章示例) 和 `my_first_project/` 放在一起，比较:

```bash
diff <(cd hello_cargo && cargo tree 2>/dev/null || echo "no deps") \
     <(cd my_first_project && cargo tree 2>/dev/null || echo "no deps")
```

两个项目有什么共同点？有什么不同？

**目标**: 脱离 `cargo new` 脚手架，理解一个 Cargo 项目的最简组成部分就是一个 `Cargo.toml` + `src/main.rs`。

---

## 思考题

### Q1: 为什么 Rust 需要 Cargo，而 Python 可以一直用 `python script.py` 运行？

请从以下几个方面思考并写下你的答案:

1. **编译型 vs 解释型**: Rust 是编译型语言 (AOT / Ahead-Of-Time Compilation)，Python 是解释型语言 (虽然 CPython 有字节码编译步骤，但通常被视为解释型)。编译型语言为什么需要一套更复杂的构建工具链？
2. **依赖管理**: Python 的 `pip install` 将依赖安装到全局或虚拟环境的 `site-packages/` 目录。Cargo 如何处理依赖？为什么 Rust 不需要虚拟环境 (venv)？
3. **版本管理**: `rustup` 可以管理多个 Rust 工具链版本。Python 生态中哪些工具承担了类似角色 (`pyenv`, `conda` 等)？为什么 Rust 把版本管理 (`rustup`) 和包管理 (`cargo`) 分开？
4. **发布产物**: Python 发布 `.py` 源码或 `.whl` 包，Rust 发布二进制可执行文件。这两种发布模型各有什么优缺点？
5. **生态统一性**: Cargo 是 Rust 事实上的唯一构建系统，几乎每个 Rust 项目都用 Cargo。Python 生态有 `pip`、`poetry`、`pipenv`、`uv`、`conda`、`PDM` 等多种选择。统一 vs 多样各有什么利弊？

把你的答案写下来，与同学或 AI 讨论。这个思考题没有标准答案，但理解这些问题能帮助你在 Rust 和 Python 之间建立更深层的心智连接。

---

## 建议尝试的命令汇总

以下命令列表可以作为日常 Rust 开发的**肌肉记忆训练**:

```bash
# === 项目创建 ===
cargo new my_project          # 创建新的 binary 项目
cargo new --lib my_lib        # 创建新的 library 项目

# === 日常开发循环 ===
cargo check                   # 快速检查 (建议每写完一个函数就跑一次)
cargo fmt                     # 格式化
cargo clippy                  # lint 检查
cargo build                   # 编译
cargo run                     # 运行

# === 测试 ===
cargo test                    # 运行所有测试
cargo test -- --nocapture     # 运行测试并显示 println 输出

# === 构建产物对比 ===
cargo build                   # → target/debug/
cargo build --release         # → target/release/
ls -lh target/debug/ target/release/

# === 清理 ===
cargo clean                   # 删除 target/ 目录

# === 文档 ===
cargo doc --open              # 生成并打开项目文档 (含依赖)

# === 深入调试 ===
cargo build --verbose         # 显示 cargo 实际调用的 rustc 命令
cargo tree                    # 显示依赖树
cargo metadata --format-version=1 | python3 -m json.tool  # 项目元数据

# === 工具链管理 (rustup) ===
rustc --version               # rustc 版本
cargo --version               # cargo 版本
rustup show                   # 当前工具链详情
rustup update                 # 更新工具链
rustup component list --installed  # 已安装组件
rustup doc --std              # 打开本地标准库文档
```

---

## 练习检查清单

在进入下一章之前，确认你能做到以下每一项:

- [ ] 能解释 `rustc`、`cargo`、`rustup` 各自的职责
- [ ] 知道 `Cargo.toml` 中 `[package]` 段每个字段的含义
- [ ] 能说出 Debug 和 Release 构建的至少三个区别
- [ ] 知道 `cargo check` 和 `cargo build` 的区别
- [ ] 理解 `println!` vs `eprintln!` (stdout vs stderr)
- [ ] 能独立创建一个最小的 Cargo 项目 (不用 cargo new)
- [ ] 了解 `const fn` 的用途和限制
- [ ] 知道 `target/` 目录应该加入 `.gitignore`
- [ ] 习惯在提交前运行 `cargo fmt && cargo clippy`
- [ ] 能够用 Python 术语向 Python 开发者解释 Cargo 生态

全部打勾后，恭喜——你已经做好了进入 Rust 类型系统和所有权世界的准备！

---

*练习完这些题目后，建议回顾 README.md 的"本章小结"部分，确认所有知识点都已掌握。*
