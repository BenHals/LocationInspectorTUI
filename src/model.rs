pub mod main_screen;
pub mod summary_screen;
use crate::{db::DbConnection, event_handling::Message};
use main_screen::{main_screen_update, MainScreen};
use summary_screen::SummaryScreen;

use self::summary_screen::summary_screen_update;

#[derive(Debug, Clone)]
pub enum Screen {
    Main(MainScreen),
    Summary(SummaryScreen),
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub key: usize,
    pub app_state: ApplicationState,
    pub active_screen: Screen,
}

#[derive(Debug)]
pub struct Model<D: DbConnection> {
    pub state: AppState,
    db: D,
}

impl<D: DbConnection> Model<D> {
    pub fn new(db: D) -> Self {
        Self {
            state: AppState {
                key: 0,
                app_state: ApplicationState::Running,
                active_screen: Screen::Main(MainScreen {
                    id: db.get_id(&0),
                    err_msg: None,
                }),
            },
            db,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ApplicationState {
    Running,
    Loading,
    Done,
    Crashed,
}

pub fn update<T: DbConnection>(model: Model<T>, msg: Message) -> (Model<T>, Option<Message>) {
    let (next_state, next_msg) = match &model.state.active_screen {
        Screen::Main(screen) => main_screen_update(&model, &msg, screen),
        Screen::Summary(screen) => summary_screen_update(&model, &msg, &screen),
    };
    (
        Model {
            state: next_state,
            db: model.db,
        },
        next_msg,
    )
}
