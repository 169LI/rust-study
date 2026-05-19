fn main() {
    // 定义一个函数，检查括号是否匹配
    fn is_valid_brackets(s: &str) -> bool {
        // 创建一个 Vec<char> 作为栈，存储左括号
        let mut stack: Vec<char> = Vec::new();

        // 遍历字符串中的每个字符
        for c in s.chars() {
            match c {
                // 遇到左括号，压入栈
                '(' | '[' | '{' => stack.push(c),
                // 遇到右括号，检查栈顶并匹配
                ')' => {
                    // 如果栈为空或栈顶不是 '('，返回 false
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    // 如果栈为空或栈顶不是 '['，返回 false
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    // 如果栈为空或栈顶不是 '{'，返回 false
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                // 忽略非括号字符
                _ => {}
            }
        }

        // 检查栈是否为空（为空表示所有括号匹配）
        stack.is_empty()
    }

    // 测试用例
    let test_cases = [
        "()",           // 正确
        "{}[]",         // 正确
        "{[()]}",       // 正确
        "(]",           // 错误
        "([)]",         // 错误
        "{[}",          // 错误
        ""              // 正确（空字符串）
    ];

    // 遍历测试用例并打印结果
    for test in test_cases.iter() {
        println!("输入: {}, 是否匹配: {}", test, is_valid_brackets(test));
    }
}