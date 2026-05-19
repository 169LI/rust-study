/// 冒泡排序函数
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            // 比较相邻元素，若逆序则交换
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 测试用例 1：普通整数数组
    let mut numbers = vec![4, 2, 7, 1, 3];
    println!("原始整数数组: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("冒泡排序后: {:?}", numbers);

    // 测试用例 2：空数组
    let mut empty: Vec<i32> = vec![];
    println!("\n原始空数组: {:?}", empty);
    bubble_sort(&mut empty);
    println!("冒泡排序后: {:?}", empty);

    // 测试用例 3：已排序数组
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("\n原始已排序数组: {:?}", sorted);
    bubble_sort(&mut sorted);
    println!("冒泡排序后: {:?}", sorted);

    // 测试用例 4：字符串数组
    let mut words = vec!["苹果", "香蕉", "橙子", "葡萄"];
    println!("\n原始字符串数组: {:?}", words);
    bubble_sort(&mut words);
    println!("冒泡排序后: {:?}", words);
}