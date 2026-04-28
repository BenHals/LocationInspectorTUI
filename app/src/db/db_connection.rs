use crate::domain::location::{Location, LocationTag};

pub trait DBConnection {
    fn get_by_id(&self, key: &String) -> Option<Location>;
    fn get_tags(&self) -> Vec<LocationTag>;
}
