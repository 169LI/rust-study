/// 快速排序主函数
fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // 长度为 0 或 1，无需排序
    }
    quick_sort_recursive(arr, 0, arr.len() - 1);
}

/// 快速排序递归函数
fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        // 分区并获取基准索引
        let pivot_index = partition(arr, low, high);
        // 递归排序左半部分
        if pivot_index > 0 && pivot_index <= high {
            quick_sort_recursive(arr, low, pivot_index - 1);
        }
        // 递归排序右半部分
        quick_sort_recursive(arr, pivot_index + 1, high);
    }
}

/// 分区函数，选择最后一个元素作为基准
fn partition<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high; // 选择最后一个元素作为基准
    let mut i = low; // 小于基准的区域边界

    // 遍历数组，将小于基准的元素移到左边
    for j in low..high {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    // 将基准放到正确位置
    arr.swap(i, high);
    i // 返回基准索引
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("快速排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    quick_sort(&mut empty);
    println!("快速排序后: {:?}", empty);

    // 测试用例 3：已排序数组（最差情况）
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("\n原始已排序数组: {:?}", sorted);
    quick_sort(&mut sorted);
    println!("快速排序后: {:?}", sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果".to_string(), "香蕉".to_string(), "橙子".to_string(), "葡萄".to_string()];
    println!("\n原始字符串数组: {:?}", words);
    quick_sort(&mut words);
    println!("快速排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    quick_sort(&mut duplicates);
    println!("快速排序后: {:?}", duplicates);
}