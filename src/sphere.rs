//! Representation of a sphere to be ray traced

use intersection::Intersection;
use material::Material;
use ray::Ray;
use vector::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub position: Vector3,
    pub material: Material,
}

const EPSILON: f64 = 0.001;

impl Sphere {
    pub fn new(radius: f64, position: Vector3, material: Material) -> Self {
        Self {
            radius,
            position,
            material,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Solve a quadratic to see if the ray intersects the sphere
        let start_to_center = ray.start - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&start_to_center);
        let c =
            start_to_center.dot(&start_to_center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            let p1 = ray.eval(t1);
            let p2 = ray.eval(t2);
            if t1 < t2 && t1 > EPSILON {
                Some(Intersection::new((p1 - self.position).normalized(), p1))
            } else if t1 >= t2 && t2 > EPSILON {
                Some(Intersection::new((p2 - self.position).normalized(), p2))
            } else {
                None
            }
        } else {
            None
        }
    }
}
