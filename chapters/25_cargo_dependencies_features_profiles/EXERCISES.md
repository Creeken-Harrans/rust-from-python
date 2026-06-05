# 练习: Cargo 工程能力

## 基础练习

### 练习 1: 理解 Feature 系统 (15 分钟)

**目标**: 通过条件编译观察不同 feature 组合下的行为变化。

**步骤**:
1. 阅读 `src/main.rs` 中 `#[cfg(feature = "...")]` 的用法
2. 运行 `cargo run` 观察默认 feature 组合的输出
3. 运行 `cargo run --features advanced` 观察输出差异
4. 运行 `cargo run --all-features` 观察所有 feature 启用时的输出
5. 运行 `cargo run --no-default-features` 观察基本 feature 都被禁用的输出

**思考题**:
- 为什么启用 `advanced` 时 `basic` 也自动启用了？
- 如果删除 `advanced = ["basic"]`，运行 `cargo run --features advanced --no-default-features` 会发生什么？
- 在什么场景下你会使用 `--no-default-features`？

### 练习 2: 添加自定义 Feature (20 分钟)

**目标**: 在现有项目中添加一个新的 feature 并使用它。

**步骤**:
1. 在 `Cargo.toml` 中添加一个新的 feature `pretty_print`，依赖 `json_output`
2. 在 `src/main.rs` 中添加条件编译的函数:
   ```rust
   #[cfg(feature = "pretty_print")]
   fn pretty_output() {
       println!("美化输出模式已启用");
   }
   ```
3. 在 `main()` 中调用该函数（使用 `#[cfg]` 保护）
4. 运行 `cargo run --features pretty_print` 验证

**思考题**:
- 运行 `cargo run --features pretty_print` 时，`serde` 和 `serde_json` 是否被引入？为什么？
- 如果 `pretty_print` 不依赖 `json_output`，改为独立 feature，会对行为产生什么影响？

### 练习 3: Profile 对比实验 (15 分钟)

**目标**: 亲身体验不同 Profile 对编译和运行的影响。

**步骤**:
1. 计算 Fibonacci 数列（例如 F(40) = 102334155），分别用递归和迭代实现
2. 使用 `std::time::Instant` 测量运行时间
3. 编译 Release 版本: `cargo build --release`
4. 运行并对比 dev 和 release 版本的运行时间
5. 检查编译产物的体积差异:
   ```bash
   ls -lh target/debug/cargo_features
   ls -lh target/release/cargo_features
   ```

**思考题**:
- Release 版本比 Debug 版本快了多少倍？为什么？
- 二进制文件体积的差异主要来自哪里？
- 什么样的代码优化效果最明显（计算密集型 vs IO 密集型）？

## 进阶练习

### 练习 4: 分析依赖树 (20 分钟)

**目标**: 深入理解项目依赖图的结构。

**步骤**:
1. 运行 `cargo tree` 查看基本依赖树
2. 运行 `cargo tree --all-features` 对比差异
3. 运行 `cargo tree --edges features` 理解 feature 如何影响依赖
4. 运行 `cargo metadata | jq '.packages[] | {name, version}'` 列出所有包
5. 运行 `cargo tree --invert -p serde` 观察谁依赖于 serde

**思考题**:
- 启用 `json_output` feature 后，依赖树增加了哪些包？
- 如果项目中有两个依赖各自依赖不同版本的同一个包，Cargo 如何处理？使用 `cargo tree --duplicates` 查看。

### 练习 5: 分析 Feature 依赖链（笔试题）

**背景**:
```toml
[features]
default = ["f1"]
f1 = ["f2"]
f2 = ["f3"]
f3 = []
extra = ["f1", "f2", "f3"]
```

**问题**:
1. 运行 `cargo run` 时，哪些 feature 被激活？
2. 运行 `cargo run --features extra --no-default-features` 时，哪些 feature 被激活？
3. 如果 `f3` 依赖一个可选依赖 `serde = { version = "1", optional = true }`，运行 `cargo run` 时 `serde` 是否会被引入？为什么？
4. 假设 `f3` 又依赖 `f1`，会形成循环依赖。Cargo 会如何处理这种情况？

**思考题**:
- Feature 依赖关系必须是 DAG（有向无环图）吗？为什么？

### 练习 6: 设计一个 Feature 架构 (30 分钟)

**目标**: 为一个虚拟的"数据处理库"设计合理的 feature 架构。

**需求描述**:
- 核心功能：读取/写入 CSV 文件
- 高级功能：读取/写入 JSON 文件（需要 serde, serde_json）
- 高级功能：读取/写入 Parquet 文件（需要 parquet, arrow）
- 网络功能：从 HTTP URL 读取数据（需要 reqwest）
- 压缩支持：支持 gzip 和 zstd 压缩（需要 flate2, zstd）

**任务**:
1. 设计 Cargo.toml 的 `[features]` 部分
2. 解释哪些 feature 应该互斥、哪个应该是 default
3. 画出 feature 依赖关系图
4. 说明用户如何使用你的设计：
   - "我只想用 CSV" → 运行哪些 features
   - "我需要全部功能" → 运行哪些 features
   - "我只需要 JSON + HTTP" → 运行哪些 features

## 综合练习

### 练习 7: 构建你自己的 Cargo 子命令

**目标**: 了解 Cargo 工具的扩展机制。

**步骤**:
1. 研究 `cargo-edit`、`cargo-watch` 等子命令是如何工作的
2. 设计一个简单的 Cargo 子命令概念（如 `cargo analyze` 分析项目依赖）
3. 描述它将如何解析 `Cargo.toml`、如何展示信息

**提示**: Cargo 子命令本质上就是放在 `$PATH` 中的 `cargo-<name>` 命名的可执行文件。

**思考题**:
- 你的工具如何集成到现有的 Cargo 工作流中？
- 它能为开发者提供哪些 `cargo metadata` 或 `cargo tree` 无法直接提供的信息？

### 练习 8: 阅读与批判

**任务**: 选择三个流行的 Rust crate（如 `tokio`、`reqwest`、`clap`），阅读它们的 Cargo.toml 和 feature 设计。

**问题**:
1. 每个 crate 有多少 features？哪些是 default？
2. feature 的命名和分组策略是什么？
3. 你是否同意它们的 feature 设计？有没有可以改进的地方？
4. 从这些 crate 的设计中学到了什么可以应用到自己的项目中？

**参考 crate**:
- `tokio`: 异步运行时，feature 设计是 Rust 生态的典范
- `reqwest`: HTTP 客户端，TLS 后端的 feature 选择机制值得学习
- `clap`: CLI 框架，其 derive feature 展示了如何组织解析功能
- `serde`: 序列化框架，derive feature 是可选依赖的经典案例

## 提交要求

- 练习 1-3 为基础必做，完成后提交代码和运行输出
- 练习 4-6 为进阶选做（建议至少完成 1 个）
- 练习 7-8 为思考题，提交分析报告即可
- 所有练习的输出和代码整理到一个目录中提交
