use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
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
    matcher: SkimMatcherV2,
    matches: Vec<usize>,
}

impl LocationSelectScreen {
    pub fn new(db: &FileDB) -> Self {
        let tags = db.get_tags();
        let n_tags = tags.len();
        Self {
            idx: 0,
            location_tags: tags,
            query: None,
            matcher: SkimMatcherV2::default(),
            matches: (0..n_tags).collect(),
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
        let items: Vec<&LocationTag> = self
            .matches
            .iter()
            .map(|&i| &self.location_tags[i])
            .collect();

        let selected_tag = &items[self.idx];
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
                Message::Char(c) => {
                    query.push(*c);
                    if let Some(matches) =
                        recompute_matches(&self.query, &self.location_tags, &self.matcher)
                    {
                        self.matches = matches;
                    }
                }
                Message::Backspace => {
                    query.pop();
                    if let Some(matches) =
                        recompute_matches(&self.query, &self.location_tags, &self.matcher)
                    {
                        self.matches = matches;
                    }
                }
                Message::Esc => {
                    self.query = None;
                    if let Some(matches) =
                        recompute_matches(&self.query, &self.location_tags, &self.matcher)
                    {
                        self.matches = matches;
                    }
                }
                Message::Up => self.move_up(),
                Message::Down => self.move_down(),
                Message::Enter => {
                    let selected = self.select(db);
                    self.query = None;
                    self.matches = (0..self.location_tags.len()).collect();
                    return selected;
                }
                _ => {}
            }
        } else {
            // nav mode — wasd/vim/arrows navigate, '/' enters search
            match msg {
                Message::Up | Message::Char('w') | Message::Char('k') => self.move_up(),
                Message::Down | Message::Char('s') | Message::Char('j') => self.move_down(),
                Message::Char('/') => self.query = Some(String::new()),
                Message::Enter => {
                    let selected = self.select(db);
                    self.query = None;
                    self.matches = (0..self.location_tags.len()).collect();
                    return selected;
                }
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
            .matches
            .iter()
            .map(|&i| {
                let tag = &self.location_tags[i];
                ListItem::new(format!("{} - {}", tag.id, tag.name))
            })
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

fn recompute_matches(
    query: &Option<String>,
    tags: &[LocationTag],
    matcher: &SkimMatcherV2,
) -> Option<Vec<usize>> {
    if let Some(query_str) = query {
        let mut hits: Vec<(usize, i64)> = tags
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                matcher
                    .fuzzy_match(tag_to_search_str(&t), &query_str)
                    .map(|score| (i, score))
            })
            .collect();
        hits.sort_by(|a, b| b.1.cmp(&a.1)); // best first
        let matched_indicies = hits.into_iter().map(|(i, _)| i).collect();
        return Some(matched_indicies);
    }
    None
}

fn tag_to_search_str(tag: &LocationTag) -> &str {
    &tag.name
}
