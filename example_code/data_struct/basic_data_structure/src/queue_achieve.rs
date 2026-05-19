// 队列结构体，基于 Vec 实现
#[derive(Debug)]
struct Queue<T> {
    cap: usize,   // 最大容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    // 创建指定容量的新队列
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size), // 预分配容量，避免频繁重新分配
        }
    }

    // 入队：将元素添加到队列尾部
    // 如果队列已满，返回错误
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val); // 尾部入队，O(1)
        Ok(())
    }

    // 出队：从队列头部移除并返回元素
    // 如果队列为空，返回 None
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0)) // 头部出队，O(n)
        }
    }

    // 检查队列是否为空
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    // 返回队列当前元素数量
    fn size(&self) -> usize {
        self.data.len()
    }

    // 查看队列头部元素（不移除）
    fn peek(&self) -> Option<&T> {
        self.data.first() // O(1)
    }
}

fn main() {
    // 创建容量为 3 的队列
    let mut q = Queue::new(3);

    // 入队操作
    let r1 = q.enqueue(1); // 入队 1，q = [1]
    let r2 = q.enqueue(2); // 入队 2，q = [1, 2]
    let r3 = q.enqueue(3); // 入队 3，q = [1, 2, 3]
    let r4 = q.enqueue(4); // 尝试入队 4，队列已满

    // 检查入队结果
    println!("入队结果: r1={:?}, r2={:?}, r3={:?}, r4={:?}", r1, r2, r3, r4);
    if let Err(error) = r4 {
        println!("入队错误: {}", error); // 打印错误: No space available
    }

    // 出队操作
    if let Some(data) = q.dequeue() {
        println!("出队数据: {}", data); // 出队 1
    } else {
        println!("队列为空");
    }

    // 再次出队
    if let Some(data) = q.dequeue() {
        println!("出队数据: {}", data); // 出队 2
    } else {
        println!("队列为空");
    }

    // 查看队列状态
    println!("队列大小: {}", q.size()); // 1
    println!("队列是否为空: {}", q.is_empty()); // false
    println!("队列头部元素: {:?}", q.peek()); // Some(3)
    println!("队列内容: {:?}", q); // [3]

    // 清空队列
    while q.dequeue().is_some() {} // 出队所有元素
    println!("清空后大小: {}", q.size()); // 0
    println!("清空后是否为空: {}", q.is_empty()); // true
    println!("清空后内容: {:?}", q); // []

    // 尝试从空队列出队
    if let Some(data) = q.dequeue() {
        println!("出队数据: {}", data);
    } else {
        println!("队列为空"); // 打印: 队列为空
    }
}