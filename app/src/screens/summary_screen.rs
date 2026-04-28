use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{
    component::Component,
    db::{db_connection::DBConnection, file_db::FileDB},
    domain::location::{Location, LocationTag},
    message::Message,
    model::Model,
    update::Update,
};

pub struct SummaryScreen {}

impl SummaryScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for SummaryScreen {
    fn update(&mut self, msg: &Message, _model: &Model, db: &FileDB) -> Vec<Update> {
        match msg {
            _ => vec![],
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, model: &Model) {
        let err_str = match &model.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let summary_string = match &model.selected_location {
            Some(location) => format!("Location name {}", location.tag.name),
            _ => "No location selected".to_string(),
        };
        let p = Paragraph::new(format!("Summary for: {}{}", summary_string, err_str));
        frame.render_widget(p, area);
    }
}
