use crate::update::Update;

pub enum ScreenType {
    LocationSelect,
    Summary,
    Inspect,
}

#[derive(PartialEq)]
pub enum ApplicationStatus {
    Running,
    Done,
}

pub struct Model {
    pub application_status: ApplicationStatus,
    pub screen: ScreenType,
    pub err: Option<String>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            application_status: ApplicationStatus::Running,
            screen: ScreenType::LocationSelect,
            err: None,
        }
    }

    pub fn apply(&mut self, update: Update) {
        match update {
            Update::Quit => self.application_status = ApplicationStatus::Done,
            Update::GoToScreen(screen) => self.screen = screen,
            Update::SetError(err) => self.err = Some(err),
        }
    }
}
