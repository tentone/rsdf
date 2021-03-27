extern crate glium;
extern crate glutin;

use std::time::Instant;

use glium::{Display, Frame};
use glutin::window::WindowBuilder;
use glutin::event_loop::EventLoop;
use glutin::{ContextBuilder, NotCurrent};
use glutin::event::Event;
use glium::Surface;
use glium::implement_vertex;
use glium::uniform;

// Structure to represent a vertex
#[derive(Copy, Clone)]
struct Vertex {
    // Position of the vertex in x, y
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

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
        Vertex{position: [-1.0, -1.0]},
        Vertex{position: [1.0,  1.0]},
        Vertex{position: [1.0, -1.0]},
        Vertex{position: [-1.0, 1.0]},
        Vertex{position: [1.0,  1.0]},
        Vertex{position: [-1.0, -1.0]}
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &quad).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Shader program to render the scene.
    let program = glium::Program::from_source(&display, include_str!("vertex.glsl"), include_str!("fragment.glsl"), None).unwrap();

    // Frame time in milliseconds
    let start: Instant = std::time::Instant::now();

    // Window resolution
    let resolution: [f32;  2] = [1024.0, 768.0];

    let mut eye: [f32; 3] = [8.0, 5.0, 7.0];

    // Time of the last frame.
    let mut last_time: f32 = 0.0;

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let time: f32 = start.elapsed().as_secs_f32();
        let delta: f32 = time - last_time;
        last_time = time;

        let uniforms = &uniform! {
            resolution: resolution,
            time: time,
            eye: eye,
        };

        let mut frame: Frame = display.draw();
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