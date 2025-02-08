use crate::app_state::Mode;
use ratatui::{
    widgets::{Widget, Paragraph},
    layout::Rect,
    buffer::Buffer,
};

/// The hints for controls that can be used in `Mode::Normal`
const NORMAL_MODE_CONTROL_HINTS: &str =
    "Controls: h: Move Left, l: Move Right, i: Enter Node, Q: Quit";
const INSERT_MODE_CONTROL_HINTS: &str = "Controls: <Esc>: NormalMode, Q: Quit";

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
        Mode::Normal => Paragraph::new(NORMAL_MODE_CONTROL_HINTS),
        Mode::Insert => Paragraph::new(INSERT_MODE_CONTROL_HINTS),
    }
}
