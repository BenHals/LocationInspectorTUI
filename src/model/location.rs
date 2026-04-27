use geo::Point;
use geo::Polygon;

pub struct Location {
    id: String,
    name: String,
    world_centroid: Point,
    polygons: Vec<Polygon>,
}
