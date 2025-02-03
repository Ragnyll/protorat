/// The app's state
#[derive(Clone, Debug, Default)]
pub struct AppState {
    focused_node: Node,
    running_state: RunningState,
}

#[derive(Clone, Debug, Default)]
pub enum Node {
    #[default]
    Left,
    Right,
}

impl AppState {
    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done;
    }

    pub fn focused_node(&self) -> &Node {
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
fn find_next_node(current_node: &Node, direction: &Direction) -> Option<Node> {
    match current_node {
        Node::Left => match direction {
            Direction::Right => Some(Node::Right),
            _ => None,
        },
        Node::Right => match direction {
            Direction::Left => Some(Node::Left),
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
    Up,
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
