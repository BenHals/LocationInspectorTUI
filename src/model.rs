use crate::{db::DbConnection, event_handling::Message};

#[derive(Debug)]
pub struct Model<D: DbConnection> {
    pub key: usize,
    pub app_state: ApplicationState,
    pub active_screen: Screen,
    db: D,
}

#[derive(Debug)]
pub struct MainScreen {}

#[derive(Debug)]
pub struct SummaryScreen {
    pub id: String,
}

impl<T: DbConnection> Model<T> {
    pub fn new(db: T) -> Self {
        Self {
            key: 0,
            app_state: ApplicationState::Running,
            active_screen: Screen::Main(MainScreen {}),
            db,
        }
    }

    pub fn get_id(self: &Self) -> Option<String> {
        match &self.active_screen {
            Screen::Main(MainScreen {}) => self.db.get_id(&self.key),
            Screen::Summary(SummaryScreen { id }) => Some(id.clone()),
        }
    }
    pub fn get_name(self: &Self) -> Option<String> {
        match &self.active_screen {
            Screen::Main(MainScreen {}) => None,
            Screen::Summary(SummaryScreen { id }) => self.db.get_name(&id),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ApplicationState {
    Running,
    Loading,
    Done,
    Crashed,
}

#[derive(Debug)]
pub enum Screen {
    Main(MainScreen),
    Summary(SummaryScreen),
}

pub fn update<T: DbConnection>(model: &mut Model<T>, msg: Message) -> Option<Message> {
    match model.active_screen {
        Screen::Main(MainScreen {}) => match msg {
            Message::Increment => {
                model.key += 1;
            }
            Message::Decrement => {
                if model.key > 0 {
                    model.key -= 1;
                }
            }
            Message::Select => match model.get_id() {
                None => (),
                Some(id) => model.active_screen = Screen::Summary(SummaryScreen { id }),
            },
            Message::Reset => model.key = 0,
            Message::Quit => {
                model.app_state = ApplicationState::Done;
            }
        },
        _ => match msg {
            Message::Quit => {
                model.app_state = ApplicationState::Done;
            }
            _ => (),
        },
    };
    None
}
