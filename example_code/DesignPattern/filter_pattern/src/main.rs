//! 过滤器模式 (Filter Pattern) 示例项目
//!
//! 本项目演示了 Rust 中过滤器模式的经典用法。
//!
//! ## 核心思想：
//! 将“对元素的条件筛选逻辑”抽象出来（通过闭包表达），
//! 使数据处理逻辑与具体集合类型解耦，并允许多个筛选条件组合成管道式处理。
//!
//! ## 场景：
//! 从用户列表中筛选出“年龄大于 18 岁”且“状态为活跃”的用户，并统计数量。

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

fn main() {
    // 1. 准备数据源
    let users = vec![
        User {
            name: String::from("Alice"),
            age: 22,
            active: true,
        },
        User {
            name: String::from("Bob"),
            age: 17,
            active: true,
        },
        User {
            name: String::from("Charlie"),
            age: 25,
            active: false,
        },
        User {
            name: String::from("Dave"),
            age: 19,
            active: true,
        },
        User {
            name: String::from("Eve"),
            age: 30,
            active: true,
        },
    ];

    println!("--- 1. 原始用户列表 ---");
    for u in &users {
        println!("{:?}", u);
    }

    // ==============================================================
    // 反面教材：如果不使用过滤器模式（传统的面向过程写法）
    // ==============================================================
    let mut traditional_filtered = Vec::new();
    for u in &users {
        // 遍历和筛选逻辑深度耦合，多个条件嵌套在一起
        if u.age > 18 {
            if u.active {
                traditional_filtered.push(u.clone());
            }
        }
    }
    println!("\n--- 2. 传统写法筛选出的用户 ---");
    for u in &traditional_filtered {
        println!("{:?}", u);
    }

    // ==============================================================
    // 正面示范：使用 Rust 过滤器模式（迭代器链 + filter 闭包）
    // ==============================================================

    // 我们将复杂的筛选条件拆分为多个独立的过滤器（filter）
    // 这样做的好处是：条件解耦，非常容易增加或移除某个过滤环节
    let active_adults: Vec<&User> = users
        .iter()
        // 过滤器 1：筛选年龄大于 18 岁
        .filter(|u| u.age > 18)
        // 过滤器 2：筛选状态为活跃
        .filter(|u| u.active)
        .collect();

    println!("\n--- 3. 过滤器模式筛选出的用户 ---");
    for u in &active_adults {
        println!("{:?}", u);
    }

    // ==============================================================
    // 进阶：重用和组合过滤器（将闭包抽离为独立的变量或函数）
    // ==============================================================
    // 在复杂的业务中，过滤条件可能需要复用
    let is_adult = |u: &&User| u.age > 18;
    let is_active = |u: &&User| u.active;

    let valid_count = users.iter().filter(is_adult).filter(is_active).count();

    println!("\n--- 4. 统计结果 ---");
    println!("符合条件（成年且活跃）的用户总数为: {} 人", valid_count);
}
