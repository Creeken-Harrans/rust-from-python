# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：编写第一个单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn divide(a: i32, b: i32) -> i32 { a / b }
```

#### 常见错误

- 忘记 `#[cfg(test)]` → 测试代码混入主构建
- `should_panic` 没写 `expected` → 任何 panic 都通过

---

### 1-2：`assert!` / `assert_eq!` / `assert_ne!`

```rust
#[test]
fn test_assertions() {
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}
```

`assert_eq!` 内部用 `==`（要求 `PartialEq`），失败时打印两个值的 debug 输出。

---

### 1-3：文档测试

```rust
/// 将两个整数相加。
///
/// # 示例
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

运行：`cargo test` 自动执行文档中的代码块作为测试。

---

## Level 2：组合应用

### 2-1：集成测试

```
tests/
├── common/mod.rs      // 共享辅助函数
└── integration_test.rs
```

```rust
// tests/integration_test.rs
mod common;
use my_crate::add;

#[test]
fn test_add_via_public_api() {
    assert_eq!(add(100, 200), 300);
}
```

#### 为什么集成测试在 `tests/` 而非 `src/`？

- 每个 `tests/*.rs` 被编译为独立 crate，只能访问公共 API
- 模拟真实外部用户的视角
- 如果希望共享辅助代码，放在 `tests/common/mod.rs`（该目录不会被当作测试文件）

---

### 2-2：测试组织最佳实践

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod unit_tests {
        use super::*;
        #[test] fn test_small() { /* ... */ }
    }

    mod edge_cases {
        use super::*;
        #[test] fn test_empty() { /* ... */ }
    }

    mod error_cases {
        use super::*;
        #[test] #[should_panic] fn test_invalid() { /* ... */ }
    }
}
```

---

## Level 3：设计思考

### 测试覆盖率不是唯一目标

1. **边界条件**比覆盖率数字更重要：空输入、极端值、错误路径
2. **每个 `unwrap()` 应该被测试覆盖**——它可能在生产 panic
3. **重构后测试通过不一定意味着正确**：测试也可以有 bug

### 什么时候写测试？

- 修复 bug 时：先写复现测试，再修复
- 添加新功能时：先写预期行为的测试
- 重构前：确保有足够覆盖率保护

---

## 思考题

### Rust 测试与 Python 测试的比较

| 方面 | Rust | Python (pytest) |
|------|------|----------------|
| 测试位置 | 同一文件 `#[cfg(test)]` 或 `tests/` | 独立 `test_*.py` 文件 |
| 断言 | `assert_eq!` 等宏 | `assert` 语句 |
| 文档测试 | 内置 `cargo test` 执行 | 需 `doctest` 模块或 pytest 插件 |
| 并行执行 | 默认并行 | pytest `-n auto` |
| 类型检查 | 编译时保证测试代码类型正确 | 运行到才检查 |

---

*测试在 Rust 中是一等公民——`cargo test` 开箱即用，无需安装额外工具。把测试当作 API 设计的工具：如果你发现很难测试某个函数，可能意味着接口设计需要改进。*
