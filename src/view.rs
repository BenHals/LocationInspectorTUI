use ratatui::{widgets::Paragraph, Frame};

use crate::{
    db::DbConnection,
    model::summary_screen::SummaryScreen,
    model::{Model, Screen},
};

pub fn view<T: DbConnection>(model: &mut Model<T>, f: &mut Frame) {
    match &model.state.active_screen {
        Screen::Main(screen) => {
            let id = match &screen.id {
                None => "No matching id found".to_string(),
                Some(n) => n.clone(),
            };
            let err_str = match &screen.err_msg {
                Some(msg) => msg.clone(),
                None => "No errors".to_string(),
            };
            f.render_widget(
                Paragraph::new(format!(
                    "Key: {}, id: {}, err: {}",
                    model.state.key, id, err_str
                )),
                f.size(),
            )
        }
        Screen::Summary(SummaryScreen { id, name, err_msg }) => {
            let name_str = match name {
                Some(n) => n.clone(),
                None => "No Name Found".to_string(),
            };
            let err_str = match err_msg {
                Some(msg) => msg.clone(),
                None => "No errors".to_string(),
            };
            f.render_widget(
                Paragraph::new(format!("Id: {}, Name: {}, err: {}", id, name_str, err_str)),
                f.size(),
            )
        }
    }
}
