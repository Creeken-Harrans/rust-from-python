# 猜数字游戏 — 参考实现说明

## 1. 需求拆分

| 层级 | 需求 | 实现位置 |
|------|------|---------|
| 核心 | 随机数生成 1-100 | `generate_secret()` |
| 核心 | 循环接收输入 | `game_loop()` 中的 `loop` |
| 核心 | 解析输入为 u32 | `parse_guess(&str) -> Result<u32, String>` |
| 核心 | 比较并提示 | `check_guess()` + `match Ordering` |
| 扩展 | 尝试次数跟踪 | `attempts: u32` 变量累加 |
| 扩展 | 非法输入处理 | `parse_guess` 返回 `Err`，`game_loop` 中 `continue` |
| 扩展 | EOF 处理 | `read_input` 检测 `Ok(0)` 字节读取 |
| 扩展 | 性能评价 | `print_performance(attempts)` 分档匹配 |

## 2. 推荐实现顺序

1. **先让游戏跑起来**：最简单文件，`loop` + `read_line` + `parse` + `if/else` 比较
2. **引入 `match`**：将 `if/else` 重构为 `match`，处理 `Result` 和 `Ordering`
3. **拆分函数**：提取 `generate_secret`、`read_input`、`parse_guess`、`check_guess`
4. **添加错误处理**：非法输入、EOF、范围检查
5. **添加功能**：尝试次数、性能评价、调试模式
6. **添加测试**：单元测试 + 集成测试

## 3. 模块划分（L3-1 参考）

```
src/
├── main.rs       // fn main() { game::game_loop(); }
├── lib.rs        // pub mod game; pub mod input; ...
├── game.rs       // pub fn game_loop() — 核心循环
├── input.rs      // pub fn read_input() -> io::Result<String>
│                 // pub fn parse_guess(input: &str) -> Result<u32, String>
├── random.rs     // pub fn generate_secret() -> u32
└── output.rs     // pub fn print_welcome()
                  // pub fn print_performance(attempts: u32)
```

依赖关系: `game` -> `input`, `random`, `output`。`input` `random` `output` 互不依赖。

## 4. 核心数据结构

### 游戏状态（L2-3 范围提示用）

```rust
struct GameState {
    secret: u32,
    attempts: u32,
    min_guess: u32,  // 当前已知下界（初始 = 1）
    max_guess: u32,  // 当前已知上界（初始 = 100）
}
```

### 配置（L3-4 用）

```rust
struct GameConfig {
    min: u32,        // 默认 1
    max: u32,        // 默认 100
    language: Lang,  // 默认中文
}

enum Lang { Zh, En }
```

### 排行榜条目（L3-3 用）

```rust
#[derive(Serialize, Deserialize)]
struct ScoreEntry {
    player_name: String,
    attempts: u32,
    difficulty: Difficulty,
    timestamp: String,  // ISO 8601
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum Difficulty { Easy, Medium, Hard }
```

## 5. 关键函数签名

```rust
// 核心函数
fn generate_secret() -> u32;
fn read_input() -> io::Result<String>;
fn parse_guess(input: &str) -> Result<u32, String>;  // 注意 &str 而非 String
fn check_guess(guess: u32, secret: u32) -> Ordering;
fn game_loop();
fn print_performance(attempts: u32);

// 扩展后的签名
fn generate_secret_range(min: u32, max: u32) -> u32;
fn game_loop_with_config(config: &GameConfig);

// 反向猜数
fn computer_guess() -> Result<u32, String>;  // 返回用了几次猜中
fn binary_search_next(min: u32, max: u32) -> u32;
```

## 6. 设计决策

### 6.1 `loop` vs `while`

**正确选择: `loop`**。理由：
- 退出条件来自内部多分支（`Ordering` 比较 + 错误处理），没有单一布尔条件
- `loop` 语义是"无限重复直到显式 break"
- Rust 编译器知道 `loop` 至少执行一次

**重玩重构时**（L2-2）：外层使用 `while` 或 `loop` 均可。推荐：

```rust
fn game_loop() {
    loop {  // 外层: 控制"是否再来一局"
        let secret = generate_secret();
        // 内层: 单局游戏（现有的 loop { ... break; }）
        let won = play_one_game(secret);
        if !ask_replay() { break; }
    }
}
```

将单局逻辑提取为 `play_one_game(secret: u32) -> bool`，返回是否猜中。

### 6.2 错误处理策略

| 错误类型 | 策略 | 理由 |
|---------|------|------|
| 解析失败 | `continue` | 用户可重新输入 |
| 范围越界 | `continue` | 提示并重新输入 |
| EOF | `return` | 用户主动退出 |
| I/O 错误 | `return` | 不可恢复 |

在 `main.rs` 中使用 `if let Err(e) = ...` 处理，不使用 `unwrap()`。

### 6.3 参数类型选择：`&str` 而非 `String`

`parse_guess(&str) -> Result<u32, String>`：
- 调用处：`parse_guess(&input)` — 借用
- 只读数据，不需要所有权
- 可用字符串字面量直接测试：`parse_guess("42")`
- 错误时返回 `Result<u32, String>` — `String` 是拥有的错误消息

