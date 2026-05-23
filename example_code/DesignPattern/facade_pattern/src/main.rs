//! 外观模式 (Facade Pattern) 示例：文件处理流水线的统一入口
//!
//! 这个模块做什么：
//! - 用一个 Facade（外观）把多个子系统的调用顺序、参数细节封装起来，调用方只需要调用一个高层方法。
//!
//! 场景说明：
//! - 子系统包含：读取文件、压缩、加密、写入文件。
//! - 调用方不需要了解“先读再压缩再加密再写入”的顺序，也不需要知道每个子系统的细节参数。
//! - 调用方只调用 `FilePipelineFacade::process_file(...)`。

use std::fs;
use std::path::Path;

/// 子系统 A：文件系统读写。
struct FileSystem;

impl FileSystem {
    /// 读取文件为字节数组。
    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| format!("read failed: {e}"))
    }

    /// 将字节数组写入文件。
    fn write_bytes(&self, path: &Path, data: &[u8]) -> Result<(), String> {
        fs::write(path, data).map_err(|e| format!("write failed: {e}"))
    }
}

/// 子系统 B：压缩器（示例使用简单 RLE）。
struct Compressor;

impl Compressor {
    /// 压缩输入数据。
    fn compress(&self, input: &[u8]) -> Vec<u8> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::with_capacity(input.len());
        let mut current = input[0];
        let mut count: u8 = 1;

        for &b in &input[1..] {
            if b == current && count < u8::MAX {
                count += 1;
            } else {
                out.push(count);
                out.push(current);
                current = b;
                count = 1;
            }
        }

        out.push(count);
        out.push(current);
        out
    }
}

/// 子系统 C：加密器（示例使用 XOR）。
struct Encryptor {
    key: u8,
}

impl Encryptor {
    /// 加密/解密（对称操作）。
    fn xor(&self, input: &[u8]) -> Vec<u8> {
        input.iter().map(|b| b ^ self.key).collect()
    }
}

/// 外观（Facade）：对外提供一个高层接口，把复杂子系统组合起来。
struct FilePipelineFacade {
    fs: FileSystem,
    compressor: Compressor,
    encryptor: Encryptor,
}

impl FilePipelineFacade {
    /// 创建外观对象。
    fn new(key: u8) -> Self {
        Self {
            fs: FileSystem,
            compressor: Compressor,
            encryptor: Encryptor { key },
        }
    }

    /// 读取 -> 压缩 -> 加密 -> 写入 的统一入口。
    ///
    /// # Errors
    /// - 读取或写入文件失败会返回错误。
    fn process_file(&self, input_path: &Path, output_path: &Path) -> Result<(), String> {
        let raw = self.fs.read_bytes(input_path)?;
        let compressed = self.compressor.compress(&raw);
        let encrypted = self.encryptor.xor(&compressed);
        self.fs.write_bytes(output_path, &encrypted)?;

        println!(
            "input_size={} compressed_size={} output_size={}",
            raw.len(),
            compressed.len(),
            encrypted.len()
        );
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let input_path = Path::new("./facade_input.txt");
    let output_path = Path::new("./facade_output.bin");

    fs::write(input_path, b"AAAAABBBBBCCCCCCCCCCCCDDDDDDDDDD")
        .map_err(|e| format!("prepare input failed: {e}"))?;

    let facade = FilePipelineFacade::new(0xA5);
    facade.process_file(input_path, output_path)?;

    println!("done: output written to {}", output_path.display());
    Ok(())
}
