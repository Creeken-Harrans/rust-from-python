# 参考答案

建议先独立完成练习，再阅读本文件。

---

## Level 1：基础巩固

### 1-1：Drop 顺序追踪

```rust
struct Tracker(&'static str);
impl Drop for Tracker {
    fn drop(&mut self) { println!("Drop: {}", self.0); }
}

fn main() {
    let a = Tracker("A");
    let b = Tracker("B");
    let c = Tracker("C");
    // 输出: Drop: C, Drop: B, Drop: A (LIFO逆序)
}
```

#### 结论

Drop 顺序是声明顺序的逆序（LIFO），类似于 C++ 局部对象析构顺序。与变量名无关，与声明位置有关。

---

### 1-2：Deref 练习

```rust
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
```

Deref 是编译器自动调用 `*` 时的钩子。对 `&MyBox<T>` 调用 `.method_of_T()` 时，编译器自动插入 `deref()` 调用（Deref 强制转换）。

#### 常见错误

- 忘记 `type Target = T;` 关联类型
- 误以为 Deref 能用于 Copy/Clone — 它只处理引用转换

---

### 1-3：提前 drop

```rust
let resource = Resource::new("temp");
drop(resource);  // 提前释放
// resource 已不可用
// 注意：std::mem::drop 只是一个空函数，通过获取所有权来触发 Drop
```

`drop()` vs `ManuallyDrop`：前者主动触发析构，后者阻止自动析构（用于 FFI 等特殊场景）。

---

## Level 2：组合应用

### 2-1：RAII 锁守卫

```rust
use std::sync::Mutex;

fn update_data(data: &Mutex<i32>) {
    let mut guard = data.lock().unwrap();
    *guard += 1; // 锁在 guard 离开作用域时自动释放
} // Drop for MutexGuard → 释放锁
```

关键：锁的自动释放由 `MutexGuard` 的 `Drop` 实现保证，不需要手动 `unlock()`。即使 `*guard += 1` panic，锁也会因栈展开而正确释放。

---

### 2-2：文件资源管理

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_config(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // File 的 Drop 自动关闭
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents) // file 在这里被 drop，自动关闭
    // 即使 ? 提前返回，file 也会在栈展开时正确关闭
}
```

与 C 对照：不需要手动 `fclose()`。与 Python 对照：不需要 `with open(...)`，行为等价但不需显式 `with`。

---

## Level 3：设计思考

### 3-1：RAII 管理非内存资源

| 资源 | 获取 | 释放（Drop） |
|------|------|------------|
| 堆内存 | `Box::new()` | `dealloc` |
| 文件 | `File::open()` | `close` |
| 互斥锁 | `Mutex::lock()` | `unlock` |
| 网络连接 | `TcpStream::connect()` | `shutdown` |
| 数据库事务 | `Transaction::begin()` | `rollback/commit` |

所有资源用同一模式管理 —— 这就是 RAII 的威力。

### 3-2：什么时候不应该依赖 Drop

1. **需要错误处理的清理**：Drop 是 infallible 的（`drop()` 返回 `()`），如果文件关闭可能失败，需要在 Drop 外显式处理
2. **需要保证顺序**：Rc/Arc 的 Drop 时机不确定，跨线程场景下不应依赖析构时机
3. **大型对象的性能敏感路径**：Drop 是隐式的，在热路径中可能累积开销

---

## 迁移思维练习

### C++ 析构函数 vs Rust Drop

| 方面 | C++ 析构 | Rust Drop |
|------|---------|----------|
| 语法 | `~ClassName()` | `impl Drop for T { fn drop(&mut self) }` |
| 调用时机 | 离开作用域 / `delete` | 离开作用域 |
| 能否手动调用 | 可（需谨慎） | 只用 `drop()` 间接 |
| 能否失败 | 不推荐（允许） | 不允许（签名无 Result） |
| RAII | 原生支持 | 原生 + 编译期所有权检查 |

**核心差异**：C++ RAII 依赖程序员自律（拷贝后双重释放是 UB），Rust 在此基础上加了编译期所有权检查——Move 语义使 RAII 的"谁负责释放"在编译期确定。

---

*RAII 加上 Rust 的所有权系统，让资源管理从"靠约定"变成了"靠编译器"。这是 Rust 在工程可靠性上的核心优势之一。*
