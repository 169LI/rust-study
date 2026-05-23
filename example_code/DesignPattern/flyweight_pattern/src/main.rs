//! 享元模式 (Flyweight Pattern) 示例：树的类型共享（Arc + HashMap 缓存池）
//!
//! 这个模块做什么：
//! - 把大量对象中重复的“内部状态”抽离出来共享，只保存一份，减少内存占用。
//!
//! 场景说明：
//! - 地图上有很多棵树（Tree），每棵树的位置/高度不同（外部状态）。
//! - 但树的“种类信息”（TreeType：名称/纹理/模型）重复度很高（内部状态），应该共享。
//! - 通过 `TreeFactory`（缓存池）确保同一种 TreeType 只创建一次，Tree 仅持有 `Arc<TreeType>`。

use std::collections::HashMap;
use std::sync::Arc;

/// 享元对象：可共享的内部状态（通常保持不可变）。
#[derive(Debug)]
struct TreeType {
    name: String,
    texture: String,
    model: String,
}

/// 具体对象：外部状态（每个实例不同） + 指向享元对象的共享引用。
#[derive(Debug)]
struct Tree {
    x: f32,
    y: f32,
    height: f32,
    tree_type: Arc<TreeType>,
}

/// 享元工厂：缓存池，负责复用 TreeType。
struct TreeFactory {
    types: HashMap<String, Arc<TreeType>>,
}

impl TreeFactory {
    /// 创建空的享元工厂。
    fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// 获取某种树类型的享元对象；如果缓存中不存在则创建并缓存。
    fn get_tree_type(&mut self, name: &str, texture: &str, model: &str) -> Arc<TreeType> {
        if let Some(existing) = self.types.get(name) {
            return Arc::clone(existing);
        }

        let tree_type = Arc::new(TreeType {
            name: name.to_string(),
            texture: texture.to_string(),
            model: model.to_string(),
        });

        self.types.insert(name.to_string(), Arc::clone(&tree_type));
        tree_type
    }

    /// 当前缓存了多少种 TreeType（享元数量）。
    fn cached_types(&self) -> usize {
        self.types.len()
    }
}

fn main() {
    let mut factory = TreeFactory::new();

    let oak = factory.get_tree_type("Oak", "oak.png", "oak.obj");
    let pine = factory.get_tree_type("Pine", "pine.png", "pine.obj");

    let trees = vec![
        Tree {
            x: 10.0,
            y: 20.0,
            height: 5.0,
            tree_type: Arc::clone(&oak),
        },
        Tree {
            x: 11.0,
            y: 21.0,
            height: 6.0,
            tree_type: Arc::clone(&oak),
        },
        Tree {
            x: 30.0,
            y: 40.0,
            height: 8.0,
            tree_type: Arc::clone(&pine),
        },
        Tree {
            x: 31.0,
            y: 41.0,
            height: 7.5,
            tree_type: Arc::clone(&pine),
        },
    ];

    println!("cached_types = {}", factory.cached_types());
    println!("oak_ptr  = {:p}", Arc::as_ptr(&oak));
    println!("pine_ptr = {:p}", Arc::as_ptr(&pine));
    println!("oak_strong_count  = {}", Arc::strong_count(&oak));
    println!("pine_strong_count = {}", Arc::strong_count(&pine));

    println!();
    for (i, tree) in trees.iter().enumerate() {
        let _ = (&tree.tree_type.texture, &tree.tree_type.model);
        println!(
            "tree#{i}: pos=({:.1},{:.1}) height={:.1} type={} type_ptr={:p}",
            tree.x,
            tree.y,
            tree.height,
            tree.tree_type.name,
            Arc::as_ptr(&tree.tree_type),
        );
    }
}
