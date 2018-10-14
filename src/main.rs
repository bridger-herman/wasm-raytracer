extern crate image as ext_image;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate conrod;
#[cfg(test)]
extern crate glm;

pub mod camera;
pub mod image;
pub mod intersection;
pub mod lights;
pub mod material;
pub mod pixel;
pub mod ray;
pub mod ray_tracer;
pub mod scene;
pub mod sphere;

// TODO remove this
pub mod vector;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Colorable, Positionable, Widget};

const WINDOW_SIZE: (f64, f64) = (800.0, 800.0);
const TITLE: &str = "Ray Tracer";

fn main() {
    let matches = clap_app!(raytracer =>
        (version: crate_version!())
        (author: crate_authors!())
        (@arg scene_file: +required)
    ).get_matches();

    // Create the GUI window and main loop
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((WINDOW_SIZE.0, WINDOW_SIZE.1).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Create the UI iteself
    let mut ui = conrod::UiBuilder::new([WINDOW_SIZE.0, WINDOW_SIZE.1]).build();
    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

    // Add the font
    ui.fonts
        .insert_from_file("./fonts/NotoSans-Regular.ttf")
        .expect("Unable to load font");

    // Map a texture
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Create the UI for the renderer
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // Main loop
    'main: loop {
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        for event in events {
            if let glium::glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode:
                                    Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                }
            }
        }

        let ui = &mut ui.set_widgets();

        // "Hello World!" in the middle of the screen.
        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(ids.text, ui);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

    // let scene_file_path = matches
    // .value_of("scene_file")
    // .expect("Scene file not found");

    // let scene = scene::Scene::from_file(scene_file_path);

    // let rt = ray_tracer::RayTracer;
    // let rendered = rt.render(&scene);
    // rendered
    // .write(&scene.output_image)
    // .expect("Unable to write image");
}
