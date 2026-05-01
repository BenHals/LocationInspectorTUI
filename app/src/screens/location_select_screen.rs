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
    query: Option<String>,
}

impl LocationSelectScreen {
    pub fn new(db: &FileDB) -> Self {
        Self {
            idx: 0,
            location_tags: db.get_tags(),
            query: None,
        }
    }

    fn move_up(&mut self) {
        self.idx = self.idx.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if self.idx < self.location_tags.len().saturating_sub(1) {
            self.idx += 1;
        }
    }

    fn select(&mut self, db: &FileDB) -> (Vec<Update>, Vec<Message>) {
        let selected_tag = &self.location_tags[self.idx];
        match db.get_by_id(&selected_tag.id) {
            Some(loc) => {
                self.query = None;
                (vec![Update::SetLocation(loc)], vec![])
            }
            None => (
                vec![Update::SetError(
                    "Location not able to be loaded".to_string(),
                )],
                vec![],
            ),
        }
    }
}

impl Component for LocationSelectScreen {
    type Ctx<'a> = &'a Model;

    fn update(
        &mut self,
        msg: &Message,
        _model: &Model,
        db: &FileDB,
    ) -> (Vec<Update>, Vec<Message>) {
        if let Some(query) = &mut self.query {
            // search mode — chars feed the query, arrows navigate, esc exits
            match msg {
                Message::Char(c) => query.push(*c),
                Message::Backspace => {
                    query.pop();
                }
                Message::Esc => self.query = None,
                Message::Up => self.move_up(),
                Message::Down => self.move_down(),
                Message::Enter => return self.select(db),
                _ => {}
            }
        } else {
            // nav mode — wasd/vim/arrows navigate, '/' enters search
            match msg {
                Message::Up | Message::Char('w') | Message::Char('k') => self.move_up(),
                Message::Down | Message::Char('s') | Message::Char('j') => self.move_down(),
                Message::Char('/') => self.query = Some(String::new()),
                Message::Enter => return self.select(db),
                _ => {}
            }
        }
        (vec![], vec![])
    }

    fn render(&self, frame: &mut Frame, area: Rect, _ctx: &Model) {
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

        let p = match &self.query {
            Some(q) => Paragraph::new(format!("/ {}", q)),
            None => Paragraph::new("Press '/' to search"),
        };
        frame.render_widget(p, layout[1]);
    }
}
