//! A half-line (a starting point and a direction)

use vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub start: Vector3,
    pub direction: Vector3,
    pub t_min: f64,
    pub t_max: f64,
}

impl Ray {
    pub fn new(start: Vector3, direction: Vector3) -> Self {
        Self::with_t_max(start, direction, 1000.0)
    }

    pub fn with_t_max(start: Vector3, direction: Vector3, t_max: f64) -> Self {
        Self {
            start,
            direction: direction.normalized(),
            t_min: 0.0,
            t_max,
        }
    }

    /// Try to evaluate a ray, if the given `t` is within bounds
    pub fn eval(&self, t: f64) -> Option<Vector3> {
        if t > self.t_max || t < self.t_min {
            None
        } else {
            Some(self.start + self.direction * t)
        }
    }

    pub fn reflect(&self, new_start: Vector3, normal: Vector3) -> Self {
        Self::new(new_start, self.direction.reflect(&normal).normalized())
    }

    pub fn refract(
        &self,
        new_start: Vector3,
        normal: Vector3,
        ior: f64,
    ) -> Self {
        Self::new(new_start, self.direction.refract(normal, ior).normalized())
    }
}
