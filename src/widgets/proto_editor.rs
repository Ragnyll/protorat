use crate::{
    app_state::{proto_editor_state::ProtoEditorState, NodeInteractiveState},
    styling::*,
};
use ratatui::{
    widgets::{Block, Widget, StatefulWidget},
    layout::Rect,
    buffer::Buffer,
};

/// An interface for editing requests in protobuf.
#[derive(Default)]
pub struct ProtoEditor {}

impl StatefulWidget for ProtoEditor {
    type State = ProtoEditorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut block = Block::bordered()
            .title_top("Editor")
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
