use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, List, ListState},
    Frame,
};

use crate::{
    component::Component,
    components::map_view::{ColorMap, FillByValue, MapView, MapViewCtx},
    config::LayerConfig,
    db::file_db::FileDB,
    domain::{
        geometry::{Local, Point},
        location::Location,
    },
    message::Message,
    model::{InspectingLocationView, LayerState},
    update::Update,
};

const ORIGIN: Point<Local> = Point::new(0.0, 0.0);
const DEFAULT_LAYER_ID: &str = "boundaries";

pub struct InspectScreenCtx<'a> {
    pub location: &'a Location,
    pub layers: &'a HashMap<String, LayerState>,
    pub active_layer: &'a String,
    pub configured_layers: &'a [LayerConfig],
    pub err: &'a Option<String>,
}
pub struct InspectScreen {
    pub map: MapView<Local>,
    selected_region: Option<usize>,
}

impl InspectScreen {
    pub fn new() -> Self {
        Self {
            map: MapView::new(&[], Some(0.1), false, true),
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
                self.map
                    .fit_polygons(&ctx.location.boundaries, &ctx.location.regions);
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
                    self.selected_region = Some(ctx.location.regions.len() - 1);
                }
                return (vec![], vec![]);
            }
            Message::Down => {
                if let Some(i) = self.selected_region {
                    if i < ctx.location.regions.len() - 1 {
                        self.selected_region = Some(i + 1);
                    } else {
                        self.selected_region = None;
                    }
                } else {
                    self.selected_region = Some(0);
                }
                return (vec![], vec![]);
            }
            Message::Left => {
                let layer_ids: Vec<String> = std::iter::once(DEFAULT_LAYER_ID.to_string())
                    .chain(ctx.configured_layers.iter().map(|s| s.id.to_string()))
                    .collect();

                let layer_idx = layer_ids
                    .iter()
                    .position(|id| id == ctx.active_layer)
                    .unwrap_or_default();
                let mut next_layer_idx = layer_idx as i64 - 1;
                if next_layer_idx < 0 {
                    next_layer_idx = (layer_ids.len() - 1) as i64;
                }

                return (
                    vec![Update::SetActiveLayer {
                        layer_id: layer_ids[next_layer_idx as usize].clone(),
                    }],
                    vec![],
                );
            }
            Message::Right => {
                let layer_ids: Vec<String> = std::iter::once(DEFAULT_LAYER_ID.to_string())
                    .chain(ctx.configured_layers.iter().map(|s| s.id.to_string()))
                    .collect();

                let layer_idx = layer_ids
                    .iter()
                    .position(|id| id == ctx.active_layer)
                    .unwrap_or_default();
                let mut next_layer_idx = layer_idx as i64 + 1;
                if next_layer_idx >= layer_ids.len() as i64 {
                    next_layer_idx = 0;
                }

                return (
                    vec![Update::SetActiveLayer {
                        layer_id: layer_ids[next_layer_idx as usize].clone(),
                    }],
                    vec![],
                );
            }
            _ => (),
        }
        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            boundaries: &ctx.location.boundaries,
            regions: &ctx.location.regions,
            polylines: &[],
            points: &[],
            title: &ctx.location.tag.name,
            selected_region: &self.selected_region,
            fill_info: None,
        };
        self.map.update(msg, map_ctx, db)
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: InspectScreenCtx<'a>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Percentage(100)])
            .split(area);

        let layer_fills = match ctx.layers.get(ctx.active_layer) {
            Some(LayerState::Loading) => None,
            Some(LayerState::Failed(_)) => None,
            Some(LayerState::Loaded(values)) => Some(FillByValue {
                map: ColorMap::magma(),
                values: values.clone(),
            }),
            None => None,
        };
        let map_ctx = MapViewCtx {
            center: &ORIGIN,
            boundaries: &ctx.location.boundaries,
            regions: &ctx.location.regions,
            polylines: &[],
            points: &[],
            title: &ctx.location.tag.name,
            selected_region: &self.selected_region,
            fill_info: layer_fills,
        };
        self.map.render(frame, layout[1], map_ctx);

        let controls_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        let layers_block = Block::bordered().title("Layers");
        frame.render_widget(&layers_block, controls_layout[0]);
        let mut layers_list_state = ListState::default();
        let layer_labels: Vec<String> = std::iter::once("Boundaries".to_string())
            .chain(ctx.configured_layers.iter().map(|s| {
                let layer_status = match ctx.layers.get(&s.id) {
                    Some(LayerState::Loading) => "Loading".to_string(),
                    Some(LayerState::Loaded(_)) => "Loaded!".to_string(),
                    Some(LayerState::Failed(err)) => format!("Failed - {}", err),
                    None => "Not Triggered".to_string(),
                };
                format!("{} - {}", s.name, layer_status)
            }))
            .collect();

        let layer_ids: Vec<String> = std::iter::once(DEFAULT_LAYER_ID.to_string())
            .chain(ctx.configured_layers.iter().map(|s| s.id.to_string()))
            .collect();
        let layers_list = List::new(layer_labels)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("▶ ");

        let layer_idx = layer_ids.iter().position(|id| id == ctx.active_layer);
        layers_list_state.select(layer_idx);
        frame.render_stateful_widget(
            &layers_list,
            layers_block.inner(controls_layout[0]),
            &mut layers_list_state,
        );

        let regions_block = Block::bordered().title("Regions");
        frame.render_widget(&regions_block, controls_layout[1]);
        let mut region_list_state = ListState::default();

        let region_labels: Vec<String> = std::iter::once("<None>".to_string())
            .chain(ctx.location.regions.iter().map(|p| {
                let region_layer_val = match ctx.layers.get(ctx.active_layer) {
                    Some(LayerState::Loading) => "Loading".to_string(),
                    Some(LayerState::Failed(_)) => "".to_string(),
                    Some(LayerState::Loaded(values)) => match values.get(&p.metadata.id) {
                        Some(v) => format!("{:.2}", v),
                        None => "-".to_string(),
                    },
                    None => "".to_string(),
                };
                format!("{} - {}", p.metadata.name.clone(), region_layer_val)
            }))
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
