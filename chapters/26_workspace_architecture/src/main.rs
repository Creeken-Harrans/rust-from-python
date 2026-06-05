#![allow(clippy::empty_line_after_doc_comments)]
//
// 工作空间架构 (Workspace Architecture)
//
// 注意: 本 Chapter 作为一个独立的展示包，不创建嵌套 workspace
// （那样会与教材根 workspace 冲突）。
// 本章通过代码输出和注释来解释 workspace 的概念和用法。
//

/// 工作空间（Workspace）是 Cargo 提供的多 Package 项目管理机制。
///
/// 核心概念:
/// - 一个 workspace 包含多个 Rust package (crate)
/// - 所有 member 共享同一个 Cargo.lock 文件
/// - 所有 member 共享同一个 target/ 编译输出目录
/// - 通过路径依赖 (path = "../other") 连接各 member
///
/// Workspace 的典型目录结构:
///
/// ```text
/// my-project/              # workspace 根目录
/// ├── Cargo.toml           # workspace 清单 (定义 members)
/// ├── Cargo.lock           # 共享的 lock 文件
/// ├── target/              # 共享的编译输出目录
/// ├── core/                # member: 核心库
/// │   ├── Cargo.toml
/// │   └── src/lib.rs
/// ├── cli/                 # member: CLI 应用
/// │   ├── Cargo.toml
/// │   └── src/main.rs
/// ├── server/              # member: HTTP 服务
/// │   ├── Cargo.toml
/// │   └── src/main.rs
/// └── tests/               # member: 集成测试
///     ├── Cargo.toml
///     └── src/lib.rs
/// ```
///
/// Workspace root Cargo.toml 示例:
///
/// ```toml
/// [workspace]
/// members = [
///     "core",
///     "cli",
///     "server",
///     "tests",
/// ]
/// exclude = ["old-experiment"]
/// resolver = "3"
///
/// [workspace.dependencies]
/// serde = { version = "1", features = ["derive"] }
/// tokio = { version = "1", features = ["full"] }
/// ```
///
/// 路径依赖语法:
/// 在 cli/Cargo.toml 中引用 core:
///
/// ```toml
/// [dependencies]
/// core = { path = "../core" }
/// ```
///
/// Workspace 级别依赖 (workspace.dependencies):
/// 统一管理版本，各 member 只需引用:
///
/// ```toml
/// # 在 member 的 Cargo.toml 中
/// [dependencies]
/// serde = { workspace = true }
/// tokio = { workspace = true, features = ["macros"] }
/// ```

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║       Cargo 工作空间架构 (Workspace Architecture)          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    section_what_is_workspace();
    section_virtual_vs_root();
    section_members_and_exclude();
    section_shared_resources();
    section_path_dependencies();
    section_resolver_v3();
    section_workspace_dependencies();
    section_typical_patterns();
    section_why_this_tutorial_uses_workspace();
    section_when_not_to_use_workspace();
    section_ascii_diagram();
    section_commands();
}

/// 什么是 Workspace
fn section_what_is_workspace() {
    println!("═══════════════════════════════════════════════");
    println!("  一、什么是 Workspace");
    println!("═══════════════════════════════════════════════\n");

    println!("Workspace (工作空间) 是 Cargo 提供的多 Package (crate) 项目");
    println!("管理机制。它允许在一个仓库 (repository) 中管理多个相互关联");
    println!("的 Rust 包。\n");

    println!("核心特点:");
    println!("  • 一个 workspace 包含多个 package (crate)");
    println!("  • 所有 member 共享一个 Cargo.lock 文件");
    println!("  • 所有 member 共享一个 target/ 编译输出目录");
    println!("  • 通过路径依赖连接各 member");
    println!("  • 统一的依赖版本管理 (workspace.dependencies)");
    println!("  • 一条命令构建/测试所有 member\n");

    println!("对比 Python 生态:");
    println!("  Python 的 monorepo 通常使用 poetry workspaces 或");
    println!("  pip 的 -e (editable install) 来管理多包项目。");
    println!("  Cargo workspace 提供了更集成的体验：共享 lock 文件、");
    println!("  共享编译缓存、统一的构建命令。\n");
}

