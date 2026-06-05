# 第八章练习题：结构体与方法

## 练习说明

- **Level 1**：基础练习，巩固核心语法，预计 10-15 分钟/题
- **Level 2**：进阶练习，需要组合多个概念，预计 20-30 分钟/题
- **Level 3**：挑战练习，需要独立设计和推理，预计 40-60 分钟
- **思考题**：不需要写代码，但值得深入思考

所有练习请在 `08_structs_methods_associated_functions/` 目录下完成。
推荐为每道 Level 2 和 Level 3 题目创建独立的 `.rs` 文件（或在 `main.rs` 中用模块组织）。

编译命令：
```bash
cargo build          # 完整编译
cargo run            # 编译并运行
cargo check          # 仅类型检查（更快）
cargo clippy         # 运行 linter（需要安装：rustup component add clippy）
```

---

## Level 1：基础巩固

### L1-1: Circle 结构体

**目标**：练习定义命名字段结构体（Named Field Struct）和基本方法。

**要求**：
1. 定义一个 `Circle` 结构体，包含一个字段 `radius: f64`
2. 为 `Circle` 实现以下方法：
   - `new(radius: f64) -> Self` — 关联函数，创建新圆
   - `area(&self) -> f64` — 返回面积（π * r²）
   - `circumference(&self) -> f64` — 返回周长（2 * π * r）
   - `diameter(&self) -> f64` — 返回直径（2 * r）
3. 派生 `Debug`，在 `main()` 中创建圆并打印面积和周长
4. 使用 `std::f64::consts::PI` 获取 π 值

**预期输出示例**：
```
圆: Circle { radius: 5.0 }
面积: 78.54
周长: 31.42
直径: 10.00
```

---

### L1-2: RGB 颜色元组结构体

**目标**：练习定义元组结构体（Tuple Struct）。

**要求**：
1. 定义一个元组结构体 `Color(u8, u8, u8)`，分别表示 R、G、B 分量
2. 为 `Color` 实现以下方法：
   - `new(r: u8, g: u8, b: u8) -> Self` — 关联函数
   - `red(&self) -> u8`、`green(&self) -> u8`、`blue(&self) -> u8` — 返回各分量（用 `self.0`、`self.1`、`self.2` 访问）
   - `brightness(&self) -> f64` — 返回亮度（(R + G + B) / 3.0 / 255.0）
   - `is_grayscale(&self) -> bool` — 当三个分量相等时返回 `true`
3. 派生 `Debug`，创建几种颜色并测试

**预期输出示例**：
```
颜色: Color(255, 128, 64)
红色分量: 255
绿色分量: 128
蓝色分量: 64
亮度: 0.58
灰度? false
纯黑灰度? true
```

---

### L1-3: 结构体更新语法

**目标**：练习结构体更新语法（Struct Update Syntax）。

**要求**：
1. 使用本章代码中的 `Rectangle` 结构体
2. 创建一个基准矩形 `base`
3. 使用结构体更新语法 `..base` 创建三个不同的矩形，每次只覆盖一个字段
4. 验证 `base` 在更新语法后是否仍然可用（因为是 `f64`，实现了 `Copy`）
5. 打印 `{:#?}` 格式验证结果

**提示**：`..` 语法会移动未实现 `Copy` 的字段。因为 `f64` 实现了 `Copy`，所以 `base` 之后仍可用。

---

## Level 2：进阶练习

### L2-1: 银行账户（BankAccount）

**目标**：综合练习 `&self`、`&mut self` 和多种方法。

**要求**：
1. 定义 `BankAccount` 结构体：
   ```rust
   struct BankAccount {
       owner: String,
       balance: f64,
       account_number: u64,
   }
   ```
2. 实现以下方法：
   - `new(owner: String, account_number: u64) -> Self` — 初始余额为 0.0
   - `deposit(&mut self, amount: f64)` — 存款，如果 `amount <= 0` 则打印错误
   - `withdraw(&mut self, amount: f64) -> bool` — 取款，余额不足返回 `false`，成功返回 `true`
   - `balance(&self) -> f64` — 查询余额
   - `transfer(&mut self, to: &mut BankAccount, amount: f64) -> bool` — 转账，从 `self` 转给 `to`
   - `summary(&self) -> String` — 返回格式化的账户摘要字符串
3. 在 `main()` 中创建两个账户，执行一系列操作并打印结果
4. **思考**：`transfer` 方法为什么需要 `&mut self` 和 `&mut to` 两个可变借用？

**预期输出示例**：
```
--- 创建账户 ---
账户 1: 张三 #10001, 余额: 0.00
账户 2: 李四 #10002, 余额: 0.00

--- 操作 ---
存款 1000.00 -> 账户 1, 余额: 1000.00
取款 200.00 <- 账户 1, 余额: 800.00
取款 2000.00 <- 账户 1: 余额不足!

转账 300.00: 账户 1 -> 账户 2
账户 1 余额: 500.00
账户 2 余额: 300.00
```

---

### L2-2: 书籍与图书馆（Library）

**目标**：练习结构体数组操作、`&self` 方法与逻辑组合。

**要求**：
1. 定义 `Book` 结构体：
   ```rust
   struct Book {
       title: String,
       author: String,
       pages: u32,
       is_available: bool,
   }
   ```
2. 为 `Book` 实现：
   - `new(title: &str, author: &str, pages: u32) -> Self` — 创建书籍，`is_available` 初始为 `true`
   - `borrow(&mut self) -> bool` — 借书：如果可用则设为不可用并返回 `true`，否则返回 `false`
   - `return_book(&mut self)` — 还书：设为可用
   - `summary(&self) -> String` — 格式化的摘要
