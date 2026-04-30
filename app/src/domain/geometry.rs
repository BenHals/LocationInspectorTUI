use geo::LineString as GeoLineString;
use geo::Polygon as GeoPolygon;
use std::marker::PhantomData;

pub trait Projection {
    // How many units fit into one cell in each direction
    // at scale = 1.
    const UNITS_PER_CELL_X: f64;
    // Note: in a terminal, Y is twice the size,
    // so it should probably be twice the amount
    const UNITS_PER_CELL_Y: f64;
}
pub struct Local;
pub struct WGS84;
impl Projection for Local {
    const UNITS_PER_CELL_X: f64 = 50.0;
    const UNITS_PER_CELL_Y: f64 = 100.0;
}
impl Projection for WGS84 {
    const UNITS_PER_CELL_X: f64 = 2.0;
    const UNITS_PER_CELL_Y: f64 = 4.0;
}

pub struct Point<P: Projection> {
    pub x: f64,
    pub y: f64,
    _proj: PhantomData<P>,
}

impl<P: Projection> Point<P> {
    pub const fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            _proj: PhantomData,
        }
    }
}

pub struct Polygon<P: Projection> {
    pub inner: GeoPolygon,
    _proj: PhantomData<P>,
}

impl<P: Projection> Polygon<P> {
    pub fn new(inner: GeoPolygon) -> Self {
        Self {
            inner,
            _proj: PhantomData,
        }
    }
}

pub struct Polyline<P: Projection> {
    pub inner: GeoLineString,
    _proj: PhantomData<P>,
}

impl<P: Projection> Polyline<P> {
    pub fn new(inner: GeoLineString) -> Self {
        Self {
            inner,
            _proj: PhantomData,
        }
    }
}
