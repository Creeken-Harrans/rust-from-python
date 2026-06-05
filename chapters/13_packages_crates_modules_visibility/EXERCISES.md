# 练习题 — 包、箱与模块

## 说明

以下练习题旨在帮助你深入理解 Rust 的 Package、Crate、Module 以及可见性系统。
每个练习都有一个明确的目标，建议按顺序完成。动手实践是最有效的学习方式。

---

## 练习 1：基础概念填空

在不查阅资料的情况下，尝试回答以下问题。完成后对照 README.md 验证。

1. 一个 Package 最多可以包含 _____ 个 library crate。
2. Library crate 的根文件是 _____。
3. 默认的 binary crate 的根文件是 _____。
4. 在 lib.rs 中使用 _____ 关键字声明模块。
5. 在 lib.rs 中使用 _____ 关键字导入路径别名。
6. Rust 中，模块和函数的默认可见性是 _____。
7. `pub(crate)` 使项在 _____ 内可见。
8. `pub(super)` 使项在 _____ 内可见。
9. 要引用父模块，使用 _____ 前缀。
10. 要引用当前 crate 根，使用 _____ 前缀。

---

## 练习 2：添加新模块

### 目标

在项目中添加一个新的公开模块 `utils`，包含一个公开函数 `capitalize_first`。

### 步骤

1. 创建 `src/utils.rs`
2. 在 `src/lib.rs` 中声明 `pub mod utils;`
3. 在 `utils.rs` 中实现：
   ```rust
   /// 将字符串的首字母大写。
   pub fn capitalize_first(s: &str) -> String {
       let mut chars: Vec<char> = s.chars().collect();
       if let Some(first) = chars.first_mut() {
           *first = first.to_uppercase().next().unwrap_or(*first);
       }
       chars.into_iter().collect()
   }
   ```
4. 在 `src/main.rs` 中添加对 `capitalize_first` 的调用
5. 在 `tests/integration_test.rs` 中添加测试
6. 运行 `cargo test` 确认一切通过

### 思考

- 为什么需要在 `lib.rs` 中显式声明 `pub mod utils;`？
- 如果改为 `mod utils;`（去掉 `pub`），集成测试还能访问 `capitalize_first` 吗？

---

## 练习 3：可见性实验

### 目标

通过实际编译错误理解可见性边界。

### 步骤

1. 打开 `tests/integration_test.rs`
2. 取消注释尝试访问 `validator::sanitize` 的代码：
   ```rust
   use packages_and_modules::services::validator::sanitize;
   ```
3. 运行 `cargo test`，观察编译错误
4. 记录错误信息，用自己的话解释为什么会失败：
   - 第一步：因为 `validator` 模块是 _____ 的
   - 第二步：即使模块是公开的，`sanitize` 是 `pub(crate)` 的，而集成测试是一个 _____ crate
5. 还原代码

### 扩展

尝试在 `services/mod.rs` 中将 `mod validator;` 改为 `pub mod validator;`，再次运行测试。错误信息有什么变化？

---

## 练习 4：理解 re-export

### 目标

理解 `pub use` 如何改变 API 的对外呈现。

### 背景

在 `services/mod.rs` 中有一行：
```rust
pub use parser::parse_task_line;
```

### 步骤

1. 暂时注释掉这行 `pub use`
2. 修改 `tests/integration_test.rs` 中的导入，改为：
   ```rust
   use packages_and_modules::services::parser::parse_task_line;
   ```
3. 运行 `cargo test`，确认测试仍然通过
4. 思考：`pub use` 给使用者带来了什么便利？
5. 还原代码

### 思考题

如果要让外部使用者能够使用 `packages_and_modules::services::validate_title`，需要做哪些修改？

---

## 练习 5：实现批量任务标记

### 目标

在 `models.rs` 中添加一个函数，批量标记任务完成。

### 要求

1. 在 `models.rs` 中实现：
   ```rust
   /// 将一组任务全部标记为已完成。
   pub fn mark_all_done(tasks: &mut [Task]) {
       for task in tasks.iter_mut() {
           task.mark_done();
       }
   }
   ```
