use std::collections::{HashMap, HashSet};

fn main() {
    // 1. 使用 HashMap 进行键值对查找
    // 定义一个 HashMap，键为 i32，值为 &str
    let mut map: HashMap<i32, &str> = HashMap::new();
    // 插入键值对
    map.insert(1, "一");
    map.insert(2, "二");
    map.insert(7, "七");

    // 定义查找的目标键
    let target_key1 = 7; // 存在于 HashMap
    let target_key2 = 5; // 不存在于 HashMap

    // 使用 get 查找值
    println!("测试 HashMap 查找键 {}", target_key1);
    match map.get(&target_key1) {
        Some(value) => println!("找到键 {}，值为 {}", target_key1, value),
        None => println!("键 {} 未找到", target_key1),
    }

    // 测试不存在的键
    println!("\n测试 HashMap 查找键 {}", target_key2);
    match map.get(&target_key2) {
        Some(value) => println!("找到键 {}，值为 {}", target_key2, value),
        None => println!("键 {} 未找到", target_key2),
    }

    // 使用 contains_key 检查键是否存在
    println!("\n检查键 {} 是否存在", target_key1);
    if map.contains_key(&target_key1) {
        println!("键 {} 存在", target_key1);
    } else {
        println!("键 {} 不存在", target_key1);
    }

    // 2. 使用 HashSet 进行元素存在性检查
    // 定义一个 HashSet，存储 i32
    let mut set: HashSet<i32> = HashSet::new();
    // 插入元素
    set.insert(10);
    set.insert(20);
    set.insert(30);

    // 定义查找的目标元素
    let target_elem = 20;
    // 使用 contains 检查元素
    println!("\n测试 HashSet 查找元素 {}", target_elem);
    if set.contains(&target_elem) {
        println!("元素 {} 存在", target_elem);
    } else {
        println!("元素 {} 不存在", target_elem);
    }
}