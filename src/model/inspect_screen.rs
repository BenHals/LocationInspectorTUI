use geo::{Point, Polygon};

use crate::{db::DbConnection, event_handling::Message, model::Screen};

use super::main_screen::MainScreen;
use super::screens::SelectedScreen;
use super::summary_screen::SummaryScreen;

#[derive(Debug, Clone)]
pub struct InspectScreen {
    pub id: String,
    pub name: Option<String>,
    pub coord: Option<Point>,
    pub polygons: Vec<Polygon>,
    pub map_offset: Point,
    pub map_scale: f64,
    pub selected_screen: SelectedScreen,
    pub err_msg: Option<String>,
}

impl InspectScreen {
    pub fn new(id: String, name: Option<String>, coord: Option<Point>) -> Self {
        Self {
            id,
            name,
            coord,
            polygons: vec![],
            map_offset: Point::new(0.0, 0.0),
            map_scale: 1.0,
            selected_screen: SelectedScreen::Inspect,
            err_msg: None,
        }
    }
    pub fn clear_err(&mut self) {
        self.err_msg = None
    }
}

pub fn inspect_screen_update(
    _db: &dyn DbConnection,
    msg: &Message,
    screen: &InspectScreen,
) -> (Screen, Option<Message>) {
    let (next_screen, next_msg) = match msg {
        Message::ZoomIn => (
            Screen::Inspect(InspectScreen {
                map_scale: screen.map_scale * 0.9,
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ZoomOut => (
            Screen::Inspect(InspectScreen {
                map_scale: screen.map_scale * 1.1,
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftUp => (
            Screen::Inspect(InspectScreen {
                map_offset: Point::new(
                    screen.map_offset.x(),
                    screen.map_offset.y() + screen.map_scale,
                ),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftLeft => (
            Screen::Inspect(InspectScreen {
                map_offset: Point::new(
                    screen.map_offset.x() - screen.map_scale,
                    screen.map_offset.y(),
                ),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftDown => (
            Screen::Inspect(InspectScreen {
                map_offset: Point::new(
                    screen.map_offset.x(),
                    screen.map_offset.y() - screen.map_scale,
                ),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftRight => (
            Screen::Inspect(InspectScreen {
                map_offset: Point::new(
                    screen.map_offset.x() + screen.map_scale,
                    screen.map_offset.y(),
                ),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::Tab => (
            Screen::Inspect(InspectScreen {
                selected_screen: match screen.selected_screen {
                    SelectedScreen::Main => SelectedScreen::Summary,
                    SelectedScreen::Summary => SelectedScreen::Inspect,
                    SelectedScreen::Inspect => SelectedScreen::Main,
                },
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::Select => (
            match screen.selected_screen {
                SelectedScreen::Main => super::Screen::Main(MainScreen {
                    key: 0,
                    id: None,
                    err_msg: Some("Select Location".to_string()),
                }),
                SelectedScreen::Summary => super::Screen::Summary(SummaryScreen {
                    id: screen.id.clone(),
                    name: screen.name.clone(),
                    coord: screen.coord,
                    map_offset: Point::new(0.0, 0.0),
                    map_scale: 1.0,
                    selected_screen: SelectedScreen::Summary,
                    err_msg: None,
                }),
                SelectedScreen::Inspect => Screen::Inspect(screen.clone()),
            },
            None,
        ),
        _ => (super::Screen::Inspect(screen.clone()), None),
    };
    (next_screen, next_msg)
}
