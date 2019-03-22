extern crate wasm_bindgen;
#[macro_use]
extern crate log;
extern crate wasm_logger;

use wasm_bindgen::prelude::*;

/// Library for testing wasm code
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init_with_level(log::Level::Trace)
        .map_err(|_| JsValue::from("Failed to initialize logger"))?;

    info!("Hello from Rust!");
    Ok(())
}
