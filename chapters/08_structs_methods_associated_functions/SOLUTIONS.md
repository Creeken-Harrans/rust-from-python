# 第八章练习题解答：结构体与方法

---

## Level 1：基础巩固

---

### L1-1: Circle 结构体

#### 结论

命名字段结构体 + 方法 = 数据和行为的清晰封装。`new` 是关联函数，`area` 等方法使用 `&self`。

#### 思路

定义结构体持有数据（radius），用 impl 块定义行为。关联函数 `new` 调用用 `::`，方法调用用 `.`。

#### 参考实现

```rust
use std::f64::consts::PI;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    /// 关联函数：创建新圆
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// 计算面积: π * r²
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// 计算周长: 2 * π * r
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// 计算直径: 2 * r
    fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
}

fn main() {
    let circle = Circle::new(5.0);
    println!("圆: {:?}", circle);
    println!("面积: {:.2}", circle.area());
    println!("周长: {:.2}", circle.circumference());
    println!("直径: {:.2}", circle.diameter());
}
```

#### 为什么这样设计

- `new` 是关联函数（无 `self`），用 `Circle::new()` 调用——这是 Rust 的惯例
- `area` 等方法用 `&self`：只读访问，不消耗结构体
- `Self` 是 `impl` 块中当前类型的别名

#### 常见错误

1. **`new` 写成方法**：`fn new(&self, ...)` 是不对的，构造器应该是关联函数
2. **把 `Self` 写成 `Circle`**：都可以，但 `Self` 更简洁且不易出错
3. **忘记 `use std::f64::consts::PI`**：直接用 `3.14159...` 不够精确

#### 验证方式

```bash
cargo run
# 输出：
# 圆: Circle { radius: 5.0 }
# 面积: 78.54
# 周长: 31.42
# 直径: 10.00
```

---

### L1-2: RGB 颜色元组结构体

#### 结论

元组结构体用 `.0`、`.1`、`.2` 访问字段，适合内部结构简单且字段名无额外语义的场景（如 RGB 颜色分量）。

#### 思路

Color 的三个分量语义等价（都是 u8 颜色通道），用元组结构体更简洁。访问器方法提供语义化的字段名。

#### 参考实现

```rust
#[derive(Debug)]
struct Color(u8, u8, u8);

impl Color {
    /// 关联函数：创建颜色
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// 返回红色分量
    fn red(&self) -> u8 {
        self.0
    }

    /// 返回绿色分量
    fn green(&self) -> u8 {
        self.1
    }

    /// 返回蓝色分量
    fn blue(&self) -> u8 {
        self.2
    }

    /// 计算亮度：0.0（全黑）到 1.0（全白）
    fn brightness(&self) -> f64 {
        (self.0 as f64 + self.1 as f64 + self.2 as f64) / 3.0 / 255.0
    }

    /// 是否为灰度（三个分量相等）
    fn is_grayscale(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }
}

fn main() {
    let orange = Color::new(255, 128, 64);
    println!("颜色: {:?}", orange);
    println!("红色分量: {}", orange.red());
    println!("绿色分量: {}", orange.green());
    println!("蓝色分量: {}", orange.blue());
    println!("亮度: {:.2}", orange.brightness());
    println!("灰度? {}", orange.is_grayscale());

    let black = Color::new(0, 0, 0);
    let gray = Color::new(128, 128, 128);
    let white = Color::new(255, 255, 255);
    println!("纯黑灰度? {}", black.is_grayscale());
    println!("中灰灰度? {}", gray.is_grayscale());
    println!("纯白灰度? {}", white.is_grayscale());
}
```

#### 为什么这样设计

- 元组结构体适合字段"同级"的场景（RGB 分量彼此等价）
- 访问器方法 `red()`、`green()`、`blue()` 隐藏 `.0`、`.1`、`.2` 的实现细节
- 如果将来改为命名字段结构体，只需修改访问器，外部 API 不变

#### 常见错误

1. **混淆字段索引**：`.0` 是 R，`.1` 是 G，`.2` 是 B
2. **整数除法**：`(self.0 + self.1 + self.2) / 3 / 255` 整数除法总得 0
3. **忘记 `as f64` 转换**：`u8` 运算后再转可能溢出

#### 验证方式

