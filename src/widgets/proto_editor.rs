use crate::{
    app_state::{AppState, FocusedNode, Mode},
    styling::*,
};
use ratatui::{
    widgets::{StatefulWidget, Block, Widget},
    layout::Rect,
    buffer::Buffer,
};

#[derive(Default)]
pub struct ProtoEditor {}

impl StatefulWidget for ProtoEditor {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Editor")
            .border_type(ratatui::widgets::BorderType::Rounded);
        if matches!(state.focused_node(), FocusedNode::ProtoEditor) {
            block = block.style(*FOCUSED_NORMAL_MODE_NODE_STYLE);
            if matches!(state.mode(), Mode::Insert) {
                block = block.style(*FOCUSED_INSERT_MODE_NODE_STYLE);
            }
        }
        block.render(area, buf)
    }
}
