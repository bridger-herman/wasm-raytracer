//! Spot light. Has a position and a direction.

use crate::camera::Camera;
use crate::intersection::Intersection;
use crate::lights::light::Light;
use crate::material::Material;
use crate::pixel::Pixel;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct SpotLight {
    pub color: Pixel,
    pub position: Vector3,
    pub direction: Vector3,
    pub angle1: f64,
    pub angle2: f64,
}

impl SpotLight {
    pub fn new(
        color: Pixel,
        position: Vector3,
        direction: Vector3,
        angle1: f64,
        angle2: f64,
    ) -> Self {
        Self {
            color,
            position,
            direction,
            angle1: angle1.to_radians(),
            angle2: angle2.to_radians(),
        }
    }
}

impl Light for SpotLight {
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
        let angle_to_light = self
            .direction_to_light(intersection)
            .angle(&-self.direction);
        let source_illumination =
            1.0 / (self.distance_to_light(intersection).powf(2.0));
        let angle = intersection
            .surface_normal
            .dot(&self.direction_to_light(intersection).normalized())
            .max(0.0);
        let output =
            self.color * material.diffuse * angle * source_illumination;
        if angle_to_light > self.angle2 {
            Pixel::from_rgb(0.0, 0.0, 0.0)
        } else if angle_to_light < self.angle2 && angle_to_light > self.angle1 {
            let percentage =
                (angle_to_light - self.angle1) / (self.angle2 - self.angle1);
            output.lerp(&Pixel::from_rgb(0.0, 0.0, 0.0), percentage)
        } else {
            output
        }
    }

    fn specular(
        &self,
        camera: &Camera,
        intersection: &Intersection,
        material: &Material,
    ) -> Pixel {
        let angle_to_light = self
            .direction_to_light(intersection)
            .angle(&-self.direction);
        let view = (camera.position - intersection.point).normalized();
        let reflection = self
            .direction_to_light(intersection)
            .normalized()
            .reflect(&intersection.surface_normal);
        let phong_dot =
            view.dot(&reflection).min(0.0).powf(material.phong_power);
        let output = self.color.clamp() * material.specular * phong_dot;
        if angle_to_light > self.angle2 {
            Pixel::from_rgb(0.0, 0.0, 0.0)
        } else if angle_to_light < self.angle2 && angle_to_light > self.angle1 {
            let percentage =
                (angle_to_light - self.angle1) / (self.angle2 - self.angle1);
            output.lerp(&Pixel::from_rgb(0.0, 0.0, 0.0), percentage)
        } else {
            output
        }
    }
}
