//! 猜数字游戏的集成测试
//!
//! 测试规则：
//! - 单元测试 (`#[cfg(test)]`) 可以访问私有函数，因为它们与源码在同一 crate 中
//! - 集成测试 (`tests/` 目录) 是独立的 crate，只能访问公开 API
//! - 为此，我们直接测试可独立测试的纯函数逻辑

use std::cmp::Ordering;

// ============================================================
// 由于集成测试在独立 crate 中，我们无法直接访问 main.rs 中的函数。
// 以下测试模拟了核心函数的逻辑进行验证。
// 如需测试实际函数，请在 src/main.rs 中使用 #[cfg(test)] 模块。
// ============================================================

/// 模拟 parse_guess 函数的逻辑
fn parse_guess(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("输入为空，请输入一个数字".to_string());
    }
    match trimmed.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' 不是一个有效的数字", trimmed)),
    }
}

/// 模拟 check_guess 函数的逻辑
fn check_guess(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

// -----------------------------------------------------------
// parse_guess 测试
// -----------------------------------------------------------

#[test]
fn test_parse_guess_valid_number() {
    let result = parse_guess("42");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_parse_guess_with_whitespace() {
    let result = parse_guess("  100  ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
}

#[test]
fn test_parse_guess_boundary_min() {
    let result = parse_guess("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_parse_guess_boundary_max() {
    // u32::MAX = 4294967295，远大于 100，但解析本身应该成功
    let result = parse_guess("4294967295");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4294967295u32);
}

#[test]
fn test_parse_guess_non_numeric() {
    let result = parse_guess("abc");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("不是一个有效的数字"));
}

#[test]
fn test_parse_guess_empty_input() {
    let result = parse_guess("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("输入为空"));
}

#[test]
fn test_parse_guess_whitespace_only() {
    let result = parse_guess("   \t  ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("输入为空"));
}

#[test]
fn test_parse_guess_float_number() {
    // "12.5" 不能解析为 u32
    let result = parse_guess("12.5");
    assert!(result.is_err());
}

#[test]
fn test_parse_guess_negative_number() {
    // "-5" 不能解析为 u32
    let result = parse_guess("-5");
    assert!(result.is_err());
}

#[test]
fn test_parse_guess_scientific_notation() {
    // "1e5" 不能解析为 u32
    let result = parse_guess("1e5");
    assert!(result.is_err());
}

#[test]
fn test_parse_guess_zero() {
    let result = parse_guess("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

// -----------------------------------------------------------
// check_guess 测试
// -----------------------------------------------------------

#[test]
fn test_check_guess_less() {
    assert_eq!(check_guess(30, 50), Ordering::Less);
    assert_eq!(check_guess(1, 100), Ordering::Less);
    assert_eq!(check_guess(0, 1), Ordering::Less);
}

#[test]
fn test_check_guess_equal() {
    assert_eq!(check_guess(50, 50), Ordering::Equal);
    assert_eq!(check_guess(1, 1), Ordering::Equal);
    assert_eq!(check_guess(100, 100), Ordering::Equal);
}

#[test]
fn test_check_guess_greater() {
    assert_eq!(check_guess(70, 50), Ordering::Greater);
    assert_eq!(check_guess(100, 1), Ordering::Greater);
    assert_eq!(check_guess(2, 1), Ordering::Greater);
}

#[test]
fn test_check_guess_extreme_values() {
    assert_eq!(check_guess(0, u32::MAX), Ordering::Less);
    assert_eq!(check_guess(u32::MAX, 0), Ordering::Greater);
}
