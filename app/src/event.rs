use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::message::Message;

pub fn poll_and_handle_event() -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('q') => Some(Message::Quit),
                    _ => None,
                });
            }
        }
    }
    Ok(None)
}
