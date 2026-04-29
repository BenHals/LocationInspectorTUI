use std::path::Path;

use crate::{
    app::App, db::file_db::FileDB, event::poll_and_handle_event, model::ApplicationStatus,
};

mod app;
mod component;
mod components;
mod db;
mod domain;
mod event;
mod message;
mod model;
mod screens;
mod tui;
mod update;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let db_path_str = std::env::var("DB_LOCATIONS_FILE")?;
    let db = FileDB::new(Path::new(&db_path_str))?;
    let mut app = App::new(db);
    while app.model.application_status == ApplicationStatus::Running {
        terminal.draw(|frame| app.render(frame))?;
        if let Some(msg) = poll_and_handle_event()? {
            app.handle(msg);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
