//! A simple description of a scene used for ray tracing

use std::fs::File;
use std::io::prelude::*;

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
        let mut scene_file =
            File::open(scene_file).expect("Unable to open scene file");
        let mut scene_contents = String::new();
        scene_file
            .read_to_string(&mut scene_contents)
            .expect("Unable to read scene file");
        let scene_lines: Vec<_> = scene_contents
            .lines()
            .filter(|&line| !line.starts_with('#') && !line.is_empty())
            .collect();
        let tokens_per_line: Vec<Vec<_>> = scene_lines
            .iter()
            .map(|&line| line.split_whitespace().collect())
            .collect();

        let mut scene = Self::default();

        for line in &tokens_per_line {
            if line.is_empty() {
                continue;
            }
            match line[0] {
                "camera" => {
                    assert_eq!(line.len(), 11);
                    let float_tokens = parse_full_slice(&line[1..]);
                    scene.camera = Camera::from_parameters(&float_tokens)
                }
                "output_image" => {
                    assert_eq!(line.len(), 2);
                    scene.output_image = line[1].to_string();
                }
                "background" => {
                    assert_eq!(line.len(), 4);
                    let float_tokens = parse_full_slice(&line[1..]);
                    scene.background = Pixel::from(float_tokens.as_slice());
                }
                _ => (),
            }
        }

        println!("Loaded scene:\n{:#?}", scene);
        scene
    }
}

fn parse_full_slice(str_slice: &[&str]) -> Vec<f64> {
    str_slice
        .iter()
        .map(|&s| s.parse::<f64>().unwrap_or(0.0))
        .collect()
}
