/// 堆排序主函数
fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return; // 长度为 0 或 1，无需排序
    }

    // 构建最大堆
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // 提取堆顶并调整堆
    for i in (1..n).rev() {
        arr.swap(0, i); // 将堆顶（最大值）移到数组末尾
        heapify(arr, i, 0); // 调整剩余部分为最大堆
    }
}

/// 调整堆函数，保持最大堆性质
fn heapify<T: PartialOrd>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i; // 假设当前节点为最大
    let left = 2 * i + 1; // 左子节点
    let right = 2 * i + 2; // 右子节点

    // 比较左子节点
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    // 比较右子节点
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // 如果最大值不是当前节点，交换并递归调整
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    heap_sort(&mut numbers);
    println!("堆排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    heap_sort(&mut empty);
    println!("堆排序后: {:?}", empty);

    // 测试用例 3：已排序数组
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("\n原始已排序数组: {:?}", sorted);
    heap_sort(&mut sorted);
    println!("堆排序后: {:?}", sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果".to_string(), "香蕉".to_string(), "橙子".to_string(), "葡萄".to_string()];
    println!("\n原始字符串数组: {:?}", words);
    heap_sort(&mut words);
    println!("堆排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    heap_sort(&mut duplicates);
    println!("堆排序后: {:?}", duplicates);
}