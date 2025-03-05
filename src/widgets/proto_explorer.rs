use crate::{
    app_state::{proto_explorer_state::ProtoExplorerState, NodeInteractiveState},
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
    type State = ProtoExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Explorer")
            .border_type(ratatui::widgets::BorderType::Rounded);
        match state.node_interactive_state {
            NodeInteractiveState::Idle => (),
            NodeInteractiveState::Focused => block = block.style(*FOCUSED_NORMAL_MODE_NODE_STYLE),
            NodeInteractiveState::Interactive => {
                block = block.style(*FOCUSED_INSERT_MODE_NODE_STYLE)
            }
        };

        block.render(area, buf)
    }
}
