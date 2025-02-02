use ratatui::Frame;

use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
};

/// Lays out how the widgets will render on the terminal.
pub fn ui(frame: &mut Frame) {
    let area = frame.area();
    let window_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let block_left = Block::bordered()
        .title_top("Left")
        .border_type(ratatui::widgets::BorderType::Rounded);
    let block_right = Block::bordered()
        .title_top("Right")
        .border_type(ratatui::widgets::BorderType::Rounded);

    frame.render_widget(&block_left, window_chunks[0]);
    frame.render_widget(&block_right, window_chunks[1]);
}