/// 虚拟 Workspace vs 根 Package Workspace
fn section_virtual_vs_root() {
    println!("═══════════════════════════════════════════════");
    println!("  二、虚拟 Workspace vs 根 Package Workspace");
    println!("═══════════════════════════════════════════════\n");

    println!("Cargo 支持两种 workspace 根结构:\n");

    println!("1. 虚拟 Workspace (Virtual Workspace)");
    println!("   - 根 Cargo.toml 只有 [workspace] 段，没有 [package] 段");
    println!("   - 根目录本身不产出 crate");
    println!("   - 只负责管理子 member");
    println!("   - 示例: 本教材的根 workspace\n");

    println!("  根 Cargo.toml 示例:");
    println!("  ┌──────────────────────────────────────┐");
    println!("  │ [workspace]                          │");
    println!("  │ members = [\"core\", \"cli\", \"server\"]  │");
    println!("  │ resolver = \"3\"                       │");
    println!("  │                                      │");
    println!("  │ # 注意: 没有 [package] 段            │");
    println!("  └──────────────────────────────────────┘\n");

    println!("2. 根 Package Workspace (Root Package)");
    println!("   - 根 Cargo.toml 同时有 [package] 和 [workspace] 段");
    println!("   - 根目录本身是一个 crate（通常是主应用）");
    println!("   - 子 member 作为辅助库存在\n");

    println!("  根 Cargo.toml 示例:");
    println!("  ┌──────────────────────────────────────┐");
    println!("  │ [package]                            │");
    println!("  │ name = \"my-app\"                      │");
    println!("  │ version = \"0.1.0\"                    │");
    println!("  │                                      │");
    println!("  │ [workspace]                          │");
    println!("  │ members = [\"lib-core\", \"lib-utils\"]  │");
    println!("  └──────────────────────────────────────┘\n");

    println!("本教程使用的是虚拟 Workspace 结构，根目录不产出 crate。\n");
}

/// Members 与 Exclude
fn section_members_and_exclude() {
    println!("═══════════════════════════════════════════════");
    println!("  三、Members 与 Exclude");
    println!("═══════════════════════════════════════════════\n");

    println!("members 字段定义了 workspace 包含哪些 package:\n");

    println!("  [workspace]");
    println!("  members = [");
    println!("      \"core\",             # 单个目录");
    println!("      \"apps/cli\",         # 子目录中的 package");
    println!("      \"services/*\",       # glob 模式: services 下的所有目录");
    println!("      \"libs/network-*\",   # glob: 匹配 network-http, network-tcp 等");
    println!("  ]\n");

    println!("exclude 字段排除不需要的目录:");
    println!("  exclude = [");
    println!("      \"experiments/*\",    # 实验性代码不参与构建");
    println!("      \"benchmarks\",        # 独立的基准测试目录");
    println!("  ]\n");

    println!("注意事项:");
    println!("  • members 中的路径使用 glob 模式，类似 shell 通配符");
    println!("  • members 路径是相对于 workspace 根目录的");
    println!("  • 子目录中有 Cargo.toml 但不是 member 的目录会被忽略");
    println!("  • 每个 member 都是一个独立的 crate，有自己的 Cargo.toml\n");
}

/// 共享资源
fn section_shared_resources() {
    println!("═══════════════════════════════════════════════");
    println!("  四、共享资源: Cargo.lock 与 target/");
    println!("═══════════════════════════════════════════════\n");

    println!("Workspace 的核心价值之一是资源共享:\n");

    println!("共享 Cargo.lock:");
    println!("  • workspace 根目录只需要一个 Cargo.lock");
    println!("  • 所有 member 的依赖版本在这个 lock 文件中统一管理");
    println!("  • 保证所有 member 使用相同版本的同名依赖");
    println!("  • 避免 diamond dependency 问题（菱形依赖）");
    println!("  • 运行 cargo update 会统一更新所有 member 的依赖\n");

    println!("共享 target/ 目录:");
    println!("  • 所有 member 的编译产物放在同一个 target/ 下");
    println!("  • 共享依赖的编译缓存 → 大幅减少重复编译时间");
    println!("  • 共享 build script 的输出");
    println!("  • 如果 core lib 被 cli 和 server 同时依赖，core 只编译一次\n");

    println!("共享的好处（以示例说明）:");
    println!("  假设 workspace 有: core, cli, server");
    println!("  cli 和 server 都依赖 core 和 serde");
    println!("  serde 在 workspace 范围内只编译一次");
    println!("  core 也只编译一次");
    println!("  如果没有 workspace: 三个独立项目各自编译 serde, 3 倍时间\n");

    println!("共享 .cargo/config.toml:");
    println!("  • workspace 根目录的 .cargo/config.toml 对所有 member 生效");
    println!("  • 统一的工具链配置、注册表设置、构建标志等\n");
}

