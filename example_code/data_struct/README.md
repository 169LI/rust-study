# data_struct

Rust 数据结构与算法示例代码。

## 目录说明

- `basic_data_structure/` — 基础数据结构：数组、向量、切片、字符串、链表、栈、队列、双端队列。
- `binary_tree/` — 二叉树相关：二叉树、二叉搜索树、平衡二叉树、二叉堆。
- `graph/` — 图相关：邻接矩阵、邻接表、BFS、DFS、Dijkstra 最短路径。
- `search_algorithm/` — 查找算法：顺序查找、二分查找、插值查找、哈希查找、KMP、并行查找等。
- `sort_algorithm/` — 排序算法：冒泡、选择、插入、归并、快速、堆、希尔、基数、桶、计数排序。

## 运行方式

在 `data_struct/` 目录下，运行某个 crate 中的指定示例：

```bash
cargo run -p <crate名> --bin <示例名>
```

例如运行二分查找示例：

```bash
cargo run -p search_algorithm --bin binary_search_achieve
```

也可进入对应 crate 目录直接 `cargo run --bin <示例名>`。
