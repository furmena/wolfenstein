use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_HEIGHT: u32 = 480;
const SCREEN_WIDTH: u32 = 640;
const MAP_HEIGHT: u32 = 24;
const MAP_WIDTH: u32 = 24;

fn main() {
    println!("Hello, world!");

    // remeber https://docs.rs/sdl2/latest/sdl2
    // notes for future me; figure out what .unwrap(); does
    let sdl_context = sdl2::init().unwrap(); 
    let video_subsystem = sdl_context.video().unwrap();

    // i think this is the window we will be outputing to
    let window = video_subsystem.window("wolfenstein-style-renderer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // and this is the canvas we will be outputting to
    let mut canvas = window.into_canvas().build().unwrap();

    // draw 0, 255, 255 for 1 frame 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // game loop (i suspect we will do most of the code here)
    let mut i = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        i = (i +1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main         
                },
                _ => {}
            }
        }
        // where most of the code will go \/\/\/

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
