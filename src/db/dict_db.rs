use core::panic;
use geo::Transform;
use proj::Proj;
use std::collections::HashMap;

use geo::Centroid;
use geo::Coord;
use geo::Geometry;
use geo::LineString;
use geo::Point;
use geo::Polygon;
use geojson::Feature;
use geojson::FeatureCollection;
use geojson::GeoJson;

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
        let geojson_str = std::fs::read_to_string("src/db/raw/001.geojson").unwrap();
        let geojson_fc: geojson::GeoJson = geojson_str.parse::<GeoJson>().unwrap();
        let feature: FeatureCollection = FeatureCollection::try_from(geojson_fc).unwrap();
        let mut auck_polys: Vec<Polygon> = Vec::new();
        let proj = Proj::new_known_crs("EPSG:4326", "EPSG:32760", None).unwrap();
        for f in feature.features {
            let g: Geometry = Geometry::try_from(f.geometry.unwrap()).unwrap();
            match g {
                Geometry::Polygon(poly) => {
                    // let poly: Polygon<f64> = Polygon::try_from(g).unwrap();
                    auck_polys.push(poly.transformed(&proj).unwrap());
                }
                Geometry::MultiPolygon(multipoly) => {
                    for poly in multipoly {
                        auck_polys.push(poly.transformed(&proj).unwrap());
                    }
                }
                _ => {}
            }
        }
        polygons.insert("001".to_string(), auck_polys);
        polygons.insert(
            "010".to_string(),
            vec![
                Polygon::new(
                    LineString::new(vec![
                        Coord { x: 0.0, y: 0.0 },
                        Coord { x: 10.0, y: 0.0 },
                        Coord { x: 10.0, y: 10.0 },
                        Coord { x: 0.0, y: 10.0 },
                        Coord { x: 0.0, y: 0.0 },
                    ]),
                    Vec::new(),
                ),
                Polygon::new(
                    LineString::new(vec![
                        Coord { x: 0.0, y: 0.0 },
                        Coord { x: -10.0, y: 0.0 },
                        Coord { x: -10.0, y: -10.0 },
                        Coord { x: 0.0, y: -10.0 },
                        Coord { x: 0.0, y: 0.0 },
                    ]),
                    Vec::new(),
                ),
            ],
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
