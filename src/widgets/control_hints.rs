use ratatui::{
    widgets::{StatefulWidget, Widget, Paragraph},
    layout::Rect,
    buffer::Buffer,
};
use crate::app_state::control_hint_state::ControlHintState;

/// A Widget providing the user with hints for some controls they have access to in the current
/// node.
#[derive(Default)]
pub struct ControlHints {}

impl StatefulWidget for ControlHints {
    type State = ControlHintState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let controls = Paragraph::new(state.get_hint());
        controls.render(area, buf);
    }
}
