## DesignPattern Workspace 使用说明

本目录是 Rust 设计模式示例代码的 Cargo Workspace。

### 与设计模式章节的关系

`src/Rust学习/设计模式/` 中的每个设计模式章节，对应本目录下一个可独立运行的 Cargo package。章节负责解释概念、适用场景和 Rust 实现思路；`example_code/DesignPattern/` 负责提供能运行的小项目演示，用代码验证章节中的模式角色和使用场景。

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
| 外观模式（Facade Pattern） | `facade_pattern` | `cargo run -p facade_pattern` | 文件处理外观：读取→压缩→加密→写入 的统一入口 |
| 组合模式（Composite Pattern） | `composite_pattern` | `cargo run -p composite_pattern` | 目录树结构：统一处理文件（叶子）与目录（组合） |
| 桥接模式（Bridge Pattern） | `bridge_pattern` | `cargo run -p bridge_pattern` | 消息类型 × 发送方式：拆分两个维度，避免组合爆炸 |
| 享元模式（Flyweight Pattern） | `flyweight_pattern` | `cargo run -p flyweight_pattern` | 树类型共享：Arc + HashMap 缓存池复用内部状态 |
| 命令模式（Command Pattern） | `command_pattern` | `cargo run -p command_pattern` | 文本编辑器操作封装成命令：可排队执行并支持 undo/redo |
| 观察者模式（Observer Pattern） | `observer_pattern` | `cargo run -p observer_pattern` | 温度传感器状态变化后自动通知显示器、日志器、告警器 |
| 责任链模式（Chain of Responsibility Pattern） | `chain_of_responsibility_pattern` | `cargo run -p chain_of_responsibility_pattern` | Web 请求中间件链：认证、缓存、路由依次判断处理 |
| 中介者模式（Mediator Pattern） | `mediator_pattern` | `cargo run -p mediator_pattern` | GUI 控件通过 `UiMediator` 协调交互，避免控件互相直接依赖 |
| 状态模式（State Pattern） | `state_pattern` | `cargo run -p state_pattern` | 订单状态机：`Created`、`Paid`、`Shipped`、`Finished`、`Cancelled` 的合法流转 |
| 备忘录模式（Memento Pattern） | `memento_pattern` | `cargo run -p memento_pattern` | 文本编辑器保存状态快照，并通过历史栈支持 undo/redo |
| 解释器模式（Interpreter Pattern） | `interpreter_pattern` | `cargo run -p interpreter_pattern` | `enum + match + Box` 构造表达式树并递归求值 |
| 访问者模式（Visitor Pattern） | `visitor_pattern` | `cargo run -p visitor_pattern` | 表达式树保持稳定，统计、打印、求值由不同 Visitor 执行 |

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
