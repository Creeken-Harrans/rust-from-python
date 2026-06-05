# 常用命令速查 (Commands Reference)

按用途整理的 Rust/Cargo 常用命令。

---

## 工具链管理

```bash
# 查看 rustc 版本
rustc --version

# 查看 Cargo 版本
cargo --version

# 更新 Rust 工具链
rustup update

# 查看已安装的工具链
rustup show

# 添加组件（如 rustfmt, clippy）
rustup component add rustfmt clippy

# 在浏览器中打开本地 Rust 官方书籍
rustup doc --book

# 在浏览器中打开标准库文档
rustup doc --std
```

---

## 创建项目

```bash
# 创建新的二进制项目（带 Cargo.toml 和 src/main.rs）
cargo new project_name

# 创建新的库项目（带 Cargo.toml 和 src/lib.rs）
cargo new --lib library_name

# 在当前目录初始化 Cargo 项目
cargo init

# 在当前目录初始化库项目
cargo init --lib
```

---

## 编译与运行

```bash
# 快速检查代码是否可编译（不生成二进制文件，速度最快）
cargo check

# 编译（Debug 模式，未优化）
cargo build

# 编译并运行
cargo run

# 编译 Release 模式（优化，较慢）
cargo build --release

# 运行 Release 版本
cargo run --release

# 清理构建产物
cargo clean
```

**使用时机**: 
- 日常开发迭代时用 `cargo check` 快速验证（不生成可执行文件）
- 确认代码正确后用 `cargo run` 执行
- 准备发布时用 `cargo build --release`
- `cargo clean` 清空 `target/` 目录以释放磁盘空间

---

## 格式化与静态检查

```bash
# 格式化代码
cargo fmt

# 检查代码格式是否规范（不修改代码）
cargo fmt --check

# 对所有项目检查格式
cargo fmt --all -- --check

# 运行 Clippy 静态分析
cargo clippy

# 将 Clippy 警告视为错误
cargo clippy -- -D warnings

# 对所有项目运行 Clippy（所有 target、所有 feature）
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

## 测试

```bash
# 运行所有测试
cargo test

# 运行名称匹配的测试
cargo test test_name

# 运行测试并显示标准输出（包括 println!）
cargo test -- --nocapture

# 单线程运行测试
cargo test -- --test-threads=1

# 忽略 #[ignore] 标记的测试
cargo test -- --ignored

# 运行文档测试
cargo test --doc

# 对整个 Workspace 运行测试
cargo test --workspace
```

---

## 文档

```bash
# 生成文档
cargo doc

# 生成文档并在浏览器打开
cargo doc --open

# 生成不包括依赖的文档
cargo doc --no-deps

# 对 Workspace 生成文档
cargo doc --workspace --no-deps
```

---

## 依赖管理

```bash
# 显示依赖树
cargo tree

# 显示重复依赖
cargo tree -d

# 查看项目元数据
cargo metadata --format-version 1

# 更新依赖（在 Cargo.lock 约束内）
cargo update

# 安装 Cargo 插件或二进制工具
cargo install tool_name
```

---

## Workspace 检查

```bash
# 检查整个 Workspace（所有 target）
cargo check --workspace --all-targets

# 对整个 Workspace 运行测试
cargo test --workspace

# 对整个 Workspace 格式化检查
cargo fmt --all -- --check

# 对整个 Workspace Clippy 检查
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

## 常用组合

```bash
# 日常开发快速检查
cargo check && cargo test && cargo clippy

# 提交前完整检查
cargo fmt --all -- --check && \
cargo check --workspace --all-targets && \
cargo test --workspace && \
cargo clippy --workspace --all-targets --all-features -- -D warnings && \
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

# 本教程的一键检查
./scripts/check_all.sh
```

---

## 常见问题命令

```bash
# 查看 Cargo 配置
cargo config get

# 查看更详细的编译输出
cargo build -vv

# 查看展开宏后的代码
cargo expand  # 需要安装: cargo install cargo-expand

# 分析二进制文件大小
cargo bloat  # 需要安装: cargo install cargo-bloat

# 基准测试
cargo bench  # 需要 nightly 工具链
```

---

## 编辑器集成

大多数编辑器（VS Code、IntelliJ Rust、Neovim）的 Rust 插件会在保存时自动执行：
- `cargo fmt`（格式化）
- `cargo check`（类型检查）
- `cargo clippy`（静态分析）

可以配置保存时自动执行的命令，不需要每次都手动运行。
