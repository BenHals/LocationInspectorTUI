use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

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
    fn update(&mut self, msg: &Message, _model: &Model, db: &FileDB) -> (Vec<Update>, Vec<Message>) {
        match msg {
            Message::ListDown => {
                if self.idx < self.location_tags.len().saturating_sub(1) {
                    self.idx += 1;
                }
                (vec![], vec![])
            }
            Message::ListUp => {
                if self.idx > 0 {
                    self.idx -= 1;
                }
                (vec![], vec![])
            }
            Message::Select => {
                let selected_tag = &self.location_tags[self.idx];
                let selected_location = db.get_by_id(&selected_tag.id);
                if let Some(loc) = selected_location {
                    (vec![Update::SetLocation(loc)], vec![])
                } else {
                    (
                        vec![Update::SetError("Location not able to be loaded".to_string())],
                        vec![],
                    )
                }
            }
            _ => (vec![], vec![]),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, ctx: &Model) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);
        let items: Vec<ListItem> = self
            .location_tags
            .iter()
            .map(|l| ListItem::new(format!("{} - {}", l.id, l.name)))
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::all()).title("Locations"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("▶ ");
        let mut list_state = ListState::default();
        list_state.select(Some(self.idx));
        frame.render_stateful_widget(list, layout[0], &mut list_state);

        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let p = Paragraph::new(format!(
            "Location: {}{}",
            self.location_tags[self.idx].name, err_str
        ));
        frame.render_widget(p, layout[1]);
    }
}
