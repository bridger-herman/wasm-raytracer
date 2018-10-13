//! A generic light.
//!
//! Calculates the diffuse and specular components for a point on a surface.

use std::fmt;

use camera::Camera;
use intersection::Intersection;
use material::Material;
use pixel::Pixel;
use vector::Vector3;

pub trait Light {
    /// Calculate the direction of the light from the intersection
    fn direction(&self, intersection: &Intersection) -> Vector3;

    /// Calculate the diffuse component
    fn diffuse(
        &self,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel;

    /// Calculate the specular component
    fn specular(
        &self,
        camera: &Camera,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel;
}

impl fmt::Debug for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Light")
    }
}
