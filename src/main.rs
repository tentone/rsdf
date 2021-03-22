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

        uniform float time;
        uniform vec3 resolution;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader: &str = r#"
        #version 140

        out vec4 color;

        uniform float time;
        uniform vec3 resolution;

        const int MAX_MARCHING_STEPS = 255;
        const float MIN_DIST = 0.0;
        const float MAX_DIST = 100.0;
        const float EPSILON = 0.0001;

        /**
         * Signed distance function for a sphere centered at the origin with radius 1.0;
         */
        float sphereSDF(vec3 samplePoint) {
            return length(samplePoint) - 1.0;
        }

        /**
         * Signed distance function describing the scene.
         *
         * Absolute value of the return value indicates the distance to the surface.
         * Sign indicates whether the point is inside or outside the surface,
         * negative indicating inside.
         */
        float sceneSDF(vec3 samplePoint) {
            return sphereSDF(samplePoint);
        }

        /**
         * Return the shortest distance from the eyepoint to the scene surface along
         * the marching direction. If no part of the surface is found between start and end,
         * return end.
         *
         * eye: the eye point, acting as the origin of the ray
         * marchingDirection: the normalized direction to march in
         * start: the starting distance away from the eye
         * end: the max distance away from the ey to march before giving up
         */
        float shortestDistanceToSurface(vec3 eye, vec3 marchingDirection, float start, float end) {
            float depth = start;
            
            for (int i = 0; i < MAX_MARCHING_STEPS; i++) {
                float dist = sceneSDF(eye + depth * marchingDirection);
                if (dist < EPSILON) {
                    return depth;
                }
                
                depth += dist;
                
                if (depth >= end) {
                    return end;
                }
            }
            return end;
        }

        /**
         * Return the normalized direction to march in from the eye point for a single pixel.
         *
         * fieldOfView: vertical field of view in degrees
         * size: resolution of the output image
         * fragCoord: the x,y coordinate of the pixel in the output image
         */
        vec3 rayDirection(float fieldOfView, vec2 size, vec2 fragCoord) {
            vec2 xy = fragCoord - size / 2.0;
            float z = size.y / tan(radians(fieldOfView) / 2.0);
            return normalize(vec3(xy, -z));
        }

        void main() {
            float t = time / 10.0;
            vec2 fragCoord = gl_FragCoord.xy;

            vec3 dir = rayDirection(60.0, resolution.xy, fragCoord);
            vec3 eye = vec3(0.0, 0.0, 5.0);
            float dist = 0.0; // shortestDistanceToSurface(eye, dir, MIN_DIST, MAX_DIST);

            if (dist > MAX_DIST - EPSILON) {
                // Didn't hit anything
                color = vec4(0.0, 0.0, 0.0, 0.0);
                return;
            }

            color = vec4(sin(t), cos(t), 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    let mut time: f32 = 0.0;
    
    
    let frame: u64 = 16;

    event_loop.run(move |event: Event<()>, _, control_flow| {
        let mut target = display.draw();

        time += (frame as f32) / 1e3;

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = &uniform! {
            time: time,
            resolution: [800, 600, 0]
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