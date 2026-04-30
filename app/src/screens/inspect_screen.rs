use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{
    component::Component, components::map_view::{MapView, MapViewCtx}, db::file_db::FileDB, domain::{geometry::{Local, Point}, location::Location}, message::Message, model::InspectingLocationView, update::Update
};

const ORIGIN: Point<Local> = Point::new(0.0, 0.0);

pub struct InspectScreenCtx<'a> {
    pub location: &'a Location,
    pub err: &'a Option<String>,
}
pub struct InspectScreen {
    pub map: MapView<Local>,
}

impl InspectScreen {
    pub fn new() -> Self {
        Self {
            map: MapView::new(&[], Some(0.1)),
        }
    }
}

impl Component for InspectScreen {
    type Ctx<'a> = InspectScreenCtx<'a>;
    fn update(&mut self, msg: &Message, ctx: InspectScreenCtx, db: &FileDB) -> Vec<Update> {
        match msg {
            Message::Back => return vec![Update::ClearLocation],
            Message::Tab => return vec![Update::SetInspectingLocationView(InspectingLocationView::SummaryScreen)],
            _ => (),
        }
        let mut updates: Vec<Update> = vec![];
        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            polygons: &ctx.location.polygons,
            polylines: &[],
            title: &ctx.location.tag.name,
        };
        updates.extend(self.map.update(msg, map_ctx, db));
        updates
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: InspectScreenCtx<'a>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Min(10)])
            .split(area);

        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            polygons: &ctx.location.polygons,
            polylines: &[],
            title: &ctx.location.tag.name,
        };
        self.map.render(frame, layout[1], map_ctx);
        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let summary_string = format!("Location name {}", ctx.location.tag.name);
        let p = Paragraph::new(format!("Inspecting: {}{}", summary_string, err_str));
        frame.render_widget(p, layout[0]);
    }
}
