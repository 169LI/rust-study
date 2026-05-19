/// 希尔排序

fn shell_sort<T: PartialOrd+Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return; // 长度为 0 或 1，无需排序
    }

    // 逐步减少增量
    let mut gap = n / 2;
    while gap > 0 {
        // 对每个增量进行插入排序
        for i in gap..n {
            let temp = arr[i].clone(); // 当前要插入的元素
            let mut j = i;

            // 将小于 temp 的元素向后移动
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap].clone();
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2; // 缩小增量
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    shell_sort(&mut numbers);
    println!("希尔排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    shell_sort(&mut empty);
    println!("希尔排序后: {:?}", empty);

    // 测试用例 3：近乎有序数组
    let mut nearly_sorted = vec![1, 2, 4, 3, 5];
    println!("\n原始近乎有序数组: {:?}", nearly_sorted);
    shell_sort(&mut nearly_sorted);
    println!("希尔排序后: {:?}", nearly_sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果".to_string(), "香蕉".to_string(), "橙子".to_string(), "葡萄".to_string()];
    println!("\n原始字符串数组: {:?}", words);
    shell_sort(&mut words);
    println!("希尔排序后: {:?}", words);

    // 测试用例 5：包含重复元素的数组
    let mut duplicates = vec![3, 1, 4, 1, 5, 2];
    println!("\n原始重复元素数组: {:?}", duplicates);
    shell_sort(&mut duplicates);
    println!("希尔排序后: {:?}", duplicates);

    // 测试用例 6：逆序数组
    let mut reverse = vec![5, 4, 3, 2, 1];
    println!("\n原始逆序数组: {:?}", reverse);
    shell_sort(&mut reverse);
    println!("希尔排序后: {:?}", reverse);
}