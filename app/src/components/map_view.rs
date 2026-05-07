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

/// 256-entry lookup mapping an octant bitmask to its rendering character.
/// Bit numbering is row-major over a 2x4 sub-cell grid:
///   bit 0 (mask   1) = (col 0, row 0) top-left
///   bit 1 (mask   2) = (col 1, row 0) top-right
///   bit 2 (mask   4) = (col 0, row 1)
///   bit 3 (mask   8) = (col 1, row 1)
///   bit 4 (mask  16) = (col 0, row 2)
///   bit 5 (mask  32) = (col 1, row 2)
///   bit 6 (mask  64) = (col 0, row 3) bottom-left
///   bit 7 (mask 128) = (col 1, row 3) bottom-right
///
/// Most entries are from the Unicode 16 "Block Octants" block (U+1CD00..U+1CDE5);
/// remaining entries fall back to legacy quadrants (U+2596..U+259F), half/quarter
/// blocks (U+2580..U+2590), and a few characters from the Symbols for Legacy
/// Computing Supplement (U+1CEA0..) and Symbols for Legacy Computing (U+1FB80..).
///
/// **Terminal caveat**: requires Unicode 16-aware fonts (Sept 2024). Modern
/// terminals (kitty, recent iTerm2, WezTerm, Ghostty) render the full table;
/// older terminals will show tofu for the U+1CDxx and U+1CExx ranges.
const OCTANT_TABLE: [&str; 256] = [
    "\u{0020}", "\u{1CEA8}", "\u{1CEAB}", "\u{1FB82}", "\u{1CD00}", "\u{2598}", "\u{1CD01}", "\u{1CD02}",
    "\u{1CD03}", "\u{1CD04}", "\u{259D}", "\u{1CD05}", "\u{1CD06}", "\u{1CD07}", "\u{1CD08}", "\u{2580}",
    "\u{1CD09}", "\u{1CD0A}", "\u{1CD0B}", "\u{1CD0C}", "\u{1FBE6}", "\u{1CD0D}", "\u{1CD0E}", "\u{1CD0F}",
    "\u{1CD10}", "\u{1CD11}", "\u{1CD12}", "\u{1CD13}", "\u{1CD14}", "\u{1CD15}", "\u{1CD16}", "\u{1CD17}",
    "\u{1CD18}", "\u{1CD19}", "\u{1CD1A}", "\u{1CD1B}", "\u{1CD1C}", "\u{1CD1D}", "\u{1CD1E}", "\u{1CD1F}",
    "\u{1FBE7}", "\u{1CD20}", "\u{1CD21}", "\u{1CD22}", "\u{1CD23}", "\u{1CD24}", "\u{1CD25}", "\u{1CD26}",
    "\u{1CD27}", "\u{1CD28}", "\u{1CD29}", "\u{1CD2A}", "\u{1CD2B}", "\u{1CD2C}", "\u{1CD2D}", "\u{1CD2E}",
    "\u{1CD2F}", "\u{1CD30}", "\u{1CD31}", "\u{1CD32}", "\u{1CD33}", "\u{1CD34}", "\u{1CD35}", "\u{1FB85}",
    "\u{1CEA3}", "\u{1CD36}", "\u{1CD37}", "\u{1CD38}", "\u{1CD39}", "\u{1CD3A}", "\u{1CD3B}", "\u{1CD3C}",
    "\u{1CD3D}", "\u{1CD3E}", "\u{1CD3F}", "\u{1CD40}", "\u{1CD41}", "\u{1CD42}", "\u{1CD43}", "\u{1CD44}",
    "\u{2596}", "\u{1CD45}", "\u{1CD46}", "\u{1CD47}", "\u{1CD48}", "\u{258C}", "\u{1CD49}", "\u{1CD4A}",
    "\u{1CD4B}", "\u{1CD4C}", "\u{259E}", "\u{1CD4D}", "\u{1CD4E}", "\u{1CD4F}", "\u{1CD50}", "\u{259B}",
    "\u{1CD51}", "\u{1CD52}", "\u{1CD53}", "\u{1CD54}", "\u{1CD55}", "\u{1CD56}", "\u{1CD57}", "\u{1CD58}",
    "\u{1CD59}", "\u{1CD5A}", "\u{1CD5B}", "\u{1CD5C}", "\u{1CD5D}", "\u{1CD5E}", "\u{1CD5F}", "\u{1CD60}",
    "\u{1CD61}", "\u{1CD62}", "\u{1CD63}", "\u{1CD64}", "\u{1CD65}", "\u{1CD66}", "\u{1CD67}", "\u{1CD68}",
    "\u{1CD69}", "\u{1CD6A}", "\u{1CD6B}", "\u{1CD6C}", "\u{1CD6D}", "\u{1CD6E}", "\u{1CD6F}", "\u{1CD70}",
    "\u{1CEA0}", "\u{1CD71}", "\u{1CD72}", "\u{1CD73}", "\u{1CD74}", "\u{1CD75}", "\u{1CD76}", "\u{1CD77}",
    "\u{1CD78}", "\u{1CD79}", "\u{1CD7A}", "\u{1CD7B}", "\u{1CD7C}", "\u{1CD7D}", "\u{1CD7E}", "\u{1CD7F}",
    "\u{1CD80}", "\u{1CD81}", "\u{1CD82}", "\u{1CD83}", "\u{1CD84}", "\u{1CD85}", "\u{1CD86}", "\u{1CD87}",
    "\u{1CD88}", "\u{1CD89}", "\u{1CD8A}", "\u{1CD8B}", "\u{1CD8C}", "\u{1CD8D}", "\u{1CD8E}", "\u{1CD8F}",
    "\u{2597}", "\u{1CD90}", "\u{1CD91}", "\u{1CD92}", "\u{1CD93}", "\u{259A}", "\u{1CD94}", "\u{1CD95}",
    "\u{1CD96}", "\u{1CD97}", "\u{2590}", "\u{1CD98}", "\u{1CD99}", "\u{1CD9A}", "\u{1CD9B}", "\u{259C}",
    "\u{1CD9C}", "\u{1CD9D}", "\u{1CD9E}", "\u{1CD9F}", "\u{1CDA0}", "\u{1CDA1}", "\u{1CDA2}", "\u{1CDA3}",
    "\u{1CDA4}", "\u{1CDA5}", "\u{1CDA6}", "\u{1CDA7}", "\u{1CDA8}", "\u{1CDA9}", "\u{1CDAA}", "\u{1CDAB}",
    "\u{2582}", "\u{1CDAC}", "\u{1CDAD}", "\u{1CDAE}", "\u{1CDAF}", "\u{1CDB0}", "\u{1CDB1}", "\u{1CDB2}",
    "\u{1CDB3}", "\u{1CDB4}", "\u{1CDB5}", "\u{1CDB6}", "\u{1CDB7}", "\u{1CDB8}", "\u{1CDB9}", "\u{1CDBA}",
    "\u{1CDBB}", "\u{1CDBC}", "\u{1CDBD}", "\u{1CDBE}", "\u{1CDBF}", "\u{1CDC0}", "\u{1CDC1}", "\u{1CDC2}",
    "\u{1CDC3}", "\u{1CDC4}", "\u{1CDC5}", "\u{1CDC6}", "\u{1CDC7}", "\u{1CDC8}", "\u{1CDC9}", "\u{1CDCA}",
    "\u{1CDCB}", "\u{1CDCC}", "\u{1CDCD}", "\u{1CDCE}", "\u{1CDCF}", "\u{1CDD0}", "\u{1CDD1}", "\u{1CDD2}",
    "\u{1CDD3}", "\u{1CDD4}", "\u{1CDD5}", "\u{1CDD6}", "\u{1CDD7}", "\u{1CDD8}", "\u{1CDD9}", "\u{1CDDA}",
    "\u{2584}", "\u{1CDDB}", "\u{1CDDC}", "\u{1CDDD}", "\u{1CDDE}", "\u{2599}", "\u{1CDDF}", "\u{1CDE0}",
    "\u{1CDE1}", "\u{1CDE2}", "\u{259F}", "\u{1CDE3}", "\u{2586}", "\u{1CDE4}", "\u{1CDE5}", "\u{2588}",
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
                // Accumulate octant bits per cell so multiple points in the same
                // cell combine into a single composite glyph.
                let mut cells: HashMap<(i64, i64), u8> = HashMap::new();
                for pt in ctx.points {
                    if let Some((key, bit)) =
                        octant_bit_for_point(pt.x, pt.y, x_bounds, y_bounds, cell_w, cell_h)
                    {
                        *cells.entry(key).or_insert(0) |= bit;
                    }
                }
                for ((cx, cy), mask) in &cells {
                    if *mask == 0 {
                        continue;
                    }
                    let world_x = x_bounds[0] + (*cx as f64 + 0.5) * cell_w;
                    let world_y = y_bounds[1] - (*cy as f64 + 0.5) * cell_h;
                    let glyph = OCTANT_TABLE[*mask as usize];
                    c.print(
                        world_x,
                        world_y,
                        Span::styled(
                            glyph,
                            Style::new()
                                .fg(Color::Rgb(255, 255, 255))
                                .add_modifier(Modifier::BOLD),
                        ),
                    );
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

/// Locate a continuous (px, py) point at its precise sub-cell position. Returns
/// the cell index plus an octant bitmask with a single bit set for that point's
/// sub-cell. Returns `None` if the point is outside the canvas bounds. Caller
/// OR-combines bits from multiple points falling in the same cell, then looks
/// up `OCTANT_TABLE[mask as usize]` for the composite glyph.
fn octant_bit_for_point(
    px: f64,
    py: f64,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    cell_w: f64,
    cell_h: f64,
) -> Option<((i64, i64), u8)> {
    if px < x_bounds[0] || px > x_bounds[1] || py < y_bounds[0] || py > y_bounds[1] {
        return None;
    }
    let cx = ((px - x_bounds[0]) / cell_w).floor() as i64;
    let cy = ((y_bounds[1] - py) / cell_h).floor() as i64;
    let frac_col = ((px - x_bounds[0]) / cell_w).rem_euclid(1.0);
    let frac_row = ((y_bounds[1] - py) / cell_h).rem_euclid(1.0);
    let col_half = if frac_col < 0.5 { 0 } else { 1 };
    let row_quarter = ((frac_row * 4.0).floor() as usize).min(3);
    let bit = (row_quarter * 2 + col_half) as u8;
    Some(((cx, cy), 1u8 << bit))
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
