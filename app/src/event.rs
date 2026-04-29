use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::message::Message;

pub fn poll_and_handle_event() -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('q') => Some(Message::Quit),
                    KeyCode::Up => Some(Message::ListUp),
                    KeyCode::Down => Some(Message::ListDown),
                    KeyCode::Enter => Some(Message::Select),
                    KeyCode::Esc => Some(Message::Back),
                    KeyCode::Char('w') => Some(Message::ShiftUp),
                    KeyCode::Char('s') => Some(Message::ShiftDown),
                    KeyCode::Char('a') => Some(Message::ShiftLeft),
                    KeyCode::Char('d') => Some(Message::ShiftRight),
                    KeyCode::Char('+') => Some(Message::ZoomIn),
                    KeyCode::Char('-') => Some(Message::ZoomOut),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}
