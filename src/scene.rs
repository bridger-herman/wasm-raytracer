//! A simple description of a scene used for ray tracing

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use camera::Camera;
use material::Material;
use pixel::Pixel;
use sphere::Sphere;
use vector::Vector3;

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

    pub spheres: Vec<Sphere>,
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
            spheres: Vec::new(),
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
        let mut current_material = Material::default();

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
                "film_resolution" | "resolution" => {
                    assert_eq!(line.len(), 3);
                    let width_height = parse_full_slice(&line[1..]);
                    scene.resolution = (width_height[0], width_height[1]);
                }
                "sphere" => {
                    assert_eq!(line.len(), 5);
                    let float_tokens = parse_full_slice(&line[1..]);
                    let position = Vector3::from(&float_tokens[..3]);
                    let radius = float_tokens[3];
                    scene.spheres.push(Sphere::new(
                        radius,
                        position,
                        current_material.clone(),
                    ));
                }
                "material" => {
                    assert_eq!(line.len(), 15);
                    let float_tokens = parse_full_slice(&line[1..]);
                    let ambient = Pixel::from(&float_tokens[..3]);
                    let diffuse = Pixel::from(&float_tokens[3..6]);
                    let specular = Pixel::from(&float_tokens[6..9]);
                    let transmissive = Pixel::from(&float_tokens[10..13]);
                    current_material = Material::new(
                        ambient,
                        diffuse,
                        specular,
                        float_tokens[9],
                        transmissive,
                        float_tokens[13],
                    );
                }
                _ => (),
            }
        }

        println!("Loaded scene:\n{:#?}", scene);
        scene
    }
}

fn parse_full_slice<T: FromStr + Default>(str_slice: &[&str]) -> Vec<T> {
    str_slice
        .iter()
        .map(|&s| s.parse::<T>().unwrap_or_default())
        .collect()
}
