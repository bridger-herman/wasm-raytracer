//! A simple description of a scene used for ray tracing

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use camera::Camera;
use lights::directional_light::DirectionalLight;
use lights::light::Light;
use lights::point_light::PointLight;
use material::Material;
use objects::object::Object;
use objects::sphere::Sphere;
use objects::triangle::Triangle;
use pixel::Pixel;
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

    /// All the objects in the scene
    pub objects: Vec<Box<Object>>,

    /// Ambient lighting in a scene
    pub ambient_light: Pixel,

    /// All other lights in the scene
    pub lights: Vec<Box<Light>>,

    /// The max depth of a ray
    pub max_depth: usize,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            resolution: (640, 480),
            output_image: String::from("./raytraced.bmp"),
            background: Pixel::from_rgb(0.0, 0.0, 0.0),
            objects: Vec::new(),
            ambient_light: Pixel::from_rgb(0.0, 0.0, 0.0),
            lights: Vec::new(),
            max_depth: 5,
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
            .map(|line| {
                if let Some(index) = line.find('#') {
                    &line[..index]
                } else {
                    line
                }
            }).collect();
        let tokens_per_line: Vec<Vec<_>> = scene_lines
            .iter()
            .map(|&line| line.split_whitespace().collect())
            .collect();

        let mut scene = Self::default();
        let mut current_material = Material::default();

        let mut vertices = Vec::new();
        let mut vertices_so_far = 0;
        let mut max_vertices = None;

        let mut normals = Vec::new();
        let mut normals_so_far = 0;
        let mut max_normals = None;

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
                    scene.objects.push(Box::new(Sphere::new(
                        radius,
                        position,
                        current_material.clone(),
                    )));
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
                "ambient_light" => {
                    assert_eq!(line.len(), 4);
                    let float_tokens = parse_full_slice(&line[1..]);
                    scene.ambient_light = Pixel::from(float_tokens.as_slice());
                }
                "point_light" => {
                    assert_eq!(line.len(), 7);
                    let float_tokens = parse_full_slice(&line[1..]);
                    let color = Pixel::from_slice_unclamped(&float_tokens[..3]);
                    let position = Vector3::from(&float_tokens[3..]);
                    scene
                        .lights
                        .push(Box::new(PointLight::new(color, position)));
                }
                "directional_light" => {
                    assert_eq!(line.len(), 7);
                    let float_tokens = parse_full_slice(&line[1..]);
                    let color = Pixel::from_slice_unclamped(&float_tokens[..3]);
                    let direction = Vector3::from(&float_tokens[3..]);
                    scene.lights.push(Box::new(DirectionalLight::new(
                        color, direction,
                    )));
                }
                "max_depth" => {
                    assert_eq!(line.len(), 2);
                    scene.max_depth = line[1].parse::<usize>().unwrap_or(5);
                }
                "max_vertices" => {
                    assert_eq!(line.len(), 2);
                    max_vertices = Some(
                        line[1]
                            .parse::<usize>()
                            .expect("Max vertices must be an integer"),
                    );
                    vertices.resize(max_vertices.unwrap(), Vector3::default());
                }
                "max_normals" => {
                    assert_eq!(line.len(), 2);
                    max_normals = Some(
                        line[1]
                            .parse::<usize>()
                            .expect("Max normals must be an integer"),
                    );
                    normals.resize(max_normals.unwrap(), Vector3::default());
                }
                "vertex" => {
                    assert_eq!(line.len(), 4);
                    max_vertices.expect("Max vertices must be provided before specifying any vertices");
                    let float_tokens = parse_full_slice(&line[1..]);
                    vertices[vertices_so_far] =
                        Vector3::from(float_tokens.as_slice());
                    vertices_so_far += 1;
                }
                "normal" => {
                    assert_eq!(line.len(), 4);
                    max_normals.expect("Max normals must be provided before specifying any normals");
                    let float_tokens = parse_full_slice(&line[1..]);
                    normals[normals_so_far] =
                        Vector3::from(float_tokens.as_slice()).normalized();
                    normals_so_far += 1;
                }
                "triangle" => {
                    assert_eq!(line.len(), 4);
                    let indices: Vec<usize> = parse_full_slice(&line[1..]);
                    for t in &indices {
                        assert!(t < &vertices.len());
                    }
                    let (v1, v2, v3) = (
                        vertices[indices[0]],
                        vertices[indices[1]],
                        vertices[indices[2]],
                    );
                    scene.objects.push(Box::new(Triangle::guess_normal(
                        current_material.clone(),
                        v1,
                        v2,
                        v3,
                        &scene.camera,
                    )));
                }
                "normal_triangle" => {
                    assert_eq!(line.len(), 7);
                    let vert_indices: Vec<usize> =
                        parse_full_slice(&line[1..4]);
                    for t in &vert_indices {
                        assert!(t < &vertices.len());
                    }
                    let norm_indices: Vec<usize> = parse_full_slice(&line[4..]);
                    for t in &norm_indices {
                        assert!(t < &normals.len());
                    }
                    scene.objects.push(Box::new(Triangle::new(
                        current_material.clone(),
                        vertices[vert_indices[0]],
                        vertices[vert_indices[1]],
                        vertices[vert_indices[2]],
                        normals[norm_indices[0]],
                        normals[norm_indices[1]],
                        normals[norm_indices[2]],
                    )));
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
