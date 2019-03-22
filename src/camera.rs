//! A virtual camera

use crate::vector::Vector3;

/// A virtual camera
#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub vert_half_angle: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            right: Vector3::new(1.0, 0.0, 0.0),
            vert_half_angle: 45.0_f64.to_radians(),
        }
    }
}

impl Camera {
    pub fn from_parameters(parameters: &[f64]) -> Self {
        assert_eq!(parameters.len(), 10);
        let direction = Vector3::from(&parameters[3..6]).normalized();
        let up = Vector3::from(&parameters[6..9]).normalized();
        let right = up.cross(&direction).normalized();
        let up = if direction.dot(&up) != 0.0 {
            direction.cross(&right)
        } else {
            up
        };
        Self {
            position: Vector3::from(&parameters[0..3]),
            direction,
            up,
            right,
            vert_half_angle: parameters[9].to_radians(),
        }
    }
}
