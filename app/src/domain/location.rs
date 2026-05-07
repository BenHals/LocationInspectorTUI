use std::path::Path;

use geo::LineString;
use geo::Polygon as GeoPolygon;

use crate::domain::geometry::{Local, Point, Polygon, RegionMetadata, WGS84};

pub struct Location {
    pub tag: LocationTag,
    pub latlng: Point<WGS84>,
    pub boundaries: Vec<Polygon<Local>>,
    pub regions: Vec<Polygon<Local>>,
}

pub struct LocationTag {
    pub id: String,
    pub name: String,
    pub coord: Point<WGS84>,
    pub country_code: String,
    pub country_subdivision: String,
    pub kind: String,
    pub status: String,
    pub created_date: String,
}

#[derive(serde::Deserialize)]
pub struct LocationFile {
    pub id: String,
    pub name: String,
    pub coord: [f64; 2],
    pub polygon_path: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "countrySubdivision")]
    pub country_subdivision: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub status: String,
    #[serde(rename = "createdDate")]
    pub created_date: String,
}

#[derive(serde::Deserialize)]
struct PolygonsFile {
    boundaries: Vec<PolygonEntry>,
    regions: Vec<PolygonEntry>,
}

/// JSON shape for one polygon entry. Extra fields in the file are ignored.
#[derive(serde::Deserialize)]
struct PolygonEntry {
    name: String,
    id: String,
    kind: Option<String>,
    category: String,
    ring: Vec<[f64; 2]>,
}

fn parse_polygon(entry: PolygonEntry) -> Polygon<Local> {
    let exterior = LineString::from(
        entry
            .ring
            .into_iter()
            .map(|p| (p[0], p[1]))
            .collect::<Vec<_>>(),
    );
    Polygon::new(
        RegionMetadata {
            name: entry.name,
            id: entry.id,
            kind: entry.kind,
            category: entry.category,
        },
        GeoPolygon::new(exterior, vec![]),
    )
}

impl LocationFile {
    pub fn get_location(&self, base_path: &Path) -> Option<Location> {
        let latlng = Point::new(self.coord[0], self.coord[1]);
        let polygon_path = base_path.join(self.polygon_path.clone());
        let raw_polygons = std::fs::read_to_string(polygon_path).ok()?;
        let parsed: PolygonsFile = serde_json::from_str(&raw_polygons).ok()?;
        let boundaries = parsed.boundaries.into_iter().map(parse_polygon).collect();
        let regions = parsed.regions.into_iter().map(parse_polygon).collect();
        Some(Location {
            tag: self.get_location_tag(),
            latlng,
            boundaries,
            regions,
        })
    }
    pub fn get_location_tag(&self) -> LocationTag {
        LocationTag {
            id: self.id.clone(),
            name: self.name.clone(),
            coord: Point::new(self.coord[0], self.coord[1]),
            country_code: self.country_code.clone(),
            country_subdivision: self.country_subdivision.clone(),
            kind: self.kind.clone(),
            status: self.status.clone(),
            created_date: self.created_date.clone(),
        }
    }
}
