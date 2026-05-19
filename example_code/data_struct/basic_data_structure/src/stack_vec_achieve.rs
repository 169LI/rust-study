fn main() {
    // 创建一个 Vec<i32> 作为栈，预分配容量以优化性能
    let mut stack: Vec<i32> = Vec::with_capacity(10);
    println!("初始栈: {:?}", stack); // 输出: []

    // === 压入元素（push） ===

    // 将元素压入栈顶
    stack.push(100); // 压入 100
    stack.push(200); // 压入 200
    stack.push(300); // 压入 300
    println!("压入 100, 200, 300 后的栈: {:?}", stack); // 输出: [100, 200, 300]

    // === 查看栈顶（last） ===

    // 使用 last 查看栈顶元素（类似 peek）
    if let Some(top) = stack.last() {
        println!("栈顶元素: {}", top); // 输出: 300
    } else {
        println!("栈为空");
    }

    // === 弹出元素（pop） ===

    // 使用 pop 移除并返回栈顶元素
    if let Some(value) = stack.pop() {
        println!("弹出栈顶元素: {}", value); // 输出: 300
    }
    println!("弹出后的栈: {:?}", stack); // 输出: [100, 200]

    // === 修改栈顶（last_mut） ===

    // 使用 last_mut 修改栈顶元素
    if let Some(top) = stack.last_mut() {
        *top = 250; // 将栈顶 200 改为 250
        println!("修改栈顶后的栈: {:?}", stack); // 输出: [100, 250]
    }

    // === 栈状态查询 ===

    // 检查栈是否为空
    println!("栈是否为空: {}", stack.is_empty()); // 输出: false

    // 获取栈的元素数量
    println!("栈元素数量: {}", stack.len()); // 输出: 2

    // === 清空栈（clear） ===

    // 清空栈
    stack.clear();
    println!("清空后的栈: {:?}", stack); // 输出: []
    println!("清空后是否为空: {}", stack.is_empty()); // 输出: true

    // === 函数中操作栈 ===

    // 定义函数，模拟栈的压入和弹出
    fn stack_operations(stack: &mut Vec<i32>, to_push: &[i32]) {
        // 压入一组元素
        for &value in to_push {
            stack.push(value);
        }
        println!("函数内压入后的栈: {:?}", stack); // 输出栈内容

        // 弹出并返回栈顶元素
        if let Some(value) = stack.pop() {
            println!("函数内弹出: {}", value);
        }
    }

    // 调用函数，测试栈操作
    let values = [1, 2, 3];
    stack_operations(&mut stack, &values);
    println!("函数操作后的栈: {:?}", stack); // 输出: [1, 2]

    // === 再次压入并验证容量 ===

    // 压入元素，验证容量是否影响
    stack.push(400);
    println!("再次压入 400 后的栈: {:?}", stack); // 输出: [1, 2, 400]
    println!("当前容量: {}", stack.capacity()); // 输出: >=10（因预分配）
}