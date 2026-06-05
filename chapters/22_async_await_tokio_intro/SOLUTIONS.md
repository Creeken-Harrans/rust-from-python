# 迁移思维练习答案 — 异步编程与 Tokio

## 迁移思维练习答案

### 1. Python asyncio 和 Rust async/await 的关键区别在哪里？

Python 的 async 函数调用后立即开始执行——调用 `async def foo()` 返回一个 coroutine 对象，await 它时由事件循环调度执行。Rust 的 async fn 返回 Future，是惰性的——仅调用 async fn 不做任何事，必须 .await 或被交给 runtime（如 tokio::spawn）才会执行。Rust 标准库不提供异步运行时（需引入 Tokio、async-std 等第三方库），而 Python 的 asyncio 是标准库一部分。两者都是协作式调度，但 Rust 的 Future 经过编译器的状态机转换，避免了 Python coroutine 的堆分配开销。

### 2. 什么任务适合 async，什么任务适合多线程？

I/O 密集型任务（网络请求、文件读写、数据库查询、等待外部服务）适合 async——用少量操作系统线程管理海量并发连接，在等待 I/O 时让出 CPU 给其他任务。CPU 密集型任务（数学计算、图像/视频处理、加密解密、大 JSON 解析）适合多线程或 rayon 等并行库——需要真正使用多个 CPU 核心并行计算。混合场景的常见方案：async 作为主体框架，CPU 密集部分通过 tokio::task::spawn_blocking 交给专用线程池，避免阻塞异步事件循环。

### 3. 从 Python asyncio 迁移到 Rust async 需要注意什么陷阱？

第一个陷阱是"忘记 .await"——Rust 的 Future 是惰性的，如果不 .await 或 spawn，什么都不会发生，编译器通常会给出 `unused` 警告但不会报错。第二个是"阻塞异步 runtime"——在 async 上下文中调用 std::thread::sleep 或执行耗时循环会阻塞整个线程的事件循环，应该用 tokio::time::sleep 和 spawn_blocking。第三个是生命周期问题——async 函数返回的 Future 必须拥有其使用的数据或确保引用活得足够久，这在与借用交互时可能让人困惑。