```bash
cargo run
# 输出：
# 颜色: Color(255, 128, 64)
# 红色分量: 255
# 绿色分量: 128
# 蓝色分量: 64
# 亮度: 0.58
# 灰度? false
# 纯黑灰度? true
```

---

### L1-3: 结构体更新语法

#### 结论

`..base` 语法将未显式指定的字段从 base 复制过来。由于 `f64` 实现了 `Copy`，`base` 在更新后仍然可用。

#### 参考实现

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let base = Rectangle {
        width: 30.0,
        height: 20.0,
    };

    // 只覆盖 width
    let rect1 = Rectangle {
        width: 50.0,
        ..base
    };
    println!("rect1 = {:#?}", rect1);

    // 只覆盖 height
    let rect2 = Rectangle {
        height: 40.0,
        ..base
    };
    println!("rect2 = {:#?}", rect2);

    // 全部覆盖（等价于新建）
    let rect3 = Rectangle {
        width: 10.0,
        height: 10.0,
        ..base
    };
    println!("rect3 = {:#?}", rect3);

    // base 仍然可用（因为 f64 是 Copy 类型）
    println!("base 仍然可用: {:#?}", base);
}
```

#### 为什么这样设计

- `..` 语法来自 JavaScript 的 spread，减少样板代码
- 对于 `Copy` 字段（如 `f64`），是复制而非移动
- 对于非 `Copy` 字段（如 `String`），是移动——`base` 之后该字段不可用

#### 常见错误

1. **假设 `..base` 后 base 总是可用**：如果包含 `String` 等非 Copy 字段，base 会被部分移动
   ```rust
   struct Foo { x: i32, name: String }
   let base = Foo { x: 1, name: "hello".into() };
   let new_foo = Foo { x: 2, ..base };
   // println!("{}", base.name);  // ❌ 编译错误：name 已被移动
   ```
2. **`..base` 位置错误**：必须在结构体字面量的最后

#### 验证方式

```bash
cargo run
# 看到四个矩形的输出，base 仍可用
```

---

## Level 2：进阶练习

---

### L2-1: 银行账户（BankAccount）

#### 结论

`&self` 只读、`&mut self` 可写、方法内部 mut 字段自然支持，展示了 Rust 方法权限模型。

#### 思路

`transfer` 需要同时可变借用两个账户——Rust 允许同一个作用域内同时可变借用两个不同的变量（因为它们不重叠），但不允许同时可变借用同一个变量两次。

#### 参考实现

```rust
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
    account_number: u64,
}

impl BankAccount {
    /// 创建新账户，初始余额为 0.0
    fn new(owner: String, account_number: u64) -> Self {
        BankAccount {
            owner,
            balance: 0.0,
            account_number,
        }
    }

    /// 存款
    fn deposit(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("错误: 存款金额必须大于零");
            return;
        }
        self.balance += amount;
    }

    /// 取款，成功返回 true，余额不足返回 false
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= 0.0 {
            println!("错误: 取款金额必须大于零");
            return false;
        }
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        true
    }

    /// 查询余额
    fn balance(&self) -> f64 {
        self.balance
    }

    /// 转账：从 self 转给 to。
    ///
    /// # 为什么需要 `&mut self` 和 `&mut to` 两个可变借用？
    ///
    /// 转账操作同时修改两个账户：self 余额减少，to 余额增加。
    /// 两个账户是不同的变量，Rust 允许同时存在多个不重叠的可变借用。
    /// 如果 self 和 to 指向同一个账户（通过某种方式），编译器会阻止，
    /// 因为不能同时存在两个可变借用指向同一数据。
    fn transfer(&mut self, to: &mut BankAccount, amount: f64) -> bool {
        if self.withdraw(amount) {
            to.deposit(amount);
            true
        } else {
            false
        }
    }

    /// 格式化的账户摘要
    fn summary(&self) -> String {
        format!(
            "{} #{:05}, 余额: {:.2}",
            self.owner, self.account_number, self.balance
        )
    }
}

