# 教程创建与增强进度

## 总体状态

- 开始时间: 2026-06-05
- 第一轮完成: 2026-06-05 (28 章 + 5 项目 + 10 根级文档)
- 第二轮增强: 2026-06-05 (C/C++ 对照 + 思维模型 + 误解审查)
- Rust 版本: 1.96.0 stable
- 目标: 面向 Python + C/C++ 基础学习者，提供系统化的 Rust 教学

## 第二轮增强内容 (2026-06-05)

### 新增根级文档
- [x] C_CPP_TO_RUST.md — C/C++ → Rust 系统概念对照 (8 大主题，含表格)
- [x] MENTAL_MODELS.md — 9 个核心思维模型 (变量绑定、所有权、借用、生命周期等)
- [x] MISCONCEPTIONS.md — 25 条常见误解澄清

### 根级文档更新
- [x] README.md — 更新面向人群、推荐阅读顺序、目录结构
- [x] COURSE_MAP.md — 标注最高难度章节，新增文档入口
- [x] LEARNING_GUIDE.md — 新增 C/C++ 学习者提醒
- [x] GLOSSARY.md — 补充 C/C++ 术语对照说明表
- [x] PROGRESS.md — 本文件更新

### 章节增强 (C/C++ 对照)
- [x] 00_course_orientation — Python/C/C++/Rust 定位对照表
- [x] 01_hello_cargo — Cargo 与 C/C++ 构建工具对照
- [x] 02_variables_and_types — 变量、类型、char、数组对照
- [x] 03_functions_expressions_control_flow — 表达式导向 vs 语句导向
- [x] 04_stack_heap_and_raii — C malloc/C++ RAII/Rust 所有权对照流程图
- [x] 05_ownership_move_copy_clone — Rust Move vs C++ Move 专节
- [x] 06_references_borrowing_slices — 指针/引用对照表、NLL 时间线
- [x] 08_structs_methods — struct+impl vs class 对照
- [x] 09_enums_option_pattern_matching — 空指针问题、match 穷尽性
- [x] 10_collections — UTF-8 字符串背景、字符串接口表
- [x] 12_error_handling — C errno/C++ 异常/Python 异常/Rust Result
- [x] 13_packages_modules — use vs #include、模块概念对照表
- [x] 15_generics_traits — 泛型/模板对照、选择表
- [x] 16_lifetimes — "先别害怕生命周期"、修复优先级
- [x] 17_trait_objects — 虚函数/vtable 背景、静态/动态分派
- [x] 18_closures_iterators — Python/C++ 迭代器对照
- [x] 19_smart_pointers — 决策树、C++ 智能指针对照
- [x] 20_resource_management — C/C++ Drop 对照
- [x] 21_threads_concurrency — Send/Sync、类型对照表
- [x] 22_async_await — asyncio/Future 对照、异步 vs 多线程
- [x] 24_unsafe_ffi — unsafe 能力表、FFI 注意事项
- [x] 25_cargo_features — Feature 是编译期选择

### 练习增强
- [x] 11 个重点章节新增"迁移思维练习" (2-4 题/章)

### 综合项目增强
- [x] 5 个项目 README 新增"迁移设计差异"小节

### 交叉链接
- [x] 章节间相关链接 (04→05→06, 15→16→17, 19→20→21 等)

## 阶段进度 (第一轮)

### Phase 1: 根结构 ✓
- [x] 目录结构
- [x] Cargo.toml (Virtual Workspace)
- [x] rust-toolchain.toml
- [x] .gitignore
- [x] broken_examples/README.md
- [x] scripts/check_all.sh
- [x] scripts/check_all.ps1

### Phase 2-4: 全部 28 章节 ✓
- [x] 00-27 全部创建、编译通过、测试通过

### Phase 5: 综合项目 ✓
- [x] P01-P05 全部创建、编译通过、测试通过

### Phase 6: 根级文档 ✓
- [x] 14 个根级文档 (含第二轮新增 3 个)

### Phase 7: 全量验收
- [ ] cargo fmt --all -- --check
- [ ] cargo check --workspace --all-targets
- [ ] cargo test --workspace
- [ ] cargo clippy --workspace --all-targets --all-features -- -D warnings
- [ ] cargo doc --workspace --no-deps
