## DesignPattern Workspace 使用说明

本目录是 Rust 设计模式示例代码的 Cargo Workspace。

### 运行方式

在本目录下执行：

```powershell
cargo run -p <package_name> -- <args>
```

其中 `<package_name>` 对应一个设计模式示例项目。

### 示例项目与命令对照表

| 设计模式 | 包名（package） | 运行命令 | 说明 |
| --- | --- | --- | --- |
| 迭代器模式（Iterator Pattern） | `iterator_pattern` | `cargo run -p iterator_pattern` | 真实文件扫描器：自定义迭代器封装目录遍历逻辑 |
| 过滤器模式（Filter Pattern） | `filter_pattern` | `cargo run -p filter_pattern` | 用户列表过滤：`filter()` 链式组合条件 |
| 原型模式（Prototype Pattern） | `prototype_pattern` | `cargo run -p prototype_pattern` | `Clone` 复制原型配置并局部修改（含 `Arc` 共享对比） |
| 单例模式（Singleton Pattern） | `singleton_pattern` | `cargo run -p singleton_pattern` | `OnceLock`（只读单例）+ `OnceLock<Mutex<_>>`（可变单例） |
| 构建者模式（Builder Pattern） | `builder_pattern` | `cargo run -p builder_pattern` | `RequestBuilder` 方法链构建复杂请求对象 |
| 策略模式（Strategy Pattern） | `strategy_pattern` | `cargo run -p strategy_pattern -- --algo rle` | 文件压缩策略：`--algo none\|rle\|xor` 运行时切换策略 |
| 工厂模式（Factory Pattern） | `factory_pattern` | `cargo run -p factory_pattern -- --db mysql` | 数据库客户端工厂：`--db mysql\|postgres\|sqlite` 选择实现 |
| 适配器模式（Adapter Pattern） | `adapter_pattern` | `cargo run -p adapter_pattern` | 统一日志接口：包装适配 + From/Into 类型转换 |
| 模板方法模式（Template Method Pattern） | `template_method_pattern` | `cargo run -p template_method_pattern` | 固定流程（读取→解析→处理→写入），CSV/JSON 仅定制步骤 |
| 代理模式（Proxy Pattern） | `proxy_pattern` | `cargo run -p proxy_pattern -- --user alice --role admin` | 打印服务代理：权限控制（保护代理）+ 延迟初始化（虚拟代理） |
| 装饰器模式（Decorator Pattern） | `decorator_pattern` | `cargo run -p decorator_pattern` | 消息发送器装饰链：日志/大写/计时等功能可叠加 |

### 参数示例

```powershell
# 策略模式：选择压缩策略
cargo run -p strategy_pattern -- --algo rle
cargo run -p strategy_pattern -- --algo xor

# 工厂模式：选择数据库类型
cargo run -p factory_pattern -- --db mysql
cargo run -p factory_pattern -- --db postgres
cargo run -p factory_pattern -- --db sqlite

# 代理模式：选择用户与角色
cargo run -p proxy_pattern -- --user alice --role admin
cargo run -p proxy_pattern -- --user bob --role guest
```
