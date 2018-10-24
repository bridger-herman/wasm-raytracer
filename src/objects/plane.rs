//! A plane

use intersection::Intersection;
use material::Material;
use objects::object::Object;
use ray::Ray;
use vector::Vector3;

const EPSILON: f64 = 0.001;

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Plane {
    pub fn new(material: Material, point: Vector3, normal: Vector3) -> Self {
        Self {
            point,
            normal: normal.normalized(),
            material,
        }
    }
}

impl Object for Plane {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let d = self.point.dot(&self.normal);
        let t = -(ray.start.dot(&self.normal) - d)
            / (ray.direction.dot(&self.normal));
        if t >= EPSILON {
            let p = ray.eval(t);
            if p.is_none() {
                return None;
            }
            let p = p.unwrap();
            Some(Intersection::new(self.normal, p))
        } else {
            None
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn info(&self) -> String {
        format!("Plane: {:?} {:?}", self.point, self.normal)
    }
}
