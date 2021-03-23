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
    let window: WindowBuilder = glutin::window::WindowBuilder::new();

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
    let resolution: [f32;  2] = [800.0, 600.0];

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let time = start.elapsed().as_secs_f32();
        // let res = display.gl_window().get_inner_size_pixels();
        // resolution = [res[0], res[1]];

        let uniforms = &uniform! {
            resolution: resolution,
            time: time,
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
            _ => (),
        }
    });
}