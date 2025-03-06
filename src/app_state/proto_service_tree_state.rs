use tui_tree_widget::{TreeState, TreeItem};

/// The state of the Tree for the proto service tree
#[derive(Debug)]
pub struct ProtoServiceTreeState {
    pub tree_state: TreeState<String>,
    pub tree_items: Vec<TreeItem<'static, String>>,
}

impl Default for ProtoServiceTreeState {
    fn default() -> Self {
        Self {
            tree_state: TreeState::default(),
            tree_items: vec![
                TreeItem::new_leaf(String::from("a"), "Alfa"),
                TreeItem::new(
                    String::from("b"),
                    "Bravo",
                    vec![
                        TreeItem::new_leaf(String::from("c"), "Charlie"),
                        TreeItem::new(
                            String::from("d"),
                            "Delta",
                            vec![
                                TreeItem::new_leaf(String::from("e"), "Echo"),
                                TreeItem::new_leaf(String::from("f"), "Foxtrot"),
                            ],
                        )
                        .expect("all item identifiers are unique"),
                        TreeItem::new_leaf(String::from("g"), "Golf"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf(String::from("h"), "Hotel"),
                TreeItem::new(
                    String::from("i"),
                    "India",
                    vec![
                        TreeItem::new_leaf(String::from("j"), "Juliett"),
                        TreeItem::new_leaf(String::from("k"), "Kilo"),
                        TreeItem::new_leaf(String::from("l"), "Lima"),
                        TreeItem::new_leaf(String::from("m"), "Mike"),
                        TreeItem::new_leaf(String::from("n"), "November"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf(String::from("o"), "Oscar"),
                TreeItem::new(
                    String::from("p"),
                    "Papa",
                    vec![
                        TreeItem::new_leaf(String::from("q"), "Quebec"),
                        TreeItem::new_leaf(String::from("r"), "Romeo"),
                        TreeItem::new_leaf(String::from("s"), "Sierra"),
                        TreeItem::new_leaf(String::from("t"), "Tango"),
                        TreeItem::new_leaf(String::from("u"), "Uniform"),
                        TreeItem::new(
                            String::from("v"),
                            "Victor",
                            vec![
                                TreeItem::new_leaf(String::from("w"), "Whiskey"),
                                TreeItem::new_leaf(String::from("x"), "Xray"),
                                TreeItem::new_leaf(String::from("y"), "Yankee"),
                            ],
                        )
                        .expect("all item identifiers are unique"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf(String::from("z"), "Zulu"),
            ],
        }
    }
}
