use std::collections::VecDeque;
// 定义一个结构体，使用 VecDeque 模拟栈
struct Stack<T> {
    queue: VecDeque<T>,
}

impl<T: Clone> Stack<T> {
    // 创建空栈
    fn new() -> Self {
        Stack {
            queue: VecDeque::new(),
        }
    }

    // 压入元素（直接添加到队列尾部）
    fn push(&mut self, value: T) {
        self.queue.push_back(value);
    }

    // 弹出栈顶元素（重排队列使最新元素在头部）
    fn pop(&mut self) -> Option<T> {
        // 如果队列为空，返回 None
        if self.queue.is_empty() {
            return None;
        }

        // 将队列前 n-1 个元素出队并重新入队，使最新元素移到头部
        let len = self.queue.len();
        for _ in 0..len - 1 {
            if let Some(item) = self.queue.pop_front() {
                self.queue.push_back(item);
            }
        }

        // 弹出头部元素（即栈顶）
        self.queue.pop_front()
    }

    // 查看栈顶元素（类似 pop 但不移除）
    fn peek(&self) -> Option<&T> {
        self.queue.back()
    }

    // 检查栈是否为空
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    // 获取栈大小
    fn len(&self) -> usize {
        self.queue.len()
    }
}

fn main() {

    // 创建一个栈实例
    let mut stack: Stack<i32> = Stack::new();
    println!("初始栈: {:?}", stack.queue); // 输出: []

    // === 压入元素 ===
    stack.push(10); // 压入 10
    stack.push(20); // 压入 20
    stack.push(30); // 压入 30
    println!("压入 10, 20, 30 后的栈: {:?}", stack.queue); // 输出: [10, 20, 30]

    // === 查看栈顶 ===
    if let Some(top) = stack.peek() {
        println!("栈顶元素: {}", top); // 输出: 30
    } else {
        println!("栈为空");
    }

    // === 弹出元素 ===
    if let Some(value) = stack.pop() {
        println!("弹出栈顶元素: {}", value); // 输出: 30
    }
    println!("弹出后的栈: {:?}", stack.queue); // 输出: [20, 10]

    // 再次弹出
    if let Some(value) = stack.pop() {
        println!("再次弹出栈顶元素: {}", value); // 输出: 20
    }
    println!("再次弹出后的栈: {:?}", stack.queue); // 输出: [10]

    // === 检查状态 ===
    println!("栈是否为空: {}", stack.is_empty()); // 输出: false
    println!("栈元素数量: {}", stack.len()); // 输出: 1

    // === 清空栈 ===
    stack.pop(); // 弹出最后一个元素
    println!("清空后的栈: {:?}", stack.queue); // 输出: []
    println!("清空后是否为空: {}", stack.is_empty()); // 输出: true

    // === 函数中操作栈 ===
    fn stack_operations(stack: &mut Stack<i32>, values: &[i32]) {
        // 压入一组元素
        for &value in values {
            stack.push(value);
        }
        println!("函数内压入后的栈: {:?}", stack.queue);

        // 弹出并打印栈顶
        if let Some(value) = stack.pop() {
            println!("函数内弹出: {}", value);
        }
    }

    // 测试函数
    let values = [100, 200, 300];
    stack_operations(&mut stack, &values);
    println!("函数操作后的栈: {:?}", stack.queue); // 输出: [100, 200]
}