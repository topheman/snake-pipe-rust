pub mod physics;
pub mod snake;
pub mod game;

// use std::io::Write;

use crossterm::event::{read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

pub fn run() -> std::io::Result<()> {
  enable_raw_mode()?; // https://docs.rs/crossterm/0.27.0/crossterm/terminal/index.html#raw-mode
  println!("RUN\r");
  let mut main = game::Game::new(WIDTH, HEIGHT);
  main.start();
  loop {
    let event = read()?;

    if let None = main.key_down(event) {
      disable_raw_mode()?;
      println!("Bye!\n");
      return Ok(());
    }
  }
}
