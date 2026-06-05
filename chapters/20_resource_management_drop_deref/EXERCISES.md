# 第20章练习: 资源管理 — Drop、Deref 与 RAII

## 练习说明

本章练习分为三个等级：
- **Level 1**：基础练习，巩固 Drop 和 Deref 的基本用法
- **Level 2**：进阶练习，需要综合运用所学知识
- **Level 3**：挑战题，涉及更复杂的设计和工程考量
- **思考题**：无需编码，但值得深入思考

---

## Level 1: 基础练习

### 练习 1-1: 实现一个 TempFile 类型

**目标**：实现一个 `TempFile` 类型，它在创建时打开一个临时文件，在 Drop 时自动删除该文件。

**要求**：
1. `TempFile::new(prefix: &str) -> std::io::Result<Self>` — 创建一个临时文件
2. `TempFile::path(&self) -> &Path` — 返回文件路径
3. `TempFile::write_all(&mut self, data: &[u8]) -> std::io::Result<()>` — 写入数据
4. `TempFile::read_to_string(&mut self) -> std::io::Result<String>` — 读取文件内容为字符串
5. 实现 `Drop`：删除磁盘上的临时文件
6. 在 Drop 中忽略删除失败（不 panic），但打印一条警告到 stderr

**提示**：
- 使用 `std::env::temp_dir()` 获取系统临时目录
- 使用 `std::fs::File` 进行文件操作
- 使用 `std::fs::remove_file` 删除文件
- Drop 中不要 panic

**预期行为**：
```rust
{
    let mut tmp = TempFile::new("rust-demo").unwrap();
    tmp.write_all(b"Hello, RAII!").unwrap();
    // tmp 离开作用域 → Drop → 删除临时文件
}
// 此处文件已被删除
```

---

### 练习 1-2: 实现带引用计数的 Drop 日志记录器

**目标**：实现一个 `LoggedResource` 类型，它在创建和销毁时打印日志，并维护一个全局的"存活资源计数"。

**要求**：
1. `LoggedResource::new(name: &str) -> Self` — 创建资源，打印 `[NEW]` 日志，全局计数 +1
2. 实现 `Drop`：打印 `[FREE]` 日志，全局计数 -1
3. `fn alive_count() -> usize` — 静态方法，返回当前存活的资源数量
4. 使用原子类型 (`AtomicUsize`) 保证线程安全

**提示**：
- 使用 `std::sync::atomic::AtomicUsize` 和 `Ordering::SeqCst`
- `alive_count()` 应为关联函数（非方法）

**预期输出示例**：
```
[NEW] resource-a: 第 1 个资源
[NEW] resource-b: 第 2 个资源
[FREE] resource-b: 剩余 1 个资源
[FREE] resource-a: 剩余 0 个资源
```

---

### 练习 1-3: 理解 Deref Coercion 的边界

**目标**：通过编译实验理解 Deref Coercion 的适用范围。

**任务**：阅读以下代码片段，判断每个调用是否能通过编译。如果不能，解释原因。

```rust
use std::ops::Deref;

struct MyVec<T>(Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> { &self.0 }
}

fn takes_slice(items: &[i32]) {
    println!("{} items", items.len());
}

fn takes_vec(items: &Vec<i32>) {
    println!("{} items in vec", items.len());
}

fn main() {
    let mv = MyVec(vec![1, 2, 3]);

    takes_slice(&mv);       // (1) 能否编译?
    takes_vec(&mv);         // (2) 能否编译?
    let s: &[i32] = &mv;    // (3) 能否编译?
    let v: &Vec<i32> = &mv; // (4) 能否编译?
    takes_slice(&*mv);      // (5) 能否编译?
}
```

**提示**：记住 Deref Coercion 只在需要类型转换的特定位置生效，不是所有地方都会自动转换。

---

## Level 2: 进阶练习

### 练习 2-1: 实现 Transaction Guard

**目标**：实现一个数据库事务守卫 `Transaction`，它保证事务要么被提交、要么被回滚。

