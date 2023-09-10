pub mod game;
pub mod physics;
pub mod snake;

// use std::io::Write;

use std::time::{Duration, Instant};

use crossterm::event::{poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

pub fn run() -> std::io::Result<()> {
    enable_raw_mode()?; // https://docs.rs/crossterm/0.27.0/crossterm/terminal/index.html#raw-mode
    println!("RUN\r");
    let mut main = game::Game::new(WIDTH, HEIGHT);
    let mut last_loop_duration: Duration = Duration::new(0, 0);
    main.start();
    loop {
        let start = Instant::now();
        if poll(Duration::from_millis(20))? {
            let event = read()?;

            if let None = main.key_down(event) {
                disable_raw_mode()?;
                println!("Bye!\n");
                return Ok(());
            }
        }
        if main.update(last_loop_duration.as_millis() as f64) {
            println!("{:?}\r", main);
        }
        last_loop_duration = start.elapsed();
    }
}
