//! The static styles for use across the application. Different `Widgets` may access these for
//! various reasons.

use ratatui::style::Style;
use std::sync::LazyLock;

/// The style of the active node.
pub static FOCUSED_NODE_STYLE: LazyLock<Style> =
    LazyLock::new(|| Style::default().fg(ratatui::style::Color::Yellow));
