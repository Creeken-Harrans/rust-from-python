#![allow(rustdoc::invalid_html_tags)]
// ============================================================
// Chapter 19: 智能指针 — Box<T>、Rc<T>、RefCell<T>
// ============================================================
// Rust 的智能指针不仅仅是指针：它们拥有额外的元数据和能力。
// 本章演示三种核心智能指针及其组合用法。

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// ============================================================
// 第 1 部分: Box<T> — 堆分配
// ============================================================

/// 一个较大的结构体 — 约 8KB，如果分配在栈上可能超出栈空间
#[allow(dead_code)]
struct LargeData {
    buffer: [u8; 8192], // 8 KiB
    id: u64,
    label: String,
}

/// 递归类型: ConsList — 必须使用 Box 因为编译期大小不可确定
#[derive(Debug)]
#[allow(dead_code)]
enum ConsList {
    /// 空列表
    Nil,
    /// 包含一个 i32 和指向下一个节点的 Box
    Cons(i32, Box<ConsList>),
}

/// 一个简单的 trait 用于演示 Box<dyn Trait>
trait Animal {
    fn speak(&self) -> &'static str;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> &'static str {
        "汪汪!"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> &'static str {
        "喵~"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

fn demo_box() {
    println!("========== Box<T> 演示 ==========");

    // --- Box 基础: 将数据放在堆上 ---
    let boxed_int = Box::new(42i32);
    println!("Box<i32> 指向的值为: {}", *boxed_int);
    println!("Box<i32> 的大小(指针):   {} 字节", size_of_val(&boxed_int));
    println!("i32 本身的大小:        {} 字节", size_of::<i32>());
    // Box 本身只是一个指针大小(8 字节在 64 位平台上)，但指向的数据在堆上

    // --- 大结构体: 避免栈溢出 ---
    // 如果写成 let data = LargeData { ... }; 会尝试在栈上分配 8KB+
    // 使用 Box 将 LargeData 放在堆上，栈上只保留一个 8 字节指针
    let large_on_heap = Box::new(LargeData {
        buffer: [0u8; 8192],
        id: 1,
        label: String::from("大块数据"),
    });
    println!(
        "LargeData 的大小: {} 字节 → Box<LargeData> 只占 {} 字节(指针)",
        size_of::<LargeData>(),
        size_of_val(&large_on_heap)
    );
    println!(
        "大结构体通过 Box 放在堆上: id={}, label={}",
        large_on_heap.id, large_on_heap.label
    );

    // --- 递归类型: ConsList ---
    // 不使用 Box 的话，编译器无法计算 ConsList 的大小(无限递归)
    let list = ConsList::Cons(
        1,
        Box::new(ConsList::Cons(
            2,
            Box::new(ConsList::Cons(3, Box::new(ConsList::Nil))),
        )),
    );
    println!("递归 ConsList: {:?}", list);

    // 辅助函数: 计算列表长度
    fn list_len(list: &ConsList) -> usize {
        match list {
            ConsList::Nil => 0,
            ConsList::Cons(_, next) => 1 + list_len(next),
        }
    }
    println!("列表长度: {}", list_len(&list));

    // --- Box<dyn Trait>: trait 对象 ---
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("旺财"),
        }),
        Box::new(Cat {
            name: String::from("咪咪"),
        }),
    ];
    for animal in &animals {
        println!("{}: {}", animal.name(), animal.speak());
    }
}

// ============================================================
// 第 2 部分: Rc<T> — 引用计数，单线程共享所有权
// ============================================================

#[derive(Debug)]
#[allow(dead_code)]
struct SharedConfig {
    api_url: String,
    timeout_secs: u32,
    retry_count: u32,
}

