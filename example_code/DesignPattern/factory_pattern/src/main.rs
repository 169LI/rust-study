//! 工厂模式 (Factory Pattern) 示例：创建不同类型的“数据库客户端”
//!
//! 这个模块做什么：
//! - 将对象创建逻辑集中在工厂函数中，调用方只传入“类型/配置”，不关心具体 struct 如何构造。
//! - 工厂返回统一抽象（`Box<dyn Database>`），调用方只依赖 trait 接口。
//!
//! 场景说明：
//! - 模拟数据库连接：MySQL / Postgres / SQLite 三种实现。
//! - 通过命令行参数选择：`--db mysql|postgres|sqlite`。

use std::env;
use std::fmt;
use std::process;

/// 数据库客户端抽象（Product 接口）。
trait Database {
    /// 返回数据库类型名，用于展示。
    fn kind(&self) -> &'static str;

    /// 模拟连接动作。
    fn connect(&self) -> String;

    /// 模拟一次查询。
    fn query(&self, sql: &str) -> String;
}

#[derive(Debug, Clone)]
struct DbConfig {
    url: String,
}

impl DbConfig {
    /// 关联函数：用于构造配置（这不是工厂模式本身，但常作为工厂的输入）。
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

struct MySql {
    config: DbConfig,
}

impl Database for MySql {
    fn kind(&self) -> &'static str {
        "mysql"
    }

    fn connect(&self) -> String {
        format!("connect to MySQL via {}", self.config.url)
    }

    fn query(&self, sql: &str) -> String {
        format!("MySQL execute: {sql}")
    }
}

struct Postgres {
    config: DbConfig,
}

impl Database for Postgres {
    fn kind(&self) -> &'static str {
        "postgres"
    }

    fn connect(&self) -> String {
        format!("connect to Postgres via {}", self.config.url)
    }

    fn query(&self, sql: &str) -> String {
        format!("Postgres execute: {sql}")
    }
}

struct Sqlite {
    file_path: String,
}

impl Database for Sqlite {
    fn kind(&self) -> &'static str {
        "sqlite"
    }

    fn connect(&self) -> String {
        format!("open SQLite file {}", self.file_path)
    }

    fn query(&self, sql: &str) -> String {
        format!("SQLite execute: {sql}")
    }
}

#[derive(Debug, Clone, Copy)]
enum DbType {
    MySql,
    Postgres,
    Sqlite,
}

#[derive(Debug)]
enum FactoryError {
    UnknownDbType(String),
}

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactoryError::UnknownDbType(raw) => {
                write!(f, "unknown db type: {raw} (expected mysql|postgres|sqlite)")
            }
        }
    }
}

impl std::error::Error for FactoryError {}

/// 工厂函数：封装对象创建逻辑，并返回统一抽象（trait object）。
///
/// # Errors
/// - 当 db_type 不支持时返回 `FactoryError::UnknownDbType`。
fn create_database(db_type: DbType, config: &DbConfig) -> Result<Box<dyn Database>, FactoryError> {
    match db_type {
        DbType::MySql => Ok(Box::new(MySql {
            config: config.clone(),
        })),
        DbType::Postgres => Ok(Box::new(Postgres {
            config: config.clone(),
        })),
        DbType::Sqlite => Ok(Box::new(Sqlite {
            file_path: config.url.clone(),
        })),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), FactoryError> {
    let db_arg = parse_db_arg();
    let db_type = parse_db_type(&db_arg)?;

    let config = match db_type {
        DbType::MySql => DbConfig::new("mysql://localhost:3306/app"),
        DbType::Postgres => DbConfig::new("postgres://localhost:5432/app"),
        DbType::Sqlite => DbConfig::new("./app.db"),
    };

    let db = create_database(db_type, &config)?;

    println!("db_kind  = {}", db.kind());
    println!("connect  = {}", db.connect());
    println!("query    = {}", db.query("SELECT * FROM users LIMIT 3"));

    Ok(())
}

fn parse_db_arg() -> String {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--db" {
            return args.next().unwrap_or_else(|| "mysql".to_string());
        }
    }
    "mysql".to_string()
}

fn parse_db_type(raw: &str) -> Result<DbType, FactoryError> {
    match raw {
        "mysql" => Ok(DbType::MySql),
        "postgres" => Ok(DbType::Postgres),
        "sqlite" => Ok(DbType::Sqlite),
        _ => Err(FactoryError::UnknownDbType(raw.to_string())),
    }
}
