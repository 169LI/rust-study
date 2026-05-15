# 04\_单例模式 Singleton Pattern

## 1. 基本信息

**中文名称：** 单例模式

**英文名称：** Singleton Pattern

**模式类型：** 创建型设计模式

**Rust 中常见实现方式：** `OnceLock`、`LazyLock`、`static` 结合 `Mutex` 或 `RwLock`、`Arc`

单例模式的核心是：**保证某个类型在整个程序中只有一个实例，并提供全局访问点。**
在 Rust 中，单例模式通常结合静态初始化和线程安全机制实现，避免全局可变状态带来的安全问题。

***

## 2. 模式核心思想

单例模式的核心思想是：

> **一个类型只有一个实例，全局唯一，通过统一访问接口获取，并确保初始化和访问的线程安全性。**

它关注两个核心问题：

```text
如何保证只有一个实例被创建
如何提供全局可访问的接口
```

***

## 3. 这个模式解决什么问题

在实际开发中，可能有一些资源或管理对象需要全局唯一，例如：

```text
配置管理器
日志系统
缓存管理器
数据库连接池管理
系统唯一 ID 生成器
```

如果没有单例模式：

- 多处代码可能创建多个实例，导致状态不一致
- 配置或缓存可能无法统一管理
- 对共享资源的竞争和安全问题增加

单例模式提供了一个统一访问入口，并确保实例唯一。

***

## 4. 不使用这个模式会怎样

如果不使用单例模式，可能出现：

```text
多个实例导致全局状态混乱
配置或缓存对象状态不一致
初始化顺序不受控
线程安全问题增加
调试和维护成本上升
```

***

## 5. 传统面向对象中的实现思路

在 Java/C++ 中，单例模式通常通过以下方式实现：

```text
私有构造函数，禁止外部 new
提供静态方法 getInstance() 返回唯一实例
懒加载或饿汉式初始化
线程安全措施（双重检查锁、同步关键字、静态初始化）
```

结构图示：

```text
Singleton
 ├─ 私有构造函数
 └─ 静态实例 + getInstance()
```

***

## 6. Rust 中的实现思路

### 6.1 Rust 中通常怎么实现

在 Rust 中，可以使用标准库提供的静态初始化类型：

```text
OnceLock<T>      // 一次性初始化，线程安全
LazyLock<T>      // 惰性初始化，线程安全
static + Mutex<T> // 手动管理全局可变实例
```

示例（使用 OnceLock）：

```rust
use std::sync::OnceLock;

// 定义一个单例类型
struct Config {
    field: String,
}

impl Config {
    // 提供实例方法
    fn print(&self) {
        println!("Config field: {}", self.field);
    }

    fn update(&mut self, value: &str) {
        self.field = value.to_string();
    }
}

// 创建全局唯一实例
static CONFIG: OnceLock<Config> = OnceLock::new();

// 获取单例实例的全局访问函数
fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        field: "default".to_string(),
    })
}

fn main() {
    // 获取单例实例并调用方法
    let cfg = get_config();
    cfg.print(); // 输出: Config field: default

    // 如果需要修改，可以把单例放在 Mutex 中（线程安全可变）
    use std::sync::Mutex;

    static MUTABLE_CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

    fn get_mutable_config() -> &'static Mutex<Config> {
        MUTABLE_CONFIG.get_or_init(|| Mutex::new(Config {
            field: "default".to_string(),
        }))
    }

    // 多线程使用
    use std::thread;

    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let cfg_lock = get_mutable_config();
                let mut cfg = cfg_lock.lock().unwrap();
                cfg.update(&format!("value {}", i));
                println!("Thread {} updated config: {}", i, cfg.field);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // 最终访问单例实例
    let cfg_lock = get_mutable_config();
    let cfg = cfg_lock.lock().unwrap();
    println!("Final config field: {}", cfg.field);
}
```

说明：

- 创建：通过 `OnceLock::get_or_init()` 初始化单例，只执行一次。
- 访问：通过全局函数 `get_config()` 获取实例。
- 方法调用：直接调用实例方法，如 ` print()` 或 `update()`。
- 可变操作：如果需要修改单例对象，放入 `Mutex` 中以保证线程安全。
- 多线程安全：多线程访问时，每个线程通过锁获得可变访问权。

