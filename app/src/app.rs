use ratatui::Frame;

use crate::{component::Component, message::Message, model::Model, view::View};

pub struct App {
    pub model: Model,
    pub view: View,
}

impl App {
    pub fn new() -> Self {
        Self {
            model: Model::new(),
            view: View {},
        }
    }

    pub fn handle(&mut self, msg: Message) {
        self.model.err = None;
        let updates = self.view.update(&msg, &self.model);
        for u in updates {
            self.model.apply(u);
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        self.view.render(frame, area, &self.model);
    }
}
