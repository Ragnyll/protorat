use crate::app_state::{NodeEventHandler, AppStateUpdate, NodeInteractiveState, AppNodeNames};
use crossterm::event::{KeyEvent, KeyCode};

/// The state of the ProtoEditor.
#[derive(Clone, Debug)]
pub struct ProtoEditorState {
    pub node_interactive_state: NodeInteractiveState,
}

impl Default for ProtoEditorState {
    fn default() -> Self {
        Self {
            node_interactive_state: NodeInteractiveState::Idle,
        }
    }
}

impl NodeEventHandler for ProtoEditorState {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<AppStateUpdate> {
        match self.node_interactive_state {
            NodeInteractiveState::Idle => unreachable!(
                "There should be no state that allows a node to handle any events while idle."
            ),
            NodeInteractiveState::Focused => self.handle_focused_key_events(key_event),
            NodeInteractiveState::Interactive => self.handle_interactive_key_events(key_event),
        }
    }
}

impl ProtoEditorState {
    /// Handle events while the node is in `NodeInteractiveState::Focused`.
    /// This should not do any modification to the interactive state of the calling object. The app
    /// state update is in charge of that. This will only dipsatch other events. The app state
    fn handle_focused_key_events(&mut self, key_event: KeyEvent) -> Option<AppStateUpdate> {
        match key_event.code {
            KeyCode::Char('h') => {
                self.node_interactive_state = NodeInteractiveState::Idle;
                Some(AppStateUpdate::FocusNode(AppNodeNames::ProtoExplorer))
            }
            KeyCode::Char('i') => {
                self.node_interactive_state = NodeInteractiveState::Interactive;
                None
            }
            _ => None,
        }
    }

    /// Handle events while the node is in `NodeInteractiveState::Interactive`.
    fn handle_interactive_key_events(&mut self, key_event: KeyEvent) -> Option<AppStateUpdate> {
        match key_event.code {
            KeyCode::Esc => {
                self.node_interactive_state = NodeInteractiveState::Focused;
                None
            }
            _ => None,
        }
    }
}
