extern crate image as ext_image;
#[macro_use]
extern crate clap;

#[cfg(test)]
extern crate glm;

pub mod camera;
pub mod image;
pub mod pixel;
pub mod ray;
pub mod ray_tracer;
pub mod scene;
pub mod sphere;

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

    let rt = ray_tracer::RayTracer;
    rt.render(&scene).expect("Unable to render scene");
}
