use ratatui::{style::Color, widgets::canvas::Shape};
pub struct LineCustom {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub color: Color,
    pub scale: f64,
}

impl Shape for LineCustom {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let line_len = ((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2)).sqrt();
        let n_points = 1 + 25 * (line_len / self.scale).round() as i64;
        for i in 0..n_points {
            let p: f64 = i as f64 / n_points as f64;
            let x = self.x1 + (self.x2 - self.x1) * p;
            let y = self.y1 + (self.y2 - self.y1) * p;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color)
            }
        }
    }
}
