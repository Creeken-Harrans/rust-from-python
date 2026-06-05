//
// Cargo 工程能力 - 依赖、Feature 与 Profile
//
// 构建命令:
//   cargo run                          # 使用默认 features (basic)
//   cargo run --features advanced       # 启用 advanced (同时启用 basic)
//   cargo run --all-features            # 启用所有 features
//   cargo run --no-default-features     # 禁用默认 features
//
// 依赖树分析:
//   cargo tree                          # 查看完整依赖树
//   cargo tree --features json_output   # 查看特定 feature 的依赖树
//   cargo metadata                      # 输出 JSON 格式的项目元数据
//
// Profile 差异:
//   cargo run                           # 使用 dev profile (opt-level=0, debug=true)
//   cargo run --release                 # 使用 release profile (opt-level=3, debug=false)
//   cargo build --profile release       # 等价于 --release
//

// ============================================================================
// 数据结构 - 展示 serde 可选依赖的使用
// ============================================================================

/// 当 `json_output` feature 启用时，本结构体可通过 serde 序列化/反序列化
#[cfg(feature = "json_output")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct FeatureReport {
    basic_enabled: bool,
    advanced_enabled: bool,
    json_output_enabled: bool,
    compile_time: String,
}

#[cfg(feature = "json_output")]
impl FeatureReport {
    fn new(basic_enabled: bool, advanced_enabled: bool, json_output_enabled: bool) -> Self {
        Self {
            basic_enabled,
            advanced_enabled,
            json_output_enabled,
            compile_time: "2026-06-05".to_string(),
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("序列化失败: {}", e))
    }
}

// 当 json_output feature 未启用时，提供一个简化的输出
#[cfg(not(feature = "json_output"))]
struct FeatureReport {}

#[cfg(not(feature = "json_output"))]
impl FeatureReport {
    fn to_json(&self) -> String {
        "JSON 输出功能未启用。请使用: cargo run --features json_output".to_string()
    }
}

// ============================================================================
// Feature 演示函数
// ============================================================================

/// 输出当前编译时启用的 features 信息
fn demonstrate_features() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│        Cargo Features 状态报告               │");
    println!("├─────────────────────────────────────────────┤");

    // basic feature: 默认启用的基础功能
    #[cfg(feature = "basic")]
    {
        println!("│ [✓] basic    - 基础功能 (默认启用)            │");
    }
    #[cfg(not(feature = "basic"))]
    {
        println!("│ [✗] basic    - 基础功能 (未启用)              │");
    }

    // advanced feature: 依赖 basic，因此启用 advanced 时 basic 也启用
    #[cfg(feature = "advanced")]
    {
        println!("│ [✓] advanced - 高级功能 (依赖 basic)          │");
    }
    #[cfg(not(feature = "advanced"))]
    {
        println!("│ [✗] advanced - 高级功能 (依赖 basic)          │");
    }

    // json_output feature: 启用 serde + serde_json 可选依赖
    #[cfg(feature = "json_output")]
    {
        println!("│ [✓] json_output - JSON 输出 (启用 serde 依赖)  │");
    }
    #[cfg(not(feature = "json_output"))]
    {
        println!("│ [✗] json_output - JSON 输出 (未启用, serde 未链接)│");
    }

    println!("├─────────────────────────────────────────────┤");

    // 列举所有启用的 features（编译时确定）
    print!("│ 启用的 features: [default: ");
    print!("basic");
    #[cfg(feature = "advanced")]
    print!(" advanced");
    #[cfg(feature = "json_output")]
    print!(" json_output");
    println!("]");
    println!("└─────────────────────────────────────────────┘\n");
}

/// 演示 basic feature 下的功能
#[cfg(feature = "basic")]
fn basic_operation() {
    println!("--- basic 功能演示 ---");
    println!("这是基础功能。当你运行 cargo run (不带任何 features 参数) 时，");
    println!("default = [\"basic\"] 确保此代码块被编译。\n");

    println!("在 Cargo.toml 中定义的 features:");
    println!("  [features]");
    println!("  default = [\"basic\"]    ← 默认启用 basic");
    println!("  basic = []              ← basic 本身不依赖其他 feature");
    println!("  advanced = [\"basic\"]   ← advanced 自动启用 basic");
    println!("  json_output = [\"serde\", \"serde_json\"]  ← 启用可选依赖\n");
}

/// 演示 advanced feature 下的功能
#[cfg(feature = "advanced")]
fn advanced_operation() {
    println!("--- advanced 功能演示 ---");
    println!("高级功能已启用。当你运行 'cargo run --features advanced' 时，");
    println!("不仅 advanced 代码被编译，basic 也自动被包含（因为 advanced 依赖 basic）。\n");

    println!("Feature 依赖链示例:");
    println!("  advanced ──→ basic");
    println!("  json_output ──→ serde + serde_json");
    println!("  这意味着启用 advanced 等同于同时启用 basic 和 advanced。\n");
}

