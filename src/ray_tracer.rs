//! The main ray tracing implementation

use image::Image;
use scene::Scene;
use ray::Ray;

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene) -> Result<(), String> {
        let img = Image::new(scene.resolution.0, scene.resolution.1)
            .with_background(scene.background);

        let horiz_half_angle = scene.resolution.0 as f64
            * scene.camera.vert_half_angle
            / scene.resolution.1 as f64;
        let viewport_width = 2.0 * horiz_half_angle.tan();
        let viewport_height = 2.0 * scene.camera.vert_half_angle.tan();
        let pixel_width = viewport_width / scene.resolution.1 as f64;
        let pixel_height = viewport_height / scene.resolution.0 as f64;

        // The upper-left-most pixel
        // 0.5s are to center the rays on each pixel
        let upper_left = scene.camera.position + scene.camera.direction
            + scene.camera.up
                * pixel_height
                * (scene.resolution.0 as f64 / 2.0 - 0.5)
            - scene.camera.right
                * pixel_width
                * (scene.resolution.1 as f64 / 2.0 - 0.5);

        for row in 0..scene.resolution.1 {
            for col in 0..scene.resolution.0 {
                let image_plane_location = upper_left
                    - scene.camera.up * pixel_height * row as f64
                    + scene.camera.right * pixel_width * col as f64;
                let ray_direction = image_plane_location - scene.camera.position;
                let ray = Ray::new(scene.camera.position, ray_direction);
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
