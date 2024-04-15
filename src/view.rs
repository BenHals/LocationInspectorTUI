use geo::Point;
use proj::Coord;
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};
use ratatui::{widgets::Paragraph, Frame};

use crate::model::{summary_screen::SummaryScreen, AppState, Screen};

pub fn view(state: &AppState, f: &mut Frame) {
    match &state.active_screen {
        Screen::Main(screen) => {
            let id = match &screen.id {
                None => "No matching id found".to_string(),
                Some(n) => n.clone(),
            };
            let err_str = match &screen.err_msg {
                Some(msg) => msg.clone(),
                None => "No errors".to_string(),
            };
            f.render_widget(
                Paragraph::new(format!("Key: {}, id: {}, err: {}", screen.key, id, err_str)),
                f.size(),
            )
        }
        Screen::Summary(SummaryScreen {
            id,
            name,
            coord,
            err_msg,
        }) => {
            let name_str = match name {
                Some(n) => n.clone(),
                None => "No Name Found".to_string(),
            };
            let err_str = match err_msg {
                Some(msg) => msg.clone(),
                None => "No errors".to_string(),
            };
            let coord_str = match coord {
                Some(n) => format!("<{}, {}>", n.x(), n.y()),
                None => "No location found".to_string(),
            };
            let screen_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(f.size());
            let right_pane = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(screen_layout[1]);
            let summary_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(right_pane[0]);

            f.render_widget(
                Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("World"))
                    .paint(|ctx| {
                        ctx.draw(&Map {
                            color: Color::Green,
                            resolution: MapResolution::High,
                        });
                        match coord {
                            None => (),
                            Some(c) => {
                                ctx.draw(&MapCoord {
                                    color: Color::Yellow,
                                    coord: c.clone(),
                                });
                            }
                        }
                    })
                    // .x_bounds([-180.0, 180.0])
                    // .y_bounds([-90.0, 90.0]),
                    .x_bounds([140.0, 180.0])
                    .y_bounds([-50.0, -20.0]),
                screen_layout[0],
            );
            f.render_widget(
                Paragraph::new(format!("Name: {}", name_str,)),
                summary_layout[0],
            );
            f.render_widget(Paragraph::new(format!("Id: {}", id,)), summary_layout[1]);
            f.render_widget(
                Paragraph::new(format!("Location: {}", coord_str,)),
                summary_layout[2],
            );

            f.render_widget(
                Paragraph::new(format!("Error: {}", err_str)).block(
                    Block::default()
                        .title("Error")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Red)),
                ),
                right_pane[1],
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapCoord {
    pub coord: Point,
    pub color: Color,
}

impl Shape for MapCoord {
    fn draw(&self, painter: &mut Painter) {
        if let Some((x, y)) = painter.get_point(self.coord.x(), self.coord.y()) {
            painter.paint(x, y, self.color);
        }
    }
}
