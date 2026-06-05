# 第 19 章练习题答案 — 智能指针：Box、Rc、RefCell

---

## Level 1：基础练习

---

### 习题 1-1: 用 Box 构建二叉树

**结论**

`Box<T>` 在递归数据结构中是**必需的**而非可选的。Rust 必须在编译期确定每个类型的大小，而 `Bst::Node(i32, Bst, Bst)` 会产生无限递归的大小计算——编译器需要一个间接层来打破递归。`Box<Bst>` 的大小等于一个指针（usize），编译器可以确定整个枚举的大小为 `max(sizeof(i32) + 2 * sizeof(usize), sizeof(Empty))`。

`Option` 在此场景也可用（如 `Option<Box<Node>>`），但枚举直接包含 `Empty` 变体更加语义清晰——"树不存在"表达为空树，而非"节点不存在"表达为 `None`。

**思路**

1. 定义 `Bst` 枚举：`Empty` 和 `Node(i32, Box<Bst>, Box<Bst>)`
2. `bst_sum`：递归遍历，`Empty` 返回 0，`Node` 返回当前值 + 左子树和 + 右子树和
3. `bst_to_vec`：中序遍历——先左子树，再当前节点值，再右子树

**参考实现**

```rust
#[derive(Debug, PartialEq)]
enum Bst {
    Empty,
    Node(i32, Box<Bst>, Box<Bst>),
}

use Bst::{Empty, Node};

impl Bst {
    /// 计算二叉树所有节点值的总和
    fn bst_sum(&self) -> i32 {
        match self {
            Empty => 0,
            Node(value, left, right) => value + left.bst_sum() + right.bst_sum(),
        }
    }

    /// 中序遍历：左子树 -> 当前节点 -> 右子树
    fn bst_to_vec(&self) -> Vec<i32> {
        match self {
            Empty => Vec::new(),
            Node(value, left, right) => {
                let mut result = left.bst_to_vec();
                result.push(*value);
                result.extend(right.bst_to_vec());
                result
            }
        }
    }
}

fn main() {
    // 构建三层二叉树:
    //        5
    //       / \
    //      3   8
    //     / \   \
    //    1   4   9
    let tree = Node(
        5,
        Box::new(Node(
            3,
            Box::new(Node(1, Box::new(Empty), Box::new(Empty))),
            Box::new(Node(4, Box::new(Empty), Box::new(Empty))),
        )),
        Box::new(Node(
            8,
            Box::new(Empty),
            Box::new(Node(9, Box::new(Empty), Box::new(Empty))),
        )),
    );

    println!("树的总和: {}", tree.bst_sum());
    println!("中序遍历: {:?}", tree.bst_to_vec());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_sum() {
        let tree = Node(10, Box::new(Empty), Box::new(Empty));
        assert_eq!(tree.bst_sum(), 10);

        let empty = Empty;
        assert_eq!(empty.bst_sum(), 0);
    }

    #[test]
    fn test_bst_to_vec() {
        //   2
        //  / \
        // 1   3
        let tree = Node(
            2,
            Box::new(Node(1, Box::new(Empty), Box::new(Empty))),
            Box::new(Node(3, Box::new(Empty), Box::new(Empty))),
        );
        assert_eq!(tree.bst_to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_bst() {
        let tree = Node(
            5,
            Box::new(Node(3,
                Box::new(Node(1, Box::new(Empty), Box::new(Empty))),
                Box::new(Node(4, Box::new(Empty), Box::new(Empty))),
            )),
            Box::new(Node(8,
                Box::new(Empty),
                Box::new(Node(9, Box::new(Empty), Box::new(Empty))),
            )),
        );
        assert_eq!(tree.bst_sum(), 30); // 1+3+4+5+8+9
        assert_eq!(tree.bst_to_vec(), vec![1, 3, 4, 5, 8, 9]);
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 不使用 `Box`，直接 `Node(i32, Bst, Bst)` | 编译器报错：`recursive type has infinite size`。递归类型必须通过指针间接引用 |
| 中序遍历顺序错误 | 中序 = 左 -> 当前 -> 右。先序（当前 -> 左 -> 右）和后序（左 -> 右 -> 当前）是不同遍历方式 |
| `Empty` 分支返回 `vec![]` 但在外部拼接 | `bst_to_vec` 中 `Empty` 应返回空 Vec，`Node` 分支拼接左 + 当前 + 右 |
| 使用 `Rc` 代替 `Box` | 二叉树是严格树状结构（无环、单所有者），`Box` 是正确选择。`Rc` 引入不必要的引用计数开销 |

**验证方式**

```bash
cargo test test_bst
cargo run
# 预期输出:
# 树的总和: 30
# 中序遍历: [1, 3, 4, 5, 8, 9]
```

---

### 习题 1-2: Rc 共享配置

**结论**

`Rc<T>` 用于单线程中**不可变数据的共享所有权**。它不像 `Box<T>` 要求唯一所有者，也不像 `Arc<T>` 需要原子引用计数。`Rc::clone` 是廉价操作（仅增加引用计数，不克隆数据），命名上的 `clone` 而非隐式拷贝强调了"共享"的意图。

**Arc 不自动保证线程安全**：`Arc` 保证引用计数本身是线程安全的（原子递增/递减），但 `Arc<T>` 中 `T` 的数据安全性仍取决于 `T` 自身的类型约束。`Arc<Mutex<T>>` 提供线程安全共享，`Arc<RefCell<T>>` 不能跨线程（RefCell 不实现 Sync）。

**思路**

1. 定义 `DatabaseConfig` 结构体包含 `host`、`port`、`database_name`
2. 用 `Rc::new(config)` 创建共享配置
3. 每个 Service 存储 `Rc<DatabaseConfig>`，通过 `Rc::clone` 共享
4. 实现 `Drop` trait 打印 `Rc::strong_count`

**参考实现**

```rust
use std::rc::Rc;

