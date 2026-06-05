# 第 19 章练习题: 智能指针 — Box\<T\>、Rc\<T\>、RefCell\<T\>

---

## 习题难度说明

- **Level 1**（★★☆☆☆）：基础练习，巩固本章核心概念，代码量约 10-30 行
- **Level 2**（★★★☆☆）：综合练习，需要结合多种智能指针，代码量约 30-80 行
- **Level 3**（★★★★☆）：设计练习，需要独立设计数据结构并做出类型选择
- **思考题**：无需编码，需书面回答

---

## Level 1 习题

### 习题 1-1: 用 Box 构建二叉树

**需求**：定义一个二叉搜索树（Binary Search Tree）的枚举类型 `Bst`，包含 `Empty`（空节点）和 `Node`（包含一个值、左子树和右子树）。然后实现以下功能：

1. 创建一个 3 层的二叉树示例
2. 实现一个函数 `bst_sum` 计算所有节点值的总和
3. 实现一个函数 `bst_to_vec` 将二叉树中序遍历转换为 `Vec<i32>`

**提示**：

```rust
enum Bst {
    Empty,
    Node(i32, Box<Bst>, Box<Bst>),
}
```

**期望**：你的代码应该通过 `cargo test`。确保递归函数正确处理 `Empty` 情况。

**运行命令**：

```bash
cargo test test_bst
```

---

### 习题 1-2: Rc 共享配置

**需求**：定义一个 `DatabaseConfig` 结构体，包含 `host`、`port` 和 `database_name` 三个字段（均为 `String`）。然后：

1. 使用 `Rc<DatabaseConfig>` 包装一份配置
2. 创建至少 3 个"服务"（ServiceA、ServiceB、ServiceC 结构体），每个服务持有 `Rc<DatabaseConfig>` 的克隆
3. 打印每个服务的配置信息
4. 在每个服务的 `drop` 时（或作用域结束时）打印当前的 `strong_count`

**提示**：

```rust
struct ServiceA {
    config: Rc<DatabaseConfig>,
    name: String,
}
```

**运行命令**：

```bash
cargo test test_rc_config
```

---

### 习题 1-3: RefCell 计数器

**需求**：实现一个 `HitCounter` 结构体，内部使用 `RefCell<u64>` 存储点击次数。关键要求：

1. `increment` 方法的签名必须是 `&self`（不是 `&mut self`）
2. `count` 方法返回当前计数
3. `reset` 方法将计数归零

然后编写测试：
- 创建计数器，调用 `increment` 3 次，验证计数为 3
- 借用 `HitCounter` 的不可变引用，仍能调用 `increment`（证明内部可变性）

**提示**：`HitCounter` 不需要 `mut` 就能被修改。

**运行命令**：

```bash
cargo test test_hit_counter
```

---

## Level 2 习题

### 习题 2-1: 用 Rc\<RefCell\<T\>\> 实现图

**需求**：设计一个简单的社交网络图结构：

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    friends: RefCell<Vec<Rc<Person>>>,
}
```

实现以下功能：

1. **创建人物**：`fn new_person(name: &str) -> Rc<Person>`
2. **添加朋友**：为 `Person` 实现 `add_friend(&self, friend: Rc<Person>)`——注意签名是 `&self`
3. **统计朋友的朋友（二度人脉）**：实现函数 `second_degree_count(person: &Rc<Person>) -> usize`，返回一个人所有朋友的朋友总数（去重）
4. **查找共同朋友**：实现函数 `common_friends(a: &Rc<Person>, b: &Rc<Person>) -> Vec<String>`

**示例测试**：

```rust
let alice = new_person("Alice");
let bob = new_person("Bob");
let charlie = new_person("Charlie");
let diana = new_person("Diana");

alice.add_friend(Rc::clone(&bob));
alice.add_friend(Rc::clone(&charlie));
bob.add_friend(Rc::clone(&charlie));
bob.add_friend(Rc::clone(&diana));
charlie.add_friend(Rc::clone(&diana));

assert_eq!(common_friends(&alice, &bob), vec!["Charlie"]);
```

**运行命令**：

```bash
cargo test test_social_graph
```

---

### 习题 2-2: 实现简单缓存（带内部可变性）

**需求**：实现一个泛型缓存 `Cache<K, V>`，使用 `RefCell` 实现内部可变性。

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<K, V> {
    storage: RefCell<HashMap<K, V>>,
}
```

要求实现的方法（都通过 `&self` 访问）：

| 方法 | 签名 | 功能 |
|------|------|------|
| `new` | `fn new() -> Self` | 创建空缓存 |
| `get` | `fn get(&self, key: &K) -> Option<V>` 其中 `V: Clone` | 获取值 |
| `set` | `fn set(&self, key: K, value: V)` | 设置值 |
| `get_or_insert` | `fn get_or_insert(&self, key: K, f: impl FnOnce() -> V) -> V` 其中 `V: Clone` | 有则返回，无则计算并插入 |
| `stats` | `fn stats(&self) -> (usize, usize)` | 返回(命中次数, 未命中次数) |

