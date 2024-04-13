use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::{
    db::DbConnection,
    model::{Model, Screen},
};

#[derive(Debug)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
    Select,
}

pub fn handle_event<T: DbConnection>(
    model: &Model<T>,
) -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(&model.active_screen, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(active_screen: &Screen, key: event::KeyEvent) -> Option<Message> {
    match active_screen {
        Screen::Main(_) => match key.code {
            KeyCode::Char('w') => Some(Message::Increment),
            KeyCode::Char('s') => Some(Message::Decrement),
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Enter => Some(Message::Select),
            _ => None,
        },
        _ => match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        },
    }
}