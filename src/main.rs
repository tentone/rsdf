extern crate glium;
extern crate glutin;
extern crate image;

use std::time::Instant;
use std::io::{Cursor, Read, Error};

use glium::{Display, Frame};
use glutin::window::WindowBuilder;
use glutin::event_loop::EventLoop;
use glutin::{ContextBuilder, NotCurrent};
use glutin::event::Event;
use glium::Surface;
use glium::implement_vertex;
use glium::uniform;
use image::RgbaImage;
use std::fs::File;
use std::path::Path;

// Structure to represent a vertex
#[derive(Copy, Clone)]
struct Vertex {
    // Position of the vertex in x, y
    position: [f32; 2],

    // UV mapping for the position
    uv: [f32; 2],
}

implement_vertex!(Vertex, position, uv);

fn main() {
    let window: WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("RSDF")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    let context: ContextBuilder<NotCurrent> = glutin::ContextBuilder::new();

    let event_loop: EventLoop<()> = glutin::event_loop::EventLoop::new();

    let display: Display = glium::Display::new(window, context, &event_loop).unwrap();

    // Full screen quad
    let quad: Vec<Vertex> = vec![
        Vertex{position: [-1.0, -1.0], uv: [0.0, 0.0]},
        Vertex{position: [1.0,  1.0], uv: [1.0, 1.0]},
        Vertex{position: [1.0, -1.0], uv: [1.0, 0.0]},
        Vertex{position: [-1.0, 1.0], uv: [0.0, 1.0]},
        Vertex{position: [1.0,  1.0], uv: [1.0, 1.0]},
        Vertex{position: [-1.0, -1.0], uv: [0.0, 0.0]}
    ];

    // Create a path to the desired file
    let path = Path::new("./textures/noise.png");
    let display = path.display();

    // Read texture file content
    // let mut file = match File::open(&path) {
    //     Ok(file) => file,
    //     Err(why) => panic!("Could not read file {}", why),
    // };
    // let mut texture_data: Vec<u8> = Vec::new();
    // file.read_to_end(&mut texture_data);

    // Create noise texture from data
    /*let image: RgbaImage = image::load(Cursor::new(texture_data), image::ImageFormat::Png).unwrap().to_rgba8();
    let dimension = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimension);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();*/

    let image: RgbaImage = image::load(Cursor::new(&include_bytes!("./textures/noise.png")[..]), image::ImageFormat::Png).unwrap().to_rgba8();
    let dimension = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimension);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    /* sampled().
    magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).
    unwrap();*/

    // Create quad vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &quad).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Shader program to render the scene.
    let program = glium::Program::from_source(&display, include_str!("vertex.glsl"), include_str!("fragment.glsl"), None).unwrap();

    // Frame time in milliseconds
    let start: Instant = std::time::Instant::now();

    // Window resolution
    let mut resolution: [f32;  2] = [1024.0, 768.0];

    let mut eye: [f32; 3] = [8.0, 5.0, 7.0];

    // Time of the last frame.
    let mut last_time: f32 = 0.0;

    // Perspective matrix
    /* let perspective: [[f32; 4]; 4] = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio: f32 = height as f32 / width as f32;

        let fov: f32 = (std::f64::consts::PI / 3.0) as f32;
        let zfar: f32 = 1024.0;
        let znear: f32 = 0.1;
        let f: f32 = 1.0 / (fov / 2.0).tan();

        // Composed perspective matrix
        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
            [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
        ]
    };*/

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let time: f32 = start.elapsed().as_secs_f32();
        let delta: f32 = time - last_time;
        last_time = time;


        let mut frame: Frame = display.draw();
        let (w, h) = frame.get_dimensions();

        resolution[0] = w as f32;
        resolution[1] = h as f32;

        let uniforms = &uniform! {
            resolution: resolution,
            time: time,
            eye: eye,
            tex: &texture
        };

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();

        // Event handler
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::DeviceEvent { event, device_id: _ } => match event {
                glutin::event::DeviceEvent::Key(kin) => {
                    let speed: f32 = 750.0 * delta;

                    // W
                    if kin.scancode == 17 { eye[1] += speed; }
                    // S
                    if kin.scancode == 31 { eye[1] -= speed; }
                    // A
                    if kin.scancode == 30 { eye[0] -= speed; }
                    // D
                    if kin.scancode == 32 { eye[0] += speed; }

                    // println!("DeviceEvent Key: {:?}", kin);
                },
                _ => {},
            },
            _ => (),
        }
    });
}