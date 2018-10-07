//! Representation of a sphere to be ray traced

use ray::Ray;
use vector::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub position: Vector3,
    // TODO add material
}

impl Sphere {
    pub fn new(radius: f64, position: Vector3) -> Self {
        Self { radius, position }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        // Solve a quadratic to see if the ray intersects the sphere
        let a = ray.direction.dot(&ray.direction);
        let b = (ray.direction * 2.0).dot(&(ray.start - self.position));
        let start_to_center = ray.start - self.position;
        let c =
            start_to_center.dot(&start_to_center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
        // let p1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        // let p2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    }
}
