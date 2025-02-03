use crate::app_state::{AppState, Node};
use std::sync::LazyLock;
use ratatui::{
    style::Style,
    Frame,
    layout::{Constraint, Layout},
    widgets::Block,
};

/// The style of the active node.
static ACTIVE_NODE_STYLE: LazyLock<Style> =
    LazyLock::new(|| Style::default().fg(ratatui::style::Color::Yellow));

/// Lays out how the widgets will render on the terminal.
pub fn ui(frame: &mut Frame, app_state: &AppState) {
    let area = frame.area();

    let window_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let mut block_left = Block::bordered()
        .title_top("Left")
        .border_type(ratatui::widgets::BorderType::Rounded);
    let mut block_right = Block::bordered()
        .title_top("Right")
        .border_type(ratatui::widgets::BorderType::Rounded);

    match app_state.focused_node() {
        Node::Left => {
            block_left = block_left.border_style(*ACTIVE_NODE_STYLE);
        }
        Node::Right => {
            block_right = block_right.border_style(*ACTIVE_NODE_STYLE);
        }
    };

    frame.render_widget(&block_left, window_chunks[0]);
    frame.render_widget(&block_right, window_chunks[1]);
}
