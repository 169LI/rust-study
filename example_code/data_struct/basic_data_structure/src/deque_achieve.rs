// 双端队列（Deque）实现
#[derive(Debug)]
struct Deque<T> {
    cap: usize,   // 最大容量
    data: Vec<T>, // 数据存储容器（Vec 末尾为队首，首部为队尾）
}

impl<T> Deque<T> {
    // 创建一个指定容量的新 Deque
    fn new(cap: usize) -> Self {
        Deque {
            cap,
            data: Vec::with_capacity(cap), // 预分配指定容量
        }
    }

    // 在队首（Vec 末尾）添加元素
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space available".to_string()); // 容量已满
        }
        self.data.push(val); // O(1) 操作
        Ok(())
    }

    // 在队尾（Vec 首部）添加元素
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space available".to_string()); // 容量已满
        }
        self.data.insert(0, val); // O(n) 操作
        Ok(())
    }

    // 从队首（Vec 末尾）移除并返回元素
    fn remove_front(&mut self) -> Option<T> {
        self.data.pop() // O(1) 操作，若为空返回 None
    }

    // 从队尾（Vec 首部）移除并返回元素
    fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None // 队列为空
        } else {
            Some(self.data.remove(0)) // O(n) 操作
        }
    }

    // 检查队列是否为空
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    // 返回当前元素数量
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    // 创建一个容量为 4 的 Deque
    let mut d = Deque::new(4);

    // 向队首添加 1 和 2，向队尾添加 3 和 4
    d.add_front(1).unwrap();
    d.add_front(2).unwrap();
    d.add_rear(3).unwrap();
    d.add_rear(4).unwrap();

    // 尝试向队首添加 5（因容量满将失败）
    if let Err(error) = d.add_front(5) {
        println!("add_front error: {error}");
    }

    // 从队尾移除元素并打印
    if let Some(data) = d.remove_rear() {
        println!("remove data from rear: {data}");
    } else {
        println!("empty deque");
    }

    // 从队首移除元素并打印
    if let Some(data) = d.remove_front() {
        println!("remove data from front: {data}");
    } else {
        println!("empty deque");
    }

    // 打印队列大小、是否为空以及内容
    println!("size: {}, is_empty: {}", d.size(), d.is_empty());
    println!("content: {:?}", d);
}