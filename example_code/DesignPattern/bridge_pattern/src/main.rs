//! 桥接模式 (Bridge Pattern) 示例：消息类型 × 发送方式
//!
//! 这个模块做什么：
//! - 演示当系统存在两个独立变化维度时，如何把“抽象层”和“实现层”拆开，通过组合连接，避免类型组合爆炸。
//!
//! 场景说明：
//! - 抽象维度（可变化）：消息类型（普通 / 加急）。
//! - 实现维度（可变化）：发送方式（邮件 / 短信）。
//! - 桥接后：消息类型只依赖 `MessageSender` 抽象接口，可以自由组合任何发送方式。
//!
//! 演示内容
//! - 两个独立变化维度被拆开：
//!   - 抽象层： NormalMessage / UrgentMessage
//!   - 实现层： EmailSender / SmsSender （统一接口 MessageSender ）
//! - 同时演示两种桥接方式：
//!  - 静态桥接：泛型 S: MessageSender （编译期确定实现）
//!  - 动态桥接： Box<dyn MessageSender> （运行时选择实现）
//!
//!
//!

/// 实现层接口：定义“怎么发送”。
trait MessageSender {
    /// 发送一条消息。
    fn send(&self, content: &str);

    /// 返回发送方式名称，用于输出展示。
    fn channel_name(&self) -> &'static str;
}

/// 具体实现：邮件发送器。
struct EmailSender {
    from: String,
}

impl EmailSender {
    fn new(from: impl Into<String>) -> Self {
        Self { from: from.into() }
    }
}

impl MessageSender for EmailSender {
    fn send(&self, content: &str) {
        println!("[email from={}] {}", self.from, content);
    }

    fn channel_name(&self) -> &'static str {
        "email"
    }
}

/// 具体实现：短信发送器。
struct SmsSender {
    phone: String,
}

impl SmsSender {
    fn new(phone: impl Into<String>) -> Self {
        Self {
            phone: phone.into(),
        }
    }
}

impl MessageSender for SmsSender {
    fn send(&self, content: &str) {
        println!("[sms to={}] {}", self.phone, content);
    }

    fn channel_name(&self) -> &'static str {
        "sms"
    }
}

/// 抽象层：普通消息（通过组合“桥接”到实现层）。
struct NormalMessage<S: MessageSender> {
    sender: S,
}

impl<S: MessageSender> NormalMessage<S> {
    fn new(sender: S) -> Self {
        Self { sender }
    }

    fn dispatch(&self, content: &str) {
        println!("kind=normal channel={}", self.sender.channel_name());
        self.sender.send(content);
    }
}

/// 抽象层：加急消息（仍然只依赖实现层接口，而不是依赖具体实现）。
struct UrgentMessage<S: MessageSender> {
    sender: S,
}

impl<S: MessageSender> UrgentMessage<S> {
    fn new(sender: S) -> Self {
        Self { sender }
    }

    fn dispatch(&self, content: &str) {
        println!("kind=urgent channel={}", self.sender.channel_name());
        self.sender.send(&format!("[URGENT] {content}"));
    }
}

/// 动态桥接：运行时选择发送实现（更灵活，但有动态分发开销）。
struct DynamicMessage {
    kind: MessageKind,
    sender: Box<dyn MessageSender>,
}

impl DynamicMessage {
    fn new(kind: MessageKind, sender: Box<dyn MessageSender>) -> Self {
        Self { kind, sender }
    }

    fn dispatch(&self, content: &str) {
        match self.kind {
            MessageKind::Normal => {
                println!("kind=normal channel={}", self.sender.channel_name());
                self.sender.send(content);
            }
            MessageKind::Urgent => {
                println!("kind=urgent channel={}", self.sender.channel_name());
                self.sender.send(&format!("[URGENT] {content}"));
            }
        }
    }
}

#[derive(Clone, Copy)]
enum MessageKind {
    Normal,
    Urgent,
}

fn main() {
    println!("--- 静态桥接（泛型，编译期确定实现） ---");
    let normal_email = NormalMessage::new(EmailSender::new("noreply@example.com"));
    normal_email.dispatch("system update");

    let urgent_sms = UrgentMessage::new(SmsSender::new("+86-138-0000-0000"));
    urgent_sms.dispatch("server down");

    println!();
    println!("--- 动态桥接（Box<dyn Trait>，运行时选择实现） ---");
    let dynamic_normal = DynamicMessage::new(
        MessageKind::Normal,
        Box::new(SmsSender::new("+86-139-0000-0000")),
    );
    dynamic_normal.dispatch("dynamic choose sms");

    let dynamic_urgent = DynamicMessage::new(
        MessageKind::Urgent,
        Box::new(EmailSender::new("alert@example.com")),
    );
    dynamic_urgent.dispatch("dynamic choose email");
}
