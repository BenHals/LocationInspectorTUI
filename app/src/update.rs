use crate::appstate::ScreenState;

pub enum Update {
    Quit,
    GoToScreen(ScreenState),
    SetError(String),
}
