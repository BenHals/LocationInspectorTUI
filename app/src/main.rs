use crate::{app::App, event::poll_and_handle_event, model::ApplicationStatus};

mod app;
mod component;
mod event;
mod message;
mod model;
mod tui;
mod update;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let mut app = App::new();
    while app.model.application_status == ApplicationStatus::Running {
        terminal.draw(|frame| app.render(frame))?;
        if let Some(msg) = poll_and_handle_event()? {
            app.handle(msg);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
