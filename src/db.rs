pub mod dict_db;
use geo::Point;
use geo::Polygon;

pub trait DbConnection {
    fn get_id(self: &Self, idx: &usize) -> Option<String>;
    fn get_name(self: &Self, key: &String) -> Option<String>;
    fn get_latlng(self: &Self, id: &String) -> Option<Point>;
    fn get_polygons(self: &Self, id: &String) -> Option<Vec<Polygon>>;
}
