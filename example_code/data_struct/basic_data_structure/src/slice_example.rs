fn main() {
    // === 字符串切片 (&str) 演示 ===

    // 从字符串字面量创建字符串切片
    let lit = "Hello, Rust!";
    let str_slice: &str = &lit[0..5]; // 取 "Hello"
    println!("字符串切片: {}", str_slice); // 输出: Hello

    // 从 String 创建字符串切片
    let s = String::from("Welcome to Rust");
    let str_slice_from_string: &str = &s[0..7]; // 取 "Welcome"
    println!("从 String 创建的切片: {}", str_slice_from_string); // 输出: Welcome

    // 字符串切片操作：长度、判空
    println!("切片长度: {}", str_slice.len()); // 输出: 5
    println!("切片是否为空: {}", str_slice.is_empty()); // 输出: false

    // 字符串切片方法：包含、分割、清理空白
    println!("是否包含 'lo': {}", str_slice.contains("lo")); // 输出: true
    println!("是否以 'He' 开头: {}", str_slice.starts_with("He")); // 输出: true
    let words: Vec<&str> = lit.split(" ").collect(); // 按空格分割
    println!("分割结果: {:?}", words); // 输出: ["Hello,", "Rust!"]

    // 字符串切片迭代：按字符
    print!("按字符迭代: ");
    for c in str_slice.chars() {
        print!("{} ", c); // 输出: H e l l o
    }
    println!();

    // === 数组切片 (&[T]) 演示 ===

    // 从 Vec 创建数组切片
    let numbers = vec![10, 20, 30, 40, 50];
    let num_slice: &[i32] = &numbers[1..4]; // 取 [20, 30, 40]
    println!("数组切片: {:?}", num_slice); // 输出: [20, 30, 40]

    // 从数组创建数组切片
    let arr = [1, 2, 3, 4, 5];
    let arr_slice: &[i32] = &arr[0..3]; // 取 [1, 2, 3]
    println!("从数组创建的切片: {:?}", arr_slice); // 输出: [1, 2, 3]

    // 数组切片操作：长度、判空
    println!("数组切片长度: {}", num_slice.len()); // 输出: 3
    println!("数组切片是否为空: {}", num_slice.is_empty()); // 输出: false

    // 数组切片迭代
    print!("按元素迭代: ");
    for &x in num_slice.iter() {
        print!("{} ", x); // 输出: 20 30 40
    }
    println!();

    // 数组切片操作：求和
    let sum: i32 = num_slice.iter().sum();
    println!("切片元素和: {}", sum); // 输出: 90

    // === 可变数组切片 (&mut [T]) 演示 ===

    // 创建可变 Vec 并获取可变切片
    let mut mutable_numbers = vec![100, 200, 300, 400];
    let mut_slice: &mut [i32] = &mut mutable_numbers[1..3]; // 取 [200, 300]
    println!("可变切片（修改前）: {:?}", mut_slice); // 输出: [200, 300]

    // 修改可变切片中的元素
    mut_slice[0] = 250; // 将 200 改为 250
    mut_slice[1] = 350; // 将 300 改为 350
    println!("可变切片（修改后）: {:?}", mut_slice); // 输出: [250, 350]
    println!("原 Vec（受影响）: {:?}", mutable_numbers); // 输出: [100, 250, 350, 400]

    // === 切片作为函数参数 ===
    // 定义函数，接受字符串切片和数组切片
    fn process_slices(str_slice: &str, num_slice: &[i32]) {
        println!("处理字符串切片: {}", str_slice);
        println!("处理数组切片: {:?}", num_slice);
    }

    // 调用函数，传入不同来源的切片
    process_slices(&s[0..7], &numbers[0..2]); // 输出: Welcome 和 [10, 20]
    process_slices("Rust", &arr[1..3]); // 输出: Rust 和 [2, 3]
}