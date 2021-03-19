use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;
extern crate sdl2;

fn main() {
    // let mut guess: String = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    //
    // let mut rng: ThreadRng = rand::thread_rng();
    // let secret: i32 = rng.gen_range(1..10);

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let _window = video_subsystem
        .window("Game", 640, 480)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,  _ => {}
            }
        }
    }
}
