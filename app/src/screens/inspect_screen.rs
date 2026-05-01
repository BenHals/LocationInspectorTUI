use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, List, ListState},
    Frame,
};

use crate::{
    component::Component,
    components::map_view::{MapView, MapViewCtx},
    db::file_db::FileDB,
    domain::{
        geometry::{Local, Point},
        location::Location,
    },
    message::Message,
    model::InspectingLocationView,
    update::Update,
};

const ORIGIN: Point<Local> = Point::new(0.0, 0.0);

pub struct InspectScreenCtx<'a> {
    pub location: &'a Location,
    pub err: &'a Option<String>,
}
pub struct InspectScreen {
    pub map: MapView<Local>,
    selected_region: Option<usize>,
}

impl InspectScreen {
    pub fn new() -> Self {
        Self {
            map: MapView::new(&[], Some(0.1), false),
            selected_region: None,
        }
    }
}

impl Component for InspectScreen {
    type Ctx<'a> = InspectScreenCtx<'a>;
    fn update(
        &mut self,
        msg: &Message,
        ctx: InspectScreenCtx,
        db: &FileDB,
    ) -> (Vec<Update>, Vec<Message>) {
        match msg {
            Message::Esc => return (vec![Update::ClearLocation], vec![]),
            Message::Tab => {
                return (
                    vec![Update::SetInspectingLocationView(
                        InspectingLocationView::SummaryScreen,
                    )],
                    vec![Message::Activated],
                )
            }
            Message::Activated => {
                self.map.fit_polygons(&ctx.location.polygons);
                return (vec![], vec![]);
            }
            Message::Up => {
                if let Some(i) = self.selected_region {
                    if i > 0 {
                        self.selected_region = Some(i - 1);
                    } else {
                        self.selected_region = None;
                    }
                } else {
                    self.selected_region = Some(ctx.location.polygons.len() - 1);
                }
                return (vec![], vec![]);
            }
            Message::Down => {
                if let Some(i) = self.selected_region {
                    if i < ctx.location.polygons.len() - 1 {
                        self.selected_region = Some(i + 1);
                    } else {
                        self.selected_region = None;
                    }
                } else {
                    self.selected_region = Some(0);
                }
                return (vec![], vec![]);
            }
            _ => (),
        }
        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            polygons: &ctx.location.polygons,
            polylines: &[],
            title: &ctx.location.tag.name,
            selected_polygon: &self.selected_region,
        };
        self.map.update(msg, map_ctx, db)
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: InspectScreenCtx<'a>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Percentage(100)])
            .split(area);

        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            polygons: &ctx.location.polygons,
            polylines: &[],
            title: &ctx.location.tag.name,
            selected_polygon: &self.selected_region,
        };
        self.map.render(frame, layout[1], map_ctx);

        let controls_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        let layers_block = Block::bordered().title("Layers");
        frame.render_widget(&layers_block, controls_layout[0]);
        let mut layers_list_state = ListState::default();
        let layers_list = List::new(["Regions"])
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("▶ ");
        layers_list_state.select(Some(0));
        frame.render_stateful_widget(
            &layers_list,
            layers_block.inner(controls_layout[0]),
            &mut layers_list_state,
        );

        let regions_block = Block::bordered().title("Regions");
        frame.render_widget(&regions_block, controls_layout[1]);
        let mut region_list_state = ListState::default();

        let region_labels: Vec<String> = std::iter::once("<None>".to_string())
            .chain(
                ctx.location
                    .polygons
                    .iter()
                    .map(|p| p.metadata.name.clone()),
            )
            .collect();
        let region_list = List::new(region_labels.iter().map(|s| s.as_str()))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("▶ ");

        match self.selected_region {
            Some(i) => {
                region_list_state.select(Some(1 + i));
            }
            None => {
                region_list_state.select(Some(0));
            }
        }
        frame.render_stateful_widget(
            region_list,
            regions_block.inner(controls_layout[1]),
            &mut region_list_state,
        );
    }
}
