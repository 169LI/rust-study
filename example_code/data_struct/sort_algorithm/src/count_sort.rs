/// 计数排序主函数（假设输入为非负整数，范围 [0, k]）
fn counting_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // 长度为 0 或 1，无需排序
    }

    let max = arr.iter().max().unwrap_or(&0); // 找到最大值
    let mut count = vec![0; (*max + 1) as usize]; // 计数数组

    // 统计每个元素出现次数
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    // 计算每个元素的位置（累加）
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    // 创建输出数组，保持稳定性
    let mut output = vec![0; arr.len()];
    for &num in arr.iter().rev() { // 反向遍历以保持稳定性
        let index = count[num as usize] - 1;
        output[index] = num;
        count[num as usize] -= 1;
    }

    // 复制回原数组
    for i in 0..arr.len() {
        arr[i] = output[i];
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    counting_sort(&mut numbers);
    println!("计数排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    counting_sort(&mut empty);
    println!("计数排序后: {:?}", empty);

    // 测试用例 3：单位数数组
    let mut single_digit = vec![5, 2, 8, 1, 9, 3, 7, 4];
    println!("\n原始单位数数组: {:?}", single_digit);
    counting_sort(&mut single_digit);
    println!("计数排序后: {:?}", single_digit);

    // 测试用例 4：包含重复元素的数组
    let mut duplicates = vec![4, 2, 4, 1, 3, 2, 1];
    println!("\n原始重复元素数组: {:?}", duplicates);
    counting_sort(&mut duplicates);
    println!("计数排序后: {:?}", duplicates);
}