# 项目参考实现说明 — CLI 文本搜索工具

## 使用说明

本项目答案提供设计决策分析和关键代码模式，不是完整的复制粘贴源码。建议先独立实现全部功能，再对照本文件分析差异。

---

## 1. 需求拆分

1. 接收命令行参数（查询词 + 文件路径）
2. 读取文件内容
3. 按行搜索匹配（大小写敏感/不敏感可选）
4. 输出带行号的匹配结果
5. 正确传播 I/O 错误和参数错误

## 2. 推荐实现顺序

1. `lib.rs`: 定义 `SearchConfig` 结构体 + `new()` 构造器
2. `lib.rs`: 实现 `search()` 和 `search_case_insensitive()`
3. `lib.rs`: 实现 `run()` 编排函数
4. `main.rs`: 调用 `run()`
5. 测试: 单元测试搜索逻辑 + 集成测试 `run()`

## 3. 模块划分

```
src/
├── main.rs    # 参数解析 + 调用 run()
└── lib.rs     # 所有业务逻辑: SearchConfig, search, run
```

**为什么 `lib.rs` 是关键**：
- `lib.rs` 的函数是对外公共 API，测试可以直接 `use cli_text_search::search`
- `main.rs` 只做薄薄的参数层，可以被集成测试绕过

## 4. 核心数据结构

```rust
pub struct SearchConfig {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,  // 从环境变量 CASE_INSENSITIVE 读取
}
```

## 5. 关键函数签名

```rust
pub fn new(args: &[String]) -> Result<SearchConfig, &'static str>
pub fn run(config: SearchConfig) -> Result<(), Box<dyn Error>>
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str>
```

## 6. 设计决策

### 为什么 `search` 返回 `Vec<&str>` 而非 `Vec<String>`？

返回引用零拷贝，利用借用从 `contents`（文件内容）切片。这要求 `contents` 的存活时间覆盖所有返回值——这由生命周期 `<'a>` 保证。

### 为什么 `config` 用 `String` 而非 `&str`？

`new()` 解析 `env::args()` 后获得拥有所有权的 `String`，将其移入 `SearchConfig` 字段避免了 `new()` 返回后临时 `String` 被释放的问题。

### 环境变量控制大小写敏感

`CASE_INSENSITIVE=1` 设置后 `search` 自动忽略大小写。实现了"运行时配置 + 零额外参数"。

## 7. 关键代码片段

```rust
// 大小写不敏感搜索的核心：预转换为小写比较
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

## 8. 测试策略

- **单元测试**：测试 `search` 和 `search_case_insensitive` 的独立行为
- **边界条件**：空查询、空文件、查询不匹配、多行匹配
- **集成测试**：创建临时文件 → 调用 `run()` → 验证输出
- **大小写**：分别测试 `CASE_INSENSITIVE` 开启和关闭的情况

## 9. 常见失败方式

| 错误 | 原因 | 修复 |
|------|------|------|
| `args` 不足取 `args[2]` 时 panic | 直接索引越界 | 用 `args.get(2)` 或检查 `args.len()` |
| 返回的 `&str` 生命周期不足 | `contents` 在某局部被释放 | 确保 `contents` 的生命周期覆盖搜索结果 |
| `search` 返回了 `search_case_insensitive` 的结果 | 混用两种搜索 | 根据 `config.case_sensitive` 分支 |

## 10. 可选扩展

- 正则表达式搜索（`regex` crate）
- 递归目录搜索
- 彩色高亮输出匹配文本
- 统计匹配行数和总行数
- 支持从 stdin 读取（无需文件参数）

---

*本项目是 Rust 经典的"mini-grep"设计。重点学习 lib.rs/main.rs 拆分、&str 生命周期设计和测试驱动开发。*
