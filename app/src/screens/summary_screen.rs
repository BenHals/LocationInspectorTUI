use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{
    component::Component, db::file_db::FileDB, domain::location::Location, message::Message,
    update::Update,
};

pub struct SummaryScreenCtx<'a> {
    pub location: &'a Location,
    pub err: &'a Option<String>,
}
pub struct SummaryScreen {}

impl SummaryScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for SummaryScreen {
    type Ctx<'a> = SummaryScreenCtx<'a>;
    fn update(&mut self, msg: &Message, _ctx: SummaryScreenCtx, _db: &FileDB) -> Vec<Update> {
        match msg {
            Message::Back => vec![Update::ClearLocation],
            _ => vec![],
        }
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: SummaryScreenCtx<'a>) {
        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let summary_string = format!("Location name {}", ctx.location.tag.name);
        let p = Paragraph::new(format!("Summary for: {}{}", summary_string, err_str));
        frame.render_widget(p, area);
    }
}
