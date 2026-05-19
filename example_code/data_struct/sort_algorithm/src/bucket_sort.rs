/// 桶内插入排序函数（保持稳定性）
fn insertion_sort_bucket(bucket: &mut Vec<f64>) {
    for i in 1..bucket.len() {
        let key = bucket[i];
        let mut j = i - 1;
        while j < bucket.len() && bucket[j] > key {
            bucket[j + 1] = bucket[j];
            if j == 0 { break; }
            j -= 1;
        }
        if j == 0 && bucket[j] > key {
            bucket[j + 1] = bucket[j];
            bucket[j] = key;
        } else {
            bucket[j + 1] = key;
        }
    }
}

/// 桶排序主函数（假设输入范围为 [0, 1) 的浮点数）
fn bucket_sort(arr: &mut [f64]) {
    let n = arr.len();
    if n <= 1 {
        return; // 长度为 0 或 1，无需排序
    }

    // 创建 n 个桶
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];

    // 分配元素到桶中
    for &num in arr.iter() {
        let bucket_idx = (num * n as f64).floor() as usize;
        let bucket_idx = bucket_idx.min(n - 1); // 防止越界
        buckets[bucket_idx].push(num);
    }

    // 对每个桶进行插入排序
    for bucket in buckets.iter_mut() {
        insertion_sort_bucket(bucket);
    }

    // 合并所有桶
    let mut index = 0;
    for bucket in buckets {
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}

fn main() {
    // 测试用例 1：普通浮点数组（范围 [0, 1)）
    let mut numbers = vec![0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68];
    println!("原始浮点数组: {:?}", numbers);
    bucket_sort(&mut numbers);
    println!("桶排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<f64> = vec![];
    println!("\n原始空数组: {:?}", empty);
    bucket_sort(&mut empty);
    println!("桶排序后: {:?}", empty);

    // 测试用例 3：已排序数组
    let mut sorted = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    println!("\n原始已排序数组: {:?}", sorted);
    bucket_sort(&mut sorted);
    println!("桶排序后: {:?}", sorted);

    // 测试用例 4：包含重复元素的数组
    let mut duplicates = vec![0.42, 0.32, 0.33, 0.42, 0.12, 0.89];
    println!("\n原始重复元素数组: {:?}", duplicates);
    bucket_sort(&mut duplicates);
    println!("桶排序后: {:?}", duplicates);
}