use std::collections::HashMap;

use crate::{config::Config, domain::location::Location, update::Update};

pub struct InspectingState {
    pub location: Location,
    pub layers: HashMap<String, LayerState>,
    pub active_layer: String,
    pub view: InspectingLocationView,
}

pub enum InteractionMode {
    BrowsingLocation,
    InspectingLocation { state: InspectingState },
}
pub enum InspectingLocationView {
    SummaryScreen,
    InspectScreen,
}

#[derive(PartialEq)]
pub enum ApplicationStatus {
    Running,
    Done,
}

pub enum LayerState {
    Loading,
    Loaded(HashMap<String, f64>),
    Failed(String),
}

pub struct Model {
    pub application_status: ApplicationStatus,
    pub interaction_mode: InteractionMode,
    pub err: Option<String>,
    pub config: Config,
}

impl Model {
    pub fn new(config: Config) -> Self {
        Self {
            application_status: ApplicationStatus::Running,
            interaction_mode: InteractionMode::BrowsingLocation,
            err: None,
            config,
        }
    }

    pub fn apply(&mut self, update: Update) {
        match update {
            Update::Quit => self.application_status = ApplicationStatus::Done,
            Update::SetError(err) => self.err = Some(err),
            Update::ClearLocation => self.interaction_mode = InteractionMode::BrowsingLocation,
            Update::SetLocation(location) => {
                self.interaction_mode = InteractionMode::InspectingLocation {
                    state: InspectingState {
                        location,
                        layers: HashMap::new(),
                        active_layer: "boundaries".to_string(),
                        view: InspectingLocationView::SummaryScreen,
                    },
                }
            }
            Update::SetInspectingLocationView(new_view) => {
                if let InteractionMode::InspectingLocation { state } = &mut self.interaction_mode {
                    state.view = new_view;
                }
            }
            Update::TriggerLayerLoad {
                location_id,
                layer_id,
            } => {
                if let InteractionMode::InspectingLocation { state } = &mut self.interaction_mode {
                    if state.location.tag.id == location_id {
                        state.layers.insert(layer_id, LayerState::Loading);
                    }
                }
            }
            Update::SetLayerData {
                location_id,
                layer_id,
                layer_data,
            } => {
                if let InteractionMode::InspectingLocation { state } = &mut self.interaction_mode {
                    if state.location.tag.id == location_id {
                        state
                            .layers
                            .insert(layer_id, LayerState::Loaded(layer_data));
                    }
                }
            }
            Update::SetLayerFailed {
                location_id,
                layer_id,
                err_msg,
            } => {
                if let InteractionMode::InspectingLocation { state } = &mut self.interaction_mode {
                    if state.location.tag.id == location_id {
                        state.layers.insert(layer_id, LayerState::Failed(err_msg));
                    }
                }
            }
            Update::SetActiveLayer { layer_id } => {
                if let InteractionMode::InspectingLocation { state } = &mut self.interaction_mode {
                    state.active_layer = layer_id;
                }
            }
        }
    }
}
