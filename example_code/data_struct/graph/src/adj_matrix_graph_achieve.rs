use std::collections::{HashSet, VecDeque};

// 点定义
#[derive(Debug)]
struct Vertex {
    id: usize,  // 顶点编号
}

impl Vertex {
    // 创建新顶点
    fn new(id: usize) -> Self {
        Self { id }
    }
}

// 边定义
#[derive(Debug, Clone)]
struct Edge {
    edge: bool,  // 表示是否有边，true 表示存在边，false 表示无边
}

impl Edge {
    // 创建默认边（无边状态）
    fn new() -> Self {
        Self { edge: false }
    }

    // 设置边为存在状态
    fn set_edge() -> Self {
        Edge { edge: true }
    }
}

// 图定义
#[derive(Debug)]
struct Graph {
    nodes: usize,           // 顶点数量
    graph: Vec<Vec<Edge>>,  // 邻接矩阵：graph[i][j] 表示顶点 i 到 j 的边
}

impl Graph {
    // 创建新图，初始化邻接矩阵
    fn new(nodes: usize) -> Self {
        Self {
            nodes,
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    // 返回顶点数量
    fn len(&self) -> usize {
        self.nodes
    }

    // 判断图是否为空
    fn is_empty(&self) -> bool {
        0 == self.nodes
    }

    // 添加无向边，设置边属性为 true（双向）
    fn add_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::set_edge(); // n1 -> n2
            self.graph[n2.id][n1.id] = Edge::set_edge(); // n2 -> n1（无向图）
        } else {
            panic!("顶点索引超出范围");
        }
    }

    // 删除无向边，设置边属性为 false（双向）
    fn remove_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::new(); // 删除 n1 -> n2
            self.graph[n2.id][n1.id] = Edge::new(); // 删除 n2 -> n1
        } else {
            panic!("顶点索引超出范围");
        }
    }

    // 深度优先搜索 (DFS) 遍历
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_helper(start, &mut visited, &mut result);
        result
    }

    // DFS 辅助函数，递归实现
    fn dfs_helper(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        if vertex >= self.nodes {
            return;
        }
        visited.insert(vertex);
        result.push(vertex);
        for neighbor in 0..self.nodes {
            if self.graph[vertex][neighbor].edge && !visited.contains(&neighbor) {
                self.dfs_helper(neighbor, visited, result);
            }
        }
    }

    // 广度优先搜索 (BFS) 遍历
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        if start >= self.nodes {
            return result;
        }

        visited.insert(start);
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);
            for neighbor in 0..self.nodes {
                if self.graph[vertex][neighbor].edge && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        result
    }

    // 打印邻接矩阵，1 表示有边，0 表示无边
    fn print_graph(&self) {
        println!("邻接矩阵表示（1 表示有边，0 表示无边）：");
        for i in 0..self.nodes {
            print!("顶点 {}: ", i);
            for j in 0..self.nodes {
                print!("{} ", if self.graph[i][j].edge { 1 } else { 0 });
            }
            println!();
        }
    }
}

fn main() {
    // 创建图，4 个顶点
    let mut g = Graph::new(4);
    let n1 = Vertex::new(0);
    let n2 = Vertex::new(1);
    let n3 = Vertex::new(2);
    let n4 = Vertex::new(3);

    // 添加边，形成一个连通图
    g.add_edge(&n1, &n2);
    g.add_edge(&n1, &n3);
    g.add_edge(&n2, &n3);
    g.add_edge(&n2, &n4);
    g.add_edge(&n3, &n4);

    // 打印图的详细信息
    println!("图的完整结构:");
    println!("{:#?}", g);
    println!("图是否为空: {}", g.is_empty());
    println!("图的顶点数量: {}", g.len());

    // 打印邻接矩阵
    g.print_graph();

    // 执行 DFS 和 BFS 遍历
    println!("\nDFS 遍历 (从顶点 0 开始): {:?}", g.dfs(0));
    println!("BFS 遍历 (从顶点 0 开始): {:?}", g.bfs(0));

    // 删除边 (1, 2)
    println!("\n删除边 (1, 2) 后:");
    g.remove_edge(&n2, &n3);
    g.print_graph();
    println!("DFS 遍历 (从顶点 0 开始): {:?}", g.dfs(0));
    println!("BFS 遍历 (从顶点 0 开始): {:?}", g.bfs(0));

    // 测试空图
    let empty_graph = Graph::new(0);
    println!("\n空图测试:");
    println!("图是否为空: {}", empty_graph.is_empty());
    empty_graph.print_graph();
}