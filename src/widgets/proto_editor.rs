use crate::{
    app_state::{AppState, FocusedNode},
    styling::FOCUSED_NODE_STYLE,
};
use ratatui::{
    widgets::{StatefulWidget, Block, Widget},
    layout::Rect,
    buffer::Buffer,
};

pub struct ProtoEditor {
    content: String,
}

impl Default for ProtoEditor {
    fn default() -> Self {
        Self {
            content: String::from("Gallow"),
        }
    }
}

impl StatefulWidget for ProtoEditor {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Editor")
            .border_type(ratatui::widgets::BorderType::Rounded);
        if matches!(state.focused_node(), FocusedNode::ProtoEditor) {
            block = block.style(*FOCUSED_NODE_STYLE);
        }
        block.render(area, buf)
    }
}
