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
        //let items = &state.items;
        //let widget = Tree::new(&items)
        //.expect("all item identifiers are unique")
        //.block(
        //Block::bordered()
        //.title("Tree Widget")
        //.title_bottom(format!("{:?}", state.tree_state)),
        //)
        //.experimental_scrollbar(Some(
        //Scrollbar::new(ScrollbarOrientation::VerticalRight)
        //.begin_symbol(None)
        //.track_symbol(None)
        //.end_symbol(None),
        //))
        //.highlight_style(
        //Style::new()
        //.fg(Color::Black)
        //.bg(Color::LightGreen)
        //.add_modifier(Modifier::BOLD),
        //)
        //.highlight_symbol(">> ");
        //widget.render(area, buf, &mut state.tree_state);
        block.render(area, buf)
    }
}
