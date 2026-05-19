use std::cmp::{max, Ordering::*};
use std::fmt::Debug;
use std::mem::replace;

// 定义AVL树，可能为空（Null）或包含节点（Tree）
#[derive(Debug)]
enum AvlTree<T> {
    Null,              // 空树
    Tree(Box<AvlNode<T>>), // 包含节点的树
}

// 定义AVL树节点
#[derive(Debug)]
struct AvlNode<T> {
    val: T,            // 节点值
    left: AvlTree<T>,  // 左子树
    right: AvlTree<T>, // 右子树
    bfactor: i8,       // 平衡因子（左子树高度 - 右子树高度）
}

use AvlTree::*;

impl<T> AvlTree<T>
where
    T: Ord + Clone + Debug, // 值需支持比较、克隆和调试输出
{
    // 创建空AVL树
    fn new() -> Self {
        Null
    }

    // 判断树是否为空
    fn is_empty(&self) -> bool {
        matches!(*self, Null)
    }

    // 计算树的节点数
    fn len(&self) -> usize {
        match self {
            Null => 0,
            Tree(node) => 1 + node.left.len() + node.right.len(),
        }
    }

    // 计算树的深度（高度）
    fn depth(&self) -> usize {
        match self {
            Null => 0,
            Tree(node) => max(node.left.depth(), node.right.depth()) + 1,
        }
    }

    // 查找值是否存在
    fn search(&self, val: &T) -> bool {
        match self {
            Null => false,
            Tree(node) => match node.val.cmp(val) {
                Equal => true,
                Greater => node.left.search(val),
                Less => node.right.search(val),
            },
        }
    }

    // 插入值，返回(是否插入, 树是否加深)
    fn insert(&mut self, val: T) -> (bool, bool) {
        let ret = match self {
            Null => {
                // 空树，直接插入新节点
                *self = Tree(Box::new(AvlNode {
                    val,
                    left: Null,
                    right: Null,
                    bfactor: 0,
                }));
                (true, true)
            }
            Tree(node) => match node.val.cmp(&val) {
                Equal => (false, false), // 重复值，不插入
                Less => {
                    // 值大于当前节点，插入右子树
                    let (inserted, deepened) = node.right.insert(val);
                    if deepened {
                        node.bfactor -= 1; // 右子树加深，平衡因子减少
                        match node.bfactor {
                            -1 => (inserted, true),  // 平衡因子变为-1，树加深
                            0 => (inserted, false),  // 平衡因子恢复为0，树未加深
                            -2 => (inserted, false), // 右重，需平衡
                            _ => unreachable!(),
                        }
                    } else {
                        (inserted, deepened)
                    }
                }
                Greater => {
                    // 值小于当前节点，插入左子树
                    let (inserted, deepened) = node.left.insert(val);
                    if deepened {
                        node.bfactor += 1; // 左子树加深，平衡因子增加
                        match node.bfactor {
                            1 => (inserted, true),   // 平衡因子变为1，树加深
                            0 => (inserted, false),  // 平衡因子恢复为0，树未加深
                            2 => (inserted, false),  // 左重，需平衡
                            _ => unreachable!(),
                        }
                    } else {
                        (inserted, deepened)
                    }
                }
            },
        };
        self.rebalance(); // 插入后检查并调整平衡
        ret
    }

    // 删除值，返回(是否删除, 树是否变浅)
    fn delete(&mut self, val: &T) -> (bool, bool) {
        let ret = match self {
            Null => (false, false), // 空树，无需删除
            Tree(node) => match node.val.cmp(val) {
                Equal => {
                    // 找到要删除的节点
                    if let Null = node.left {
                        // 情况1：无左子树，返回右子树
                        *self = replace(&mut node.right, Null);
                        (true, true)
                    } else if let Null = node.right {
                        // 情况2：无右子树，返回左子树
                        *self = replace(&mut node.left, Null);
                        (true, true)
                    } else {
                        // 情况3：有两个子树，用右子树最小值替换
                        let min_node = node.right.find_min();
                        node.val = min_node.val.clone();
                        let (deleted, shallowed) = node.right.delete(&node.val);
                        if shallowed {
                            node.bfactor += 1; // 右子树变浅，平衡因子增加
                            match node.bfactor {
                                0 => (deleted, true),   // 平衡因子变为0，树变浅
                                1 => (deleted, false),  // 平衡因子变为1，树未变浅
                                2 => (deleted, false),  // 左重，需平衡
                                _ => unreachable!(),
                            }
                        } else {
                            (deleted, shallowed)
                        }
                    }
                }
                Greater => {
                    // 值小于当前节点，删除左子树中的值
                    let (deleted, shallowed) = node.left.delete(val);
                    if shallowed {
                        node.bfactor -= 1; // 左子树变浅，平衡因子减少
                        match node.bfactor {
                            0 => (deleted, true),   // 平衡因子变为0，树变浅
                            -1 => (deleted, false), // 平衡因子变为-1，树未变浅
                            -2 => (deleted, false), // 右重，需平衡
                            _ => unreachable!(),
                        }
                    } else {
                        (deleted, shallowed)
                    }
                }
                Less => {
                    // 值大于当前节点，删除右子树中的值
                    let (deleted, shallowed) = node.right.delete(val);
                    if shallowed {
                        node.bfactor += 1; // 右子树变浅，平衡因子增加
                        match node.bfactor {
                            0 => (deleted, true),   // 平衡因子变为0，树变浅
                            1 => (deleted, false),  // 平衡因子变为1，树未变浅
                            2 => (deleted, false),  // 左重，需平衡
                            _ => unreachable!(),
                        }
                    } else {
                        (deleted, shallowed)
                    }
                }
            },
        };
        self.rebalance(); // 删除后检查并调整平衡
        ret
    }

    // 查找最小值节点
    fn find_min(&self) -> &AvlNode<T> {
        match self {
            Null => panic!("Empty tree"),
            Tree(node) => {
                if let Null = node.left {
                    node
                } else {
                    node.left.find_min()
                }
            }
        }
    }

    // 中序遍历，输出升序序列
    fn inorder(&self) {
        match self {
            Null => (),
            Tree(node) => {
                node.left.inorder();
                print!("{:?} ", node.val);
                node.right.inorder();
            }
        }
    }

    // 获取节点引用
    fn node(&mut self) -> &mut AvlNode<T> {
        match self {
            Null => panic!("尝试访问空树的节点"),
            Tree(node) => node,
        }
    }

    // 获取左子树
    fn left_subtree(&mut self) -> &mut Self {
        match self {
            Null => panic!("尝试访问空树的左子树"),
            Tree(node) => &mut node.left,
        }
    }

    // 获取右子树
    fn right_subtree(&mut self) -> &mut Self {
        match self {
            Null => panic!("尝试访问空树的右子树"),
            Tree(node) => &mut node.right,
        }
    }

    // 左旋操作
    fn rotate_left(&mut self) {
        let mut v = replace(self, Null);
        let mut right = replace(v.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *v.right_subtree() = right_left;
        *right.left_subtree() = v;
        *self = right;
    }

    // 右旋操作
    fn rotate_right(&mut self) {
        let mut v = replace(self, Null);
        let mut left = replace(v.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *v.left_subtree() = left_right;
        *left.right_subtree() = v;
        *self = left;
    }

    // 调整平衡因子，处理失衡情况
    fn rebalance(&mut self) {
        match self {
            Null => (), // 空树无需平衡
            Tree(node) => match node.bfactor {
                2 => {
                    // 左重
                    let lbf = node.left.node().bfactor;
                    if lbf >= 0 {
                        // LL或LR情况，右旋
                        let (a, b) = if lbf == 1 { (0, 0) } else { (1, -1) };
                        self.rotate_right();
                        self.node().left.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if lbf == -1 {
                        // LR情况，先左旋左子树，再右旋
                        let (a, b) = match self.node().left.node().right.node().bfactor {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        };
                        self.node().left.rotate_left();
                        self.rotate_right();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!();
                    }
                }
                -2 => {
                    // 右重
                    let rbf = node.right.node().bfactor;
                    if rbf <= 0 {
                        // RR或RL情况，左旋
                        let (a, b) = if rbf == -1 { (0, 0) } else { (-1, 1) };
                        self.rotate_left();
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if rbf == 1 {
                        // RL情况，先右旋右子树，再左旋
                        let (a, b) = match self.node().right.node().left.node().bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };
                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!();
                    }
                }
                _ => (), // 平衡因子正常，无需调整
            },
        }
    }
}

fn main() {
    let mut avl = AvlTree::new();
    // 插入测试数据
    let values = vec![10, 5, 15, 3, 7, 12, 18];
    for v in values {
        avl.insert(v);
    }

    // 测试基本功能
    println!("是否为空: {}", avl.is_empty()); // 输出: false
    println!("节点数: {}", avl.len()); // 输出: 7
    println!("树深度: {}", avl.depth()); // 输出: 3
    println!("查找 7: {}", avl.search(&7)); // 输出: true
    println!("查找 9: {}", avl.search(&9)); // 输出: false
    print!("中序遍历: ");
    avl.inorder(); // 输出: 3 5 7 10 12 15 18
    println!();

    // 测试删除
    avl.delete(&5);
    println!("\n删除 5 后:");
    print!("中序遍历: ");
    avl.inorder(); // 输出: 3 7 10 12 15 18
    println!();
    println!("节点数: {}", avl.len()); // 输出: 6
    println!("树深度: {}", avl.depth()); // 输出: 3
}