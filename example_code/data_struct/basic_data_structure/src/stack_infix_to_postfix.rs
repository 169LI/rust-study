fn main() {
    // 定义一个函数，将中缀表达式转换为后缀表达式
    fn infix_to_postfix(expr: &str) -> String {
        // 创建一个 Vec<char> 作为栈，存储运算符
        let mut stack: Vec<char> = Vec::new();
        // 创建一个 String 存储结果
        let mut result = String::new();

        // 定义运算符优先级
        fn precedence(op: char) -> i32 {
            match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                _ => 0,
            }
        }

        // 遍历表达式中的每个字符
        for c in expr.chars() {
            match c {
                // 遇到操作数（字母或数字），直接添加到结果
                'A'..='Z' | 'a'..='z' | '0'..='9' => result.push(c),
                // 遇到左括号，压入栈
                '(' => stack.push(c),
                // 遇到右括号，弹出栈中运算符直到匹配左括号
                ')' => {
                    while let Some(top) = stack.last() {
                        if *top == '(' {
                            stack.pop(); // 弹出 '('
                            break;
                        }
                        result.push(stack.pop().unwrap());
                    }
                }
                // 遇到运算符，比较优先级并处理
                '+' | '-' | '*' | '/' => {
                    // 弹出栈中优先级更高或相等的运算符
                    while let Some(top) = stack.last() {
                        if *top != '(' && precedence(*top) >= precedence(c) {
                            result.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    // 当前运算符压入栈
                    stack.push(c);
                }
                // 忽略空格
                ' ' => {}
                // 其他字符视为无效
                _ => {}
            }
        }

        // 弹出栈中剩余的运算符
        while let Some(op) = stack.pop() {
            if op != '(' {
                result.push(op);
            }
        }

        result
    }

    // 测试用例
    let test_cases = [
        "A + B * C",          // 简单表达式
        "(A + B) * C",        // 带括号
        "A + B * C + D",      // 多运算符
        "(A + B) * (C + D)",  // 多括号
        "A * B + C / D",      // 不同优先级
    ];

    // 遍历测试用例并打印结果
    for test in test_cases.iter() {
        println!("中缀表达式: {}, 后缀表达式: {}", test, infix_to_postfix(test));
    }
}