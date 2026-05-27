//! 解释器模式 (Interpreter Pattern) 示例：简单数学表达式解释器。
//!
//! 这个模块做什么：
//! - 使用 `enum Expr` 表示一门很小的表达式语言。
//! - 使用递归 `eval()` 解释表达式树并计算结果。
//!
//! 场景说明：
//! - 支持数字、加法、减法、乘法和变量。
//! - `Context` 提供变量值，表达式本身只描述语法结构。

use std::collections::HashMap;

fn main() -> Result<(), InterpretError> {
    let mut context = Context::new();
    context.set("x", 10);
    context.set("y", 20);
    context.set("z", 5);

    // 构建复杂表达式: (x + 5) * (y - z) + 10
    // 预期结果: (10 + 5) * (20 - 5) + 10 = 15 * 15 + 10 = 235
    let expr1 = Expr::add(
        Expr::mul(
            Expr::add(Expr::var("x"), Expr::number(5)),
            Expr::sub(Expr::var("y"), Expr::var("z")),
        ),
        Expr::number(10),
    );

    println!("[main] 正常求值:");
    println!("  expr1 = {}", expr1.format());
    println!("  result = {}", expr1.eval(&context)?);

    // 演示错误处理: 包含未定义变量的表达式 (w * 2)
    println!("\n[main] 错误处理演示:");
    let expr2 = Expr::mul(Expr::var("w"), Expr::number(2));
    println!("  expr2 = {}", expr2.format());
    match expr2.eval(&context) {
        Ok(val) => println!("  result = {}", val),
        Err(e) => println!("  error = {}", e),
    }

    Ok(())
}

/// 表达式语法树：每个变体代表一种语法节点。
#[derive(Debug, Clone)]
enum Expr {
    Number(i32),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// 创建数字节点。
    fn number(value: i32) -> Self {
        Self::Number(value)
    }

    /// 创建变量节点。
    fn var(name: &str) -> Self {
        Self::Variable(name.to_string())
    }

    /// 创建加法节点。
    fn add(left: Expr, right: Expr) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// 创建减法节点。
    fn sub(left: Expr, right: Expr) -> Self {
        Self::Sub(Box::new(left), Box::new(right))
    }

    /// 创建乘法节点。
    fn mul(left: Expr, right: Expr) -> Self {
        Self::Mul(Box::new(left), Box::new(right))
    }

    /// 解释表达式并返回求值结果。
    ///
    /// # Errors
    /// - 当表达式引用了上下文中不存在的变量时返回 `InterpretError::UnknownVariable`。
    fn eval(&self, context: &Context) -> Result<i32, InterpretError> {
        match self {
            Expr::Number(value) => Ok(*value),
            Expr::Variable(name) => context.get(name),
            Expr::Add(left, right) => Ok(left.eval(context)? + right.eval(context)?),
            Expr::Sub(left, right) => Ok(left.eval(context)? - right.eval(context)?),
            Expr::Mul(left, right) => Ok(left.eval(context)? * right.eval(context)?),
        }
    }

    /// 格式化表达式，便于观察语法树结构。
    fn format(&self) -> String {
        match self {
            Expr::Number(value) => value.to_string(),
            Expr::Variable(name) => name.clone(),
            Expr::Add(left, right) => format!("({} + {})", left.format(), right.format()),
            Expr::Sub(left, right) => format!("({} - {})", left.format(), right.format()),
            Expr::Mul(left, right) => format!("({} * {})", left.format(), right.format()),
        }
    }
}

/// 解释上下文：提供变量名到值的映射。
#[derive(Debug, Default)]
struct Context {
    variables: HashMap<String, i32>,
}

impl Context {
    /// 创建空上下文。
    fn new() -> Self {
        Self::default()
    }

    /// 设置变量值。
    fn set(&mut self, name: &str, value: i32) {
        self.variables.insert(name.to_string(), value);
    }

    fn get(&self, name: &str) -> Result<i32, InterpretError> {
        self.variables
            .get(name)
            .copied()
            .ok_or_else(|| InterpretError::UnknownVariable(name.to_string()))
    }
}

/// 解释执行错误。
#[derive(Debug)]
enum InterpretError {
    UnknownVariable(String),
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::UnknownVariable(name) => write!(f, "unknown variable: {name}"),
        }
    }
}

impl std::error::Error for InterpretError {}
