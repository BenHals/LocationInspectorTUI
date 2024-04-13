use std::collections::HashMap;

use super::DbConnection;

pub struct DictDb {
    ids: Vec<String>,
    name_data: HashMap<String, String>,
}

impl DictDb {
    pub fn new() -> Self {
        let ids = vec!["001".to_string(), "010".to_string()];
        let mut name_data = HashMap::new();
        name_data.insert("001".to_string(), "Auckland".to_string());
        Self { ids, name_data }
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
}
