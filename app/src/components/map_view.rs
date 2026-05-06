use geo::Coord;
use itertools::Itertools;
use std::{collections::HashMap, marker::PhantomData};

use ratatui::{
    style::{Color, Modifier, Style},
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

/// Single-octant block characters from Unicode 16 "Block Octants" (U+1CD00 block).
/// Indexed as `[col_half][row_quarter]` where col_half is 0 (left) or 1 (right)
/// and row_quarter is 0..=3 from the top of the cell.
///
/// Bitmask layout assumed: bit i is set when sub-cell i is filled, with sub-cells
/// numbered top→bottom, left→right (0=top-left, 1=top-right, 2=upper-mid-left ...
/// 7=bottom-right). Codepoint = U+1CD00 + (mask - 1).
///
/// **Terminal support caveat**: octants are Unicode 16 (Sept 2024). Modern terminals
/// (kitty, recent iTerm2, WezTerm, Ghostty) render them; older terminals or fonts
/// without coverage will show tofu. If you see boxes, fall back to quadrants
/// (▘▝▖▗ at U+2598/U+259D/U+2596/U+2597) for half the resolution but universal support.
const OCTANT_GLYPHS: [[&str; 4]; 2] = [
    ["\u{1CD00}", "\u{1CD03}", "\u{1CD0F}", "\u{1CD3F}"],
    ["\u{1CD01}", "\u{1CD07}", "\u{1CD1F}", "\u{1CD7F}"],
];

#[derive(Clone)]
pub struct ColorMap {
    stops: Vec<(f64, (u8, u8, u8))>,
}

impl ColorMap {
    pub fn sample(&self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        if t <= self.stops[0].0 {
            let (_, c) = self.stops[0];
            return Color::Rgb(c.0, c.1, c.2);
        }
        for w in self.stops.windows(2) {
            let (t0, c0) = w[0];
            let (t1, c1) = w[1];
            if t <= t1 {
                let f = (t - t0) / (t1 - t0);
                let lerp = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * f) as u8;
                return Color::Rgb(lerp(c0.0, c1.0), lerp(c0.1, c1.1), lerp(c0.2, c1.2));
            }
        }
        let (_, c) = *self.stops.last().unwrap();
        Color::Rgb(c.0, c.1, c.2)
    }

    pub fn magma() -> Self {
        Self {
            stops: vec![
                (0.00, (0, 0, 4)),
                (0.13, (28, 16, 68)),
                (0.25, (59, 15, 112)),
                (0.38, (98, 25, 128)),
                (0.50, (139, 41, 129)),
                (0.63, (181, 54, 122)),
                (0.75, (222, 73, 104)),
                (0.88, (243, 129, 119)),
                (0.94, (251, 176, 116)),
                (1.00, (252, 253, 191)),
            ],
        }
    }
}

pub struct MapView<P: Projection + 'static> {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
    pub background: &'static [Polyline<P>],
    pub show_location: bool,
    pub center_on: bool,
    _proj: PhantomData<P>,
}

pub struct FillByValue {
    pub map: ColorMap,
    pub values: HashMap<String, f64>,
}

pub struct MapViewCtx<'a, P: Projection> {
    pub center: &'a Point<P>,
    pub boundaries: &'a [Polygon<P>],
    pub regions: &'a [Polygon<P>],
    pub polylines: &'a [Polyline<P>],
    pub points: &'a [Point<P>],
    pub title: &'a str,
    pub selected_region: &'a Option<usize>,
    pub fill_info: Option<FillByValue>,
}

impl<P: Projection + 'static> MapView<P> {
    pub fn new(
        background: &'static [Polyline<P>],
        scale: Option<f64>,
        show_location: bool,
        center_on: bool,
    ) -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: scale.unwrap_or(1.0),
            background,
            show_location,
            center_on,
            _proj: PhantomData,
        }
    }

    /// Reset offsets to origin and set scale so the given polygons fit the viewport
    /// with a small margin. Polygons are assumed to be in this MapView's projection.
    pub fn fit_polygons(&mut self, boundaries: &[Polygon<P>], regions: &[Polygon<P>]) {
        let half_extent = boundaries
            .iter()
            .chain(regions.iter())
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
        let center_x = if self.center_on { ctx.center.x } else { 0.0 };
        let center_y = if self.center_on { ctx.center.y } else { 0.0 };
        let cx = center_x + self.offset_x;
        let cy = center_y + self.offset_y;
        let half_x = (area.width as f64 / 2.0) * P::UNITS_PER_CELL_X * self.scale;
        let half_y = (area.height as f64 / 2.0) * P::UNITS_PER_CELL_Y * self.scale;
        let x_bounds = [cx - half_x, cx + half_x];
        let y_bounds = [cy - half_y, cy + half_y];

        let max_fill_value = match &ctx.fill_info {
            Some(fill_info) => fill_info.values.values().copied().reduce(f64::max),
            None => None,
        };
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(ctx.title))
            .marker(Marker::Braille)
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|c| {
                for poly in ctx.boundaries {
                    for (a, b) in poly.inner.exterior().coords().tuple_windows() {
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

                let mut selected_polys = vec![];
                for (i, poly) in ctx.regions.iter().enumerate() {
                    let selected = ctx.selected_region == &Some(i);
                    if selected {
                        selected_polys.push(poly);
                        continue;
                    }
                    let fill_color = ctx.fill_info.as_ref().and_then(|fi| {
                        fi.values
                            .get(&poly.metadata.id)
                            .map(|v| fi.map.sample(v / max_fill_value.unwrap_or(1.0)))
                    });
                    match fill_color {
                        Some(color) => {
                            fill_polygon::<P>(c, poly, color, x_bounds, y_bounds, self.scale)
                        }
                        None => {
                            for (a, b) in poly.inner.exterior().coords().tuple_windows() {
                                if let Some([x1, y1, x2, y2]) = clip_line(a, b, x_bounds, y_bounds)
                                {
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

                let cell_w = (x_bounds[1] - x_bounds[0]) / area.width as f64;
                let cell_h = (y_bounds[1] - y_bounds[0]) / area.height as f64;
                for pt in ctx.points {
                    if let Some(glyph) =
                        octant_glyph_for_point(pt.x, pt.y, x_bounds, y_bounds, cell_w, cell_h)
                    {
                        c.print(
                            pt.x,
                            pt.y,
                            Span::styled(
                                glyph,
                                Style::new()
                                    .fg(Color::Rgb(255, 255, 255))
                                    .add_modifier(Modifier::BOLD),
                            ),
                        );
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

/// Pick the octant block character that visually anchors a continuous (px, py)
/// point at its precise sub-cell position. Returns `None` if the point is
/// outside the canvas bounds. Caller is responsible for styling.
fn octant_glyph_for_point(
    px: f64,
    py: f64,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    cell_w: f64,
    cell_h: f64,
) -> Option<&'static str> {
    if px < x_bounds[0] || px > x_bounds[1] || py < y_bounds[0] || py > y_bounds[1] {
        return None;
    }
    let frac_col = ((px - x_bounds[0]) / cell_w).rem_euclid(1.0);
    let frac_row = ((y_bounds[1] - py) / cell_h).rem_euclid(1.0);
    let col_half = if frac_col < 0.5 { 0 } else { 1 };
    let row_quarter = ((frac_row * 4.0).floor() as usize).min(3);
    Some(OCTANT_GLYPHS[col_half][row_quarter])
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
