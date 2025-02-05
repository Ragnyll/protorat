/// The app's state
#[derive(Clone, Debug, Default)]
pub struct AppState {
    focused_node: FocusedNode,
    running_state: RunningState,
}

#[derive(Clone, Debug, Default)]
pub enum FocusedNode {
    #[default]
    ProtoExplorer,
    ProtoEditor,
}

impl AppState {
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done;
    }

    pub fn focused_node(&self) -> &FocusedNode {
        &self.focused_node
    }

    pub fn update(&mut self, msg: AppStateUpdate) -> Option<AppStateUpdate> {
        match msg {
            AppStateUpdate::Quit => {
                self.quit();
            }
            AppStateUpdate::FocusWindow(direction) => {
                if let Some(node) = find_next_node(&self.focused_node, &direction) {
                    self.focused_node = node;
                }
            }
        };
        None
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
    FocusWindow(Direction),
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
