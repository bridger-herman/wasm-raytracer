extern crate base64;
extern crate png;
extern crate wasm_bindgen;
#[macro_use]
extern crate log;
extern crate wasm_logger;

pub mod camera;
pub mod image;
pub mod intersection;
pub mod lights;
pub mod material;
pub mod objects;
pub mod pixel;
pub mod ray;
pub mod ray_tracer;
pub mod scene;
pub mod vector;

use ray_tracer::RayTracer;
use scene::Scene;

use wasm_bindgen::prelude::*;

/// Library for testing wasm code
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init_with_level(log::Level::Info)
        .map_err(|_| JsValue::from("Failed to initialize logger"))?;
    Ok(())
}

#[wasm_bindgen]
pub fn render_scene(scene_contents: String) -> String {
    let scene = Scene::from_text(scene_contents);

    let rt = RayTracer;
    let image = rt.render(&scene);

    let bytes = image.to_png_bytes();
    base64::encode(&bytes)
}
