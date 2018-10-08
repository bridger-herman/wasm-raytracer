//! Point light. Radiates equally in all directions.

use pixel::Pixel;
use vector::Vector3;

#[derive(Debug)]
pub struct PointLight {
    pub color: Pixel,
    pub position: Vector3,
}

impl PointLight {
    pub fn new(color: Pixel, position: Vector3) -> Self {
        Self { color, position }
    }
}
