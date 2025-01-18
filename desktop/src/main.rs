use chip8_core::*;
use std::env;
use std::fs::File;
use std::io::Read;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32)* SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run /path/to/game");
        return;
    }

    // setting up SDL window
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys
        .window("CHIP-8 EMULATOR", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();

    // setting up chip8 core backend
    let mut chip8 = Emu::new();
    // load rom
    let mut rom = File::open(&args[1]).expect("Failed to load file");
    let mut buff = Vec::new();
    // load rom into buffer
    rom.read_to_end(&mut buff).unwrap();
    chip8.load(&buff);

    // setting up events
    let mut event_pump = sdl_context.event_pump().unwrap();

    // labeled loop for the emulator
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }
        // clock cycle
        chip8.tick();
        draw_screen(&chip8, &mut canvas);
    }
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    // clear canvas --> set to black by default
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buff = emu.get_display();
    // set draw color to white, draw pixel with white if the display pixel boolean is true
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buff.iter().enumerate() {
        if *pixel {
            // convert 1D screen buffer to 2D (x, y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            // draw a rectangle at (x,y) scaled up
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
