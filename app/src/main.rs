use crate::{event::poll_and_handle_event, message::Message};

mod appstate;
mod event;
mod message;
mod tui;
mod update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let mut running = true;
    while running {
        terminal.draw(|frame| ())?;
        if let Some(msg) = poll_and_handle_event()? {
            match msg {
                Message::Quit => running = false,
                _ => (),
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
