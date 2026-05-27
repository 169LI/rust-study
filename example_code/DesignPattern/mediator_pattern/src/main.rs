//! 中介者模式 (Mediator Pattern) 示例：GUI 控件事件协调。
//!
//! 这个模块做什么：
//! - 让按钮、文本框、列表不直接互相调用，而是把事件交给 `UiMediator`。
//! - 中介者集中保存交互规则，控件只暴露简单的状态变更方法。
//!
//! 场景说明：
//! - 文本框输入关键字后，中介者刷新列表候选项。
//! - 列表选择项后，中介者更新文本框。
//! - 按钮点击后，中介者根据当前文本提交搜索。

fn main() {
    let mut mediator = UiMediator::new();

    mediator.text_changed("rust");
    mediator.item_selected(1);
    mediator.button_clicked();
}

/// 中介者接口：控件把事件交给这里，而不是直接依赖其他控件。
trait Mediator {
    /// 文本框内容变化事件。
    fn text_changed(&mut self, text: &str);

    /// 列表选择事件。
    fn item_selected(&mut self, index: usize);

    /// 按钮点击事件。
    fn button_clicked(&mut self);
}

/// 具体中介者：集中协调控件之间的交互。
struct UiMediator {
    search_box: TextBox,
    result_list: ResultList,
    submit_button: Button,
}

impl UiMediator {
    /// 创建包含三个控件的中介者。
    fn new() -> Self {
        Self {
            search_box: TextBox::new(),
            result_list: ResultList::new(),
            submit_button: Button::new("Search"),
        }
    }
}

impl Mediator for UiMediator {
    fn text_changed(&mut self, text: &str) {
        self.search_box.set_text(text);
        let items = vec![
            format!("{text} book"),
            format!("{text} tutorial"),
            format!("{text} examples"),
        ];
        self.result_list.set_items(items);
        self.submit_button.set_enabled(!text.trim().is_empty());
        println!("[mediator] refreshed results for {:?}", text);
    }

    fn item_selected(&mut self, index: usize) {
        if let Some(item) = self.result_list.select(index) {
            self.search_box.set_text(&item);
            println!("[mediator] selected item copied to text box");
        }
    }

    fn button_clicked(&mut self) {
        if self.submit_button.enabled {
            println!("[mediator] submit search: {:?}", self.search_box.text);
        } else {
            println!("[mediator] button ignored because it is disabled");
        }
    }
}

/// 同事对象：文本框，只维护自己的文本状态。
struct TextBox {
    text: String,
}

impl TextBox {
    /// 创建空文本框。
    fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    /// 设置文本内容。
    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        println!("[textbox] text={:?}", self.text);
    }
}

/// 同事对象：按钮，只维护标题和可用状态。
struct Button {
    label: String,
    enabled: bool,
}

impl Button {
    /// 创建按钮。
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            enabled: false,
        }
    }

    /// 设置按钮是否可点击。
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        println!("[button] {} enabled={}", self.label, self.enabled);
    }
}

/// 同事对象：列表，只维护候选项和当前选择。
struct ResultList {
    items: Vec<String>,
    selected: Option<usize>,
}

impl ResultList {
    /// 创建空列表。
    fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
        }
    }

    /// 替换列表数据。
    fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.selected = None;
        println!("[list] items={:?}", self.items);
    }

    /// 选择列表项，并返回所选文本。
    fn select(&mut self, index: usize) -> Option<String> {
        if index >= self.items.len() {
            println!("[list] invalid selection index={index}");
            return None;
        }

        self.selected = Some(index);
        let item = self.items[index].clone();
        println!("[list] selected={:?}", item);
        Some(item)
    }
}
