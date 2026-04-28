use crate::model::ScreenType;

pub enum Update {
    Quit,
    GoToScreen(ScreenType),
    SetError(String),
}
