use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{component::Component, message::Message, model::Model, update::Update};

pub struct LocationSelectScreen {
    pub key: usize,
}

impl LocationSelectScreen {
    pub fn new() -> Self {
        Self { key: 0 }
    }
}

impl Component for LocationSelectScreen {
    fn update(&mut self, msg: &Message, _model: &Model) -> Vec<Update> {
        match msg {
            Message::ListUp => {
                self.key += 1;
                vec![]
            }
            Message::ListDown => {
                if self.key > 0 {
                    self.key -= 1;
                    vec![]
                } else {
                    vec![Update::SetError("No more keys!".to_string())]
                }
            }
            _ => vec![],
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, model: &Model) {
        let err_str = match &model.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let p = Paragraph::new(format!("Key: {}{}", self.key, err_str));
        frame.render_widget(p, area);
    }
}
