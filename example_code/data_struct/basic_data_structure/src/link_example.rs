use std::collections::LinkedList;

fn main() {
    // 创建一个空的 LinkedList，存储 i32 类型元素
    let mut list: LinkedList<i32> = LinkedList::new();

    // 使用 push_back 在尾部添加元素
    list.push_back(1);
    list.push_back(2);

    // 使用 push_front 在头部添加元素
    list.push_front(0);

    // 检查链表长度
    println!("链表长度: {}", list.len()); // 应打印 3

    // 获取头部元素
    if let Some(value) = list.front() {
        println!("头部元素: {}", value); // 应打印 0
    }

    // 移除并获取头部元素
    if let Some(value) = list.pop_front() {
        println!("移除的头部元素: {}", value); // 应打印 0
    }

    // 遍历链表并打印所有元素
    print!("链表元素: ");
    for value in list.iter() {
        print!("{} ", value); // 应打印 1 2
    }
    println!();

    // 检查链表是否为空
    println!("链表是否为空: {}", list.is_empty()); // 应打印 false

    // 清空链表
    list.clear();
    println!("清空后链表长度: {}", list.len()); // 应打印 0
}