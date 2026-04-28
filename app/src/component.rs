use ratatui::{layout::Rect, Frame};

use crate::{db::file_db::FileDB, message::Message, model::Model, update::Update};

pub trait Component {
    fn update(&mut self, msg: &Message, model: &Model, db: &FileDB) -> Vec<Update>;
    fn render(&self, frame: &mut Frame, area: Rect, model: &Model);
}
