//! The static styles for use across the application. Different `Widgets` may access these for
//! various reasons.

use ratatui::style::Style;
use std::sync::LazyLock;

/// The style of focused node in normal mode.
pub static FOCUSED_NORMAL_MODE_NODE_STYLE: LazyLock<Style> =
    LazyLock::new(|| Style::default().fg(ratatui::style::Color::Yellow));

pub static FOCUSED_INSERT_MODE_NODE_STYLE: LazyLock<Style> =
    LazyLock::new(|| Style::default().fg(ratatui::style::Color::LightGreen));
