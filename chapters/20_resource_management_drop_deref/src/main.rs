// ============================================================
// 第20章: 资源管理 — Drop、Deref 与 RAII 实践
// ============================================================

use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};

// -----------------------------------------------------------
// 全局计数器，用于追踪 FileGuard 的创建顺序
// -----------------------------------------------------------
static FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

// ============================================================
// 1. FileGuard — 模拟文件句柄，实现 Drop 自动清理
// ============================================================
#[derive(Debug)]
struct FileGuard {
    name: String,
    handle_id: u64,
    creation_order: u64,
}

impl FileGuard {
    fn open(name: &str) -> Self {
        let id = FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let guard = FileGuard {
            name: name.to_string(),
            handle_id: id * 1000 + 42,
            creation_order: id,
        };
        println!(
            "[CREATE] FileGuard #{order}: \"{name}\" (handle: {hid})",
            order = guard.creation_order,
            name = guard.name,
            hid = guard.handle_id
        );
        guard
    }

    fn read_line(&self) -> String {
        format!("[模拟读取] 文件 {} 中的一行数据...", self.name)
    }

    fn write(&self, data: &str) {
        println!("[模拟写入] 向文件 {} 写入: {}", self.name, data);
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!(
            "[DROP]   FileGuard #{order}: \"{name}\" (handle: {hid})",
            order = self.creation_order,
            name = self.name,
            hid = self.handle_id
        );
    }
}

// ============================================================
// 2. ConnectionPool — 模拟数据库连接池，实现 Drop 断开连接
// ============================================================
struct ConnectionPool {
    url: String,
    active: bool,
}

impl ConnectionPool {
    fn connect(url: &str) -> Self {
        println!("[CREATE] ConnectionPool → {url}");
        ConnectionPool {
            url: url.to_string(),
            active: true,
        }
    }

    fn query(&self, sql: &str) -> Result<String, String> {
        if !self.active {
            return Err("连接池已关闭，无法执行查询".to_string());
        }
        println!("[QUERY ] 执行 SQL: {sql}  @ {}", self.url);
        // 模拟查询结果
        Ok(format!(
            "查询结果: [{{id: 1, name: \"Alice\"}}, {{id: 2, name: \"Bob\"}}] (来源: {})",
            self.url
        ))
    }

    #[allow(dead_code)]
    fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        println!(
            "[DROP]   ConnectionPool: \"{}\" (active={})",
            self.url, self.active
        );
        self.active = false;
    }
}

// ============================================================
// 3. MyBox<T> — 简单包装类型，演示 Deref trait
// ============================================================
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// ============================================================
// 4. MeteredResource — 带计时/度量的资源，演示 RAII 指标收集
// ============================================================
struct MeteredResource {
    name: String,
    created_at: std::time::Instant,
}

impl MeteredResource {
    fn new(name: &str) -> Self {
        let now = std::time::Instant::now();
        println!(
            "[CREATE] MeteredResource \"{name}\" @ {t:?}",
            name = name,
            t = now
        );
        MeteredResource {
            name: name.to_string(),
            created_at: now,
        }
    }

    fn do_work(&self) {
        println!("[WORK  ] MeteredResource \"{}\" 执行任务...", self.name);
    }
}

impl Drop for MeteredResource {
    fn drop(&mut self) {
        let elapsed = self.created_at.elapsed();
        println!(
            "[DROP]   MeteredResource \"{name}\": 存活时间 {elapsed:?}",
            name = self.name,
            elapsed = elapsed
        );
    }
}

