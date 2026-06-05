# Cargo Workspace 工作空间架构

## 目录

1. [概述](#概述)
2. [什么是 Workspace](#什么是-workspace)
3. [虚拟 Workspace vs 根 Package Workspace](#虚拟-workspace-vs-根-package-workspace)
4. [Members 与 Exclude](#members-与-exclude)
5. [共享资源: Cargo.lock 与 target/](#共享资源-cargolock-与-target)
6. [路径依赖](#路径依赖)
7. [Resolver 版本](#resolver-版本)
8. [Workspace 级别依赖](#workspace-级别依赖)
9. [典型 Workspace 模式](#典型-workspace-模式)
10. [本教程的 Workspace 设计](#本教程的-workspace-设计)
11. [什么时候不要用 Workspace](#什么时候不要用-workspace)
12. [实战: 创建 Workspace](#实战-创建-workspace)
13. [最佳实践](#最佳实践)
14. [常见问题](#常见问题)
15. [总结](#总结)

## 概述

当 Rust 项目增长到一定规模，单个 `main.rs` 不再足够。你可能需要：
- 将核心逻辑提取到独立的库中
- 同时维护 CLI 工具和 Web 服务
- 管理一组共享通用代码的微服务
- 将 proc-macro、测试工具与主库解耦

Cargo Workspace（工作空间）就是为解决这些多 Package 项目管理问题而生的。它让你在一个 Git 仓库中管理多个相互关联的 Rust crate，共享编译缓存和依赖锁文件。

对于从 Python 转过来的开发者，可以将 Workspace 理解为 Python 的 monorepo 工具（如 poetry workspaces 或 setuptools 的 `-e` editable install）+ 共享的 `requirements.txt` + 统一的构建系统的结合体，但 Cargo 提供了更深的集成。

## 什么是 Workspace

Workspace 是 Cargo 提供的多 crate 项目组织方式。关键特征：

1. **多个 Package**: 一个 workspace 包含多个 Rust package（每个 package 是一个 crate，有自己的 `Cargo.toml`）
2. **共享 Cargo.lock**: 所有 member 使用同一个 lock 文件，确保版本一致性
3. **共享 target/**: 所有 member 的编译产物在同一目录，避免重复编译
4. **路径依赖**: Member 之间通过相对路径互相引用
5. **统一命令**: `cargo build --workspace` 一条命令构建所有 member

### Workspace 的核心价值

```
没有 Workspace:
  项目A 编译 serde → 2 分钟
  项目B 编译 serde → 2 分钟  (重复!)
  项目C 编译 serde → 2 分钟  (重复!!)
  总计: 6 分钟

有 Workspace:
  serde 编译一次 → 2 分钟
  项目A、B、C 直接复用 → 0 分钟
  总计: 2 分钟
```

这不仅仅是编译时间的节省。共享 `Cargo.lock` 还保证了所有 member 使用完全相同版本的依赖，避免了 "在我的机器上能跑" 的问题。

## 虚拟 Workspace vs 根 Package Workspace

Cargo 支持两种 Workspace 根结构：

### 虚拟 Workspace (Virtual Workspace)

根 `Cargo.toml` **没有** `[package]` 段，只包含 `[workspace]` 定义：

```toml
# 根 Cargo.toml
[workspace]
members = [
    "core",
    "cli",
    "server",
]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

特点：
- 根目录本身不产出 crate
- 纯粹的管理角色
- 适合 monorepo 结构
- 不需要担心根 crate 的依赖污染 member

### 根 Package Workspace (Root Package)

根 `Cargo.toml` 同时有 `[package]` 和 `[workspace]` 段：

```toml
# 根 Cargo.toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"

[workspace]
members = ["lib-core", "lib-utils"]

[dependencies]
# 根 crate 自身的依赖
lib-core = { path = "./lib-core" }
lib-utils = { path = "./lib-utils" }
```

特点：
- 根目录本身是一个 crate（通常是主应用）
- 子 member 作为辅助库
- 适合主应用 + 支持库的模式

### 如何选择

| 场景 | 推荐 |
|------|------|
| Monorepo 多服务 | 虚拟 Workspace |
| 主应用 + 内部库 | 根 Package Workspace |
| 教程/示例集合（如本教程） | 虚拟 Workspace |
| 多 crate 库家族 | 虚拟 Workspace |
| 嵌入式项目（bootloader + app） | 根 Package Workspace |

## Members 与 Exclude

### members 配置

`members` 字段定义了哪些子目录是 workspace 的一部分：

```toml
[workspace]
members = [
    "core",                    # 单个目录
    "services/user-service",   # 嵌套路径
    "plugins/*",               # glob: plugins 下的所有直接子目录
    "libs/network-*",          # glob: 匹配模式
    "examples/?",              # glob: 单字符匹配
]
```

Cargo 的 members glob 支持以下模式：
- `*` 匹配任意字符（不包括路径分隔符 `/`）
- `?` 匹配单个字符
- `[abc]` 匹配字符集中的任意字符
- `[a-z]` 匹配字符范围

实际例子：
- `chapters/*` 匹配 `chapters/01_basics`、`chapters/02_ownership` 等
- `services/user-*` 匹配 `services/user-api`、`services/user-worker` 等
- `plugins/[a-z]*` 只匹配小写字母开头的目录

### exclude 配置

`exclude` 字段排除那些不应该作为 member 的目录（通常是与 members glob 配合使用）：

```toml
[workspace]
members = ["*"]
exclude = [
    "experiments",      # 实验性代码
    "archive/*",        # 归档的旧代码
    "docs",             # 文档目录
    "scripts",          # 脚本目录
]
```

**重要**: 不要在 members 中添加根目录本身（即 `.`），这可能导致递归包含问题。同样，避免创建嵌套的 workspace。

### 自动发现

从 Rust 2021 edition 开始，如果 `[workspace]` 段存在但没有 `members` 字段，Cargo 会尝试自动发现。但**不推荐依赖自动发现**，因为它会使构建行为不够明确。显式列出 members 是更好的做法。

## 共享资源: Cargo.lock 与 target/

### 共享 Cargo.lock

Workspace 只在**根目录**维护一个 `Cargo.lock` 文件。这个文件锁定了所有 member 的所有依赖的精确版本。

工作原理：
1. 每个 member 在自己的 `Cargo.toml` 中声明依赖
2. Cargo 解析所有 member 的依赖图
3. 生成一个统一的 `Cargo.lock`，确保无版本冲突
4. 如果两个 member 依赖同一个 crate 的不同版本，两个版本都会出现在 lock 中

好处：
- **一致性**: 团队所有人构建时得到相同的依赖
- **可重现**: CI 和生产环境使用相同版本
- **安全性**: 依赖审计时只需要检查一个文件

### 共享 target/ 目录

所有 member 的构建产物存放在 workspace 根目录的 `target/` 下，而不是各自目录下。

这意味着：
- `cargo build -p cli` 将 `cli` 和其依赖编译到 `target/debug/`
- `cargo build -p server` 复用已经在 `target/debug/` 中的依赖编译缓存
- `core` lib 只编译一次，即使被多个 member 依赖

对于大型 workspace，这可以节省大量的编译时间。例如，如果一个 workspace 有 20 个 member 且都依赖 `tokio`，`tokio` 只需编译一次。

### 共享配置

Workspace 根目录的 `.cargo/config.toml` 对**所有 member** 生效：

```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=native"]

[registries]
company-registry = { index = "https://artifacts.company.com/git/index" }

[target.x86_64-unknown-linux-gnu]
linker = "clang"
```

这提供了一个集中管理构建配置的位置，无需在每个 member 的目录中重复设置。

## 路径依赖

路径依赖是 workspace 内部 member 之间相互引用的标准方式。

### 基本语法

在 `cli/Cargo.toml` 中：

```toml
[dependencies]
core = { path = "../core" }
utils = { path = "../libs/utils" }
```

### 带版本号的路径依赖

```toml
[dependencies]
core = { path = "../core", version = "0.1" }
```

当指定 `version` 时，Cargo 会执行额外的验证：本地 crate 的实际版本必须满足指定的版本约束。这在以下场景非常有用：
- **准备发布**: 确保内部路径依赖的版本号与 crates.io 的要求一致
- **逐步迁移**: 先将 crate 发布到 crates.io，然后逐步将路径依赖替换为注册表依赖

### 路径依赖 vs 注册表依赖

| 维度 | 路径依赖 | 注册表依赖 |
|------|---------|-----------|
| 来源 | 本地源码 | crates.io / Git |
| 版本控制 | 总是最新（HEAD） | 由 Cargo.lock 锁定 |
| 修改反馈 | 即时 | 需 cargo update |
| 发布要求 | 不需要 publish | 需要 publish |
| 外部使用 | 不可用 | 可用 |
| 适用场景 | 内部开发 | 稳定依赖 |

### Workspace 内的路径依赖解析

当 Cargo 发现一个路径依赖的目标恰好在同一个 workspace 中时，它会：
1. 自动识别为内部依赖
2. 不将其视为外部包
3. 使用 workspace 共享的 Cargo.lock 进行版本解析
4. 确保编译顺序正确（先编译被依赖的 crate）

## Resolver 版本

### 什么是 Resolver

Resolver（解析器）是 Cargo 用来决定依赖图中每个包的具体版本和 features 的算法。不同版本的 resolver 有不同的解析策略。

### 版本演进

| Resolver | Edition | 主要特性 |
|----------|---------|---------|
| v1 | 2015, 2018 | 原始解析器，feature 全局合并 |
| v2 | 2021 | 平台特定依赖，不合并 dev-deps features |
| v3 | 2024 | 默认，改进 feature 处理，更好的可选依赖 |

### v3 Resolver 的关键特性

**Feature Unification**（Feature 统一）:
同一个 workspace 中的同一个包，其 features 被统一处理。特定 feature 的解析取决于构建目标类型。

**平台特定依赖**:
```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
nix = "0.29"
```
v3 resolver 确保在 Linux 上构建时不会拉取 Windows 依赖。

**可选依赖优化**:
`optional = true` 的依赖只在被 feature 显式启用时才参与解析，减少不必要的依赖下载和编译。

### 配置 Resolver

```toml
[workspace]
members = ["..."]
resolver = "3"     # 推荐使用，Rust 2024 默认
```

### 迁移到 v3

如果你从旧版 resolver 迁移，可能遇到以下变化：
- 某些 features 不再自动启用（需要显式声明）
- dev-dependencies 的 features 不再影响主构建
- 平台特定依赖的行为可能变化

如果在迁移后遇到编译错误，通常是某些 feature 需要显式启用。使用 `cargo tree --edges features` 来分析 feature 传递链。

## Workspace 级别依赖

Rust 2024 edition 引入了 workspace 级别的依赖管理。

### 定义

在根 `Cargo.toml` 中：

```toml
[workspace.dependencies]
# 注册表依赖
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
rand = "0.9"

# Git 依赖
internal-lib = { git = "https://github.com/company/internal-lib", branch = "main" }

# 路径依赖
utils = { path = "./libs/utils" }
```

### 在 Member 中引用

```toml
[dependencies]
serde = { workspace = true }                         # 继承完整定义
tokio = { workspace = true, features = ["rt"] }      # 覆盖 features
rand.workspace = true                                # 简洁写法

# 使用不同 features
serde = { workspace = true, features = [] }           # 不使用默认 features
```

### 优势

1. **集中版本管理**: 所有 member 共享依赖版本定义
2. **一处修改**: 升级依赖版本时只需修改根 Cargo.toml
3. **版本一致性**: 确保 workspace 内同一依赖版本统一
4. **减少重复**: 新 member 不需要重新写依赖声明

### 最佳实践

- 将所有 workspace 内多个 member 使用的依赖放在 `[workspace.dependencies]`
- 只有一个 member 使用的依赖可以放在该 member 自己的 `[dependencies]` 中
- 利用 feature 覆盖来满足不同 member 的不同需求
- 定期运行 `cargo update` 来保持依赖最新

## 典型 Workspace 模式

### 模式一: 核心库 + 多应用

```
my-project/
├── Cargo.toml          [workspace]
├── core/               lib crate - 业务逻辑
├── cli/                bin crate - 命令行工具
├── web/                bin crate - Web 服务器
└── worker/             bin crate - 后台任务
```

适用场景：需要多个入口点但共享核心逻辑的应用。

### 模式二: 库家族

```
my-framework/
├── Cargo.toml          [workspace]
├── my-core/            lib - 核心抽象
├── my-derive/          proc-macro - 派生宏
├── my-json/            lib - JSON 支持 (feature: json)
├── my-yaml/            lib - YAML 支持 (feature: yaml)
└── my/                 lib - 聚合 crate (re-export 所有)
```

适用场景：功能丰富的框架，按模块拆分为独立 crate（如 `tokio` 的模型）。

### 模式三: Monorepo 微服务

```
backend/
├── Cargo.toml          [workspace]
├── common/             lib - 共享 types, errors, utils
├── user-service/       bin - 用户服务
├── order-service/      bin - 订单服务
├── payment-service/    bin - 支付服务
├── api-gateway/        bin - API 网关
└── migration/          bin - 数据库迁移
```

适用场景：微服务架构，共享领域类型和工具库。

### 模式四: 教程/文档集合（本教程的模式）

```
rust-from-python/
├── Cargo.toml          [workspace] 虚拟 workspace
└── chapters/
    ├── 01_basics/
    ├── 02_ownership/
    ├── ...
    └── 30_ffi/
```

适用场景：教育材料、示例代码集合、技术文档。

## 本教程的 Workspace 设计

《Rust from Python》教程使用虚拟 workspace 组织所有章节：

```toml
# 根 Cargo.toml
[workspace]
members = ["chapters/*"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

设计考量：

1. **虚拟 workspace**: 根目录不产出 crate，避免与章节内容混淆
2. **Glob members**: `chapters/*` 自动包含所有章节，新增章节零配置
3. **集中依赖版本**: 常用库的版本在 workspace 级别统一管理
4. **共享编译缓存**: serde 等公共依赖在章节间复用
5. **每个章节独立 package**: 可单独运行、测试某个章节的代码

**注意事项**:
- 各章节内部**不能创建嵌套 workspace**，否则会与根 workspace 冲突
- 某些章节（如本章）需要解释 workspace 概念，但仅通过代码注释和输出来展示
- 路径依赖在章节之间不使用（各章节相互独立），但在教学材料中会解释其语法

## 什么时候不要用 Workspace

Workspace 是优秀的工具，但并非所有场景都适用：

### 不适合 Workspace 的场景

1. **独立项目**: crate 之间没有代码共享或相互引用关系
2. **单 crate 项目**: 只有一个 crate 时不需要 workspace 的开销
3. **版本迭代速度差异大**: 如果 core 库几个月不变，而应用频繁变更，分开管理可能更高效
4. **不相关的技术栈**: Python + Rust 的 monorepo，Rust 部分可以用 workspace，但不需要把 Python 部分纳入
5. **不同注册表**: 如果不同 crate 需要发布到不同的私有注册表，分开管理更清晰

### Workspace 的局限性

1. **统一版本**: 所有 member 必须使用相同版本的同名依赖（这是设计而非缺陷，但限制了灵活性）
2. **编译范围**: `cargo build --workspace` 构建所有 member，对于大型 workspace 可能耗时较长
3. **嵌套限制**: 不能在 workspace 内部创建另一个嵌套的 workspace
4. **发布复杂度**: workspace 中的每个 crate 需要单独发布到 crates.io

### 迁移建议

如果你的单 crate 项目开始增长，以下信号表明可能需要迁移到 workspace：
- 多个 `mod` 模块之间存在循环依赖风险
- 需要将某些模块作为独立库发布
- 编译时间过长，需要利用并行编译
- 不同的入口点需要不同的依赖集

## 实战: 创建 Workspace

### 创建虚拟 Workspace

```bash
# 1. 创建目录结构
mkdir my-project
cd my-project

# 2. 创建根 Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = ["core", "cli"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
EOF

# 3. 创建 core 库
cargo new core --lib

# 4. 创建 cli 应用
cargo new cli

# 5. 在 cli 中依赖 core
# 编辑 cli/Cargo.toml，添加:
# [dependencies]
# core = { path = "../core" }

# 6. 验证
cargo check --workspace
cargo build --workspace
cargo test --workspace
```

### 将现有项目迁移到 Workspace

```bash
# 1. 创建根目录
mkdir my-workspace
cd my-workspace

# 2. 创建根 Cargo.toml
# [workspace]
# members = ["my-app"]
# resolver = "3"

# 3. 将现有项目移入
mv /path/to/my-app .

# 4. 移除 my-app 中的 Cargo.lock (workspace 将统一管理)
rm my-app/Cargo.lock

# 5. 验证
cargo check
```

## 最佳实践

1. **使用 `workspace.dependencies`**: 集中管理所有共享依赖的版本
2. **使用 `resolver = "3"`**: 与 Rust 2024 edition 保持一致
3. **显式列出 members**: 虽然支持 glob 和自动发现，但显式列表更可预测
4. **一个 workspace 一个 Cargo.lock**: 不要手动在各 member 中创建 Cargo.lock
5. **在 CI 中构建整个 workspace**: `cargo build --workspace` 确保全局一致
6. **使用 `-p` 标志**: 开发和测试时使用 `cargo test -p my-crate` 只处理特定 member
7. **根目录保持简洁**: 虚拟 workspace 的根目录只放 Cargo.toml、Cargo.lock 和配置文件
8. **避免嵌套 workspace**: 一个 Git 仓库通常只需要一个 workspace

## 常见问题

### Q: Workspace 中能否有不同 edition 的 crate？

可以。每个 crate 独立声明自己的 `edition`。例如，旧的 crate 可以使用 2021 edition，新的可以使用 2024 edition。

### Q: 如何只测试 workspace 中的一个 crate？

```bash
cargo test -p crate_name
```

### Q: Workspace 中如何管理私有 crate 的可见性？

使用 `pub`、`pub(crate)`、`pub(super)` 等可见性修饰符。Workspace 不改变 Rust 的可见性规则。每个 crate 的公开 API 对所有依赖它的 crate 可见。

### Q: Workspace 能包含非 Rust 项目吗？

从 Cargo 的角度，workspace 只管理 Rust crate。但可以在同一 Git 仓库中存放 Python、JavaScript 等项目，它们不会被 Cargo 处理。

### Q: 如何将 workspace crate 发布到 crates.io？

```bash
cd my-crate-dir
cargo publish
```

每个 crate 独立发布，需要分别满足 crates.io 的元数据要求。

## 总结

Cargo Workspace 是 Rust 生态中管理多 crate 项目的标准方案：

- **虚拟 Workspace** 和 **根 Package Workspace** 两种结构满足不同场景
- **共享 Cargo.lock** 和 **共享 target/** 提供构建一致性和效率
- **路径依赖** 连接 workspace 内的各 crate
- **resolver = "3"** 提供现代化的依赖解析策略
- **workspace.dependencies** 实现集中版本管理

Workspace 让 Rust 项目能够从小到大平滑演进：从单个 `main.rs` 开始，逐步拆分为多个 crate，最终可以扩展为包含数十个 crate 的 monorepo，而整个过程由统一的工具链提供支持。

对于从 Python transitioning 的开发者：Cargo workspace 的概念与 Python 的 monorepo 工具（poetry workspaces、pip editable installs）类似，但 Cargo 的集成更深——编译缓存共享、lock 文件统一、构建配置集中管理，这些在 Python 生态中通常需要额外工具和手动配置才能实现。

## 参考资源

- [Cargo Workspaces - The Cargo Book](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Package Layout - The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Resolver V3 RFC](https://rust-lang.github.io/rfcs/3492-cargo-resolver-v3.html)
- [Workspace Inheritance](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-dependencies-from-the-workspace-root)
