pub mod inspect_screen;
pub mod main_screen;
pub mod screens;
pub mod summary_screen;
use crate::{db::DbConnection, event_handling::Message};
use main_screen::{main_screen_update, MainScreen};
use summary_screen::SummaryScreen;

use self::{
    inspect_screen::{inspect_screen_update, InspectScreen},
    summary_screen::summary_screen_update,
};

#[derive(Debug, Clone)]
pub enum Screen {
    Main(MainScreen),
    Summary(SummaryScreen),
    Inspect(InspectScreen),
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub app_state: RunningState,
    pub active_screen: Screen,
}

impl AppState {
    pub fn initial_app_state(db: &dyn DbConnection) -> Self {
        Self {
            app_state: RunningState::Running,
            active_screen: Screen::Main(MainScreen {
                key: 0,
                id: db.get_id(&0),
                err_msg: None,
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RunningState {
    Running,
    Loading,
    Done,
    Crashed,
}

pub fn update(state: AppState, msg: Message, db: &dyn DbConnection) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match &state.active_screen {
        Screen::Main(screen) => main_screen_update(db, &msg, screen),
        Screen::Summary(screen) => summary_screen_update(db, &msg, &screen),
        Screen::Inspect(screen) => inspect_screen_update(db, &msg, &screen),
    };
    (next_state, next_msg)
}
