//! 迭代器模式 (Iterator Pattern) 核心演示：真实文件扫描器
//!
//! 本项目着重展示迭代器模式的**最核心本质**：
//! **“把遍历逻辑从数据结构本身中抽离出来，让调用者不关心底层结构也能统一访问”。**
//!
//! 场景说明：
//! 底层是真实的操作系统文件目录树。
//! 如果不使用迭代器，调用者需要自己写递归函数配合 `std::fs::read_dir` 来遍历。
//! 使用迭代器后，目录树的 DFS（深度优先遍历）状态被封装在迭代器内部，
//! 调用者只需要像遍历一维数组一样，传入一个初始路径，不断调用 `.next()` 即可获取所有文件。

use std::fs;
use std::path::{Path, PathBuf};

/// 这是一个自定义的迭代器结构体，用于扫描真实文件系统。
/// 它负责“记住”当前遍历到哪个目录了。
/// 这里我们使用一个栈（Vec）来保存待处理的路径，实现深度优先搜索。
struct FileScanner {
    // 栈中保存了待访问的路径
    stack: Vec<PathBuf>,
}

impl FileScanner {
    /// 传入一个真实的文件夹路径，初始化扫描器
    fn new<P: AsRef<Path>>(root_path: P) -> Self {
        FileScanner {
            stack: vec![root_path.as_ref().to_path_buf()],
        }
    }
}

/// 为我们的 FileScanner 实现标准库的 Iterator trait。
impl Iterator for FileScanner {
    // 每次调用 next 返回的是文件的真实路径
    type Item = PathBuf;

    /// 核心逻辑：获取下一个文件。
    fn next(&mut self) -> Option<Self::Item> {
        // 只要栈里还有路径，就继续处理
        while let Some(current_path) = self.stack.pop() {
            // 如果是文件夹，读取内部的条目并压入栈中
            if current_path.is_dir() {
                if let Ok(entries) = fs::read_dir(&current_path) {
                    for entry in entries.flatten() {
                        self.stack.push(entry.path());
                    }
                }
            }
            // 如果是文件，直接返回该路径
            else if current_path.is_file() {
                return Some(current_path);
            }
        }
        // 栈空了，说明整个目录树遍历完了
        None
    }
}

fn main() {
    println!("--- 开始扫描当前项目下的 src 目录 ---");

    // 1. 创建迭代器，直接传入真实的文件夹路径
    let scanner = FileScanner::new(".");

    // 2. 收集到一个集合中（这样可以多次复用结果，而不必重新扫描硬盘）
    // 注意：for 循环或 collect 都会“消耗”迭代器。
    // 如果我们想多次处理同一批文件，最好的办法是先收集到 Vec 中，
    // 后续再对这个 Vec 调用 `.iter()` 产生不可变引用的迭代器。
    let all_files: Vec<PathBuf> = scanner.collect();

    for file_path in &all_files {
        println!("找到文件: {}", file_path.display());
    }

    println!("\n--- 证明它是一个标准的迭代器，结合链式调用 ---");

    // 3. 复用扫描结果
    // 这里我们不再重新创建 FileScanner，而是基于已经扫描好的 all_files 再次开启迭代。
    // 此时是对 PathBuf 的不可变引用进行操作。
    let rs_count = all_files
        .iter()
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
        .inspect(|p| println!("发现 Rust 源文件: {}", p.display()))
        .count();

    println!("当前目录下总共包含 {} 个 .rs 文件", rs_count);
}
