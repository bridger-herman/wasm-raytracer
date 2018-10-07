//! The main ray tracing implementation

use image::Image;
use pixel::Pixel;
use ray::Ray;
use scene::Scene;

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene) -> Result<(), String> {
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

                for sphere in &scene.spheres {
                    if sphere.intersects(&ray) {
                        img.set_pixel(row, col, Pixel::from_rgb(1.0, 1.0, 1.0));
                    }
                }
                // TODO: Remove (only for Mathematica visualization)
                // println!(
                // "{{{},{},{}}},",
                // ray_direction.x, ray_direction.y, ray_direction.z
                // );
            }
        }

        img.write(&scene.output_image)?;
        Ok(())
    }
}
