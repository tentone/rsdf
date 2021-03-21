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

    let vertex_shader: &str = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader: &str = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        let next_frame: Instant = std::time::Instant::now() + std::time::Duration::from_millis(16);
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