use ratatui::{layout::Rect, Frame};

use crate::{message::Message, model::Model, update::Update};

pub trait Component {
    fn update(&mut self, msg: &Message, model: &Model) -> Vec<Update>;
    fn render(&self, frame: &mut Frame, area: Rect, model: &Model);
}
