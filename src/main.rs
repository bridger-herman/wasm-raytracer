#[macro_use]
extern crate clap;

pub mod camera;
pub mod pixel;
pub mod scene;

// TODO remove this
pub mod vector;

fn main() {
    let matches = clap_app!(raytracer =>
        (version: crate_version!())
        (author: crate_authors!())
        (@arg scene_file: +required)
    ).get_matches();

    let scene_file_path = matches
        .value_of("scene_file")
        .expect("Scene file not found");

    let scene = scene::Scene::from_file(scene_file_path);
}