***

### 6.2 和传统 OOP 写法相比有什么不同

1. 不需要私有构造函数，Rust 通过模块私有性控制外部访问。
2. 不依赖类静态方法，使用 `static` 结合 OnceLock/LazyLock 即可。
3. 线程安全由标准库封装，无需手动加锁或双重检查。
4. 没有继承体系，单例实例通常是结构体或引用类型。

***

### 6.3 Rust 中是否有更自然的替代写法

- 对只读全局数据，可以使用 `const` 或 `static`。
- 对可变全局状态，可以结合 `Arc<Mutex<T>>` 或 `LazyLock` 管理。
- 如果全局状态复杂，可以考虑通过传递引用或依赖注入代替单例。

***

## 7. Rust 中涉及的语言特性

```text
static
OnceLock / LazyLock
Arc / Mutex / RwLock
模块私有性
所有权和借用
函数返回 'static 引用
```

这些特性解决了：

- 全局实例唯一性
- 线程安全
- 可共享的全局访问

***

## 8. 性能与工程代价

- 初始化只执行一次，惰性加载可避免启动开销
- 使用 `OnceLock/LazyLock` 的读操作几乎没有额外开销
- 如果使用 `Arc<Mutex<T>>`，会有锁开销和堆分配开销
- 全局可变状态增加了维护复杂度
- Rust 显式管理所有权，避免了传统 OOP 的隐式同步问题

***

## 9. 典型应用场景

```text
配置管理器
日志系统
缓存管理器
数据库连接池
唯一标识生成器
```

在 Rust 中，这类模式通常用于全局只读或线程安全的共享资源管理。

***

## 10. 和相似模式的区别

- 原型模式：关注如何快速创建新对象，通过复制已有对象生成多个相似实例。强调对象创建的效率和模板复用。

与单例不同：原型模式允许创建任意数量的对象，而单例模式只允许一个实例。

- 工厂模式：关注如何根据条件或类型动态创建对象。强调封装创建逻辑、实现解耦。

与单例不同：工厂模式可以生成多个对象，每次调用可能返回不同类型或不同实例，而单例模式保证全局唯一实例。

- 构建者模式：关注如何分步骤构造复杂对象。强调对象的分阶段配置和可读性。

与单例不同：构建者模式主要解决复杂对象初始化过程，而单例模式关注“实例唯一性”，可以用构建者模式创建唯一实例。

- 享元模式：关注大量相似对象的共享，降低内存占用。强调对象状态共享和资源复用。

与单例不同：享元模式允许多个对象共享内在状态，并不限制实例数量；单例模式只允许一个全局唯一实例，无论状态是否共享。

***

## 11. 使用该模式的优点

```text
保证全局唯一实例
提供统一访问点
管理全局共享资源
线程安全由标准库封装
初始化逻辑集中
```

***

## 12. 使用该模式的代价

```text
全局状态可能增加耦合
过度依赖单例会影响测试和复用
可变单例需要关注锁竞争
可能隐藏对象所有权边界
```

***

## 13. 什么时候不应该使用

```text
对象不需要全局唯一
可通过传递引用或依赖注入实现共享
测试或复用需要独立实例
全局可变状态可能带来线程竞争
```

***

## 14. 一个简单例子思路

**场景：** 日志系统

- 日志系统在程序中全局唯一
- 配置只初始化一次
- 多线程同时记录日志

**处理流程：**

```text
使用 static + OnceLock 创建日志实例
提供 get_instance() 获取全局引用
多线程调用记录日志方法
确保初始化只执行一次
```

适合练习 Rust 特性：`OnceLock`, `LazyLock`, `Arc<Mutex<T>>`, 模块私有性, 'static 引用

***

## 15. 总结一句话

单例模式的本质是：

> **确保全局只有一个实例，并提供统一、线程安全的访问接口。在 Rust 中，通过 OnceLock、LazyLock 或 Arc/Mutex 结合静态变量即可高效实现。**