**要求**：
1. `Transaction::begin(conn: &mut Connection) -> Self` — 开始事务
2. `Transaction::commit(self) -> Result<(), Error>` — 提交事务（消耗 self）
3. `Transaction::execute(&mut self, sql: &str) -> Result<(), Error>` — 执行 SQL
4. 实现 `Drop`：如果 `self` 没有被显式 `commit`，则自动 `rollback`
5. **关键**：`commit` 消耗 `self`（所有权转移），阻止 Drop 中的 rollback

**提示**：
- 使用内部的 `bool` 或 `Option` 标记事务状态
- `commit(self)` 消耗所有权后，self 不再存在，Drop 不会被调用
- 可以使用 `std::mem::ManuallyDrop` 或内部 `Option<bool>` 跟踪是否已处理
- 一个可行方案：在 `commit` 中设置 `committed = true`，Drop 检查该标记

**预期行为**：
```rust
fn test_transaction_rollback() {
    let mut conn = Connection::new();
    let mut tx = Transaction::begin(&mut conn);
    tx.execute("INSERT INTO users VALUES (1, 'Alice')").unwrap();
    // tx 离开作用域，没有 commit → Drop 自动 rollback
}

fn test_transaction_commit() {
    let mut conn = Connection::new();
    let mut tx = Transaction::begin(&mut conn);
    tx.execute("INSERT INTO users VALUES (1, 'Alice')").unwrap();
    tx.commit().unwrap();
    // tx 已被消耗，Drop 不会被调用 → 数据已提交
}
```

---

### 练习 2-2: 实现 AutoFlush Buffer

**目标**：实现一个自动刷新的写入缓冲区 `AutoFlushWriter`，它在 Drop 时自动将缓冲区内容写入目标。

**要求**：
1. `AutoFlushWriter::new(writer: W) -> Self` — 包装任意 `std::io::Write` 实现者
2. 实现 `std::io::Write` trait 给 `AutoFlushWriter<W>`
3. 实现 `Drop`：自动调用 `flush()` 方法
4. 使用 `ManuallyDrop` 或类似技术，在处理 Drop 时安全获取内部 writer
5. **挑战**：`drop` 获取 `&mut self`，但你需要对 writer 调用 `flush()`，这在 Drop 中是可行的；问题是如何在 Drop 后防止对 writer 的二次释放。使用 `ManuallyDrop` 包装 writer，在 Drop 中手动取出。

**提示**：
- 研究 `std::io::BufWriter` 的 Drop 实现作为参考
- 使用 `unsafe { std::ptr::read() }` 或 `ManuallyDrop::take()` 来取出内部值
- 如果 flush 失败，忽略错误但打印警告

---

## Level 3: 挑战题

### 练习 3-1: 实现 ResourceManager — 可取消的 RAII

**目标**：实现 `ResourceManager<T>`，它是一个 RAII 守卫，但支持"取消"自动清理。类似 `std::mem::forget` 但更安全可控。

**要求**：
1. `ResourceManager::new(resource: T, cleanup: F) -> Self`
   - `T` 是资源类型
   - `F: FnOnce(&mut T)` 是清理函数，在 Drop 时调用
2. `ResourceManager::cancel(self) -> T` — 取消自动清理，返回内部资源
3. `ResourceManager::resource(&self) -> &T` — 获取资源的不可变引用
4. `ResourceManager::resource_mut(&mut self) -> &mut T` — 获取资源的可变引用
5. 实现 `Drop`：调用 `cleanup` 函数
6. `cancel()` 消耗所有权并返回资源，防止 Drop 被调用
7. 实现 `Deref<Target = T>` 和 `DerefMut`，使 ResourceManager 像透明包装一样工作

**提示**：
- 使用 `Option<(T, F)>` 内部存储，`cancel()` 和 `Drop` 都通过 `take()` 取出
- Drop 中使用 `if let Some((mut res, cleanup)) = self.inner.take() { cleanup(&mut res); }`
- 注意 `F` 的约束：`FnOnce` 只能调用一次