额外要求：
- 维护 `hits` 和 `misses` 计数器（也放在 `RefCell` 中）
- `get_or_insert` 方法：如果 key 存在，增加 hits 并返回已有值；如果不存在，调用 `f()` 计算值，增加 misses，插入并返回

**示例测试**：

```rust
let cache: Cache<String, i32> = Cache::new();
let v = cache.get_or_insert(String::from("key1"), || 42);
assert_eq!(v, 42);
assert_eq!(cache.stats(), (0, 1));

let v2 = cache.get_or_insert(String::from("key1"), || 100);
assert_eq!(v2, 42); // 返回缓存值，不是 100
assert_eq!(cache.stats(), (1, 1));
```

**运行命令**：

```bash
cargo test test_cache
```

---

## Level 3 习题

### 习题 3-1: 实现双向链表（安全版本）

**需求**：使用 `Rc`、`RefCell` 和 `Weak` 实现一个安全的双向链表。这是 Rust 中一个经典的设计挑战，因为双向链表天然存在引用循环。

**数据结构定义**：

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: RefCell<Option<Weak<RefCell<Node<T>>>>>,
    next: RefCell<Link<T>>,
}

struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}
```

请注意 prev 和 tail 使用 `Weak` 来避免循环引用（head→节点→next→节点→prev→前一个节点，但 prev 是 Weak，所以不阻止释放）。

**需要实现的方法**：

| 方法 | 功能 |
|------|------|
| `new()` | 创建空链表 |
| `push_back(&mut self, value: T)` | 在尾部插入元素 |
| `pop_front(&mut self) -> Option<T>` | 从头部移除并返回元素 |
| `len(&self) -> usize` | 返回链表长度 |

**示例测试**：

```rust
let mut list = DoublyLinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);
assert_eq!(list.len(), 3);
assert_eq!(list.pop_front(), Some(1));
assert_eq!(list.pop_front(), Some(2));
assert_eq!(list.pop_front(), Some(3));
assert_eq!(list.pop_front(), None);
```

**提示**：
- `push_back` 时，新建的节点的 `prev` 应指向原来的 `tail`
- `pop_front` 时，移除节点后需更新新头节点的 `prev` 为 `None`
- 确保 `tail` 始终是 `Weak`，否则会形成循环

**运行命令**：

```bash
cargo test test_doubly_linked_list
```

---

## 思考题

### 思考题 1: 智能指针的设计哲学

**题目**：请阅读以下场景，然后回答后续问题。

**场景 A**：你正在编写一个 Web 服务器的请求处理器。每个请求都需要访问一份只读的全局配置（数据库连接字符串、端口号等）。配置在程序启动时加载，之后不再改变。

**场景 B**：你正在实现一个编译器的符号表。在分析过程中，多个 AST 节点需要引用同一个符号定义，并且在符号解析过程中可能需要向符号表添加新的条目。

**场景 C**：你需要在内存中表示一个大型的材质纹理数据（~100MB），这个数据需要被场景图中的多个渲染节点引用。

**问题**：

1. 对于场景 A，你会选择 `Rc<Config>` 还是 `Arc<Config>`？为什么？如果配置是在编译期就确定的常量，你会改变选择吗？

2. 对于场景 B，为什么 `Rc<RefCell<SymbolTable>>` 比单纯的 `&mut SymbolTable` 更适合？如果编译器是多线程的（如 rust-analyzer），你的选择会如何变化？

3. 对于场景 C，为什么使用 `Arc<[u8]>` 或 `Arc<Vec<u8>>` 而不是 `Box<Vec<u8>>`？如果其中一个渲染节点需要修改纹理的一部分（例如，应用滤镜），你需要的类型是什么？

4. **核心问题**：Rust 选择将 `Box`、`Rc`、`RefCell` 等作为库类型实现（通过 trait），而不是像 C++ 的 `std::shared_ptr` 那样作为语言内置特性。这种设计决策有什么优缺点？请从以下角度讨论：
   - a) 类型系统的灵活性
   - b) 运行时开销的可选择性
   - c) 与其他语言特性的集成（如 `?` 运算符、match、模式匹配）
   - d) 学习曲线

---

## 推荐运行命令

```bash
# 运行本章所有演示
cargo run

# 运行所有测试
cargo test

# 运行单个测试（替换 test_name）
cargo test test_bst
cargo test test_rc_config
cargo test test_hit_counter
cargo test test_social_graph
cargo test test_cache
cargo test test_doubly_linked_list

# 查看测试输出（包括 println!）
cargo test -- --nocapture

# 构建发布版本
cargo build --release

# 检查代码风格
cargo clippy

# 格式化代码
cargo fmt

