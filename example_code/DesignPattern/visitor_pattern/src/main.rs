//! 访问者模式 (Visitor Pattern) 示例：表达式树的多种操作。
//!
//! 这个模块做什么：
//! - 表达式树 `Expr` 只负责保存结构和分发访问。
//! - 统计、打印、求值三种操作分别放到不同 Visitor 中，避免把操作塞进节点结构。
//!
//! 场景说明：
//! - 当表达式节点结构相对稳定，但要不断增加分析、打印、求值等操作时，访问者模式更清晰。
//! - `accept()` 负责根据节点类型调用访问者方法。

fn main() {
    let expr = Expr::add(Expr::number(1), Expr::mul(Expr::number(2), Expr::number(3)));

    let mut counter = CountVisitor::default();
    expr.accept(&mut counter);

    let mut printer = PrintVisitor::default();
    expr.accept(&mut printer);

    let mut evaluator = EvalVisitor::default();
    expr.accept(&mut evaluator);

    println!("[main] node_count={}", counter.count);
    println!("[main] printed={}", printer.result);
    println!("[main] value={}", evaluator.value);
}

/// 被访问的数据结构：一个简单表达式树。
#[derive(Debug, Clone)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// 创建数字节点。
    fn number(value: i32) -> Self {
        Self::Number(value)
    }

    /// 创建加法节点。
    fn add(left: Expr, right: Expr) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// 创建乘法节点。
    fn mul(left: Expr, right: Expr) -> Self {
        Self::Mul(Box::new(left), Box::new(right))
    }

    /// 接受访问者，并把分发逻辑集中在数据结构内部。
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Expr::Number(value) => visitor.visit_number(*value),
            Expr::Add(left, right) => visitor.visit_add(left, right),
            Expr::Mul(left, right) => visitor.visit_mul(left, right),
        }
    }
}

/// 访问者接口：为每类节点提供访问方法。
trait Visitor {
    /// 访问数字节点。
    fn visit_number(&mut self, value: i32);

    /// 访问加法节点。
    fn visit_add(&mut self, left: &Expr, right: &Expr);

    /// 访问乘法节点。
    fn visit_mul(&mut self, left: &Expr, right: &Expr);
}

/// 具体访问者：统计表达式树节点数量。
#[derive(Debug, Default)]
struct CountVisitor {
    count: usize,
}

impl Visitor for CountVisitor {
    fn visit_number(&mut self, _value: i32) {
        self.count += 1;
    }

    fn visit_add(&mut self, left: &Expr, right: &Expr) {
        self.count += 1;
        left.accept(self);
        right.accept(self);
    }

    fn visit_mul(&mut self, left: &Expr, right: &Expr) {
        self.count += 1;
        left.accept(self);
        right.accept(self);
    }
}

/// 具体访问者：把表达式树打印为中缀表达式。
#[derive(Debug, Default)]
struct PrintVisitor {
    result: String,
}

impl Visitor for PrintVisitor {
    fn visit_number(&mut self, value: i32) {
        self.result = value.to_string();
    }

    fn visit_add(&mut self, left: &Expr, right: &Expr) {
        self.result = format!("({} + {})", render(left), render(right));
    }

    fn visit_mul(&mut self, left: &Expr, right: &Expr) {
        self.result = format!("({} * {})", render(left), render(right));
    }
}

/// 具体访问者：计算表达式树的值。
#[derive(Debug, Default)]
struct EvalVisitor {
    value: i32,
}

impl Visitor for EvalVisitor {
    fn visit_number(&mut self, value: i32) {
        self.value = value;
    }

    fn visit_add(&mut self, left: &Expr, right: &Expr) {
        self.value = evaluate(left) + evaluate(right);
    }

    fn visit_mul(&mut self, left: &Expr, right: &Expr) {
        self.value = evaluate(left) * evaluate(right);
    }
}

fn render(expr: &Expr) -> String {
    let mut visitor = PrintVisitor::default();
    expr.accept(&mut visitor);
    visitor.result
}

fn evaluate(expr: &Expr) -> i32 {
    let mut visitor = EvalVisitor::default();
    expr.accept(&mut visitor);
    visitor.value
}
