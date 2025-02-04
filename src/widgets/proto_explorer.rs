use crate::{
    app_state::{AppState, FocusedNode},
    static_styles::FOCUSED_NODE_STYLE,
};
use ratatui::{
    widgets::{StatefulWidget, Block, Widget},
    layout::Rect,
    buffer::Buffer,
};

pub struct ProtoExplorer {
    content: String,
}

impl Default for ProtoExplorer {
    fn default() -> Self {
        Self {
            content: String::from("Jake"),
        }
    }
}

impl StatefulWidget for ProtoExplorer {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Left")
            .border_type(ratatui::widgets::BorderType::Rounded);
        if matches!(state.focused_node(), FocusedNode::ProtoExplorer) {
            block = block.style(*FOCUSED_NODE_STYLE);
        }
        block.render(area, buf);
    }
}
