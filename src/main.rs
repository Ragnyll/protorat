use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::Style,
    text,
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, ListItem, List},
    DefaultTerminal, Frame,
    text::{Span, Line},
    style::Modifier,
};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
};
use std::{
    thread,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    current_node: CurrentNode,
}

#[derive(Default, Debug)]
pub enum CurrentNode {
    #[default]
    Left,
    Right,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tasks = ["A", "B", "C", "D"];
        while !self.exit {
            terminal.draw(|frame| {
                let area = frame.area();
                let window_chunks = Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

                // Draw tasks
                let tasks: Vec<ListItem> = tasks
                    .iter()
                    .map(|i| ListItem::new(vec![text::Line::from(*i)]))
                    .collect();
                let tasks = List::new(tasks)
                    .block(Block::bordered().title("List"))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol("> ");

                let mut block_left = Block::bordered().title_top("Left").border_type(ratatui::widgets::BorderType::Rounded);
                let mut block_right = Block::bordered().title_top("Right").border_type(ratatui::widgets::BorderType::Rounded);

                let active_style = Style::default().fg(ratatui::style::Color::Blue);
                match self.current_node {
                    CurrentNode::Left => block_left = block_left.border_style(active_style),
                    CurrentNode::Right => block_right = block_right.border_style(active_style),
                };

                frame.render_widget(&block_left, window_chunks[0]);
                frame.render_widget(&tasks, window_chunks[0]);
                frame.render_widget(&block_right, window_chunks[1]);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('h') => self.current_node = CurrentNode::Left,
            KeyCode::Char('l') => self.current_node = CurrentNode::Right,
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
