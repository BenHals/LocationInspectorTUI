use crate::{domain::location::Location, model::InspectingLocationView};

pub enum Update {
    Quit,
    SetError(String),
    SetLocation(Location),
    ClearLocation(),
    SetInspectingLocationView(InspectingLocationView),
}
