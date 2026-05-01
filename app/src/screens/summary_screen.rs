use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{
    coastlines::coastlines,
    component::Component,
    components::map_view::{MapView, MapViewCtx},
    db::file_db::FileDB,
    domain::{geometry::WGS84, location::Location},
    message::Message,
    model::InspectingLocationView,
    update::Update,
};

pub struct SummaryScreenCtx<'a> {
    pub location: &'a Location,
    pub err: &'a Option<String>,
}
pub struct SummaryScreen {
    pub map: MapView<WGS84>,
}

impl SummaryScreen {
    pub fn new() -> Self {
        Self {
            map: MapView::new(coastlines(), Some(0.1), true),
        }
    }
}

impl Component for SummaryScreen {
    type Ctx<'a> = SummaryScreenCtx<'a>;
    fn update(
        &mut self,
        msg: &Message,
        ctx: SummaryScreenCtx,
        db: &FileDB,
    ) -> (Vec<Update>, Vec<Message>) {
        match msg {
            Message::Esc => return (vec![Update::ClearLocation], vec![]),
            Message::Tab => {
                return (
                    vec![Update::SetInspectingLocationView(
                        InspectingLocationView::InspectScreen,
                    )],
                    vec![Message::Activated],
                )
            }
            _ => (),
        }
        let map_ctx = MapViewCtx {
            center: &ctx.location.latlng,
            polygons: &[],
            polylines: &[],
            title: &ctx.location.tag.name,
        };
        self.map.update(msg, map_ctx, db)
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: SummaryScreenCtx<'a>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Min(10)])
            .split(area);

        let map_ctx = MapViewCtx {
            center: &ctx.location.latlng,
            polygons: &[],
            polylines: &[],
            title: "None",
        };
        self.map.render(frame, layout[1], map_ctx);
        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };
        let summary_string = format!("Location name {}", ctx.location.tag.name);
        let p = Paragraph::new(format!("Summary for: {}{}", summary_string, err_str));
        frame.render_widget(p, layout[0]);
    }
}
