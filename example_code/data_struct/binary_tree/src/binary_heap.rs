struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    // 创建空堆
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    // 插入元素
    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    // 上滤
    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    // 删除堆顶（最小值）
    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        if self.data.len() == 1 {
            return self.data.pop();
        }
        let result = self.data[0];
        self.data[0] = self.data.pop().unwrap();
        self.sift_down(0);
        Some(result)
    }

    // 下滤
    fn sift_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.data.swap(index, smallest);
            index = smallest;
        }
    }

    // 查看堆顶
    fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    // 从数组构建堆
    fn from_vec(vec: Vec<i32>) -> Self {
        let mut heap = MinHeap { data: vec };
        for i in (0..heap.data.len() / 2).rev() {
            heap.sift_down(i);
        }
        heap
    }
}

fn main() {
    // 测试插入和删除
    let mut heap = MinHeap::new();
    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);
    println!("Min heap peek: {:?}", heap.peek()); // 输出: Some(1)
    println!("Min heap pop: {:?}", heap.pop()); // 输出: Some(1)
    println!("Min heap pop: {:?}", heap.pop()); // 输出: Some(3)

    // 测试从数组构建堆
    let vec = vec![5, 3, 7, 1, 4];
    let heap = MinHeap::from_vec(vec);
    println!("Built heap peek: {:?}", heap.peek()); // 输出: Some(1)
}