/// 路径依赖
fn section_path_dependencies() {
    println!("═══════════════════════════════════════════════");
    println!("  五、路径依赖 (Path Dependencies)");
    println!("═══════════════════════════════════════════════\n");

    println!("路径依赖是 workspace 内部 member 之间互相引用的方式。\n");

    println!("在 cli/Cargo.toml 中依赖同 workspace 的 core:");
    println!("  ┌──────────────────────────────────────┐");
    println!("  │ [dependencies]                       │");
    println!("  │ core = {{ path = \"../core\" }}          │");
    println!("  │ utils = {{ path = \"../libs/utils\" }}   │");
    println!("  └──────────────────────────────────────┘\n");

    println!("路径依赖的特点:");
    println!("  • 使用相对路径，便于项目整体移动");
    println!("  • Cargo 自动识别 workspace 内的路径依赖");
    println!("  • 不经过 crates.io 注册表，直接从本地源码编译");
    println!("  • 修改立即生效，无需 cargo publish");
    println!("  • pub API 变更会导致依赖方重新编译");
    println!("  • 可以使用 version 字段指定兼容版本号");
    println!("    core = {{ path = \"../core\", version = \"0.1\" }}\n");

    println!("路径依赖的版本匹配:");
    println!("  当指定 version 时，Cargo 会检查本地 crate 的版本是否");
    println!("  满足版本约束。如果本地 core 的版本是 0.1.5，而 cli 要求");
    println!("  core = {{ path = \"../core\", version = \"0.2\" }}，构建会失败。\n");

    println!("这在你准备将内部 crate 发布到 crates.io 时特别有用：");
    println!("发布后，外部使用者通过版本号获取你的 crate，而 workspace");
    println!("内部仍然使用路径依赖以便快速迭代。\n");
}

/// resolver = "3"
fn section_resolver_v3() {
    println!("═══════════════════════════════════════════════");
    println!("  六、Resolver = \"3\"");
    println!("═══════════════════════════════════════════════\n");

    println!("Resolver 是 Cargo 的依赖解析策略。Rust 2024 edition 默认");
    println!("使用 resolver = \"3\"（也称为 v3 resolver）。\n");

    println!("Resolver 版本历史:");
    println!("  • v1 (resolver = \"1\"): 最初的解析器，2018 edition 及之前");
    println!("  • v2 (resolver = \"2\"): 2021 edition 默认");
    println!("  • v3 (resolver = \"3\"): 2024 edition 默认\n");

    println!("v3 resolver 的关键改进:");
    println!("  1. Feature 统一 (Feature Unification)");
    println!("     同一个包在 workspace 范围内只解析一次");
    println!("     所有 member 对同一个依赖启用的 features 取并集");
    println!("     \n");
    println!("  2. 平台特定依赖");
    println!("     [target.'cfg(windows)'.dependencies] 仅在对应平台解析");
    println!("     避免了不需要的平台依赖被拉入 Cargo.lock");
    println!("     \n");
    println!("  3. 更好的可选依赖处理");
    println!("     optional = true 的依赖只在被 feature 启用时才解析\n");

    println!("在 workspace 中使用:");
    println!("  [workspace]");
    println!("  resolver = \"3\"  # 推荐，与 Rust 2024 edition 一致\n");

    println!("迁移注意事项:");
    println!("  从 v1 → v2/v3 可能导致某些之前\"恰好能编译\"的代码出现");
    println!("  feature 缺失的错误。这是因为 v2/v3 不再自动为 dev-dependencies");
    println!("  合并 features。遇到这类问题需要显式声明 features。\n");
}

/// workspace.dependencies
fn section_workspace_dependencies() {
    println!("═══════════════════════════════════════════════");
    println!("  七、Workspace 级别依赖 (workspace.dependencies)");
    println!("═══════════════════════════════════════════════\n");

    println!("这是 Rust 2024 edition 的重要特性，允许在 workspace 根");
    println!("集中管理所有依赖的版本和 feature 设置。\n");

    println!("根 Cargo.toml:");
    println!("  ┌────────────────────────────────────────────┐");
    println!("  │ [workspace.dependencies]                   │");
    println!("  │ serde = {{ version = \"1\", features = [\"derive\"] }}");
    println!("  │ tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!("  │ uuid = \"1\"                                 │");
    println!("  │ rand = \"0.9\"                               │");
    println!("  │ thiserror = \"2\"                            │");
    println!("  └────────────────────────────────────────────┘\n");

    println!("各 member 中引用:");
    println!("  ┌────────────────────────────────────────────┐");
    println!("  │ [dependencies]                             │");
    println!("  │ serde = {{ workspace = true }}               │");
    println!("  │ tokio = {{ workspace = true }}  # 继承完整配置│");
    println!("  │ uuid.workspace = true       # 简洁写法     │");
    println!("  │                                             │");
    println!("  │ # 可以覆盖 features                         │");
    println!("  │ tokio = {{ workspace = true, features = [\"rt\"] }}");
    println!("  └────────────────────────────────────────────┘\n");

    println!("优点:");
    println!("  • 集中管理版本号，避免各 member 版本不一致");
    println!("  • 修改版本时只需改一处");
    println!("  • 新 member 加入时减少重复配置");
    println!("  • CI 中的依赖审计更简单\n");
}

