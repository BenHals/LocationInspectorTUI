use std::path::Path;

use geo::LineString;
use geo::Polygon as GeoPolygon;

use crate::domain::geometry::{Local, Point, Polygon, RegionMetadata, WGS84};

pub struct Location {
    pub tag: LocationTag,
    pub latlng: Point<WGS84>,
    pub polygons: Vec<Polygon<Local>>,
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

/// JSON shape for one polygon entry. Extra fields in the file are ignored.
#[derive(serde::Deserialize)]
struct PolygonFile {
    name: String,
    id: String,
    kind: Option<String>,
    category: String,
    ring: Vec<[f64; 2]>,
}

impl LocationFile {
    pub fn get_location(&self, base_path: &Path) -> Option<Location> {
        let latlng = Point::new(self.coord[0], self.coord[1]);
        let polygon_path = base_path.join(self.polygon_path.clone());
        let raw_polygons = std::fs::read_to_string(polygon_path).ok()?;
        let entries: Vec<PolygonFile> = serde_json::from_str(&raw_polygons).ok()?;
        let polygons: Vec<Polygon<Local>> = entries
            .into_iter()
            .map(|pf| {
                let exterior = LineString::from(
                    pf.ring
                        .into_iter()
                        .map(|p| (p[0], p[1]))
                        .collect::<Vec<_>>(),
                );
                Polygon::new(
                    RegionMetadata {
                        name: pf.name,
                        id: pf.id,
                        kind: pf.kind,
                        category: pf.category,
                    },
                    GeoPolygon::new(exterior, vec![]),
                )
            })
            .collect();
        Some(Location {
            tag: LocationTag {
                id: self.id.clone(),
                name: self.name.clone(),
            },
            latlng,
            polygons,
        })
    }
    pub fn get_location_tag(&self) -> LocationTag {
        LocationTag {
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}
