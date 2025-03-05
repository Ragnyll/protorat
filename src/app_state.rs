pub mod app_nodes;
pub mod proto_editor_state;
pub mod proto_explorer_state;

use ratatui::crossterm::event::KeyEvent;
use std::fmt::Debug;
use app_nodes::AppNodes;

/// The app's state
#[derive(Debug)]
pub struct AppState {
    pub running_state: RunningState,
    pub app_node_states: AppNodes,
}

/// Handle events forwarded by the base event handler.
pub trait NodeEventHandler {
    /// The given node handles key events sent to it.
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<AppStateUpdate>;
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running_state: RunningState::default(),
            app_node_states: AppNodes::default(),
        }
    }
}

/// The names of the nodes in the application.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppNodeNames {
    #[default]
    ProtoExplorer,
    ProtoEditor,
}

/// The states a node can be in from the ui's perspective.
#[derive(Default, Clone, Debug)]
pub enum NodeInteractiveState {
    /// The user is not focused on this node.
    #[default]
    Idle,
    /// The user is focused on this node.
    Focused,
    /// The user is interacting with this node.
    Interactive,
}

impl AppState {
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done;
    }

    pub fn update(&mut self, msg: AppStateUpdate) -> Option<AppStateUpdate> {
        match msg {
            AppStateUpdate::Quit => {
                self.quit();
                None
            }
            AppStateUpdate::SendKey(key_event) => self.handle_key_event_by_node(key_event),
            AppStateUpdate::FocusNode(next_node_name) => {
                self.update_node_focused_state(next_node_name);
                None
            }
        }
    }

    fn handle_key_event_by_node(&mut self, key_event: KeyEvent) -> Option<AppStateUpdate> {
        let active_node = self.app_node_states.get_active_node_name().expect(
            "There should not be a route to handling key events while there is no focued node. Actually i guess this could happen if you input a bunch of events hand it in between update cycles somehow. that seems unlikely i think.",
        );
        match active_node {
            AppNodeNames::ProtoExplorer => self
                .app_node_states
                .proto_explorer
                .handle_key_event(key_event),
            AppNodeNames::ProtoEditor => self
                .app_node_states
                .proto_editor
                .handle_key_event(key_event),
        }
    }

    fn update_node_focused_state(&mut self, next_node: AppNodeNames) {
        match next_node {
            AppNodeNames::ProtoExplorer => {
                self.app_node_states.proto_explorer.node_interactive_state =
                    NodeInteractiveState::Focused
            }
            AppNodeNames::ProtoEditor => {
                self.app_node_states.proto_editor.node_interactive_state =
                    NodeInteractiveState::Focused;
            }
        }
    }
}

/// The Commands used to change the AppState.
#[derive(PartialEq, Debug, Clone)]
pub enum AppStateUpdate {
    /// Update the state to focus on a different Node.
    FocusNode(AppNodeNames),
    ///  Send a key to a node to have it handled by the app state
    SendKey(KeyEvent),
    /// Quit the app.
    Quit,
}

/// Whether the App is runnign or not.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
