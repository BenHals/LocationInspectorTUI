use geo::Coord;
use itertools::Itertools;
use std::marker::PhantomData;

use ratatui::{
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Context, Line, Points},
        Block, Borders,
    },
};

use crate::{
    component::Component,
    domain::geometry::{Point, Polygon, Polyline, Projection},
    message::Message,
};

pub struct MapView<P: Projection + 'static> {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
    pub background: &'static [Polyline<P>],
    pub show_location: bool,
    _proj: PhantomData<P>,
}

pub struct MapViewCtx<'a, P: Projection> {
    pub center: &'a Point<P>,
    pub polygons: &'a [Polygon<P>],
    pub polylines: &'a [Polyline<P>],
    pub title: &'a str,
    pub selected_polygon: &'a Option<usize>,
}

impl<P: Projection + 'static> MapView<P> {
    pub fn new(
        background: &'static [Polyline<P>],
        scale: Option<f64>,
        show_location: bool,
    ) -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: scale.unwrap_or(1.0),
            background,
            show_location,
            _proj: PhantomData,
        }
    }

    /// Reset offsets to origin and set scale so the given polygons fit the viewport
    /// with a small margin. Polygons are assumed to be in this MapView's projection.
    pub fn fit_polygons(&mut self, polygons: &[Polygon<P>]) {
        let half_extent = polygons
            .iter()
            .flat_map(|p| p.inner.exterior().coords())
            .fold(0.0_f64, |acc, c| acc.max(c.x.abs()).max(c.y.abs()));

        const ASSUMED_HALF_CELLS: f64 = 80.0;
        const PADDING: f64 = 1.2;
        self.scale = half_extent * PADDING / (ASSUMED_HALF_CELLS * P::UNITS_PER_CELL_X);
        self.offset_x = 0.0;
        self.offset_y = 0.0;
    }
}

impl<P: Projection + 'static> Component for MapView<P> {
    type Ctx<'a> = MapViewCtx<'a, P>;

    fn update<'a>(
        &mut self,
        msg: &crate::message::Message,
        _ctx: Self::Ctx<'a>,
        _db: &crate::db::file_db::FileDB,
    ) -> (Vec<crate::update::Update>, Vec<Message>) {
        // pan: N screen-cells per press, projection-aware
        const PAN_CELLS: f64 = 5.0;
        let pan_x = PAN_CELLS * P::UNITS_PER_CELL_X * self.scale;
        let pan_y = PAN_CELLS * P::UNITS_PER_CELL_Y * self.scale;
        // zoom: symmetric multiplicative factor (in-then-out returns to start)
        const ZOOM_FACTOR: f64 = 1.0 / 0.9;

        match msg {
            Message::Char('w') | Message::Char('k') | Message::Up => self.offset_y += pan_y,
            Message::Char('s') | Message::Char('j') | Message::Down => self.offset_y -= pan_y,
            Message::Char('a') | Message::Char('h') | Message::Left => self.offset_x -= pan_x,
            Message::Char('d') | Message::Char('l') | Message::Right => self.offset_x += pan_x,
            Message::Char('+') => self.scale /= ZOOM_FACTOR,
            Message::Char('-') => self.scale *= ZOOM_FACTOR,
            _ => (),
        };
        (vec![], vec![])
    }

    fn render<'a>(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        ctx: Self::Ctx<'a>,
    ) {
        let cx = ctx.center.x + self.offset_x;
        let cy = ctx.center.y + self.offset_y;
        let half_x = (area.width as f64 / 2.0) * P::UNITS_PER_CELL_X * self.scale;
        let half_y = (area.height as f64 / 2.0) * P::UNITS_PER_CELL_Y * self.scale;
        let x_bounds = [cx - half_x, cx + half_x];
        let y_bounds = [cy - half_y, cy + half_y];

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(ctx.title))
            .marker(Marker::Braille)
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|c| {
                let mut selected_polys = vec![];
                for (i, poly) in ctx.polygons.iter().enumerate() {
                    let selected = ctx.selected_polygon == &Some(i);
                    if selected {
                        selected_polys.push(poly);
                        continue;
                    }
                    let color = Color::Red;
                    for (a, b) in poly.inner.exterior().coords().tuple_windows() {
                        if let Some([x1, y1, x2, y2]) = clip_line(a, b, x_bounds, y_bounds) {
                            c.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color,
                            });
                        }
                    }
                }

                for poly in selected_polys {
                    let color = Color::Green;
                    fill_polygon::<P>(c, poly, color, x_bounds, y_bounds, self.scale);
                    for (a, b) in poly.inner.exterior().coords().tuple_windows() {
                        if let Some([x1, y1, x2, y2]) = clip_line(a, b, x_bounds, y_bounds) {
                            c.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color,
                            });
                        }
                    }
                }
                for line in ctx.polylines {
                    for (a, b) in line.inner.coords().tuple_windows() {
                        if let Some([x1, y1, x2, y2]) = clip_line(a, b, x_bounds, y_bounds) {
                            c.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color: Color::Red,
                            });
                        }
                    }
                }
                for line in self.background {
                    for (a, b) in line.inner.coords().tuple_windows() {
                        if let Some([x1, y1, x2, y2]) = clip_line(a, b, x_bounds, y_bounds) {
                            c.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color: Color::Green,
                            });
                        }
                    }
                }

                if self.show_location {
                    c.print(
                        ctx.center.x,
                        ctx.center.y,
                        Span::styled("X", Style::new().red().bold()),
                    );
                }
            });

        frame.render_widget(canvas, area);
    }
}

