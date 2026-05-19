fn main() {
    // 1. 创建 Vec
    // 使用 vec! 宏初始化一个包含元素的 Vec
    let mut v: Vec<i32> = vec![1, 2, 3];
    // 创建一个空 Vec，预分配容量为 10
    let mut v2: Vec<i32> = Vec::with_capacity(10);
    // 从迭代器创建 Vec
    let v3: Vec<i32> = (4..=6).collect();

    // 2. 添加元素
    // 在 v 末尾添加单个元素
    v.push(4); // v = [1, 2, 3, 4]
    // 在 v2 中插入元素到指定索引
    v2.insert(0, 10); // v2 = [10]
    // 批量添加多个元素
    v.extend([5, 6]); // v = [1, 2, 3, 4, 5, 6]

    // 3. 移除元素
    // 移除并返回末尾元素
    let popped = v.pop(); // popped = Some(6), v = [1, 2, 3, 4, 5]
    // 移除指定索引的元素
    v.remove(1); // v = [1, 3, 4, 5]
    // 清空 v2
    v2.clear(); // v2 = []

    // 4. 访问元素
    // 使用索引访问（可能 panic）
    let first = v[0]; // first = 1
    // 安全访问，复制值而非持有引用
    let second = *v.get(1).unwrap(); // second = 3（复制值）
    // 访问最后一个元素
    let last = *v.last().unwrap(); // last = 5（复制值）

    // 5. 修改元素
    // 修改指定索引的元素
    v[2] = 40; // v = [1, 3, 40, 5]
    // 交换两个元素
    v.swap(0, 3); // v = [5, 3, 40, 1]

    // 6. 切片操作
    // 获取 v 的子切片，并在可变操作前使用
    let slice = &v[1..3]; // slice = [3, 40]
    println!("slice: {:?}", slice); // 立即打印 slice，结束借用

    // 7. 迭代 Vec
    // 不可变迭代
    print!("v: ");
    for x in &v {
        print!("{} ", x); // 打印: 5 3 40 1
    }
    println!();

    // 可变迭代，修改每个元素
    for x in &mut v {
        *x += 10; // v = [15, 13, 50, 11]
    }

    // 8. 容量管理
    // 查看当前长度和容量
    let len = v.len(); // len = 4
    let cap = v.capacity(); // cap >= 4
    // 缩减容量以适应数据
    v.shrink_to_fit();

    // 9. 综合示例：打印结果
    println!("v: {:?}", v); // v: [15, 13, 50, 11]
    println!("v3: {:?}", v3); // v3: [4, 5, 6]
    println!("popped: {:?}", popped); // popped: Some(6)
    println!("first: {}, second: {}, last: {}", first, second, last); // first: 1, second: 3, last: 5
    println!("len: {}, capacity: {}", len, cap); // len: 4, capacity: >=4
}