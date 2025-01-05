use chip8_core::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run /path/to/game");
        return;
    }
}
