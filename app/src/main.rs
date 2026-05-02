use std::{path::Path, sync::mpsc};

use crate::{
    app::App, config::Config, db::file_db::FileDB, event::poll_and_handle_event,
    model::ApplicationStatus, update::Update,
};

mod app;
mod coastlines;
mod component;
mod components;
mod config;
mod db;
mod domain;
mod event;
mod layers;
mod message;
mod model;
mod screens;
mod tui;
mod update;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = match std::env::var("LOCTUI_CONFIG").ok() {
        Some(config_path) => {
            let raw = std::fs::read_to_string(config_path)?;
            toml::from_str(&raw)?
        }
        None => Config::default(),
    };
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let db = FileDB::new(Path::new(&config.data.root_dir.join("locations.json")))?;
    let (async_tx, async_rx) = mpsc::channel::<Update>();
    let mut app = App::new(db, config, async_tx);
    while app.model.application_status == ApplicationStatus::Running {
        terminal.draw(|frame| app.render(frame))?;

        while let Ok(u) = async_rx.try_recv() {
            app.model.apply(u);
        }

        if let Some(msg) = poll_and_handle_event()? {
            app.handle(msg);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
