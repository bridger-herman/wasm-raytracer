//! A simple vector

/// 3 dimensional vector
#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'a> From<&'a [f64]> for Vector3 {
    fn from(slice: &'a [f64]) -> Self {
        assert_eq!(slice.len(), 3);
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
