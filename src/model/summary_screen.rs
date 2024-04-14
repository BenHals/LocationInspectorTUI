use crate::{
    db::DbConnection,
    event_handling::Message,
    model::{AppState, ApplicationState, Model},
};

#[derive(Debug, Clone)]
pub struct SummaryScreen {
    pub id: String,
    pub name: Option<String>,
    pub err_msg: Option<String>,
}

impl SummaryScreen {
    pub fn new(id: String, name: Option<String>) -> Self {
        Self {
            id,
            name,
            err_msg: None,
        }
    }
    pub fn clear_err(self: &mut Self) -> () {
        self.err_msg = None
    }
}

pub fn summary_screen_update<T: DbConnection>(
    model: &Model<T>,
    msg: &Message,
    _screen: &SummaryScreen,
) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match msg {
        Message::Quit => {
            let key = model.state.key;
            (
                AppState {
                    key,
                    app_state: ApplicationState::Done,
                    active_screen: model.state.active_screen.clone(),
                },
                None,
            )
        }
        _ => (model.state.clone(), None),
    };
    (next_state, next_msg)
}
