#![allow(rustdoc::invalid_rust_codeblocks, clippy::approx_constant)]
//
// 代码质量 - Lint、格式化、文档与 CI
//
// 本文件的目标:
//   - 编写干净、符合 Rust 惯例的代码
//   - 通过 cargo fmt --check (格式化)
//   - 通过 cargo clippy -- -D warnings (零警告)
//   - 所有公共函数都有 /// 文档注释
//   - 演示标准的 CI 工作流
//
// 代码质量工具速查:
//   cargo fmt              # 自动格式化代码
//   cargo fmt -- --check   # 检查格式（不修改文件）
//   cargo clippy           # 运行 linter
//   cargo clippy -- -D warnings  # 将所有警告视为错误
//   cargo check            # 快速检查能否通过编译（不生成二进制）
//   cargo test             # 运行测试
//   cargo doc --no-deps    # 生成文档
//   cargo doc --open       # 生成并在浏览器中打开文档
//

/// 自然对数的底 e 的近似值。
/// 用于数学计算中的常数。
const E: f64 = std::f64::consts::E;

/// 黄金比例的近似值，出现在自然界和艺术中。
const GOLDEN_RATIO: f64 = 1.618_033_988;

/// 计算 n 的阶乘 (n!)，使用迭代实现以避免栈溢出。
///
/// # 参数
/// - `n`: 非负整数，表示要计算阶乘的数。
///   对于 n > 20，结果会超过 u64 的范围。
///
/// # 返回值
/// - n! 的值。0! = 1。
///
/// # 示例
///
/// ```
/// // 由于本函数不在 lib crate 中，无法在 doc-test 中导入。
/// // 这里仅作为文档示例:
/// // assert_eq!(calculate_factorial(5), 120);
/// // assert_eq!(calculate_factorial(0), 1);
/// ```
///
/// # 复杂度
/// - 时间复杂度: O(n)
/// - 空间复杂度: O(1)
///
/// # Panics
/// 本函数不 panic。对于会导致溢出的输入，使用饱和运算。
fn calculate_factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut result: u64 = 1;
    for i in 2..=n {
        result = result.saturating_mul(i);
    }
    result
}

/// 判断一个数是否为质数。
///
/// 使用优化的试除法：
/// - 2 和 3 是质数
/// - 小于 2 的数不是质数
/// - 能被 2 或 3 整除的大于 3 的数不是质数
/// - 其余情况检查 6k ± 1 形式的因子
///
/// # 参数
/// - `n`: 要判断的整数
///
/// # 返回值
/// - `true` 如果 n 是质数
/// - `false` 如果 n 不是质数
///
/// # 复杂度
/// - 时间复杂度: O(sqrt(n))
/// - 空间复杂度: O(1)
///
/// # 示例
///
/// ```
/// // assert!(is_prime(2));
/// // assert!(is_prime(17));
/// // assert!(!is_prime(4));
/// // assert!(!is_prime(1));
/// ```
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    let mut i: u64 = 5;
    while i * i <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}

/// 生成小于等于 limit 的所有质数。
///
/// 使用埃拉托色尼筛法 (Sieve of Eratosthenes) 实现，
/// 这是一种高效的批量质数生成算法。
///
/// # 参数
/// - `limit`: 上限值（包含），质数 <= limit
///
/// # 返回值
/// - `Vec<u64>`: 包含所有满足 p <= limit 的质数的向量
///
/// # 复杂度
/// - 时间复杂度: O(n log(log n))
/// - 空间复杂度: O(n)
///
/// # 示例
///
/// ```
/// // assert_eq!(generate_primes(10), vec![2, 3, 5, 7]);
/// // assert_eq!(generate_primes(2), vec![2]);
/// // assert!(generate_primes(1).is_empty());
/// ```
fn generate_primes(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }

    let limit_usize = limit as usize;
    let mut is_prime_mask = vec![true; limit_usize + 1];
    is_prime_mask[0] = false;
    is_prime_mask[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime_mask[i] {
            let mut multiple = i * i;
            while multiple <= limit_usize {
                is_prime_mask[multiple] = false;
                multiple += i;
            }
        }
    }

    is_prime_mask
        .iter()
        .enumerate()
        .filter(|(_, is_p)| **is_p)
        .map(|(idx, _)| idx as u64)
        .collect()
}

