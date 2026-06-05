# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：依赖声明

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", optional = true }
rand = "0.8"
```

`^1.2.3`（脱字符默认）等价于 `>=1.2.3, <2.0.0`，遵循 SemVer 兼容规则。

---

### 1-2：Feature 定义

```toml
[features]
default = ["std"]
std = []
json = ["serde", "serde_json"]
full = ["json", "async"]
async = ["tokio"]
```

Feature 是编译期选项（不是运行时开关），是加性的（additive）——启用 feature 只会增加代码，不会移除。不应设计互斥的 feature。

---

### 1-3：Profile 配置

```toml
[profile.release]
opt-level = 3      # 最大优化
lto = true         # 链接时优化
codegen-units = 1  # 更好的优化但编译更慢
panic = "abort"    # 减小二进制体积

[profile.dev]
opt-level = 0      # 快速编译
debug = true
```

---

## Level 2：组合应用

### 2-1：条件编译

```rust
#[cfg(feature = "json")]
fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
}

#[cfg(not(feature = "json"))]
fn to_json<T>(_data: &T) -> String {
    "json feature not enabled".to_string()
}
```

---

### 2-2：依赖冲突排查

```bash
cargo tree              # 查看依赖树
cargo tree -d           # 查看重复依赖
cargo tree -i <crate>   # 谁依赖了这个 crate
```

Cargo 默认允许同一 crate 的多个 SemVer 不兼容版本共存。

---

## Level 3：设计思考

### 3-1：Feature 设计原则

1. **加性**：每个 feature 添加功能，不删除功能
2. **正交**：feature 之间尽量独立
3. **默认最小化**：`default` 只包含最常用组合
4. **文档化**：doc comment 说明每个 feature 的作用

**不良设计**：互斥 feature（如 `backend-a` vs `backend-b`）-> 应改为编译期 trait 选择或 Cargo feature + `cfg`。

### 3-2：Cargo vs pip/npm

| 特性 | Cargo | pip | npm |
|------|------|-----|-----|
| 锁文件 | `Cargo.lock`（确定性构建） | `requirements.txt`（松散） | `package-lock.json` |
| 版本规范 | SemVer + `^` `~` 等 | `==`, `>=` 等 | SemVer + `^` `~` |
| 依赖解析 | 统一 SAT 求解 | 线性安装 | 嵌套 |
| features | 编译期条件编译 | extras（可选依赖） | — |

---

## 迁移思维练习

### from pyproject.toml to Cargo.toml

Python 的 `pyproject.toml` 和 Rust 的 `Cargo.toml` 名字来源于同一标准（TOML），但用途不同：

- Python `[project.optional-dependencies]` 近似 Cargo `[features]`
- Python `[tool.poetry.dependencies]` 近似 Cargo `[dependencies]`
- Python 可选依赖是运行时可选，Cargo feature 是编译期可选

**迁移提示**：Cargo 的 feature 让库的消费者"按需编译"，这是 Python 生态目前无法做到的。代价是条件编译增加了组合爆炸的测试负担。

---

*Cargo 是 Rust 工程化的核心。熟练使用依赖管理、feature 和 profile 配置，是写出高效、可维护 Rust 项目的基础。*
