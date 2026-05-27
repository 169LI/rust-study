//! 命令模式 (Command Pattern) 示例：文本编辑器的“命令队列 + undo/redo”
//!
//! 这个模块做什么：
//! - 把“对文本的操作”封装成独立的命令对象（Command），命令可以被保存、传递、排队与重放。
//! - 调用方只负责产生/调度命令，不需要直接操作 `Editor`（Receiver）的具体 API。
//!
//! 场景说明：
//! - `Editor` 是接收者 (Receiver)：真正修改文本内容。
//! - `Command` 是命令接口：统一 `execute/undo`，便于放入同一队列。
//! - `CommandInvoker` 是调用者/调度器 (Invoker)：负责排队执行、记录历史并提供 undo/redo。

use std::collections::VecDeque;
use std::ops::Range;

fn main() -> Result<(), String> {
    let editor = Editor::new();
    let mut invoker = CommandInvoker::new(editor);

    invoker.enqueue(Box::new(AppendTextCommand::new("Hello".to_string())));
    invoker.enqueue(Box::new(AppendTextCommand::new(", ".to_string())));
    invoker.enqueue(Box::new(AppendTextCommand::new("World".to_string())));
    invoker.run_all()?;

    invoker.enqueue(Box::new(DeleteLastCommand::new(5)));
    invoker.run_all()?;

    invoker.undo()?;
    invoker.undo()?;
    invoker.redo()?;

    invoker.enqueue(Box::new(AppendTextCommand::new("!".to_string())));
    invoker.run_all()?;

    println!("[main] done: text={:?}", invoker.editor().text());
    Ok(())
}

/// 接收者 (Receiver)：真正执行业务逻辑（修改文本）。
#[derive(Debug, Default)]
struct Editor {
    text: String,
}

impl Editor {
    /// 创建一个空编辑器。
    fn new() -> Self {
        Self::default()
    }

    /// 读取当前文本快照。
    fn text(&self) -> &str {
        &self.text
    }

    /// 当前文本的字节长度。
    fn len(&self) -> usize {
        self.text.len()
    }

    /// 在指定位置插入文本。
    fn insert_at(&mut self, index: usize, content: &str) -> Result<(), String> {
        if index > self.text.len() {
            return Err(format!(
                "insert_at out of range: index={index} len={}",
                self.text.len()
            ));
        }
        if !self.text.is_char_boundary(index) {
            return Err(format!("insert_at not at char boundary: index={index}"));
        }
        self.text.insert_str(index, content);
        Ok(())
    }

    /// 删除指定区间的文本，并返回被删除的内容。
    fn delete_range(&mut self, range: Range<usize>) -> Result<String, String> {
        if range.start > range.end || range.end > self.text.len() {
            return Err(format!(
                "delete_range out of range: start={} end={} len={}",
                range.start,
                range.end,
                self.text.len()
            ));
        }
        if !self.text.is_char_boundary(range.start) || !self.text.is_char_boundary(range.end) {
            return Err(format!(
                "delete_range not at char boundary: start={} end={}",
                range.start, range.end
            ));
        }

        let deleted = self.text[range.clone()].to_string();
        self.text.replace_range(range, "");
        Ok(deleted)
    }
}

/// 命令接口：把一次操作封装为对象，支持执行与撤销。
trait Command {
    /// 命令名称，用于打印执行日志。
    fn name(&self) -> &'static str;

    /// 执行命令：对接收者应用一次操作。
    fn execute(&mut self, editor: &mut Editor) -> Result<(), String>;

    /// 撤销命令：对接收者回滚一次操作。
    fn undo(&mut self, editor: &mut Editor) -> Result<(), String>;
}

/// 具体命令：追加文本。
#[derive(Debug)]
struct AppendTextCommand {
    text: String,
    start: Option<usize>,
}

impl AppendTextCommand {
    /// 创建“追加文本”命令。
    fn new(text: String) -> Self {
        Self { text, start: None }
    }
}

impl Command for AppendTextCommand {
    fn name(&self) -> &'static str {
        "AppendText"
    }

    fn execute(&mut self, editor: &mut Editor) -> Result<(), String> {
        let start = editor.len();
        editor.insert_at(start, &self.text)?;
        self.start = Some(start);
        println!(
            "[execute] cmd={} start={} text={:?} -> editor={:?}",
            self.name(),
            start,
            self.text,
            editor.text()
        );
        Ok(())
    }

    fn undo(&mut self, editor: &mut Editor) -> Result<(), String> {
        let start = self
            .start
            .ok_or_else(|| format!("cannot undo {}: never executed", self.name()))?;
        let end = start + self.text.len();
        editor.delete_range(start..end)?;
        println!(
            "[undo] cmd={} start={} text={:?} -> editor={:?}",
            self.name(),
            start,
            self.text,
            editor.text()
        );
        Ok(())
    }
}