/// 典型模式
fn section_typical_patterns() {
    println!("═══════════════════════════════════════════════");
    println!("  八、典型 Workspace 模式");
    println!("═══════════════════════════════════════════════\n");

    println!("模式一: 核心库 + CLI 应用");
    println!("  适用于: 将核心逻辑和用户界面分离\n");
    println!("  结构:");
    println!("    my-tool/");
    println!("    ├── core/            # 核心库 (lib crate)");
    println!("    ├── cli/             # CLI 应用 (bin crate)");
    println!("    └── gui/             # GUI 应用 (bin crate, 可选)\n");

    println!("模式二: Monorepo 服务集合");
    println!("  适用于: 包含多个微服务的后端项目\n");
    println!("  结构:");
    println!("    backend/");
    println!("    ├── common/          # 共享的 types, utils");
    println!("    ├── user-service/    # 用户服务");
    println!("    ├── order-service/   # 订单服务");
    println!("    ├── gateway/         # API 网关");
    println!("    └── migration/       # 数据库迁移工具\n");

    println!("模式三: 库 + 示例 + 基准测试");
    println!("  适用于: 公共 crate 开发\n");
    println!("  结构:");
    println!("    my-lib/");
    println!("    ├── my-lib-core/     # 核心实现");
    println!("    ├── my-lib-derive/   # proc-macro crate");
    println!("    ├── examples/        # 示例应用");
    println!("    └── benches/         # 基准测试 (使用 criterion)\n");

    println!("模式四: 本教程的结构");
    println!("  虚拟 workspace + 章节式组织:");
    println!("    rust-from-python/");
    println!("    └── chapters/");
    println!("        ├── 01_basics/");
    println!("        ├── 02_ownership/");
    println!("        └── ...\n");
}

/// 为什么本教程用 workspace
fn section_why_this_tutorial_uses_workspace() {
    println!("═══════════════════════════════════════════════");
    println!("  九、为什么本教程使用 Workspace");
    println!("═══════════════════════════════════════════════\n");

    println!("《Rust from Python》教材使用虚拟 workspace 组织各章节:\n");

    println!("  优点:");
    println!("  1. 统一的依赖管理：所有章节共享 serde, tokio 等库的版本");
    println!("  2. 共享编译缓存：serde 只编译一次，各章节直接复用");
    println!("  3. 一条命令验证全部: cargo check --workspace 检查所有章节");
    println!("  4. 统一的工具配置: .cargo/config.toml 全局生效");
    println!("  5. 清晰的目录结构: 每章一个独立 package\n");

    println!("  挑战:");
    println!("  1. 章节之间不能创建嵌套 workspace");
    println!("     （这就是本章作为展示包而非真实 workspace 的原因）");
    println!("  2. 每个章节的 Cargo.toml 必须声明自己的依赖");
    println!("     （即使版本由 workspace.dependencies 统一管理）");
    println!("  3. 大型 workspace 的 cargo check 时间较长\n");
}

/// 什么时候不要用 workspace
fn section_when_not_to_use_workspace() {
    println!("═══════════════════════════════════════════════");
    println!("  十、什么时候不要用 Workspace");
    println!("═══════════════════════════════════════════════\n");

    println!("Workspace 并非银弹，以下场景不适合使用:\n");

    println!("  1. 完全独立的项目");
    println!("     如果几个 crate 之间没有路径依赖关系");
    println!("     （不互相引用），各自独立开发更好\n");

    println!("  2. 版本迭代速度差异大");
    println!("     如果 core 库非常稳定（3 个月一版），而 cli 日更迭代,");
    println!("     放在同一 workspace 可能导致不必要的重编译\n");

    println!("  3. 不相关的技术栈");
    println!("     一个 Rust 项目 + 一个 Python 项目的 monorepo 不适合");
    println!("     用 Cargo workspace 管理（Python 部分无法参与）\n");

    println!("  4. 小型单 crate 项目");
    println!("     只有一个 crate 的项目不需要 workspace。");
    println!("     等到需要拆分时再迁移也不迟\n");

    println!("  5. 需要不同 Rust edition 的 crate");
    println!("     虽然 Cargo 允许同一个 workspace 中混用 edition,");
    println!("     但可能带来混淆和兼容性问题\n");

    println!("  6. 发布到不同注册表");
    println!("     如果不同的 crate 需要发布到不同的注册表（如内网私有");
    println!("     crates.io vs 公共 crates.io），管理会更复杂\n");
}

