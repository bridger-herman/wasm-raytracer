//! The main ray tracing implementation

use image::Image;
use scene::Scene;

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene) -> Result<(), String> {
        let img = Image::new(scene.resolution.0, scene.resolution.1)
            .with_background(scene.background);
        img.write(&scene.output_image)?;
        Ok(())
    }
}
