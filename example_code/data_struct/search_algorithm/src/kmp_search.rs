/// KMP 算法查找模式串在主串中的位置

fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    // 将字符串转换为字符向量，处理 UTF-8
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    // 空串处理
    if pattern.is_empty() || text.is_empty() {
        return None;
    }

    // 计算部分匹配表（LPS 数组）
    let lps = compute_lps(&pattern);

    let mut i = 0; // 主串索引
    let mut j = 0; // 模式串索引
    while i < text.len() {
        if pattern[j] == text[i] {
            // 字符匹配，继续比较
            i += 1;
            j += 1;
        }
        if j == pattern.len() {
            // 模式串匹配完成，返回起始位置
            return Some(i - j);
        } else if i < text.len() && pattern[j] != text[i] {
            // 失配时，利用 LPS 表跳跃
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    None // 未找到
}

/// 计算模式串的部分匹配表（Longest Proper Prefix which is also Suffix）
fn compute_lps(pattern: &[char]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut len = 0; // 前缀长度
    let mut i = 1; // 后缀索引

    while i < pattern.len() {
        if pattern[i] == pattern[len] {
            // 匹配，扩展前缀
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            // 失配，回退到上一个前缀
            len = lps[len - 1];
        } else {
            // 无前缀匹配
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

fn main() {
    // 定义测试用的主串和模式串
    let text = "ABABDABACDABABCABAB";
    let pattern1 = "ABABC"; // 存在于主串
    let pattern2 = "XYZ"; // 不存在于主串
    let pattern3 = ""; // 空模式串

    // 测试用例 1：查找存在的模式串
    println!("测试查找模式串 '{}'", pattern1);
    match kmp_search(text, pattern1) {
        Some(index) => println!("模式串 '{}' 出现在索引 {}", pattern1, index),
        None => println!("模式串 '{}' 未找到", pattern1),
    }

    // 测试用例 2：查找不存在的模式串
    println!("\n测试查找模式串 '{}'", pattern2);
    match kmp_search(text, pattern2) {
        Some(index) => println!("模式串 '{}' 出现在索引 {}", pattern2, index),
        None => println!("模式串 '{}' 未找到", pattern2),
    }

    // 测试用例 3：查找空模式串
    println!("\n测试查找空模式串");
    match kmp_search(text, pattern3) {
        Some(index) => println!("空模式串出现在索引 {}", index),
        None => println!("空模式串未找到"),
    }

    // 测试用例 4：查找短模式串
    let short_pattern = "ABA";
    println!("\n测试查找短模式串 '{}'", short_pattern);
    match kmp_search(text, short_pattern) {
        Some(index) => println!("模式串 '{}' 出现在索引 {}", short_pattern, index),
        None => println!("模式串 '{}' 未找到", short_pattern),
    }
}