/// 具体命令：删除末尾 N 个字节（用于演示 undo/redo；示例输入为 ASCII 文本）。
#[derive(Debug)]
struct DeleteLastCommand {
    count: usize,
    start: Option<usize>,
    deleted: Option<String>,
}

impl DeleteLastCommand {
    /// 创建“删除末尾 N 个字节”命令。
    fn new(count: usize) -> Self {
        Self {
            count,
            start: None,
            deleted: None,
        }
    }
}

impl Command for DeleteLastCommand {
    fn name(&self) -> &'static str {
        "DeleteLast"
    }

    fn execute(&mut self, editor: &mut Editor) -> Result<(), String> {
        let len = editor.len();
        let actual = self.count.min(len);
        let start = len - actual;
        let deleted = editor.delete_range(start..len)?;
        self.start = Some(start);
        self.deleted = Some(deleted.clone());
        println!(
            "[execute] cmd={} start={} count={} deleted={:?} -> editor={:?}",
            self.name(),
            start,
            self.count,
            deleted,
            editor.text()
        );
        Ok(())
    }

    fn undo(&mut self, editor: &mut Editor) -> Result<(), String> {
        let start = self
            .start
            .ok_or_else(|| format!("cannot undo {}: never executed", self.name()))?;
        let deleted = self
            .deleted
            .as_deref()
            .ok_or_else(|| format!("cannot undo {}: missing deleted text", self.name()))?;
        editor.insert_at(start, deleted)?;
        println!(
            "[undo] cmd={} start={} restored={:?} -> editor={:?}",
            self.name(),
            start,
            deleted,
            editor.text()
        );
        Ok(())
    }
}

/// 调用者/调度器 (Invoker)：负责排队执行命令，并记录历史用于 undo/redo。
struct CommandInvoker {
    editor: Editor,
    queue: VecDeque<Box<dyn Command>>,
    history: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandInvoker {
    /// 创建一个命令调度器，并绑定接收者。
    fn new(editor: Editor) -> Self {
        Self {
            editor,
            queue: VecDeque::new(),
            history: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// 访问接收者（只读），用于查看结果。
    fn editor(&self) -> &Editor {
        &self.editor
    }

    /// 把命令当成“数据”加入队列，稍后再执行。
    fn enqueue(&mut self, command: Box<dyn Command>) {
        println!("[enqueue] cmd={}", command.name());
        self.queue.push_back(command);
    }

    /// 执行队列中的一个命令（如果存在）。
    fn run_next(&mut self) -> Result<bool, String> {
        let Some(mut command) = self.queue.pop_front() else {
            return Ok(false);
        };

        let name = command.name();
        command.execute(&mut self.editor)?;
        self.history.push(command);
        self.redo_stack.clear();
        self.print_status(&format!("after execute {name}"));
        Ok(true)
    }

    /// 执行队列中的全部命令。
    fn run_all(&mut self) -> Result<(), String> {
        while self.run_next()? {}
        Ok(())
    }

    /// 撤销最近一次执行的命令。
    fn undo(&mut self) -> Result<(), String> {
        let Some(mut command) = self.history.pop() else {
            return Err("no commands to undo".to_string());
        };

        let name = command.name();
        command.undo(&mut self.editor)?;
        self.redo_stack.push(command);
        self.print_status(&format!("after undo {name}"));
        Ok(())
    }

    /// 重做最近一次被撤销的命令。
    fn redo(&mut self) -> Result<(), String> {
        let Some(mut command) = self.redo_stack.pop() else {
            return Err("no commands to redo".to_string());
        };

        let name = command.name();
        command.execute(&mut self.editor)?;
        self.history.push(command);
        self.print_status(&format!("after redo {name}"));
        Ok(())
    }

    fn print_status(&self, label: &str) {
        println!(
            "[status] {label}: queue={} history={} redo={} text={:?}",
            self.queue.len(),
            self.history.len(),
            self.redo_stack.len(),
            self.editor.text()
        );
    }
}