#[derive(Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    database_name: String,
}

struct ServiceA {
    config: Rc<DatabaseConfig>,
    name: String,
}

struct ServiceB {
    config: Rc<DatabaseConfig>,
    name: String,
}

struct ServiceC {
    config: Rc<DatabaseConfig>,
    name: String,
}

impl Drop for ServiceA {
    fn drop(&mut self) {
        println!(
            "[{} drop] strong_count = {}",
            self.name,
            Rc::strong_count(&self.config)
        );
    }
}

impl Drop for ServiceB {
    fn drop(&mut self) {
        println!(
            "[{} drop] strong_count = {}",
            self.name,
            Rc::strong_count(&self.config)
        );
    }
}

impl Drop for ServiceC {
    fn drop(&mut self) {
        println!(
            "[{} drop] strong_count = {}",
            self.name,
            Rc::strong_count(&self.config)
        );
    }
}

fn main() {
    let config = Rc::new(DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        database_name: "mydb".to_string(),
    });

    println!("初始 strong_count = {}", Rc::strong_count(&config));

    // 三个服务共享同一份配置（不复制数据）
    let service_a = ServiceA {
        config: Rc::clone(&config),
        name: "ServiceA".to_string(),
    };
    println!("创建 ServiceA 后 strong_count = {}", Rc::strong_count(&config));

    let service_b = ServiceB {
        config: Rc::clone(&config),
        name: "ServiceB".to_string(),
    };

    let service_c = ServiceC {
        config: Rc::clone(&config),
        name: "ServiceC".to_string(),
    };
    println!("创建所有服务后 strong_count = {}", Rc::strong_count(&config));

    println!("ServiceA: {:?}", service_a.config);
    println!("ServiceB: {:?}", service_b.config);
    println!("ServiceC: {:?}", service_c.config);
}
// 作用域结束，service_a/b/c/config 依次 drop，strong_count 逐步递减到 0
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 用 `.clone()` 而不是 `Rc::clone()` | `.clone()` 也会工作（Rc 实现了 Clone），但 `Rc::clone` 明确表达"增加引用计数"的意图，且不会误克隆内部数据 |
| 在需要 `Arc` 的场景使用 `Rc` | 多线程场景下 `Rc` 不实现 `Send`/`Sync`，编译器会拒绝。但这不应被视为限制——编译器在阻止真正的数据竞争风险 |
| 误以为 `Rc::clone` 会深拷贝数据 | `Rc` 内部只在堆上保存一份数据，`Rc::clone` 只增加引用计数器 |
| `strong_count` 在 Drop 中可能为 1 | 当作用域结束时，Drop 按声明顺序的反序执行。最后一个持有者的 Drop 中 `strong_count` 为 1（只剩自己） |

