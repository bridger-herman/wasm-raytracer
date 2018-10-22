//! A simple vector

use std::f64;
use std::ops::{Add, Mul, Neg, Sub};

pub const MAX_VECTOR3: Vector3 = Vector3 {
    x: f64::MAX,
    y: f64::MAX,
    z: f64::MAX,
};

/// 3 dimensional vector
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'a> From<&'a [f64]> for Vector3 {
    fn from(slice: &'a [f64]) -> Self {
        assert_eq!(slice.len(), 3);
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn reflect(self, normal: &Self) -> Self {
        self - (*normal * self.dot(normal)) * 2.0
    }

    // Uses the same math from the GLM-rs library
    pub fn refract(self, normal: Self, eta: f64) -> Self {
        let dot_ni = self.dot(&normal);

        let k = 1.0 - eta * eta * (1.0 - dot_ni) * dot_ni;
        if k < 0.0 {
            Vector3::default()
        } else {
            self * eta - normal * (eta * dot_ni + k.sqrt())
        }
    }

    pub fn angle(&self, other: &Self) -> f64 {
        self.normalized().dot(&other.normalized()).acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glm;

    const EPSILON: f32 = 0.000001;

    fn make_vectors() -> (glm::Vec3, glm::Vec3, Vector3, Vector3) {
        let a_glm = glm::Vec3::new(1.0, 1.0, 1.0);
        let b_glm = glm::Vec3::new(1.0, 0.0, 1.0);

        let a = Vector3::new(1.0, 1.0, 1.0);
        let b = Vector3::new(1.0, 0.0, 1.0);
        (a_glm, b_glm, a, b)
    }
    #[test]
    fn cross() {
        let (a_glm, b_glm, a, b) = make_vectors();

        let cross_glm = glm::cross(a_glm, b_glm);
        let cross = a.cross(&b);

        assert_eq!(cross.x as f32, cross_glm.x);
        assert_eq!(cross.y as f32, cross_glm.y);
        assert_eq!(cross.z as f32, cross_glm.z);
    }

    #[test]
    fn dot() {
        let (a_glm, b_glm, a, b) = make_vectors();

        let dot_glm = glm::dot(a_glm, b_glm);
        let dot = a.dot(&b);
        assert_eq!(dot as f32, dot_glm);
    }

    #[test]
    fn length() {
        let (a_glm, _b_glm, a, _b) = make_vectors();

        let len_glm = glm::length(a_glm);
        let len = a.length();
        assert_eq!(len as f32, len_glm);
    }

    #[test]
    fn normalized() {
        let (a_glm, _b_glm, a, _b) = make_vectors();

        let norm_glm = glm::normalize(a_glm);
        let norm = a.normalized();
        assert_eq!(norm.x as f32, norm_glm.x);
        assert_eq!(norm.y as f32, norm_glm.y);
        assert_eq!(norm.z as f32, norm_glm.z);
    }

    #[test]
    fn reflect() {
        let (a_glm, b_glm, a, b) = make_vectors();

        let reflect_glm = glm::reflect(a_glm, b_glm);
        let reflect = a.reflect(&b);
        assert_eq!(reflect.x as f32, reflect_glm.x);
        assert_eq!(reflect.y as f32, reflect_glm.y);
        assert_eq!(reflect.z as f32, reflect_glm.z);
    }

    #[test]
    fn refract() {
        // let (a_glm, b_glm, a, b) = make_vectors();
        let (a_glm, b_glm, a, b) = (
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::normalize(glm::Vec3::new(-1.0, -1.0, 0.0)),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0).normalized(),
        );
        println!("a: {:?} {:?}", a_glm, a);
        println!("b: {:?} {:?}", b_glm, b);

        let refract_glm = glm::refract(a_glm, b_glm, 1.0);
        let refract = a.refract(b, 1.0);
        println!("{:?} {:?}", refract_glm, refract);
        assert!((refract.x as f32 - refract_glm.x) < EPSILON);
        assert!((refract.y as f32 - refract_glm.y) < EPSILON);
        assert!((refract.z as f32 - refract_glm.z) < EPSILON);
    }

    #[test]
    fn angle() {
        let (a_glm, b_glm, a, b) = make_vectors();

        let angle_glm = glm::ext::angle(a_glm, b_glm);
        let angle = a.angle(&b);
        assert!((angle as f32 - angle_glm).abs() < EPSILON);
    }
}
