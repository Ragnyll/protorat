use crate::app_state::AppStateUpdate;

const DEFAULT_CONTROL_HINTS: &str = "Controls: h: Move Left, l: Move Right, i: Enter Node, Q: Quit";

/// Other nodes can supply `Hint`s for what the user's controls are at a given time.
pub trait Hint {
    fn get_hint(&self) -> String;
}

/// The state of the ControlHint Widget
#[derive(Debug)]
pub struct ControlHintState {
    hint: String,
}

impl Default for ControlHintState {
    fn default() -> Self {
        Self {
            hint: String::from(DEFAULT_CONTROL_HINTS),
        }
    }
}

impl ControlHintState {
    pub fn get_hint(&self) -> &str {
        &self.hint
    }

    /// Updates the active hint for the active node.
    pub fn update_hint(&mut self, hint: String) -> Option<AppStateUpdate> {
        self.hint = hint;
        None
    }
}
