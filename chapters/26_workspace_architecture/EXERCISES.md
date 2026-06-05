# 练习: Workspace 架构

## 基础练习

### 练习 1: 理解 Workspace 结构 (15 分钟)

**目标**: 通过运行命令直观理解 workspace 的组织方式。

**步骤**:
1. 在教程根目录运行 `cargo metadata --no-deps` 查看所有 member
2. 运行 `cargo check --workspace` 检查所有章节
3. 运行 `cargo build -p workspace_architecture` 单独构建本章
4. 观察 `target/` 目录的位置（在 workspace 根，不在本章目录）

**思考题**:
- `cargo metadata` 输出的 workspace_members 有哪些？
- 为什么 `target/` 目录在根目录而不是在各章节？
- 如果删除根 Cargo.lock 文件并重新构建，会发生什么？

### 练习 2: 路径依赖分析 (15 分钟)

**目标**: 理解路径依赖在 workspace 中的工作方式。

**步骤**:
1. 阅读 `src/main.rs` 中关于路径依赖的注释
2. 在一张纸上画出以下 workspace 的依赖关系图:

```
[workspace]
members = ["core", "cli", "server", "common"]
```

假设依赖关系为:
- cli 依赖 core 和 common
- server 依赖 core 和 common
- common 无外部依赖
- core 依赖 common

3. 回答: Cargo 会按什么顺序编译这些 crate？为什么？

**思考题**:
- 如果 `core` 和 `cli` 都依赖 `common`，`common` 会被编译几次？为什么？
- 如果 `common` 的公开 API 发生变化，哪些 crate 需要重新编译？

### 练习 3: 虚拟 Workspace vs 根 Package Workspace (10 分钟)

**目标**: 区分两种 workspace 结构。

**场景 A**:
```toml
# 根 Cargo.toml
[workspace]
members = ["core", "cli"]
```

**场景 B**:
```toml
# 根 Cargo.toml
[package]
name = "my-app"
version = "0.1.0"

[workspace]
members = ["core", "cli"]
```

**问题**:
1. 场景 A 的根目录是否产出一个 crate？
2. 场景 B 的根目录产出的 crate 类型是什么（lib 还是 bin）？
3. 在场景 B 中，`my-app` 如何依赖 `core`？
4. 本教程使用的是哪种结构？为什么这种选择是合理的？

## 进阶练习

### 练习 4: 设计一个 Workspace (30 分钟)

**目标**: 为一个虚拟项目设计完整的 workspace 结构。

**项目描述**: "TaskFlow" - 一个任务管理后端系统。

**需求**:
- **核心领域模型**: Task, User, Project 的定义和基本操作
- **REST API 服务**: 提供 HTTP API
- **CLI 管理工具**: 命令行管理接口
- **数据库迁移工具**: 管理数据库 schema
- **共享工具库**: 错误类型、日志配置、配置解析

**任务**:
1. 画出目录结构（树状图）
2. 写出根 Cargo.toml 的内容（[workspace] 段 + workspace.dependencies）
3. 写出至少两个 member 的 Cargo.toml 框架
4. 标出所有路径依赖关系
5. 推荐合适的公共依赖（serde, tokio, sqlx 等）及其版本约束

**思考题**:
- 为什么将 domain model 单独作为一个 crate？
- 如果后续要添加 gRPC 服务，应该创建新 member 还是在现有 API 服务中添加？
- migration 工具是否需要依赖 core？为什么？

### 练习 5: Workspace 依赖版本管理 (20 分钟)

**目标**: 实践 workspace.dependencies 的使用。

**背景**:
```toml
# 根 Cargo.toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
rand = "0.9"
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
```

**任务**:
1. 为以下 member 写出其 `[dependencies]` 段:
   - `core` 需要 serde(derive), rand
   - `api` 需要 tokio(full), serde(derive), sqlx(postgres)
   - `cli` 需要 tokio(rt only), serde(derive), core(path)

2. 如果要将 `serde` 从 1.0 升级到 2.0（假设发布），需要在哪些文件中做修改？

**思考题**:
- `cli` 如何才能使用 `tokio` 但不需要 `rt-multi-thread` feature？
- 如果 `api` 需要 sqlx 的额外 feature `chrono`，怎样在继承 workspace 定义的同时添加该 feature？

### 练习 6: 评估 CI 中 Workspace 构建策略 (15 分钟)

**目标**: 设计高效的 CI 流水线。

**场景**: workspace 有 20 个 member，完全构建需要 30 分钟。

**问题**:
1. 哪些命令适合在 PR 的 CI 中运行？
2. 如何在 CI 中只构建受 PR 变更影响的 member？
3. 如何利用 GitHub Actions 的缓存来加速 workspace 构建？

**提示**:
- 可以使用 `git diff --name-only HEAD~1` 确定变更的文件
- Cargo 的增量编译和 sccache 都可以加速 CI

## 综合练习

### 练习 7: 迁移分析 (30 分钟)

**目标**: 规划从一个单 crate 项目到 workspace 的迁移。

**背景**: 你有一个单 crate 项目，包含以下模块:
```
src/
├── main.rs          # 入口点 (1500 行)
├── domain/          # 领域模型
│   ├── mod.rs
│   ├── user.rs
│   └── task.rs
├── api/             # API 处理器
│   ├── mod.rs
│   └── handlers.rs
├── db/              # 数据库层
│   ├── mod.rs
│   └── queries.rs
└── utils/           # 工具函数
    ├── mod.rs
    └── config.rs
```

**任务**:
1. 识别可以拆分为独立 crate 的部分
2. 为每个新 crate 命名并说明其职责
3. 画出迁移后的 workspace 结构
4. 描述迁移步骤（按什么顺序、每步做什么）
5. 识别潜在的风险点（循环依赖、公开 API 变更等）

**思考题**:
- 迁移过程中如何保持项目可以继续开发和部署？
- 如果 domain 模块被多个 crate 使用，应该作为独立的 lib crate 还是保留在某个 crate 内？

### 练习 8: 对比其他语言的 Monorepo 方案

**任务**: 研究并对比至少两种其他语言的 monorepo/workspace 方案。

**比较维度**:
- Python: poetry workspaces / pip editable installs
- JavaScript/TypeScript: npm workspaces / yarn workspaces / pnpm workspaces
- Go: Go modules + workspace (go.work)
- Java: Gradle multi-project builds

**问题**:
1. 各方案的依赖解析策略有何不同？
2. 版本锁定机制对比（Cargo.lock vs poetry.lock vs package-lock.json）
3. 哪些方案与 Cargo workspace 最相似？哪些最不同？
4. Cargo workspace 有哪些优势是其他方案没有的？
5. 其他方案有哪些特性是 Cargo workspace 可以借鉴的？

## 提交要求

- 练习 1-3 为基础必做，提交运行命令和输出
- 练习 4-6 为进阶选做（建议至少完成 2 个）
- 练习 7-8 为思考题，提交书面分析
- 所有练习整理到一个目录中提交
