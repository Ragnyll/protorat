use crate::app_state::{
    proto_explorer_state::ProtoExplorerState, proto_editor_state::ProtoEditorState, AppNodeNames,
    NodeInteractiveState,
};
/// A HashMap Like structure specifically for getting the individual widgets states.
#[derive(Debug, Default)]
pub struct AppNodes {
    pub proto_explorer: ProtoExplorerState,
    pub proto_editor: ProtoEditorState,
}

impl AppNodes {
    pub fn get_proto_explorer_state(&mut self) -> &mut ProtoExplorerState {
        &mut self.proto_explorer
    }

    pub fn get_proto_editor_state(&mut self) -> &mut ProtoEditorState {
        &mut self.proto_editor
    }

    /// Get the node that is either focused or in interactive mod
    pub fn get_active_node_name(&self) -> Option<AppNodeNames> {
        if matches!(
            self.proto_explorer.node_interactive_state,
            NodeInteractiveState::Focused
        ) || matches!(
            self.proto_explorer.node_interactive_state,
            NodeInteractiveState::Interactive
        ) {
            return Some(AppNodeNames::ProtoExplorer);
        }
        if matches!(
            self.proto_editor.node_interactive_state,
            NodeInteractiveState::Focused
        ) || matches!(
            self.proto_editor.node_interactive_state,
            NodeInteractiveState::Interactive
        ) {
            return Some(AppNodeNames::ProtoEditor);
        }
        None
    }

    /// Update the given focused node, make all other nodes idle.
    pub fn update_focused_node(&mut self, new_focused_node: &AppNodeNames) {
        match new_focused_node {
            AppNodeNames::ProtoEditor => {
                self.proto_editor.node_interactive_state = NodeInteractiveState::Focused;
                self.proto_explorer.node_interactive_state = NodeInteractiveState::Idle;
            }
            AppNodeNames::ProtoExplorer => {
                self.proto_explorer.node_interactive_state = NodeInteractiveState::Focused;
                self.proto_editor.node_interactive_state = NodeInteractiveState::Idle;
            }
        }
    }
}
