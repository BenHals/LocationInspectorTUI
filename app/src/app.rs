use ratatui::Frame;

use crate::{
    component::Component, db::file_db::FileDB, message::Message, model::Model, view::View,
};

pub struct App {
    pub model: Model,
    pub view: View,
    pub db: FileDB,
}

impl App {
    pub fn new(db: FileDB) -> Self {
        Self {
            model: Model::new(),
            view: View::new(&db),
            db,
        }
    }

    pub fn handle(&mut self, msg: Message) {
        self.model.err = None;
        let updates = self.view.update(&msg, &self.model, &self.db);
        for u in updates {
            self.model.apply(u);
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        self.view.render(frame, area, &self.model);
    }
}
