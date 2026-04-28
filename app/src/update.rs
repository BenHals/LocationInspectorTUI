use crate::{domain::location::Location, model::ScreenType};

pub enum Update {
    Quit,
    GoToScreen(ScreenType),
    SetError(String),
    SetLocation(Location),
}
