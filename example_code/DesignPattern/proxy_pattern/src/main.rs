//! 代理模式 (Proxy Pattern) 示例：打印服务的访问控制 + 延迟初始化
//!
//! 这个模块做什么：
//! - 通过代理对象替代真实对象对外提供统一接口，并在调用前后插入控制逻辑。
//! - 演示两类常见代理能力：
//!   - 保护代理：权限检查（不允许的用户直接拒绝访问）
//!   - 虚拟代理：延迟初始化“昂贵对象”（只有真正调用时才创建真实对象）
//!
//! 场景说明：
//! - 调用方只依赖 `Printer` trait，不直接接触 `RealPrinter`。
//! - `PrinterProxy` 在 `print()` 前做权限校验，并在首次调用时延迟创建 `RealPrinter`。

use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy)]
enum Role {
    Admin,
    Guest,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    role: Role,
}

/// 统一接口（Subject）。
trait Printer {
    /// 打印一段文本。
    fn print(&self, content: &str) -> Result<(), String>;
}

/// 真实对象（RealSubject）：执行业务逻辑。
struct RealPrinter {
    device_name: String,
}

impl RealPrinter {
    /// 初始化真实打印机（模拟昂贵创建过程）。
    fn new(device_name: &str) -> Self {
        println!("[real] init RealPrinter: device_name={device_name}");
        Self {
            device_name: device_name.to_string(),
        }
    }
}

impl Printer for RealPrinter {
    fn print(&self, content: &str) -> Result<(), String> {
        println!("[real] device={} print: {}", self.device_name, content);
        Ok(())
    }
}

/// 代理对象（Proxy）：控制对真实对象的访问。
struct PrinterProxy {
    user: User,
    real: OnceLock<RealPrinter>,
}

impl PrinterProxy {
    /// 创建代理对象。
    fn new(user: User) -> Self {
        Self {
            user,
            real: OnceLock::new(),
        }
    }

    /// 权限检查（保护代理能力）。
    fn check_permission(&self) -> Result<(), String> {
        match self.user.role {
            Role::Admin => Ok(()),
            Role::Guest => Err(format!("permission denied for user={}", self.user.name)),
        }
    }
}

impl Printer for PrinterProxy {
    fn print(&self, content: &str) -> Result<(), String> {
        println!("[proxy] before: user={:?}", self.user);
        self.check_permission()?;

        let real = self
            .real
            .get_or_init(|| RealPrinter::new("OfficePrinter-01"));
        let result = real.print(content);

        println!("[proxy] after: done");
        result
    }
}

fn main() {
    let user = parse_user();
    let proxy = PrinterProxy::new(user);

    let result1 = proxy.print("print task #1");
    println!("[main] result1={result1:?}");

    let result2 = proxy.print("print task #2");
    println!("[main] result2={result2:?}");
}

fn parse_user() -> User {
    let mut args = env::args().skip(1);
    let mut name = "alice".to_string();
    let mut role = Role::Guest;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--user" => {
                name = args.next().unwrap_or_else(|| "alice".to_string());
            }
            "--role" => {
                role = match args.next().as_deref() {
                    Some("admin") => Role::Admin,
                    Some("guest") => Role::Guest,
                    _ => Role::Guest,
                };
            }
            _ => {}
        }
    }

    User { name, role }
}
