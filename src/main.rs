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
use conrod::{
    color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
};

const TITLE: &str = "Ray Tracer";

fn main() {
    let matches = clap_app!(raytracer =>
        (version: crate_version!())
        (author: crate_authors!())
        (@arg scene_file: +required)
    ).get_matches();

    let mut scene_file_path = matches
        .value_of("scene_file")
        .expect("Scene file not found")
        .to_string();

    let scene = scene::Scene::from_file(scene_file_path.as_str());

    let rt = ray_tracer::RayTracer;
    let mut rendered = rt.render(&scene);
    let window_size = (rendered.width as f64 + 200.0, rendered.height as f64);

    // Create the GUI window and main loop
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((window_size.0, window_size.1).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Create the UI iteself
    let mut ui = conrod::UiBuilder::new([window_size.0, window_size.1]).build();
    widget_ids!(struct Ids { rendered_img, render_button, scene_file });
    let ids = Ids::new(ui.widget_id_generator());

    // Add the font
    ui.fonts
        .insert_from_file("./fonts/NotoSans-Regular.ttf")
        .expect("Unable to load font");

    // Map a texture
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Insert an image into the map
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &rendered.to_bytes(),
        (rendered.width as u32, rendered.height as u32),
    );
    let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
    let rendered_img = image_map.insert(texture);

    // Create the UI for the renderer
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // Main loop
    'main: loop {
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        for event in events {
            // Use the `winit` backend feature to convert the winit event
            // to a conrod one.
            if let Some(event) =
                conrod::backend::winit::convert_event(event.clone(), &display)
            {
                ui.handle_event(event);
            }

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

        widget::Image::new(rendered_img)
            .w_h(rendered.width as f64, rendered.height as f64)
            .top_left()
            .set(ids.rendered_img, ui);

        if widget::Button::new()
            .hover_color(color::GREEN)
            .press_color(color::BLUE)
            .label("Render Image")
            .w_h(200.0, 50.0)
            .bottom_right()
            .set(ids.render_button, ui)
            .was_clicked()
        {
            rendered = render_img(&rt, scene_file_path.as_str());
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &rendered.to_bytes(),
                (rendered.width as u32, rendered.height as u32),
            );
            let texture =
                glium::texture::Texture2d::new(&display, raw_image).unwrap();
            image_map
                .replace(rendered_img, texture)
                .expect("Couldn't get image");
        }

        if let Some(edit) = widget::TextEdit::new(scene_file_path.as_str())
            .color(color::WHITE)
            .mid_top_with_margin_on(ids.render_button, -30.0)
            .center_justify()
            .set(ids.scene_file, ui)
        {
            scene_file_path = edit;
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn render_img(
    ray_tracer: &ray_tracer::RayTracer,
    scene_file_path: &str,
) -> image::Image {
    let scene = scene::Scene::from_file(scene_file_path);
    ray_tracer.render(&scene)
}