**使用示例**：
```rust
// 示例: 自动回滚的配置修改
fn update_config() -> Result<(), Error> {
    let config = load_config();
    let mut manager = ResourceManager::new(
        config,
        |cfg| { save_config(cfg); println!("[AUTO-SAVE] 配置已保存"); }
    );

    manager.modify("key", "new_value")?;
    manager.modify("another_key", "another_value")?;

    // 一切顺利，取消自动保存（可能想用不同的方式保存）
    let config = manager.cancel();
    atomic_save(config)?;
    Ok(())
}
```

---

## 思考题

### 思考题: 为什么 Rust 不允许手动调用 drop() 方法？

在 Rust 中，以下代码无法编译：

```rust
let file = FileGuard::open("/tmp/test.txt");
file.drop(); // 编译错误: explicit use of destructor method
```

但在 C++ 中，以下操作是合法的：

```cpp
auto file = FileGuard("/tmp/test.txt");
file.~FileGuard(); // 合法: 显式调用析构函数
```

**请思考并回答以下问题**：

1. 为什么 Rust 选择禁止显式调用 `drop()` 方法？设计上的考量是什么？

2. 如果 Rust 允许显式调用 `drop()`，会出现什么问题？请给出具体的代码示例说明。

3. `std::mem::drop()` 函数（注意：这是标准库函数，不是 trait 方法）是如何解决"提前释放"这一需求的？它的实现只用了一行代码（空函数体），为什么这样就能工作？

4. `Drop::drop()` 和 `std::mem::drop()` 虽然名字相同，但本质完全不同。请从以下维度分析两者的区别：
   - 调用者
   - 参数类型
   - 返回值
   - 对所有权的影响
   - 是否触发析构逻辑

5. 在某些场景下，开发者确实需要在 Drop 之前完成一些操作，但又不想让 Drop 执行默认逻辑。除了 `std::mem::drop()`，Rust 还提供了哪些工具来处理这类需求？

**提示**：考虑 `ManuallyDrop`、`Option::take()` 模式、`mem::forget`、`mem::replace` 等。

---

## 推荐操作命令

### 基础验证

```bash
# 编译本章示例代码
cd chapters/20_resource_management_drop_deref
cargo build

# 运行本章示例代码
cargo run

# 以 Release 模式编译和运行
cargo build --release
cargo run --release
```

### 练习开发

```bash
# 创建练习项目
cargo new exercises_drop_deref
cd exercises_drop_deref

# 在 src/main.rs 中编写练习代码
# 然后编译运行
cargo build
cargo run
```

### 验证 Drop 行为

```bash
# 添加日志观察 Drop 顺序
# 使用 RUST_BACKTRACE 观察 panic 展开中的 Drop
RUST_BACKTRACE=1 cargo run

# 检查编译器的所有权错误（尝试在 drop 后使用值）
# 编译器会给出精确的错误信息
```

### 性能分析

```bash
# 对比 RAII vs 手动管理的性能
cargo bench  # 如果有基准测试

# 检查是否有意外的 Drop 调用（编译优化后）
cargo build --release
objdump -d target/release/resource_management | grep -A5 "drop"
```

---

## 练习要点总结

| 练习 | 核心知识点 | 难度 |
|------|-----------|------|
| 1-1 TempFile | Drop 实现、错误忽略 | Level 1 |
| 1-2 LoggedResource | 原子变量、全局状态、Drop 计数 | Level 1 |
| 1-3 Deref Coercion | Deref 的多层转换边界 | Level 1 |
| 2-1 Transaction | 所有权消耗、Commit/Rollback 模式 | Level 2 |
| 2-2 AutoFlush | 泛型包装、ManuallyDrop、unsafe | Level 2 |
| 3-1 ResourceManager | 闭包、所有权、取消模式 | Level 3 |
| 思考题 | 设计哲学、语言安全性 | — |

---

*练习设计目标：从 API 使用到内部实现，全面理解 Rust 资源管理机制。*
