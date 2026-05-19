/// 插入排序函数
fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i].clone(); // 当前要插入的元素
        let mut j = i - 1; // 已排序部分的最后一个索引

        // 将比 key 大的元素向后移动
        // 添加 j > 0 条件防止溢出
        while j < n && j > 0 && arr[j] > key {
            arr[j + 1] = arr[j].clone();
            j -= 1;
        }
        // 处理 j == 0 的情况
        if j == 0 && arr[j] > key {
            arr[j + 1] = arr[j].clone();
            arr[j] = key;
        } else {
            arr[j + 1] = key;
        }
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("插入排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    insertion_sort(&mut empty);
    println!("插入排序后: {:?}", empty);

    // 测试用例 3：近乎有序数组
    let mut nearly_sorted = vec![1, 2, 4, 3, 5];
    println!("\n原始近乎有序数组: {:?}", nearly_sorted);
    insertion_sort(&mut nearly_sorted);
    println!("插入排序后: {:?}", nearly_sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果".to_string(), "香蕉".to_string(), "橙子".to_string(), "葡萄".to_string()];
    println!("\n原始字符串数组: {:?}", words);
    insertion_sort(&mut words);
    println!("插入排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    insertion_sort(&mut duplicates);
    println!("插入排序后: {:?}", duplicates);

    // 测试用例 6：逆序数组（触发溢出测试）
    let mut reverse = vec![5, 4, 3, 2, 1];
    println!("\n原始逆序数组: {:?}", reverse);
    insertion_sort(&mut reverse);
    println!("插入排序后: {:?}", reverse);
}