**验证方式**

```bash
cargo test test_rc_config
cargo run
# 预期输出:
# 初始 strong_count = 1
# 创建 ServiceA 后 strong_count = 2
# 创建所有服务后 strong_count = 4
# ServiceA: DatabaseConfig { host: "localhost", ... }
# ...
# [ServiceC drop] strong_count = 2
# [ServiceB drop] strong_count = 2
# [ServiceA drop] strong_count = 2
# (config 最后 drop，strong_count 降为 0)
```

---

### 习题 1-3: RefCell 计数器

**结论**

`RefCell<T>` 实现**内部可变性**（interior mutability）：允许通过 `&self`（不可变引用）修改内部数据。关键洞察是——`RefCell` 不改变 Rust 的借用规则，它只是将规则的检查从**编译期移到运行时**。

**RefCell 不是绕过规则**：`RefCell` 的 `borrow_mut()` 在运行时检查：如果已有活跃的不可变借用，`borrow_mut()` 会 panic。双重可变借用同样会导致 panic。`RefCell` 是为特定设计模式服务的（如观察者模式、mock 对象），而非逃避借用检查器的万能工具。

**思路**

1. 结构体持有一个 `RefCell<u64>`
2. `increment(&self)`：调用 `self.count.borrow_mut()` 获取 `RefMut<u64>`，解引用后 `+= 1`
3. `count(&self)`：调用 `self.count.borrow()` 获取 `Ref<u64>`，解引用后复制返回
4. `reset(&self)`：`borrow_mut()` 后 `*count = 0`

**参考实现**

```rust
use std::cell::RefCell;

struct HitCounter {
    count: RefCell<u64>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter {
            count: RefCell::new(0),
        }
    }

    /// 增加计数。注意签名是 `&self`，不是 `&mut self`
    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }

    /// 返回当前计数
    fn count(&self) -> u64 {
        *self.count.borrow()
    }

    /// 重置计数
    fn reset(&self) {
        *self.count.borrow_mut() = 0;
    }
}

fn main() {
    let counter = HitCounter::new();

    counter.increment();
    counter.increment();
    counter.increment();
    println!("计数: {}", counter.count()); // 3

    counter.reset();
    println!("重置后: {}", counter.count()); // 0

    // 证明内部可变性：counter 是不可变绑定
    let counter_ref = &counter; // 不可变引用
    counter_ref.increment();    // 但仍能修改内部状态！
    println!("通过不可变引用增加后: {}", counter.count()); // 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_counter() {
        let counter = HitCounter::new();
        assert_eq!(counter.count(), 0);

        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.count(), 3);

        counter.reset();
        assert_eq!(counter.count(), 0);

        // 不可变引用下仍可调用 increment
        let r = &counter;
        r.increment();
        assert_eq!(r.count(), 1);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_refcell_double_borrow_mut_panics() {
        let counter = HitCounter::new();
        let _a = counter.count.borrow_mut();
        let _b = counter.count.borrow_mut(); // panic! 运行时借用检查
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 把 `increment` 签名写成 `&mut self` | 这就不需要用 `RefCell` 了。`RefCell` 的价值正是 `&self` 签名下的可变性 |
| 同时持有 `borrow` 和 `borrow_mut` | 运行时 panic：`already borrowed: BorrowMutError`。这是 `RefCell` 不是绕过规则的直接证据——它在运行时执行了与编译期相同的检查 |
| 误以为 `RefCell<T>` 对 `T: Send + Sync` 时也是线程安全的 | `RefCell` 不实现 `Sync`，编译器会阻止跨线程使用。线程安全场景请使用 `Mutex` 或 `RwLock` |
| 忘记解引用 `RefMut`/`Ref` | `self.count.borrow()` 返回 `Ref<u64>`，需要通过 `*` 或 `.clone()` 取出值 |

**验证方式**

```bash
cargo test test_hit_counter
cargo run
# 预期输出:
# 计数: 3
# 重置后: 0
# 通过不可变引用增加后: 1
```

---

## Level 2：综合练习

---

### 习题 2-1: 用 Rc<RefCell<T>> 实现图

**结论**

`Rc<RefCell<T>>` 组合是 Rust 中实现**可变图结构**（有向图、社交网络）的标准模式：
- `Rc` 提供多重所有权——多个人可以共享同一个朋友的关系
- `RefCell` 提供内部可变性——可以在不持有 `&mut Person` 的情况下修改朋友列表

这个组合的代价是：借用检查完全推迟到运行时。如果两个 `Person` 在 `add_friend` 时互相持有 `RefCell` 的 `borrow_mut`，会导致死锁般的 panic。因此需要在设计上避免同时可变借用多个节点。

**思考 2-1 中"RefCell 不是绕过规则"的体现**：`friends.borrow_mut()` 在运行时仍然检查独占性——如果已经持有 `friends.borrow()` 的不可变借用，再调用 `borrow_mut()` 会 panic。你只是把检查时间移到了运行时，规则本身没变。

**思路**

1. `Person` 使用 `RefCell<Vec<Rc<Person>>>` 存储朋友列表
2. `new_person` 返回 `Rc<Person>`
3. `add_friend(&self, ...)` 通过 `self.friends.borrow_mut().push(friend)`
4. `second_degree_count`：遍历直接朋友 -> 遍历每个朋友的朋友 -> 用 HashSet 去重（排除自身和直接朋友）
5. `common_friends`：收集 a 的朋友到 HashSet，遍历 b 的朋友检查交集 -> 收集名字

**参考实现**

```rust
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct Person {
    name: String,
    friends: RefCell<Vec<Rc<Person>>>,
}