fn demo_rc() {
    println!("\n========== Rc<T> 演示 ==========");

    // --- 创建共享配置 ---
    let config = Rc::new(SharedConfig {
        api_url: String::from("https://api.example.com/v2"),
        timeout_secs: 30,
        retry_count: 3,
    });
    println!("初始引用计数: {}", Rc::strong_count(&config));

    // --- 多个"所有者"通过 Rc::clone 共享 ---
    // 注意: Rc::clone 不深拷贝数据，只增加引用计数
    let config2 = Rc::clone(&config);
    let config3 = Rc::clone(&config);
    println!("clone 2 次后的引用计数: {}", Rc::strong_count(&config)); // 应为 3

    println!("config  API URL: {}", config.api_url);
    println!("config2 API URL: {}", config2.api_url);
    println!("config3 API URL: {}", config3.api_url);

    // --- Rc 不允许可变借用 ---
    // 以下代码如果取消注释将无法编译:
    // config.timeout_secs = 60;  // 错误: Rc 不实现 DerefMut
    // let m = Rc::get_mut(&mut config); // 只有在引用计数为 1 时才能用

    // --- 作用域与引用计数 ---
    {
        let _config4 = Rc::clone(&config);
        println!("在内部作用域中, 引用计数: {}", Rc::strong_count(&config)); // 4
        println!("释放前——config4 仍在使用中");
    } // config4 超出作用域，计数 -1
    println!("离开内部作用域后, 引用计数: {}", Rc::strong_count(&config)); // 3

    // --- Rc 摘要 ---
    println!("Rc<T> 关键点:");
    println!("  - 仅适用于单线程场景 (非 Send/Sync)");
    println!("  - Rc::clone 只增加引用计数, 不深拷贝数据");
    println!("  - 数据在最后一个 Rc 被丢弃时自动释放");
    println!("  - 通过共享引用 (&T) 访问, 不支持可变访问");
}

// ============================================================
// 第 3 部分: RefCell<T> — 内部可变性
// ============================================================

fn demo_refcell() {
    println!("\n========== RefCell<T> 演示 ==========");

    // --- 基础: RefCell 允许通过不可变引用进行修改 ---
    let data = RefCell::new(42i32);

    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("借用值: {}", *borrowed);
    } // borrowed 在此处被丢弃

    // 可变借用 — 注意 data 本身不是 mut!
    {
        let mut borrowed_mut = data.borrow_mut();
        *borrowed_mut += 58;
        println!("修改后的值: {}", *borrowed_mut);
    }

    // 验证修改已持久化
    println!("持久化后的值: {}", *data.borrow());

    // --- 运行时的借用检查 ---
    // RefCell 在运行时执行借用规则:
    //   - 任意数量不可变借用 OR 恰好一个可变借用
    //   - 违反规则 → 运行时 panic!

    println!("\n运行时借用检查对比:");
    println!("  编译期 (标准引用): 借用检查器在编译时验证规则");
    println!("  运行时 (RefCell):  规则在程序运行时执行");

    // 以下代码会在运行时 panic (多个可变借用):
    // let r1 = data.borrow_mut();
    // let r2 = data.borrow_mut(); // PANIC: 已存在可变借用
    // println!("{} {}", r1, r2);

    // 以下代码也会 panic (混合可变/不可变借用):
    // let r1 = data.borrow_mut();
    // let r2 = data.borrow();     // PANIC: 已存在可变借用
    // println!("{}", *r1);

    // --- 为什么需要 RefCell? ---
    // 当你需要修改数据，但数据结构对外暴露的接口是 &self (不可变借用) 时
    // RefCell 让你在"逻辑上不可变"的上下文中修改"内部可变"的数据

    println!("\nRefCell<T> 核心概念:");
    println!("  - 内部可变性: 通过 &self 修改内部值");
    println!("  - borrow()  → 不可变借用 (可多个)");
    println!("  - borrow_mut() → 可变借用 (仅一个)");
    println!("  - 违反规则 → 运行时 panic (非编译错误)");
    println!("  - RefCell 不是'绕过 Rust 规则', 而是将检查移到运行时");
}

// ============================================================
// 第 4 部分: Rc<RefCell<T>> — 共享所有权 + 内部可变性
// ============================================================

/// 图节点: 值 + 边的列表(可在共享引用下修改)
#[derive(Debug)]
struct Node {
    value: String,
    /// edges 存储指向相邻节点的引用计数指针
    edges: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: &str) -> Rc<Node> {
        Rc::new(Node {
            value: String::from(value),
            edges: RefCell::new(Vec::new()),
        })
    }

    /// 从 self 添加一条边到 target (通过共享引用!)
    fn add_edge(&self, target: Rc<Node>) {
        self.edges.borrow_mut().push(target);
    }
}

