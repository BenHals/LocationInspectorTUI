use std::sync::mpsc;

use ratatui::Frame;

use crate::{
    component::Component, config::Config, db::file_db::FileDB, message::Message, model::Model,
    update::Update, view::View,
};

pub struct App {
    pub model: Model,
    pub view: View,
    pub db: FileDB,
    pub async_tx: mpsc::Sender<Update>,
}

impl App {
    pub fn new(db: FileDB, config: Config, async_tx: mpsc::Sender<Update>) -> Self {
        Self {
            model: Model::new(config),
            view: View::new(&db),
            db,
            async_tx,
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
