use crate::app_state::AppStateUpdate;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn handle_event() -> std::io::Result<Option<AppStateUpdate>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            // no matter the mode, `Q` (capital Q) should always quit
            if is_quit_request(&key) {
                return Ok(Some(AppStateUpdate::Quit));
            }
            // Let the other individual app_state handlers handle
            return Ok(Some(AppStateUpdate::SendKey(key)));
        }
    }
    Ok(None)
}

fn is_quit_request(key: &KeyEvent) -> bool {
    matches!(key.kind, event::KeyEventKind::Press) && matches!(key.code, KeyCode::Char('Q'))
}
