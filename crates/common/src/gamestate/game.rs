use crossterm::event::KeyModifiers;
use rand::Rng;
use serde::Serialize;

use crate::gamestate::physics::{Direction, Position};
use crate::gamestate::snake::Snake;

fn calc_random_pos(height: u32, width: u32) -> Position {
    // todo check inverted params
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
}

impl Game {
    pub fn new(width: u32, height: u32, frame_duration: f64) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            frame_duration,
            waiting_time: 0.0,
            score: 0,
            state: GameState::Paused,
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

    // pub fn toggle_game_state(&mut self) {
    //     if self.paused {
    //         self.start();
    //     } else {
    //         self.pause();
    //     }
    // }

    /// returns true if the state has been updated because it was time to
    pub fn update(&mut self, delta_time: f64) -> bool {
        self.waiting_time += delta_time;

        // if self.over {
        // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > self.frame_duration && self.state != GameState::Over {
            // self.check_colision() use snake.get_head_pos;
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

        // match key {
        //     Key::R => self.over = false, // temp solution -> replace current game state trough new one
        //     Key::Space => self.toggle_game_state(),
        //     _ => self.start(),
        // }

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
                code: KeyCode::Left,
                ..
            }) => {
                self.snake.set_dir(Direction::Up); // todo review directions
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                self.snake.set_dir(Direction::Down);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                self.snake.set_dir(Direction::Left);
                Some(())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                self.snake.set_dir(Direction::Right);
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

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}

// fn calc_not_overlapping_pos(pos_vec: Vec<Position>, width: u32, height: u32) {
//     let mut fruit_pos: Position = calc_random_pos(width, height);

//     loop {
//         // if snake_pos.y != fruit_pos.y {
//         //     break;
//         // }

//         for pos in pos_vec {
//             if
//         }

//         snake_pos = calc_random_pos(width, height);
//         fruit_pos = calc_random_pos(width, height);
//     }
// }
