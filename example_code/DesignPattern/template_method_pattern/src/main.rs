//! 模板方法模式 (Template Method Pattern) 示例：文件处理流程模板
//!
//! 这个模块做什么：
//! - 在 trait 默认方法中定义固定的处理流程（模板方法），并将可变步骤留给具体实现类型定制。
//!
//! 场景说明：
//! - 固定流程：读取输入 -> 解析 -> 处理 -> 写入输出。
//! - CSV 与 JSON 处理器共享相同流程，但解析与处理步骤不同。

/// 文件处理器模板（在 trait 默认方法中提供算法骨架）。
trait FileProcessor {
    /// 模板方法：固定整体流程顺序。
    fn process(&self, input: &str) -> String {
        let raw = self.read(input);
        let parsed = self.parse(&raw);
        let handled = self.handle(parsed);
        self.write(handled)
    }

    /// 读取步骤：默认实现直接返回输入。
    fn read(&self, input: &str) -> String {
        input.to_string()
    }

    /// 解析步骤：由具体实现类型定制。
    fn parse(&self, raw: &str) -> Vec<String>;

    /// 处理步骤：由具体实现类型定制。
    fn handle(&self, items: Vec<String>) -> Vec<String>;

    /// 写入步骤：默认实现将处理结果拼成一段文本。
    fn write(&self, items: Vec<String>) -> String {
        items.join("\n")
    }
}

/// CSV 文件处理器：按逗号分割并做去空白处理。
struct CsvProcessor;

impl FileProcessor for CsvProcessor {
    fn parse(&self, raw: &str) -> Vec<String> {
        raw.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn handle(&self, items: Vec<String>) -> Vec<String> {
        items.into_iter().map(|s| s.to_uppercase()).collect()
    }
}

/// JSON 文件处理器（示例用）：只实现一个非常简化的解析逻辑，不依赖第三方库。
///
/// 预期输入格式示例：`["a","b","c"]`
struct JsonProcessor;

impl FileProcessor for JsonProcessor {
    fn parse(&self, raw: &str) -> Vec<String> {
        let trimmed = raw.trim();
        if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
            return Vec::new();
        }

        let inner = &trimmed[1..trimmed.len() - 1];
        inner
            .split(',')
            .map(|s| s.trim().trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn handle(&self, items: Vec<String>) -> Vec<String> {
        items
            .into_iter()
            .enumerate()
            .map(|(idx, s)| format!("{idx}: {s}"))
            .collect()
    }
}

fn main() {
    let csv_input = " alice, bob ,  carol,";
    let json_input = r#"["alice","bob","carol"]"#;

    let csv = CsvProcessor;
    let json = JsonProcessor;

    println!("--- CSV Processor Output ---");
    println!("{}", csv.process(csv_input));
    println!();

    println!("--- JSON Processor Output ---");
    println!("{}", json.process(json_input));
}
