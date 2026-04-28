use ratatui::{layout::Rect, Frame};

use crate::{db::file_db::FileDB, message::Message, update::Update};

pub trait Component {
    type Ctx<'a>;
    fn update<'a>(&mut self, msg: &Message, ctx: Self::Ctx<'a>, db: &FileDB) -> Vec<Update>;
    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: Self::Ctx<'a>);
}
