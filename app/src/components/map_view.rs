use itertools::Itertools;
use ratatui::style::Stylize;
use std::marker::PhantomData;

use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line, Map, MapResolution},
        Block, Borders,
    },
};

use crate::{
    component::Component,
    domain::geometry::{Point, Polygon, Projection},
    message::Message,
};

pub struct MapView<P: Projection> {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
    _proj: PhantomData<P>,
}

pub struct MapViewCtx<'a, P: Projection> {
    pub center: &'a Point<P>,
    pub polygons: &'a [Polygon<P>],
    pub title: &'a str,
    pub draw_world_map: bool,
}

impl<P: Projection> MapView<P> {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            _proj: PhantomData,
        }
    }
}

impl<P: Projection + 'static> Component for MapView<P> {
    type Ctx<'a> = MapViewCtx<'a, P>;

    fn update<'a>(
        &mut self,
        msg: &crate::message::Message,
        ctx: Self::Ctx<'a>,
        db: &crate::db::file_db::FileDB,
    ) -> Vec<crate::update::Update> {
        match msg {
            Message::ShiftUp => self.offset_y += self.scale,
            Message::ShiftDown => self.offset_y -= self.scale,
            Message::ShiftLeft => self.offset_x -= self.scale,
            Message::ShiftRight => self.offset_x += self.scale,
            Message::ZoomIn => self.scale *= 0.9,
            Message::ZoomOut => self.scale *= 1.1,
            _ => (),
        };
        vec![]
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
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|c| {
                if ctx.draw_world_map {
                    c.draw(&Map {
                        color: Color::Green,
                        resolution: MapResolution::High,
                    });
                }

                for poly in ctx.polygons {
                    for (a, b) in poly.inner.exterior().coords().tuple_windows() {
                        c.draw(&Line {
                            x1: a.x,
                            y1: a.y,
                            x2: b.x,
                            y2: b.y,
                            color: Color::Red,
                        });
                    }
                }

                c.print(
                    ctx.center.x,
                    ctx.center.y,
                    Span::styled("X", Style::new().red().bold()),
                );
            });

        frame.render_widget(canvas, area);
    }
}