/// 打印数学计算相关的分隔标题。
///
/// 将输出格式化为带有分隔线的标题块，提高可读性。
fn print_section_header(title: &str) {
    let width: usize = 55;
    let padding = (width.saturating_sub(title.len())) / 2;
    println!("\n{:=<width$}", "");
    println!("{:>padding$}{}", "", title,);
    println!("{:=<width$}", "");
}

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║   代码质量: Lint · 格式化 · 文档 · CI                ║");
    println!("╚══════════════════════════════════════════════════════╝");

    // =========================================================================
    // 数学函数演示
    // =========================================================================
    print_section_header("数学函数演示");

    // 阶乘计算
    let numbers = [0, 1, 5, 10, 15, 20];
    println!("\n阶乘 (n!):");
    for &n in &numbers {
        let fact = calculate_factorial(n);
        println!("  {:>2}! = {}", n, fact);
    }

    // 质数判断
    let test_numbers = [1, 2, 3, 4, 17, 97, 100, 7919];
    println!("\n质数判断:");
    for &n in &test_numbers {
        let prime_marker = if is_prime(n) {
            "✓ 质数"
        } else {
            "✗ 非质数"
        };
        println!("  {:>5}: {}", n, prime_marker);
    }

    // 质数生成
    let limits = [10, 50, 100];
    println!("\n质数生成 (埃拉托色尼筛法):");
    for &limit in &limits {
        let primes = generate_primes(limit);
        println!(
            "  小于等于 {} 的质数共 {} 个: {:?}",
            limit,
            primes.len(),
            primes
        );
    }

    // =========================================================================
    // 数学常数
    // =========================================================================
    print_section_header("数学常数");
    println!("\n  e (自然对数的底)     = {:.10}", E);
    println!("  φ (黄金比例)         = {:.10}", GOLDEN_RATIO);
    println!("  e + 1/e              = {:.10}", E + 1.0 / E);
    println!(
        "  φ^2                  = {:.10}",
        GOLDEN_RATIO * GOLDEN_RATIO
    );
    println!(
        "  φ - 1/φ              = {:.10}",
        GOLDEN_RATIO - 1.0 / GOLDEN_RATIO
    );

    // =========================================================================
    // 代码质量工具说明
    // =========================================================================
    print_section_header("Rust 代码质量工具链");

    println!();
    println!("  ┌─────────────────────────────────────────────────────┐");
    println!("  │ 工具          │ 功能              │ 命令              │");
    println!("  ├─────────────────────────────────────────────────────┤");
    println!("  │ cargo fmt     │ 代码格式化         │ cargo fmt         │");
    println!("  │ cargo fmt     │ 格式化检查         │ cargo fmt --check │");
    println!("  │ cargo clippy  │ 静态分析/Linter    │ cargo clippy      │");
    println!("  │ cargo check   │ 类型检查(无产物)   │ cargo check       │");
    println!("  │ cargo test    │ 运行测试           │ cargo test        │");
    println!("  │ cargo doc     │ 生成文档           │ cargo doc --no-deps│");
    println!("  │ cargo fix     │ 自动修复           │ cargo fix --edition-idioms│");
    println!("  │ cargo audit   │ 安全漏洞检查       │ cargo audit       │");
    println!("  └─────────────────────────────────────────────────────┘");

    // =========================================================================
    // Clippy Lint 级别说明
    // =========================================================================
    print_section_header("Clippy Lint 级别");

    println!();
    println!("  Rust 编译器将 lint 分为四个级别:");
    println!();
    println!("  allow    - 允许 (抑制该 lint, 不报告)");
    println!("  warn     - 警告 (报告但不阻止编译, 默认级别)");
    println!("  deny     - 拒绝 (报告并阻止编译)");
    println!("  forbid   - 禁止 (与 deny 相同, 但不能被覆盖)");
    println!();
    println!("  在 CI 中推荐的配置:");
    println!("    cargo clippy -- -D warnings");
    println!("    (将所有警告提升为错误，确保零警告通过)");
    println!();
    println!("  在代码中使用属性控制 lint:");
    println!("    #[allow(dead_code)]             // 禁止'未使用代码'警告");
    println!("    #[allow(clippy::needless_return)]  // 针对特定的 clippy lint");
    println!("    #![deny(missing_docs)]          // 要求所有公开项有文档");
    println!();
    println!("  常用的 clippy lint 组:");
    println!("    clippy::pedantic    - 严格的惯用法检查");
    println!("    clippy::nursery     - 实验性 lint");
    println!("    clippy::cargo       - Cargo.toml 检查");
    println!("    clippy::perf        - 性能相关 lint");
    println!("    clippy::complexity  - 复杂度相关 lint");

    // =========================================================================
    // CI 工作流说明
    // =========================================================================
    print_section_header("CI (持续集成) 工作流");

    println!();
    println!("  推荐的 CI Pipeline 步骤:");
    println!();
    println!("  1. cargo fmt --all -- --check");
    println!("     └─ 确保所有代码格式一致");
    println!();
    println!("  2. cargo check --workspace --all-targets");
    println!("     └─ 快速检查编译，不生成二进制（比 build 快）");
    println!();
    println!("  3. cargo test --workspace");
    println!("     └─ 运行所有测试（包括 doc-tests）");
    println!();
    println!("  4. cargo clippy --workspace --all-targets --all-features -- -D warnings");
    println!("     └─ 严格的静态分析，零警告要求");
    println!();
    println!("  5. RUSTDOCFLAGS=\"-D warnings\" cargo doc --workspace --no-deps");
    println!("     └─ 生成文档，确保文档中的代码示例能编译");
    println!();
    println!("  GitHub Actions 配置文件位于:");
    println!("    .github/workflows/rust.yml");
    println!();
    println!("  该工作流在以下事件触发:");
    println!("    - push to main 分支");
    println!("    - Pull Request to main 分支");

    // =========================================================================
    // 为什么这些工具不是可选的
    // =========================================================================
    print_section_header("代码质量工具的必要性");

    println!();
    println!("  为什么格式化 (fmt) 不是可选的:");
    println!("    • 统一的代码风格降低认知负担");
    println!("    • Code Review 时只关注逻辑，不争论格式");
    println!("    • 新人 onboarding 更快，代码风格一致");
    println!();
    println!("  为什么 Lint (clippy) 不是可选的:");
    println!("    • 在编译时捕获常见错误模式");
    println!("    • 引导开发者使用更地道的 Rust 写法");
    println!("    • 预防性能陷阱和不安全的模式");
    println!();
    println!("  为什么测试 (test) 不是可选的:");
    println!("    • 验证代码行为符合预期");
    println!("    • 作为重构的安全网");
    println!("    • 文档化的使用示例 (doc-tests)");
    println!();
    println!("  为什么文档 (doc) 不是可选的:");
    println!("    • 良好的文档是 API 可用性的基础");
    println!("    • cargo doc 免费生成，只需写注释");
    println!("    • doc-tests 确保文档示例始终有效");

    // =========================================================================
    // 总结
    // =========================================================================
    print_section_header("总结");

    println!();
    println!("  Rust 的代码质量工具链是业界最完善的内置工具链之一:");
    println!();
    println!("    cargo fmt      # Python 的 black / ruff format");
    println!("    cargo clippy   # Python 的 pylint / ruff");
    println!("    cargo check    # Python 的 mypy (类型检查)");
    println!("    cargo test     # Python 的 pytest");
    println!("    cargo doc      # Python 的 sphinx / pdoc");
    println!();
    println!("  将这些工具集成到 CI 流水线中，确保:");
    println!("    • 每次 PR 自动运行格式、lint、类型、测试检查");
    println!("    • 问题在合并前被捕获，不进入 main 分支");
    println!("    • 团队遵守统一的代码质量标准");
    println!();
    println!("  ╔══════════════════════════════════════════════════╗");
    println!("  ║  良好的代码质量不是偶然的，而是工程化的结果。    ║");
    println!("  ╚══════════════════════════════════════════════════╝");
}
