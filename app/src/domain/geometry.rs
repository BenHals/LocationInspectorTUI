use std::marker::PhantomData;

pub trait Projection {}
pub struct Local;
pub struct WGS84;
impl Projection for Local {}
impl Projection for WGS84 {}

pub struct Point<P: Projection> {
    x: f64,
    y: f64,
    _proj: PhantomData<P>,
}

impl<P: Projection> Point<P> {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            _proj: PhantomData,
        }
    }
}
