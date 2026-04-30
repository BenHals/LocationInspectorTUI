use std::path::Path;

use geo::{Centroid, LineString};
use geo::Polygon as GeoPolygon;

use crate::domain::geometry::{Local, Point, Polygon, WGS84};

pub struct Location {
    pub tag: LocationTag,
    pub latlng: Point<WGS84>,
    pub polygons: Vec<Polygon<Local>>,
    pub local_center: Point<Local>,
}

pub struct LocationTag {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct LocationFile {
    pub id: String,
    pub name: String,
    pub coord: [f64; 2],
    pub polygon_path: String,
}

impl LocationFile {
    pub fn get_location(&self, base_path: &Path) -> Option<Location> {
        let latlng = Point::new(self.coord[0], self.coord[1]);
        let polygon_path = base_path.join(self.polygon_path.clone());
        let raw_polygons = std::fs::read_to_string(polygon_path).ok()?;
        let rings: Vec<Vec<[f64; 2]>> = serde_json::from_str(&raw_polygons).ok()?;
        let polygons: Vec<Polygon<Local>> = rings
            .into_iter()
            .map(|ring| {
                let exterior =
                    LineString::from(ring.into_iter().map(|p| (p[0], p[1])).collect::<Vec<_>>());
                Polygon::new(GeoPolygon::new(exterior, vec![]))
            })
            .collect();
        let (sx, sy, n) = polygons.iter().filter_map(|p| p.inner.centroid()).fold((0.0, 0.0, 0_usize), |(sx, sy, n), c| {(sx + c.x(), sy + c.y(), n + 1)});
        let local_center = if n > 0 {
            Point::<Local>::new(sx / n as f64, sy / n as f64)
        } else {
            Point::<Local>::new(0.0, 0.0)
        };
        Some(Location {
            tag: LocationTag {
                id: self.id.clone(),
                name: self.name.clone(),
            },
            latlng,
            polygons,
            local_center,
        })
    }
    pub fn into_location_tag(self) -> LocationTag {
        LocationTag {
            id: self.id,
            name: self.name,
        }
    }
    pub fn get_location_tag(&self) -> LocationTag {
        LocationTag {
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}
