# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：Clippy 练习

```bash
# 运行所有 lint
cargo clippy --all-targets --all-features

# 以 error 级别处理所有 warning
cargo clippy -- -D warnings

# 针对特定 lint
cargo clippy -- -W clippy::unwrap_used
```

故意写出 `unwrap()` 在错误处理路径上，观察 Clippy 的警告信息和建议。

#### 常见 lint 触发场景

```rust
// clippy::unwrap_used
let x = some_option.unwrap(); // 警告：建议用 match/expect

// clippy::clone_on_copy
let y = 42.clone(); // 警告：i32 是 Copy，不需要 clone

// clippy::needless_return
return result; // 警告：移除 return，直接用尾部表达式
```

---

### 1-2：rustfmt 配置

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
edition = "2024"
```

重要规则：
- `cargo fmt -- --check` 在 CI 中检查格式
- `cargo fmt --all` 递归格式化 workspace 中所有 crate
- 团队协作时统一 rustfmt 配置避免格式冲突

---

### 1-3：文档检查

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

常见文档警告：
- 文档中的代码块语法错误
- 文档引用不存在的项
- 文档中有未闭合的 HTML 标签

---

## Level 2：组合应用

### 2-1：CI 脚本设计

```yaml
# .github/workflows/ci.yml 关键步骤
- cargo fmt --all -- --check
- cargo check --workspace --all-targets
- cargo test --workspace
- cargo clippy --workspace --all-targets -- -D warnings
- RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

#### 为什么这个顺序？

1. `fmt`：最快，先检查风格
2. `check`：检查类型但不运行测试（快）
3. `test`：计算最重，放后面
4. `clippy` + `doc`：补充检查

### 2-2：`#[allow]` 适当使用

```rust
// 全局允许（应用于整个 crate）
#![allow(clippy::vec_init_then_push)] // 教学目的

// 局部允许（应用于单个项）
#[allow(clippy::approx_constant)]
const PI: f64 = 3.14159;
```

原则：用 `#[allow]` 标注有意违反 lint 的位置并注释原因，不要全局关闭 lint。

---

## Level 3：设计思考

### 3-1：lint 配置策略

| 场景 | 推荐 |
|------|------|
| 新项目 | 使用默认 lint，只允许特定例外 |
| 已有项目 | 逐步开启，`-W` 新规则，`-A` 暂时放行的旧问题 |
| 教学项目 | 允许教学目的的模式（`unwrap`、`vec!` 内部实现等） |
| 生产项目 | `-D warnings` 作为 CI 门禁 |

### 3-2：CI 设计的工程意义

- **确定性**：所有检查在任何机器上结果一致
- **渐进性**：从快检查到慢检查，节省 CI 时间
- **可重复**：`./scripts/check_all.sh` 在本地复现 CI 环境
- **不依赖外部服务**：除文档构建外，不依赖网络

---

## 迁移思维练习

### Python lint 与 Rust lint

| 工具 | Python | Rust |
|------|--------|------|
| 代码风格 | `black` / `ruff format` | `rustfmt`（官方，零配置） |
| 静态分析 | `ruff` / `pylint` / `mypy` | `clippy`（官方，550+ lints） |
| 文档检查 | `pydocstyle` | `cargo doc` + `-D warnings` |
| 类型检查 | `mypy`（可选，外部工具） | rustc（编译期强制） |

Rust 的 lint、format、doc 都内建于工具链，无需选择第三方工具，降低了项目配置的决策成本。

---

*Lint、格式化和 CI 是工程成熟度的标志，不是可有可无的附加品。建立 CI 门禁的习惯越早越好。*
