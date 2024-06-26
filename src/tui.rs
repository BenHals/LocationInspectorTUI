use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::Stdout;
use std::{io::stdout, panic};

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}


