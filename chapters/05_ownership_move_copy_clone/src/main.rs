// 所有权规则:
// 1. 每个值有且仅有一个所有者 (owner)
// 2. 所有者离开作用域时值被释放 (drop)
// 3. 同一时刻只能有一个所有者
//
// Copy 类型 (赋值时自动复制, 原变量仍可用):
//   - 所有整数类型: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
//   - 浮点类型: f32, f64
//   - 布尔类型: bool
//   - 字符类型: char
//   - 元素全为 Copy 的元组: (i32, bool), (char, f64) 等
//   - 元素全为 Copy 的数组: [i32; 5], [bool; 3] 等
//   - 不可变引用: &T (但引用本身是 Copy, 不是所指向的数据)
//
// 非 Copy 类型 (赋值时移动所有权, 原变量失效):
//   - String
//   - Vec<T>
//   - HashMap<K, V>
//   - Box<T>
//   - 任何实现了 Drop trait 的类型
//   - 包含非 Copy 字段的结构体

// ---------------------------------------------------------------------------
// 演示: 移动语义 (Move)
// ---------------------------------------------------------------------------
fn demonstrate_move() {
    println!("--- demonstrate_move ---");
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2, s1 不再有效

    // 取消下面注释会导致编译错误: value borrowed after move
    // println!("s1 = {}", s1); // error[E0382]: borrow of moved value: `s1`

    println!("s2 = {}", s2); // s2 是唯一的所有者, 正常工作
    println!();
}

// ---------------------------------------------------------------------------
// 演示: Copy 语义 —— 整数 / bool / char 自动复制
// ---------------------------------------------------------------------------
fn demonstrate_copy() {
    println!("--- demonstrate_copy ---");
    let x = 5;
    let y = x; // i32 是 Copy 类型, x 的值被复制给 y, x 仍然有效

    println!("x = {}, y = {}", x, y); // 两者都可用

    let flag_a = true;
    let flag_b = flag_a; // bool 是 Copy
    println!("flag_a = {}, flag_b = {}", flag_a, flag_b);

    let c1 = '睦';
    let c2 = c1; // char 是 Copy
    println!("c1 = {}, c2 = {}", c1, c2);

    // 元素全为 Copy 的元组也实现 Copy
    let t1: (i32, bool, char) = (42, false, 'R');
    let t2 = t1; // t1 被复制, 仍然可用
    println!("t1 = {:?}, t2 = {:?}", t1, t2);

    // 元素全为 Copy 的数组也是 Copy
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;
    println!("arr1 = {:?}, arr2 = {:?}", arr1, arr2);
    println!();
}

// ---------------------------------------------------------------------------
// 演示: Clone —— 显式深拷贝
// ---------------------------------------------------------------------------
fn demonstrate_clone() {
    println!("--- demonstrate_clone ---");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 显式深拷贝, 在堆上分配新内存

    println!("s1 = {}, s2 = {}", s1, s2); // 两者都有效, 各自拥有独立内存
    // 注意: clone() 有运行时开销, 不同于 Copy 的编译期位复制

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone(); // Vec 也支持 Clone

    println!("v1 = {:?}, v2 = {:?}", v1, v2);
    println!("v1.len() = {}, v2.len() = {}", v1.len(), v2.len());
    println!();
}

// ---------------------------------------------------------------------------
// 演示: 函数获取并返回所有权
// ---------------------------------------------------------------------------
fn take_ownership(s: String) -> String {
    // s 进入作用域, 拥有所有权
    println!("    take_ownership 收到了: {}", s);
    let uppercase = s.to_uppercase();
    // s 在函数结束时被 drop, 但我们返回了 uppercase
    uppercase // 所有权转移给调用者
}

fn borrow_then_return() {
    println!("--- borrow_then_return ---");
    let original = String::from("rustacean");

    // 所有权进入函数...
    let processed = take_ownership(original);
    // original 已经失效, 不能再使用
    // println!("{}", original); // 编译错误!

    // 所有权从函数返回, processed 接手
    println!("    取回的所有权: {}", processed);
    println!();
}

// ---------------------------------------------------------------------------
// 结构体与移动语义
// ---------------------------------------------------------------------------
#[derive(Debug)]
struct Person {
    name: String, // 非 Copy
    age: i32,     // Copy
}

fn demonstrate_struct_move() {
    println!("--- demonstrate_struct_move ---");
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let bob = alice; // 结构体整体移动！
    // alice 不再有效, 因为 Person 包含非 Copy 字段 name (String)
    // 取消下面注释会导致编译错误: value borrowed after move
    // println!("alice = {:?}", alice); // error[E0382]

    // bob 完全拥有 name 和 age, 可以分别访问
    println!("bob.name = {}, bob.age = {}", bob.name, bob.age);
    println!("bob = {:?}", bob);

    // 但 age 字段单独拿出来是 Copy —— 只是整个结构体已经移动了
    // 如果重构成只有 Copy 字段, 自动推导 Copy trait 即可整体复制
    println!();
}

