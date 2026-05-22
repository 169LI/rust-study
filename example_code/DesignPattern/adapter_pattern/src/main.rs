//! 适配器模式 (Adapter Pattern) 示例：统一日志接口
//!
//! 这个模块做什么：
//! - 将“老接口/第三方接口”适配成调用方期望的统一接口。
//! - 演示两种常见适配方式：
//!   - 包装结构体 + trait（对象适配器）
//!   - `From/Into`（类型转换适配）
//!
//! 场景说明：
//! - 业务侧希望统一调用 `Logger::log(level, msg)`。
//! - 但已有的老日志组件只提供 `OldLogger::write_line(&str)`，接口不兼容。
//! - 通过 `OldLoggerAdapter` 包装老组件并实现 `Logger`，做到“不修改老代码也能接入新接口”。

use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        };
        write!(f, "{s}")
    }
}

/// 调用方期望的目标接口（Target）。
trait Logger {
    /// 记录一条日志。
    fn log(&self, level: Level, message: &str);
}

/// 老接口（Adaptee）：无法修改其实现（假设来自遗留代码或第三方库）。
struct OldLogger;

impl OldLogger {
    /// 老接口只支持写入“一行字符串”，没有级别参数。
    fn write_line(&self, line: &str) {
        println!("[old] {line}");
    }
}

/// 适配器（Adapter）：包装老接口并实现目标接口。
struct OldLoggerAdapter {
    inner: OldLogger,
}

impl OldLoggerAdapter {
    /// 创建适配器实例。
    fn new(inner: OldLogger) -> Self {
        Self { inner }
    }
}

impl Logger for OldLoggerAdapter {
    fn log(&self, level: Level, message: &str) {
        let line = format!("{level}: {message}");
        self.inner.write_line(&line);
    }
}

/// 新实现：直接实现目标接口（用于对比）。
struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, level: Level, message: &str) {
        println!("[new] {level}: {message}");
    }
}

/// 业务函数：只依赖目标接口（Target），不关心具体是新实现还是适配器。
fn run_business(logger: &dyn Logger) {
    logger.log(Level::Debug, "debug trace");
    logger.log(Level::Info, "start process");
    logger.log(Level::Warn, "cache miss");
    logger.log(Level::Error, "request failed");
}

/// 下面演示 From/Into 的“类型转换适配”：华氏度 -> 摄氏度。
#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    let modern = StdoutLogger;
    run_business(&modern);

    println!();

    let legacy = OldLogger;
    let adapter = OldLoggerAdapter::new(legacy);
    run_business(&adapter);

    println!();

    let f = Fahrenheit(212.0);
    let c: Celsius = f.into();
    println!("temperature: {f:?} -> {c:?} (value={})", c.0);
}
