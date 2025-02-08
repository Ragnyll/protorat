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
pub struct ProtoExplorer {}

impl StatefulWidget for ProtoExplorer {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Explorer")
            .border_type(ratatui::widgets::BorderType::Rounded);
        if matches!(state.focused_node(), FocusedNode::ProtoExplorer) {
            block = block.style(*FOCUSED_NORMAL_MODE_NODE_STYLE);
            if matches!(state.mode(), Mode::Insert) {
                block = block.style(*FOCUSED_INSERT_MODE_NODE_STYLE);
            }
        }
        block.render(area, buf)
    }
}