fn demo_rc_refcell() {
    println!("\n========== Rc<RefCell<T>> 演示 ==========");

    // 创建几个图节点
    let alice = Node::new("Alice");
    let bob = Node::new("Bob");
    let charlie = Node::new("Charlie");
    let diana = Node::new("Diana");

    // 通过共享引用添加边 — 节点本身不需要 mut
    alice.add_edge(Rc::clone(&bob));
    alice.add_edge(Rc::clone(&charlie));
    bob.add_edge(Rc::clone(&charlie));
    bob.add_edge(Rc::clone(&diana));
    charlie.add_edge(Rc::clone(&alice)); // 创建了一个环!
    diana.add_edge(Rc::clone(&bob));

    // 打印图结构
    println!("图结构:");
    print_node(&alice, &[]);
    println!("\n(由于存在环, 我们只打印一层深度)");

    // --- 通过共享引用修改节点值 ---
    // 如果可以添加 RefCell<String> 会怎样?
    // 我们在此演示通过 Rc<RefCell<Node>> 的模式:
    let shared_node = Rc::new(RefCell::new(String::from("共享可变数据")));
    println!("\n共享可变字符串:");
    println!("  初始: {}", shared_node.borrow());

    // 通过 Rc 的共享引用修改内部 RefCell
    let shared_node2 = Rc::clone(&shared_node);
    shared_node2
        .borrow_mut()
        .push_str(" → 由 shared_node2 修改");
    println!("  修改后: {}", shared_node.borrow());

    let shared_node3 = Rc::clone(&shared_node);
    shared_node3
        .borrow_mut()
        .push_str(" → 由 shared_node3 追加");
    println!("  再次修改后: {}", shared_node.borrow());
    println!("  最终引用计数: {}", Rc::strong_count(&shared_node));
}

fn print_node(node: &Rc<Node>, _visited: &[&str]) {
    let edges = node.edges.borrow();
    print!("  {} → [", node.value);
    let edge_values: Vec<String> = edges.iter().map(|n| n.value.clone()).collect();
    println!("{}]", edge_values.join(", "));
}

// ============================================================
// 第 5 部分: 引用循环警告 & Weak<T>
// ============================================================

/// 演示循环引用的父子结构
#[derive(Debug)]
struct Parent {
    name: String,
    /// 使用 RefCell<Vec<Rc<Child>>> 让父节点持有子节点
    children: RefCell<Vec<Rc<Child>>>,
}

#[derive(Debug)]
struct Child {
    name: String,
    /// 如果这里也用 Rc<Parent>, 会形成 Rc 循环 → 内存泄漏!
    /// 注意: 在实际项目中, 应该使用 Weak<Parent> 来打破循环
    /// 但为了演示问题, 我们在这里展示了错误的设计(已注释):
    // parent: RefCell<Rc<Parent>>, // ⚠️ 这会造成循环引用!
    /// 正确的做法:
    parent: RefCell<Weak<Parent>>, // ✅ 使用 Weak 打破循环
}

fn demo_reference_cycles() {
    println!("\n========== 引用循环与 Weak<T> ==========");

    // --- 循环引用问题 ---
    println!("【引用循环问题】");
    println!("当 A 持有 B 的 Rc, B 又持有 A 的 Rc 时:");
    println!("  A.ref_count >= 1 (来自 B)");
    println!("  B.ref_count >= 1 (来自 A)");
    println!("  → 两者永远不会被释放 → 内存泄漏!");

    // --- Weak<T> 方案 ---
    println!("\n【Weak<T> 解决方案】");
    println!("Weak<T> 是一个'非拥有'指针:");
    println!("  - Weak::new()  创建空弱引用");
    println!("  - Rc::downgrade(&rc)  从 Rc 创建 Weak");
    println!("  - weak.upgrade()  尝试获取 Option<Rc<T>>");
    println!("  - 当所有 Rc 被丢弃后, upgrade() 返回 None");
    println!("  - Weak 不增加 strong_count");

    // --- 演示 Weak 升级 ---
    let rc_data = Rc::new(42i32);
    let weak_data = Rc::downgrade(&rc_data);

    println!("强引用计数: {}", Rc::strong_count(&rc_data)); // 1
    println!("弱引用计数: {}", Rc::weak_count(&rc_data)); // 1

    // 从弱引用升级
    match weak_data.upgrade() {
        Some(shared) => println!("成功升级弱引用, 值: {}", *shared),
        None => println!("弱引用已失效"),
    }

    // 丢弃原始 Rc, 弱引用将不再能升级
    drop(rc_data);
    match weak_data.upgrade() {
        Some(_) => println!("仍然有效"),
        None => println!("原始 Rc 已丢弃, 弱引用无法升级"), // 预期输出
    }

    // --- 正确使用 Weak 的亲子关系 ---
    println!("\n【正确模式: 父母 ↔ 子女 (使用 Weak)】");

    let parent = Rc::new(Parent {
        name: String::from("父亲"),
        children: RefCell::new(Vec::new()),
    });

    let child = Rc::new(Child {
        name: String::from("孩子"),
        parent: RefCell::new(Weak::new()),
    });

    // 建立双向关系
    parent.children.borrow_mut().push(Rc::clone(&child));
    *child.parent.borrow_mut() = Rc::downgrade(&parent);

    println!("父节点: {}", parent.name);
    println!(
        "  强引用计数: {} (非 0 → 父节点存活)",
        Rc::strong_count(&parent)
    );
    println!(
        "  弱引用计数: {} (来自子节点对父节点的 Weak)",
        Rc::weak_count(&parent)
    );

    // 子节点可以访问父节点
    if let Some(p) = child.parent.borrow().upgrade() {
        println!("  子节点可以访问: 我的{}叫{}", &child.name, p.name);
    }

    println!("\n总结: 当强引用计数归零时, 数据被释放;");
    println!("      即使弱引用计数仍然大于 0, 也不会阻止释放。");
}

