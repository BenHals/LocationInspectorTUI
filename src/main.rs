use loctui::db::dict_db::DictDb;
use loctui::event_handling::handle_event;
use loctui::model::{update, ApplicationState, Model};
use loctui::tui;
use loctui::view::view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;
    let db = DictDb::new();
    let mut app_state = Model::new(db);

    while app_state.state.app_state != ApplicationState::Done {
        terminal.draw(|frame| view(&mut app_state, frame))?;

        let mut current_msg = handle_event(&app_state)?;
        while current_msg.is_some() {
            (app_state, current_msg) = update(app_state, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
