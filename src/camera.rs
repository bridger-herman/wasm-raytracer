//! A virtual camera

use vector::Vector3;

/// A virtual camera
#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,
    pub up: Vector3,
    pub half_angle: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            half_angle: 45.0,
        }
    }
}
