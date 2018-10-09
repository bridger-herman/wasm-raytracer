//! The main ray tracing implementation

use image::Image;
use intersection::Intersection;
use pixel::Pixel;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vector::MAX_VECTOR3;

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene) -> Image {
        let mut img = Image::new(scene.resolution.0, scene.resolution.1)
            .with_background(scene.background);

        let viewport_height = 2.0 * scene.camera.vert_half_angle.tan();
        let viewport_width = viewport_height
            * (scene.resolution.0 as f64 / scene.resolution.1 as f64);
        let pixel_width = viewport_width / scene.resolution.0 as f64;
        let pixel_height = viewport_height / scene.resolution.1 as f64;

        // The upper-left-most pixel
        // 0.5s are to center the rays on each pixel
        let upper_left = scene.camera.position + scene.camera.direction
            + scene.camera.up
                * pixel_height
                * (scene.resolution.1 as f64 / 2.0 - 0.5)
            - scene.camera.right
                * pixel_width
                * (scene.resolution.0 as f64 / 2.0 - 0.5);

        for row in 0..scene.resolution.1 {
            for col in 0..scene.resolution.0 {
                // Compute the ray shooting from the eye
                let image_plane_location = upper_left
                    - scene.camera.up * pixel_height * row as f64
                    + scene.camera.right * pixel_width * col as f64;
                let ray_direction =
                    image_plane_location - scene.camera.position;
                let ray = Ray::new(scene.camera.position, ray_direction);
                let mut closest_intersection =
                    Intersection::new(MAX_VECTOR3, MAX_VECTOR3);

                for sphere in &scene.spheres {
                    if let Some(intersection) = sphere.intersects(&ray) {
                        let distance =
                            (intersection.point - ray.start).length();
                        if distance
                            < (closest_intersection.point - ray.start).length()
                        {
                            img.set_pixel(
                                row,
                                col,
                                self.calculate_illumination(
                                    scene,
                                    &sphere,
                                    &intersection,
                                ),
                            );
                            closest_intersection = intersection;
                        }
                    }
                }
            }
        }

        img
    }

    fn calculate_illumination(
        &self,
        scene: &Scene,
        sphere: &Sphere,
        intersection: &Intersection,
    ) -> Pixel {
        let mut sum = Pixel::from_rgba_unclamped(0.0, 0.0, 0.0, 0.0);
        sum = sum + sphere.material.ambient * scene.ambient_light;
        for point_light in &scene.point_lights {
            // Calculate diffuse lighting
            let to_light = point_light.position - intersection.point;
            let source_illumination = 1.0 / (to_light.length().powf(2.0));
            let angle = intersection
                .surface_normal
                .dot(&to_light.normalized())
                .max(0.0);
            let unclamped_color = Pixel::from_pix_unclamped(point_light.color);
            sum = sum + unclamped_color
                * point_light.power
                * sphere.material.diffuse
                * angle
                * source_illumination;
        }

        sum
    }
}
