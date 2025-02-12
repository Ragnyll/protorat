/// The app's state
#[derive(Debug)]
pub struct AppState {
    pub mode: UserMode,
    pub focused_node: FocusedNode,
    pub running_state: RunningState,
    pub proto_editor_state: ProtoEditorState,
    pub proto_explorer_state: ProtoExplorerState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: UserMode::default(),
            focused_node: FocusedNode::default(),
            running_state: RunningState::default(),
            proto_editor_state: ProtoEditorState::default(),
            proto_explorer_state: ProtoExplorerState::default(),
        }
    }
}

///  The node that is currently focused on.
///  You may ask "isnt that information also in the individul node's states?" and to that I say
///  yes. But having this at the top level ui state is much easier than checking everything's state
///  just for navigation. So fk off.
#[derive(Default, Debug, Clone)]
pub enum FocusedNode {
    #[default]
    ProtoExplorer,
    ProtoEditor,
}

/// The modes used to interact with different nodes within the application. These are roughly based
/// off vim, so use a similar terminology.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum UserMode {
    /// The normal mode used to navigate from node to node.
    #[default]
    Normal,
    /// The mode to "insert" in the given block.
    Insert,
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

/// The state of the ProtoExplorer.
#[derive(Clone, Debug)]
pub struct ProtoExplorerState {
    pub node_interactive_state: NodeInteractiveState,
}

impl Default for ProtoExplorerState {
    fn default() -> Self {
        Self {
            node_interactive_state: NodeInteractiveState::Focused,
        }
    }
}

/// The state of the ProtoEditor.
#[derive(Default, Clone, Debug)]
pub struct ProtoEditorState {
    pub node_interactive_state: NodeInteractiveState,
}

impl AppState {
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done;
    }

    pub fn mode(&self) -> &UserMode {
        &self.mode
    }

    pub fn update(&mut self, msg: AppStateUpdate) -> Option<AppStateUpdate> {
        match msg {
            AppStateUpdate::Quit => {
                self.quit();
            }
            AppStateUpdate::FocusNode(direction) => {
                if let Some(new_focused_node) = find_next_node(&self.focused_node, &direction) {
                    self.focused_node = new_focused_node.clone();
                    self.update_nodes_focus_states(new_focused_node);
                }
            }
            AppStateUpdate::ChangeMode(next_mode) => {
                self.mode = next_mode.clone();
                self.update_nodes_mode_states(next_mode);
            }
        };
        None
    }

    fn update_nodes_focus_states(&mut self, new_focused_node: FocusedNode) {
        match new_focused_node {
            FocusedNode::ProtoExplorer => {
                self.proto_explorer_state.node_interactive_state = NodeInteractiveState::Focused;
                self.proto_editor_state.node_interactive_state = NodeInteractiveState::Idle;
            }
            FocusedNode::ProtoEditor => {
                self.proto_editor_state.node_interactive_state = NodeInteractiveState::Focused;
                self.proto_explorer_state.node_interactive_state = NodeInteractiveState::Idle;
            }
        }
    }

    fn update_nodes_mode_states(&mut self, next_mode: UserMode) {
        match self.focused_node {
            FocusedNode::ProtoExplorer => match next_mode {
                UserMode::Normal => {
                    self.proto_explorer_state.node_interactive_state =
                        NodeInteractiveState::Focused;
                    self.proto_editor_state.node_interactive_state = NodeInteractiveState::Idle;
                }
                UserMode::Insert => {
                    self.proto_explorer_state.node_interactive_state =
                        NodeInteractiveState::Interactive;
                    self.proto_editor_state.node_interactive_state = NodeInteractiveState::Idle;
                }
            },
            FocusedNode::ProtoEditor => match next_mode {
                UserMode::Normal => {
                    self.proto_editor_state.node_interactive_state = NodeInteractiveState::Focused;
                    self.proto_explorer_state.node_interactive_state = NodeInteractiveState::Idle;
                }
                UserMode::Insert => {
                    self.proto_editor_state.node_interactive_state =
                        NodeInteractiveState::Interactive;
                    self.proto_explorer_state.node_interactive_state = NodeInteractiveState::Idle;
                }
            },
        }
    }
}

/// Finds the next node to traverse to given the direction. If the given direction cannot validly
/// traverse to a different node then `None` is returned.
fn find_next_node(current_node: &FocusedNode, direction: &Direction) -> Option<FocusedNode> {
    match current_node {
        FocusedNode::ProtoExplorer => match direction {
            Direction::Right => Some(FocusedNode::ProtoEditor),
            _ => None,
        },
        FocusedNode::ProtoEditor => match direction {
            Direction::Left => Some(FocusedNode::ProtoExplorer),
            _ => None,
        },
    }
}

/// The Commands used to change the AppState.
#[derive(PartialEq, Debug, Clone)]
pub enum AppStateUpdate {
    /// Update the state to focus on a different Node.
    FocusNode(Direction),
    /// Change the state of the app to the given mode.
    ChangeMode(UserMode),
    /// Quit the app.
    Quit,
}

/// A relative direciton
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    #[allow(unused)]
    Up,
    #[allow(unused)]
    Down,
    Left,
    Right,
}

/// Whether the App is runnign or not.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
