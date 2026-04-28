use std::{collections::HashMap, error, path::Path};

use crate::{
    db::db_connection::DBConnection,
    domain::location::{Location, LocationFile, LocationTag},
};

pub struct FileDB {
    base_path: String,
    locations: HashMap<String, LocationFile>,
}

impl FileDB {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = std::fs::read_to_string(path)?;
        let entries: Vec<LocationFile> = serde_json::from_str(&raw)?;
        let mut map = HashMap::new();
        for l in entries {
            map.insert(l.id.clone(), l);
        }
        let base_path = path.parent().unwrap().to_str().unwrap().clone();
        Ok(Self {
            base_path: String::from(base_path),
            locations: map,
        })
    }
}

impl DBConnection for FileDB {
    fn get_tags(&self) -> Vec<LocationTag> {
        self.locations
            .values()
            .map(|l| l.get_location_tag())
            .collect()
    }
    fn get_by_id(&self, id: &String) -> Option<Location> {
        let location_file = self.locations.get(id)?;
        location_file.get_location(Path::new(&self.base_path))
    }
}