// ---------------------------------------------------------------------------
// 元组混合类型演示
// ---------------------------------------------------------------------------
fn demonstrate_tuple_mixed() {
    println!("--- demonstrate_tuple_mixed ---");
    // (String, i32): String 非 Copy, i32 是 Copy
    // 整个元组不实现 Copy
    let t1 = (String::from("hello"), 42);
    let t2 = t1; // t1 的所有权移动到 t2
    // println!("t1 = {:?}", t1); // 编译错误: t1 已移动

    println!("t2 = {:?}", t2);
    println!();
}

// ---------------------------------------------------------------------------
// 最典型的 "为什么要有所有权" 场景
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 带 Drop 的结构体 —— 观察值离开作用域
// ---------------------------------------------------------------------------
struct Resource {
    id: u32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("    >>> Resource #{} 被释放 (drop) <<<", self.id);
    }
}

fn drop_demo_inner() {
    println!("    drop_demo_inner: 进入作用域");
    let _r1 = Resource { id: 1 };
    let _r2 = Resource { id: 2 };
    println!("    drop_demo_inner: 即将离开作用域...");
    // _r1 和 _r2 在离开时按声明逆序 drop: 先 r2, 后 r1
}

fn demonstrate_drop() {
    println!("--- demonstrate_drop ---");
    drop_demo_inner();
    println!("    (已回到 demonstrate_drop)");
    println!();
}

// ---------------------------------------------------------------------------
// 为什么不能什么都 clone?
// ---------------------------------------------------------------------------
fn why_not_always_clone() {
    println!("--- why_not_always_clone ---");
    println!(
        "滥用 clone() 的问题:\n\
         - 性能: 每次 clone 都是一次堆分配 + 数据复制, 可能很昂贵\n\
         - 隐藏设计问题: 如果某处\"必须\" clone, 往往说明所有权设计有问题\n\
         - 信号: 编译器报 move error 是帮你发现\"资源被多处持有\"的设计缺陷\n\
         \n\
        適当使用 clone() 的场景:\n\
         - 原型阶段快速验证逻辑 (之后再优化)\n\
         - 确实需要两份独立数据 (如两个线程各自拥有副本)\n\
         - 数据很小且 clone 开销可忽略\n\
         - 作为临时措施绕过借用检查, 但需要在注释中说明原因\n\
         \n\
        核心思路: Rust 的所有权系统不是障碍, 是帮你提前发现 bug 的工具。\n\
        遇到 move error 时, 优先考虑\"谁应该拥有这个值?\",\n\
        而不是盲目加 .clone()。"
    );
    println!();
}

// ===========================================================================
// 关于 Copy 和 Clone 的区别
// ===========================================================================
fn copy_vs_clone_summary() {
    println!("--- Copy vs Clone 小结 ---");
    println!(
        "Copy  trait: 编译器自动在位复制 (memcpy 语义), 隐式, 无开销。\n\
              只适用于\"栈上数据\"类型——整数、bool、char、Copy元组等。\n\
         Clone trait: 显式调用 .clone(), 可能涉及堆分配, 有运行时成本。\n\
              适用于需要深度复制的类型: String, Vec, HashMap 等。\n\
         \n\
         关键差异: Copy 赋值后原变量仍可用; Move 赋值后原变量失效。"
    );
    println!();
}

// ===========================================================================
// main
// ===========================================================================
fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║     所有权、移动与复制 —— Rust 最核心的概念            ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    // ── 第 1 部分: Move ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 1 部分: Move (移动)                    │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_move();

    // ── 第 2 部分: Copy ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 2 部分: Copy (复制语义)                │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_copy();

    // ── 第 3 部分: Clone ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 3 部分: Clone (克隆)                   │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_clone();

    // ── 第 4 部分: 函数所有权 ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 4 部分: 函数参数与返回值的所有权        │");
    println!("└──────────────────────────────────────────┘");
    borrow_then_return();

    // ── 第 5 部分: 结构体移动 ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 5 部分: 结构体与移动语义                │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_struct_move();

    // ── 第 6 部分: 混合元组 ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 6 部分: 混合类型元组                    │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_tuple_mixed();

    // ── 第 7 部分: Drop ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 7 部分: Drop (析构)                    │");
    println!("└──────────────────────────────────────────┘");
    demonstrate_drop();

    // ── 第 8 部分: 不要滥用 clone ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 8 部分: 为什么不能什么都 clone?         │");
    println!("└──────────────────────────────────────────┘");
    why_not_always_clone();

    // ── 第 9 部分: Copy vs Clone ──
    println!("┌──────────────────────────────────────────┐");
    println!("│  第 9 部分: Copy vs Clone 对比             │");
    println!("└──────────────────────────────────────────┘");
    copy_vs_clone_summary();

    println!("════════════════════════════════════════════");
    println!(" 所有演示完成。运行 `cargo run` 查看完整输出。");
    println!("════════════════════════════════════════════");
}