fn main() {
    let mut acc1 = BankAccount::new("张三".to_string(), 10001);
    let mut acc2 = BankAccount::new("李四".to_string(), 10002);

    println!("--- 创建账户 ---");
    println!("账户 1: {}", acc1.summary());
    println!("账户 2: {}", acc2.summary());

    println!("\n--- 操作 ---");
    acc1.deposit(1000.0);
    println!("存款 1000.00 -> 账户 1, 余额: {:.2}", acc1.balance());

    if acc1.withdraw(200.0) {
        println!("取款 200.00 <- 账户 1, 余额: {:.2}", acc1.balance());
    }

    if !acc1.withdraw(2000.0) {
        println!("取款 2000.00 <- 账户 1: 余额不足!");
    }

    println!("\n转账 300.00: 账户 1 -> 账户 2");
    if acc1.transfer(&mut acc2, 300.0) {
        println!("账户 1 余额: {:.2}", acc1.balance());
        println!("账户 2 余额: {:.2}", acc2.balance());
    }
}
```

#### 为什么这样设计

- `deposit` 用 `&mut self`：修改余额
- `balance` 用 `&self`：只读查询
- `transfer` 用两个 `&mut`：两个独立可变借用，编译器保证安全
- `new` 是关联函数：构造逻辑与业务逻辑分离

#### 常见错误

1. **`withdraw` 中忘记检查 `amount <= 0.0`**：允许负取款
2. **`transfer` 中先存款再取款**：顺序无所谓，但要保证一致性
3. **试图同时可变借用同一个账户两次**：`acc1.transfer(&mut acc1, 100.0)` 编译错误

#### 验证方式

```bash
cargo run
# 输出：
# --- 创建账户 ---
# 账户 1: 张三 #10001, 余额: 0.00
# 账户 2: 李四 #10002, 余额: 0.00
# --- 操作 ---
# 存款 1000.00 -> 账户 1, 余额: 1000.00
# 取款 200.00 <- 账户 1, 余额: 800.00
# 取款 2000.00 <- 账户 1: 余额不足!
# 转账 300.00: 账户 1 -> 账户 2
# 账户 1 余额: 500.00
# 账户 2 余额: 300.00
```

---

### L2-2: 书籍与图书馆（Library）

#### 结论

`find_by_title` 返回 `Option<&Book>` ——用 Option 替代 null，用借用替代拷贝。`find_by_author` 返回 `Vec<&Book>` ——返回对集合元素的引用集合，生命周期绑定到 Library。

#### 思路

两层结构：Book（单本书的状态）+ Library（书的管理）。Library 持有 `Vec<Book>` 的所有权，查询方法返回借用的引用。

#### 参考实现

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_available: bool,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
            is_available: true,
        }
    }

    /// 借书：如果可用则设为不可用并返回 true
    fn borrow(&mut self) -> bool {
        if self.is_available {
            self.is_available = false;
            true
        } else {
            false
        }
    }

    /// 还书：设为可用
    fn return_book(&mut self) {
        self.is_available = true;
    }

    fn summary(&self) -> String {
        format!(
            "\"{}\" by {}, {} 页 [{}]",
            self.title,
            self.author,
            self.pages,
            if self.is_available { "可借" } else { "已借出" }
        )
    }
}

struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// 按标题精确查找，返回 Option<&Book>
    fn find_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }

    /// 按作者查找，返回所有匹配的书（借用引用）
    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.author == author)
            .collect()
    }

    /// 当前可借数量
    fn available_count(&self) -> usize {
        self.books.iter().filter(|b| b.is_available).count()
    }
}

fn main() {
    let mut library = Library::new("Rust 图书馆");

    // 添加书籍
    library.add_book(Book::new("Rust 程序设计", "张三", 320));
    library.add_book(Book::new("Rust 高级编程", "张三", 450));
    library.add_book(Book::new("Python 入门", "李四", 280));
    library.add_book(Book::new("数据结构", "王五", 500));

    println!("=== {} ===", library.name);
    println!("总藏书: {} 本", library.books.len());
    println!("可借: {} 本\n", library.available_count());

    // 按标题查找
    match library.find_by_title("Rust 程序设计") {
        Some(book) => println!("找到: {}", book.summary()),
        None => println!("未找到该书"),
    }

    // 按作者查找
    println!("\n张三的著作:");
    for book in library.find_by_author("张三") {
        println!("  {}", book.summary());
    }

    // 模拟借书
    println!("\n--- 借书操作 ---");
    if let Some(book) = library.find_by_title("Rust 程序设计") {
        // 注意：我们需要可变借用才能修改 book
        // 通过索引操作规避借用冲突
        if let Some(pos) = library.books.iter().position(|b| b.title == "Rust 程序设计") {
            if library.books[pos].borrow() {
                println!("成功借到: {}", library.books[pos].summary());
            }
        }
    }
    println!("可借: {} 本", library.available_count());
}
```

