pub mod dict_db;

pub trait DbConnection {
    fn get_id(self: &Self, idx: &usize) -> Option<String>;
    fn get_name(self: &Self, key: &String) -> Option<String>;
}
