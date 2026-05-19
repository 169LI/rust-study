use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // 最大堆
    let mut max_heap = BinaryHeap::new();
    max_heap.push(5);
    max_heap.push(3);
    max_heap.push(7);
    max_heap.push(1);
    println!("Max heap pop: {:?}", max_heap.pop()); // 输出: Some(7)
    println!("Max heap peek: {:?}", max_heap.peek()); // 输出: Some(5)

    // 最小堆（使用 Reverse）
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(7));
    min_heap.push(Reverse(1));
    println!("Min heap pop: {:?}", min_heap.pop().map(|Reverse(x)| x)); // 输出: Some(1)
    println!("Min heap peek: {:?}", min_heap.peek().map(|Reverse(x)| x)); // 输出: Some(3)
}