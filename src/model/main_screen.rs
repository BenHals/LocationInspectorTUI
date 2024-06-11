use crate::{
    db::DbConnection,
    event_handling::Message,
    model::summary_screen::SummaryScreen,
    model::{AppState, RunningState, Screen},
};
use geo::Point;

use super::screens::SelectedScreen;

#[derive(Debug, Clone)]
pub struct MainScreen {
    pub key: usize,
    pub id: Option<String>,
    pub err_msg: Option<String>,
}

impl MainScreen {
    pub fn new(key: usize, id: Option<String>) -> Self {
        Self {
            key,
            id,
            err_msg: None,
        }
    }
    pub fn clear_err(self: &mut Self) -> () {
        self.err_msg = None
    }
}

pub fn main_screen_update(
    db: &dyn DbConnection,
    msg: &Message,
    screen: &MainScreen,
) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match msg {
        Message::Increment => {
            let key = screen.key + 1;
            (
                AppState {
                    app_state: RunningState::Running,
                    active_screen: Screen::Main(MainScreen {
                        key,
                        id: db.get_id(&key),
                        err_msg: None,
                    }),
                },
                None,
            )
        }
        Message::Decrement => {
            let mut key = screen.key.clone();
            let mut err_msg: Option<String> = None;
            if key > 0 {
                key -= 1;
            } else {
                err_msg = Some("No More IDs!".to_string());
            }
            (
                AppState {
                    app_state: RunningState::Running,
                    active_screen: Screen::Main(MainScreen {
                        key,
                        id: db.get_id(&key),
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
                        app_state: RunningState::Running,
                        active_screen: Screen::Main(MainScreen {
                            key: screen.key,
                            id: db.get_id(&screen.key),
                            err_msg: Some("No valid ID found".to_string()),
                        }),
                    },
                    None,
                ),
                Some(id) => {
                    let name = db.get_name(&id);
                    let coord = db.get_latlng(&id);
                    (
                        AppState {
                            app_state: RunningState::Running,
                            active_screen: Screen::Summary(SummaryScreen {
                                id: id.clone(),
                                name,
                                coord,
                                map_offset: Point::new(0.0, 0.0),
                                map_scale: 1.0,
                                selected_screen: SelectedScreen::Summary,
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
                app_state: RunningState::Running,
                active_screen: Screen::Main(MainScreen {
                    key: 0,
                    id: db.get_id(&0),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::Quit => {
            let key = screen.key;
            (
                AppState {
                    app_state: RunningState::Done,
                    active_screen: Screen::Main(MainScreen {
                        key,
                        id: db.get_id(&key),
                        err_msg: None,
                    }),
                },
                None,
            )
        }
        _ => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Main(screen.clone()),
            },
            None,
        ),
    };
    (next_state, next_msg)
}