// ============================================================
// 5. process_files_raii() — 演示即使提前返回/出错，Drop 也会执行
// ============================================================
fn process_files_raii() -> Result<(), String> {
    println!("\n┌─ process_files_raii() 开始 ──────────────────────");

    let log_file = FileGuard::open("/var/log/app.log");
    let data_file = FileGuard::open("/data/config.json");

    // 读取一些数据
    let _line = log_file.read_line();

    // 模拟可能的错误路径 —— 即使这里出错，上面两个 FileGuard 也会被 drop
    let should_fail = false; // 改为 true 可测试错误路径
    if should_fail {
        println!("│ [ERROR] 处理过程中遇到错误，提前返回!");
        return Err("模拟的致命错误: 数据损坏".to_string());
    }

    data_file.write("{\"status\": \"ok\"}");
    println!("│ 所有文件处理完毕 (正常路径)");

    // 无论走哪条路径，FileGuard 的 Drop 都会在这里（或提前返回前）执行
    println!("└─ process_files_raii() 结束 ──────────────────────");
    Ok(())
}

// ============================================================
// 6. demonstrate_deref_coercion — 演示解引用强制转换链
// ============================================================
fn demonstrate_deref_coercion() {
    println!("\n┌─ demonstrate_deref_coercion() ───────────────────");

    // MyBox<String>
    let boxed_string: MyBox<String> = MyBox::new(String::from("你好，Rust!"));

    // 解引用强制转换: &MyBox<String> → &String → &str
    // 可以自动传递给接受 &str 的函数
    greet(&boxed_string);

    // 显式观察每一步
    let string_ref: &String = &boxed_string; // 手动 deref: &MyBox<String> → &String
    let str_ref: &str = string_ref.as_str(); // &String → &str
    println!("│ 显式转换: string_ref = \"{string_ref}\", str_ref = \"{str_ref}\"");

    // 直接调用 .len() 等方法 —— Deref 使 MyBox<String> 可以调用 String 的方法
    println!("│ MyBox 中的字符串长度: {}", boxed_string.len());

    // 嵌套 MyBox
    let double_boxed: MyBox<MyBox<String>> = MyBox::new(MyBox::new(String::from("嵌套解引用")));
    // 多重解引用强制转换: &MyBox<MyBox<String>> → &MyBox<String> → &String → &str
    greet(&double_boxed);

    println!("└─ demonstrate_deref_coercion() 结束 ─────────────");
}

fn greet(name: &str) {
    println!("│ [greet] 你好，{name}!");
}

// ============================================================
// 7. 演示 Drop 顺序 (LIFO)
// ============================================================
fn demonstrate_drop_order() {
    println!("\n┌─ demonstrate_drop_order() 开始 ─────────────────");

    println!("│ --- 作用域开始 ---");
    {
        let f1 = FileGuard::open("/tmp/file1.txt");
        let f2 = FileGuard::open("/tmp/file2.txt");
        let f3 = FileGuard::open("/tmp/file3.txt");
        f1.read_line();
        f2.read_line();
        f3.read_line();
        println!("│ [INFO] 即将离开作用域，f3/f2/f1 将按逆序 drop");
        // 离开作用域: drop(f3) → drop(f2) → drop(f1)   (LIFO)
    }
    println!("│ --- 作用域结束 (所有 FileGuard 已 drop) ---");

    // 显示 std::mem::drop() 显式提前 drop
    println!("│ --- 显式 drop() 演示 ---");
    let early = FileGuard::open("/tmp/early_drop.txt");
    early.read_line();
    println!("│ [INFO] 调用 std::mem::drop(early) 显式提前丢弃...");
    std::mem::drop(early);
    // 此时 early 已被 move/consume，不能再使用
    // 取消注释下面这行会导致编译错误:
    // early.read_line(); // error[E0382]: use of moved value: `early`

    println!("│ [INFO] drop() 之后, early 已不可用 ─ 编译时保证安全!");

    println!("└─ demonstrate_drop_order() 结束 ─────────────────");
}