fn new_person(name: &str) -> Rc<Person> {
    Rc::new(Person {
        name: name.to_string(),
        friends: RefCell::new(Vec::new()),
    })
}

impl Person {
    fn add_friend(&self, friend: Rc<Person>) {
        self.friends.borrow_mut().push(friend);
    }
}

/// 计算二度人脉数量（朋友的朋友，去重，排除自身和直接朋友）
fn second_degree_count(person: &Rc<Person>) -> usize {
    let direct_friends = person.friends.borrow();
    let direct_set: HashSet<*const Person> = direct_friends
        .iter()
        .map(|f| Rc::as_ptr(f))
        .collect();

    let mut second_degree = HashSet::new();
    let self_ptr = Rc::as_ptr(person);

    for friend in direct_friends.iter() {
        for friend_of_friend in friend.friends.borrow().iter() {
            let ptr = Rc::as_ptr(friend_of_friend);
            // 排除自身和直接朋友
            if ptr != self_ptr && !direct_set.contains(&ptr) {
                second_degree.insert(ptr);
            }
        }
    }

    second_degree.len()
}

/// 查找共同朋友
fn common_friends(a: &Rc<Person>, b: &Rc<Person>) -> Vec<String> {
    let friends_a = a.friends.borrow();
    let a_set: HashSet<*const Person> = friends_a
        .iter()
        .map(|f| Rc::as_ptr(f))
        .collect();

    let friends_b = b.friends.borrow();
    let mut common = Vec::new();

    for friend in friends_b.iter() {
        if a_set.contains(&Rc::as_ptr(friend)) {
            common.push(friend.name.clone());
        }
    }

    common.sort();
    common
}

