//! Representation of a sphere to be ray traced

use crate::intersection::Intersection;
use crate::material::Material;
use crate::objects::object::Object;
use crate::ray::Ray;
use crate::vector::Vector3;

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
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
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
            if p1.is_some() && t1 < t2 && t1 > EPSILON {
                Some(Intersection::new(
                    (p1.unwrap() - self.position).normalized(),
                    p1.unwrap(),
                ))
            } else if p2.is_some() && t1 >= t2 && t2 > EPSILON {
                Some(Intersection::new(
                    (p2.unwrap() - self.position).normalized(),
                    p2.unwrap(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn info(&self) -> String {
        format!("Sphere: {:?} {:?}", self.position, self.radius)
    }
}
