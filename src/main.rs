use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_HEIGHT: i32 = 480;
const SCREEN_WIDTH: i32 = 640;
const MAP_HEIGHT: usize = 24;
const MAP_WIDTH: usize = 24;

// https://www.reddit.com/r/rust/comments/icpdvh/rust_matrix_structure/
pub struct Vec2 {
    x: f32,
    y: f32
}

pub struct Matrix {
    grid: [u32; MAP_WIDTH*MAP_HEIGHT]
}

const WORLD_MAP: Matrix = Matrix {
    grid:   
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1,
    1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1,
    1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
};

// bad index function arughh this code is duct tape  and paper

fn main() {
    // remeber https://docs.rs/sdl2/latest/sdl2
    // notes for future me; figure out what .unwrap(); does
    let sdl_context = sdl2::init().unwrap(); 
    let video_subsystem = sdl_context.video().unwrap();

    // i think this is the window we will be outputing to
    let window = video_subsystem.window("wolfenstein-style-renderer",
                                                SCREEN_WIDTH.try_into().unwrap(), 
                                                SCREEN_HEIGHT.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    // and this is the canvas we will be outputting to
    let mut canvas = window.into_canvas().build().unwrap();

    // game loop (i suspect we will do most of the code here)
    let mut i = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    // game loop varibles
    let mut pos: Vec2 = Vec2 {x: 22., y: 12.}; // player posit<i32 as TryInto<u8>>::try_into(m_index(map_x, map_y)).unwrap()ion
    let mut dir: Vec2 = Vec2 {x: -1., y: 0.}; // player direction
    let mut plane: Vec2 = Vec2 {x: 0., y: 0.66}; // pos camera plane

    let mut time: f32 = 0.; // time of current frame
    let mut old_time: f32 = 0.; // time of previous frame


    'main: loop {
        i = (i +1) % 255;

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
        // raycasting code
        let mut x: i32 = 0;
        while x < SCREEN_WIDTH {
            //let w: f32 = SCREEN_WIDTH as f32;
            let camera_x: f32 = (2 * x as i32 / SCREEN_WIDTH - 1) as f32;
            let ray_dir = Vec2{x: dir.x + plane.x * camera_x,
                                     y: dir.y + plane.y * camera_x};
            // the positon of us i supppose
            let mut map_x: i32 = pos.x as i32;
            let mut map_y: i32 = pos.y as i32;
            
            // the length of ray from current positon to next x or y-side
            let mut side_dist: Vec2;
            
            // urm idk goto line 98 in raycaster_flat.cpp
            let delta_dist: Vec2 = Vec2 {
                x: ((ray_dir.x * ray_dir.x) + 1.).sqrt() / (ray_dir.x * ray_dir.x),
                y: ((ray_dir.y * ray_dir.y) + 1.).sqrt() / (ray_dir.y * ray_dir.y)
            };
            
            let perp_wall_dist: f32;
            
            // step direction
            let step_x: i32;
            let step_y: i32;
            
            let mut hit: i32 = 0;
            let mut side: i32 = -1; // rust was being meanie weanie 
            // calculate step and initial side Dist note that since we calate x first we set y as 0
            if ray_dir.x < 0. {
                step_x = -1;
                side_dist = Vec2 {x: (pos.x - (map_x as f32)) * delta_dist.x, y: 0.};
            } else {
                step_x = 1;
                side_dist = Vec2 {x: ((map_x as f32) + 1. - pos.x) * delta_dist.x, y: 0.}
            }
            if ray_dir.y < 0. {
                step_y = -1;
                side_dist.y = (pos.y - map_y as f32) * delta_dist.y
            } else {
                step_y = 1;
                side_dist.y = (map_y as f32 + 1. - pos.y) * delta_dist.y
            }

            // DDA !!!!
            while hit == 0 
            {
                if side_dist.x < side_dist.y
                {
                    side_dist.x += delta_dist.x;
                    map_x += step_x;
                    side = 0;
                } 
                else 
                {
                    side_dist.y += delta_dist.y;
                    map_y += step_y;
                    side = 1;
                }

                // check if ray hits wall !!!    
                let index: usize = ((map_y * MAP_WIDTH as i32) + map_x) as usize;
                if WORLD_MAP.grid[index] > 0 {
                    hit = 1;
                }
            }
            
            //urmm idk how to explain this look at the 11th image at 
            // https://lodev.org/cgtutor/raycasting.html#Introduction
            if side == 0 {
                perp_wall_dist = side_dist.x - delta_dist.x;
            } else {
                perp_wall_dist = side_dist.y - delta_dist.y;
            }

            // line height
            let line_height: i32 = SCREEN_HEIGHT / perp_wall_dist as i32;
            
            //lowest and heighest pixel
            let mut draw_start: i32 = -line_height / 2 + SCREEN_HEIGHT / 2;
            if draw_start < 0 {draw_start = 0;}
            let mut draw_end: i32 = line_height / 2 + SCREEN_HEIGHT / 2;
            if draw_end >= SCREEN_HEIGHT { draw_end = SCREEN_HEIGHT - 1;}

            let color: Color;
            let index: usize = ((map_y * MAP_WIDTH as i32) + map_x) as usize;
            match WORLD_MAP.grid[index] {
                1 => color = Color::RED,
                2 => color = Color::GREEN,
                3 => color = Color::CYAN,
                4 => color = Color::WHITE,
                _ => color = Color::YELLOW
            }

            canvas.set_draw_color(color);
            let _ = canvas.draw_line((x, draw_start), (x, draw_end));
            x += 1;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
