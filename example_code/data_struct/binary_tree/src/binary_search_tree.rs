use std::cmp::Ordering;
use std::fmt::Debug;

// 定义二叉查找树子节点链接类型，使用 Option 和 Box 管理内存
type Link<T, U> = Option<Box<BST<T, U>>>;

// 二叉查找树节点定义，包含键、值和左右子节点
#[derive(Debug)]
struct BST<T, U> {
    key: Option<T>,    // 节点键，None 表示空节点
    val: Option<U>,    // 节点值，None 表示无值
    left: Link<T, U>,  // 左子树链接
    right: Link<T, U>, // 右子树链接
}

impl<T, U> BST<T, U>
where
    T: Clone + Ord + Debug, // 键需支持克隆、比较和调试输出
    U: Clone + Debug,       // 值需支持克隆和调试输出
{
    // 创建空二叉查找树
    fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    // 判断树是否为空
    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    // 计算树的节点数
    fn len(&self) -> usize {
        self.calc_len(0)
    }

    // 递归计算节点数
    fn calc_len(&self, i: usize) -> usize {
        if self.key.is_none() {
            return i;
        }
        let mut count = i + 1; // 计入当前节点
        // 递归计算左子树节点数
        if let Some(left) = &self.left {
            count = left.calc_len(count);
        }
        // 递归计算右子树节点数
        if let Some(right) = &self.right {
            count = right.calc_len(count);
        }
        count
    }

    // 前序遍历（根-左-右）
    fn preorder(&self) {
        if self.key.is_none() {
            return;
        }
        // 打印当前节点
        println!("key: {:?}, value: {:?}", &self.key, &self.val);
        // 递归遍历左子树
        if let Some(left) = &self.left {
            left.preorder();
        }
        // 递归遍历右子树
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    // 中序遍历（左-根-右），输出升序键序列
    fn inorder(&self) {
        if self.key.is_none() {
            return;
        }
        // 递归遍历左子树
        if let Some(left) = &self.left {
            left.inorder();
        }
        // 打印当前节点
        println!("key: {:?}, value: {:?}", &self.key, &self.val);
        // 递归遍历右子树
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    // 后序遍历（左-右-根）
    fn postorder(&self) {
        if self.key.is_none() {
            return;
        }
        // 递归遍历左子树
        if let Some(left) = &self.left {
            left.postorder();
        }
        // 递归遍历右子树
        if let Some(right) = &self.right {
            right.postorder();
        }
        // 打印当前节点
        println!("key: {:?}, value: {:?}", &self.key, &self.val);
    }

    // 插入键值对
    fn insert(&mut self, key: T, val: U) {
        if self.key.is_none() {
            // 空节点，直接插入
            self.key = Some(key);
            self.val = Some(val);
            return;
        }
        if let Some(k) = &self.key {
            match key.cmp(k) {
                Ordering::Equal => {
                    // 键相等，更新值
                    self.val = Some(val);
                }
                Ordering::Less => {
                    // 键小于当前节点，插入左子树
                    if self.left.is_none() {
                        let mut new_node = BST::new();
                        new_node.key = Some(key);
                        new_node.val = Some(val);
                        self.left = Some(Box::new(new_node));
                    } else {
                        self.left.as_mut().unwrap().insert(key, val);
                    }
                }
                Ordering::Greater => {
                    // 键大于当前节点，插入右子树
                    if self.right.is_none() {
                        let mut new_node = BST::new();
                        new_node.key = Some(key);
                        new_node.val = Some(val);
                        self.right = Some(Box::new(new_node));
                    } else {
                        self.right.as_mut().unwrap().insert(key, val);
                    }
                }
            }
        }
    }

    // 查找键是否存在
    fn search(&self, key: &T) -> bool {
        if self.key.is_none() {
            return false;
        }
        if let Some(k) = &self.key {
            match k.cmp(key) {
                Ordering::Equal => true, // 找到键
                Ordering::Less => {
                    // 键大于当前节点，查找右子树
                    self.right.as_ref().map_or(false, |node| node.search(key))
                }
                Ordering::Greater => {
                    // 键小于当前节点，查找左子树
                    self.left.as_ref().map_or(false, |node| node.search(key))
                }
            }
        } else {
            false
        }
    }

    // 获取键对应的值
    fn get(&self, key: &T) -> Option<&U> {
        if self.key.is_none() {
            return None;
        }
        if let Some(k) = &self.key {
            match k.cmp(key) {
                Ordering::Equal => self.val.as_ref(), // 找到键，返回值
                Ordering::Less => {
                    // 键大于当前节点，查找右子树
                    self.right.as_ref().and_then(|node| node.get(key))
                }
                Ordering::Greater => {
                    // 键小于当前节点，查找左子树
                    self.left.as_ref().and_then(|node| node.get(key))
                }
            }
        } else {
            None
        }
    }

    // 查找最小键值对
    fn min(&self) -> (Option<&T>, Option<&U>) {
        if self.key.is_none() {
            return (None, None);
        }
        // 最小键在最左节点
        let mut current = self;
        while let Some(left) = &current.left {
            current = left;
        }
        (current.key.as_ref(), current.val.as_ref())
    }

    // 查找最大键值对
    fn max(&self) -> (Option<&T>, Option<&U>) {
        if self.key.is_none() {
            return (None, None);
        }
        // 最大键在最右节点
        let mut current = self;
        while let Some(right) = &current.right {
            current = right;
        }
        (current.key.as_ref(), current.val.as_ref())
    }

    // 删除键
    fn delete(&mut self, key: &T) {
        let root = std::mem::replace(self, BST::new());
        *self = Self::delete_recursive(root, key);
    }

    // 递归删除节点
    fn delete_recursive(mut node: BST<T, U>, key: &T) -> BST<T, U> {
        if node.key.is_none() {
            return node;
        }
        if let Some(k) = &node.key {
            match k.cmp(key) {
                Ordering::Less => {
                    // 键大于当前节点，删除右子树中的键
                    if let Some(right) = node.right {
                        node.right = Some(Box::new(Self::delete_recursive(*right, key)));
                    }
                    node
                }
                Ordering::Greater => {
                    // 键小于当前节点，删除左子树中的键
                    if let Some(left) = node.left {
                        node.left = Some(Box::new(Self::delete_recursive(*left, key)));
                    }
                    node
                }
                Ordering::Equal => {
                    // 找到要删除的节点
                    if node.left.is_none() {
                        // 情况1：无左子树，返回右子树
                        if let Some(right) = node.right {
                            *right
                        } else {
                            BST::new()
                        }
                    } else if node.right.is_none() {
                        // 情况2：无右子树，返回左子树
                        if let Some(left) = node.left {
                            *left
                        } else {
                            BST::new()
                        }
                    } else {
                        // 情况3：有两个子树
                        // 找到右子树最小节点
                        let mut min_node = node.right.take().unwrap();
                        let (min_key, min_val) = {
                            let mut current = &mut min_node;
                            while let Some(left) = &mut current.left {
                                current = left;
                            }
                            (
                                current.key.take().unwrap(),
                                current.val.take().unwrap(),
                            )
                        };
                        // 用最小节点的键值替换当前节点
                        node.key = Some(min_key.clone());
                        node.val = Some(min_val);
                        // 删除右子树中的最小节点
                        node.right = Some(Box::new(Self::delete_recursive(*min_node, &min_key)));
                        node
                    }
                }
            }
        } else {
            node
        }
    }
}

fn main() {
    let mut bst = BST::<i32, char>::new();
    // 插入测试数据
    bst.insert(8, 'e');
    bst.insert(6, 'c');
    bst.insert(7, 'd');
    bst.insert(5, 'b');
    bst.insert(10, 'g');
    bst.insert(9, 'f');
    bst.insert(11, 'h');
    bst.insert(4, 'a');

    // 测试基本功能
    println!("是否为空: {:?}", bst.is_empty()); // 输出: false
    println!("节点数: {:?}", bst.len()); // 输出: 8
    println!("最小键值对: {:?}", bst.min()); // 输出: (Some(4), Some('a'))
    println!("最大键值对: {:?}", bst.max()); // 输出: (Some(11), Some('h'))
    println!("查找键 5 的值: {:?}", bst.get(&5)); // 输出: Some('b')
    println!("是否存在键 5: {:?}", bst.search(&5)); // 输出: true
    println!("是否存在键 12: {:?}", bst.search(&12)); // 输出: false

    // 测试遍历
    println!("\n前序遍历:");
    bst.preorder();
    println!("\n中序遍历:");
    bst.inorder();
    println!("\n后序遍历:");
    bst.postorder();

    // 测试删除
    println!("\n删除键 6 后中序遍历:");
    bst.delete(&6);
    bst.inorder();
}