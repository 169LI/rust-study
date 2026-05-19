use std::rc::Rc;
use std::cell::RefCell;

// 定义节点链接类型，使用 Rc 和 RefCell 实现共享和内部可变性
type Link = Option<Rc<RefCell<Node>>>;

// 节点结构，包含数据和指向下一个节点的链接
#[derive(Debug)]
struct Node {
    data: usize,  // 节点数据（顶点编号）
    next: Link,   // 指向下一个邻接节点的链接
}

impl Node {
    // 创建新节点
    fn new(data: usize) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

// 图结构，使用链表表示邻接表
#[derive(Debug)]
struct Graph {
    first: Link,  // 第一个邻接节点
    last: Link,   // 最后一个邻接节点
}

impl Graph {
    // 创建空图
    fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    // 判断图是否为空
    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    // 获取第一个节点（返回克隆以保持共享）
    fn get_first(&self) -> Link {
        self.first.clone()
    }

    // 打印图的邻接节点
    fn print_node(&self) {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            print!("[{}]", val.borrow().data);
            curr = val.borrow().next.clone();
        }
        print!("\n");
    }

    // 插入新节点到图的末尾
    fn insert(&mut self, data: usize) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        if self.is_empty() {
            self.first = Some(node.clone());
            self.last = Some(node);
        } else {
            if let Some(last) = self.last.as_mut() {
                last.borrow_mut().next = Some(node.clone());
                self.last = Some(node);
            }
        }
    }

    // 检查是否存在某邻接节点
    fn has_neighbor(&self, data: usize) -> bool {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            if val.borrow().data == data {
                return true;
            }
            curr = val.borrow().next.clone();
        }
        false
    }
}

// 根据数据构建图，返回包含图和访问标记的向量
fn create_graph(data: [[usize; 2]; 20]) -> Vec<(Graph, usize)> {
    let mut arr: Vec<(Graph, usize)> = Vec::new();

    // 初始化 9 个图，每个图对应一个顶点，访问标记初始化为 0
    for _ in 0..9 {
        arr.push((Graph::new(), 0));
    }

    // 构建图的邻接关系
    for i in 1..9 {
        for j in 0..data.len() {
            if data[j][0] == i && !arr[i].0.has_neighbor(data[j][1]) {
                arr[i].0.insert(data[j][1]); // 仅插入未存在的邻居
            }
        }
        print!("[{i}]->");
        arr[i].0.print_node();
    }

    arr
}

// 执行广度优先搜索
fn bfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes = Vec::new(); // 存储待访问的节点

    // 从顶点 1 开始 BFS
    gp[1].1 = 1; // 标记顶点 1 为已访问
    let mut curr = gp[1].0.get_first().clone();

    // 打印初始节点
    print!("{}->", 1);
    while let Some(val) = curr {
        nodes.push(val.borrow().data); // 将邻居加入待访问队列
        curr = val.borrow().next.clone();
    }
    println!();

    // 宽度优先遍历
    while !nodes.is_empty() {
        let data = nodes.remove(0); // 获取下一个待访问节点
        if gp[data].1 == 0 {
            gp[data].1 = 1; // 标记为已访问
            print!("{data}->");
            let mut curr = gp[data].0.get_first().clone();
            while let Some(val) = curr {
                if gp[val.borrow().data].1 == 0 { // 仅添加未访问的邻居
                    nodes.push(val.borrow().data);
                }
                curr = val.borrow().next.clone();
            }
        }
    }
    println!();
}

fn main() {
    // 定义邻接关系数据，[起点, 终点]
    let data = [
        [1, 2], [2, 1], [1, 3], [3, 1], [2, 4], [4, 2], [2, 5], [5, 2],
        [3, 6], [6, 3], [3, 7], [7, 3], [4, 5], [5, 4], [6, 7], [7, 6],
        [5, 8], [8, 5], [6, 8], [8, 6],
    ];
    let gp = create_graph(data);
    bfs(gp);
}