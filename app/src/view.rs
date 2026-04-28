use ratatui::{layout::Rect, Frame};

use crate::{
    component::Component,
    db::file_db::FileDB,
    message::Message,
    model::{InspectingLocationView, InteractionMode, Model},
    screens::{
        location_select_screen::LocationSelectScreen,
        summary_screen::{SummaryScreen, SummaryScreenCtx},
    },
    update::Update,
};

pub struct View {
    pub location_select_screen: LocationSelectScreen,
    pub summary_screen: SummaryScreen,
}

impl View {
    pub fn new(db: &FileDB) -> Self {
        Self {
            location_select_screen: LocationSelectScreen::new(db),
            summary_screen: SummaryScreen::new(),
        }
    }
}

impl Component for View {
    type Ctx<'a> = &'a Model;
    fn update(&mut self, msg: &Message, ctx: &Model, db: &FileDB) -> Vec<Update> {
        match msg {
            Message::Quit => return vec![Update::Quit],
            _ => {}
        }
        match &ctx.interaction_mode {
            InteractionMode::BrowsingLocation => self.location_select_screen.update(msg, ctx, db),
            InteractionMode::InspectingLocation { view, location } => match view {
                InspectingLocationView::SummaryScreen => {
                    let ctx = SummaryScreenCtx {
                        location,
                        err: &ctx.err,
                    };
                    self.summary_screen.update(msg, ctx, db)
                }
                _ => vec![],
            },
        }
    }
    fn render(&self, frame: &mut Frame, area: Rect, ctx: &Model) {
        match &ctx.interaction_mode {
            InteractionMode::BrowsingLocation => {
                self.location_select_screen.render(frame, area, ctx)
            }
            InteractionMode::InspectingLocation { view, location } => match view {
                InspectingLocationView::SummaryScreen => {
                    let screen_ctx = SummaryScreenCtx {
                        location,
                        err: &ctx.err,
                    };
                    self.summary_screen.render(frame, area, screen_ctx)
                }
                _ => (),
            },
        }
    }
}
