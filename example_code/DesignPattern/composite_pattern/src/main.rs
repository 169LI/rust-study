//! 组合模式 (Composite Pattern) 示例：文件系统目录树
//!
//! 这个模块做什么：
//! - 用“树形结构”表达“整体—部分”关系（目录包含文件/目录）。
//! - 让调用方用统一方式处理“单个对象（文件）”和“组合对象（目录）”，例如：打印结构、计算总大小、按名称查找。
//!
//! 场景说明：
//! - `Node` 表示统一的节点类型：要么是文件，要么是目录（目录里又包含子节点）。
//! - 调用方只需要调用 `print_tree / total_size / find_by_name`，不需要在外部写递归和区分逻辑。

/// 文件系统节点：文件（Leaf）或目录（Composite）。
#[derive(Debug)]
enum Node {
    File { name: String, size_bytes: u64 },
    Directory { name: String, children: Vec<Node> },
}

impl Node {
    /// 创建一个文件节点。
    fn file(name: impl Into<String>, size_bytes: u64) -> Self {
        Self::File {
            name: name.into(),
            size_bytes,
        }
    }

    /// 创建一个目录节点。
    fn dir(name: impl Into<String>, children: Vec<Node>) -> Self {
        Self::Directory {
            name: name.into(),
            children,
        }
    }

    /// 返回节点名称（目录名或文件名）。
    fn name(&self) -> &str {
        match self {
            Node::File { name, .. } => name,
            Node::Directory { name, .. } => name,
        }
    }

    /// 计算节点的总大小。
    ///
    /// - 文件：返回自身大小。
    /// - 目录：递归累加所有子节点大小。
    fn total_size(&self) -> u64 {
        match self {
            Node::File { size_bytes, .. } => *size_bytes,
            Node::Directory { children, .. } => children.iter().map(Node::total_size).sum(),
        }
    }

    /// 以树形结构打印节点。
    fn print_tree(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            Node::File { name, size_bytes } => {
                println!("{indent}- file: {name} ({size_bytes} bytes)");
            }
            Node::Directory { name, children } => {
                println!("{indent}+ dir : {name}");
                for child in children {
                    child.print_tree(depth + 1);
                }
            }
        }
    }

    /// 在树中按名称查找第一个匹配节点（深度优先）。
    fn find_by_name(&self, target: &str) -> Option<&Node> {
        if self.name() == target {
            return Some(self);
        }

        match self {
            Node::File { .. } => None,
            Node::Directory { children, .. } => {
                children.iter().find_map(|c| c.find_by_name(target))
            }
        }
    }
}

fn main() {
    let tree = Node::dir(
        "root",
        vec![
            Node::file("README.md", 120),
            Node::dir(
                "src",
                vec![
                    Node::file("main.rs", 380),
                    Node::file("lib.rs", 210),
                    Node::dir("nested", vec![Node::file("mod.rs", 95)]),
                ],
            ),
            Node::dir(
                "assets",
                vec![Node::file("logo.png", 8_192), Node::file("bg.jpg", 54_321)],
            ),
        ],
    );

    println!("--- tree ---");
    tree.print_tree(0);

    println!();
    println!("total_size = {} bytes", tree.total_size());

    println!();
    let target = "logo.png";
    match tree.find_by_name(target) {
        Some(node) => println!(
            "found '{target}': kind={:?}, size={} bytes",
            node,
            node.total_size()
        ),
        None => println!("not found: '{target}'"),
    }
}
