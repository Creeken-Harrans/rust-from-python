# Cargo 工程能力: 依赖、Feature 与 Profile

## 目录

1. [概述](#概述)
2. [依赖管理](#依赖管理)
3. [Feature 系统](#feature-系统)
4. [Profile 配置](#profile-配置)
5. [Cargo 工具集](#cargo-工具集)
6. [crates.io 生态](#cratesio-生态)
7. [实战指南](#实战指南)
8. [最佳实践](#最佳实践)
9. [常见问题](#常见问题)
10. [总结](#总结)

## 概述

Cargo 是 Rust 的官方构建系统和包管理器。它不仅仅是编译工具，更是一整套工程能力体系，涵盖了依赖管理、条件编译、构建配置、发布分发等完整的软件生命周期管理。本章将深入探讨 Cargo 的三个核心能力领域：依赖管理（Dependencies）、Feature 系统（Features）和 Profile 配置（Profiles）。

理解 Cargo 的工程能力对于从 Python 生态转向 Rust 的开发者至关重要。在 Python 中，我们习惯使用 `pip`、`requirements.txt`、`pyproject.toml` 等工具来管理依赖；而在 Rust 中，Cargo 提供了一个统一且强大的方案，将依赖管理、构建配置、测试、文档生成等功能集成在一个工具中。

Cargo 的设计哲学是"约定优于配置"（Convention over Configuration）。它提供了一套合理的默认值，让开发者能够以最小的配置投入启动一个项目，同时保留了足够灵活的配置能力来满足复杂项目的需求。

## 依赖管理

### 版本规范

Rust 使用语义化版本（Semantic Versioning，简称 SemVer）来管理 crate 的版本。SemVer 的格式为 `MAJOR.MINOR.PATCH`，例如 `1.2.3`：

- **MAJOR**: 不兼容的 API 修改（破坏性变更）
- **MINOR**: 向后兼容的功能新增
- **PATCH**: 向后兼容的问题修复

在 Cargo.toml 中指定依赖版本时，可以使用多种版本约束符：

```toml
[dependencies]
# 脱字符要求（默认行为）: ^1.2.3 等价于 >=1.2.3, <2.0.0
serde = "1.0"

# 波浪线要求: ~1.2.3 等价于 >=1.2.3, <1.3.0
rand = "~0.8.5"

# 精确版本
uuid = "=1.6.0"

# 通配符: 1.* 等价于 >=1.0.0, <2.0.0
log = "0.4.*"

# 比较运算符: >=, >, <, <=
tokio = ">=1.35, <2.0"

# 多个约束组合
crossbeam = ">=0.8, <0.9"
```

理解版本约束对于维护项目的长期稳定性非常重要。过于宽松的约束可能导致意外引入破坏性变更；过于严格的约束则会限制与其他依赖的兼容性。推荐的做法是：对于 0.x 版本使用 `"0.x.y"`（精确到 minor），对于 1.0+ 版本使用 `"1.x"`（使用脱字符语义）。

### Cargo.lock 文件

`Cargo.lock` 是 Cargo 自动生成的文件，精确记录了当前项目使用的每个依赖的具体版本和来源。它的作用是：

- **可重现构建**: 确保团队成员和 CI 环境使用完全相同的依赖版本
- **安全审计**: 提供完整的依赖图谱，便于安全漏洞追踪
- **版本管理**: 记录传递性依赖的精确版本，包括来源 hash

对于**二进制项目**（有 `main.rs` 的 crate），Cargo.lock 应该提交到版本控制系统中。这确保了：
- 任何人在任何时间 clone 项目后，`cargo build` 都能产生相同的二进制产物
- CI 流水线中构建的产物与实际发布的产物一致
- 安全性审计能够针对具体的依赖版本进行

对于**库项目**（只有 `lib.rs` 的 crate），Cargo 默认将 Cargo.lock 添加到 `.gitignore` 中，但在实践中许多项目（包括官方项目）也会选择提交。提交的好处是：
- CI 构建更加可靠和可重现
- 开发者之间更容易共享可复现的环境
- 依赖版本的变更更容易被追踪和 review

当你运行 `cargo update` 时，Cargo 会根据 Cargo.toml 中的版本约束，更新 Cargo.lock 中记录的依赖到最新的兼容版本。

### 依赖类型

Cargo 支持多种依赖类型和来源：

#### 注册表依赖（Registry Dependencies）

从 crates.io 下载的标准依赖：

```toml
[dependencies]
# 默认来源，crates.io
serde = { version = "1", features = ["derive"] }

# 指定注册表（需要先在 .cargo/config.toml 中配置）
my-crate = { version = "1", registry = "my-registry" }
```

#### Git 依赖（Git Dependencies）

直接从 Git 仓库引入依赖：

```toml
[dependencies]
# 指定分支
my-lib = { git = "https://github.com/user/my-lib", branch = "main" }

# 指定 tag
my-lib = { git = "https://github.com/user/my-lib", tag = "v1.2.0" }

# 指定 commit hash
my-lib = { git = "https://github.com/user/my-lib", rev = "abc1234" }

# 指定路径中的子 crate
my-sub-crate = { git = "https://github.com/user/monorepo", package = "sub-crate" }
```

Git 依赖在以下场景中非常有用：
- 依赖尚未发布到 crates.io 的实验性功能
- 内部工具库（当不便搭建私有注册表时）
- 临时使用某个 fork 版本
- 针对特定 commit 的 bug 修复 hotfix

但需要注意，Git 依赖会降低构建的可重现性，因为分支内容可能随时间变化。如果可以，尽量使用具体的 tag 或 commit hash。

#### 路径依赖（Path Dependencies）

本地开发中最常用的依赖方式，用于 workspace 或本地多 crate 项目：

```toml
[dependencies]
my-utils = { path = "../my-utils" }
my-core = { path = "../core" }
```

路径依赖的优势：
- 不需要先 `cargo publish` 就可以开发和测试
- 修改立即生效，无需版本号管理
- 开发迭代速度远超注册表依赖
- 可在同一 workspace 中方便地管理多个相互依赖的 crate

在 workspace 中，路径依赖是连接各个 crate 的纽带。不同 crate 之间通过相对路径引用，Cargo 会自动处理依赖图的解析和构建顺序。

### 依赖类别

Cargo 支持三类依赖，分别用于不同的构建阶段：

#### [dependencies] - 运行时依赖

```toml
[dependencies]
serde = "1"
tokio = { version = "1", features = ["full"] }
```

这些依赖在最终产物中被链接。对于库 crate，它们是传递性依赖的一部分；对于二进制 crate，它们被静态链接到可执行文件中。

#### [dev-dependencies] - 开发依赖

```toml
[dev-dependencies]
criterion = "0.5"    # 性能基准测试
pretty_assertions = "1"  # 更好看的断言输出
tempfile = "3"        # 创建临时文件/目录
```

开发依赖仅在以下场景中被编译和链接：
- 运行 `cargo test` 时
- 运行 `cargo bench` 时
- 运行示例（examples）时
- 运行集成测试时

开发依赖不会被包含在最终发布的 crate 依赖图中。这意味着你的用户的 `Cargo.lock` 中不会出现这些依赖，从而减少依赖树的大小。

#### [build-dependencies] - 构建依赖

```toml
[build-dependencies]
cc = "1"             # 编译 C/C++ 代码
bindgen = "0.69"     # 从 C 头文件生成 Rust FFI 绑定
prost-build = "0.12" # 编译 Protocol Buffers 定义
```

构建依赖用于编译 `build.rs` 脚本，在 crate 编译之前执行。`build.rs` 可以用来：
- 编译链接 C/C++ 库
- 生成 Rust 代码（如 protobuf 绑定）
- 检测系统特性（如 CPU 指令集支持）
- 设置编译时的环境变量

构建依赖不会被传递到下游使用者。它们只在构建阶段存在，构建完成后即被丢弃。

### 可选依赖与 Feature 的关系

将依赖标记为 `optional = true`，该依赖就变成了一个隐式的 feature：

```toml
[dependencies]
serde = { version = "1", optional = true }
serde_json = { version = "1", optional = true }

[features]
json_support = ["serde", "serde_json"]
```

- `optional = true` 表示默认不引入该依赖
- 该依赖的名称自动成为一个隐式 feature
- 在 feature 定义中显式引用来创建更有意义的 feature 名称
- 只有当对应的 feature 被启用时，依赖才会被下载、编译和链接

这种方式让你的 crate 可以保持轻量，使用者只需引入需要的依赖。一个典型的例子是 `reqwest`：默认不依赖 TLS 库，用户可以根据需要选择 `native-tls` 或 `rustls` feature。

## Feature 系统

### Feature 是什么

Feature 是 Cargo 的条件编译机制，允许 crate 的消费者选择性地启用功能。Feature 的主要用途包括：

- **可选功能**: 某些功能不是所有用户都需要，通过 feature 控制
- **可选依赖**: 依赖某些大型库时，提供"按需引入"的能力
- **行为切换**: 同一接口的不同实现策略（如不同的 TLS 后端）
- **条件编译**: 在代码中使用 `#[cfg(feature = "...")]` 条件编译
- **平台适配**: 为不同平台启用不同的功能集

### Feature 定义

在 Cargo.toml 中定义 features：

```toml
[features]
# 基础 feature 定义
default = ["basic"]           # 默认启用的 features
basic = []                    # 独立 feature，不依赖其他
advanced = ["basic"]          # 依赖 basic，启用 advanced 会同时启用 basic

# 可选依赖 feature
json = ["serde", "serde_json"]

# 可选依赖本身也是隐式 feature
# serde = { version = "1", optional = true } 会自动创建 feature "serde"
```

### Feature 依赖链

Feature 之间存在依赖关系：

```
default ──→ basic
advanced ──→ basic
json_output ──→ serde + serde_json
```

当启用 `advanced` 时，Cargo 会自动启用它所依赖的 `basic`。这种依赖链设计让 feature 系统能够：
- 避免重复声明公共依赖
- 形成清晰的 feature 层级结构
- 简化用户的使用体验（只需启用顶层 feature）

### 条件编译

在 Rust 源代码中使用 feature 进行条件编译：

```rust
// 编译时检查 feature 是否启用
#[cfg(feature = "basic")]
fn basic_only() {
    // 仅当 basic feature 启用时才编译此函数
}

#[cfg(feature = "json_output")]
use serde::{Serialize, Deserialize};

// cfg! 宏在运行时返回 bool（但仍在编译时确定）
if cfg!(feature = "json_output") {
    println!("JSON 支持已启用");
}

// 互斥 feature 用法
#[cfg(all(feature = "basic", not(feature = "advanced")))]
fn basic_not_advanced() { }

// 组合条件
#[cfg(any(feature = "json_output", feature = "yaml_output"))]
fn any_output_format() { }
```

`#[cfg]` 和 `cfg!` 的核心区别：
- `#[cfg]` 在**编译时**排除代码，被排除的代码根本不会被编译
- `cfg!` 展开为 `true` 或 `false`，被包裹的代码始终需要能通过编译检查

因此，当某些代码引用了仅在特定 feature 下才存在的类型或函数时，必须使用 `#[cfg]` 而不是 `cfg!`。

## Profile 配置

### 什么是 Profile

Profile 定义了 Rust 编译器的优化和调试设置。Cargo 内置了两个主要 Profile：

| 属性 | dev (开发) | release (发布) |
|------|-----------|---------------|
| opt-level | 0 | 3 |
| debug | true | false |
| debug-assertions | true | false |
| overflow-checks | true | false |
| lto | false | false |
| panic | unwind | unwind |
| incremental | true | false |
| codegen-units | 256 | 16 |

### opt-level 优化级别

`opt-level` 控制编译器的优化程度：

- **0**: 无优化，编译速度最快。适合日常开发中的快速迭代
- **1**: 基本优化，编译速度较快，运行时性能有所提升
- **2**: 标准优化，编译和运行性能的良好平衡（LLVM -O2）
- **3**: 激进优化，运行速度最快，但编译时间最长（LLVM -O3）
- **"s"**: 体积优化，牺牲部分运行速度以减小二进制体积
- **"z"**: 激进体积优化，进一步减小体积（可能影响性能）

```toml
[profile.dev]
opt-level = 0  # 开发时快速编译

[profile.release]
opt-level = 3  # 发布时最大性能

# 自定义 Profile
[profile.staging]
inherits = "release"
opt-level = "s"  # 优化体积
debug = true     # 但仍保留调试符号
```

### 自定义 Profile

除了 `dev` 和 `release`，你还可以定义自定义 Profile：

```toml
# 测试 Profile：类似 release 但有调试信息
[profile.bench]
inherits = "release"
debug = true
debug-assertions = true

# CI Profile：平衡编译速度和运行性能
[profile.ci]
inherits = "dev"
opt-level = 2
incremental = false  # CI 环境不需要增量编译

# 发布体积优化
[profile.dist]
inherits = "release"
opt-level = "s"
lto = true          # 启用链接时优化
codegen-units = 1    # 单个代码生成单元以获得更好的优化
strip = true         # 删除符号表
```

使用自定义 Profile：`cargo build --profile staging`

## Cargo 工具集

### cargo tree

`cargo tree` 以树状结构展示项目的依赖图，对于理解依赖关系和调试版本冲突非常有用。

基本用法：
```bash
# 展示当前 crate 的依赖树
cargo tree

# 显示 features 信息
cargo tree --edges features

# 反向查看谁依赖了某个包
cargo tree --invert -p serde

# 只显示直接依赖
cargo tree --depth 1

# 过滤特定包
cargo tree -p tokio

# 查看特定 feature 激活下的依赖树
cargo tree --features json_output

# 查看所有 feature 激活下的依赖树
cargo tree --all-features

# 过滤重复依赖（去重）
cargo tree --duplicates
```

`cargo tree` 对于解决"为什么我的二进制文件这么大"或"为什么这个依赖的某个版本被包含了"这类问题非常有用。重复依赖通常是通过 `cargo tree --duplicates` 发现并在 Cargo.toml 中通过 `[patch]` 或更新版本来解决的。

### cargo metadata

`cargo metadata` 输出 crate 的完整结构化元数据，包括依赖图、features、目标等信息，以 JSON 格式呈现。

```bash
# 基本用法
cargo metadata

# 格式化输出（需要 jq）
cargo metadata | jq .

# 只查看 workspace members
cargo metadata --no-deps | jq '.workspace_members'

# 查看特定包的解析结果
cargo metadata --format-version 1 | jq '.resolve'

# 查看所有 features
cargo metadata | jq '.packages[].features'

# 列出所有依赖名称和版本
cargo metadata | jq '.packages[] | {name: .name, version: .version}'
```

`cargo metadata` 是 IDE 工具（如 rust-analyzer）和构建工具链的基础。它提供了机器可读的项目描述，用于驱动代码补全、依赖分析等功能。

### cargo install

`cargo install` 从 crates.io（或指定的 Git 仓库）下载、编译并安装 Rust 二进制工具：

```bash
# 安装常用工具
cargo install cargo-edit      # 添加 cargo add/rm/upgrade 命令
cargo install cargo-watch     # 文件变更时自动执行命令
cargo install cargo-audit     # 安全漏洞检查
cargo install cargo-outdated  # 检查过时的依赖
cargo install cargo-deny      # 依赖许可证检查
cargo install cargo-nextest   # 更快的测试运行器
cargo install cargo-expand    # 展开宏
cargo install bat             # 更好的 cat 替代品
cargo install ripgrep         # 更快的 grep
cargo install fd-find         # 更快的 find

# 指定版本安装
cargo install cargo-edit --version 0.12.0

# 安装后列出所有已安装的工具
cargo install --list

# 从 Git 仓库安装
cargo install --git https://github.com/user/tool

# 从本地路径安装
cargo install --path ./my-tool
```

安装后的二进制文件默认放在 `~/.cargo/bin/` 目录下。确保该路径在 `$PATH` 中即可直接使用。

### crates.io 发布

将你的 crate 发布到 crates.io 让其他人可以使用：

```bash
# 1. 在 crates.io 注册账号并获取 API token
# 2. 登录
cargo login <your-api-token>

# 3. 确保 Cargo.toml 中包含必要字段
# [package]
# name = "your-crate"
# version = "0.1.0"
# edition = "2024"
# description = "..."
# license = "MIT OR Apache-2.0"

# 4. 打包验证
cargo package

# 5. 发布到 crates.io
cargo publish

# 6. 更新版本后重新发布
# 修改 version 字段后
cargo publish

# 撤回某个版本（yank，不是删除）
cargo yank --vers 0.1.0
cargo yank --vers 0.1.0 --undo  # 取消撤回
```

发布前务必确保：
- `description` 和 `license` 字段已填写（crates.io 强制要求）
- README.md 包含基本的使用说明
- API 文档已生成（`cargo doc --open` 确认文档质量）
- `cargo test` 全部通过
- 版本号遵循 SemVer

## 最佳实践

### Feature 设计原则

1. **Default 应该是最常用的功能集**：`default` feature 应该包含大多数用户需要的功能，保持简洁实用
2. **Feature 命名清晰**：使用有意义的名称，如 `json`、`tls`、`async`，一目了然
3. **避免 feature 爆炸**：不要在单个 crate 中定义过多 feature（通常不超过 10-15 个），必要时考虑拆分 crate
4. **文档化 features**：在 README 和文档注释中清楚说明每个 feature 的作用
5. **使用 feature 依赖**：当 feature A 总是需要 feature B 时，让 A 依赖 B 以简化用户配置
6. **互斥 feature 使用编译错误**：使用 `#[cfg(not(...))]` 配合 `compile_error!` 宏在用户误用时给出清晰提示

### 依赖管理建议

1. **锁定主版本号**：生产项目使用 `"1.0"` 而非 `"*"`
2. **定期更新依赖**：使用 `cargo update` 或 `cargo outdated` 定期检查
3. **审计依赖安全**：使用 `cargo audit` 检查已知漏洞
4. **最小化依赖树**：避免引入不必要的依赖，使用 `cargo tree` 分析

### Profile 选择指南

| 场景 | 推荐 Profile | 理由 |
|------|-------------|------|
| 日常开发 | dev | 编译快，debug 信息全 |
| 本地测试 | dev | 快速迭代 |
| CI 测试 | dev (但 incremental=false) | 避免缓存问题 |
| 生产部署 | release | 最大性能 |
| 嵌入式/容器 | release + opt="s" + lto | 最小化体积 |
| 性能剖析 | release + debug=true | 优化 + 符号信息 |

## 常见问题

### Q: Cargo.lock 应该提交到 Git 吗？

**二进制项目**（应用程序）：应该提交。确保可重现构建。

**库项目**：传统上不提交，但现在越来越多的库项目也选择提交，便于 CI 和开发协作。如果你维护一个库并希望确保 CI 构建可重现，提交 Cargo.lock 是推荐的。

### Q: 如何解决依赖冲突？

1. 使用 `cargo tree --duplicates` 查看重复依赖
2. 使用 `[patch]` 覆盖特定依赖的版本
3. 更新 Cargo.toml 中的版本约束
4. 向依赖上游提交 PR 放宽版本约束

### Q: 怎样减小编译后的二进制文件体积？

```toml
[profile.release]
opt-level = "s"      # 或 "z"
lto = true           # 启用链接时优化
codegen-units = 1    # 更好的优化
strip = true         # 删除符号表
panic = "abort"      # 移除 unwind 表
```

## 总结

Cargo 是 Rust 生态的核心基础设施，它提供的工程能力远超一般语言的包管理器：

- **依赖管理**: 支持注册表、Git、路径三种来源，严格的 SemVer 约束，自动解析依赖图
- **Feature 系统**: 灵活的条件编译机制，支持可选功能、可选依赖和功能组合
- **Profile 配置**: 精细的编译优化控制，从开发到生产的全流程支持
- **工具集**: `cargo tree`、`cargo metadata`、`cargo install` 等丰富的辅助工具
- **生态**: crates.io 提供了 15 万+ 的高质量 crate

掌握 Cargo 的工程能力，是写出高质量 Rust 项目的基础。从简单的 `cargo new` 开始，逐步熟悉依赖管理、feature 设计和 profile 调优，你将能够构建出健壮、高效、可维护的 Rust 项目。

## 参考资源

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Features Documentation](https://doc.rust-lang.org/cargo/reference/features.html)
- [Profiles Documentation](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Semantic Versioning](https://semver.org/)
- [crates.io](https://crates.io/)
