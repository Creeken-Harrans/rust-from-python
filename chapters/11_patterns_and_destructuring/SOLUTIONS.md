# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 练习 1-1：match 穷尽性

#### 结论

`Option<i32>` 的两个变体 `Some` 和 `None` 都必须被处理，否则编译失败。

#### 参考实现

```rust
fn classify_option(opt: Option<i32>) -> &'static str {
    match opt {
        Some(n) if n > 0 => "正数",
        Some(0) => "零",
        Some(_) => "负数",
        None => "无值",
    }
}
```

#### 验证方式

省略 `None` 分支后编译，观察编译器精确报告缺失分支的文件和行号。

---

### 练习 1-2：匹配守卫

```rust
fn is_adult(age: Option<u8>) -> bool {
    matches!(age, Some(a) if a >= 18)
}
```

---

### 练习 1-3：结构体解构

```rust
struct Point3D { x: f64, y: f64, z: f64 }

fn describe_point(p: Point3D) -> String {
    let Point3D { x, y, z } = p;
    if z == 0.0 { format!("({}, {})", x, y) }
    else { format!("({}, {}, {})", x, y, z) }
}
```

#### 常见错误

- 解构后 `p` 不再可用（所有权被解构移动），如需保留原值用 `let Point3D { x, y, z } = &p;`（此时 `x/y/z` 是引用）

---

### 练习 1-4：元组解构

```rust
fn swap_pair<T, U>(pair: (T, U)) -> (U, T) {
    let (a, b) = pair;
    (b, a)
}
```

---

### 练习 1-5：`if let` 与 `while let`

```rust
fn process_queue(items: Vec<Option<i32>>) -> Vec<i32> {
    let mut nums = vec![];
    for item in items {
        if let Some(val) = item { nums.push(val); }
    }
    nums
}

fn drain_stack(stack: &mut Vec<i32>) {
    while let Some(top) = stack.pop() {
        println!("pop: {}", top);
    }
}
```

---

## Level 2：组合应用

### 练习 2-1：多重模式匹配

```rust
fn describe_pair(x: Option<i32>, y: Option<i32>) -> String {
    match (x, y) {
        (Some(a), Some(b)) if a == b => format!("相同: {}", a),
        (Some(a), Some(b)) => format!("不同: {} 和 {}", a, b),
        (Some(a), None) => format!("只有第一个: {}", a),
        (None, Some(b)) => format!("只有第二个: {}", b),
        (None, None) => "两者都没有".to_string(),
    }
}
```

---

### 练习 2-2：@ 绑定

```rust
fn classify_range(n: i32) -> String {
    match n {
        v @ 0..=9 => format!("个位数: {}", v),
        v @ 10..=99 => format!("两位数: {}", v),
        v @ 100..=999 => format!("三位数: {}", v),
        _ => "其他".to_string(),
    }
}
```

---

### 练习 2-3：ref 绑定

```rust
fn longest_str_ref(a: &String, b: &String) -> &String {
    match (a, b) {
        (ref x, ref y) if x.len() >= y.len() => x,
        _ => b,
    }
}
// 等价简写（Rust 会自动加 ref）：
fn longest_str(a: &String, b: &String) -> &String {
    if a.len() >= b.len() { a } else { b }
}
```

---

## Level 3：设计思考

### 练习 3-1：JSON 值建模

```rust
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn describe_json(val: &JsonValue) -> String {
    match val {
        JsonValue::Null => "null".into(),
        JsonValue::Bool(b) => format!("boolean({})", b),
        JsonValue::Number(n) => format!("number({})", n),
        JsonValue::String(s) => format!("string(\"{}\")", s),
        JsonValue::Array(arr) => format!("array[{}]", arr.len()),
        JsonValue::Object(map) => format!("object{{{}}}", map.len()),
    }
}
```

#### 为什么 Enum 比多个布尔字段好

用 `bool` 字段表达的"可能是 String 或 Number"在类型层面无法区分"同时为二者"或"两者都不"的非法状态。Enum 的每个变体互斥，类型系统直接排除了非法组合。

---

## 迁移思维练习

### 从 Python match 到 Rust match

Python 3.10+ 的 `match/case` 借鉴了 Rust 的模式匹配，但有两个关键区别：

1. **Python 不强制穷尽性**：遗漏分支不会报错，运行时静默跳过
2. **Python 的模式解构不涉及所有权**：Python 是引用语义，Rust 的 `match` 可能移动值

**迁移提示**：习惯 Python `match/case` 的开发者转到 Rust 后，最大的心态变化是"编译器替你检查所有遗漏"，这是安全保障而非限制。

---

*本章答案小结：模式匹配的核心价值不在于语法糖，而在于编译器强制穷尽性检查——这是从"运行时报错才发现的遗漏"到"编译期零遗漏"的转变。*
