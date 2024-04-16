use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::model::{AppState, Screen};

#[derive(Debug)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
    Select,
    ShiftLeft,
    ShiftRight,
    ShiftDown,
    ShiftUp,
    ZoomIn,
    ZoomOut,
}

pub fn handle_event(state: &AppState) -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(&state.active_screen, key));
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
        Screen::Summary(_) => match key.code {
            KeyCode::Char('w') => Some(Message::ShiftUp),
            KeyCode::Char('a') => Some(Message::ShiftLeft),
            KeyCode::Char('s') => Some(Message::ShiftDown),
            KeyCode::Char('d') => Some(Message::ShiftRight),
            KeyCode::Char('i') => Some(Message::ZoomIn),
            KeyCode::Char('o') => Some(Message::ZoomOut),
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        },
        // _ => match key.code {
        //     KeyCode::Char('q') => Some(Message::Quit),
        //     _ => None,
        // },
    }
}
