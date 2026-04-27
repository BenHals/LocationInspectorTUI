use geo::{Centroid, Point};

use crate::{db::DbConnection, event_handling::Message, model::Screen};

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
    pub fn clear_err(&mut self) {
        self.err_msg = None
    }
}

pub fn summary_screen_update(
    db: &dyn DbConnection,
    msg: &Message,
    screen: &SummaryScreen,
) -> (Screen, Option<Message>) {
    let (next_screen, next_msg) = match msg {
        Message::ZoomIn => (
            Screen::Summary(SummaryScreen {
                map_scale: screen.map_scale - 0.1,
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ZoomOut => (
            Screen::Summary(SummaryScreen {
                map_scale: screen.map_scale + 0.1,
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftUp => (
            Screen::Summary(SummaryScreen {
                map_offset: Point::new(screen.map_offset.x(), screen.map_offset.y() + 1.0),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftLeft => (
            Screen::Summary(SummaryScreen {
                map_offset: Point::new(screen.map_offset.x() - 1.0, screen.map_offset.y()),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftDown => (
            Screen::Summary(SummaryScreen {
                map_offset: Point::new(screen.map_offset.x(), screen.map_offset.y() - 1.0),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::ShiftRight => (
            Screen::Summary(SummaryScreen {
                map_offset: Point::new(screen.map_offset.x() + 1.0, screen.map_offset.y()),
                err_msg: None,
                ..screen.clone()
            }),
            None,
        ),
        Message::Tab => (
            Screen::Summary(SummaryScreen {
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
                SelectedScreen::Summary => Screen::Summary(screen.clone()),
                SelectedScreen::Inspect => {
                    let polys = db.get_polygons(&screen.id).unwrap();
                    let center = polys[0].centroid().unwrap();
                    Screen::Inspect(super::inspect_screen::InspectScreen {
                        id: screen.id.clone(),
                        name: screen.name.clone(),
                        coord: Some(center),
                        polygons: polys,
                        map_offset: Point::new(0.0, 0.0),
                        map_scale: 1.0,
                        selected_screen: SelectedScreen::Inspect,
                        err_msg: None,
                    })
                }
            },
            None,
        ),
        _ => (super::Screen::Summary(screen.clone()), None),
    };
    (next_screen, next_msg)
}
