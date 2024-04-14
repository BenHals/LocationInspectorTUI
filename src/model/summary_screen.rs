use crate::{
    db::DbConnection,
    event_handling::Message,
    model::{AppState, RunningState},
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

pub fn summary_screen_update(
    _db: &dyn DbConnection,
    msg: &Message,
    screen: &SummaryScreen,
) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match msg {
        Message::Quit => (
            AppState {
                app_state: RunningState::Done,
                active_screen: super::Screen::Summary(screen.clone()),
            },
            None,
        ),
        _ => (
            AppState {
                app_state: RunningState::Running,
                active_screen: super::Screen::Summary(screen.clone()),
            },
            None,
        ),
    };
    (next_state, next_msg)
}
