// 定义二叉树节点
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // 创建新节点
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

// 定义二叉树
struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    // 创建空树
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // 插入节点（按BST规则：小于根节点放左子树，大于放右子树）
    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
            Some(node) => {
                Self::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Some(Box::new(Node::new(value)));
                }
                Some(left) => {
                    Self::insert_recursive(left, value);
                }
            }
        } else {
            match &mut node.right {
                None => {
                    node.right = Some(Box::new(Node::new(value)));
                }
                Some(right) => {
                    Self::insert_recursive(right, value);
                }
            }
        }
    }

    // 前序遍历（根-左-右）
    fn preorder(&self) {
        print!("Preorder: ");
        Self::preorder_recursive(&self.root);
        println!();
    }

    fn preorder_recursive(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            print!("{} ", n.value);
            Self::preorder_recursive(&n.left);
            Self::preorder_recursive(&n.right);
        }
    }

    // 中序遍历（左-根-右）
    fn inorder(&self) {
        print!("Inorder: ");
        Self::inorder_recursive(&self.root);
        println!();
    }

    fn inorder_recursive(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::inorder_recursive(&n.left);
            print!("{} ", n.value);
            Self::inorder_recursive(&n.right);
        }
    }

    // 后序遍历（左-右-根）
    fn postorder(&self) {
        print!("Postorder: ");
        Self::postorder_recursive(&self.root);
        println!();
    }

    fn postorder_recursive(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::postorder_recursive(&n.left);
            Self::postorder_recursive(&n.right);
            print!("{} ", n.value);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    // 插入节点
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);

    // 遍历树
    tree.preorder();  // 预期输出: 5 3 1 4 7
    tree.inorder();   // 预期输出: 1 3 4 5 7
    tree.postorder(); // 预期输出: 1 4 3 7 5
}