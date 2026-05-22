//! 装饰器模式 (Decorator Pattern) 示例：可叠加的消息发送器
//!
//! 这个模块做什么：
//! - 使用“包装对象 + trait”的方式，在不修改原对象的前提下叠加功能。
//! - 演示装饰器可以多层组合：日志、计时、文本变换等功能可以按需叠加。
//!
//! 场景说明：
//! - 业务侧只依赖 `Messenger` 接口发送消息。
//! - `ConsoleMessenger` 是基础实现（ConcreteComponent）。
//! - `LoggingDecorator`、`TimingDecorator`、`UppercaseDecorator` 是装饰器（Decorator），可多层包裹。

use std::time::Instant;

/// 抽象组件（Component）：统一接口。
trait Messenger {
    /// 发送消息。
    fn send(&self, message: &str) -> Result<(), String>;
}

/// 具体组件（ConcreteComponent）：最基础的实现。
struct ConsoleMessenger;

impl Messenger for ConsoleMessenger {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[console] {message}");
        Ok(())
    }
}

/// 装饰器 1：打印发送前后日志。
struct LoggingDecorator {
    inner: Box<dyn Messenger>,
}

impl LoggingDecorator {
    /// 创建装饰器。
    fn new(inner: Box<dyn Messenger>) -> Self {
        Self { inner }
    }
}

impl Messenger for LoggingDecorator {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[log] before send");
        let result = self.inner.send(message);
        println!("[log] after send: ok={}", result.is_ok());
        result
    }
}

/// 装饰器 2：统计一次发送耗时。
struct TimingDecorator {
    inner: Box<dyn Messenger>,
}

impl TimingDecorator {
    /// 创建装饰器。
    fn new(inner: Box<dyn Messenger>) -> Self {
        Self { inner }
    }
}

impl Messenger for TimingDecorator {
    fn send(&self, message: &str) -> Result<(), String> {
        let start = Instant::now();
        let result = self.inner.send(message);
        let elapsed = start.elapsed();
        println!("[timing] elapsed = {:?}", elapsed);
        result
    }
}

/// 装饰器 3：对消息进行转换（全部大写）。
struct UppercaseDecorator {
    inner: Box<dyn Messenger>,
}

impl UppercaseDecorator {
    /// 创建装饰器。
    fn new(inner: Box<dyn Messenger>) -> Self {
        Self { inner }
    }
}

impl Messenger for UppercaseDecorator {
    fn send(&self, message: &str) -> Result<(), String> {
        let transformed = message.to_uppercase();
        self.inner.send(&transformed)
    }
}

fn main() -> Result<(), String> {
    let base: Box<dyn Messenger> = Box::new(ConsoleMessenger);

    println!("--- 1. 不使用装饰器（基础实现） ---");
    base.send("hello decorator")?;

    println!();
    println!("--- 2. 叠加装饰器（日志 + 大写 + 计时） ---");

    let decorated: Box<dyn Messenger> = Box::new(LoggingDecorator::new(base));
    let decorated: Box<dyn Messenger> = Box::new(UppercaseDecorator::new(decorated));
    let decorated: Box<dyn Messenger> = Box::new(TimingDecorator::new(decorated));

    decorated.send("hello decorator")?;
    Ok(())
}
