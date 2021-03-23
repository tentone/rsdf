extern crate glium;
extern crate glutin;

use glium::Display;
use glutin::window::WindowBuilder;
use glutin::event_loop::EventLoop;
use glutin::{ContextBuilder, NotCurrent};
use std::time::Instant;
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
    let frame: u64 = 16;

    // Time counter
    let mut time: f32 = 0.0;

    // Window resolution
    let resolution: [f32;  2] = [800.0, 600.0];

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let mut target = display.draw();

        time += (frame as f32) / 1e3;

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = &uniform! {
            resolution: resolution,
            time: time,
        };

        target.draw(&vertex_buffer, &indices, &program, uniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        let next_frame: Instant = std::time::Instant::now() + std::time::Duration::from_millis(frame);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame);

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