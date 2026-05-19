## Rust 项目注释与命名规范（约定）

### 1. 文件/模块级别注释（必需）

- 每个 Rust 源文件（`main.rs` / `lib.rs` / 各 `mod.rs` 或模块文件）文件顶部必须包含模块文档注释：使用 `//!`。
- 模块文档注释应回答三个问题：
  - 这个模块做什么
  - 场景说明

### 2. 函数/类型级别注释（建议）

- 对外公开的 API（`pub`）必须使用 `///` 添加文档注释（Rustdoc）。
- 非 `pub` 的函数：
  - 逻辑复杂、边界条件多、或有容易误用的约束时建议加 `///` 或行内说明。
  - “显而易见”的代码不要堆注释，优先通过命名表达意图。

文档注释建议包含：

- **Purpose**：做什么
- **Parameters**：参数含义（必要时）
- **Returns**：返回值语义
- **Errors**：失败条件（尤其是 `Result`）
- **Safety**：`unsafe` 的不变量与前置条件（如存在）

示例：

```rust
/// 根据用户输入构建过滤条件。
///
/// # Errors
/// - 当输入包含不支持的字段名时返回 `FilterError::UnknownField`。
fn build_filter(expr: &str) -> Result<Filter, FilterError> {
    // ...
}
```

### 3. Rust 风格命名规则（必需）

- 模块/文件：`snake_case`（如 `file_scanner.rs`）
- 变量/函数：`snake_case`（如 `scan_dir`, `file_count`）
- 类型（struct/enum/trait）：`UpperCamelCase`（如 `FileScanner`, `FilterPredicate`）
- 枚举变体：`UpperCamelCase`（如 `LogLevel::Warn`）
- 常量：`SCREAMING_SNAKE_CASE`（如 `DEFAULT_TIMEOUT_MS`）
- 泛型：短且语义清晰（如 `T`, `K`, `V`；必要时用 `Item`, `Error`）
- 生命周期：简短（如 `'a`, `'b`），避免无意义的长名字
- 错误类型：以 `Error` 结尾（如 `ParseError`），结果类型优先 `Result<T, XxxError>`

### 4. 代码风格与工具（建议）

- 保持 `rustfmt` 默认格式；不要手工对齐空格。
- 通过 `clippy` 的常见建议（命名、迭代器、所有权）来提升可读性。
- 注释语言以中文为主，术语可保留英文括注（如 “惰性求值 (Lazy Evaluation)”）。
