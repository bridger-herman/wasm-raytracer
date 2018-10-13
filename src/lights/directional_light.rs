//! Directional light. Simulates a star-like light.

use camera::Camera;
use intersection::Intersection;
use lights::light::Light;
use material::Material;
use pixel::Pixel;
use vector::Vector3;

#[derive(Debug)]
pub struct DirectionalLight {
    pub color: Pixel,
    pub direction: Vector3,
}

impl DirectionalLight {
    pub fn new(color: Pixel, direction: Vector3) -> Self {
        Self { color, direction }
    }
}

impl Light for DirectionalLight {
    fn to_light(&self, _intersection: &Intersection) -> Vector3 {
        -self.direction
    }

    fn diffuse(
        &self,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel {
        let to_light = self.to_light(intersection);

        let angle = intersection
            .surface_normal
            .dot(&to_light.normalized())
            .max(0.0);
        self.color * material.diffuse * angle
    }

    fn specular(
        &self,
        camera: &Camera,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel {
        let to_light = self.to_light(intersection);

        let view = (camera.position - intersection.point).normalized();
        let reflection =
            to_light.normalized().reflect(&intersection.surface_normal);
        let phong_dot =
            view.dot(&reflection).min(0.0).powf(material.phong_power);
        self.color.clamp() * material.specular * phong_dot
    }
}
