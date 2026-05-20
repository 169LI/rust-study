//! 策略模式 (Strategy Pattern) 示例：文件压缩策略
//!
//! 这个模块做什么：
//! - 将“压缩算法”抽象成统一接口（trait），并提供多个可替换实现。
//! - 演示调用方（Context）只依赖抽象接口，可在运行时或编译期切换策略。
//!
//! 场景说明：
//! - 模拟一个“文件压缩工具”，支持三种压缩方式：none（不压缩）、rle（简单游程编码）、xor（示例算法）。
//! - 通过命令行参数选择策略：`--algo none|rle|xor`。

use std::env;

/// 压缩策略接口（Strategy）。
trait Compressor {
    /// 返回策略名称，用于输出展示。
    fn name(&self) -> &'static str;

    /// 对输入数据进行压缩。
    fn compress(&self, input: &[u8]) -> Vec<u8>;

    /// 对压缩后的数据进行解压，恢复原始输入。
    fn decompress(&self, input: &[u8]) -> Result<Vec<u8>, String>;
}

/// 策略 1：不压缩（原样返回）。
struct NoCompression;

impl Compressor for NoCompression {
    fn name(&self) -> &'static str {
        "none"
    }

    fn compress(&self, input: &[u8]) -> Vec<u8> {
        input.to_vec()
    }

    fn decompress(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(input.to_vec())
    }
}

/// 策略 2：RLE（Run-Length Encoding）游程编码。
///
/// 编码格式（非常简化的示例，不追求最优）：
/// - 输出由若干对 (count, byte) 组成，其中 count 为 1..=255。
struct RleCompression;

impl Compressor for RleCompression {
    fn name(&self) -> &'static str {
        "rle"
    }

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

    fn decompress(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        if input.len() % 2 != 0 {
            return Err("invalid rle payload: length must be even".to_string());
        }

        let mut out = Vec::new();
        for pair in input.chunks_exact(2) {
            let count = pair[0];
            let byte = pair[1];
            if count == 0 {
                return Err("invalid rle payload: count must be >= 1".to_string());
            }
            out.extend(std::iter::repeat_n(byte, count as usize));
        }
        Ok(out)
    }
}

/// 策略 3：XOR“压缩”（其实是变换/加密的例子，用来演示策略可替换性）。
///
/// 说明：
/// - 该策略并不真正“压缩”数据（长度不会变），但它满足“算法可替换”的演示目的。
/// - compress/decompress 对称：同一个 XOR 运算执行两次可还原。
struct XorTransform {
    key: u8,
}

impl Compressor for XorTransform {
    fn name(&self) -> &'static str {
        "xor"
    }

    fn compress(&self, input: &[u8]) -> Vec<u8> {
        input.iter().map(|b| b ^ self.key).collect()
    }

    fn decompress(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(input.iter().map(|b| b ^ self.key).collect())
    }
}

/// Context：持有策略，并对外提供统一的“压缩/解压”入口。
struct CompressorContext {
    compressor: Box<dyn Compressor>,
}

impl CompressorContext {
    /// 创建新的 Context。
    fn new(compressor: Box<dyn Compressor>) -> Self {
        Self { compressor }
    }

    /// 运行压缩并打印统计信息。
    fn run(&self, input: &[u8]) -> Result<(), String> {
        let compressed = self.compressor.compress(input);
        let restored = self.compressor.decompress(&compressed)?;

        println!("algo = {}", self.compressor.name());
        println!("original_size   = {}", input.len());
        println!("compressed_size = {}", compressed.len());
        println!("restored_ok     = {}", restored == input);

        Ok(())
    }
}

/// 编译期选择策略（静态分发）的演示：泛型版本。
fn run_static_dispatch<S: Compressor>(strategy: S, input: &[u8]) -> Result<(), String> {
    let compressed = strategy.compress(input);
    let restored = strategy.decompress(&compressed)?;

    println!("[static] algo = {}", strategy.name());
    println!("[static] restored_ok = {}", restored == input);
    Ok(())
}

fn main() -> Result<(), String> {
    let algo = parse_algo();
    let input = demo_input();

    let compressor: Box<dyn Compressor> = match algo.as_str() {
        "none" => Box::new(NoCompression),
        "rle" => Box::new(RleCompression),
        "xor" => Box::new(XorTransform { key: 0xA5 }),
        _ => return Err(format!("unknown algo: {algo} (expected none|rle|xor)")),
    };

    let ctx = CompressorContext::new(compressor);
    ctx.run(&input)?;

    println!();
    run_static_dispatch(RleCompression, &input)?;
    // 策略在编译期就确定 （这里固定是 RleCompression ）
    // 这样通常性能更好（静态分发、可内联），也更符合 Rust 常见“零成本抽象”的风格。
    Ok(())
}

fn parse_algo() -> String {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--algo" {
            return args.next().unwrap_or_else(|| "none".to_string());
        }
    }
    "none".to_string()
}

fn demo_input() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend(std::iter::repeat_n(b'A', 50));
    data.extend(std::iter::repeat_n(b'B', 10));
    data.extend(std::iter::repeat_n(b'C', 80));
    data.extend(std::iter::repeat_n(b'Z', 5));
    data
}
