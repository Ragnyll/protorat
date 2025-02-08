use crate::{
    app_state::AppState,
    widgets::{proto_explorer::ProtoExplorer, proto_editor::ProtoEditor},
};
use ratatui::{
    layout::{Constraint, Layout, Direction},
    widgets::Paragraph,
    Frame,
};

/// Lays out how the widgets will render on the terminal.
pub fn ui(frame: &mut Frame, app_state: &AppState) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let row_0_columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rows[0]);
    let row_1_columns = rows[1];

    let proto_explorer = ProtoExplorer::default();
    let proto_editor = ProtoEditor::default();
    let controls = Paragraph::new("Controls");

    frame.render_stateful_widget(proto_explorer, row_0_columns[0], &mut app_state.clone());
    frame.render_stateful_widget(proto_editor, row_0_columns[1], &mut app_state.clone());
    frame.render_widget(controls, row_1_columns)
}