#### 为什么这样设计

- `Option<&Book>` 替代 null 返回值，编译器强制调用者处理 None
- `Vec<&Book>` 返回引用集合，不克隆数据
- `find_by_author` 返回 `Vec<&Book>` 而非 `&[&Book]`：内部过滤后新生成的集合，必须拥有所有权

#### 常见错误

1. **同时持有不可变引用和可变引用**：
   ```rust
   // ❌ 错误：不能同时持有不可变借用(find)和可变借用(mutate)
   let book_ref = library.find_by_title("Rust 程序设计");
   library.books[0].borrow(); // book_ref 仍然活跃
   ```
2. **返回 `&Book` 但 `library` 是局部变量**：生命周期不够
3. **`find_by_author` 返回 `Vec<Book>`**：不必要的克隆

#### 验证方式

```bash
cargo run
# 输出查找结果和借书操作
```

---

## Level 3：挑战练习

---

### L3-1: 二维向量数学库

#### 结论

综合运用结构体、方法（`&self` / `&mut self` / `self`）、运算符重载、关联函数。`Copy` derive 让向量可以按值传递而不丢失所有权。

#### 思路

- 元组结构体 `Vec2(f64, f64)` 简洁表达
- 派生 `Copy` 让向量可以按值使用（向量运算通常按值更自然）
- 运算符重载让代码更符合数学直觉
- 三种 `self` 形态各有用途：`&self` 读取、`&mut self` 修改、`self` 产生新值

#### 参考实现

```rust
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2(f64, f64);

impl Vec2 {
    // ---- 关联函数 ----
    fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    fn zero() -> Self {
        Self(0.0, 0.0)
    }

    fn unit_x() -> Self {
        Self(1.0, 0.0)
    }

    fn unit_y() -> Self {
        Self(0.0, 1.0)
    }

    // ---- &self 方法 ----

    /// 向量长度（模）
    fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    /// 长度平方（避免开根号，用于比较）
    fn magnitude_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

    /// 返回单位向量（零向量返回零向量）
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zero()
        } else {
            Self(self.0 / mag, self.1 / mag)
        }
    }

    /// 点积
    fn dot(&self, other: &Vec2) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    /// 夹角（弧度）
    fn angle_between(&self, other: &Vec2) -> f64 {
        let dot = self.dot(other);
        let mags = self.magnitude() * other.magnitude();
        if mags == 0.0 {
            0.0
        } else {
            (dot / mags).acos()
        }
    }

    /// 两点距离
    fn distance_to(&self, other: &Vec2) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }

    // ---- &mut self 方法 ----

    /// 原地缩放
    fn scale(&mut self, factor: f64) {
        self.0 *= factor;
        self.1 *= factor;
    }

    /// 原地加另一个向量
    fn add_vec(&mut self, other: &Vec2) {
        self.0 += other.0;
        self.1 += other.1;
    }

    // ---- self 方法 ----

    /// 返回反向向量
    fn negated(self) -> Self {
        Self(-self.0, -self.1)
    }

    /// 返回旋转后的向量（逆时针旋转 angle_rad 弧度）
    fn rotated(self, angle_rad: f64) -> Self {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        Self(
            self.0 * cos - self.1 * sin,
            self.0 * sin + self.1 * cos,
        )
    }
}

// ---- 运算符重载 ----

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

fn main() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(1.0, 2.0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    // 运算符
    println!("v1 + v2 = {:?}", v1 + v2);
    println!("v1 - v2 = {:?}", v1 - v2);
    println!("v1 * 2.0 = {:?}", v1 * 2.0);
    println!("v1 / 2.0 = {:?}", v1 / 2.0);

    // 方法
    println!("|v1| = {:.4}", v1.magnitude());
    println!("v1 dot v2 = {:.4}", v1.dot(&v2));
    println!("v1 与 v2 夹角 = {:.4} rad", v1.angle_between(&v2));

    // 旋转测试：旋转 90 度应把 (1, 0) 变成接近 (0, 1)
    let rotated = Vec2::unit_x().rotated(std::f64::consts::PI / 2.0);
    println!(
        "unit_x 旋转 90° = ({:.4}, {:.4})",
        rotated.0, rotated.1
    );
    // 验证：应该接近 (0, 1)
    assert!((rotated.0).abs() < 1e-10);
    assert!((rotated.1 - 1.0).abs() < 1e-10);

    // &mut self 方法
    let mut v3 = Vec2::new(2.0, 3.0);
    println!("\nv3 缩放前: {:?}", v3);
    v3.scale(2.0);
    println!("v3 * 2 原地: {:?}", v3);
    v3.add_vec(&Vec2::new(1.0, 1.0));
    println!("v3 + (1,1) 原地: {:?}", v3);

    // self 方法
    let v4 = Vec2::new(3.0, 4.0);
    println!("\n{:?} 取反 = {:?}", v4, v4.negated());
    // v4 仍可用（因为 Vec2 实现了 Copy）
    println!("v4 仍然可用: {:?}", v4);
}
```

