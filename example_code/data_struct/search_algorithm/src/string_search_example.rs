fn main() {
    // 定义主串和模式串
    let text = "ABABDABACDABABCABAB";
    let pattern = "ABABC";

    // 1. 使用 str::contains 检查模式串是否存在
    // 返回 bool，适合快速存在性检查
    println!("测试模式串 '{}' 是否存在", pattern);
    if text.contains(pattern) {
        println!("模式串 '{}' 存在于主串中", pattern);
    } else {
        println!("模式串 '{}' 不存在于主串中", pattern);
    }

    // 2. 使用 str::find 查找模式串第一次出现的位置
    // 返回 Option<usize>，返回第一个匹配的起始索引
    println!("\n测试查找模式串 '{}' 的第一次出现", pattern);
    match text.find(pattern) {
        Some(index) => println!("模式串 '{}' 出现在索引 {}", pattern, index),
        None => println!("模式串 '{}' 未找到", pattern),
    }

    // 3. 使用 str::rfind 查找模式串最后一次出现的位置
    // 返回 Option<usize>，从右向左查找
    println!("\n测试查找模式串 '{}' 的最后一次出现", pattern);
    match text.rfind(pattern) {
        Some(index) => println!("模式串 '{}' 最后出现在索引 {}", pattern, index),
        None => println!("模式串 '{}' 未找到", pattern),
    }

    // 4. 使用 str::match_indices 查找所有匹配位置
    // 返回迭代器，包含所有匹配的索引和子串
    println!("\n测试查找模式串 '{}' 的所有出现位置", pattern);
    let matches: Vec<_> = text.match_indices(pattern).collect();
    if matches.is_empty() {
        println!("模式串 '{}' 未找到", pattern);
    } else {
        for (index, matched) in matches {
            println!("模式串 '{}' 出现在索引 {}，匹配内容: {}", pattern, index, matched);
        }
    }
}