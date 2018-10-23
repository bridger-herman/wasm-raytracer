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
    pub n1: Vector3,
    pub n2: Vector3,
    pub n3: Vector3,
    pub material: Material,
    plane_normal: Vector3,
}

impl Triangle {
    pub fn new(
        material: Material,
        v1: Vector3,
        v2: Vector3,
        v3: Vector3,
        n1: Vector3,
        n2: Vector3,
        n3: Vector3,
    ) -> Self {
        Self {
            v1,
            v2,
            v3,
            n1,
            n2,
            n3,
            material,
            plane_normal: (v1 - v2).cross(&(v3 - v2)).normalized(),
        }
    }

    pub fn single_normal(
        material: Material,
        v1: Vector3,
        v2: Vector3,
        v3: Vector3,
        n: Vector3,
    ) -> Self {
        Self {
            v1,
            v2,
            v3,
            n1: n,
            n2: n,
            n3: n,
            material,
            plane_normal: n,
        }
    }
}

impl Object for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let d = self.v1.dot(&self.plane_normal);
        let t = -(ray.start.dot(&self.plane_normal) - d)
            / (ray.direction.dot(&self.plane_normal));
        if t >= EPSILON {
            let p = ray.eval(t);
            if same_side(p, self.v1, self.v2, self.v3)
                && same_side(p, self.v2, self.v1, self.v3)
                && same_side(p, self.v3, self.v1, self.v2)
            {
                Some(Intersection::new(
                    bary_interp(
                        p, self.v1, self.v2, self.v3, self.n1, self.n2,
                        self.n3,
                    ),
                    p,
                ))
            // Some(Intersection::new(self.plane_normal, p))
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
        format!("Triangle: {:?} {:?} {:?}", self.v1, self.v2, self.v3)
    }
}

fn same_side(p1: Vector3, p2: Vector3, a: Vector3, b: Vector3) -> bool {
    let cp1 = (b - a).cross(&(p1 - a));
    let cp2 = (b - a).cross(&(p2 - a));
    cp1.dot(&cp2) >= 0.0
}

fn bary_interp(
    point: Vector3,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    n1: Vector3,
    n2: Vector3,
    n3: Vector3,
) -> Vector3 {
    let d1 = (point - p1).length();
    let d2 = (point - p2).length();
    let d3 = (point - p3).length();
    let total = (d1 * d1 + d2 * d2 + d3 * d3).sqrt();
    let d1 = d1 / total;
    let d2 = d2 / total;
    let d3 = d3 / total;
    (n1 * d1 + n2 * d2 + n3 * d3).normalized()
}