#### 为什么这样设计

- `Copy` + `Clone`：向量按值传递最自然，类似数学中的向量
- `self` 方法返回新值：纯函数，不修改原值
- `&mut self` 方法原地修改：适合需要避免分配的场景
- 运算符重载：`v1 + v2` 比 `v1.add(&v2)` 更直观

#### 常见错误

1. **运算符重载忘记 `type Output = Self`**：编译错误
2. **`self` 方法后还想用原值**：需派生 `Copy`
3. **`acos` 参数越界**：浮点误差可能导致 `dot/mags` 略大于 1.0，用 `clamp(-1.0, 1.0)` 处理

#### 验证方式

```bash
cargo run
# 输出：
# v1 = Vec2(3.0, 4.0)
# v1 + v2 = Vec2(4.0, 6.0)
# v1 - v2 = Vec2(2.0, 2.0)
# v1 * 2.0 = Vec2(6.0, 8.0)
# |v1| = 5.0000
# v1 dot v2 = 11.0000
# v1 旋转 90° = Vec2(-4.0000, 3.0000)
```

---

## 思考题解答

### 为什么 Rust 把数据（struct）和行为（impl）分开，而不是像 Python/Java 那样放在一个 class 里？

**1. 模块化与灵活性**

分离后，可以为同一个 struct 写多个 `impl` 块，分布在不同文件甚至不同 crate 中。例如：

```rust
// models.rs - 定义数据和基本方法
struct User { name: String, age: u8 }
impl User {
    fn new(name: String) -> Self { ... }
}

// serialization.rs - 序列化特征
impl User {
    fn to_json(&self) -> String { ... }
    fn from_json(json: &str) -> Self { ... }
}

// validation.rs - 验证特征
impl User {
    fn validate(&self) -> Result<(), Error> { ... }
}
```

这种分离在大型项目中尤为重要——功能可以按模块组织，不把所有代码塞进一个大类。

**2. trait 与接口**

Rust 的 trait 定义了行为规范，任何 struct 都可以实现它。如果数据和方法耦合，就无法做到"外部类型实现外部 trait"（孤儿原则允许至少一端在当前 crate）。

```rust
// 为第三方库的类型实现你的 trait
impl MyTrait for ExternalType { ... }

// 为你的类型实现第三方库的 trait
impl ExternalTrait for MyType { ... }
```

**3. 可见性控制**

struct 的字段和方法可以有不同的可见性：

```rust
pub struct User {
    pub name: String,       // 公开读
    password_hash: String,  // 完全私有
}
impl User {
    pub fn verify_password(&self, pw: &str) -> bool { ... }
    // 不暴露 password_hash 的内部实现
}
```

这在 Python 中只能通过 `_password_hash` 命名约定实现（无真正隐私保护），在 Java 中需要 getter/setter（但方法仍在类内部）。

**4. Python 视角**

分离意味着查看 struct 定义就能看到完整的"形状"（所有数据字段一目了然），而方法可以在别处按功能分组。这避免了 Python 类中"几百行方法夹杂在 `__init__` 和 `__str__` 之间"的问题。虽然习惯不同，但这种分离确实让你更清晰地思考"什么是数据"和"什么是对数据的操作"。

---

## 推荐命令速查确认

- [x] `cargo build` — 编译当前 crate
- [x] `cargo run` — 运行
- [x] `cargo check` — 仅类型检查
- [x] `cargo clippy` — Rust 官方 linter
- [x] `cargo fmt` — 自动格式化