## 7. 关键代码片段

### 7.1 二分搜索实现（L3-2 用）

```rust
fn computer_guess_loop() -> Result<u32, &'static str> {
    let mut low: u32 = 1;
    let mut high: u32 = 100;
    let mut attempts: u32 = 0;

    loop {
        if low > high {
            return Err("你的反馈有矛盾！我无法找到答案。");
        }
        let guess = low + (high - low) / 2;  // 防止溢出
        attempts += 1;
        println!("是 {} 吗？(+ 太小 / - 太大 / = 正确):", guess);
        match read_user_feedback()? {
            Feedback::TooSmall  => low = guess + 1,
            Feedback::TooLarge  => high = guess - 1,
            Feedback::Correct   => return Ok(attempts),
        }
    }
}
```

### 7.2 Entry API 变体模式（虽然本游戏不直接使用，但与其他项目对照）

```rust
// parse_guess 的错误处理变体（更 Rusty）
fn parse_guess_v2(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err("输入为空".into())
    } else {
        trimmed.parse::<u32>().map_err(|_| format!("'{}' 不是有效数字", trimmed))
    }
}
```

### 7.3 范围验证的两种写法

```rust
// 方式1: RangeInclusive + contains
if !(1..=100).contains(&guess) { ... }

// 方式2: 手动比较
if guess < 1 || guess > 100 { ... }

// 方式3: match 守卫（更 Rusty，适合多分支）
match guess {
    n if n < 1   => println!("太小"),
    n if n > 100 => println!("太大"),
    _ => { /* 合法 */ }
}
```

## 8. 测试策略

| 层次 | 测试什么 | 如何测 |
|------|---------|--------|
| 单元测试 | `parse_guess` 处理各种输入 | `assert_eq!(parse_guess("42").unwrap(), 42)` |
| 单元测试 | `check_guess` 比较逻辑 | `assert_eq!(check_guess(30, 50), Ordering::Less)` |
| 单元测试 | `print_performance` 边界值 | 验证不同 attempts 值不会 panic |
| 集成测试 | 端到端：解析后比较 | `tests/integration_test.rs` 中测试 `parse_guess` |

**测试文件示例** (`tests/integration_test.rs`):

```rust
use guessing_game::parse_guess;

#[test]
fn test_parse_edge_cases() {
    assert!(parse_guess("").is_err());
    assert!(parse_guess("abc").is_err());
    assert!(parse_guess("-5").is_err());
    assert!(parse_guess("99999999999999999999").is_err());
    assert_eq!(parse_guess("  42  ").unwrap(), 42);
    assert_eq!(parse_guess("1").unwrap(), 1);
    assert_eq!(parse_guess("100").unwrap(), 100);
}
```

## 9. 常见失败方式

### 9.1 忘记修改所有相关位置

改范围从 100 到 50，只改了 `generate_secret()` 中的 `gen_range(1..=50)`，但忘记改：
- `if !(1..=100).contains(&guess)` 中的 `100`
- `print_welcome()` 中的文字 "1 到 100"
- `print_performance()` 中计算理论最优时用到 `100`

**解**: 将范围提取为常量或配置结构体。

```rust
const MIN: u32 = 1;
const MAX: u32 = 100;
// 所有引用处使用 MIN 和 MAX
```

### 9.2 `read_line` 保留换行符

`stdin().read_line(&mut input)` 会保留 `\n`。如果直接对原始 `input` 做比较而不 `trim()`，`"42\n" != "42"`。

**解**: 在 `parse_guess` 中 `input.trim()` 是必须的。

### 9.3 误用 `if let` 代替 `match`

```rust
// 错误: 忘记处理 Err 分支
if let Ok(num) = trimmed.parse::<u32>() {
    // 只处理成功情况，错误被静默忽略
}
```

**解**: 对 `Result` 使用 `match`，确保两个分支都处理。

### 9.4 EOF 处理位置错误

如果在 `read_input` 中将 EOF 静默返回空字符串，`game_loop` 无法区分"用户输入空行"和"EOF"。

**解**: 在 `read_input` 中将 `Ok(0)` 映射为 `UnexpectedEof` 错误，让上层决定如何处理。

### 9.5 重玩时的状态残留

```rust
// 错误: 重玩时 attempts 和范围未重置
let mut attempts: u32 = 0; // 在 loop 外层
loop { // 外层的重玩循环
    loop { // 内层的猜数循环
        attempts += 1; // 不断累加！
    }
}
```

**解**: 每次新游戏开始时重置所有状态变量。

## 10. 可选扩展

| 方向 | 难度 | 涉及知识点 |
|------|------|-----------|
| 难度选择 | 低 | 函数参数化、match |
| 重玩功能 | 低 | 嵌套循环、用户交互 |
| 范围提示 | 中 | 状态管理、RangeInclusive |
| 二进制搜索 | 中 | 算法、输入验证、溢出处理 |
| 排行榜系统 | 高 | 文件 I/O、序列化、排序 |
| 配置系统 | 高 | clap、toml、配置优先级 |
| GUI 界面 | 高 | egui/iced crate、事件循环 |
| 网络对战 | 高 | tokio、TCP、协议设计 |
