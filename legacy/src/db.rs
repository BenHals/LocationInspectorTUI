pub mod dict_db;
use geo::Point;
use geo::Polygon;

pub trait DbConnection {
    fn get_id(&self, idx: &usize) -> Option<String>;
    fn get_name(&self, key: &str) -> Option<String>;
    fn get_latlng(&self, id: &str) -> Option<Point>;
    fn get_polygons(&self, id: &str) -> Option<Vec<Polygon>>;
}
