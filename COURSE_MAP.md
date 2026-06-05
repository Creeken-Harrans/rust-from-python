# 课程地图 (Course Map)

本教程的完整章节地图，按学习阶段组织。

---

## 阶段 A：认识 Rust 与 Cargo

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 00 | [课程导览](chapters/00_course_orientation/) | 无 | 系统编程、编译vs解释、Rust核心价值 | 1h | 是 | 无 |
| 01 | [Hello Cargo](chapters/01_hello_cargo/) | 00 | rustc/cargo/rustup, Cargo.toml, Debug/Release | 1.5h | 是 | 无 |
| 02 | [变量与类型](chapters/02_variables_and_types/) | 01 | 变量绑定、不可变性、标量类型、复合类型 | 2h | 是 | 无 |
| 03 | [函数与控制流](chapters/03_functions_expressions_control_flow/) | 02 | 函数、语句vs表达式、if表达式、循环 | 2h | 是 | 无 |

---

## 阶段 B：Rust 的核心——所有权系统

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 04 | [栈、堆与 RAII](chapters/04_stack_heap_and_raii/) | 03 | Stack, Heap, Pointer, RAII, Drop | 2h | 是 | 无 |
| 05 | [所有权与移动](chapters/05_ownership_move_copy_clone/) | 04 | Ownership, Move, Copy, Clone, Drop | 3h | **是** | 无 |
| 06 | [引用、借用与切片](chapters/06_references_borrowing_slices/) | 05 | Reference, Borrowing, Slice, &str, Borrow Rules | 3h | **是** | 无 |
| 07 | [所有权实战：文本分析器](chapters/07_ownership_practice_text_analyzer/) | 06 | 所有权综合应用、参数/返回类型选择 | 2h | **是** | 无 |

**阶段 B 是 Rust 学习的核心**。如果只能完整学习一个阶段，选这个。

---

## 阶段 C：建模、模式匹配与常用数据结构

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 08 | [结构体与方法](chapters/08_structs_methods_associated_functions/) | 07 | Struct, impl, Method, Associated Function, Debug | 2h | 是 | 项目01 |
| 09 | [枚举与模式匹配](chapters/09_enums_option_pattern_matching/) | 08 | Enum, Option, match, if let, let else | 3h | **是** | 无 |
| 10 | [集合类型](chapters/10_collections_vec_string_hashmap/) | 09 | Vec, String, HashMap, Entry API, UTF-8 | 2.5h | 是 | 无 |
| 11 | [模式与解构](chapters/11_patterns_and_destructuring/) | 09 | Pattern, Destructuring, Match Guard, @ Binding | 2h | 是 | 无 |

---

## 阶段 D：错误处理与工程拆分

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 12 | [错误处理](chapters/12_error_handling_result_question_mark/) | 10 | Result, panic!, ?, Error Propagation | 2.5h | **是** | 项目02 |
| 13 | [包、箱与模块](chapters/13_packages_crates_modules_visibility/) | 12 | Package, Crate, Module, mod, use, pub | 2.5h | 是 | 项目02 |
| 14 | [测试与文档](chapters/14_testing_documentation_benchmindset/) | 13 | Unit Test, Integration Test, Doc Test, Rustdoc | 2h | 是 | 项目03 |

---

## 阶段 E：泛型、Trait 与生命周期

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 15 | [泛型与特征](chapters/15_generics_traits_trait_bounds/) | 13 | Generic, Trait, Trait Bound, Monomorphization | 3h | **是** | 无 |
| 16 | [生命周期](chapters/16_lifetimes/) | 06, 15 | Lifetime, Lifetime Elision, 'static | 2.5h | **是** | 无 |
| 17 | [特征对象](chapters/17_trait_objects_dynamic_dispatch/) | 15 | Trait Object, dyn Trait, Dynamic Dispatch | 2h | 是 | 无 |

---

## 阶段 F：函数式风格、智能指针与资源管理

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 18 | [闭包与迭代器](chapters/18_closures_iterators/) | 15 | Closure (Fn/FnMut/FnOnce), Iterator, Lazy | 2.5h | 是 | 无 |
| 19 | [智能指针](chapters/19_smart_pointers_box_rc_refcell/) | 06, 15 | Box, Rc, RefCell, Interior Mutability, Weak | 3h | 是 | 无 |
| 20 | [资源管理](chapters/20_resource_management_drop_deref/) | 04, 19 | Drop, Deref, Deref Coercion, RAII实践 | 2h | 是 | 无 |

---

## 阶段 G：并发与异步

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 21 | [线程与并发](chapters/21_threads_channels_shared_state/) | 05, 19 | Thread, Channel, Arc, Mutex, Send, Sync | 3h | 是 | 项目04 |
| 22 | [异步编程入门](chapters/22_async_await_tokio_intro/) | 21 | Future, async/await, Tokio, Runtime | 2.5h | 推荐 | 无 |

---