// ============================================================
// 第 6 部分: 类型选择总结
// ============================================================

fn demo_type_selection() {
    println!("\n========== 类型选择指南 ==========");

    println!("┌─────────────────┬──────────────────────────────────┐");
    println!("│ 需求             │ 推荐类型                          │");
    println!("├─────────────────┼──────────────────────────────────┤");
    println!("│ 堆分配大值       │ Box<T>                           │");
    println!("│ 递归类型         │ Box<T>                           │");
    println!("│ trait 对象       │ Box<dyn Trait>                   │");
    println!("│ 单线程共享不可变 │ Rc<T>                            │");
    println!("│ 单线程共享可变   │ Rc<RefCell<T>>                   │");
    println!("│ 多线程共享不可变 │ Arc<T>                           │");
    println!("│ 多线程共享可变   │ Arc<Mutex<T>> 或 Arc<RwLock<T>>  │");
    println!("│ 只引用不拥有     │ &T, &mut T, Weak<T>              │");
    println!("│ 内部可变性       │ RefCell<T>, Cell<T>              │");
    println!("└─────────────────┴──────────────────────────────────┘");

    println!("\n重要提醒:");
    println!("  RefCell 不是'绕过 Rust 的规则'——");
    println!("  它将借用检查从编译时移到运行时。");
    println!("  如果违反借用规则, 程序仍然会失败 (panic),");
    println!("  只是错误发生在运行时而非编译时。");
}

// ============================================================
// main
// ============================================================

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║  Chapter 19: 智能指针                    ║");
    println!("║  Box<T>, Rc<T>, RefCell<T>              ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    demo_box();
    demo_rc();
    demo_refcell();
    demo_rc_refcell();
    demo_reference_cycles();
    demo_type_selection();

    println!("\n✓ 所有演示完成。");
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basics() {
        let b = Box::new(100);
        assert_eq!(*b, 100);
    }

    #[test]
    fn test_rc_sharing() {
        let r1 = Rc::new(String::from("shared"));
        let r2 = Rc::clone(&r1);
        assert_eq!(Rc::strong_count(&r1), 2);
        assert_eq!(*r1, "shared");
        assert_eq!(*r2, "shared");
    }

    #[test]
    fn test_refcell_mutation() {
        let cell = RefCell::new(0);
        *cell.borrow_mut() = 42;
        assert_eq!(*cell.borrow(), 42);
    }

    #[test]
    fn test_rc_refcell() {
        let shared = Rc::new(RefCell::new(String::from("hello")));
        let shared2 = Rc::clone(&shared);
        shared2.borrow_mut().push_str(" world");
        assert_eq!(*shared.borrow(), "hello world");
    }

    #[test]
    fn test_cons_list() {
        let list = ConsList::Cons(10, Box::new(ConsList::Cons(20, Box::new(ConsList::Nil))));
        match &list {
            ConsList::Cons(val, _) => assert_eq!(*val, 10),
            _ => panic!("预期 Cons"),
        }
    }

    #[test]
    fn test_weak_upgrade() {
        let rc = Rc::new(99);
        let weak = Rc::downgrade(&rc);
        assert!(weak.upgrade().is_some());
        drop(rc);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_node_graph() {
        let n1 = Node::new("A");
        let n2 = Node::new("B");
        n1.add_edge(Rc::clone(&n2));
        assert_eq!(n1.edges.borrow().len(), 1);
    }
}
