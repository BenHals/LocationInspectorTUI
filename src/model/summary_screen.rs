use geo::{Centroid, Point};

use crate::{
    db::DbConnection,
    event_handling::Message,
    model::Screen,
    model::{AppState, RunningState},
};

use super::main_screen::MainScreen;
use super::screens::SelectedScreen;

#[derive(Debug, Clone)]
pub struct SummaryScreen {
    pub id: String,
    pub name: Option<String>,
    pub coord: Option<Point>,
    pub map_offset: Point,
    pub map_scale: f64,
    pub selected_screen: SelectedScreen,
    pub err_msg: Option<String>,
}

impl SummaryScreen {
    pub fn new(id: String, name: Option<String>, coord: Option<Point>) -> Self {
        Self {
            id,
            name,
            coord,
            map_offset: Point::new(0.0, 0.0),
            map_scale: 1.0,
            selected_screen: SelectedScreen::Summary,
            err_msg: None,
        }
    }
    pub fn clear_err(self: &mut Self) -> () {
        self.err_msg = None
    }
}

pub fn summary_screen_update(
    db: &dyn DbConnection,
    msg: &Message,
    screen: &SummaryScreen,
) -> (AppState, Option<Message>) {
    let (next_state, next_msg) = match msg {
        Message::ZoomIn => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: screen.map_offset.clone(),
                    map_scale: screen.map_scale - 0.1,
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::ZoomOut => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: screen.map_offset.clone(),
                    map_scale: screen.map_scale + 0.1,
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::ShiftUp => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: Point::new(screen.map_offset.x(), screen.map_offset.y() + 1.0),
                    map_scale: screen.map_scale.clone(),
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::ShiftLeft => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: Point::new(screen.map_offset.x() - 1.0, screen.map_offset.y()),
                    map_scale: screen.map_scale.clone(),
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::ShiftDown => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: Point::new(screen.map_offset.x(), screen.map_offset.y() - 1.0),
                    map_scale: screen.map_scale.clone(),
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::ShiftRight => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: Point::new(screen.map_offset.x() + 1.0, screen.map_offset.y()),
                    map_scale: screen.map_scale.clone(),
                    selected_screen: screen.selected_screen.clone(),
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::Tab => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: screen.map_offset.clone(),
                    map_scale: screen.map_scale.clone(),
                    selected_screen: match screen.selected_screen {
                        SelectedScreen::Main => SelectedScreen::Summary,
                        SelectedScreen::Summary => SelectedScreen::Inspect,
                        SelectedScreen::Inspect => SelectedScreen::Main,
                    },
                    err_msg: None,
                }),
            },
            None,
        ),
        Message::Select => (
            match screen.selected_screen {
                SelectedScreen::Main => AppState {
                    app_state: RunningState::Running,
                    active_screen: super::Screen::Main(MainScreen {
                        key: 0,
                        id: None,
                        err_msg: Some("Select Location".to_string()),
                    }),
                },
                SelectedScreen::Summary => AppState {
                    app_state: RunningState::Running,
                    active_screen: Screen::Summary(screen.clone()),
                },
                SelectedScreen::Inspect => {
                    let polys = db.get_polygons(&screen.id).unwrap();
                    let center = polys[0].centroid().unwrap();
                    AppState {
                        app_state: RunningState::Running,
                        active_screen: Screen::Inspect(super::inspect_screen::InspectScreen {
                            id: screen.id.clone(),
                            name: screen.name.clone(),
                            coord: Some(center),
                            polygons: polys,
                            map_offset: Point::new(0.0, 0.0),
                            map_scale: 1.0,
                            selected_screen: SelectedScreen::Inspect,
                            err_msg: None,
                        }),
                    }
                }
            },
            None,
        ),
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