3. 定义 `Library` 结构体，包含一个 `Vec<Book>`：
   ```rust
   struct Library {
       name: String,
       books: Vec<Book>,
   }
   ```
4. 为 `Library` 实现：
   - `new(name: &str) -> Self` — 创建空图书馆
   - `add_book(&mut self, book: Book)` — 添加书籍
   - `find_by_title(&self, title: &str) -> Option<&Book>` — 按标题查找，返回 `Option`
   - `find_by_author(&self, author: &str) -> Vec<&Book>` — 按作者查找，返回所有匹配的书
   - `available_count(&self) -> usize` — 当前可借数量
5. 在 `main()` 中创建图书馆、添加书籍、模拟借还操作

**提示**：`find_by_title` 返回 `Option<&Book>`，这是 Rust 中常见的模式，避免了空指针问题。

---

## Level 3：挑战练习

### L3-1: 二维向量数学库

**目标**：独立设计一套完整的类型和方法体系，考验结构体、方法、运算符重载的综合能力。

**要求**：
1. 定义 `Vec2` 元组结构体 `Vec2(f64, f64)` 表示二维向量，派生 `Debug`、`Clone`、`Copy`、`PartialEq`
2. 实现以下关联函数和方法：

   **关联函数**：
   - `new(x: f64, y: f64) -> Self` — 新向量
   - `zero() -> Self` — 零向量 (0, 0)
   - `unit_x() -> Self` — X 轴单位向量 (1, 0)
   - `unit_y() -> Self` — Y 轴单位向量 (0, 1)

   **&self 方法**：
   - `magnitude(&self) -> f64` — 向量长度
   - `magnitude_squared(&self) -> f64` — 长度平方（避免开根号，用于比较）
   - `normalize(&self) -> Self` — 返回单位向量（零向量返回零向量）
   - `dot(&self, other: &Vec2) -> f64` — 点积
   - `angle_between(&self, other: &Vec2) -> f64` — 夹角（弧度），用 `dot` 和 `magnitude` 计算
   - `distance_to(&self, other: &Vec2) -> f64` — 两点距离

   **&mut self 方法**：
   - `scale(&mut self, factor: f64)` — 原地缩放
   - `add_vec(&mut self, other: &Vec2)` — 原地加另一个向量

   **self 方法**：
   - `negated(self) -> Self` — 返回反向向量
   - `rotated(self, angle_rad: f64) -> Self` — 返回旋转后的向量

3. 实现 `std::ops::Add` 和 `std::ops::Sub` trait 让向量可以使用 `+` 和 `-` 运算符
4. 实现 `std::ops::Mul<f64>` 和 `std::ops::Div<f64>` 让向量可以乘以/除以标量
5. 在 `main()` 中编写综合测试：
   - 创建几个向量
   - 测试加减乘除运算符
   - 测试点积和夹角
   - 测试旋转（旋转 90 度应把 (1, 0) 变成接近 (0, 1)）

**实现运算符提示**：

```rust
use std::ops::{Add, Sub, Mul, Div};

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
```

**旋转公式**（绕原点逆时针旋转 angle 弧度）：
- `new_x = x * cos(angle) - y * sin(angle)`
- `new_y = x * sin(angle) + y * cos(angle)`

**预期输出示例**：
```
v1 = Vec2(3.0, 4.0)
v2 = Vec2(1.0, 2.0)
v1 + v2 = Vec2(4.0, 6.0)
v1 - v2 = Vec2(2.0, 2.0)
v1 * 2.0 = Vec2(6.0, 8.0)
|v1| = 5.0000
v1 dot v2 = 11.0000
v1 旋转 90° = Vec2(-4.0000, 3.0000)   # 近似值
```

---

## 思考题

### 为什么 Rust 把数据（struct）和行为（impl）分开，而不是像 Python/Java 那样放在一个 class 里？

请从以下几个角度思考，并写出你的理解（不要求标准答案，关键是有自己的思考过程）：

1. **模块化与灵活性**：将数据和行为分离后，你可以为同一个 struct 写多个 `impl` 块。这在什么场景下有用？试举一个具体例子。

2. **trait 与接口**：Rust 的 trait 定义了一组行为规范，任何 struct 都可以通过 `impl Trait for Struct` 来实现这个 trait。如果数据和方法耦合在一起，trait 的实现会变得怎样？

3. **可见性控制**：在 Rust 中，struct 的字段和 impl 中的方法可以有不同的 `pub` 可见性。这在哪些设计场景中很重要？

4. **从 Python 视角**：如果你是一个 Python 开发者，习惯了把所有东西塞进 class，Rust 的这种分离会让你感到不便还是更清晰？为什么？

**提示**（读过再思考）：在大型项目中，同一个 struct 可能需要实现来自不同 crate 的多个 trait。如果数据和方法耦合，你将被迫把所有实现挤在一个地方。

---

## 推荐命令速查

```bash
# 在项目目录下执行

cargo new l1_circle           # 创建新的 binary crate（如需要独立项目）
cargo build                   # 编译当前 crate
cargo run                     # 运行
cargo check                   # 仅类型检查，不生成二进制文件（速度最快）
cargo clippy                  # Rust 官方 linter，给出惯用写法建议
cargo fmt                     # 自动格式化代码

# 查看文档
rustup doc --std               # 在浏览器中打开标准库文档

# 编译特定练习（如果你把练习放在独立的 .rs 文件中）
rustc --edition 2024 l1_circle.rs -o l1_circle && ./l1_circle
```
