use crate::{
    app_state::AppState,
    widgets::{
        proto_explorer::ProtoExplorer, proto_editor::ProtoEditor, control_hints::ControlHints,
    },
};
use ratatui::{
    layout::{Constraint, Layout, Direction},
    Frame,
};

/// Lays out how the widgets will render on the terminal.
pub fn ui(frame: &mut Frame, app_state: &mut AppState) {
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
    let control_hints = ControlHints::default();

    frame.render_stateful_widget(
        proto_explorer,
        row_0_columns[0],
        app_state.app_node_states.get_proto_explorer_state(),
    );
    frame.render_stateful_widget(
        proto_editor,
        row_0_columns[1],
        app_state.app_node_states.get_proto_editor_state(),
    );
    frame.render_stateful_widget(
        control_hints,
        row_1_columns,
        app_state.get_control_hints_state(),
    )
}
