fn main() {
    // 定义一个有序整数数组
    let numbers = [1, 3, 5, 7, 9, 11, 13];
    // 定义查找的目标值
    let target1 = 7; // 存在于数组中
    let target2 = 8; // 不存在于数组中

    // 1. 使用 `binary_search` 查找目标值
    // 返回 Result<usize, usize>：Ok(索引) 表示找到，Err(插入点) 表示未找到
    println!("测试查找目标值 {}", target1);
    match numbers.binary_search(&target1) {
        Ok(index) => println!("找到 {}，索引为 {}", target1, index),
        Err(pos) => println!("未找到 {}，应插入到索引 {}", target1, pos),
    }

    // 测试不存在的目标值
    println!("\n测试查找目标值 {}", target2);
    match numbers.binary_search(&target2) {
        Ok(index) => println!("找到 {}，索引为 {}", target2, index),
        Err(pos) => println!("未找到 {}，应插入到索引 {}", target2, pos),
    }

    // 2. 使用 `binary_search_by` 自定义比较逻辑
    // 适合非标准比较或复杂类型
    let target3 = 5;
    println!("\n使用 binary_search_by 查找 {}", target3);
    match numbers.binary_search_by(|&x| x.cmp(&target3)) {
        Ok(index) => println!("找到 {}，索引为 {}", target3, index),
        Err(pos) => println!("未找到 {}，应插入到索引 {}", target3, pos),
    }

    // 3. 使用 `partition_point` 查找分区点
    // 返回第一个满足条件的索引，适合范围查询
    let threshold = 6;
    let index = numbers.partition_point(|&x| x < threshold);
    println!("\n第一个 >= {} 的索引是 {}", threshold, index);
}