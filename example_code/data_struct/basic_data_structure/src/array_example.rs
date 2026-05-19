fn main() {
    // 1. 创建和初始化数组
    // 创建一个固定大小的数组并初始化
    let mut numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // 使用默认值初始化数组
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]

    // 2. 基本数组操作
    // 打印数组长度
    println!("Array length: {}", numbers.len()); // 输出 5

    // 检查数组是否为空（固定大小数组总是 false）
    println!("Is empty: {}", zeros.is_empty()); // 输出 false

    // 3. 访问和修改元素
    numbers[1] = 25; // 修改索引 1 的值
    println!("Element at index 1: {}", numbers[1]); // 输出 25

    // 交换元素
    numbers.swap(0, 4); // 交换第一个和最后一个元素
    println!("After swap: {:?}", numbers); // [50, 25, 30, 40, 10]

    // 4. 迭代操作
    // 简单遍历
    println!("All elements:");
    for &num in numbers.iter() {
        print!("{} ", num);
    }
    println!(); // 输出: 50 25 30 40 10

    // 带索引的迭代
    println!("Elements with indices:");
    for (i, &num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }

    // 5. 切片操作
    let slice: &[i32] = &numbers[1..4]; // 切片包含 [25, 30, 40]
    println!("Slice from index 1 to 3: {:?}", slice);

    // 6. 搜索和查询
    // 查找最大值
    let max = numbers.iter().max().unwrap_or(&0);
    println!("Maximum value: {}", max); // 输出 50

    // 查找特定值的位置
    let position = numbers.iter().position(|&x| x == 30);
    println!("Position of 30: {:?}", position); // 输出 Some(2)

    // 检查是否包含某个值
    let contains_25 = numbers.iter().any(|&x| x == 25);
    println!("Contains 25: {}", contains_25); // 输出 true

    // 7. 排序和反转
    let mut sortable = numbers;
    sortable.sort(); // 升序排序
    println!("Sorted array: {:?}", sortable); // 输出: [10, 25, 30, 40, 50]

    sortable.reverse();
    println!("Reversed array: {:?}", sortable); // 输出: [50, 40, 30, 25, 10]

    // 8. 转换操作
    // 使用 map 转换元素
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled elements: {:?}", doubled); // 输出: [100, 50, 60, 80, 20]

    // 数组转换为 Vec
    let vec_from_array = numbers.to_vec();
    println!("Vec from array: {:?}", vec_from_array); // 输出: [50, 25, 30, 40, 10]
                                                      // 9. 数学运算
                                                      // 求和与乘积
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("Sum: {}, Product: {}", sum, product);

    // 10. 高级操作
    // 过滤元素
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // 窗口操作
    println!("Sliding windows:");
    for window in numbers.windows(2) {
        println!("{:?}", window); // [50,25], [25,30], etc.
    }

    // 11. 多维数组
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("Matrix:");
    for row in matrix.iter() {
        println!("{:?}", row);
    }

    // 12. 数组填充
    numbers.fill(42); // 所有元素变为 42
    println!("Filled array: {:?}", numbers); // [42, 42, 42, 42, 42]

    // 13. 旋转数组
    let mut rotatable = [1, 2, 3, 4, 5];
    rotatable.rotate_left(2);
    println!("Rotated left by 2: {:?}", rotatable); // [3, 4, 5, 1, 2]

    // 14. 数组比较
    let a = [1, 2, 3];
    let b = [1, 2, 3];
    println!("Arrays equal: {}", a == b); // true
}