/// 演示 JSON 输出功能
#[cfg(feature = "json_output")]
fn json_output_demo() {
    println!("--- json_output 功能演示 ---");
    println!("json_output feature 已启用，serde 和 serde_json 已链接。\n");

    let report = FeatureReport::new(
        cfg!(feature = "basic"),
        cfg!(feature = "advanced"),
        cfg!(feature = "json_output"),
    );

    println!("FeatureReport 序列化为 JSON:");
    println!("{}", report.to_json());
    println!();

    println!("在 Cargo.toml 中配置:");
    println!("  serde = {{ version = \"1\", features = [\"derive\"], optional = true }}");
    println!("  serde_json = {{ version = \"1\", optional = true }}");
    println!();
    println!("optional = true 表示默认不下载/不编译这些依赖。");
    println!("只有当 json_output feature 被启用时，它们才被引入。\n");
}

// ============================================================================
// Profile 演示
// ============================================================================

/// 解释 debug vs release profile 的差异
fn demonstrate_profiles() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│            Cargo Profiles 说明               │");
    println!("├─────────────────────────────────────────────┤");

    #[cfg(debug_assertions)]
    {
        println!("│ 当前编译模式: DEBUG (dev profile)            │");
        println!("│   opt-level = 0  → 无优化, 编译速度快         │");
        println!("│   debug = true   → 包含调试符号               │");
        println!("│   适用场景: 开发、调试、快速迭代              │");
    }
    #[cfg(not(debug_assertions))]
    {
        println!("│ 当前编译模式: RELEASE (release profile)       │");
        println!("│   opt-level = 3  → 最高级别优化               │");
        println!("│   debug = false  → 不包含调试符号              │");
        println!("│   适用场景: 生产部署、性能测试                │");
    }
    println!("├─────────────────────────────────────────────┤");
    println!("│ Cargo.toml 中的 Profile 配置:                │");
    println!("│                                              │");
    println!("│ [profile.dev]                                │");
    println!("│ opt-level = 0   # 编译速度优先                │");
    println!("│ debug = true    # 包含完整调试信息            │");
    println!("│                                              │");
    println!("│ [profile.release]                            │");
    println!("│ opt-level = 3   # 运行速度优先                │");
    println!("│ debug = false   # 不包含调试信息              │");
    println!("│                                              │");
    println!("│ 还可以自定义 Profile:                        │");
    println!("│ [profile.production]                         │");
    println!("│ inherits = \"release\"                        │");
    println!("│ opt-level = \"s\"  # 优化体积                  │");
    println!("└─────────────────────────────────────────────┘\n");
}

// ============================================================================
// main
// ============================================================================

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Cargo 工程能力: 依赖 · Feature · Profile    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // 1. 展示编译时的 features 状态
    demonstrate_features();

    // 2. 展示 basic 功能
    #[cfg(feature = "basic")]
    basic_operation();

    // 3. 展示 advanced 功能（仅在启用时编译）
    #[cfg(feature = "advanced")]
    advanced_operation();

    // 4. 展示 JSON 输出功能（仅在启用时编译）
    #[cfg(feature = "json_output")]
    json_output_demo();

    // 如果没有启用 json_output，显示提示信息
    #[cfg(not(feature = "json_output"))]
    {
        let fallback = FeatureReport {};
        println!("--- JSON 输出 ---");
        println!("{}", fallback.to_json());
        println!();
    }

    // 5. 展示 Profile 信息
    demonstrate_profiles();

    // 6. 补充工具说明
    println!("═══════════════════════════════════════════════");
    println!("  其他 Cargo 工具命令:");
    println!("═══════════════════════════════════════════════");
    println!("  cargo tree             # 查看依赖树");
    println!("  cargo tree --invert    # 查看反向依赖（谁依赖了某个包）");
    println!("  cargo tree --edges features  # 显示 feature 边");
    println!("  cargo metadata         # 输出项目元数据(JSON)");
    println!("  cargo metadata --format-version=1 | jq  # 格式化输出");
    println!("  cargo install <crate>  # 从 crates.io 安装二进制工具");
    println!("  cargo install --list   # 列出已安装的工具");
    println!();
    println!("  依赖管理要点:");
    println!("  • Cargo.lock 精确锁定依赖版本（库项目可选择不提交）");
    println!("  • 语义化版本 (SemVer): MAJOR.MINOR.PATCH");
    println!("  • ^1.2.3 允许 1.2.3 <= v < 2.0.0");
    println!("  • ~1.2.3 允许 1.2.3 <= v < 1.3.0");
    println!("  • =1.2.3 精确锁定版本");
    println!("  • path = \"../other_crate\" 路径依赖（本地开发）");
    println!("  • git = \"https://...\"  Git 依赖");
    println!("  • optional = true 使依赖变为可选的\n");

    println!("═══════════════════════════════════════════════");
    println!("  运行本示例的推荐命令:");
    println!("    cargo run");
    println!("    cargo run --features advanced");
    println!("    cargo run --all-features");
    println!("    cargo run --no-default-features");
    println!("    cargo run --release");
    println!("═══════════════════════════════════════════════");
}
