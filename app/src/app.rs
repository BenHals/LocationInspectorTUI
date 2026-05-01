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
        let mut queue: Vec<Message> = vec![msg];
        while let Some(m) = queue.pop() {
            let (updates, follow_ups) = self.view.update(&m, &self.model, &self.db);
            for u in updates {
                self.model.apply(u);
            }
            queue.extend(follow_ups);
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        self.view.render(frame, area, &self.model);
    }
}