fn main() {
    let alice = new_person("Alice");
    let bob = new_person("Bob");
    let charlie = new_person("Charlie");
    let diana = new_person("Diana");

    alice.add_friend(Rc::clone(&bob));
    alice.add_friend(Rc::clone(&charlie));
    bob.add_friend(Rc::clone(&charlie));
    bob.add_friend(Rc::clone(&diana));
    charlie.add_friend(Rc::clone(&diana));

    println!("Alice 的二度人脉: {}", second_degree_count(&alice));
    println!("共同朋友: {:?}", common_friends(&alice, &bob));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_graph() {
        let alice = new_person("Alice");
        let bob = new_person("Bob");
        let charlie = new_person("Charlie");
        let diana = new_person("Diana");

        alice.add_friend(Rc::clone(&bob));
        alice.add_friend(Rc::clone(&charlie));
        bob.add_friend(Rc::clone(&charlie));
        bob.add_friend(Rc::clone(&diana));
        charlie.add_friend(Rc::clone(&diana));

        // Alice 和 Bob 的共同朋友
        assert_eq!(common_friends(&alice, &bob), vec!["Charlie"]);

        // Alice 的二度人脉：Diana（通过 Bob 和 Charlie）
        assert_eq!(second_degree_count(&alice), 1);
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 二度人脉忘记去重 | 如果两个朋友都连接同一个人，不去重就会重复计数 |
| 二度人脉包含直接朋友 | "二度"意味着至少隔一个人。需要在去重时排除直接朋友集合 |
| 用 `Rc::ptr_eq` 比较时混用 Raw Pointer | `Rc::as_ptr` 返回 `*const T`，可以在集合中使用。`Rc::ptr_eq` 是另一种比较方式 |
| 共同朋友使用字符串比较 | 用 `Rc::as_ptr` 的指针比较更可靠（同一人只有一个 Rc 管理），字符串比较理论上允许多个同名人但不同的 Person |
| `add_friend` 双向链接导致循环引用 | 如果 `alice.add_friend(bob)` 和 `bob.add_friend(alice)` 都执行，`Rc` 的强引用形成循环——但这里单向朋友关系是合理的，不存在循环（朋友列表只是引用，不要求双向） |

**验证方式**

```bash
cargo test test_social_graph
cargo run
# 预期输出:
# Alice 的二度人脉: 1  (Diana)
# 共同朋友: ["Charlie"]
```

---

### 习题 2-2: 实现简单缓存（带内部可变性）

**结论**

`Cache<K, V>` 使用 `RefCell<HashMap<K, V>>` 实现内部可变性，同时维护 `RefCell<(usize, usize)>` 作为命中/未命中计数器。关键实现要点是 `RefCell` 的借用规则：**在获取可变借用前必须先释放不可变借用**。`get_or_insert` 方法需要：先 `borrow()` 检查 key 是否存在，如果不存在则 `drop` 不可变借用，再 `borrow_mut()` 插入新值。

这是"RefCell 不是绕过规则"的典型示范——如果你忘记 drop 不可变借用就尝试 `borrow_mut()`，运行时会 panic。编译器不再帮你检查，但你仍需手动维护借用规则。

**思路**

1. `Cache<K, V>` 包含两个 `RefCell`：`storage: RefCell<HashMap<K, V>>` 和 `stats: RefCell<(usize, usize)>`（hits, misses）
2. `get`：`borrow()` -> `get(key).cloned()`
3. `set`：`borrow_mut()` -> `insert(key, value)`
4. `get_or_insert`：先 `borrow()` 查找 -> 找到则 hits+1 并 clone 返回 -> 未找到则 drop `Ref` -> `borrow_mut()` 插入 -> misses+1
5. 也可以把 hits/misses 和 storage 放在同一个 `RefCell` 中（一个包含 `(HashMap<K,V>, usize, usize)` 的元组结构），减少 RefCell 数量和借用冲突风险

**参考实现**

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<K, V> {
    storage: RefCell<HashMap<K, V>>,
    hits: RefCell<usize>,
    misses: RefCell<usize>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn new() -> Self {
        Cache {
            storage: RefCell::new(HashMap::new()),
            hits: RefCell::new(0),
            misses: RefCell::new(0),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.storage.borrow().get(key).cloned()
    }

    fn set(&self, key: K, value: V) {
        self.storage.borrow_mut().insert(key, value);
    }

    /// 获取或插入：如果 key 存在返回缓存值，否则调用 f() 计算并缓存
    fn get_or_insert(&self, key: K, f: impl FnOnce() -> V) -> V {
        // 步骤 1: 先检查是否存在（不可变借用）
        {
            let storage = self.storage.borrow();
            if let Some(value) = storage.get(&key) {
                // 命中
                *self.hits.borrow_mut() += 1;
                return value.clone();
            }
        } // 不可变借用在此释放

        // 步骤 2: 不存在，计算并插入（可变借用）
        let value = f();
        self.storage.borrow_mut().insert(key, value.clone());
        *self.misses.borrow_mut() += 1;
        value
    }

    fn stats(&self) -> (usize, usize) {
        (*self.hits.borrow(), *self.misses.borrow())
    }
}

fn main() {
    let cache: Cache<String, i32> = Cache::new();

    // 第一次访问 key1 -> miss，用闭包计算
    let v = cache.get_or_insert(String::from("key1"), || 42);
    println!("key1 = {}, stats = {:?}", v, cache.stats());

    // 第二次访问 key1 -> hit，返回缓存值
    let v2 = cache.get_or_insert(String::from("key1"), || 100);
    println!("key1 = {}, stats = {:?}", v2, cache.stats());

    // 新 key -> miss
    let v3 = cache.get_or_insert(String::from("key2"), || 99);
    println!("key2 = {}, stats = {:?}", v3, cache.stats());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let cache: Cache<String, i32> = Cache::new();

        // 首次访问: miss
        let v = cache.get_or_insert(String::from("key1"), || 42);
        assert_eq!(v, 42);
        assert_eq!(cache.stats(), (0, 1));

        // 再次访问: hit，返回缓存值（不是 100）
        let v2 = cache.get_or_insert(String::from("key1"), || 100);
        assert_eq!(v2, 42);
        assert_eq!(cache.stats(), (1, 1));

        // 另一个 key: miss
        let v3 = cache.get_or_insert(String::from("key2"), || 7);
        assert_eq!(v3, 7);
        assert_eq!(cache.stats(), (1, 2));

        // set 直接写入
        cache.set(String::from("key3"), 99);
        assert_eq!(cache.get(&String::from("key3")), Some(99));
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| 在 `borrow()` 持有期间调用 `borrow_mut()` | **运行时 panic**。必须先 `drop` 不可变借用。上面的代码用 `{ }` 块显式限定 `Ref` 的作用域 |
| hits/misses 使用普通整数而非 `RefCell` | 如果 `stats` 计数器不用 `RefCell`，`get_or_insert` 的签名需要 `&mut self`，失去了内部可变性的便利 |
| `get_or_insert` 的闭包签名用 `FnOnce` 但实际多次调用 | `FnOnce` 只能调用一次（消费自身）。缓存场景中闭包仅在 miss 时调用，`FnOnce` 是正确选择 |
| `get` 方法忘记 `Clone` bound | `HashMap::get` 返回 `Option<&V>`。返回 `Option<V>` 需要 `V: Clone` |

**验证方式**

```bash
cargo test test_cache
cargo run
# 预期输出:
# key1 = 42, stats = (0, 1)
# key1 = 42, stats = (1, 1)
# key2 = 99, stats = (1, 2)
```

---

## Level 3：设计练习

---

### 习题 3-1: 实现双向链表（安全版本）

**结论**

双向链表是 Rust 中验证"所有权模型"的经典挑战。核心难点是：
- `next` 需要强引用（`Rc`）——节点被引用时不应释放
- `prev` 需要弱引用（`Weak`）——如果 prev 也用强引用，head -> node1 -> node2 -> node1 形成循环，永远无法释放

`Weak` 不增加 `strong_count`，不影响节点是否被释放。当 `strong_count` 降为 0 时，节点被释放，所有指向它的 `Weak` 升级失败（返回 `None`）。

**Arc 不自动保证线程安全**（本习题语境）：即使使用了 `Arc`，也只是引用计数操作是原子的。如果要跨线程共享双向链表，还需要 `Mutex` 或 `RwLock` 包装节点内容，确保对 `next`/`prev` 的并发修改不会产生数据竞争。`Arc` 本身只保证内存不会被过早释放，不提供互斥语义。

**思路**

1. `Node<T>` 定义：`value: T`、`prev: RefCell<Option<Weak<RefCell<Node<T>>>>>`、`next: RefCell<Link<T>>`
2. `push_back`：
   - 创建新节点，`next = None`
   - 如果链表为空：`head` 指向新节点，`tail` 也指向新节点（Weak）
   - 如果链表非空：旧尾节点的 `next` 设为新节点的 `Rc`，新节点的 `prev` 设为旧尾节点的 `Weak`
   - 更新 `tail`
3. `pop_front`：
   - 取 `head` 的 `Rc`
   - 新 `head` = 旧 `head` 的 `next`
   - 如果新 `head` 存在，将其 `prev` 设为 `None`
   - 否则链表变空，`tail = None`
   - 尝试 `Rc::try_unwrap`（仅当唯一引用时成功）取出值

**参考实现**

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
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        }));

        match self.tail.take() {
            None => {
                // 链表为空：新节点既是 head 也是 tail
                self.head = Some(Rc::clone(&new_node));
            }
            Some(old_tail_weak) => {
                if let Some(old_tail) = old_tail_weak.upgrade() {
                    // 旧尾节点的 next 指向新节点
                    *old_tail.borrow_mut().next.borrow_mut() = Some(Rc::clone(&new_node));
                    // 新节点的 prev 指向旧尾节点（Weak 引用）
                    *new_node.borrow_mut().prev.borrow_mut() =
                        Some(Rc::downgrade(&old_tail));
                } else {
                    // old_tail 已被释放，当作空链表处理
                    self.head = Some(Rc::clone(&new_node));
                }
            }
        }

        self.tail = Some(Rc::downgrade(&new_node));
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                let next = old_head.borrow_mut().next.borrow_mut().take();

                match next {
                    Some(ref new_head_rc) => {
                        // 新头节点的 prev 设为 None
                        *new_head_rc.borrow_mut().prev.borrow_mut() = None;
                        self.head = Some(Rc::clone(new_head_rc));
                    }
                    None => {
                        // 链表变空
                        self.head = None;
                        self.tail = None;
                    }
                }

                self.len -= 1;

                // 尝试取出值：当只有 head 一个强引用时成功
                Rc::try_unwrap(old_head)
                    .ok()
                    .map(|node| node.into_inner().value)
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("len = {}", list.len());
    println!("pop = {:?}", list.pop_front());
    println!("pop = {:?}", list.pop_front());
    println!("pop = {:?}", list.pop_front());
    println!("pop = {:?}", list.pop_front());
    println!("len = {}", list.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_linked_list() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_pop_interleaved() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        assert_eq!(list.pop_front(), Some(1));

        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(2));
        list.push_back(4);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), None);
    }
}
```

**常见错误**

| 错误 | 说明 |
|------|------|
| `prev` 使用 `Rc` 而非 `Weak` | 形成强引用循环：head -> node1 -> node2 -> node1（通过 prev）。所有节点永远不会被释放（内存泄漏） |
| `pop_front` 后忘记更新新 head 的 `prev` | 新 head 的 `prev` 仍指向已移除的节点，后续通过 `prev` 回溯时会访问悬垂 Weak（升级失败但不会 UB，只是逻辑错误） |
| `Rc::try_unwrap` 失败时静默忽略 | 如果节点仍被其他地方引用（如外部持有的 Weak 已被 upgrade），`try_unwrap` 返回 `Err`。生产代码应处理这种情况 |
| `tail` 忘记在 `pop_front` 清空后设为 `None` | 当链表变空时，`tail` 仍持有对最后一个节点的 Weak。虽然 Weak 不影响释放，但语义上不正确 |
| 使用 `std::rc::Rc::downgrade` 而非 `Rc::downgrade` | 两者等效，但后者更简洁（通过 Rc 的关联函数调用） |

**验证方式**

```bash
cargo test test_doubly_linked_list
cargo run
# 预期输出:
# len = 3
# pop = Some(1)
# pop = Some(2)
# pop = Some(3)
# pop = None
# len = 0
```

---

## 思考题

---

### 思考题 1: 智能指针的设计哲学

**1. 场景 A：共享只读全局配置**

选择 `Arc<Config>`，原因：
- Web 服务器天然多线程（每个请求可能在不同线程处理），`Rc` 不实现 `Send`/`Sync`，编译器会拒绝对 `Rc` 的跨线程使用
- 如果配置在编译期就确定：可以用 `&'static Config`（静态生命周期引用）或 `const CONFIG: Config = ...`，完全不需要引用计数

