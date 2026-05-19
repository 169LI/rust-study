fn main() {
    // === String 创建 ===

    // 创建空 String
    let mut empty_string = String::new();
    println!("空 String: '{}', 长度: {}", empty_string, empty_string.len()); // 输出: '', 长度: 0

    // 从 &str 创建 String
    let mut greeting = String::from("Hello");
    println!("初始 String: {}", greeting); // 输出: Hello

    // 使用 to_string 创建 String
    let rust = "Rust".to_string();
    println!("通过 to_string 创建: {}", rust); // 输出: Rust

    // === String 追加操作 ===

    // 追加单个字符
    greeting.push('!'); // 在末尾添加 '!'
    println!("追加字符后: {}", greeting); // 输出: Hello!

    // 追加字符串切片
    greeting.push_str(", world");
    println!("追加字符串后: {}", greeting); // 输出: Hello!, world

    // === String 插入操作 ===

    // 在指定索引插入字符
    greeting.insert(5, ','); // 在索引 5 插入 ','
    println!("插入字符后: {}", greeting); // 输出: Hello,!, world

    // 在指定索引插入字符串
    greeting.insert_str(7, " Rust");
    println!("插入字符串后: {}", greeting); // 输出: Hello,! Rust, world

    // === String 删除与替换 ===

    // 清空 String
    empty_string.push_str("temp");
    empty_string.clear();
    println!("清空后: '{}', 是否为空: {}", empty_string, empty_string.is_empty()); // 输出: '', true

    // 替换内容
    let replaced = greeting.replace("world", "Rustaceans");
    println!("替换后: {}", replaced); // 输出: Hello,! Rust, Rustaceans

    // === String 查询 ===

    // 获取长度和容量
    println!("String 长度: {}, 容量: {}", greeting.len(), greeting.capacity()); // 输出: 长度: 14, 容量: >=14

    // 检查是否包含子串（需转为 &str）
    println!("是否包含 'Rust': {}", greeting.contains("Rust")); // 输出: true

    // === String 与 &str 交互 ===

    // 转换为 &str
    let slice: &str = greeting.as_str();
    println!("转换为 &str: {}", slice); // 输出: Hello,! Rust, world

    // 获取子字符串切片
    let subslice = &greeting[0..5];
    println!("子字符串切片: {}", subslice); // 输出: Hello

    // 从 &str 追加到 String
    let extra = " is awesome";
    greeting.push_str(extra);
    println!("追加 &str 后: {}", greeting); // 输出: Hello,! Rust, world is awesome

    // === String 迭代 ===

    // 按字符迭代
    print!("按字符迭代: ");
    for c in greeting.chars() {
        print!("{} ", c); // 输出: H e l l o , !   R u s t ,   w o r l d   i s   a w e s o m e
    }
    println!();

    // === 函数中使用 String 和 &str ===

    // 定义函数，接受 String 和 &str
    fn process_string(s: &String, slice: &str) {
        println!("处理 String: {}", s);
        println!("处理 &str: {}", slice);
    }

    // 调用函数
    process_string(&greeting, &greeting[0..5]); // 输出: Hello,! Rust, world is awesome 和 Hello
}