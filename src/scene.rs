//! A simple description of a scene used for ray tracing

use camera::Camera;
use pixel::Pixel;

#[derive(Debug)]
pub struct Scene {
    /// The scene's camera
    pub camera: Camera,

    /// The resolution of the output image
    pub resolution: (usize, usize),

    /// The file path for the output image
    pub output_image: String,

    /// The background color
    pub background: Pixel,
    // TODO material
    // TODO sphere
    // TODO lights
    // TODO max_depth
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            resolution: (640, 480),
            output_image: String::from("./raytraced.bmp"),
            background: Pixel::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

impl Scene {
    pub fn from_file(scene_file: &str) -> Self {
        let scene = Self::default();
        println!("Loaded scene:\n{:#?}", scene);
        scene
    }
}