/// Clip a line segment from `a` to `b` to the rectangle `[x_bounds] × [y_bounds]`
/// using the Liang-Barsky algorithm. Returns `None` if the segment is entirely outside.
fn clip_line(a: &Coord, b: &Coord, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> Option<[f64; 4]> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    let mut t0 = 0.0_f64;
    let mut t1 = 1.0_f64;

    let p_q = [
        (-dx, a.x - x_bounds[0]), // left
        (dx, x_bounds[1] - a.x),  // right
        (-dy, a.y - y_bounds[0]), // bottom
        (dy, y_bounds[1] - a.y),  // top
    ];

    for (p, q) in p_q {
        if p == 0.0 {
            // line is parallel to this boundary
            if q < 0.0 {
                return None; // and entirely outside
            }
            // else: no constraint from this boundary
        } else {
            let t = q / p;
            if p < 0.0 {
                // line is entering this slab
                if t > t1 {
                    return None;
                }
                if t > t0 {
                    t0 = t;
                }
            } else {
                // line is exiting this slab
                if t < t0 {
                    return None;
                }
                if t < t1 {
                    t1 = t;
                }
            }
        }
    }

    Some([a.x + t0 * dx, a.y + t0 * dy, a.x + t1 * dx, a.y + t1 * dy])
}

/// Scanline-fill a polygon onto a Canvas Context using the even-odd rule.
/// Steps in braille sub-cell increments (2 dots × 4 dots per terminal cell).
fn fill_polygon<P: Projection>(
    c: &mut Context,
    poly: &Polygon<P>,
    color: Color,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    scale: f64,
) {
    let coords: Vec<&Coord> = poly.inner.exterior().coords().collect();
    if coords.len() < 3 {
        return;
    }
    let (y_min, y_max) = coords
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), c| {
            (lo.min(c.y), hi.max(c.y))
        });
    let y_min = y_min.max(y_bounds[0]);
    let y_max = y_max.min(y_bounds[1]);

    let dx = P::UNITS_PER_CELL_X * scale / 2.0;
    let dy = P::UNITS_PER_CELL_Y * scale / 4.0;

    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut y = y_min;
    while y <= y_max {
        let mut xs: Vec<f64> = Vec::new();
        for (a, b) in coords.iter().tuple_windows() {
            if (a.y > y) != (b.y > y) {
                let t = (y - a.y) / (b.y - a.y);
                xs.push(a.x + t * (b.x - a.x));
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for pair in xs.chunks(2) {
            if let &[x_start, x_end] = pair {
                let x0 = x_start.max(x_bounds[0]);
                let x1 = x_end.min(x_bounds[1]);
                let mut x = x0;
                while x <= x1 {
                    points.push((x, y));
                    x += dx;
                }
            }
        }
        y += dy;
    }
    c.draw(&Points {
        coords: &points,
        color,
    });
}
