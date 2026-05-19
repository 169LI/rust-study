//! 单例模式 (Singleton Pattern) 示例：OnceLock + Mutex
//!
//! 这个模块做什么：
//! - 演示如何用 `OnceLock<T>` 实现“全局唯一、只初始化一次”的只读单例。
//! - 演示如何用 `OnceLock<Mutex<T>>` 实现“全局唯一、线程安全可变”的单例。
//!
//! 场景说明：
//! - 使用 `AppConfig` 作为只读配置单例（全局共享、初始化一次）。
//! - 使用 `Logger` 作为可变日志单例（多线程写入，需要 `Mutex` 保护）。
//!
//! AppConfig 与 Logger 的区别（在本项目中的作用）：
//! - `AppConfig`：只读单例。初始化后不再修改，因此用 `OnceLock<AppConfig>` 直接提供全局共享的只读引用。
//!   重点演示 “OnceLock 只初始化一次 + 全局访问点”。
//! - `Logger`：可变单例。多个线程都会追加日志，因此需要在单例外层加 `Mutex`，用 `OnceLock<Mutex<Logger>>` 来保证线程安全写入。
//!   重点演示 “全局唯一实例 + 通过锁实现可变共享”。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;

/// 日志等级（示例用）。
#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
}

/// 应用配置（只读单例）。
#[derive(Debug)]
struct AppConfig {
    app_name: String,
    log_level: LogLevel,
}

static CONFIG_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 获取全局唯一的配置实例（只初始化一次）。
///
/// - 线程安全：`OnceLock` 保证初始化闭包最多执行一次。
/// - 返回值：`'static` 引用，表示该配置在程序生命周期内一直有效。
fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        let current = CONFIG_INIT_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        println!("[init] CONFIG 初始化中... init_count={current}");
        AppConfig {
            app_name: "design-pattern-demo".to_string(),
            log_level: LogLevel::Info,
        }
    })
}

/// 日志器（可变单例），内部保存日志行。
#[derive(Debug, Default)]
struct Logger {
    lines: Vec<String>,
}

impl Logger {
    /// 追加一条日志。
    fn log(&mut self, line: String) {
        self.lines.push(line);
    }

    /// 返回当前日志行数。
    fn len(&self) -> usize {
        self.lines.len()
    }
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

/// 获取全局唯一的日志器（可变单例）。
///
/// - 线程安全：通过 `Mutex` 序列化写入，避免数据竞争。
fn get_logger() -> &'static Mutex<Logger> {
    LOGGER.get_or_init(|| Mutex::new(Logger::default()))
}

fn main() {
    let cfg = get_config();
    let cfg_ptr = cfg as *const AppConfig;
    println!("[main] config_ptr={cfg_ptr:p} config={cfg:?}");

    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                let cfg = get_config();
                let cfg_ptr = cfg as *const AppConfig;
                let logger_ptr = get_logger() as *const Mutex<Logger>;

                let local_level = match i {
                    0 => LogLevel::Debug,
                    1 | 2 => LogLevel::Info,
                    _ => LogLevel::Warn,
                };
                let line = format!(
                    "[thread {i}] cfg_ptr={cfg_ptr:p} logger_ptr={logger_ptr:p} app_name={} cfg_level={:?} local_level={:?}",
                    cfg.app_name, cfg.log_level, local_level
                );

                let logger = get_logger();
                let mut guard = logger.lock().unwrap();
                guard.log(line);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let logger = get_logger();
    let guard = logger.lock().unwrap();
    println!("[main] logger_lines = {}", guard.len());
    for line in &guard.lines {
        println!("{line}");
    }
    println!(
        "[main] config_init_count = {}",
        CONFIG_INIT_COUNT.load(Ordering::SeqCst)
    );
}
