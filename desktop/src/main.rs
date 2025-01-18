use chip8_core::*;
use std::env;
use std::fs::File;
use std::io::Read;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32)* SCALE;
const TICK_PER_FRAME: usize = 10;

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
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(k) = keymap(key) {
                        chip8.keypress(k, true);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(k) = keymap(key) {
                        chip8.keypress(k, false);
                    }
                },
                _ => ()
            }
        }
        // clock cycle --> for loop to enhance refresh rate
        for _ in 0..TICK_PER_FRAME {
            chip8.tick();
        }
        // add 1 to timer counter register
        chip8.tick_timers();
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

fn keymap(key: Keycode) -> Option<usize> {
    // 4x4 grid
    //
    // +----+----+----+----+
    // | 1  | 2  | 4  | C  |
    // +----+----+----+----+
    // | 4  | 5  | 6  | D  |
    // +----+----+----+----+   ==> CHIP8 Keys
    // | 7  | 8  | 9  | E  |
    // +----+----+----+----+
    // | A  | 0  | B  | F  |
    // +----+----+----+----+
    //
    // +----+----+----+----+
    // | 1  | 2  | 3  | 4  |
    // +----+----+----+----+
    // | Q  | W  | E  | R  |
    // +----+----+----+----+   ==> Keyboard Keys
    // | A  | S  | D  | F  |
    // +----+----+----+----+
    // | Z  | X  | C  | V  |
    // +----+----+----+----+
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q =>    Some(0x4),
        Keycode::W =>    Some(0x5),
        Keycode::E =>    Some(0x6),
        Keycode::R =>    Some(0xD),
        Keycode::A =>    Some(0x7),
        Keycode::S =>    Some(0x8),
        Keycode::D =>    Some(0x9),
        Keycode::F =>    Some(0xE),
        Keycode::Z =>    Some(0xA),
        Keycode::X =>    Some(0x0),
        Keycode::C =>    Some(0xB),
        Keycode::V =>    Some(0xF),
        _ => None,
    }
}
