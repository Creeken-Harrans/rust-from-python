# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：识别 Workspace 结构

Root `Cargo.toml`：
```toml
[workspace]
members = ["chapters/*", "projects/*"]
resolver = "3"
```

- `members`：glob 模式，匹配所有子目录
- `resolver = "3"`：使用 2024 edition 的解析器
- 根 Package 是 Virtual Workspace（无 `[package]`，仅 `[workspace]`）

---

### 1-2：添加新成员

```toml
[workspace]
members = ["chapters/*", "projects/*", "tools/*"]
```

添加目录后运行 `cargo check --workspace` 验证新成员被识别。

---

### 1-3：共享依赖版本

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = "1"

# 子 crate 中：
# [dependencies]
# serde = { workspace = true }
# tokio = { workspace = true }
```

优势：版本号集中管理，所有子 crate 使用一致版本。

---

## Level 2：组合应用

### 2-1：互 crate 依赖

```toml
# utilities/Cargo.toml  (library crate)
[package]
name = "utilities"

# main_app/Cargo.toml  (binary crate)
[dependencies]
utilities = { path = "../utilities" }
```

`path` 依赖在 Cargo.lock 中跟踪版本，发布 crates.io 时需要改为版本依赖。

---

### 2-2：Profile 继承

```toml
# Root Cargo.toml
[workspace.package]
edition = "2024"

# 不推荐在子 crate 中独立配置 profile —— Cargo 会警告忽略
# 统一在 workspace root 配置
```

---

## Level 3：设计思考

### 3-1：何时拆分为 Workspace？

**适合拆分**：
- 多个独立二进制 + 共享库
- 编译时间过长，需要增量编译
- 需要独立发布不同组件

**不适合拆分**：
- 小型项目（< 10k 行）
- 组件紧密耦合，频繁交叉修改
- 团队规模小，管理多个 crate 开销大于收益

### 3-2：resolver = "3" 的作用

- 不再为开发依赖单独解析依赖图（统一解析）
- 避免 dev-dependencies 导致的不同特性组合冲突
- edition 2024 默认

---

## 迁移思维练习

### C++ CMake 与 Cargo Workspace

| 概念 | CMake | Cargo Workspace |
|------|-------|----------------|
| 子项目 | `add_subdirectory()` | `members = [...]` |
| 共享配置 | CMakeLists.txt 变量 | `[workspace.package]` |
| 依赖管理 | `find_package` / `FetchContent` | `path = "..."` |
| 构建缓存 | 独立 build/ 目录 | 共享 target/ |

**迁移提示**：C++ 依赖管理是分散的（CMake + vcpkg + conan + ...），Cargo Workspace 提供了集成化的解决方案。

---

*Workspace 是多 crate 项目的骨架。`resolver = "3"` + 集中依赖版本管理 + path 依赖是标准实践。*