# 生成并打开文档
cargo doc --open
```

---

## 习题答案提示

以下是各题的关键提示，建议在尝试实现后再参考。

### 1-1 提示
- `Bst` 的 `Node` 变体使用 `Box<Bst>` 是必需的，因为 `Bst` 是递归类型
- 中序遍历顺序：左子树 → 当前节点 → 右子树
- 使用 `match` 处理 `Empty` 和 `Node` 两种情况

### 1-2 提示
- 使用 `Rc::clone` 而不是 `.clone()` 来明确表达意图
- 可以将 `strong_count` 的打印放在 `Drop` 实现中

### 1-3 提示
- `HitCounter` 的所有方法都接受 `&self`
- `RefCell::borrow_mut()` 返回 `RefMut<u64>`，可以通过 `*` 解引用来读写

### 2-1 提示
- 去重：将二度人脉收集到 `HashSet` 中
- 排除自身和直接朋友
- `common_friends`：使用 `contains` 或 intersect

### 2-2 提示
- `get_or_insert` 的逻辑：
  1. `storage.borrow().get(key)` → 如果存在，hits++，返回 clone
  2. 否则，`drop` 不可变借用，获取可变借用，insert，misses++
- 注意 RefCell 的借用冲突：在获取可变借用前必须释放不可变借用

### 3-1 提示
- `push_back` 步骤：
  1. 创建新节点，`next = None`
  2. 如果链表为空，`head = Some(new_node)`
  3. 如果链表非空，设置旧尾节点的 `next` 指向新节点，新节点的 `prev` 指向旧尾节点
- `pop_front` 步骤：
  1. 如果 `head` 为 `None`，返回 `None`
  2. 取出头节点的 `next`
  3. 如果 `next` 存在，将新头节点的 `prev` 设为 `None`
  4. 通过 `Rc::try_unwrap` 获取内部值（如果引用计数为 1）

---

## 评分标准（自我评估）

| 级别 | 标准 |
|------|------|
| Level 1 | 能正确使用 Box/Rc/RefCell 并编写测试 |
| Level 2 | 能组合多种智能指针解决复杂问题 |
| Level 3 | 能理解引用循环并正确使用 Weak 打破循环 |
| 思考题 | 能说清不同场景下的类型选择理由 |

---

*完成所有习题后，你应该能够自信地在实际项目中选择和使用合适的智能指针类型。*

---

## 迁移思维练习

> 以下问题帮助你思考 C++ 的智能指针模式如何重新建模为 Rust 的智能指针，以及各类型的适用边界。

### 问题 1：C++ unique_ptr 和 Rust Box<T> 的异同？

C++ 的 `std::unique_ptr<T>` 和 Rust 的 `Box<T>` 都表达"独占所有权的堆分配"，但它们在语言层面的行为有本质差异。`unique_ptr` 可以被移走，移走后原来的变量为空（可以被访问，但解引用是 UB）；Rust 的 `Box<T>` 在 move 后原变量无法再被编译器允许使用。另外，`unique_ptr` 可以手动 `release()` 掏出裸指针而不释放——这在 Rust 中对应什么操作？为什么 Rust 把这个操作标记为 unsafe？

**提示**：`Box::into_raw` 和 `Box::from_raw` 是 unsafe 的，因为在这两个调用之间，Rust 无法追踪裸指针的有效性——这部分责任由开发者承担。

### 问题 2：哪些共享关系需要 Rc<T>，哪些需要 Arc<T>？

C++ 程序员在单线程程序中也经常使用 `std::shared_ptr`——没有类似 `Rc` 的单线程替代品。在 Rust 中，`Rc<T>` 和 `Arc<T>` 的选择是一个重要的设计决策。`Rc` 不实现 `Send` 和 `Sync`，这意味着什么？如果你把 `Rc` 用于多线程上下文会怎样？反过来，如果单线程代码中全部使用 `Arc`，虽然正确但会引入什么不必要的开销？

**提示**：`Arc` 使用原子引用计数（atomic operations），比 `Rc` 的非原子操作有额外的 CPU 开销；Rust 的设计让你只在需要线程安全时才付出这个代价。

### 问题 3：Rc<RefCell<T>> 虽然方便，为什么不应该滥用？

很多 Rust 学习者学会了 `Rc<RefCell<T>>` 组合后，会用它在任何需要"共享可变数据"的场景——这种模式有时被称为 Rust 版的"逃逸出口"。但过度使用 `Rc<RefCell<T>>` 会带来什么问题？从运行时借用检查（RefCell 的 borrow/borrow_mut 会在运行时 panic）、所有权模型模糊化、性能开销三个角度分析。在什么场景下你应该重新审视设计，尝试用唯一所有者 + 借用代替？

**提示**：`Rc<RefCell<T>>` 把 Rust 的一些编译期保证降级为运行时检查——这对于原型开发很有用，但在生产代码中应该尽量把所有权模型捋清。
