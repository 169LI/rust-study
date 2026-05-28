//! 状态模式 (State Pattern) 示例：订单状态流转。
//!
//! 这个模块做什么：
//! - 使用 `enum + match` 表达订单状态机，让同一个操作在不同状态下有不同行为。
//! - 非法状态转换返回错误，避免把状态规则散落在调用方。
//!
//! 场景说明：
//! - `Created` 可以支付或取消。
//! - `Paid` 可以发货。
//! - `Shipped` 可以确认完成。
//! - `Finished` 和 `Cancelled` 都是终态。

fn main() -> Result<(), OrderError> {
    let mut order = Order::new("order-1001");
    order.pay()?;
    order
        .cancel()
        .unwrap_or_else(|err| println!("[main] cancel failed: {err}"));
    order.ship()?;
    order.finish()?;
    order
        .pay()
        .unwrap_or_else(|err| println!("[main] pay failed: {err}"));

    println!("[main] final state: {:?}", order.state());
    Ok(())
}

/// 订单状态：用有限枚举值表达状态机中的节点。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderState {
    Created,
    Paid,
    Shipped,
    Finished,
    Cancelled,
}

/// 订单对象：上下文 (Context)，行为由当前状态决定。
#[derive(Debug)]
struct Order {
    id: String,
    state: OrderState,
}

impl Order {
    /// 创建初始状态为 `Created` 的订单。
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            state: OrderState::Created,
        }
    }

    /// 读取当前状态。
    fn state(&self) -> OrderState {
        self.state
    }

    /// 支付订单：只有 `Created` 状态允许支付。
    fn pay(&mut self) -> Result<(), OrderError> {
        match self.state {
            OrderState::Created => self.transition_to(OrderState::Paid, "paid"),
            _ => Err(OrderError::InvalidTransition {
                order_id: self.id.clone(),
                from: self.state,
                action: "pay",
            }),
        }
    }

    /// 发货订单：只有 `Paid` 状态允许发货。
    fn ship(&mut self) -> Result<(), OrderError> {
        match self.state {
            OrderState::Paid => self.transition_to(OrderState::Shipped, "shipped"),
            _ => Err(OrderError::InvalidTransition {
                order_id: self.id.clone(),
                from: self.state,
                action: "ship",
            }),
        }
    }

    /// 确认完成：只有 `Shipped` 状态允许完成。
    fn finish(&mut self) -> Result<(), OrderError> {
        match self.state {
            OrderState::Shipped => self.transition_to(OrderState::Finished, "finished"),
            _ => Err(OrderError::InvalidTransition {
                order_id: self.id.clone(),
                from: self.state,
                action: "finish",
            }),
        }
    }

    /// 取消订单：只有 `Created` 状态允许取消。
    fn cancel(&mut self) -> Result<(), OrderError> {
        match self.state {
            OrderState::Created => self.transition_to(OrderState::Cancelled, "cancelled"),
            _ => Err(OrderError::InvalidTransition {
                order_id: self.id.clone(),
                from: self.state,
                action: "cancel",
            }),
        }
    }

    fn transition_to(&mut self, next: OrderState, label: &str) -> Result<(), OrderError> {
        println!(
            "[order] id={} {:?} -> {:?} ({label})",
            self.id, self.state, next
        );
        self.state = next;
        Ok(())
    }
}

/// 订单状态转换错误。
#[derive(Debug)]
enum OrderError {
    InvalidTransition {
        order_id: String,
        from: OrderState,
        action: &'static str,
    },
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderError::InvalidTransition {
                order_id,
                from,
                action,
            } => write!(
                f,
                "order {order_id} cannot perform action {action:?} from state {from:?}"
            ),
        }
    }
}

impl std::error::Error for OrderError {}