/// ASCII 结构图
fn section_ascii_diagram() {
    println!("═══════════════════════════════════════════════");
    println!("  十一、Workspace 结构全景图");
    println!("═══════════════════════════════════════════════\n");

    println!("  一个完整的 workspace 文件布局:\n");

    println!("  rust-from-python/                    ← Git 仓库根");
    println!("  ┌──────────────────────────────────────────────┐");
    println!("  │ .cargo/                                      │");
    println!("  │ └── config.toml              ← 全局 Cargo 配置│");
    println!("  │ .gitignore                                   │");
    println!("  │ Cargo.toml                   ← [workspace]   │");
    println!("  │ Cargo.lock                   ← 共享 lock     │");
    println!("  │ target/                      ← 共享编译输出  │");
    println!("  │                                              │");
    println!("  │ chapters/                                    │");
    println!("  │ ├── 01_basics/               ← member 1      │");
    println!("  │ │   ├── Cargo.toml                           │");
    println!("  │ │   └── src/main.rs                          │");
    println!("  │ ├── 02_ownership/            ← member 2      │");
    println!("  │ │   ├── Cargo.toml                           │");
    println!("  │ │   └── src/main.rs                          │");
    println!("  │ └── ...                      ← member ...    │");
    println!("  │                                              │");
    println!("  │ scripts/                     ← 辅助脚本      │");
    println!("  │ └── check_all.sh                             │");
    println!("  └──────────────────────────────────────────────┘\n");

    println!("  依赖关系图（逻辑层面）:\n");
    println!("  ┌─────────┐");
    println!("  │  serde  │ ←─── workspace.dependencies 统一管理");
    println!("  │  tokio  │      版本，各 chapter 按需使用");
    println!("  │  rand   │");
    println!("  └─────────┘\n");

    println!("  ┌──────────┐      ┌──────────┐      ┌──────────┐");
    println!("  │ Chapter  │      │ Chapter  │      │ Chapter  │");
    println!("  │   01     │      │   02     │      │   03     │");
    println!("  │ (独立)   │      │ (独立)   │      │ (独立)   │");
    println!("  └──────────┘      └──────────┘      └──────────┘");
    println!("       ↑                 ↑                 ↑");
    println!("       └─────────────────┼─────────────────┘");
    println!("                         │");
    println!("              共享编译缓存 target/");
    println!("              共享依赖锁 Cargo.lock");
    println!("              共享 Cargo 配置\n");
}

/// 常用命令
fn section_commands() {
    println!("═══════════════════════════════════════════════");
    println!("  十二、Workspace 常用命令");
    println!("═══════════════════════════════════════════════\n");

    println!("  # 构建所有 member");
    println!("  cargo build --workspace\n");

    println!("  # 仅构建特定 member");
    println!("  cargo build -p chapter_01_basics\n");

    println!("  # 运行特定 member 的测试");
    println!("  cargo test -p chapter_02_ownership\n");

    println!("  # 检查所有 member 是否能通过编译");
    println!("  cargo check --workspace\n");

    println!("  # 对所有 member 运行 clippy");
    println!("  cargo clippy --workspace\n");

    println!("  # 格式化所有 member");
    println!("  cargo fmt --all\n");

    println!("  # 列出所有 member");
    println!("  cargo metadata --no-deps | grep '\"name\"'\n");

    println!("  # 添加新的 member 到 workspace");
    println!("  # 1. 创建新目录和 Cargo.toml");
    println!("  # 2. 在根 Cargo.toml 的 [workspace] members 中添加路径");
    println!("  # 3. cargo check -p <new-member> 验证\n");

    println!("\n═══════════════════════════════════════════════");
    println!("  Workspace 架构总结:");
    println!("  • 虚拟 workspace → 根不产出 crate，纯管理角色");
    println!("  • 共享 Cargo.lock + target/ → 构建性能和一致性");
    println!("  • 路径依赖 → 内部 crate 之间的桥接方式");
    println!("  • workspace.dependencies → 集中版本管理");
    println!("  • resolver = \"3\" → 现代化依赖解析策略");
    println!("═══════════════════════════════════════════════");
}
