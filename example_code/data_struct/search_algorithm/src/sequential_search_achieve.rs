fn manual_sequential_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    // 遍历数组，逐一比较每个元素
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index); // 找到目标，返回索引
        }
    }
    None // 未找到，返回 None
}

fn main() {
    // 定义测试用的整数数组
    let numbers = [4, 2, 7, 1, 3];
    // 定义多个测试目标值
    let target1 = 7; // 存在于数组中
    let target2 = 5; // 不存在于数组中

    // 测试用例 1：查找存在的元素
    println!("测试查找目标值 {}", target1);
    match manual_sequential_search(&numbers, &target1) {
        Some(index) => println!("找到 {}，索引为 {}", target1, index),
        None => println!("未找到 {}", target1),
    }

    // 测试用例 2：查找不存在的元素
    println!("\n测试查找目标值 {}", target2);
    match manual_sequential_search(&numbers, &target2) {
        Some(index) => println!("找到 {}，索引为 {}", target2, index),
        None => println!("未找到 {}", target2),
    }

    // 测试用例 3：查找字符串数组（展示泛型支持）
    let words = ["苹果", "香蕉", "橙子", "葡萄"];
    let target_word = "橙子";
    println!("\n测试查找字符串 '{}'", target_word);
    match manual_sequential_search(&words, &target_word) {
        Some(index) => println!("找到 '{}'，索引为 {}", target_word, index),
        None => println!("未找到 '{}'", target_word),
    }
}