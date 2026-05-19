fn main() {
    // 定义一个函数，将十进制数转换为指定进制
    fn to_base(n: u32, base: u32) -> String {
        // 确保进制在 2 到 16 之间
        if !(2..=16).contains(&base) {
            return String::from("无效的进制");
        }

        // 处理特殊情况：输入为 0
        if n == 0 {
            return String::from("0");
        }

        // 创建一个 Vec<char> 作为栈，存储余数
        let mut stack: Vec<char> = Vec::new();
        let digits = "0123456789ABCDEF"; // 进制字符表
        let mut num = n;

        // 不断除以进制，余数压入栈
        while num > 0 {
            let remainder = (num % base) as usize;
            stack.push(digits.chars().nth(remainder).unwrap());
            num /= base;
        }

        // 从栈顶弹出字符，构建结果字符串
        let mut result = String::new();
        while let Some(digit) = stack.pop() {
            result.push(digit);
        }

        result
    }

    // 测试用例：转换不同数字到不同进制
    let test_cases = [
        (42, 2),   // 十进制 42 转二进制
        (42, 8),   // 十进制 42 转八进制
        (42, 16),  // 十进制 42 转十六进制
        (255, 16), // 十进制 255 转十六进制
        (0, 10),   // 十进制 0 转十进制
        (42, 17),  // 无效进制
    ];

    // 遍历测试用例并打印结果
    for &(num, base) in test_cases.iter() {
        println!("十进制 {} 转换为 {} 进制: {}", num, base, to_base(num, base));
    }
}