//! A material to apply to a ray-traced object

use crate::pixel::Pixel;

/// Ray tracing material
///
/// Material properties:
/// - ambient
/// - diffuse
/// - specular
/// - transmissive
/// - index of refraction
#[derive(Debug, Clone)]
pub struct Material {
    pub ambient: Pixel,
    pub diffuse: Pixel,
    pub specular: Pixel,
    pub phong_power: f64,
    pub transmissive: Pixel,
    pub ior: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Pixel::from_rgb(0.0, 0.0, 0.0),
            diffuse: Pixel::from_rgb(1.0, 1.0, 1.0),
            specular: Pixel::from_rgb(0.0, 0.0, 0.0),
            phong_power: 5.0,
            transmissive: Pixel::from_rgb(0.0, 0.0, 0.0),
            ior: 1.0,
        }
    }
}

impl Material {
    pub fn new(
        ambient: Pixel,
        diffuse: Pixel,
        specular: Pixel,
        phong_power: f64,
        transmissive: Pixel,
        ior: f64,
    ) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            phong_power,
            transmissive,
            ior,
        }
    }
}
