use ratatui::{layout::Rect, Frame};

use crate::{component::Component, message::Message, model::Model, update::Update};

pub struct View {}

impl Component for View {
    fn update(&mut self, msg: &Message, model: &Model) -> Vec<Update> {
        match msg {
            Message::Quit => vec![Update::Quit],
            _ => vec![],
        }
    }
    fn render(&self, frame: &mut Frame, area: Rect, model: &Model) {}
}