2. 在 `main.rs` 中调用这个新函数
3. 在 `tests/integration_test.rs` 中添加测试

### 测试用例

```rust
#[test]
fn test_mark_all_done() {
    let mut tasks = create_sample_tasks();
    mark_all_done(&mut tasks);
    for task in &tasks {
        assert!(task.completed);
    }
}
```

---

## 练习 6：添加新的优先级解析规则

### 目标

扩展 `parse_task_line`，支持新的优先级 `[Critical]`。

### 要求

1. 在 `models.rs` 的 `Priority` 枚举中添加 `Critical` 变体
2. 在 `parser.rs` 中添加对 `[Critical]` 前缀的解析
3. 更新 `main.rs` 中的演示输入
4. 添加单元测试和集成测试

### 思考

- 为什么优先级的解析逻辑放在 `parser.rs` 而不是 `models.rs`？
- 如果要支持多种语言的优先级标记（如 `[紧急]`），应该如何设计？

---

## 练习 7：理解 crate 边界

### 目标

理解 binary crate 和 library crate 是不同的 crate。

### 步骤

1. 在 `main.rs` 中定义一个 `pub(crate)` 函数：
   ```rust
   pub(crate) fn binary_only_helper() -> &'static str {
       "这个函数只在 binary crate 中可见"
   }
   ```
2. 尝试在 `lib.rs` 中调用 `binary_only_helper()` —— 你应该得到一个编译错误
3. 解释：为什么 `pub(crate)` 的函数不能在 library crate 中使用？

---

## 练习 8：模块树绘图

### 目标

画出本项目的完整模块树。

### 要求

1. 在一张纸上画出：
   - Library crate 的完整模块树（从 `lib.rs` 开始）
   - Binary crate 的完整模块树（从 `main.rs` 开始）
2. 用不同颜色标记：
   - 公开的模块和函数（红色）
   - `pub(crate)` 的函数（蓝色）
   - `pub(super)` 的函数（绿色）
   - 完全私有的函数和模块（黑色）
3. 画出哪些路径可以从集成测试中访问

---

## 练习 9：路径转换

### 目标

熟练使用绝对路径和相对路径。

### 题目

将以下绝对路径改为等效的相对路径（在指定文件中）：

| 文件 | 绝对路径 | 改为相对路径 |
|------|---------|-------------|
| `parser.rs` | `crate::models::Priority` | （在 parser.rs 中） |
| `services/mod.rs` | `crate::services::parser::parse_task_line` | （在 services/mod.rs 中）|
| `parser.rs` | `crate::services::validator::validate_title` | （在 parser.rs 中） |
| `main.rs` | `crate::app_utils::print_module_info` | （在 main.rs 自己内部） |

---

## 练习 10：创建自己的 binary crate

### 目标

在 Package 中添加第二个 binary crate。

### 步骤

1. 创建目录 `src/bin/`
2. 创建文件 `src/bin/task_cli.rs`
3. 在 `task_cli.rs` 中使用 library crate 的类型：
   ```rust
   use packages_and_modules::models::*;
   use packages_and_modules::services::*;

   fn main() {
       let tasks = create_sample_tasks();
       println!("共 {} 个任务:", tasks.len());
       for task in tasks {
           println!("  [#{}] {} [{}]", task.id, task.title,
               if task.completed { "✓" } else { " " });
       }
   }
   ```
4. 运行 `cargo run --bin task_cli`
5. 验证：现在 Package 有 2 个 binary crate（默认的 `main.rs` 和新的 `task_cli.rs`）

### 思考

- 如果 `task_cli.rs` 中也定义了 `mod app_utils`，它和 `main.rs` 中的 `app_utils` 是同一个吗？
- 为什么？

---

## 练习 11：组织大型模块

### 目标

练习使用 `mod.rs` 模式（目录作为模块）。

### 背景

当模块变得复杂时，可以使用目录替代单个文件：
```
src/
  services/
    mod.rs          ← 模块入口
    parser.rs       ← 子模块
    validator.rs    ← 子模块
    formatter.rs    ← 新子模块（练习中创建）
```

### 步骤