**Arc 不自动保证线程安全**在此场景的含义：`Arc<Config>` 只保证 `Config` 在多线程中共享时的内存管理是安全的（通过原子引用计数），但 `Config` 内部的字段如果本身不是 `Sync`，则无法跨线程共享——你需要 `Arc<Config>` 且 `Config: Sync`。

**2. 场景 B：编译器符号表**

`Rc<RefCell<SymbolTable>>` 比 `&mut SymbolTable` 更适合，原因：
- 符号表需要被多个 AST 节点**同时**持有引用。`&mut` 要求唯一借用，这在 AST 遍历时不可能（多个节点同时存在）
- `RefCell` 允许通过共享引用修改，而 `&mut` 独占性使其无法多处持有

如果编译器是多线程的（如 rust-analyzer）：将 `Rc` 替换为 `Arc`，将 `RefCell` 替换为 `Mutex` 或 `RwLock`。`Arc<Mutex<SymbolTable>>` 允许跨线程安全共享并互斥修改。`Arc` 不自动保证线程安全——你需要 `Mutex` 来提供互斥语义。

**3. 场景 C：大型纹理数据**

使用 `Arc<[u8]>` 或 `Arc<Vec<u8>>` 而非 `Box<Vec<u8>>`，原因：
- 100MB 的数据被多个渲染节点引用，`Box` 只能有一个所有者——其他节点需要通过借用访问，但借用的生命周期限制了使用模式
- `Arc` 允许多个所有者通过引用计数共享同一份堆上数据，无需复制

