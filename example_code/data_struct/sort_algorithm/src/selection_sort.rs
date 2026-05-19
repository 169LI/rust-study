/// 选择排序函数
fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i; // 假设当前索引为最小值
        // 找到未排序部分的最小值索引
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        // 将最小值交换到已排序部分的末尾
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    selection_sort(&mut numbers);
    println!("选择排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    selection_sort(&mut empty);
    println!("选择排序后: {:?}", empty);

    // 测试用例 3：已排序数组
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("\n原始已排序数组: {:?}", sorted);
    selection_sort(&mut sorted);
    println!("选择排序后: {:?}", sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果", "香蕉", "橙子", "葡萄"];
    println!("\n原始字符串数组: {:?}", words);
    selection_sort(&mut words);
    println!("选择排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    selection_sort(&mut duplicates);
    println!("选择排序后: {:?}", duplicates);
}