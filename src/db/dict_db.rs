use std::collections::HashMap;

use geo::Point;

use super::DbConnection;

pub struct DictDb {
    ids: Vec<String>,
    name_data: HashMap<String, String>,
    loc_data: HashMap<String, Point>,
}

impl DictDb {
    pub fn new() -> Self {
        let ids = vec!["001".to_string(), "010".to_string()];
        let mut name_data = HashMap::new();
        name_data.insert("001".to_string(), "Auckland".to_string());
        let mut loc_data = HashMap::new();
        loc_data.insert("001".to_string(), Point::new(174.763336, -36.848461));
        Self {
            ids,
            name_data,
            loc_data,
        }
    }
}

impl DbConnection for DictDb {
    fn get_id(self: &Self, idx: &usize) -> Option<String> {
        Some(self.ids.get(*idx)?.clone())
    }

    fn get_name(&self, id: &String) -> Option<String> {
        let name = self.name_data.get(id)?.clone();
        Some(name)
    }

    fn get_latlng(self: &Self, id: &String) -> Option<Point> {
        let loc = self.loc_data.get(id)?.clone();
        Some(loc)
    }
}
