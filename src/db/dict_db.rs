use std::collections::HashMap;

use geo::Coord;
use geo::LineString;
use geo::Point;
use geo::Polygon;

use super::DbConnection;

pub struct DictDb {
    ids: Vec<String>,
    name_data: HashMap<String, String>,
    loc_data: HashMap<String, Point>,
    polygons: HashMap<String, Vec<Polygon>>,
}

impl DictDb {
    pub fn new() -> Self {
        let ids = vec!["001".to_string(), "010".to_string()];
        let mut name_data = HashMap::new();
        name_data.insert("001".to_string(), "Auckland".to_string());
        let mut loc_data = HashMap::new();
        loc_data.insert("001".to_string(), Point::new(174.763336, -36.848461));
        let mut polygons = HashMap::new();
        polygons.insert(
            "001".to_string(),
            vec![Polygon::new(
                LineString::new(vec![
                    Coord { x: 0.0, y: 0.0 },
                    Coord { x: 10.0, y: 0.0 },
                    Coord { x: 10.0, y: 10.0 },
                    Coord { x: 0.0, y: 10.0 },
                    Coord { x: 0.0, y: 0.0 },
                ]),
                Vec::new(),
            )],
        );
        Self {
            ids,
            name_data,
            loc_data,
            polygons,
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

    fn get_polygons(self: &Self, id: &String) -> Option<Vec<Polygon>> {
        let polys = self.polygons.get(id)?.clone();
        Some(polys)
    }
}
