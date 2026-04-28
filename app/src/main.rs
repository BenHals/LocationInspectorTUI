use std::time::Duration;

use crossterm::event::{self, Event};

mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("loctui — fresh build");
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let mut running = true;
    while running {
        terminal.draw(|frame| ())?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                    running = false;
                }
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
