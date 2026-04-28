use crate::{
    event::poll_and_handle_event,
    message::Message,
    model::{Model, ApplicationStatus},
    update::Update,
};

mod event;
mod message;
mod model;
mod tui;
mod update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let mut model = Model::new();
    while model.application_status == ApplicationStatus::Running {
        terminal.draw(|frame| ())?;
        if let Some(msg) = poll_and_handle_event()? {
            let update = match msg {
                Message::Quit => Some(Update::Quit),
                _ => None,
            };
            if let Some(u) = update {
                model.apply(u);
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
