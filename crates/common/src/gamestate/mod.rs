pub mod game;
pub mod physics;
pub mod snake;

use std::time::{Duration, Instant};

use crossterm::event::{poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::stream::InitOptions;

pub fn run(options: InitOptions) -> std::io::Result<()> {
    enable_raw_mode()?; // https://docs.rs/crossterm/0.27.0/crossterm/terminal/index.html#raw-mode
    println!("{}\r", serde_json::to_string(&options).unwrap());
    let mut main = game::Game::new(
        options.size.width,
        options.size.height,
        options.frame_duration as f64,
    );
    let mut last_loop_duration: Duration = Duration::new(0, 0);
    main.start();
    loop {
        let start = Instant::now();
        if poll(Duration::from_millis(20))? {
            let event = read()?;

            if let None = main.key_down(event) {
                disable_raw_mode()?;
                return Ok(());
            }
        }
        if main.update(last_loop_duration.as_millis() as f64) {
            println!("{}\r", serde_json::to_string(&main).unwrap());
        }
        last_loop_duration = start.elapsed();
    }
}
