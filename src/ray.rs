//! A half-line (a starting point and a direction)

use vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub start: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(start: Vector3, direction: Vector3) -> Self {
        Self {
            start,
            direction: direction.normalized(),
        }
    }
}
