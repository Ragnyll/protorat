use crate::app_state::{AppState, AppStateUpdate, Direction, UserMode};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn handle_event(app_state: &AppState) -> std::io::Result<Option<AppStateUpdate>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            // no matter the mode, `Q` (capital Q) should always quit
            if is_quit_request(&key) {
                return Ok(Some(AppStateUpdate::Quit));
            }
            // handle everything else by the mode.
            match app_state.mode() {
                UserMode::Normal => return Ok(handle_normal_mode_key_event(&key)),
                UserMode::Insert => return Ok(handle_insert_mode_key_event(&key)),
            }
        }
    }
    Ok(None)
}

/// Handles key events receieved in normal `Mode::Normal`.
fn handle_normal_mode_key_event(key: &KeyEvent) -> Option<AppStateUpdate> {
    match key.code {
        KeyCode::Char('h') => Some(AppStateUpdate::FocusNode(Direction::Left)),
        KeyCode::Char('l') => Some(AppStateUpdate::FocusNode(Direction::Right)),
        KeyCode::Char('i') => Some(AppStateUpdate::ChangeMode(UserMode::Insert)),
        _ => None,
    }
}

fn handle_insert_mode_key_event(key: &KeyEvent) -> Option<AppStateUpdate> {
    match key.code {
        KeyCode::Esc => Some(AppStateUpdate::ChangeMode(UserMode::Normal)),
        _ => None,
    }
}

fn is_quit_request(key: &KeyEvent) -> bool {
    matches!(key.kind, event::KeyEventKind::Press) && matches!(key.code, KeyCode::Char('Q'))
}
