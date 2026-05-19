use std::collections::VecDeque;

fn main() {
    // 1. 创建队列
    // 使用 VecDeque 创建一个空的队列
    let mut queue: VecDeque<i32> = VecDeque::new();
    // 使用 vec! 宏初始化一个带元素的队列
    let mut queue2: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    // 指定初始容量创建队列（避免频繁重新分配）
    let queue3: VecDeque<i32> = VecDeque::with_capacity(10);

    // 2. 入队（添加元素）
    // 在队列尾部添加元素（push_back）
    queue.push_back(10); // queue = [10]
    queue.push_back(20); // queue = [10, 20]
    queue.push_back(30); // queue = [10, 20, 30]

    // 在队列头部添加元素（push_front，展示 VecDeque 的双端特性）
    queue.push_front(5); // queue = [5, 10, 20, 30]

    // 3. 出队（移除元素）
    // 从队列头部移除并返回元素（pop_front，FIFO）
    let front = queue.pop_front(); // front = Some(5), queue = [10, 20, 30]
    // 从队列尾部移除（pop_back，非典型队列操作）
    let back = queue.pop_back(); // back = Some(30), queue = [10, 20]

    // 4. 访问元素
    // 获取队列头部元素（不移除）
    let front_elem = queue.front().copied(); // front_elem = Some(10)
    // 获取队列尾部元素（不移除）
    let back_elem = queue.back().copied(); // back_elem = Some(20)
    // 通过索引访问（小心越界）
    let first = queue.get(0).copied(); // first = Some(10)

    // 5. 检查队列状态
    // 检查队列是否为空
    let is_empty = queue.is_empty(); // is_empty = false
    // 获取队列长度
    let len = queue.len(); // len = 2
    // 获取队列容量
    let cap = queue.capacity(); // cap >= 2（取决于分配策略）

    // 6. 迭代队列
    // 不可变迭代
    print!("队列元素: ");
    for x in &queue {
        print!("{} ", x); // 打印: 10 20
    }
    println!();

    // 可变迭代（修改元素）
    for x in &mut queue {
        *x += 100; // queue = [110, 120]
    }

    // 7. 其他操作
    // 清空队列
    queue2.clear(); // queue2 = []
    // 缩减容量（释放多余内存）
    queue.shrink_to_fit();
    // 批量添加元素（扩展）
    queue.extend([300, 400]); // queue = [110, 120, 300, 400]

    // 8. 综合示例：打印结果
    println!("当前队列: {:?}", queue); // [110, 120, 300, 400]
    println!("queue2 (清空后): {:?}", queue2); // []
    println!("queue3: {:?}", queue3); // []
    println!("出队元素: {:?}", front); // Some(5)
    println!("尾部出队: {:?}", back); // Some(30)
    println!("头部元素: {:?}", front_elem); // Some(10)
    println!("尾部元素: {:?}", back_elem); // Some(20)
    println!("第一个元素: {:?}", first); // Some(10)
    println!("队列是否为空: {}", is_empty); // false
    println!("队列长度: {}", len); // 2
    println!("队列容量: {}", cap); // >= 2
}
