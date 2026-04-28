use crate::{domain::location::Location, update::Update};

pub enum InteractionMode {
    BrowsingLocation,
    InspectingLocation {
        location: Location,
        view: InspectingLocationView,
    },
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

pub struct Model {
    pub application_status: ApplicationStatus,
    pub interaction_mode: InteractionMode,
    pub err: Option<String>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            application_status: ApplicationStatus::Running,
            interaction_mode: InteractionMode::BrowsingLocation,
            err: None,
        }
    }

    pub fn apply(&mut self, update: Update) {
        match update {
            Update::Quit => self.application_status = ApplicationStatus::Done,
            Update::SetError(err) => self.err = Some(err),
            Update::ClearLocation() => self.interaction_mode = InteractionMode::BrowsingLocation,
            Update::SetLocation(location) => {
                self.interaction_mode = InteractionMode::InspectingLocation {
                    location,
                    view: InspectingLocationView::SummaryScreen,
                }
            }
            Update::SetInspectingLocationView(new_view) => {
                if let InteractionMode::InspectingLocation { view, .. } = &mut self.interaction_mode
                {
                    *view = new_view;
                }
            }
        }
    }
}
