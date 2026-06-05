//! # 结构体与方法 — 自定义数据类型和行为
//!
//! 本章展示 Rust 中三种结构体（命名字段、元组、单元）的定义与使用，
//! 以及方法、关联函数和 `self` 接收者的三种形态。

// ---------------------------------------------------------------------------
// 1. 命名字段结构体 (Named Field Struct)
// ---------------------------------------------------------------------------

/// 矩形：由宽度和高度定义的二维形状。
#[derive(Debug)]
struct Rectangle {
    /// 矩形宽度（X 轴方向）
    width: f64,
    /// 矩形高度（Y 轴方向）
    height: f64,
}

// ---------------------------------------------------------------------------
// 2. impl 块 — 方法 + 关联函数
// ---------------------------------------------------------------------------

impl Rectangle {
    /// 关联函数（不是方法）：用给定的宽和高创建新的 `Rectangle`。
    ///
    /// 关联函数不以 `self` 为第一参数，调用时使用 `Rectangle::new(w, h)`。
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// 关联函数：创建一个正方形 —— 宽高相等的矩形。
    ///
    /// 这是另一个关联函数，演示如何从特殊输入构造结构体。
    pub fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // ---- &self 方法（不可变借用）----

    /// 计算矩形面积。
    ///
    /// `&self` 表示只读借用：方法可以读取字段，但不能修改。
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// 计算矩形周长。
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// 判断当前矩形是否能完全容纳另一个矩形。
    ///
    /// `can_hold` 不要求旋转：只有当 `self` 在两个维度上都 >=
    /// `other` 时才返回 `true`。
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// 对角线长度（使用勾股定理）。
    pub fn diagonal(&self) -> f64 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }

    // ---- &mut self 方法（可变借用）----

    /// 按给定因子缩放矩形（原地修改）。
    ///
    /// `&mut self` 表示可变借用：方法可以修改字段值。
    ///
    /// # 示例
    ///
    /// ```
    /// let mut r = Rectangle::new(2.0, 3.0);
    /// r.scale(2.0);  // 现在 width = 4.0, height = 6.0
    /// ```
    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    /// 将矩形扩大指定像素值（原地修改）。
    pub fn grow(&mut self, dw: f64, dh: f64) {
        self.width += dw;
        self.height += dh;
    }

    // ---- self 方法（所有权转移）----

    /// 消费矩形，返回一个两倍尺寸的新矩形。
    ///
    /// `self`（不加 `&`）表示方法获取所有权：调用后原变量不可再用。
    /// 这类似于 Python 的"消耗型"构造器，Rust 中多用于 builder 模式。
    pub fn double(self) -> Self {
        Self {
            width: self.width * 2.0,
            height: self.height * 2.0,
        }
    }

    /// 消费矩形，返回一个将宽高各增加 `margin` 边距的新矩形。
    pub fn with_margin(self, margin: f64) -> Self {
        Self {
            width: self.width + 2.0 * margin,
            height: self.height + 2.0 * margin,
        }
    }
}

// ---------------------------------------------------------------------------
// 3. 元组结构体 (Tuple Struct)
// ---------------------------------------------------------------------------

/// 三维空间中的点，使用元组结构体表示。
///
/// 元组结构体有名字，但字段没有名字 —— 通过位置索引访问。
#[derive(Debug)]
struct Point(f64, f64, f64);

impl Point {
    /// 关联函数：创建原点。
    pub fn origin() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    /// 关联函数：用给定坐标创建点。
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    /// 计算点到原点 (0, 0, 0) 的欧几里得距离。
    pub fn distance_from_origin(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    /// 计算当前点到另一个点的欧几里得距离。
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }

    /// 返回一个新的点，坐标值是当前点的两倍。
    pub fn doubled(&self) -> Self {
        Self(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
}

// ---------------------------------------------------------------------------
// 4. 单元结构体 (Unit-Like Struct)
// ---------------------------------------------------------------------------

/// 单元结构体：不持有数据，仅作为类型标记。
///
/// 常用于 trait 实现、类型状态模式或特征标记。
/// 它的大小为零，不消耗内存。
#[derive(Debug)]
struct ConfigMarker;

/// 另一个单元结构体，演示不同标记类型。
#[derive(Debug)]
struct DebugModeEnabled;

// ---------------------------------------------------------------------------
// 5. main — 综合演示
// ---------------------------------------------------------------------------

fn main() {
    println!("===== 第八章：结构体与方法 =====\n");

    // ---------- 5a. 字段初始化简写 (Field Init Shorthand) ----------
    println!("--- 字段初始化简写 ---");
    let width = 30.0;
    let height = 20.0;
    // 当变量名与字段名相同时，可以简写为 `width` 而不是 `width: width`
    let rect1 = Rectangle { width, height };
    println!("rect1 = {rect1:?}");

    // ---------- 5b. 方法调用 ----------
    println!("\n--- 方法调用 ---");
    println!("area      = {:.2}", rect1.area());
    println!("perimeter = {:.2}", rect1.perimeter());
    println!("diagonal  = {:.2}", rect1.diagonal());

    // ---------- 5c. 关联函数 ----------
    println!("\n--- 关联函数 ---");
    let square = Rectangle::square(15.0);
    println!("square (15x15) = {square:#?}");
    println!("square area    = {:.2}", square.area());

    // ---------- 5d. 结构体更新语法 (Struct Update Syntax) ----------
    println!("\n--- 结构体更新语法 ---");
    // 基于 rect1 创建新矩形，只改变宽度
    let rect2 = Rectangle {
        width: 50.0,
        ..rect1 // 其余字段从 rect1 复制（height 等）
    };
    println!("rect2 = {rect2:?}");

    // ---------- 5e. can_hold ----------
    println!("\n--- can_hold 检查 ---");
    let small = Rectangle::new(10.0, 10.0);
    let large = Rectangle::new(60.0, 40.0);
    println!("large 能容纳 small?  {}", large.can_hold(&small));
    println!("small 能容纳 large?  {}", small.can_hold(&large));
    println!("rect2 能容纳 rect1? {}", rect2.can_hold(&rect1));
    // 对角线稍大的矩形不能容纳（只要有一个维度不够）
    let tall = Rectangle::new(15.0, 50.0);
    println!("square 能容纳 tall?  {}", square.can_hold(&tall));

    // ---------- 5f. &mut self 方法 ----------
    println!("\n--- &mut self 可变借用 ---");
    let mut flexible = Rectangle::new(5.0, 5.0);
    println!("缩放前: {flexible:?}");
    flexible.scale(3.0);
    println!("scale(3.0) 后: {flexible:?}");
    flexible.grow(1.5, 2.0);
    println!("grow(1.5, 2.0) 后: {flexible:?}");

    // ---------- 5g. self 方法（所有权转移） ----------
    println!("\n--- self 所有权方法 ---");
    let temp = Rectangle::new(7.0, 4.0);
    println!("原始: {temp:?}");
    let bigger = temp.double();
    println!("double() 后: {bigger:?}");
    // temp 在此处已不可用（所有权已转移给 double）
    // println!("{temp:?}");  // 编译错误！

    let boxed = Rectangle::new(8.0, 6.0);
    let framed = boxed.with_margin(2.0);
    println!("8x6 加 margin 2.0 后: {framed:?}");

    // ---------- 5h. 元组结构体 ----------
    println!("\n--- 元组结构体 ---");
    let p1 = Point::new(3.0, 4.0, 0.0);
    let p2 = Point::new(6.0, 8.0, 12.0);
    let origin = Point::origin();

    println!("p1     = {p1:?}");
    println!("p2     = {p2:?}");
    println!("origin = {origin:?}");
    println!("p1 到原点距离 = {:.4}", p1.distance_from_origin());
    println!("p1 到 p2 距离  = {:.4}", p1.distance_to(&p2));
    println!("p1 double 后   = {:?}", p1.doubled());

    // 按索引访问元组结构体字段
    println!("p1.x = {}, p1.y = {}, p1.z = {}", p1.0, p1.1, p1.2);

    // ---------- 5i. 单元结构体 ----------
    println!("\n--- 单元结构体 ---");
    let _marker = ConfigMarker;
    let _debug = DebugModeEnabled;
    println!("ConfigMarker   = {ConfigMarker:?}");
    println!("DebugModeEnabled = {DebugModeEnabled:?}");
    // 单元结构体的大小为 0 字节
    println!(
        "size_of::<ConfigMarker>() = {} byte(s)",
        std::mem::size_of::<ConfigMarker>()
    );

    // ---------- 5j. Debug 格式对比 ----------
    println!("\n--- Debug 格式对比 ---");
    let showcase = Rectangle {
        width: 12.34,
        height: 56.78,
    };
    println!("{{:?}}   = {showcase:?}");
    println!("{{:#?}} = {showcase:#?}");

    // ---------- 5k. 综合：结构体数组 ----------
    println!("\n--- 综合：矩形数组 ---");
    let rectangles = [
        Rectangle::new(10.0, 5.0),
        Rectangle::square(7.0),
        Rectangle::new(3.0, 9.0),
        Rectangle::new(15.0, 12.0),
    ];
    for (i, r) in rectangles.iter().enumerate() {
        println!(
            "  [{i}] {r:?}  area={:.1}  perimeter={:.1}  diag={:.2}",
            r.area(),
            r.perimeter(),
            r.diagonal()
        );
    }

    println!("\n===== 第八章演示结束 =====");
}
