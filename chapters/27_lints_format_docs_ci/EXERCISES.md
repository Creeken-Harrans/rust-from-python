# 练习: Lint、格式化、文档与 CI

## 基础练习

### 练习 1: 代码格式化 (10 分钟)

**目标**: 体验 `cargo fmt` 的工作方式。

**步骤**:
1. 故意把 `src/main.rs` 中的一些代码改乱（添加多余空格、换行不一致等）
2. 运行 `cargo fmt` 查看自动修复效果
3. 运行 `cargo fmt -- --check` 验证格式
4. 在项目根目录创建 `rustfmt.toml` 配置文件，自定义 `max_width = 80`
5. 再次运行 `cargo fmt`，观察 80 字符宽度下的格式变化

**思考题**:
- 为什么在保存文件时自动运行 `cargo fmt` 比手动运行更好？
- 如果团队中有人使用不同的 `rustfmt.toml` 配置会发生什么？

### 练习 2: Clippy Lint 分析 (20 分钟)

**目标**: 理解 Clippy 的工作方式，学会解读其输出。

**步骤**:
1. 运行 `cargo clippy` 检查当前代码
2. 有意引入一些 Rust 反模式：
   ```rust
   // 添加到 src/main.rs 的某个位置
   fn bad_code() {
       let x = vec![1, 2, 3];
       for i in 0..x.len() {
           println!("{}", x[i]);  // Clippy 会建议使用迭代器
       }

       let y = if true { true } else { false };  // Clippy 会建议简化

       let z = String::from("hello");
       takes_string(&z);  // Clippy 会建议使用 &str
   }

   fn takes_string(s: &String) {
       println!("{}", s);
   }
   ```
3. 运行 `cargo clippy`，阅读并理解每个警告
4. 按照 Clippy 的建议修复代码
5. 运行 `cargo clippy -- -D warnings` 验证零警告

**思考题**:
- Clippy 的哪些建议是你之前不知道的？
- "零 Clippy 警告" 作为 CI 的质量门槛是否合理？什么情况下应该允许某些 lint？

### 练习 3: 文档生成 (15 分钟)

**目标**: 学习 `cargo doc` 和文档测试。

**步骤**:
1. 运行 `cargo doc --no-deps` 生成本章的文档
2. 在浏览器中打开 `target/doc/lints_and_ci/index.html`
3. 观察文档注释 `///` 是如何转换为 HTML 的
4. 为 `calculate_factorial` 函数添加一个文档测试（doc-test）:
   ```rust
   /// ```
   /// // 在 main.rs 中，你需要将测试函数设为 pub 或通过某种方式访问
   /// assert_eq!(calculate_factorial(5), 120);
   /// ```
   ```
5. 运行 `cargo test --doc` 验证文档测试被执行
6. 尝试在文档中写一个有错误的代码示例，观察测试失败信息

**思考题**:
- 文档测试和单元测试的区别是什么？
- 什么类型的 API 最适合文档测试？
- 文档测试有什么局限性？

## 进阶练习

### 练习 4: 配置项目级 Lint 规则 (20 分钟)

**目标**: 学习如何在项目中配置 lint 规则。

**步骤**:
1. 在 `src/main.rs` 顶部添加以下属性:
   ```rust
   #![deny(missing_docs)]
   ```
2. 运行 `cargo clippy` 观察结果
3. 为所有函数添加文档注释，满足 `missing_docs` 的要求
4. 试添加不同的 lint 属性，理解每个的效果:
   - `#![deny(clippy::unwrap_used)]`
   - `#![warn(clippy::pedantic)]`
   - `#![allow(clippy::too_many_lines)]`

**思考题**:
- `deny` 和 `forbid` 的区别是什么？在库代码中哪个更合适？
- 如果上游库使用了 `forbid(unsafe_code)`，下游使用者能否覆盖？

### 练习 5: 编写 GitHub Actions Workflow (20 分钟)

**目标**: 理解 CI 配置文件的结构和逻辑。

**步骤**:
1. 阅读 `.github/workflows/rust.yml` 文件
2. 画出 CI 流程的流程图（可以用文字或 ASCII 图）
3. 在现有的 workflow 中添加一个 job:
   - Job 名称: "Security Audit"
   - 运行 `cargo audit`（需要先安装）
   - 仅在 `push` 到 `main` 时运行
4. 将 Clippy 和 Test 拆分为独立的 job（并行运行）

**思考题**:
- 为什么 CI 中要将不同检查步骤拆分为不同的 job？
- `dtolnay/rust-toolchain@stable` 和 `actions-rs/toolchain@v1` 有什么区别？为什么推荐前者？
- 如何利用 GitHub Actions 缓存来加速 CI？

### 练习 6: 代码审查清单 (15 分钟)

**目标**: 创建一个开发团队可用的代码审查清单。

**任务**:
编写一个 PR Review 检查清单（中文），至少包含以下类别：
- 格式化检查
- 代码风格
- 性能考量
- 安全性
- 文档完整性
- 测试覆盖

**输出格式示例**:
```
## PR 审查清单

### 自动化检查（必须全部通过）
- [ ] CI 流水线全部通过
- [ ] cargo fmt --check 通过
- [ ] cargo clippy -- -D warnings 通过

### 人工审查
- [ ] 新增的公开 API 有文档注释
- [ ] 错误处理是否正确（避免无信息的 unwrap）
...
```

## 综合练习

### 练习 7: 修复一个"问题"项目 (30 分钟)

**目标**: 综合运用所有代码质量工具。

**任务**: 创建一个包含以下"问题"的微型 Rust 项目，然后用本章的工具修复它:

1. 代码格式不一致（缩进混用、行宽不一致）
2. 包含 Clippy 会警告的常见反模式（至少 3 种）
3. 缺少文档注释
4. 有会被 `cargo test` 捕获的逻辑错误
5. 存在未使用的变量/函数

**步骤**:
1. 创建项目结构（可以在单独的目录中）
2. 运行 `cargo fmt` 修复格式
3. 运行 `cargo clippy` 并修复所有警告
4. 添加文档注释
5. 编写测试捕获并修复逻辑错误
6. 确认所有工具零问题通过

### 练习 8: 设计完整的 CI/CD Pipeline (25 分钟)

**目标**: 为一个真实的 Rust 项目设计 CI/CD 流水线。

**项目背景**: 
- 一个 workspace 包含 3 个 crate (core lib, CLI app, web server)
- 部署到 Docker 容器
- 在 Linux x86_64 和 ARM64 上运行

**任务**:
设计 CI/CD 流水线，包含以下阶段：
1. **PR 检查**: 快速的质量门（< 10 分钟）
2. **合并后构建**: 完整的构建和测试（< 30 分钟）
3. **发布**: 构建 Docker 镜像并推送到容器注册表
4. **安全**: 依赖审计和漏洞扫描

对于每个阶段，写明:
- 触发条件（何时运行）
- 运行的命令
- 预期的执行时间
- 失败后的处理方式

**加分项**: 画出流水线的流程图。

## 提交要求

- 练习 1-3 为基础必做，提交运行命令和输出截屏
- 练习 4-6 为进阶选做（建议完成 2-3 个）
- 练习 7-8 为综合练习（建议完成 1 个）
- 所有输出整理到一个目录中提交
