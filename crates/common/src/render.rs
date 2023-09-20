use crate::stream::{parse_gamestate, GameState};

pub fn run() {
    parse_gamestate();
}

fn render_frame(state: GameState) {
    println!("{:?}\r", state);
}