如果某个渲染节点需要修改纹理（应用滤镜）：需要 `Arc<Mutex<Vec<u8>>>`（线程安全）或 `Rc<RefCell<Vec<u8>>>`（单线程）。也可以使用 Copy-on-Write 模式：`Arc::make_mut` 在有多个引用时自动克隆数据再提供可变引用。

**4. 核心问题：智能指针作为库类型而非语言内置**

**a) 类型系统的灵活性**
Rust 将 `Box`、`Rc`、`RefCell` 作为 `trait` 实现（通过 `Deref`、`Drop`、`Clone` 等），意味着任何第三方库都可以通过实现相同 trait 来创建自己的智能指针类型（如 `Cow`、`ArcSwap`、`triomphe::Arc`）。这种开放性使生态可以针对特定场景优化（如不进行引用计数的 `Arc` 变体），而不需要语言委员会批准新关键字。

**b) 运行时开销的可选择性**
C++ 的 `std::shared_ptr` 总是使用原子引用计数，即使在单线程代码中。Rust 的 `Rc`（非原子）和 `Arc`（原子）分离，让你根据是否需要线程安全选择开销。这种细粒度控制在语言内置特性中很难实现。

**c) 与其他语言特性的集成**
Rust 的智能指针通过标准 trait（`Deref`、`DerefMut`、`Drop`）与语言的解引用运算符 `*`、自动解引用、作用域规则无缝集成。不需要特殊的语言语法——`*rc` 和 `*box` 用同一个语法实现不同的 trait 实现。

