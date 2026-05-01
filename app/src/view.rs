use ratatui::{layout::Rect, Frame};

use crate::{
    component::Component,
    db::file_db::FileDB,
    message::Message,
    model::{InspectingLocationView, InteractionMode, Model},
    screens::{
        inspect_screen::{InspectScreen, InspectScreenCtx},
        location_select_screen::LocationSelectScreen,
        summary_screen::{SummaryScreen, SummaryScreenCtx},
    },
    update::Update,
};

pub struct View {
    pub location_select_screen: LocationSelectScreen,
    pub summary_screen: SummaryScreen,
    pub inspect_screen: InspectScreen,
}

impl View {
    pub fn new(db: &FileDB) -> Self {
        Self {
            location_select_screen: LocationSelectScreen::new(db),
            summary_screen: SummaryScreen::new(),
            inspect_screen: InspectScreen::new(),
        }
    }
}

impl Component for View {
    type Ctx<'a> = &'a Model;
    fn update(&mut self, msg: &Message, ctx: &Model, db: &FileDB) -> (Vec<Update>, Vec<Message>) {
        match msg {
            Message::Quit => return (vec![Update::Quit], vec![]),
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
                InspectingLocationView::InspectScreen => {
                    let ctx = InspectScreenCtx {
                        location,
                        err: &ctx.err,
                    };
                    self.inspect_screen.update(msg, ctx, db)
                }
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
                InspectingLocationView::InspectScreen => {
                    let screen_ctx = InspectScreenCtx {
                        location,
                        err: &ctx.err,
                    };
                    self.inspect_screen.render(frame, area, screen_ctx)
                }
            },
        }
    }
}
