fn manual_binary_search<T: PartialOrd>(arr: &[T], target: &T) -> Option<usize> {
    // 初始化左右指针
    let mut left = 0;
    let mut right = arr.len().saturating_sub(1);

    // 当左指针不超过右指针时继续查找
    while left <= right {
        // 计算中间索引，避免溢出
        let mid = left + (right - left) / 2;

        // 比较中间元素与目标值
        if &arr[mid] == target {
            return Some(mid); // 找到目标，返回索引
        } else if &arr[mid] < target {
            left = mid + 1; // 目标在右半部分，缩小左边界
        } else {
            right = mid.saturating_sub(1); // 目标在左半部分，缩小右边界
        }
    }
    None // 未找到，返回 None
}

fn main() {
    // 定义测试用的有序整数数组
    let numbers = [1, 3, 5, 7, 9, 11, 13];
    // 定义多个测试目标值
    let target1 = 7; // 存在于数组中
    let target2 = 8; // 不存在于数组中

    // 测试用例 1：查找存在的元素
    println!("测试查找目标值 {}", target1);
    match manual_binary_search(&numbers, &target1) {
        Some(index) => println!("找到 {}，索引为 {}", target1, index),
        None => println!("未找到 {}", target1),
    }

    // 测试用例 2：查找不存在的元素
    println!("\n测试查找目标值 {}", target2);
    match manual_binary_search(&numbers, &target2) {
        Some(index) => println!("找到 {}，索引为 {}", target2, index),
        None => println!("未找到 {}", target2),
    }

    // 测试用例 3：查找字符串数组（展示泛型支持）
    let words = ["苹果", "香蕉", "橙子", "葡萄"];
    let target_word = "橙子";
    println!("\n测试查找字符串 '{}'", target_word);
    match manual_binary_search(&words, &target_word) {
        Some(index) => println!("找到 '{}'，索引为 {}", target_word, index),
        None => println!("未找到 '{}'", target_word),
    }
}