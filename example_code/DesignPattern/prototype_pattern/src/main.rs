//! 原型模式 (Prototype Pattern) 示例：Clone 复制原型对象
//!
//! 这个模块做什么：
//! - 构造一个“原型配置对象”，通过 `Clone` 快速复制出多个相似配置，并对差异字段做局部修改。
//!
//! 场景说明：
//! - 以“服务器配置模板”为例：先准备默认模板，再派生 dev/test/prod 环境配置。

use std::sync::Arc;

/// 日志等级（示例用），用于展示“从原型复制后再局部修改”的效果。
#[derive(Debug, Clone)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 服务器配置（原型对象）。
///
/// 该结构体派生 `Clone`，使得我们可以通过复制已有实例来创建“绝大部分字段相同”的新配置。
#[derive(Debug, Clone)]
struct ServerConfig {
    env: String,
    host: String,
    port: u16,
    timeout_ms: u64,
    retry: u8,
    use_tls: bool,
    log_level: LogLevel,
    tags: Vec<String>,
    banner: String,
    shared_ca_cert: Arc<String>,
}

impl ServerConfig {
    /// 基于原型对象复制出一个新配置，并按环境名做局部修改。
    ///
    /// - `prototype`：作为模板的原型对象
    /// - `env`：目标环境（如 `"dev"`, `"test"`, `"prod"`）
    fn from_prototype(prototype: &ServerConfig, env: &str) -> Self {
        let mut cfg = prototype.clone();
        cfg.env = env.to_string();

        match env {
            "dev" => {
                cfg.host = "127.0.0.1".to_string();
                cfg.port = 8080;
                cfg.use_tls = false;
                cfg.log_level = LogLevel::Debug;
                cfg.tags.push("local".to_string());
            }
            "test" => {
                cfg.host = "10.0.0.10".to_string();
                cfg.port = 18080;
                cfg.use_tls = false;
                cfg.log_level = LogLevel::Info;
                cfg.tags.push("ci".to_string());
            }
            "prod" => {
                cfg.host = "api.example.com".to_string();
                cfg.port = 443;
                cfg.use_tls = true;
                cfg.log_level = LogLevel::Warn;
                cfg.retry = 5;
                cfg.timeout_ms = 8_000;
                cfg.tags.push("critical".to_string());
            }
            _ => {
                cfg.tags.push("custom".to_string());
            }
        }

        cfg
    }
}

fn main() {
    let prototype = ServerConfig {
        env: "template".to_string(),
        host: "0.0.0.0".to_string(),
        port: 80,
        timeout_ms: 3_000,
        retry: 2,
        use_tls: true,
        log_level: LogLevel::Info,
        tags: vec!["base".to_string()],
        banner: "Welcome to Prototype Pattern Demo".to_string(),
        shared_ca_cert: Arc::new(
            "-----BEGIN CERTIFICATE-----\nCA...\n-----END CERTIFICATE-----".to_string(),
        ),
    };

    let dev = ServerConfig::from_prototype(&prototype, "dev");
    let test = ServerConfig::from_prototype(&prototype, "test");

    let mut prod = ServerConfig {
        env: "prod".to_string(),
        ..prototype.clone()
    };
    prod.host = "api.example.com".to_string();
    prod.port = 443;
    prod.use_tls = true;
    prod.log_level = LogLevel::Warn;
    prod.retry = 5;
    prod.timeout_ms = 8_000;
    prod.tags.push("critical".to_string());

    println!("prototype = {prototype:#?}");
    println!("dev       = {dev:#?}");
    println!("test      = {test:#?}");
    println!("prod      = {prod:#?}");

    println!();
    println!("banner ptr (prototype) = {:p}", prototype.banner.as_ptr());
    println!("banner ptr (dev)       = {:p}", dev.banner.as_ptr());
    println!("banner ptr (test)      = {:p}", test.banner.as_ptr());
    println!("banner ptr (prod)      = {:p}", prod.banner.as_ptr());

    // banner ptr (...) 为什么都不一样？
    // banner 的类型是 String 。
    // String::clone() 会复制堆上的字符串数据（通常会重新分配一块内存），因此每个克隆出来的配置里 banner 都有自己独立的缓冲区。
    // banner.as_ptr() 打印的是这个字符串缓冲区在内存里的地址，所以你看到：
    // prototype / dev / test / prod 的 banner ptr 都不同
    // 这说明它们不是共享同一段字符串内存。


    println!();
    println!(
        "shared_ca_cert ptr (prototype) = {:p}",
        Arc::as_ptr(&prototype.shared_ca_cert)
    );
    println!(
        "shared_ca_cert ptr (dev)       = {:p}",
        Arc::as_ptr(&dev.shared_ca_cert)
    );
    println!(
        "shared_ca_cert ptr (test)      = {:p}",
        Arc::as_ptr(&test.shared_ca_cert)
    );
    println!(
        "shared_ca_cert ptr (prod)      = {:p}",
        Arc::as_ptr(&prod.shared_ca_cert)
    );
    println!(
        "Arc strong_count (prototype.shared_ca_cert) = {}",
        Arc::strong_count(&prototype.shared_ca_cert)
    );

    // shared_ca_cert ptr (...) 为什么都一样？
    // 
    // shared_ca_cert 的类型是 Arc<String> 。
    // Arc::clone() 不会复制底层 String 数据，它只会把引用计数 +1，让多个对象共享同一份堆内存。
    // Arc::as_ptr(&arc) 打印的是 Arc 管理的那块数据地址（指向同一个 String 所在的分配），所以你看到：
    //   prototype / dev / test / prod 的 shared_ca_cert ptr 全一样
    //  这说明它们共享同一个证书字符串（没有重复拷贝大数据）。

}
