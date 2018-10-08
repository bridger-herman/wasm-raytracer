//! Intersection record

use vector::Vector3;

#[derive(Debug, Default)]
pub struct Intersection {
    pub surface_normal: Vector3,
    pub point: Vector3,
}

impl Intersection {
    pub fn new(surface_normal: Vector3, point: Vector3) -> Self {
        Self {
            surface_normal,
            point,
        }
    }
}
