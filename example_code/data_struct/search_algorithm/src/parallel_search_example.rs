// 添加 rayon 依赖到 Cargo.toml: [dependencies] rayon = "1.5"
use rayon::prelude::*;

/// 并行查找函数（适用于排序数组）
fn parallel_search(arr: &[i32], target: i32) -> i32 {
    if arr.len() <= 1 {
        if arr.is_empty() || arr[0] != target {
            return -1;
        }
        return 0;
    }

    // 使用 rayon 进行并行搜索
    let result = (0..arr.len()).into_par_iter().find_any(|&i| arr[i] == target);

    match result {
        Some(index) => index as i32,
        None => -1,
    }
}

fn main() {
    // 测试用例 1：大数组
    let arr1 = (1..=1000).collect::<Vec<i32>>();
    let targets1 = [500, 1001, 0];
    println!("测试用例 1 - 大数组 (1 到 1000): {:?}", &arr1[0..10]); // 仅显示前 10 个
    for &target in targets1.iter() {
        let result = parallel_search(&arr1, target);
        println!("查找 {}: 索引 = {}", target, result);
    }

    // 测试用例 2：空数组
    let arr2: Vec<i32> = vec![];
    let target2 = 5;
    println!("\n测试用例 2 - 空数组: {:?}", arr2);
    let result2 = parallel_search(&arr2, target2);
    println!("查找 {}: 索引 = {}", target2, result2);

    // 测试用例 3：包含重复元素的数组
    let arr3 = vec![1, 2, 2, 3, 4, 4, 5];
    let targets3 = [2, 6, 4];
    println!("\n测试用例 3 - 重复元素数组: {:?}", arr3);
    for &target in targets3.iter() {
        let result = parallel_search(&arr3, target);
        println!("查找 {}: 索引 = {}", target, result);
    }
}