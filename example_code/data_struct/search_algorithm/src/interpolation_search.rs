//!插值查找是一种针对均匀分布的有序数据的查找算法，改进了二分查找，通过估算目标值可能的位置来减少比较次数。




fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    // 初始化左右指针
    let mut left = 0;
    let mut right = arr.len().saturating_sub(1);

    // 当左指针不超过右指针且目标值在范围内时继续查找
    while left <= right && target >= arr[left] && target <= arr[right] {
        // 防止除零或无效索引
        if left == right {
            if arr[left] == target {
                return Some(left);
            }
            return None;
        }

        // 计算插值点：基于目标值与边界值的比例估算位置
        let pos = left + ((target - arr[left]) as usize * (right - left)) / ((arr[right] - arr[left]) as usize);

        // 确保 pos 在有效范围内
        if pos >= arr.len() {
            return None;
        }

        // 比较插值点处的元素
        if arr[pos] == target {
            return Some(pos); // 找到目标，返回索引
        } else if arr[pos] < target {
            left = pos + 1; // 目标在右半部分，缩小左边界
        } else {
            right = pos.saturating_sub(1); // 目标在左半部分，缩小右边界
        }
    }
    None // 未找到，返回 None
}

fn main() {
    // 定义测试用的均匀分布有序整数数组
    let numbers = [10, 20, 30, 40, 50, 60, 70];
    // 定义多个测试目标值
    let target1 = 40; // 存在于数组中
    let target2 = 45; // 不存在于数组中
    let target3 = 10; // 在数组开头

    // 测试用例 1：查找存在的元素
    println!("测试查找目标值 {}", target1);
    match interpolation_search(&numbers, target1) {
        Some(index) => println!("找到 {}，索引为 {}", target1, index),
        None => println!("未找到 {}", target1),
    }

    // 测试用例 2：查找不存在的元素
    println!("\n测试查找目标值 {}", target2);
    match interpolation_search(&numbers, target2) {
        Some(index) => println!("找到 {}，索引为 {}", target2, index),
        None => println!("未找到 {}", target2),
    }

    // 测试用例 3：查找数组开头的元素
    println!("\n测试查找目标值 {}", target3);
    match interpolation_search(&numbers, target3) {
        Some(index) => println!("找到 {}，索引为 {}", target3, index),
        None => println!("未找到 {}", target3),
    }

    // 测试用例 4：非均匀分布数据（展示局限性）
    let non_uniform = [1, 2, 100, 1000, 10000];
    let target4 = 1000;
    println!("\n测试非均匀分布数据，查找 {}", target4);
    match interpolation_search(&non_uniform, target4) {
        Some(index) => println!("找到 {}，索引为 {}", target4, index),
        None => println!("未找到 {}", target4),
    }
}