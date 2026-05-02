use std::sync::OnceLock;

use geo::LineString;
use geojson::{GeoJson, Value};

use crate::domain::geometry::{Polyline, WGS84};

const RAW: &str = include_str!("assets/coastlines.json");

pub fn coastlines() -> &'static [Polyline<WGS84>] {
    static CACHE: OnceLock<Vec<Polyline<WGS84>>> = OnceLock::new();
    CACHE.get_or_init(parse).as_slice()
}

fn parse() -> Vec<Polyline<WGS84>> {
    let geojson: GeoJson = RAW.parse().expect("Coastline GeoJson error while parsing");
    let features = match geojson {
        GeoJson::FeatureCollection(fc) => fc.features,
        _ => panic!("Expected feature collection"),
    };
    let mut polylines = Vec::new();
    for feature in features {
        if let Some(geometry) = feature.geometry {
            collect(&geometry.value, &mut polylines);
        }
    }
    polylines
}

fn collect(value: &Value, out: &mut Vec<Polyline<WGS84>>) {
    match value {
        Value::LineString(coords) => {
            let line = LineString::from(coords.iter().map(|c| (c[0], c[1])).collect::<Vec<_>>());
            out.push(Polyline::new(line));
        }
        Value::MultiLineString(lines) => {
            for coords in lines {
                let line =
                    LineString::from(coords.iter().map(|c| (c[0], c[1])).collect::<Vec<_>>());
                out.push(Polyline::new(line));
            }
        }
        _ => {} // ignore Point, Polygon, etc. — we only want lines
    }
}
