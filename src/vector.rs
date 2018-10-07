//! A simple vector

use std::ops::{Add, Mul, Sub};

/// 3 dimensional vector
#[derive(Debug, PartialEq, Clone, Copy)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use glm;

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
}
