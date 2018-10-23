//! The main ray tracing implementation

use image::Image;
use intersection::Intersection;
use objects::object::Object;
use pixel::Pixel;
use ray::Ray;
use scene::Scene;
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
        let upper_left = scene.camera.position
            + scene.camera.direction
            + scene.camera.up
                * pixel_height
                * (scene.resolution.1 as f64 / 2.0 - 0.5)
            - scene.camera.right
                * pixel_width
                * (scene.resolution.0 as f64 / 2.0 - 0.5);

        let mut done = 0;
        for row in 0..scene.resolution.1 {
            for col in 0..scene.resolution.0 {
                // Compute the ray shooting from the eye
                let image_plane_location = upper_left
                    - scene.camera.up * pixel_height * row as f64
                    + scene.camera.right * pixel_width * col as f64;
                let ray_direction =
                    image_plane_location - scene.camera.position;
                let ray = Ray::new(scene.camera.position, ray_direction);
                let color = self.trace_ray(scene, &ray, 0);
                img.set_pixel(row, col, color);

                done += 1;
                if done % 10000 == 0 {
                    println!(
                        "Ray casting: {}/{}",
                        done,
                        scene.resolution.0 * scene.resolution.1
                    );
                }
            }
        }

        img
    }

    fn trace_ray(&self, scene: &Scene, ray: &Ray, depth: usize) -> Pixel {
        let mut closest_intersection =
            Intersection::new(MAX_VECTOR3, MAX_VECTOR3);

        if depth > scene.max_depth {
            return Pixel::from_rgb(0.0, 0.0, 0.0);
        }

        let mut color = scene.background;

        for object in &scene.objects {
            if let Some(intersection) = object.intersects(&ray) {
                let distance = (intersection.point - ray.start).length();
                if distance < (closest_intersection.point - ray.start).length()
                {
                    color = self.calculate_illumination(
                        scene,
                        object,
                        &intersection,
                        ray,
                        depth,
                    );
                    closest_intersection = intersection;
                }
            }
        }
        color
    }

    fn calculate_illumination(
        &self,
        scene: &Scene,
        object: &Box<Object>,
        intersection: &Intersection,
        ray: &Ray,
        depth: usize,
    ) -> Pixel {
        // Start with ambient light
        let mut sum = Pixel::from_rgba_unclamped(0.0, 0.0, 0.0, 0.0);

        sum = sum + object.material().ambient * scene.ambient_light;

        for light in &scene.lights {
            let to_light = light.to_light(intersection);

            // Calculate shadows
            let in_shadow = scene.objects.iter().any(|object| {
                object
                    .intersects(&Ray::new(
                        intersection.point,
                        to_light.normalized(),
                    )).is_some()
            });

            if in_shadow {
                continue;
            }

            sum = sum + light.diffuse(&intersection, &object.material());

            sum = sum + light.specular(
                &scene.camera,
                intersection,
                &object.material(),
            );
        }

        let reflected =
            ray.reflect(intersection.point, intersection.surface_normal);
        sum = sum
            + object.material().specular
                * self.trace_ray(scene, &reflected, depth + 1);

        let refracted = ray.refract(
            intersection.point,
            intersection.surface_normal,
            object.material().ior,
        );
        sum = sum
            + object.material().transmissive
                * self.trace_ray(scene, &refracted, depth + 1);

        sum
    }
}
