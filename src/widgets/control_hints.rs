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
/// node.
pub struct ControlHints {}

impl Widget for ControlHints {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let controls = get_control_hints_for_mode();
        controls.render(area, buf);
    }
}

fn get_control_hints_for_mode() -> Paragraph<'static> {
        Paragraph::new(NORMAL_MODE_CONTROL_HINTS)
}
