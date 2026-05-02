use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::message::Message;

pub fn poll_and_handle_event() -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Message::Quit),
                    (KeyCode::Char(c), _) => Some(Message::Char(c)),
                    (KeyCode::Backspace, _) => Some(Message::Backspace),
                    (KeyCode::Up, _) => Some(Message::Up),
                    (KeyCode::Down, _) => Some(Message::Down),
                    (KeyCode::Left, _) => Some(Message::Left),
                    (KeyCode::Right, _) => Some(Message::Right),
                    (KeyCode::Enter, _) => Some(Message::Enter),
                    (KeyCode::Esc, _) => Some(Message::Esc),
                    (KeyCode::Tab, _) => Some(Message::Tab),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}
