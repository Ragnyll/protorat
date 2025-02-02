mod ui;

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
        while !self.exit {
            terminal.draw(|frame| {
                ui::ui(frame);
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
