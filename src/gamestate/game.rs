use crossterm::event::KeyModifiers;
use rand::Rng;
use serde::Serialize;

use crate::gamestate::physics::{Direction, Position};
use crate::gamestate::snake::Snake;

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0..width as i32),
        y: rng.gen_range(0..height as i32),
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GameState {
    Paused,
    Over,
    Running,
}

#[derive(Debug, Serialize)]
pub struct Game {
    snake: Snake,
    fruit: Position,
    #[serde(skip)]
    size: (u32, u32),
    #[serde(skip)]
    frame_duration: f64,
    #[serde(skip)]
    waiting_time: f64,
    score: u32,
    pub state: GameState,
    #[serde(skip)]
    initial_snake_length: u32,
}

impl Game {
    pub fn new(width: u32, height: u32, frame_duration: f64, snake_length: u32) -> Self {
        Self {
            snake: Snake::new(calc_random_pos(width, height), snake_length),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            frame_duration,
            waiting_time: 0.0,
            score: 0,
            state: GameState::Paused,
            initial_snake_length: snake_length,
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Running;
    }

    pub fn pause(&mut self) {
        self.state = GameState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = GameState::Running;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(
            calc_random_pos(self.size.0, self.size.1),
            self.initial_snake_length,
        );
        self.fruit = calc_random_pos(self.size.0, self.size.1);
        self.score = 0;
        self.state = GameState::Running;
    }

    /// returns true if the state has been updated because it was time to
    pub fn update(&mut self, delta_time: f64) -> bool {
        self.waiting_time += delta_time;

        if self.waiting_time > self.frame_duration && self.state != GameState::Over {
            self.waiting_time = 0.0;

            if self.state == GameState::Paused || self.state == GameState::Over {
                return true;
            }

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlapp() {
                self.snake.update(self.size.0, self.size.1); // todo check inverted params

                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
            } else {
                self.state = GameState::Over;
            }
            return true;
        }
        return false;
    }

    pub fn key_down(&mut self, event: crossterm::event::Event) -> Option<()> {
        use crossterm::event::{Event, KeyCode, KeyEvent};

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                ..
            }) => {
                if self.state != GameState::Paused {
                    self.pause();
                } else {
                    self.resume();
                }
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                ..
            }) => {
                self.restart();
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                self.snake.set_dir(Direction::Left);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                self.snake.set_dir(Direction::Right);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                self.snake.set_dir(Direction::Up);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                self.snake.set_dir(Direction::Down);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => None,
            _ => Some(()),
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calc_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32
    }
}
