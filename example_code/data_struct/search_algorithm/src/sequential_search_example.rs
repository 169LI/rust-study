fn main() {
    // 定义一个整数数组
    let numbers = [4, 2, 7, 1, 3];
    // 定义查找的目标值
    let target = 7;

    // 1. 使用 `contains` 检查目标值是否存在
    // 返回 bool 类型，适合只需要知道是否存在的情况
    if numbers.contains(&target) {
        println!("数组中包含目标值 {}", target);
    } else {
        println!("数组中不包含目标值 {}", target);
    }

    // 2. 使用 `position` 查找目标值的索引
    // 返回 Option<usize>，找到返回 Some(索引)，未找到返回 None
    match numbers.iter().position(|&x| x == target) {
        Some(index) => println!("目标值 {} 的索引是 {}", target, index),
        None => println!("目标值 {} 未找到", target),
    }

    // 3. 使用 `find` 查找第一个满足条件的元素
    // 返回 Option<&T>，可以获取元素引用，适合需要元素值的情况
    match numbers.iter().find(|&&x| x == target) {
        Some(&value) => println!("找到目标值 {}", value),
        None => println!("目标值 {} 未找到", target),
    }

    // 4. 查找第一个大于某个值的元素（示例条件查找）
    // 使用 `find` 结合条件，展示灵活性
    match numbers.iter().find(|&&x| x > 5) {
        Some(&value) => println!("第一个大于 5 的值是 {}", value),
        None => println!("没有值大于 5"),
    }
}