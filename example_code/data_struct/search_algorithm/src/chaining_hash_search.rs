/// 拉链法哈希表，支持插入和查找
struct ChainingHashMap {
    buckets: Vec<Vec<(i32, i32)>>, // 每个槽位存储一个 Vec，模拟链表
    size: usize, // 哈希表容量
}

impl ChainingHashMap {
    /// 创建指定大小的哈希表
    fn new(size: usize) -> Self {
        ChainingHashMap {
            buckets: vec![Vec::new(); size], // 初始化每个槽位为空 Vec
            size,
        }
    }

    /// 简单的哈希函数：键对桶大小取模
    fn hash(&self, key: i32) -> usize {
        (key.abs() as usize) % self.size
    }

    /// 插入键值对，冲突时追加到槽位的 Vec
    fn insert(&mut self, key: i32, value: i32) {
        let index = self.hash(key);
        // 检查是否已存在键
        for entry in self.buckets[index].iter_mut() {
            if entry.0 == key {
                // 键已存在，更新值
                entry.1 = value;
                return;
            }
        }
        // 键不存在，追加到 Vec
        self.buckets[index].push((key, value));
    }

    /// 查找键对应的值
    fn get(&self, key: i32) -> Option<i32> {
        let index = self.hash(key);
        // 遍历槽位的 Vec，查找匹配的键
        for &(k, v) in self.buckets[index].iter() {
            if k == key {
                return Some(v);
            }
        }
        None // 未找到
    }
}

fn main() {
    // 创建一个大小为 10 的哈希表
    let mut hash_map = ChainingHashMap::new(10);

    // 插入测试数据
    hash_map.insert(1, 100); // 键 1 -> 值 100
    hash_map.insert(2, 200); // 键 2 -> 值 200
    hash_map.insert(11, 1100); // 键 11 -> 值 1100（可能冲突，追加到同一 Vec）

    // 定义查找的目标键
    let target_key1 = 2; // 存在
    let target_key2 = 5; // 不存在

    // 测试用例 1：查找存在的键
    println!("测试查找键 {}", target_key1);
    match hash_map.get(target_key1) {
        Some(value) => println!("找到键 {}，值为 {}", target_key1, value),
        None => println!("键 {} 未找到", target_key1),
    }

    // 测试用例 2：查找不存在的键
    println!("\n测试查找键 {}", target_key2);
    match hash_map.get(target_key2) {
        Some(value) => println!("找到键 {}，值为 {}", target_key2, value),
        None => println!("键 {} 未找到", target_key2),
    }

    // 测试用例 3：更新已有键的值
    let update_key = 1;
    hash_map.insert(update_key, 999); // 更新键 1 的值
    println!("\n测试更新键 {}", update_key);
    match hash_map.get(update_key) {
        Some(value) => println!("找到键 {}，更新后的值为 {}", update_key, value),
        None => println!("键 {} 未找到", update_key),
    }

    // 测试用例 4：插入可能冲突的键
    let conflict_key = 21; // 与键 1 可能映射到同一槽位（21 % 10 = 1）
    hash_map.insert(conflict_key, 2100);
    println!("\n测试插入可能冲突的键 {}", conflict_key);
    match hash_map.get(conflict_key) {
        Some(value) => println!("找到键 {}，值为 {}", conflict_key, value),
        None => println!("键 {} 未找到", conflict_key),
    }
}