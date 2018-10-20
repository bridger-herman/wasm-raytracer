//! A single triangle

use intersection::Intersection;
use material::Material;
use objects::object::Object;
use ray::Ray;
use vector::Vector3;

const EPSILON: f64 = 0.001;

// Disclaimer: I know this is a poor way to represent triangles - I just don't
// have the time to make it good because I'll likely run out of time on more
// important parts.
pub struct Triangle {
    pub v1: Vector3,
    pub v2: Vector3,
    pub v3: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Triangle {
    pub fn new(
        material: Material,
        v1: Vector3,
        v2: Vector3,
        v3: Vector3,
    ) -> Self {
        Self {
            v1,
            v2,
            v3,
            normal: (v1 - v2).cross(&(v3 - v2)).normalized(), // TODO: Fix this
            material,
        }
    }
}

impl Object for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let d = self.v1.dot(&self.normal);
        let t = -(ray.start.dot(&self.normal) + d)
            / (ray.direction.dot(&self.normal));
        if t >= EPSILON {
            let p = ray.eval(t);
            if same_side(p, self.v1, self.v2, self.v3)
                && same_side(p, self.v2, self.v1, self.v3)
                && same_side(p, self.v3, self.v1, self.v2)
            {
                Some(Intersection::new(self.normal, p))
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
}

fn same_side(p1: Vector3, p2: Vector3, a: Vector3, b: Vector3) -> bool {
    let cp1 = (b - a).cross(&(p1 - a));
    let cp2 = (b - a).cross(&(p2 - a));
    cp1.dot(&cp2) >= 0.0
}
