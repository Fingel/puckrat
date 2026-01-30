use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::message::Message;
use crate::model::Model;

pub fn handle_event(_: &Model) -> io::Result<Option<Message>> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(handle_key(key_event)),
        _ => Ok(None),
    }
}

fn handle_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::MoveDown),
        KeyCode::Char('k') => Some(Message::MoveUp),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}
