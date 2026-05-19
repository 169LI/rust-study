use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

// 点结构，包含顶点名称
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    name: &'a str,  // 顶点名称
}

impl<'a> Vertex<'a> {
    // 创建新顶点
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex { name }
    }
}

// 访问过的点结构，包含顶点和距离
#[derive(Debug)]
struct Visited<V> {
    vertex: V,      // 顶点
    distance: usize, // 到该顶点的距离
}

// 为 Visited 添加全序比较功能（小顶堆，按距离升序）
impl<V> Ord for Visited<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // 距离小的优先
    }
}

impl<V> PartialOrd for Visited<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> Eq for Visited<V> {}

impl<V> PartialEq for Visited<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

// 最短路径算法
fn dijkstra<'a>(
    start: Vertex<'a>,
    adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new();   // 存储从起点到各顶点的最短距离
    let mut visited = HashSet::new();     // 标记已访问的顶点
    let mut to_visit = BinaryHeap::new(); // 优先队列，存储待访问的顶点

    // 初始化起始点的距离为 0
    distances.insert(start, 0);
    to_visit.push(Visited {
        vertex: start,
        distance: 0,
    });

    // Dijkstra 算法主循环
    while let Some(Visited { vertex, distance }) = to_visit.pop() {
        // 如果已访问过该顶点，跳过
        if !visited.insert(vertex) {
            continue;
        }

        // 获取当前顶点的邻居
        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost; // 计算新距离
                // 如果新距离更短或未访问，更新距离
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visited {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

// 打印邻接表
fn print_graph<'a>(adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>) {
    println!("邻接表表示：");
    for (vertex, neighbors) in adjacency_list {
        print!("{} -> ", vertex.name);
        for &(neighbor, cost) in neighbors {
            print!("({}，权重 {}) ", neighbor.name, cost);
        }
        println!();
    }
}

fn main() {
    // 创建顶点
    let s = Vertex::new("s");
    let t = Vertex::new("t");
    let x = Vertex::new("x");
    let y = Vertex::new("y");
    let z = Vertex::new("z");

    // 构建邻接表
    let mut adj_list = HashMap::new();
    adj_list.insert(s, vec![(t, 10), (y, 5)]);
    adj_list.insert(t, vec![(y, 2), (x, 1)]);
    adj_list.insert(x, vec![(z, 4)]);
    adj_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
    adj_list.insert(z, vec![(s, 7), (x, 6)]);

    // 打印图结构
    print_graph(&adj_list);

    // 执行 Dijkstra 算法
    let distances = dijkstra(s, &adj_list);

    // 打印从起点到各顶点的最短距离
    println!("\n从顶点 {} 到各顶点的最短距离:", s.name);
    for (v, d) in &distances {
        println!("到 {}: {}", v.name, d);
    }

    // 验证结果
    assert_eq!(distances.get(&t), Some(&8));  // s -> y -> t
    assert_eq!(distances.get(&s), Some(&0));  // 起点
    assert_eq!(distances.get(&y), Some(&5));  // s -> y
    assert_eq!(distances.get(&x), Some(&9));  // s -> y -> x
    assert_eq!(distances.get(&z), Some(&7));  // s -> y -> z
}