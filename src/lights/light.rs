//! A generic light.
//!
//! Calculates the diffuse and specular components for a point on a surface.

use std::fmt;

use crate::camera::Camera;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::pixel::Pixel;
use crate::vector::Vector3;

pub trait Light {
    /// Calculate the direction of the light from the intersection
    fn direction_to_light(&self, intersection: &Intersection) -> Vector3;

    /// Calculate the distance to the light from the intersection
    fn distance_to_light(&self, intersection: &Intersection) -> f64;

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
