use crate::app_state::Mode;
use ratatui::{
    widgets::{StatefulWidget, Widget, Paragraph},
    layout::Rect,
    buffer::Buffer,
};
use std::collections::HashMap;

/// A Widget providing the user with hints for some controls they have access to in the current
/// mode.
pub struct ControlHints {
    current_mode: Mode,
}

impl ControlHints {
    /// Creates a new instance of `ControlHints`.
    pub fn new(current_mode: Mode) -> Self {
        Self { current_mode }
    }
}

impl Widget for ControlHints {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let controls = get_control_hints_for_mode(&self.current_mode);
        controls.render(area, buf);
    }
}

fn get_control_hints_for_mode(mode: &Mode) -> Paragraph<'static> {
    match mode {
        Mode::Normal => get_normal_mode_control_hints(),
        Mode::Insert => get_insert_mode_control_hints(),
    }
}

fn get_normal_mode_control_hints() -> Paragraph<'static> {
    let mut controls = HashMap::new();
    controls.insert("h", String::from("Move Left"));
    controls.insert("l", String::from("Move Right"));
    controls.insert("i", String::from("Enter Node"));
    controls.insert("Q", String::from("Quit"));
    let controls = collect_to_control_string(controls);
    Paragraph::new(format!("Controls: {controls}"))
}

fn get_insert_mode_control_hints() -> Paragraph<'static> {
    let mut controls = HashMap::new();
    controls.insert("<Esc>", String::from("Normal Mode"));
    controls.insert("Q", String::from("Quit"));
    let controls = collect_to_control_string(controls);
    Paragraph::new(format!("Controls: {controls}"))
}

fn collect_to_control_string(controls: HashMap<&str, String>) -> String {
    controls
        .into_iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join(", ")
}
