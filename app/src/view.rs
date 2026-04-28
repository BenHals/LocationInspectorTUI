use ratatui::{layout::Rect, Frame};

use crate::{
    component::Component,
    db::file_db::FileDB,
    message::Message,
    model::{Model, ScreenType},
    screens::location_select_screen::LocationSelectScreen,
    update::Update,
};

pub struct View {
    pub location_select_screen: LocationSelectScreen,
}

impl View {
    pub fn new(db: &FileDB) -> Self {
        Self {
            location_select_screen: LocationSelectScreen::new(db),
        }
    }
}

impl Component for View {
    fn update(&mut self, msg: &Message, model: &Model, db: &FileDB) -> Vec<Update> {
        match msg {
            Message::Quit => return vec![Update::Quit],
            _ => {}
        }
        match model.screen {
            ScreenType::LocationSelect => self.location_select_screen.update(msg, model, db),
            _ => vec![],
        }
    }
    fn render(&self, frame: &mut Frame, area: Rect, model: &Model) {
        match model.screen {
            ScreenType::LocationSelect => self.location_select_screen.render(frame, area, model),
            _ => (),
        }
    }
}
