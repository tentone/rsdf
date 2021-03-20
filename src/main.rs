use sdl2::{VideoSubsystem, Sdl, EventPump};

extern crate sdl2;

fn main() {
    let sdl: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl.video().unwrap();

    let _window = video_subsystem
        .window("Game", 640, 480)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump: EventPump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,  _ => {}
            }
        }
    }
}
