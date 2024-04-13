use ratatui::{widgets::Paragraph, Frame};

use crate::{
    db::DbConnection,
    model::{MainScreen, Model, Screen, SummaryScreen},
};

pub fn view<T: DbConnection>(model: &mut Model<T>, f: &mut Frame) {
    match &model.active_screen {
        Screen::Main(MainScreen {}) => {
            let id = match model.get_id() {
                None => "No matching id found".to_string(),
                Some(n) => n,
            };
            f.render_widget(
                Paragraph::new(format!("Key: {}, id: {}", model.key, id)),
                f.size(),
            )
        }
        Screen::Summary(SummaryScreen { id }) => {
            let name = match model.get_name() {
                None => "No Name Found".to_string(),
                Some(n) => n,
            };
            f.render_widget(
                Paragraph::new(format!("Id: {}, Name: {}", id, name)),
                f.size(),
            )
        }
    }
}