1. 创建 `src/services/formatter.rs`
2. 在 `src/services/mod.rs` 中声明 `pub mod formatter;`
3. 实现一个函数：
   ```rust
   /// 将 Task 格式化为字符串。
   pub fn format_task(task: &crate::models::Task) -> String {
       format!(
           "[{}] {} {}",
           task.id,
           task.title,
           if task.completed { "✓" } else { "✗" }
       )
   }
   ```
4. 在 `main.rs` 中使用并演示
5. 添加测试

---

## 练习 12：综合挑战 — 实现一个简单的任务管理器

### 目标

综合运用所有知识，在 library crate 中实现一个 `TaskManager`。

### 要求

1. 在 `models.rs` 中添加：
   ```rust
   pub struct TaskManager {
       tasks: Vec<Task>,
       next_id: u32,
   }

   impl TaskManager {
       pub fn new() -> Self { ... }
       pub fn add_task(&mut self, title: String) -> &Task { ... }
       pub fn list_tasks(&self) -> &[Task] { ... }
       pub fn complete_task(&mut self, id: u32) -> Option<&Task> { ... }
   }
   ```

2. `TaskManager` 的字段是私有的，通过公开方法访问

3. 在 `main.rs` 中实现一个简单的交互式菜单（或直接展示调用流程）

4. 在 `tests/integration_test.rs` 中编写完整的测试：
   - 测试添加任务
   - 测试完成任务
   - 测试完成不存在的任务返回 `None`
   - 测试任务 ID 自动递增

### 测试示例

```rust
#[test]
fn test_task_manager_add_and_list() {
    let mut manager = TaskManager::new();
    manager.add_task("任务 1".to_string());
    manager.add_task("任务 2".to_string());
    assert_eq!(manager.list_tasks().len(), 2);
}

#[test]
fn test_task_manager_complete() {
    let mut manager = TaskManager::new();
    manager.add_task("任务 1".to_string());
    let task = manager.complete_task(1);
    assert!(task.is_some());
    assert!(task.unwrap().completed);
}

#[test]
fn test_task_manager_complete_nonexistent() {
    let mut manager = TaskManager::new();
    manager.add_task("任务 1".to_string());
    let task = manager.complete_task(999);
    assert!(task.is_none());
}
```

---

## 练习 13：自我检查清单

完成所有练习后，对照以下清单自我评估：

- [ ] 我能用一句话区分 Package、Crate 和 Module
- [ ] 我知道如何声明模块（`mod`）以及何时使用 `pub mod`
- [ ] 我能区分 `mod` 和 `use` 的不同作用
- [ ] 我理解 `pub`、`pub(crate)`、`pub(super)` 的区别
- [ ] 我知道 `crate::`、`super::`、`self::` 分别指向哪里
- [ ] 我理解重导出（`pub use`）的作用
- [ ] 我知道如何在 `tests/` 目录中添加集成测试
- [ ] 我理解为什么不能在集成测试中访问 `pub(crate)` 函数
- [ ] 我能区分 library crate 和 binary crate 的 `crate::` 前缀含义不同
- [ ] 我知道 Rust 的模块系统为什么需要显式声明（与 Python 不同）

---

## 提示与技巧

### 编译错误是学习工具

Rust 的编译错误信息非常优秀。当遇到以下错误时：

- `E0432: unresolved import` — 路径不存在或未公开
- `E0583: file not found for module` — 缺少 `mod` 声明或文件不存在
- `E0603: module is private` — 尝试访问私有模块
- `E0624: function is private` — 尝试调用私有函数

仔细阅读错误信息，它通常会告诉你缺少什么、应该怎么改。

### 推荐工作流

1. 阅读练习要求
2. 在纸上画出需要的模块结构
3. 编写代码
4. 运行 `cargo build` 检查编译
5. 运行 `cargo test` 检查测试
6. 如果出现编译错误，先尝试理解错误信息再修改

### 调试技巧

- 使用 `cargo check` 快速检查编译（比 `cargo build` 快，因为不生成二进制文件）
- 使用 `cargo test -- --nocapture` 查看测试中的 `println!` 输出
- 使用 `cargo doc --open` 查看生成的文档
