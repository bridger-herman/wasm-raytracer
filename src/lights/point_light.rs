//! Point light. Radiates equally in all directions.

use camera::Camera;
use intersection::Intersection;
use lights::light::Light;
use material::Material;
use pixel::Pixel;
use vector::Vector3;

#[derive(Debug)]
pub struct PointLight {
    pub color: Pixel,
    pub position: Vector3,
}

impl PointLight {
    pub fn new(color: Pixel, position: Vector3) -> Self {
        Self { color, position }
    }
}

impl Light for PointLight {
    fn direction_to_light(&self, intersection: &Intersection) -> Vector3 {
        self.position - intersection.point
    }

    fn distance_to_light(&self, intersection: &Intersection) -> f64 {
        (self.position - intersection.point).length()
    }

    fn diffuse(
        &self,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel {
        let source_illumination =
            1.0 / (self.distance_to_light(intersection).powf(2.0));
        let angle = intersection
            .surface_normal
            .dot(&self.direction_to_light(intersection).normalized())
            .max(0.0);
        self.color * material.diffuse * angle * source_illumination
    }

    fn specular(
        &self,
        camera: &Camera,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel {
        let view = (camera.position - intersection.point).normalized();
        let reflection = self
            .direction_to_light(intersection)
            .normalized()
            .reflect(&intersection.surface_normal);
        let phong_dot =
            view.dot(&reflection).min(0.0).powf(material.phong_power);
        self.color.clamp() * material.specular * phong_dot
    }
}
