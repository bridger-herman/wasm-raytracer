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

    pub fn eval(&self, t: f64) -> Vector3 {
        self.start + self.direction * t
    }

    pub fn reflect(&self, new_start: Vector3, normal: Vector3) -> Self {
        Self {
            start: new_start,
            direction: self.direction.reflect(&normal).normalized(),
        }
    }

    pub fn refract(
        &self,
        new_start: Vector3,
        normal: Vector3,
        ior: f64,
    ) -> Self {
        Self {
            start: new_start,
            direction: self.direction.refract(normal, ior).normalized(),
        }
    }
}