**d) 学习曲线**
作为库类型，每个智能指针有自己独立的文档、方法和约束。这比语言内置更难入门（需要知道何时用哪个），但提供了更清晰的概念分离：初学者从 `Box` 开始，逐步接触 `Rc`、`RefCell`、`Arc`、`Mutex`，每个都是解决特定问题的工具。

---

## 迁移思维练习答案

### 1. C++ unique_ptr 和 Rust Box<T> 的异同？

相似之处：两者都表示堆上分配的独占所有权，离开作用域时自动释放内存。关键差异：Rust 的 Box 在 Move 后，原绑定立即失效——编译器禁止继续使用。C++ 的 unique_ptr 在 std::move 后，原指针变为 nullptr，仍然是可用的对象（只是值为空），对空指针的解引用是运行时 UB 而非编译错误。此外，C++ unique_ptr 支持自定义 deleter（如 `unique_ptr<FILE, decltype(&fclose)>`），Rust 的 Box 不支持自定义 deleter，但可以通过 Drop trait 在新类型包装中实现等效效果。

### 2. 哪些共享关系需要 Rc<T>，哪些需要 Arc<T>？

仅在单线程上下文中共享所有权时使用 Rc<T>（如 GUI 中多个 widget 引用同一数据模型）。当数据可能被多个线程同时持有时必须使用 Arc<T>（如 web 服务器的共享配置、线程池中的共享状态）。关键判断标准是：数据的持有者是否会跨越线程边界？如果不确定，从 Rc 开始——编译器会在你试图跨线程传递 Rc 时报错（因为 Rc 没有实现 Send/Sync），提醒你改为 Arc。这个编译错误本身就是一种安全保障。

### 3. Rc<RefCell<T>> 虽然方便，为什么不应该滥用？

它绕过了编译期的借用检查，将规则验证推迟到运行时——违反借用规则（如同时两个可变借用）会导致 panic 而不是编译错误，在生产环境中这可能是致命的。它使所有权和可变性关系变得不透明：数据可以被不知不觉地多处修改、多处读取，增加了理解和调试的难度。应优先考虑更简单的设计：单一所有权 + 借用、重组数据结构将可变部分隔离、或使用 Enum 表达状态变化。Rc<RefCell<T>> 是解决特定问题的工具，不是组织整个程序的默认模式。
