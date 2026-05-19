/// 获取数字在指定位上的值（0-based 位，从低位开始）
fn get_digit(num: i32, pos: usize) -> i32 {
    (num / 10_i32.pow(pos as u32)) % 10
}

/// 基数排序主函数
fn radix_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // 长度为 0 或 1，无需排序
    }

    // 收集 arr 的值，计算最大值，避免借用
    let max = *arr.iter().max().unwrap_or(&0); // 使用 unwrap_or 防止空数组 panic
    let mut digit = 0;
    while 10_i32.pow(digit as u32) <= max {
        // 为每一位创建桶
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); 10];

        // 分配到桶中
        for &num in arr.iter() {
            let digit_value = get_digit(num, digit) as usize;
            buckets[digit_value].push(num);
        }

        // 收集回数组，保持稳定性
        let mut index = 0;
        for bucket in buckets {
            for &num in bucket.iter() {
                arr[index] = num;
                index += 1;
            }
        }

        digit += 1; // 移动到下一位
    }
}
fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("原始整数数组: {:?}", numbers);
    radix_sort(&mut numbers);
    println!("基数排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    radix_sort(&mut empty);
    println!("基数排序后: {:?}", empty);

    // 测试用例 3：单位数数组
    let mut single_digit = vec![5, 2, 8, 1, 9];
    println!("\n原始单位数数组: {:?}", single_digit);
    radix_sort(&mut single_digit);
    println!("基数排序后: {:?}", single_digit);

    // 测试用例 4：包含重复元素的数组
    let mut duplicates = vec![121, 432, 564, 23, 1, 45, 788];
    println!("\n原始重复元素数组: {:?}", duplicates);
    radix_sort(&mut duplicates);
    println!("基数排序后: {:?}", duplicates);
}