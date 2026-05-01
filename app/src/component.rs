use ratatui::{layout::Rect, Frame};

use crate::{db::file_db::FileDB, message::Message, update::Update};

pub trait Component {
    type Ctx<'a>;
    /// Returns (state mutations, follow-up messages).
    /// Follow-ups are re-dispatched after the updates apply, routed through View
    /// to whatever screen is then active.
    fn update<'a>(
        &mut self,
        msg: &Message,
        ctx: Self::Ctx<'a>,
        db: &FileDB,
    ) -> (Vec<Update>, Vec<Message>);
    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: Self::Ctx<'a>);
}
