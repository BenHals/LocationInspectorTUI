use crate::{
    db::DbConnection,
    event_handling::Message,
    model::summary_screen::SummaryScreen,
    model::{AppState, ApplicationState, Model, Screen},
};

#[derive(Debug, Clone)]
pub struct MainScreen {
    pub id: Option<String>,
    pub err_msg: Option<String>,
}

impl MainScreen {
    pub fn new(id: Option<String>) -> Self {
        Self { id, err_msg: None }
    }
    pub fn clear_err(self: &mut Self) -> () {
        self.err_msg = None
    }
}

pub fn main_screen_update<T: DbConnection>(
    model: &Model<T>,
    msg: &Message,
    screen: &MainScreen,
) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match msg {
        Message::Increment => {
            let key = model.state.key + 1;
            (
                AppState {
                    key,
                    app_state: ApplicationState::Running,
                    active_screen: Screen::Main(MainScreen {
                        id: model.db.get_id(&key),
                        err_msg: None,
                    }),
                },
                None,
            )
        }
        Message::Decrement => {
            let mut key = model.state.key.clone();
            let mut err_msg: Option<String> = None;
            if key > 0 {
                key -= 1;
            } else {
                err_msg = Some("No More IDs!".to_string());
            }
            (
                AppState {
                    key,
                    app_state: ApplicationState::Running,
                    active_screen: Screen::Main(MainScreen {
                        id: model.db.get_id(&key),
                        err_msg,
                    }),
                },
                None,
            )
        }
        Message::Select => {
            let id = &screen.id;
            match id {
                None => (
                    AppState {
                        key: model.state.key.clone(),
                        app_state: ApplicationState::Running,
                        active_screen: Screen::Main(MainScreen {
                            id: model.db.get_id(&model.state.key),
                            err_msg: Some("No valid ID found".to_string()),
                        }),
                    },
                    None,
                ),
                Some(id) => {
                    let key = model.state.key.clone();
                    let name = model.db.get_name(&id);
                    (
                        AppState {
                            key,
                            app_state: ApplicationState::Running,
                            active_screen: Screen::Summary(SummaryScreen {
                                id: id.clone(),
                                name,
                                err_msg: None,
                            }),
                        },
                        None,
                    )
                }
            }
        }
        Message::Reset => (
            AppState {
                key: 0,
                app_state: ApplicationState::Running,
                active_screen: Screen::Main(MainScreen {
                    id: model.db.get_id(&0),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::Quit => {
            let key = model.state.key;
            (
                AppState {
                    key,
                    app_state: ApplicationState::Done,
                    active_screen: Screen::Main(MainScreen {
                        id: model.db.get_id(&key),
                        err_msg: None,
                    }),
                },
                None,
            )
        }
    };
    (next_state, next_msg)
}
