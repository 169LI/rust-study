use std::hash::Hash;
use std::collections::HashMap;

// 顶点定义，表示图中的一个节点
#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,                  // 顶点的标识（键）
    connects: Vec<(T, i32)>, // 邻接顶点集合，存储 (邻接顶点, 边权重)
}

impl<T: Clone + PartialEq> Vertex<T> {
    // 创建一个新顶点
    fn new(key: T) -> Self {
        Self {
            key,
            connects: Vec::new(),
        }
    }

    // 判断当前顶点是否与指定顶点相邻
    fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    // 添加邻接顶点及其边权重
    fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt));
    }

    // 获取所有邻接顶点的键
    fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }
        connects
    }

    // 获取到指定邻接顶点的边权重，若不存在返回 0
    fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }

    // 删除到指定邻接顶点的边
    fn remove_neighbor(&mut self, key: &T) {
        self.connects.retain(|(k, _)| k != key);
    }
}

// 图定义，基于邻接表实现
#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,              // 顶点数量
    edgenums: u32,              // 边数量
    vertices: HashMap<T, Vertex<T>>, // 顶点集合，使用 HashMap 存储
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    // 创建一个空图
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    // 判断图是否为空
    fn is_empty(&self) -> bool {
        self.vertnums == 0
    }

    // 获取顶点数量
    fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    // 获取边数量
    fn edge_num(&self) -> u32 {
        self.edgenums
    }

    // 检查图中是否包含指定顶点
    fn contains(&self, key: &T) -> bool {
        self.vertices.contains_key(key)
    }

    // 添加一个顶点
    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        if self.contains(key) {
            return None; // 若顶点已存在，返回 None
        }
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    // 获取指定顶点的引用
    fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        self.vertices.get(key)
    }

    // 获取所有顶点的键
    fn vertex_keys(&self) -> Vec<T> {
        self.vertices.keys().cloned().collect()
    }

    // 删除指定顶点及其相关边
    fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key)?;
        self.vertnums -= 1;

        // 删除从当前顶点出发的边
        self.edgenums -= old_vertex.get_connects().len() as u32;

        // 删除其他顶点到当前顶点的边
        for vertex_key in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex_key) {
                if vt.adjacent_key(key) {
                    vt.remove_neighbor(key);
                    self.edgenums -= 1;
                }
            }
        }

        Some(old_vertex)
    }

    // 添加一条无向边 (from, to) 和 (to, from)
    fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        // 若顶点不存在，添加顶点
        if !self.contains(from) {
            self.add_vertex(from);
        }
        if !self.contains(to) {
            self.add_vertex(to);
        }

        // 添加边 (from -> to)
        if !self.is_adjacent(from, to) {
            self.vertices.get_mut(from).unwrap().add_neighbor(to.clone(), wt);
            self.edgenums += 1;
        }

        // 添加反向边 (to -> from)，实现无向图
        if !self.is_adjacent(to, from) {
            self.vertices.get_mut(to).unwrap().add_neighbor(from.clone(), wt);
            self.edgenums += 1;
        }
    }

    // 删除一条无向边 (from, to) 和 (to, from)
    fn remove_edge(&mut self, from: &T, to: &T) {
        if !self.contains(from) || !self.contains(to) {
            return; // 若顶点不存在，直接返回
        }

        // 删除边 (from -> to)
        if self.is_adjacent(from, to) {
            self.vertices.get_mut(from).unwrap().remove_neighbor(to);
            self.edgenums -= 1;
        }

        // 删除反向边 (to -> from)
        if self.is_adjacent(to, from) {
            self.vertices.get_mut(to).unwrap().remove_neighbor(from);
            self.edgenums -= 1;
        }
    }

    // 判断两个顶点是否相邻
    fn is_adjacent(&self, from: &T, to: &T) -> bool {
        if let Some(vertex) = self.vertices.get(from) {
            vertex.adjacent_key(to)
        } else {
            false
        }
    }
}

fn main() {
    // 创建一个空图
    let mut g = Graph::new();

    // 添加 0 到 5 的顶点
    for i in 0..6 {
        g.add_vertex(&i);
    }
    println!("图是否为空: {}", g.is_empty());

    // 打印所有顶点的键
    let vertices = g.vertex_keys();
    for vertex in &vertices {
        println!("顶点: {:?}", vertex);
    }

    // 添加边及其权重
    g.add_edge(&0, &1, 5);
    g.add_edge(&0, &5, 2);
    g.add_edge(&1, &2, 4);
    g.add_edge(&2, &3, 9);
    g.add_edge(&3, &4, 7);
    g.add_edge(&3, &5, 3);
    g.add_edge(&4, &0, 1);
    g.add_edge(&4, &4, 8); // 自环边
    println!("顶点数量: {}", g.vertex_num());
    println!("边数量: {}", g.edge_num());
    println!("是否包含顶点 0: {}", g.contains(&0));

    // 获取顶点 0 的信息
    let vertex = g.get_vertex(&0).unwrap();
    println!("顶点 0 的键: {}, 到顶点 1 的边权重: {}", vertex.key, vertex.get_nbr_weight(&1));

    // 打印顶点 0 的邻接顶点
    let keys = vertex.get_connects();
    for nbr in keys {
        println!("顶点 0 的邻接顶点: {}", nbr);
    }

    // 打印顶点 0 的所有邻接顶点和边权重
    for (nbr, wt) in vertex.connects.iter() {
        println!("顶点 0 的邻接顶点: {}, 边权重: {}", nbr, wt);
    }

    // 测试两个顶点是否相邻
    let res = g.is_adjacent(&0, &1);
    println!("顶点 0 与 1 是否相邻: {}", res);
    let res = g.is_adjacent(&3, &2);
    println!("顶点 3 与 2 是否相邻: {}", res);

    // 删除顶点 0 及其相关边
    let rm = g.remove_vertex(&0).unwrap();
    println!("删除的顶点: {}", rm.key);
    println!("剩余顶点数量: {}", g.vertex_num());
    println!("剩余边数量: {}", g.edge_num());
    println!("是否包含顶点 0: {}", g.contains(&0));

    // 测试删除边 (3, 5)
    println!("\n删除边 (3, 5) 前边数量: {}", g.edge_num());
    g.remove_edge(&3, &5);
    println!("删除边 (3, 5) 后边数量: {}", g.edge_num());
    let res = g.is_adjacent(&3, &5);
    println!("顶点 3 与 5 是否相邻: {}", res);
}