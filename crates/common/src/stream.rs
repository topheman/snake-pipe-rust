use serde::{Deserialize, Serialize};
use std::io::stdin;

// options

#[derive(Debug, Serialize, Deserialize)]
pub struct SizeOption {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitOptions {
    pub frame_duration: u32,
    pub size: SizeOption,
}

// gamestate

#[derive(Debug, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Snake {
    direction: Direction,
    head: Position,
    tail: Vec<Position>,
}

#[derive(Debug, Deserialize)]
pub struct GameState {
    snake: Snake,
    fruit: Position,
    score: u32,
    over: bool,
    paused: bool,
}

pub fn parse_gamestate() {
    let lines = stdin().lines();
    for line in lines {
        println!("got a line: {}\r", line.unwrap());
    }
}
