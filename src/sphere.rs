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
        let a = ray.direction.dot(&ray.direction);
        let b = (ray.direction * 2.0).dot(&(ray.start - self.position));
        let start_to_center = ray.start - self.position;
        let c =
            start_to_center.dot(&start_to_center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            let t2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            let p1 = ray.eval(t1);
            let p2 = ray.eval(t2);
            let p1_dist = (p1 - ray.start).length();
            let p2_dist = (p2 - ray.start).length();
            if p1_dist < p2_dist {
                Some(Intersection::new((p1 - self.position).normalized(), p1))
            } else {
                Some(Intersection::new((p2 - self.position).normalized(), p2))
            }
        } else {
            None
        }
    }
}
