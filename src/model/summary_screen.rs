use geo::Point;

use crate::{
    db::DbConnection,
    event_handling::Message,
    model::Screen,
    model::{AppState, RunningState},
};

#[derive(Debug, Clone)]
pub struct SummaryScreen {
    pub id: String,
    pub name: Option<String>,
    pub coord: Option<Point>,
    pub map_offset: Point,
    pub map_scale: f64,
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
        Message::ZoomIn => (
            AppState {
                app_state: RunningState::Running,
                active_screen: Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord.clone(),
                    map_offset: screen.map_offset.clone(),
                    map_scale: screen.map_scale - 0.1,
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
                    err_msg: None,
                }),
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
