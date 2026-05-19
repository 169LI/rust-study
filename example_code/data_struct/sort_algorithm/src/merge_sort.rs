/// 归并排序
fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // 长度为 0 或 1，无需排序
    }
    // 创建临时数组，用于合并
    let mut temp = arr.to_vec();
    merge_sort_recursive(arr, &mut temp, 0, arr.len() - 1);
}

/// 归并排序递归函数
fn merge_sort_recursive<T: PartialOrd + Clone>(arr: &mut [T], temp: &mut [T], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        // 递归排序左半部分
        merge_sort_recursive(arr, temp, left, mid);
        // 递归排序右半部分
        merge_sort_recursive(arr, temp, mid + 1, right);
        // 合并两个有序子数组
        merge(arr, temp, left, mid, right);
    }
}

/// 合并两个有序子数组
fn merge<T: PartialOrd + Clone>(arr: &mut [T], temp: &mut [T], left: usize, mid: usize, right: usize) {
    // 复制数据到临时数组
    for i in left..=right {
        temp[i] = arr[i].clone();
    }

    let mut i = left; // 左子数组起始索引
    let mut j = mid + 1; // 右子数组起始索引
    let mut k = left; // 合并后数组的索引

    // 合并两个子数组
    while i <= mid && j <= right {
        if temp[i] <= temp[j] {
            arr[k] = temp[i].clone();
            i += 1;
        } else {
            arr[k] = temp[j].clone();
            j += 1;
        }
        k += 1;
    }

    // 复制剩余的左子数组元素
    while i <= mid {
        arr[k] = temp[i].clone();
        i += 1;
        k += 1;
    }
    // 右子数组剩余元素已直接合并，无需额外处理
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("归并排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    merge_sort(&mut empty);
    println!("归并排序后: {:?}", empty);

    // 测试用例 3：已排序数组
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("\n原始已排序数组: {:?}", sorted);
    merge_sort(&mut sorted);
    println!("归并排序后: {:?}", sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果".to_string(), "香蕉".to_string(), "橙子".to_string(), "葡萄".to_string()];
    println!("\n原始字符串数组: {:?}", words);
    merge_sort(&mut words);
    println!("归并排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    merge_sort(&mut duplicates);
    println!("归并排序后: {:?}", duplicates);
}