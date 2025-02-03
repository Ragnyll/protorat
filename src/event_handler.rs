use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};
use crate::app_state::{AppState, AppStateUpdate, Direction};

pub fn handle_event(_: &AppState) -> std::io::Result<Option<AppStateUpdate>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

pub fn handle_key(key: event::KeyEvent) -> Option<AppStateUpdate> {
    if matches!(key.code, KeyCode::Char('q')) {
        return Some(AppStateUpdate::Quit);
    }
    // I seperated this I soon need to have the concept of a "mode".
    match key.code {
        KeyCode::Char('h') => Some(AppStateUpdate::FocusWindow(Direction::Left)),
        KeyCode::Char('l') => Some(AppStateUpdate::FocusWindow(Direction::Right)),
        _ => None,
    }
}
