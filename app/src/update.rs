use std::collections::HashMap;

use crate::{domain::location::Location, model::InspectingLocationView};

pub enum Update {
    Quit,
    SetError(String),
    SetLocation(Location),
    ClearLocation,
    SetInspectingLocationView(InspectingLocationView),
    TriggerLayerLoad {
        location_id: String,
        layer_id: String,
    },
    SetLayerData {
        location_id: String,
        layer_id: String,
        layer_data: HashMap<String, f64>,
    },
    SetLayerFailed {
        location_id: String,
        layer_id: String,
        err_msg: String,
    },
    SetActiveLayer {
        layer_id: String,
    },
}
