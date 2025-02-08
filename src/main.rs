mod event_handler;
use event_handler::handle_event;
mod app_state;
use app_state::{AppState, RunningState};
mod ui;
use ui::ui;
mod styling;
mod widgets;

fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let mut app_state = AppState::default();

    while !matches!(app_state.running_state(), RunningState::Done) {
        terminal.draw(|frame| ui(frame, &app_state))?;
        let mut current_msg = handle_event(&app_state)?;
        while current_msg.is_some() {
            current_msg = app_state.update(current_msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}