// ============================================================
// 8. 演示连接池的 RAII 模式
// ============================================================
fn demonstrate_connection_pool() -> Result<(), String> {
    println!("\n┌─ demonstrate_connection_pool() ──────────────────");

    let pool = ConnectionPool::connect("postgres://localhost:5432/mydb");

    // 执行查询
    let result = pool.query("SELECT * FROM users WHERE active = true")?;
    println!("│ 查询结果: {result}");

    // pool 在此函数结束时自动 drop，连接被关闭
    // 即使中间有 ? 提前返回，pool 也会被正确 drop
    println!("└─ demonstrate_connection_pool() 结束 ─────────────");
    Ok(())
}

// ============================================================
// 9. MeteredResource — 演示 RAII 自动测量资源生命周期
// ============================================================
fn demonstrate_metered_resource() {
    println!("\n┌─ demonstrate_metered_resource() ─────────────────");

    let res1 = MeteredResource::new("批处理任务-1");
    res1.do_work();

    {
        let res2 = MeteredResource::new("临时计算-2");
        res2.do_work();
        println!("│ res2 即将离开内层作用域...");
        // res2 在此处 drop，显示存活时间
    }

    println!("│ res2 已在内层作用域结束时 drop");

    // res1 在函数结束时 drop，存活时间更长
    println!("└─ demonstrate_metered_resource() 结束 ────────────");
}

// ============================================================
// 10. 演示 Drop 与 panic 的交互
// ============================================================
fn demonstrate_drop_on_panic() {
    println!("\n┌─ demonstrate_drop_on_panic() ────────────────────");

    struct PanicGuard {
        name: &'static str,
    }
    impl Drop for PanicGuard {
        fn drop(&mut self) {
            println!(
                "│ [DROP] PanicGuard \"{}\": 即使在 panic 展开过程中也会执行!",
                self.name
            );
        }
    }

    let _guard1 = PanicGuard { name: "guard-1" };
    let _guard2 = PanicGuard { name: "guard-2" };

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _inner = PanicGuard {
            name: "inner-guard",
        };
        println!("│ [INFO] 即将触发 panic...");
        panic!("故意的 panic，用于测试 Drop 行为");
    }));

    match result {
        Ok(_) => println!("│ 没有 panic"),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "未知 panic 载荷".to_string()
            };
            println!("│ 捕获到 panic: {msg}");
        }
    }

    println!("│ panic 已捕获，guard1/guard2 仍存活");
    println!("└─ demonstrate_drop_on_panic() 结束 ────────────────");
    // guard2 先 drop，guard1 后 drop (LIFO)
}

// ============================================================
// main() — 编排所有演示
// ============================================================
fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  第20章: 资源管理 — Drop、Deref 与 RAII       ║");
    println!("╚══════════════════════════════════════════════════╝");

    // --- 演示1: Drop 顺序 (LIFO) ---
    demonstrate_drop_order();

    // --- 演示2: RAII 文件处理 (包括错误路径) ---
    println!("\n━━━ RAII 文件处理 (process_files_raii) ━━━");
    match process_files_raii() {
        Ok(()) => println!("process_files_raii: 成功"),
        Err(e) => println!("process_files_raii: 失败 → {e}"),
    }

    // --- 演示3: 连接池 RAII ---
    println!("\n━━━ 连接池 RAII ━━━");
    match demonstrate_connection_pool() {
        Ok(()) => println!("demonstrate_connection_pool: 成功"),
        Err(e) => println!("demonstrate_connection_pool: 失败 → {e}"),
    }

    // --- 演示4: Deref 强制转换 ---
    println!("\n━━━ Deref 强制转换 ━━━");
    demonstrate_deref_coercion();

    // --- 演示5: MeteredResource 计时 ---
    println!("\n━━━ MeteredResource (计时度量) ━━━");
    demonstrate_metered_resource();

    // --- 演示6: Drop 与 panic ---
    println!("\n━━━ Drop 与 Panic ━━━");
    demonstrate_drop_on_panic();

    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║  所有演示结束。注意所有 [DROP] 日志的输出顺序 ║");
    println!("╚══════════════════════════════════════════════════╝");
}
