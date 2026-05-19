/// 简化的哈希表，支持插入和查找
struct SimpleHashMap {
    buckets: Vec<Option<(i32, i32)>>, // 存储 (键, 值) 的桶
    size: usize, // 哈希表容量
}

impl SimpleHashMap {
    /// 创建指定大小的哈希表
    fn new(size: usize) -> Self {
        SimpleHashMap {
            buckets: vec![None; size],
            size,
        }
    }

    /// 简单的哈希函数：键对桶大小取模
    fn hash(&self, key: i32) -> usize {
        (key.abs() as usize) % self.size
    }

    /// 插入键值对，使用线性探测解决冲突
    fn insert(&mut self, key: i32, value: i32) {
        let mut index = self.hash(key);
        // 线性探测：如果槽位被占用，尝试下一个
        while self.buckets[index].is_some() {
            if let Some((existing_key, _)) = self.buckets[index] {
                if existing_key == key {
                    // 键已存在，更新值
                    self.buckets[index] = Some((key, value));
                    return;
                }
            }
            index = (index + 1) % self.size; // 下一个槽位
        }
        // 插入到空槽位
        self.buckets[index] = Some((key, value));
    }

    /// 查找键对应的值
    fn get(&self, key: i32) -> Option<i32> {
        let mut index = self.hash(key);
        let mut steps = 0;
        // 线性探测查找
        while steps < self.size {
            match self.buckets[index] {
                Some((existing_key, value)) if existing_key == key => return Some(value),
                Some(_) => index = (index + 1) % self.size, // 冲突，继续探测
                None => return None, // 空槽位，停止查找
            }
            steps += 1;
        }
        None // 未找到
    }
}

fn main() {
    // 创建一个大小为 10 的哈希表
    let mut hash_map = SimpleHashMap::new(10);

    // 插入测试数据
    hash_map.insert(1, 100); // 键 1 -> 值 100
    hash_map.insert(2, 200); // 键 2 -> 值 200
    hash_map.insert(11, 1100); // 键 11 -> 值 1100（可能冲突）

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
}