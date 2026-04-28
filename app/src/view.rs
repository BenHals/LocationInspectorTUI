use ratatui::{layout::Rect, Frame};

use crate::{
    component::Component,
    message::Message,
    model::{Model, ScreenType},
    screens::location_select_screen::LocationSelectScreen,
    update::Update,
};

pub struct View {
    pub location_select_screen: LocationSelectScreen,
}

impl View {
    pub fn new() -> Self {
        Self {
            location_select_screen: LocationSelectScreen::new(),
        }
    }
}

impl Component for View {
    fn update(&mut self, msg: &Message, model: &Model) -> Vec<Update> {
        match msg {
            Message::Quit => return vec![Update::Quit],
            _ => {}
        }
        match model.screen {
            ScreenType::LocationSelect => self.location_select_screen.update(msg, model),
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
