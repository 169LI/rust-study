use std::collections::VecDeque;

fn main() {
    // 1. 创建双端队列
    let mut deque: VecDeque<i32> = VecDeque::new(); // 空队列
    let mut deque2: VecDeque<i32> = VecDeque::from([1, 2, 3]); // 初始化带元素
    let mut deque3: VecDeque<i32> = VecDeque::with_capacity(10); // 指定容量

    // 2. 添加元素
    deque.push_back(10); // 尾部入队: [10]
    deque.push_back(20); // 尾部入队: [10, 20]
    deque.push_front(5); // 头部入队: [5, 10, 20]
    deque.extend([30, 40]); // 尾部批量添加: [5, 10, 20, 30, 40]

    // 3. 移除元素
    let front = deque.pop_front(); // 头部出队: Some(5), deque = [10, 20, 30, 40]
    let back = deque.pop_back(); // 尾部出队: Some(40), deque = [10, 20, 30]

    // 4. 访问元素
    let front_elem = deque.front().copied(); // 查看头部: Some(10), 复制值
    let back_elem = deque.back().copied(); // 查看尾部: Some(30), 复制值
    let second_elem = deque.get(1).copied(); // 按索引访问: Some(20), 复制值

    // 5. 检查状态
    let is_empty = deque.is_empty(); // 是否为空: false
    let len = deque.len(); // 长度: 3
    let cap = deque.capacity(); // 容量: >= 3

    // 6. 迭代
    print!("队列元素: ");
    for x in &deque {
        print!("{} ", x); // 不可变迭代: 10 20 30
    }
    println!();

    for x in &mut deque {
        *x += 100; // 可变迭代，修改元素: [110, 120, 130]
    }

    // 7. 其他操作
    deque2.clear(); // 清空队列: []
    deque.shrink_to_fit(); // 缩减容量以适应数据
    deque3.push_back(100); // deque3: [100]

    // 8. 打印结果
    println!("当前队列: {:?}", deque); // [110, 120, 130]
    println!("deque2 (清空后): {:?}", deque2); // []
    println!("deque3: {:?}", deque3); // [100]
    println!("出队头部: {:?}", front); // Some(5)
    println!("出队尾部: {:?}", back); // Some(40)
    println!("头部元素: {:?}", front_elem); // Some(10)
    println!("尾部元素: {:?}", back_elem); // Some(30)
    println!("第二个元素: {:?}", second_elem); // Some(20)
    println!("是否为空: {}", is_empty); // false
    println!("长度: {}", len); // 3
    println!("容量: {}", cap); // >= 3
}