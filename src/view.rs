use geo::Point;
use proj::Coord;
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{
        block::{Position, Title},
        canvas::{Line, *},
        *,
    },
};
use ratatui::{widgets::Paragraph, Frame};

use crate::model::{
    inspect_screen::InspectScreen, summary_screen::SummaryScreen, AppState, Screen,
};

use crate::model::screens::SelectedScreen;

fn get_status_line(selected_screen: &SelectedScreen, active_screen: SelectedScreen) -> Paragraph {
    let mut main_screen = "Main".bold();
    let mut summary_screen = "Summary".bold();
    let mut inspect_screen = "Inspect".bold();
    match selected_screen {
        SelectedScreen::Main => main_screen = main_screen.underlined(),
        SelectedScreen::Summary => summary_screen = summary_screen.underlined(),
        SelectedScreen::Inspect => inspect_screen = inspect_screen.underlined(),
    };
    match active_screen {
        SelectedScreen::Main => main_screen = main_screen.italic(),
        SelectedScreen::Summary => summary_screen = summary_screen.italic(),
        SelectedScreen::Inspect => inspect_screen = inspect_screen.italic(),
    };

    let screen_line = match selected_screen {
        SelectedScreen::Main => ratatui::text::Line::from(vec![
            main_screen,
            Span::raw(" "),
            summary_screen,
            Span::raw(" "),
            inspect_screen,
        ]),
        SelectedScreen::Summary => ratatui::text::Line::from(vec![
            main_screen,
            Span::raw(" "),
            summary_screen,
            Span::raw(" "),
            inspect_screen,
        ]),
        SelectedScreen::Inspect => ratatui::text::Line::from(vec![
            main_screen,
            Span::raw(" "),
            summary_screen,
            Span::raw(" "),
            inspect_screen,
        ]),
    };
    return Paragraph::new(screen_line)
        .block(
            Block::default()
                .title("Screen")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red)),
        )
        .alignment(Alignment::Center);
}

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
            map_offset,
            map_scale,
            selected_screen,
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
            let map_center = match coord {
                None => Point::new(map_offset.x(), map_offset.y()),
                Some(c) => Point::new(c.x() + map_offset.x(), c.y() + map_offset.y()),
            };
            let screen_layout_h = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
                .split(f.size());
            let screen_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(screen_layout_h[1]);
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
                get_status_line(selected_screen, SelectedScreen::Summary),
                screen_layout_h[0],
            );
            let instruction = Title::from(vec![
                "Left".into(),
                " <a> ".blue().bold(),
                "Right".into(),
                " <d> ".blue().bold(),
                "Up".into(),
                " <w> ".blue().bold(),
                "Down".into(),
                " <s> ".blue().bold(),
                "Zoom In".into(),
                " <i> ".blue().bold(),
                "Zoom Out".into(),
                " <o> ".blue().bold(),
            ])
            .alignment(Alignment::Center)
            .position(Position::Bottom);
            f.render_widget(
                Canvas::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Location")
                            .title(instruction),
                    )
                    .paint(|ctx| {
                        ctx.draw(&Map {
                            color: Color::Green,
                            resolution: MapResolution::High,
                        });
                        match coord {
                            None => (),
                            Some(c) => ctx.print(
                                c.x(),
                                c.y(),
                                Span::styled("X", Style::new().red().bold()),
                            ),
                        }
                    })
                    .x_bounds([
                        map_center.x()
                            - (screen_layout[0].as_size().width as f64 / 100.0) * 180.0 * map_scale,
                        map_center.x()
                            + (screen_layout[0].as_size().width as f64 / 100.0) * 180.0 * map_scale,
                    ])
                    .y_bounds([
                        map_center.y()
                            - (screen_layout[0].as_size().height as f64 / 30.0) * 90.0 * map_scale,
                        map_center.y()
                            + (screen_layout[0].as_size().height as f64 / 30.0) * 90.0 * map_scale,
                    ]),
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
        Screen::Inspect(InspectScreen {
            id,
            name,
            coord,
            map_offset,
            map_scale,
            selected_screen,
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
            let map_center = Point::new(map_offset.x(), map_offset.y());
            let screen_layout_h = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
                .split(f.size());
            let screen_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(screen_layout_h[1]);
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
                get_status_line(selected_screen, SelectedScreen::Inspect),
                screen_layout_h[0],
            );
            let instruction = Title::from(vec![
                "Left".into(),
                " <a> ".blue().bold(),
                "Right".into(),
                " <d> ".blue().bold(),
                "Up".into(),
                " <w> ".blue().bold(),
                "Down".into(),
                " <s> ".blue().bold(),
                "Zoom In".into(),
                " <i> ".blue().bold(),
                "Zoom Out".into(),
                " <o> ".blue().bold(),
            ])
            .alignment(Alignment::Center)
            .position(Position::Bottom);
            f.render_widget(
                Canvas::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Location")
                            .title(instruction),
                    )
                    .paint(|ctx| {
                        let x1: f64 = 0.0;
                        let y1: f64 = 0.0;
                        let x2: f64 = 10.0;
                        let y2: f64 = 10.0;
                        let line_len = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                        let n_points = (line_len / map_scale) as i64;
                        let mut c: Vec<(f64, f64)> = Vec::new();
                        for i in 0..n_points {
                            let p: f64 = i as f64 / n_points as f64;
                            let x = x1 + (x2 - x1) * p;
                            let y = y1 + (y2 - y1) * p;
                            c.push((x, y))
                        }
                        ctx.draw(&Line {
                            x1: 0.0,
                            y1: 0.0,
                            x2: 10.0,
                            y2: 10.0,
                            color: Color::LightCyan,
                        });
                        ctx.draw(&Points {
                            coords: &c,
                            color: Color::Green,
                        });
                    })
                    .x_bounds([
                        map_center.x() - (screen_layout[0].as_size().width as f64 * map_scale),
                        map_center.x() + (screen_layout[0].as_size().width as f64 * map_scale),
                    ])
                    .y_bounds([
                        map_center.y() - (screen_layout[0].as_size().height as f64 * map_scale),
                        map_center.y() + (screen_layout[0].as_size().height as f64 * map_scale),
                    ]),
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
