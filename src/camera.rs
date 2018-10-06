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

impl Camera {
    pub fn from_parameters(parameters: &[f64]) -> Self {
        assert_eq!(parameters.len(), 10);
        Self {
            position: Vector3::from(&parameters[0..3]),
            direction: Vector3::from(&parameters[3..6]),
            up: Vector3::from(&parameters[6..9]),
            half_angle: parameters[9],
        }
    }
}
