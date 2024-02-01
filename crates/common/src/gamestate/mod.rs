pub mod game;
pub mod physics;
pub mod snake;

use std::time::{Duration, Instant};

use crossterm::event::{poll, read};

use crate::gamestate::game::GameState;
use crate::stream::InitOptions;

/**
 * This function is the update loop.
 * It keeps track of the user inputs via the keyboard.
 * It runs forever and returns if ctrl+c is hit.
 */
pub fn run(options: InitOptions) -> std::io::Result<()> {
    println!("{}\r", serde_json::to_string(&options).unwrap());
    let mut main = game::Game::new(
        options.size.width,
        options.size.height,
        options.frame_duration as f64,
        options.snake_length,
    );
    let mut last_loop_duration: Duration = Duration::new(0, 0);
    main.start();
    let mut prev_state = main.state.clone();
    loop {
        let start = Instant::now();
        if poll(Duration::from_millis(20))? {
            let event = read()?;

            // return Ok(()) when ctrl+c is hit
            if let None = main.key_down(event) {
                return Ok(());
            }
        }
        if main.update(last_loop_duration.as_millis() as f64) {
            if main.state == GameState::Running
                || main.state == GameState::Over
                || main.state == GameState::Paused && prev_state == GameState::Running
            {
                println!("{}\r", serde_json::to_string(&main).unwrap());
            }
            prev_state = main.state.clone();
        }
        last_loop_duration = start.elapsed();
    }
}
