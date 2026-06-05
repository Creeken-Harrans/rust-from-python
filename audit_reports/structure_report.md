# Structure Audit Report

**Total files checked**: 17 root + 28 chapters + 5 projects

## P0 Blocking Issues: 0


## P1 Quality Issues: 88

- **[P1]** Placeholder '略' in MISCONCEPTIONS.md:569: | 内存增长 | 自动，有预留（over-allocation） | 自动，类似策略 |
- **[P1]** Placeholder '略' in AUDIT_REPORT.md:207: | 05_ownership_move_copy_clone | A | Move ≠ 深拷贝，Rust Move ≠ C++ std::move，策略优先级 |
- **[P1]** Placeholder '略' in AUDIT_REPORT.md:208: | 06_references_borrowing_slices | A | 借用规则与数据竞争的关系，6 层修复策略 |
- **[P1]** Placeholder 'TODO' in AUDIT_REPORT.md:482: - ✅ 无未完成 TODO 或空壳
- **[P1]** Placeholder '略' in MENTAL_MODELS.md:311: // 通过 vtable 间接调用。略有运行时开销。
- **[P1]** Placeholder '略' in MENTAL_MODELS.md:415: "异步不是更快的线程——它是一种让 I/O 密集型任务在等待时让出 CPU 的调度策略。"
- **[P1]** Placeholder '略' in LEARNING_GUIDE.md:58: ### 错误阅读策略
- **[P1]** Placeholder '略' in C_CPP_TO_RUST.md:567: | 版本管理 | 多种策略（无统一方式） | SemVer + `Cargo.lock` |
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:203: ### 内存存储策略
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:211: | 无需实现缓存策略 | 启动时间随文件增大而线性增长 |
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:240: ### 3. 持久化策略
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:244: - 没有压缩（compaction）策略
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:311: 使用"写临时文件 + 原子重命名"策略避免文件损坏：
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:416: > - 持久化策略简单（全量重写），不适合大数据量场景
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/README.md:781: Python 的 `for line in open('file')` 自动提供缓冲和逐行迭代，这是极好的体验，但缓冲策略和内存分配对用户完全不透明。Rust 需要显式创建 `BufReader::new(File::open(...)?)` 来获得缓冲读取，略显冗长但带来了精确的控制——你知道每一步的内存分配和 I/O 策略。本项目的 `load` 方法逐行解析 `key|value` 格式，格式错误打印警告继续解析，这是"容错加载"的范例。Python 可以做到同样的事，但 Rust 额外的收获是：`File::open` 返回 `Result`，`line_result?` 传播 I/O 错误，每一步的类型都告诉你"这里可能失败，需要处理"。
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:61: ### 持久化策略：全量重写
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:74: `load()` 遇到格式错误的行时，记录警告并跳过，而非 panic 或丢弃所有数据——这是"容错加载"策略。
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:110: ## 8. 测试策略
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:133: - 内存淘汰策略（LRU）
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:137: 本项目的持久化策略（全量重写）、单线程模型、简单文件格式都无法满足生产级数据库的要求。它用于学习 Rust 的文件 I/O、HashMap 操作和错误处理模式。
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/SOLUTIONS.md:141: *学习重点：HashMap 作为内存存储、文件持久化的安全写入模式、容错加载策略。明确理解"教学项目"与"生产系统"的边界。*
- **[P1]** Placeholder '略' in projects/05_mini_kv_store/src/lib.rs:124: /// 当前使用简单的全量写入策略。对于大规模数据（百万级记录），
- **[P1]** Placeholder '略' in projects/02_cli_text_search/README.md:556: 2. **选择搜索策略**：根据 `config.case_sensitive` 分支调用 `search` 或 `search_case_insensitive`
- **[P1]** Placeholder '略' in projects/02_cli_text_search/SOLUTIONS.md:81: ## 8. 测试策略
- **[P1]** Placeholder '略' in projects/02_cli_text_search/src/lib.rs:182: /// 2. 根据 `config.case_sensitive` 选择搜索策略
- **[P1]** Placeholder '略' in projects/02_cli_text_search/src/lib.rs:212: // 根据配置选择搜索策略
- **[P1]** Placeholder '略' in projects/01_guessing_game/README.md:637: 每个步骤的错误处理策略不同：
- **[P1]** Placeholder '略' in projects/01_guessing_game/README.md:639: | 错误         | 策略       | 理由                              |
- **[P1]** Placeholder '略' in projects/01_guessing_game/README.md:751: - 实现二分查找策略的可视化
- **[P1]** Placeholder '略' in projects/01_guessing_game/README.md:782: - 实现一个使用二分查找策略的 AI 对手
- **[P1]** Placeholder '略' in projects/01_guessing_game/README.md:794: - 分析用户的猜测策略
- **[P1]** Placeholder '略' in projects/01_guessing_game/SOLUTIONS.md:125: ### 6.2 错误处理策略
- **[P1]** Placeholder '略' in projects/01_guessing_game/SOLUTIONS.md:127: | 错误类型 | 策略 | 理由 |
- **[P1]** Placeholder '略' in projects/01_guessing_game/SOLUTIONS.md:201: ## 8. 测试策略
- **[P1]** Placeholder '略' in projects/01_guessing_game/src/main.rs:209: _ => println!("💪 继续加油！试试二分查找策略？"),
- **[P1]** Placeholder '略' in projects/01_guessing_game/src/main.rs:239: /// # 错误处理策略
- **[P1]** Placeholder '略' in projects/03_todo_cli/README.md:277: ### 6.4 错误处理策略
- **[P1]** Placeholder '略' in projects/03_todo_cli/README.md:281: | 层级       | 策略                                                           |
- **[P1]** Placeholder '略' in projects/03_todo_cli/README.md:730: ### 11.4 不可变的 ID 策略
- **[P1]** Placeholder '略' in projects/03_todo_cli/SOLUTIONS.md:98: ## 8. 测试策略
- **[P1]** Placeholder '略' in projects/04_parallel_text_stats/README.md:421: ### 6. 错误恢复策略
- **[P1]** Placeholder '略' in projects/04_parallel_text_stats/SOLUTIONS.md:71: ### 错误聚合策略
- **[P1]** Placeholder '略' in projects/04_parallel_text_stats/SOLUTIONS.md:98: ## 8. 测试策略
- **[P1]** Placeholder '略' in chapters/15_generics_traits_trait_bounds/SOLUTIONS.md:491: 缓解策略：
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:483: ### 修复策略 1：限制作用域
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:496: ### 修复策略 2：提前使用不可变引用
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:509: ### 修复策略 3：Clone 数据
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:520: ### 修复策略 4：使用 Copy 类型
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:532: ### 修复策略 5：重新设计代码结构
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:940: ## 典型借用冲突修复策略
- **[P1]** Placeholder '略' in chapters/06_references_borrowing_slices/README.md:942: 当借用检查器报错时，不要慌张，也不要立即用 `.clone()` 来"堵住编译器的嘴"。以下是推荐的修复策略，按**从简单到重构**的顺序排列：
- **[P1]** Placeholder '略' in chapters/05_ownership_move_copy_clone/README.md:524: ### Move 错误的四种修复策略
- **[P1]** Placeholder '略' in chapters/05_ownership_move_copy_clone/README.md:526: **策略 1: 使用 Clone (显式深拷贝)**
- **[P1]** Placeholder '略' in chapters/05_ownership_move_copy_clone/README.md:536: **策略 2: 使用引用 (借用, 下一章详细讲)**
- **[P1]** Placeholder '略' in chapters/05_ownership_move_copy_clone/README.md:546: **策略 3: 返回所有权**
- **[P1]** Placeholder '略' in chapters/05_ownership_move_copy_clone/README.md:560: **策略 4: 重构设计, 让所有权更早落入最终使用者**
- **[P1]** Placeholder 'TODO' in chapters/05_ownership_move_copy_clone/README.md:623: - **原型阶段 (Prototyping)**: 快速验证想法, `.clone()` 让你先让代码跑起来, 之后再优化所有权设计。标注 `// TODO: 消除 clone` 是个好习惯, 避免临时代码悄悄变成永久代码。
- **[P1]** Placeholder '略' in chapters/25_cargo_dependencies_features_profiles/README.md:239: - **行为切换**: 同一接口的不同实现策略（如不同的 TLS 后端）
- **[P1]** Placeholder '略' in chapters/02_variables_and_types/README.md:681: `as` 是"信任程序员"的快捷方式；`try_from`/`try_into` 是"编译期 + 运行期双重保险"的安全路径。Rust 的选择很明确：隐式转换永远不允许，你始终在两种显式策略之间做选择。
- **[P1]** Placeholder '略' in chapters/26_workspace_architecture/README.md:267: Resolver（解析器）是 Cargo 用来决定依赖图中每个包的具体版本和 features 的算法。不同版本的 resolver 有不同的解析策略。
- **[P1]** Placeholder '略' in chapters/26_workspace_architecture/README.md:578: - **resolver = "3"** 提供现代化的依赖解析策略
- **[P1]** Placeholder '略' in chapters/26_workspace_architecture/src/main.rs:262: println!("Resolver 是 Cargo 的依赖解析策略。Rust 2024 edition 默认");
- **[P1]** Placeholder '略' in chapters/26_workspace_architecture/src/main.rs:515: println!("  • resolver = \"3\" → 现代化依赖解析策略");
- **[P1]** Placeholder '略' in chapters/27_lints_format_docs_ci/SOLUTIONS.md:105: ### 3-1：lint 配置策略
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/README.md:362: ### 三种内存管理策略对比
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/README.md:364: | 策略 | 代表语言 | 优点 | 缺点 |
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/SOLUTIONS.md:316: Rust 的 `String`（以及 `Vec<T>`）采用 **翻倍扩容**（doubling）策略。当 `push` 导致 `len > cap` 时，容量会翻倍（或从 0 增长到某个初始值）。这使得 `push` 的摊还时间复杂度为 O(1)。
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/SOLUTIONS.md:378: #### 为什么采用这种增长策略？
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/SOLUTIONS.md:382: 2. **空间换时间**：翻倍策略用较多的预留空间换取更少的扩容次数。如果采用线性增长（每次+1 或 +固定值），扩容频繁且总复制成本为 O(n^2)。
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/SOLUTIONS.md:384: 3. **与主流实现一致**：C++ 的 `std::vector`、Java 的 `ArrayList`、Go 的 slice 都采用类似策略（通常是 1.5x 或 2x 增长）。
- **[P1]** Placeholder '略' in chapters/04_stack_heap_and_raii/SOLUTIONS.md:1080: 1. 容量可以动态增长（翻倍策略）
- **[P1]** Placeholder '未实现' in chapters/08_structs_methods_associated_functions/README.md:417: > **注意**：`..rect1` 会移动（move）那些未实现 `Copy` 的字段。因为 `f64` 实现了 `Copy`，所以本例中 `rect1` 之后仍可使用。如果字段是 `String`，则 `rect1` 会被部分移动而不可再用。
- **[P1]** Placeholder '略' in chapters/08_structs_methods_associated_functions/SOLUTIONS.md:768: 3. **`acos` 参数越界**：浮点误差可能导致 `dot/mags` 略大于 1.0，用 `clamp(-1.0, 1.0)` 处理
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:281: 2. **可定制性：** 不同场景需要不同的调度策略（单线程、多线程、优先级调度等）
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:492: 多线程是一种**执行策略**，解决的是"如何使用多核 CPU"的问题。
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:707: - **无栈协程**：编译器将协程体转换为状态机——与 Rust `async fn` 的编译策略一致
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:753: - **异步是一种调度策略**：适用于大量任务反复等待 I/O 的场景。等待期间线程不闲置，可以切换到其他任务，从而在少量线程上管理海量并发。
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:758: ### I/O 密集型 vs CPU 密集型：不同场景需要不同策略
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:760: | 场景类型 | 瓶颈 | 最佳策略 | 典型示例 |
- **[P1]** Placeholder '略' in chapters/22_async_await_tokio_intro/README.md:764: | 混合型 | 两者兼有 | async + spawn_blocking | HTTP 请求接收 + 图片缩略处理 |
- **[P1]** Placeholder '未实现' in chapters/18_closures_iterators/src/main.rs:299: // 如果 String 未实现 Copy，移动后 owned_string 不可再用
- **[P1]** Placeholder '略' in chapters/21_threads_channels_shared_state/README.md:529: ### 避免死锁的策略
- **[P1]** Placeholder '略' in chapters/21_threads_channels_shared_state/SOLUTIONS.md:365: **结论**：归并排序天然适合并行化——分治策略可以将左右两半交给不同线程处理。但对于小数组，线程创建的开销会超过并行收益。
- **[P1]** Placeholder '略' in chapters/21_threads_channels_shared_state/src/main.rs:453: println!("  · 考虑使用 try_lock() 代替 lock() 实现超时/重试策略");
- **[P1]** Placeholder '略' in chapters/09_enums_option_pattern_matching/README.md:822: Rust 的 `match` 采取完全不同的策略：**编译期强制穷尽性检查，缺失任何变体就是编译错误**。
- **[P1]** Placeholder '略' in final_audit_reports/solutions_mapping.md:240: | chapters/26_workspace_architecture/EXERCISES.md | line 126 | 练习 6: 评估 CI 中 Workspace 构建策略 (15 分钟) | chapters/26_workspace_architecture/SOLUTIONS.md | PASS |  |
- **[P1]** Placeholder '略' in final_audit_reports/solutions_mapping.md:276: | projects/02_cli_text_search/EXERCISES.md | line 213 | L3-4: 插件式搜索策略 | projects/02_cli_text_search/SOLUTIONS.md | PASS |  |
- **[P1]** Placeholder '略' in final_audit_reports/solutions_mapping.md:280: | projects/03_todo_cli/EXERCISES.md | line 46 | L1-4: 理解错误处理策略 | projects/03_todo_cli/SOLUTIONS.md | PASS |  |

## P2 Optimization Issues: 0


## Summary

| Level | Count |
|-------|-------|
| P0    | 0 |
| P1    | 88 |
| P2    | 0 |

## Conclusion

**PASS**: No blocking issues found.
