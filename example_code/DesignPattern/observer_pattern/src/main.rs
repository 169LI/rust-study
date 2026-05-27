//! 观察者模式 (Observer Pattern) 示例：温度传感器通知多个观察者。
//!
//! 这个模块做什么：
//! - `TemperatureSensor` 维护观察者列表，温度变化时统一广播事件。
//! - 具体观察者通过同一个 `Observer` trait 接收通知，彼此互不依赖。
//!
//! 场景说明：
//! - 屏幕显示器负责展示最新温度。
//! - 日志观察者负责记录变化。
//! - 告警观察者只在温度超过阈值时输出告警。
//! 
//! 其实观察者模式的核心就一句话：一个对象状态变了，自动通知多个关心它的人。
//! 
//! 比如温度传感器温度变了：
//! 
//! 温度传感器 changed
//!     // ↓
//! 通知显示屏
//! 通知日志系统
//! 通知告警系统
//! 
//! TemperatureSensor   被观察者
//! TemperatureEvent    通知事件
//! Observer trait      观察者接口
//! DisplayObserver     显示观察者
//! LoggerObserver      日志观察者
//! AlertObserver       告警观察者


fn main() {
    let mut sensor = TemperatureSensor::new("warehouse-a");
    sensor.register(Box::new(DisplayObserver));
    sensor.register(Box::new(LoggerObserver));
    sensor.register(Box::new(AlertObserver::new(30.0)));

    sensor.set_temperature(24.5);
    sensor.set_temperature(32.0);
}

/// 温度事件：被观察者发给观察者的不可变快照。
#[derive(Debug, Clone)]
struct TemperatureEvent {
    sensor_id: String,
    celsius: f32,
}

/// 观察者接口：所有订阅者都通过该接口接收状态变化。
trait Observer {
    /// 接收温度变化通知。
    fn update(&self, event: &TemperatureEvent);
}

/// 被观察者：保存温度状态并管理观察者列表。
struct TemperatureSensor {
    sensor_id: String,
    celsius: f32,
    observers: Vec<Box<dyn Observer>>,
}

impl TemperatureSensor {
    /// 创建温度传感器。
    fn new(sensor_id: &str) -> Self {
        Self {
            sensor_id: sensor_id.to_string(),
            celsius: 0.0,
            observers: Vec::new(),
        }
    }

    /// 注册一个观察者。
    fn register(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    /// 修改温度，并自动通知所有观察者。
    fn set_temperature(&mut self, celsius: f32) {
        self.celsius = celsius;
        println!(
            "[sensor] {} changed to {:.1}C",
            self.sensor_id, self.celsius
        );
        self.notify_all();
    }

    fn notify_all(&self) {
        let event = TemperatureEvent {
            sensor_id: self.sensor_id.clone(),
            celsius: self.celsius,
        };

        for observer in &self.observers {
            observer.update(&event);
        }
    }
}

/// 具体观察者：显示温度。
struct DisplayObserver;

impl Observer for DisplayObserver {
    fn update(&self, event: &TemperatureEvent) {
        println!("[display] {} = {:.1}C", event.sensor_id, event.celsius);
    }
}

/// 具体观察者：记录温度变化日志。
struct LoggerObserver;

impl Observer for LoggerObserver {
    fn update(&self, event: &TemperatureEvent) {
        println!("[logger] write temperature event: {:?}", event);
    }
}

/// 具体观察者：阈值告警。
struct AlertObserver {
    threshold: f32,
}

impl AlertObserver {
    /// 创建阈值告警观察者。
    fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Observer for AlertObserver {
    fn update(&self, event: &TemperatureEvent) {
        if event.celsius > self.threshold {
            println!(
                "[alert] {} is too hot: {:.1}C > {:.1}C",
                event.sensor_id, event.celsius, self.threshold
            );
        }
    }
}
