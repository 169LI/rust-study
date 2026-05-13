# Option & Result

---

## 1. 概述

Rust 的 `Option` 和 `Result` 是 **安全处理可能缺失或失败的值** 的核心类型：

* `Option<T>`：值可能存在 (`Some`) 或不存在 (`None`)。
* `Result<T, E>`：操作可能成功 (`Ok`) 或失败 (`Err`)。

特点：

* 避免 null 引发的空指针错误。
* 结合 `match`、链式方法和 `?` 操作符可以优雅处理错误。
* 用于函数返回值、文件/网络操作、算法查找等场景。

---

## 2. 基本定义

### Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

* `Some(T)`：包含值 T
* `None`：没有值

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* `Ok(T)`：操作成功返回值 T
* `Err(E)`：操作失败返回错误 E

---

## 3. 创建与初始化

### Option 示例

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

### Result 示例

```rust
let r1: Result<i32, &str> = Ok(10);
let r2: Result<i32, &str> = Err("something went wrong");
```

---

## 4. 基本匹配与解构

### 使用 `match`

```rust
let maybe_value = Some(42);

match maybe_value {
    Some(v) => println!("Value is {}", v),
    None => println!("No value"),
}
```

### 使用 `if let` 简化匹配

```rust
if let Some(v) = maybe_value {
    println!("Value is {}", v);
}
```

### Result 匹配

```rust
let result: Result<i32, &str> = divide(10, 2);

match result {
    Ok(v) => println!("Result = {}", v),
    Err(e) => println!("Error = {}", e),
}
```

---

## 5. 常用方法

### 5.1 Option 方法

| 方法                   | 功能                 | 示例                            |
| ---------------------- | -------------------- | ------------------------------- |
| `is_some()`          | 是否为 `Some`      | `x.is_some()`                 |
| `is_none()`          | 是否为 `None`      | `x.is_none()`                 |
| `unwrap()`           | 获取值或 panic       | `x.unwrap()`                  |
| `unwrap_or(default)` | `None`时返回默认值 | `x.unwrap_or(0)`              |
| `unwrap_or_else(f)`  | `None`时调用闭包   | `x.unwrap_or_else(\|\| 0)`      |
| `map(f)`             | 映射值               | `x.map(\|v\| v * 2)`            |
| `and_then(f)`        | 链式调用             | `x.and_then(\|v\| Some(v * 2))` |
| `ok_or(err)`         | 转为 `Result`      | `x.ok_or("not found")`        |
| `as_ref()`           | 转为引用             | `x.as_ref()`                  |

示例：

```rust
let x = Some(5);
let y = x.map(|v| v * 2); // Some(10)
let z = None::<i32>.unwrap_or(0); // 0
```

---

### 5.2 Result 方法

| 方法                   | 功能                        | 示例                 |
| ---------------------- | --------------------------- | -------------------- |
| `is_ok()`            | 是否为 Ok                   | `r.is_ok()`        |
| `is_err()`           | 是否为 Err                  | `r.is_err()`       |
| `unwrap()`           | 获取 Ok 或 panic            | `r.unwrap()`       |
| `unwrap_err()`       | 获取 Err 或 panic           | `r.unwrap_err()`   |
| `unwrap_or(default)` | Err 返回默认值              | `r.unwrap_or(0)`   |
| `unwrap_or_else(f)`  | Err 调用闭包                | `r.unwrap_or_else` |
| `map(f)`             | 映射 Ok 值                  | `r.map(              |
| `map_err(f)`         | 映射 Err 值                 | `r.map_err(          |
| `and_then(f)`        | 链式调用                    | `r.and_then(         |
| `or_else(f)`         | Err 时调用闭包返回新 Result | `r.or_else(          |

示例：

```rust
let r: Result<i32, &str> = Ok(10);
let r2 = r.map(|v| v * 2); // Ok(20)
let err: Result<i32, &str> = Err("fail");
let r3 = err.unwrap_or(0); // 0
```

---

## 6. 链式调用与组合操作

### Option 链式操作

```rust
let x = Some(2)
    .map(|v| v * 3)       // Some(6)
    .and_then(|v| Some(v + 1)); // Some(7)
```

### Result 链式操作

```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

let result = parse_int("42")
    .map(|v| v * 2)      // Ok(84)
    .and_then(|v| Ok(v + 1)); // Ok(85)
```

---

## 7. `?` 操作符（快速返回）

* 用于函数返回 `Option` 或 `Result`。
* 遇到 `None` 或 `Err` 会立即返回。
* 简化嵌套 match 和错误处理。

### 在 Result 中使用

```rust
fn read_file_len(path: &str) -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // Err 会直接返回
    Ok(content.len())
}
```

### 在 Option 中使用

```rust
fn first_char(s: &str) -> Option<char> {
    let c = s.chars().next()?; // None 会直接返回
    Some(c.to_ascii_uppercase())
}
```

---

## 8. Option 与 Result 转换

* `Option<T>` → `Result<T, E>`：`ok_or(err)` 或 `ok_or_else(|| err)`
* `Result<T, E>` → `Option<T>`：`ok()`

示例：

```rust
let x: Option<i32> = Some(5);
let r: Result<i32, &str> = x.ok_or("Value not found"); // Ok(5)

let r2: Result<i32, &str> = Err("fail");
let o: Option<i32> = r2.ok(); // None
```

---

## 9. 模块化与公共 API 设计

* 函数返回 `Option`：值可能缺失，例如查找、解析失败。
* 函数返回 `Result`：操作可能失败，例如文件读取、网络请求。
* 避免在公共 API 中使用 `panic!` 或 `unwrap()`，而应使用 `Result` 或 `Option` 明确返回错误。
* 链式操作和 `?` 可以让错误处理更简洁、安全。
