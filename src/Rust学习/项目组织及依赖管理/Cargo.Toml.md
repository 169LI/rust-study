# Cargo.Toml

涵盖常用字段、依赖声明、版本规则、features、workspace 和结构化书写方法，便于规范化项目管理。

本节是大致的介绍，详细内容可参考[Cargo.toml](https://mp.weixin.qq.com/s/HdiBqU56QukVfXBMYUPCkg)

---

## 1️⃣ 基本结构

Cargo.toml 是 TOML 格式的文件，主要分为几个区块：

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A short description of the project"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/my_project"

[dependencies]
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
criterion = "0.4"

[build-dependencies]
cc = "1.0"

[features]
default = []
json_support = ["serde"]

```

---

## 2️⃣ [package] 区块规范

| 字段        | 说明         | 建议                                         |
| ----------- | ------------ | -------------------------------------------- |
| name        | crate 名称   | 用小写字母 + 下划线，避免特殊字符            |
| version     | 版本号       | 遵循语义化版本号 SemVer（MAJOR.MINOR.PATCH） |
| edition     | Rust edition | `2015`/`2018`/`2021`，统一项目         |
| authors     | 作者信息     | 方便生成文档、Cargo 发布                     |
| description | 项目描述     | 简短，描述功能                               |
| license     | 许可证       | MIT / Apache-2.0 / 自定义，格式规范          |
| repository  | 仓库地址     | GitHub 或其他托管地址                        |

---

## 3️⃣ [dependencies] 区块规范

### 3.1 基本依赖

```toml
serde = "1.0"   # caret 版本 ^1.0
regex = "1.5.4" # 精确版本
```

* 默认使用 `^` 号（caret）规则
* 避免使用 `*`（无限制版本）

### 3.2 可选依赖

```toml
serde_json = { version = "1.0", optional = true }
```

* optional = true 表示依赖不默认启用
* 必须配合 features 使用

### 3.3 Git / Path 依赖

```toml
my_crate = { git = "https://github.com/user/my_crate.git", branch = "main" }
my_local = { path = "../my_local" }
```

---

## 4️⃣ [dev-dependencies] 和 [build-dependencies]

* dev-dependencies：只在测试或示例编译
* build-dependencies：只在 build.rs 脚本编译

```toml
[dev-dependencies]
criterion = "0.4"

[build-dependencies]
cc = "1.0"
```

---

## 5️⃣ [features] 区块

* 声明可选功能或可选依赖
* default：默认启用的 feature

```toml
[features]
default = []
json_support = ["serde_json"]
```

* 编译时启用：

```bash
cargo build --features json_support
```

---


---

## 6️⃣ 版本管理规范

1、**SemVer** ：`MAJOR.MINOR.PATCH`
2、 **依赖版本约束** ：

| 写法           | 含义                   |
| -------------- | ---------------------- |
| `"1.0"`      | ^1.0 → >=1.0.0 <2.0.0 |
| `"~1.0.115"` | >=1.0.115 <1.1.0       |
| `"=1.0.115"` | 仅 1.0.115             |
| `"1.*"`      | >=1.0.0 <2.0.0         |

3、注意：避免使用 `"*"`，防止不确定更新

---

## 7️⃣ 书写规范建议

1、**字段顺序** （推荐）：

```text
[package]
[dependencies]
[dev-dependencies]
[build-dependencies]
[features]
[workspace]
```

2、**依赖统一按内外部依赖、和首字母排序** ，方便查找

3、**路径依赖、Git 依赖写明版本或分支**

4、**注释清晰** ，说明特殊依赖或 feature 用途

```toml
# serde_json 用于 JSON 解析，可选依赖
serde_json = { version = "1.0", optional = true }
```

5、**features 命名简洁、清晰** ，避免与依赖 crate 内 feature 冲突

---

## 8️⃣ Cargo.toml 示例（综合规范）

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Alice <alice@example.com>"]
description = "示例 Rust 项目"
license = "MIT"
repository = "https://github.com/alice/my_project"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", optional = true }
regex = "1.5"

[dev-dependencies]
criterion = "0.4"

[build-dependencies]
cc = "1.0"

[features]
default = []
json_support = ["serde_json"]

[workspace]
members = ["crate_a", "crate_b"]
```

---

✅ **总结**

* `[package]` → 项目信息
* `[dependencies]` → 运行时依赖
* `[dev-dependencies]` → 测试/开发依赖
* `[build-dependencies]` → build.rs 脚本依赖
* `[features]` → 可选功能/可选依赖
* `[workspace]` → 多 crate 项目管理
* **版本使用 SemVer，避免使用 "*"**

---

