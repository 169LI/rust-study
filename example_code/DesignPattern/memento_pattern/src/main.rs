//! 备忘录模式 (Memento Pattern) 示例：文本编辑器快照与撤销/重做。
//!
//! 这个模块做什么：
//! - 在编辑器修改前保存状态快照，使对象可以恢复到历史状态。
//! - `Editor` 负责创建和恢复快照，`History` 只保存快照，不理解编辑器内部逻辑。
//!
//! 场景说明：
//! - `EditorState` 是备忘录 (Memento)，记录文本和光标位置。
//! - `Editor` 是原发器 (Originator)，知道如何保存/恢复自身状态。
//! - `History` 是负责人 (Caretaker)，管理 undo/redo 栈。

fn main() -> Result<(), String> {
    let mut editor = Editor::new();
    let mut history = History::new();

    editor.type_text("Hello", &mut history);
    editor.type_text(", Rust", &mut history);
    editor.move_cursor(5, &mut history)?;
    editor.type_text(" world", &mut history);

    history.undo(&mut editor)?;
    history.undo(&mut editor)?;
    history.redo(&mut editor)?;

    println!("[main] editor={:?}", editor);
    Ok(())
}

/// 备忘录：保存编辑器某一时刻的内部状态。
#[derive(Debug, Clone)]
struct EditorState {
    text: String,
    cursor: usize,
}

/// 原发器：真实业务对象，负责创建和恢复自己的快照。
#[derive(Debug, Default)]
struct Editor {
    text: String,
    cursor: usize,
}

impl Editor {
    /// 创建空编辑器。
    fn new() -> Self {
        Self::default()
    }

    /// 输入文本。修改前先保存快照，保证可以撤销。
    fn type_text(&mut self, text: &str, history: &mut History) {
        history.save(self.create_memento());
        self.text.insert_str(self.cursor, text);
        self.cursor += text.len();
        println!(
            "[editor] type {:?} -> text={:?} cursor={}",
            text, self.text, self.cursor
        );
    }

    /// 移动光标。失败时不保存快照，也不改变状态。
    ///
    /// # Errors
    /// - 当目标位置超过文本长度或不在 UTF-8 字符边界时返回错误。
    fn move_cursor(&mut self, cursor: usize, history: &mut History) -> Result<(), String> {
        if cursor > self.text.len() || !self.text.is_char_boundary(cursor) {
            return Err(format!("invalid cursor: {cursor}"));
        }

        history.save(self.create_memento());
        self.cursor = cursor;
        println!("[editor] move cursor -> {}", self.cursor);
        Ok(())
    }

    fn create_memento(&self) -> EditorState {
        EditorState {
            text: self.text.clone(),
            cursor: self.cursor,
        }
    }

    fn restore(&mut self, state: EditorState) {
        self.text = state.text;
        self.cursor = state.cursor;
        println!(
            "[editor] restore -> text={:?} cursor={}",
            self.text, self.cursor
        );
    }
}

/// 负责人：只管理快照栈，不直接修改快照内容。
#[derive(Debug, Default)]
struct History {
    undo_stack: Vec<EditorState>,
    redo_stack: Vec<EditorState>,
}

impl History {
    /// 创建空历史记录。
    fn new() -> Self {
        Self::default()
    }

    /// 保存一个新快照，并清空 redo 栈。
    fn save(&mut self, state: EditorState) {
        self.undo_stack.push(state);
        self.redo_stack.clear();
    }

    /// 撤销到上一个快照。
    ///
    /// # Errors
    /// - 当没有可撤销快照时返回错误。
    fn undo(&mut self, editor: &mut Editor) -> Result<(), String> {
        let previous = self
            .undo_stack
            .pop()
            .ok_or_else(|| "no state to undo".to_string())?;
        self.redo_stack.push(editor.create_memento());
        editor.restore(previous);
        Ok(())
    }

    /// 重做到最近一次撤销前的快照。
    ///
    /// # Errors
    /// - 当没有可重做快照时返回错误。
    fn redo(&mut self, editor: &mut Editor) -> Result<(), String> {
        let next = self
            .redo_stack
            .pop()
            .ok_or_else(|| "no state to redo".to_string())?;
        self.undo_stack.push(editor.create_memento());
        editor.restore(next);
        Ok(())
    }
}