## 阶段 H：宏、Unsafe 与底层边界

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 23 | [宏](chapters/23_macros/) | 15 | macro_rules!, Declarative Macro, Proc Macro概述 | 2h | 推荐 | 无 |
| 24 | [Unsafe Rust 与 FFI](chapters/24_unsafe_rust_and_ffi_overview/) | 21 | Unsafe, Raw Pointer, FFI, Safe Abstraction | 2h | 推荐 | 无 |

---

## 阶段 I：Cargo 工程能力

| 编号 | 章节名称 | 前置知识 | 核心概念 | 推荐时长 | 必须 | 对应练习 |
|------|---------|---------|---------|---------|------|---------|
| 25 | [Cargo 工程能力](chapters/25_cargo_dependencies_features_profiles/) | 01, 13 | Dependencies, Features, Profiles, crates.io | 1.5h | 是 | 无 |
| 26 | [Workspace 架构](chapters/26_workspace_architecture/) | 13, 25 | Workspace, Virtual Workspace, Path Deps | 1.5h | 推荐 | 无 |
| 27 | [代码质量与 CI](chapters/27_lints_format_docs_ci/) | 14, 25 | Lint, Clippy, CI/CD, GitHub Actions | 1.5h | 推荐 | 无 |

---

## 阶段 J：综合实战项目

| 编号 | 项目名称 | 适合阶段 | 知识点 | 推荐时长 | 难度 |
|------|---------|---------|-------|---------|------|
| P01 | [猜数字游戏](projects/01_guessing_game/) | 完成阶段 A | 输入/输出, 随机数, match, loop, Result | 2h | ⭐ |
| P02 | [CLI 文本搜索](projects/02_cli_text_search/) | 完成阶段 D | Module, Ownership, Slice, Result, ?, Testing | 3h | ⭐⭐ |
| P03 | [Todo CLI](projects/03_todo_cli/) | 完成阶段 D | Struct, Enum, Result, serde, clap, Modularization | 4h | ⭐⭐⭐ |
| P04 | [并行文本统计](projects/04_parallel_text_stats/) | 完成阶段 G | Thread, Channel, Arc, Mutex, Ownership | 3h | ⭐⭐⭐ |
| P05 | [Mini KV Store](projects/05_mini_kv_store/) | 完成阶段 G | 综合所有知识 (Ownership, Collection, Error, Module, Test, CLI) | 5h | ⭐⭐⭐⭐ |

---

## 🔴 最高难度章节

以下章节是 Rust 学习的核心难点，建议投入额外时间，配合 [MENTAL_MODELS.md](MENTAL_MODELS.md) 和 [MISCONCEPTIONS.md](MISCONCEPTIONS.md) 阅读：

| 章节 | 难点 | 核心挑战 |
|------|------|---------|
| **05** | 所有权 (Ownership) | 从"一切皆引用"到"每个值有唯一主人"的思维转变 |
| **06** | 借用 (Borrowing) | 共享引用 vs 独占引用，借用检查器规则 |
| **16** | 生命周期 (Lifetimes) | 理解标注不延长寿命，表达引用关系 |
| **19** | 智能指针 (Smart Pointers) | 多种指针的选择决策、内部可变性 |
| **21** | 并发 (Concurrency) | Send/Sync、Arc<Mutex<T>>、死锁风险 |
| **22** | 异步 (Async) | Future 惰性求值、Runtime 角色、与多线程的区别 |
| **24** | Unsafe Rust | 安全边界抽象、FFI 注意事项 |

> 💡 如果你有 C/C++ 背景，阅读 [C_CPP_TO_RUST.md](C_CPP_TO_RUST.md) 可以帮助你快速定位 Rust 与 C/C++ 的差异。但请注意**相似不等于等价**。

---

## 学习路径建议

### 快速入门路径（约 20 小时）
```
00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → 10 → 12 → 15
```
跳过并发、宏、Unsafe 等高级主题，优先掌握核心概念。

### 完整学习路径（约 50 小时）
```
全部章节 00 → 27，配合综合项目 P01 → P05
```
按编号顺序学习即可。

### Python 开发者优先路径（约 30 小时）
重点投入：
1. **05-07** (所有权、借用) — 这是最大的思维转变
2. **09** (Option 与模式匹配) — 改变你对 None/null 的理解
3. **12** (错误处理) — 从异常到 Result 的转变
4. **15-16** (泛型与生命周期) — 类型系统的深度理解
5. **18-19** (闭包、迭代器、智能指针) — 从 Python 的函数式特性过渡
6. **P02-P03** (CLI 项目) — 实战巩固

---

## 完成标准

每章的完成标准是：
1. ✅ 阅读 README 并理解核心概念
2. ✅ 运行 `cargo run` 查看输出
3. ✅ 完成所有 Level 1 练习
4. ✅ 至少完成 1 道 Level 2 练习
5. ✅ 阅读思考题并尝试回答
6. ✅ 对关键术语能用自己的话解释
