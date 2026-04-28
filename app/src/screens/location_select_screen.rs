use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{
    component::Component,
    db::{db_connection::DBConnection, file_db::FileDB},
    domain::location::LocationTag,
    message::Message,
    model::Model,
    update::Update,
};

pub struct LocationSelectScreen {
    idx: usize,
    location_tags: Vec<LocationTag>,
}

impl LocationSelectScreen {
    pub fn new(db: &FileDB) -> Self {
        Self {
            idx: 0,
            location_tags: db.get_tags(),
        }
    }
}

impl Component for LocationSelectScreen {
    type Ctx<'a> = &'a Model;
    fn update(&mut self, msg: &Message, _model: &Model, db: &FileDB) -> Vec<Update> {
        match msg {
            Message::ListUp => {
                if self.idx < self.location_tags.len() - 1 {
                    self.idx += 1;
                    vec![]
                } else {
                    vec![Update::SetError("End of locations!".to_string())]
                }
            }
            Message::ListDown => {
                if self.idx > 0 {
                    self.idx -= 1;
                    vec![]
                } else {
                    vec![Update::SetError("No more keys!".to_string())]
                }
            }
            Message::Select => {
                let selected_tag = &self.location_tags[self.idx];
                let selected_location = db.get_by_id(&selected_tag.id);
                if let Some(loc) = selected_location {
                    vec![Update::SetLocation(loc)]
                } else {
                    vec![Update::SetError(
                        "Location not able to be loaded".to_string(),
                    )]
                }
            }
            _ => vec![],
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, ctx: &Model) {
        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let p = Paragraph::new(format!(
            "Location: {}{}",
            self.location_tags[self.idx].name, err_str
        ));
        frame.render_widget(p, area);
    }
}
